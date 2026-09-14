> ✅ UPDATE (coordinator, 2026-09-06 04:20): the `🔣️taxonomy.json` conflict described below is RESOLVED (file rewritten to the upstream side, JSON validates). The harness phases can be run now. Executors: run `discover`, `subject`, `oracle`, `parity` for your cases and paste the real output in your report.

# 📓️ Protocol v2 harness verification — `host-protocol-parity`

## 0. Blocking defect found: dynamic execution is currently impossible repo-wide

Every phase (`discover`, `doctor`, `contract`, `oracle`, `subject`, `parity`, `run`, `nx`, …) calls
`testTaxonomy()` (`🧪️test/📦️packages/🟦️typescript/🟦️.ts:116-119`), which does
`JSON.parse(readFileSync(TAXONOMY_REL_PATH))` where
`TAXONOMY_REL_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"`
(same file, line 19). That file is **currently an unresolved git conflict** in the working tree:

```
$ git status --porcelain -- .../🔣️taxonomy.json
UU 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
$ grep -n '^<<<<<<<\|^=======\|^>>>>>>>' .../🔣️taxonomy.json
5883:<<<<<<< Updated upstream
5888:=======
5948:>>>>>>> Stashed changes
13565:<<<<<<< Updated upstream
13928:=======
18285:>>>>>>> Stashed changes
20683:<<<<<<< Updated upstream
20768:=======
21775:>>>>>>> Stashed changes
22454:<<<<<<< Updated upstream
22717:=======
24552:>>>>>>> Stashed changes
```

No `MERGE_HEAD` exists, so this is leftover from a `git stash pop` conflict (the "Stashed changes"
marker), i.e. exactly the kind of modifying git command CLAUDE.md forbids (`git stash` / `stash
pop`) — done by some other concurrent process, not by me. `git show HEAD:<path>` IS valid JSON
(995 524 bytes), confirming the committed baseline is fine and only the working tree copy is broken.

**Effect**: `bun ./📜️script.ts discover` (and everything else) fails immediately with
`SyntaxError: JSON Parse error: Unrecognized token '<'` at
`🟦️.ts:118` → `discoverTestCases` (`🟦️.ts:540`) → `DiscoverScript.run` (`📜️script.ts:818`).
Captured in `🗑️generated/harness-verification/01-discover.txt`. I rechecked immediately before
writing this report — the conflict is still present (4 unresolved hunks). Per my instructions I did
**not** touch this file (out of scope, and another agent may be mid-resolution).

**Fix needed (not applied by me)**: whoever owns `🔨️modules/📚️library` must resolve the 4 conflict
hunks in `🔣️taxonomy.json` (or hard-reset the file to HEAD if the stash content is unwanted) and
`git add` it. Once that lands, every command below is expected to run — I verified the entire call
chain statically (README, `script.ts`, `🟦️.ts` library, schema, the `host-protocol-parity` case
files) and found no other blocker for this specific case. **Re-run this ticket's verification once
the taxonomy file is resolved** — nothing else here was diagnosed by inference; only the JSON‑parse
failure was actually executed.

All other findings below come from static reading of the source (line numbers cited), cross-checked
against the one real oracle-registry example that exists in the repo
(`🧰️framework/🛍️products/📓️print/🔣️oracle.json`, schema v1 — the test module's own registry
`🧪️test/📇️registry/🔣️.json` is schema v2 and currently empty of oracles/probes, since the framework's
own registry deliberately excludes anything domain-specific).

## 1. Exact command sequence to run once unblocked

From the repo root (`C:\git\semio`), with `RUSTC_WRAPPER=""` exported:

```bash
export RUSTC_WRAPPER=""
MOD="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun "./$MOD/📜️script.ts" discover
bun "./$MOD/📜️script.ts" doctor
bun "./$MOD/📜️script.ts" contract --case host-protocol-parity
bun "./$MOD/📜️script.ts" subject fundamental --case host-protocol-parity --implementation go
bun "./$MOD/📜️script.ts" subject fundamental --case host-protocol-parity --implementation rust
bun "./$MOD/📜️script.ts" subject fundamental --case host-protocol-parity --implementation typescript
bun "./$MOD/📜️script.ts" parity fundamental --case host-protocol-parity
```

Notes on invocation, confirmed from the code (not assumed):
- The script **must** be run with `bun`, from any cwd, using a path to `📜️script.ts` — it resolves
  its own root via `import.meta.dir` (`ScriptRouter` ctor) and the repo root via `findRepoRoot`
  (`🟦️.ts:1094`) walking up for `.🧬semio`. Running it from the module directory
  (`cd "$MOD" && bun ./📜️script.ts …`) works identically; both were intended (README shows the
  bare form assuming cwd = module dir, and `bun ./📜️script.ts test <phase>` "from the repository
  root" — meaning the root-level `📜️script.ts`/router, not this one, forwards there. I did not find
  a root `📜️script.ts` file in this checkout — only per-module ones — so from the repo root you must
  give the module-relative path as above; there is a `router.register("test", RunScript)` alias
  inside this module's own router, unrelated to a root dispatcher).
- `discover` takes no case selector; it lists **every** case under the whole tree (excluding
  `compose/**`, structurally, via `isExcludedTestPath` / `getRepoMetaDir`, `🟦️.ts:163-176`).
- `doctor` also takes no selector; it always probes bun/cargo/go/python3/dotnet
  (`📜️script.ts:986-1006`) and only fails (`process.exit(1)`) if a tool **claimed by a discovered
  case** is missing (`claimed = discoveredCases…adapters`, line ~996) — so on this host (bun ✓,
  cargo ✓, go ✓, python3 ✓; dotnet not checked but no `.cs` case here is affected) it should pass
  once `discover` itself works.
- `contract --case host-protocol-parity` runs `validateAllContracts` over just that case
  (`selectCases` matches `--case` exactly, `📜️script.ts:220-233`), then writes
  `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` and exits 1 if any breach.
- `subject <level> --case … --implementation …`: `<level>` is a **positional** arg consumed by
  `resolveTestLevel` inside `runPhases` (`📜️script.ts:701-702`), one of
  `fundamental|quick|long|exhaustive` (`TEST_LEVELS`). `--case` and `--implementation` are the
  selector flags (`selectCases`/`selectImplementations`, lines 220-241). Since
  `host-protocol-parity`'s feature carries `@no-oracle-repo-test-platform` (no oracle role at all),
  the `oracle` phase for this case is a no-op by design (`oracleDecision` returns
  `implementation: null`, `📜️script.ts:684-693`) — only `subject` and `parity` do anything, matching
  the task's own choice to skip `oracle …`.
- `parity fundamental --case host-protocol-parity` runs both `oracle` (no-op here) and `subject`
  phases and then **cross-subject** parity (`evaluateCrossSubjectParity`, `📜️script.ts:767-771`)
  under the feature's `@comparison-ordered-json-v1` profile — since there is no oracle, this pairwise
  equivalence across the three claimed adapters (go/rust/typescript) IS the whole proof for this
  case, and `oracleDecision.implementation === null` + `noOracleDecision === "repo-test-platform"`
  additionally requires (`📜️script.ts:776-781`) that the decision's `substitutes` cover every
  scenario's `@mode-…` — `repo-test-platform`'s substitutes are `specification-vectors` +
  `metamorphic-laws` (registry, not `independent-implementations`), and the case's 3 scenarios are
  tagged `@mode-differential`, `@mode-error`, `@mode-conformance` — the `error`/`conformance` ones
  are covered by those substitutes per line 779-780's allow-list, but the `differential` scenario
  (`digest-and-fixture-resolution`) is **not** in that allow-list, so it needs cross-subject pairwise
  parity to actually run and agree; with 3 subject implementations that will produce 3 pairs
  (go×rust, go×ts, rust×ts).
- There is also a bare `nx` route: `bun ./📜️script.ts nx` just dumps the discovered-case JSON used to
  generate Nx projects (`📜️script.ts:980-984`) — it does not execute anything, so it does not
  substitute for `subject`/`parity`. The real Nx targets are the ones registered in this module's own
  `📋️project.json` — `test-discover`, `test-contract`, `test-oracle`, `test-subject`, `test-parity`,
  `test-report`, `test-clean`, `test-dependency`, `test-doctor`, `test-inventory`,
  `test-fixture-{verify,audit,reproduce,generate}`, `test-probe`, `test-matrix`, `test-gc`, and plain
  `test` (→ `run`) — **all on the single project `@semio-tech/repo-test-domain`**, all just
  `nx:run-commands` wrapping `bun ./📜️script.ts <phase>` with `forwardAllArgs: true`
  (`📋️project.json`). So the Nx equivalent of the sequence above is, e.g.:
  `bun nx run @semio-tech/repo-test-domain:test-subject -- fundamental --case host-protocol-parity --implementation go`
  (untested — blocked by the same taxonomy failure; `nx run <proj>:<target> -- <args>` is standard
  Nx forwarding and matches `forwardAllArgs: true`). I did **not** find generated *per-case* Nx
  projects/targets (`test-quick`, `test-long`, `test-exhaustive` mentioned in the README as "every
  case also has generated Nx targets") inside this checkout — only this one static
  `@semio-tech/repo-test-domain` project exists on disk; the per-case ones are presumably produced by
  an Nx plugin (`readdirSync`-driven project graph, likely in `📚️library`'s Nx plugin) that itself
  calls `discoverTestCases`, so it too is blocked by the taxonomy conflict right now and could not be
  confirmed.

## 2. How Go and Rust adapters are compiled and wired (from `📜️script.ts:390-490`)

### Rust subject/oracle host — `materializeRustHost` (lines 419-480)
- Host dir: `.🧬semio/🦑️repo/⚡️cache/tests/hosts/<projectName>-<role>-rust/` (marked with an
  ownership stamp via `markOutputDir`/`markRunComplete`, `hostDirFor`, line 351-358).
- Generates `Cargo.toml` + `src/main.rs` there:
  - `Cargo.toml`: an **inline `[workspace]`** table (so it is its own workspace root, isolated from
    the repo's `Cargo.toml`), package name
    `semio-test-host-<case-slug-with-non-alnum-collapsed-to-dashes>`, one `[[bin]] name = "host"`.
  - Always depends on `semio-repo-test-host` (path = `🧪️test/📦️packages/🦀️rust`) — the shared host
    support crate every generated Rust host links.
  - The **subject-under-test (SUT) crate** is discovered by `rustSutCrate` (lines 391-407): walk up
    from the case's owner directory looking for `📦️packages/🦀️rust/Cargo.toml`; read its `name =`
    line by regex; if the name is `semio-repo-test-host` itself (i.e. the case is owned by the test
    domain, like this one), there is **no** subject crate (`sut === null`) — this case's Rust
    "subject" is the adapter alone plus the host support crate, no domain crate to link. When a real
    SUT exists, it's linked as an **optional path dependency** gated behind a `sut` Cargo feature
    (`[features] sut = ["dep:<crate-name>"]`), and the generated Cargo invocation only turns that
    feature on `role === "subject"` (`args: [...,"--features","sut"] ` only for subject, never for
    oracle — comment at lines 445-449 explains oracle role must not even compile the SUT).
  - Any **oracle packages the owner contributed** for Rust (`contributedOraclePackages`, only ones
    with a `path`, i.e. local in-repo crates — a crates.io coordinate is refused, lines 424-428) are
    added as extra `[dependencies]` lines, each with its declared `features` if any.
  - `src/main.rs` does **not copy** the adapter file; it uses
    `#[path = "<absolute path to the committed 🦀️.rs adapter>"] mod adapter;` so the compiled code is
    always exactly the committed source, then calls
    `semio_repo_test_host::run_main(adapter::adapter())`.
  - **Required adapter signature**: a free function `pub fn adapter() -> Adapter` (this case's
    `🦀️.rs` matches exactly: `pub fn adapter() -> Adapter { Adapter::new("rust").subject(...)... }`,
    imported types `semio_repo_test_host::{digest, Adapter, Context, Json, Outcome}`).
  - Run command: `cargo run --quiet --manifest-path <hostDir>/Cargo.toml [--features sut] -- --plan
    <planPath> --out <outPath>`, cwd = repo root, env adds
    `CARGO_TARGET_DIR = <agentCacheRoot>/cargo-test-hosts` (a **shared** target dir across all
    generated Rust hosts, for cache reuse) and inherits `process.env` — so `RUSTC_WRAPPER=""` set by
    the caller propagates through.
- **No `go.mod`-equivalent workspace registration needed for Rust**: because `[workspace]` is empty
  and standalone, it does *not* need to be listed in the root `Cargo.toml`'s member list (the plan's
  §3 rule "Root `Cargo.toml` lists every crate" is about the *committed* domain crates, not these
  disposable generated hosts).

### Go subject/oracle host — `materializeGoHost` (lines 483-490)
- Host dir: same `hosts/<projectName>-<role>-go/` scheme.
- Generates 3 files:
  - `go.mod`: `module semio.test/host`, `go 1.23`, `require semio.tech/repo/test v0.0.0` +
    `replace semio.tech/repo/test => <abs path to 🧪️test/📦️packages/🐹️go>`. So **yes**, a `go.mod` is
    synthesized fresh per host — the committed Go package under `🧪️test/📦️packages/🐹️go` does not
    need its own `go.mod` reachable from a workspace; it is pulled in purely via this generated
    `replace` directive pointing at its absolute path. (I did not check whether
    `🧪️test/📦️packages/🐹️go/go.mod` itself exists and declares module `semio.tech/repo/test` — that
    is required for the `replace` to resolve; static-only, unverified — see open question below.)
  - `adapter.go`: the **committed** `🐹️.go` adapter file content, copied byte-for-byte except its
    `package \w+` header is rewritten to `package main` (regex replace, line 487) — so the adapter is
    always compiled as-is, never hand-duplicated, but Go's one-package-per-directory rule is
    satisfied by textual rewrite rather than Rust's `#[path]` trick.
  - `main.go`: `import host "semio.tech/repo/test"`; `func main() { host.RunMain(Adapter()) }`.
  - **Required adapter signature**: package `adapter` (before rewrite) exposing
    `func Adapter() *host.Adapter` returning `host.NewAdapter("go").Subject(id, fn)...` — confirmed
    exactly against this case's `🐹️.go`.
- **How the Go implementation-under-test is imported**: unlike Rust there is no separate "SUT crate"
  linking step visible in `materializeGoHost` itself — a Go case's adapter would `import` its owning
  domain's Go package directly by its module path (e.g. `github.com/usalu/semio/repo/<suffix>`), and
  since the generated `go.mod` only declares a `replace` for the **test** module, a domain package
  import inside the adapter would need the domain's own module to be reachable — most likely via
  `GOWORK`/the repo's `go.work` (but `GOWORK: "off"` is explicitly set in the run env, line 489) or
  via **its own `replace` directive that would need to be added to this generator** for any case
  whose Go adapter imports a domain package outside `🧪️test`. This case's `🐹️.go` only imports
  `semio.tech/repo/test` (the host), so it doesn't exercise that path — **this is a real gap to flag
  for the wave-2 Go-domain executors**: `materializeGoHost` (lines 483-490) has no equivalent of
  `rustSutCrate`, so a Go case that needs the domain's own package compiled in will currently fail to
  resolve that import unless the harness is extended, or unless Go's `GOPATH`/module proxy already
  finds it some other way I could not verify without running.
- Run command: `go run . --plan <planPath> --out <outPath>`, cwd = the **generated host dir** (not
  repo root, unlike Rust/TS), env: `GOFLAGS=-mod=mod`, `GOWORK=off`.

### TypeScript oracle host — `materializeTypescriptHost` (lines 577-594)
- **Nothing is generated on disk** for TypeScript — it runs the committed
  `🧪️test/🖥️host/🟦️.ts` directly via `bun <hostPath> --plan <planPath> --out <outPath> --adapter
  <absolute path to the case's 🟦️.ts>`, cwd = repo root.
- Any declared TypeScript oracle package (`contributedOraclePackages(…, "typescript")`) is checked
  for resolvability from the repo root's own `node_modules` (`resolvesFromRepoRoot`, lines 585-594)
  — via `createRequire(repoRoot/package.json).resolve(specifier)`, falling back to checking
  `node_modules/<specifier split on "/">/package.json` exists. An unresolvable declared package is
  reported as a host `problem` (surfaces as `<caseDir>: declared typescript oracle package <pkg> does
  not resolve from the repository's node_modules — add it to the root manifest and install it`) and
  the whole run for that role/implementation aborts before executing (line 667 in `executeOne`: `if
  (host.problems.length > 0) return { results: [], problems: [...] }`).
- **This is exactly the mechanism for a TypeScript oracle using a third-party npm library**: the
  owner's `🔣️oracle.json` names the npm package (see §3 below); the TS adapter file just does a bare
  `import` of it; nothing else is generated — one lockfile, one `node_modules`, no private install.

### Registration of an oracle via the owner's `🔣️oracle.json`
Confirmed shape from the schema (`🧪️test/🧬️schema/🔣️.json`, `OracleRegistryEntry`, required fields:
`id, kind, ecosystem, package, capabilities, comparisonProfiles, license, testOnly, engine,
productionReachable, networkDuringExecution`) plus the one real example in the repo,
`🧰️framework/🛍️products/📓️print/🔣️oracle.json` (schema v1 — **not** yet migrated to v2 per that
file's flatter shape: `id, package, version, entry?, covers[], reason`). The v2 registry the test
module itself uses is `🧪️test/📇️registry/🔣️.json` (`schemaVersion: 2`, currently
`"oracles": [], "probes": []` — it is the **framework's own domain-neutral** registry and
deliberately holds nothing domain-specific; a real owner's per-domain registry is a **sibling**
`🔣️oracle.json` file discovered by convention, not this one). The README (`README.md:126-146`)
gives the authoritative v2 field for linking a library by implementation:

```json
"oracleHostPackages": [
  { "implementation": "typescript", "package": "semver", "version": "7.8.5" }
]
```

`path` present ⇒ local in-repo crate (Rust only, linked by Cargo path — refused for anything without
a path); `path` absent ⇒ external distribution, resolved per-ecosystem as documented in §"Reaching a
reference library" of the README (Python: cache-local venv under
`.🧬semio/🦑️repo/⚡️cache/tests/hosts/`, `--system-site-packages`, keyed by declared package set;
TypeScript: resolved from the repo's own `node_modules`, never installed privately). Every such
package must also be registered in root `🔒️dependencies.json` as `kinds: ["test-oracle"]`,
`productionReachable: false` (`loadClassifiedBaseline`, `📜️script.ts:117-166`, folds in
`oracleLinkedPackages` and `externalOracleHostPackages` automatically) — an owner does not hand-edit
that file for a new oracle package; the ratchet computes it from the registry.

## 3. Fixture resolution (`shared://`, `local://`, `asset://`) — `🟦️.ts:1001-1020`

- `shared://<name>` → `<owner>/🧫️fixtures/<name>` (this case: `shared://📡️protocol-vector.txt` →
  `🧪️test/🧫️fixtures/📡️protocol-vector.txt`, confirmed present on disk).
- `local://<name>` → `<caseDir>/🧫️fixtures/<name>` (private to one case; **never** shadows a
  `shared://` name of the same basename — they are different URI schemes entirely, resolved against
  different base directories, so there is no shadow-collision logic needed, contrary to what
  "shadow" might suggest — the README's "`local://` never shadows `shared://`" (line 161) means a
  local fixture cannot be referenced with the `shared://` scheme to override a shared one; each
  scheme is a hard-wired separate root).
- `asset://<name>` → `<owner>/<name>` directly (owner root, not a fixtures dir — for large committed
  real-world artifacts).
- All three: path-escape guarded (`resolve(abs).startsWith(guard + sep)`) and existence-checked; a
  miss goes to `missing[]`, which `buildCasePlan`/`executeOne` turn into
  `<caseDir>: unresolved fixture <uri>` problems (never a silent default).
- Every resolved fixture carries a plan-time `digest` (`fileDigest`, sha256 truncated to 32 hex
  chars) — fixtures are immutable; a scenario that needs to mutate one must copy it into the
  per-role work directory first (`workDir`, under the marked test cache).

## 4. `@level-` / `@mode-` tag effects

- Every scenario **must** carry exactly one `@id-<kebab-case>`, one
  `@level-<fundamental|quick|long|exhaustive>`, one
  `@mode-<differential|conformance|round-trip|property|error>` — enforced at parse time
  (`materializeScenario`, `🟦️.ts:441-452`); a scenario with 0 or ≥2 of either tag is a contract
  breach, not a silent default.
- `@level-` selects **which scenarios run** for a given CLI level argument, cumulatively:
  `levelsUpTo(level)` (`🟦️.ts:1084-1086`) returns every level at-or-below the requested one in
  `TEST_LEVELS` order (`fundamental, quick, long, exhaustive`), so `subject quick …` also runs every
  `fundamental` scenario. This case's 3 scenarios are `fundamental, fundamental, quick` — running
  `subject fundamental` runs only the first two; the third (`work-directory-is-cache-local`) needs
  `subject quick` or higher.
- `@mode-` decides what a **recorded no-oracle decision** is allowed to discharge for that scenario
  without cross-subject parity (`📜️script.ts:776-781`, discussed in §1): `conformance`, `property`,
  `round-trip`, `error` can be discharged by `specification-vectors`/`metamorphic-laws` substitutes
  alone; `differential` cannot — it always needs either a real oracle or ≥2 independently-written
  subject implementations agreeing (cross-subject parity, `evaluateCrossSubjectParity`). It also
  feeds `@comparison-…`'s `ComparisonProfile` selection per scenario via `planModeOf`
  (`📜️script.ts:696-698`).

## 5. How a new owner directory is discovered

From `discoverTestCases` (`🟦️.ts:539-584`): the walker (`walkDirectories`) visits **every** directory
in the repo except `isExcludedTestPath` matches (the repo's own `.🧬semio` meta root, structurally,
plus whatever `taxonomy.pathExclusions` lists — `compose/**` is the one named in the README).
Whenever it finds a directory literally named `🧪️tests` (`taxonomy.testsDirName`), its **parent** is
the owner (`ownerAbs = dirname(abs)`) — no allowlist of product/module paths, no requirement that the
owner live under `🔨️modules/`. So `🔨️modules/🧾️yaml/🧪️tests/<case>/🥒️.feature` (+ adapters) is
discovered automatically the instant those files exist on disk, with zero registration anywhere else
(the case is auto-named `testProjectName(ownerRel, entry)`, and Nx projects are meant to be generated
from this same `discoverTestCases()` call by a plugin — not confirmed running, see §1). Requirements
per case directory, all enforced by `contract` (`validateAllContracts`, referenced at
`🟦️.ts:1624-1693`):
1. A `🥒️.feature` file (mandatory — without it the directory isn't even recognized as a case).
2. At least one adapter file named exactly per `taxonomy.testAdapterFileKinds`
   (`🦀️.rs, 🟦️.ts, 🐹️.go, 🐍️.py, 🔷️.cs` — confirmed from the HEAD copy of `🔣️taxonomy.json`, since
   the working copy is unreadable right now) — zero adapters is a `no-adapter` breach.
3. The case directory may contain **only** the feature file, adapters, and an optional
   `🧫️fixtures/` child — anything else is an `unknown-case-child` / `unknown-adapter-filename`
   breach.
4. A case must **not** live under `📦️packages` (`case-in-language-package` breach) — it must be owned
   by the nearest language-neutral directory, never by one implementation's package.

## 6. Minimal complete template for a new case — Go + Rust subjects, TypeScript oracle

Example: a hypothetical `🔨️modules/🧾️yaml` owner, case `round-trip-scalar`, oracle = the npm `yaml`
package.

**`🔨️modules/🧾️yaml/🔣️oracle.json`** (owner-level, v2 shape per README + schema `OracleRegistryEntry`):
```json
{
  "$schema": "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
  "schemaVersion": 2,
  "oracles": [
    {
      "id": "yaml-js",
      "kind": "third-party-library",
      "ecosystem": "javascript",
      "package": "yaml",
      "version": "2.8.1",
      "engine": { "family": "none", "implementation": "yaml", "version": "2.8.1" },
      "capabilities": ["yaml.scalar-round-trip"],
      "comparisonProfiles": ["ordered-json-v1"],
      "license": "ISC",
      "testOnly": true,
      "productionReachable": false,
      "networkDuringExecution": false,
      "oracleHostPackages": [
        { "implementation": "typescript", "package": "yaml", "version": "2.8.1" }
      ]
    }
  ]
}
```

**`🔨️modules/🧾️yaml/🧪️tests/round-trip-scalar/🥒️.feature`**:
```gherkin
@capability-yaml-scalar-round-trip
@oracle-yaml-js
@comparison-ordered-json-v1
Feature: Scalar values survive a YAML encode/decode round trip

  @id-plain-scalar-round-trips
  @level-fundamental
  @mode-differential
  Scenario: A plain scalar decodes and re-encodes to the same value
    Given the shared conformance vector shared://📡️scalar-vector.yaml
    When the host decodes it and re-encodes the result
    Then every implementation projects the same decoded value and re-encoded text
```

**`🔨️modules/🧾️yaml/🧫️fixtures/📡️scalar-vector.yaml`**: any small immutable YAML fixture.

**`🔨️modules/🧾️yaml/🧪️tests/round-trip-scalar/🐹️.go`**:
```go
package adapter

import host "semio.tech/repo/test"

func plainScalarRoundTrips(ctx *host.Context) (host.Outcome, error) {
	vector, err := ctx.FixtureBytes("shared://📡️scalar-vector.yaml")
	if err != nil {
		return host.Outcome{}, err
	}
	decoded, encoded, err := yaml.RoundTrip(vector) // the domain's own yaml package
	if err != nil {
		return host.Outcome{}, err
	}
	return host.Outcome{Projection: map[string]any{"decoded": decoded, "encoded": encoded}}, nil
}

func Adapter() *host.Adapter {
	return host.NewAdapter("go").Subject("plain-scalar-round-trips", plainScalarRoundTrips)
}
```
(imports the domain's own Go package the normal way — this only resolves under
`materializeGoHost` if that generator is extended per the gap noted in §2, or if the domain package
is already reachable through the ambient `GOPATH`/module cache independent of this case's generated
`go.mod` — unverified, flag for whoever writes the Go adapter).

**`🔨️modules/🧾️yaml/🧪️tests/round-trip-scalar/🦀️.rs`**:
```rust
use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_framework_repo_yaml::round_trip;

fn plain_scalar_round_trips(ctx: &Context) -> Result<Outcome, String> {
    let vector = ctx.fixture_bytes("shared://📡️scalar-vector.yaml")?;
    let (decoded, encoded) = round_trip(&vector).map_err(|e| e.to_string())?;
    Ok(Outcome::projection(Json::Object(vec![
        ("decoded".to_string(), decoded),
        ("encoded".to_string(), Json::String(encoded)),
    ])))
}

pub fn adapter() -> Adapter {
    Adapter::new("rust").subject("plain-scalar-round-trips", plain_scalar_round_trips)
}
```
(requires `🔨️modules/🧾️yaml/📦️packages/🦀️rust/Cargo.toml` to exist with
`name = "semio-framework-repo-yaml"` per the plan's §3 convention — `rustSutCrate` walks up from the
case owner and finds it automatically, no registration needed beyond that crate existing.)

**`🔨️modules/🧾️yaml/🧪️tests/round-trip-scalar/🟦️.ts`** (the oracle adapter, using the npm `yaml`
package declared above):
```ts
import { parse, stringify } from "yaml";
import type { Adapter, Context } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🟦️.ts";

export function adapter(): Adapter {
  return {
    id: "typescript",
    oracle: {
      "plain-scalar-round-trips": (ctx: Context) => {
        const vector = ctx.fixtureText("shared://📡️scalar-vector.yaml");
        const decoded = parse(vector);
        return { projection: { decoded, encoded: stringify(decoded) } };
      },
    },
  };
}
```
(exact host-side `Adapter`/`Context` TypeScript shape not independently confirmed — I read the Rust
and Go host support modules directly but not the full `🖥️host/🟦️.ts`; mirror its exported types
rather than trusting this sketch verbatim.)

No other file needs to change — the next `discover` call finds this case automatically (§5).

## 7. Runtime and Windows-specific notes

- Toolchains present on this host: `bun 1.4.2`, `go` (Program Files\Go\bin), `cargo` (`~/.cargo/bin`),
  Windows-Store `python3` shim, dotnet not checked. All discoverable via plain `which`/`--version`,
  matching what `doctor` probes.
- No dynamic run completed (blocked, §0), so no real wall-clock timing or Windows-specific runtime
  failure (path separators, `cargo run` quoting, emoji paths in generated `Cargo.toml`/`go.mod`,
  etc.) could be captured. Static risks worth watching once unblocked:
  - `materializeGoHost` writes `GOFLAGS=-mod=mod` and `GOWORK=off` — on Windows this should behave
    the same as elsewhere since it's plain env + `go run .` in a generated dir with an absolute
    `replace` path; the only known Windows wrinkle is backslash-vs-forward-slash in that path, which
    `join()`/Node's path module handles automatically converted to native separators — `go.mod`
    accepts native OS paths in `replace` on Windows too.
  - The Rust host's generated `Cargo.toml` paths and the `#[path = "…"]` attribute in `main.rs` both
    embed the **absolute Windows path** (e.g. `C:\git\semio\...`) via `JSON.stringify(...)` for
    proper escaping — should be fine, but backslashes inside a Rust string literal must be escaped;
    `JSON.stringify` does escape backslashes correctly for both JSON (Cargo.toml is TOML, not JSON,
    but the escaping used, `JSON.stringify`, still produces valid TOML-quoted double-backslash
    escapes) and Rust source (Rust string literals use the same `\\` escape as JSON) — so this looks
    correct by inspection, not confirmed by execution.
  - Emoji-named paths (`🧪️tests`, `🦀️.rs`, etc.) round-tripping through `cargo`/`go` on Windows
    (NTFS + code page) were not exercised at all in this session.

## 8. Files touched by this verification

- Read-only investigation of committed source; no repo source modified.
- Captured output: `🗑️generated/harness-verification/01-discover.txt` (the one command that could
  actually run before hitting the blocker).
- This report: `📓️harness-verification.md`.
