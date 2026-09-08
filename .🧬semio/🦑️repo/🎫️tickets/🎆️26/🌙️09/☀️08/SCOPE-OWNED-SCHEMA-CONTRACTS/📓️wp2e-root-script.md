# WP2e — root script partition (rows 117, 118, 128)

Partition: root `📜️script.ts`, root `📋️project.json`, root `package.json`, `.vscode/launch.json`, `nx.json`.
Successor to W2d (`📓️wp2d-root-script.md`). Every number below is a real run on this tree, pasted verbatim.
The machine carried a sustained `load average ≈ 147` from concurrent peer cargo work throughout; where that
distorts a measurement it is said so explicitly.

## 1. Per-row results

| Row | Result |
|---|---|
| 117 (`schema test` oracle config; register the registry crate's targets) | **done** — the throwaway `tmpdir()` config is gone and the run uses the module's tracked config, which raises the oracle from **1 spec file to 3 spec files / 27 tests** (§3.1). The launch registration was already on disk in **both** catalogs when I arrived (§2.4); the crate's `test` target dispatches, builds and passes **7/7** (§3.4) |
| 118 (`--rust-entries` default + `--rust-entries-complete`) | **done** — `schema verify` now cross-checks the Rust registry by default against the committed reference dump, and `schema entries` regenerates that same tracked file. The default run is **red on real content** and the red is not mine (§3.2, §4.1) |
| 128 (`🧪️fixtures` → `🧫️fixtures` in `✏️s/…` root-script paths) | **done** — one literal, applied once its target directory landed on disk at 21:15; `✏️s/` + `🧪️fixtures` in `📜️script.ts` is now 0 (§3.5) |

## 2. What changed

### `📜️script.ts`

1. **`SCHEMA_DRAFT07_ORACLE_SPEC` deleted; `SCHEMA_ORACLE_VITEST_CONFIG` added** (row 117 / wp3c R-3).
   `oracleArguments()` no longer writes a generated one-line config into `tmpdir()`; it is now

   ```ts
   private oracleArguments(): string[] {
     return ["vitest", "run", "--config", join(this.root, SCHEMA_ORACLE_VITEST_CONFIG)];
   }
   ```

   with `SCHEMA_ORACLE_VITEST_CONFIG = "🧰️framework/🔨️modules/🧬️schema/vitest.config.ts"`. That config
   declares `include: ["🧪️tests/**/🟦️.ts"]`, so `framework.schema` declares its oracle specs once and
   adding a spec needs no root-script edit. Both `schema oracle` and `schema test` use it.

   **Deviation from R-3's literal text.** R-3 spelled the config `🧪️vitest.config.ts`. A file existed under
   that exact name and **cannot be loaded**: vitest bundles its config through esbuild, which fails with
   `Could not resolve "…/🧪️vitest.config.ts"` on the emoji filename (§3.1, first run) — the same failure the
   retired `tmpdir()` config was working around one level up. A peer renamed it to the ASCII
   `vitest.config.ts` at `20:58` mid-session (`git status`: `R  🧪️vitest.config.ts -> vitest.config.ts`);
   the constant names the ASCII file, and its docstring records why that one file is not emoji-named.

2. **`SCHEMA_RUST_ENTRIES_REFERENCE` added** (row 118 / wp3c R-6):
   `"🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/📤️schema-export-entries-dump.json"`.
   - `verify()` no longer returns early when `--rust-entries` is absent. The flag now only *overrides* the
     input; without it the committed reference dump is read, so the Rust half of the gate runs on every
     `schema verify` instead of only when a caller remembers the flag:

     ```ts
     const entriesIndex = args.indexOf("--rust-entries");
     if (entriesIndex >= 0 && !args[entriesIndex + 1]) throw new Error("[schema verify] --rust-entries requires a path to the registry dump.");
     const target = entriesIndex >= 0 ? args[entriesIndex + 1]! : SCHEMA_RUST_ENTRIES_REFERENCE;
     ```

     A missing dump is a hard error naming `schema entries` as the way to produce it.
   - `--rust-entries-complete` was already threaded into `schemaRustEntryDiagnostics(…, complete)` by W2c
     and the library half (`rust-scope-unregistered`) is implemented at
     `📚️library/🔍️discovery/🟦️.ts:3447`. Row 118 is therefore complete end to end; §3.3 is its first
     real run.
   - `rustReport()` gained an optional `tracked` home on the test descriptor.
     `SCHEMA_EXPORT_ENTRIES_TEST` now carries `tracked: SCHEMA_RUST_ENTRIES_REFERENCE`, so
     `schema entries` **regenerates the tracked reference** and then verifies against it, instead of
     writing a `tmpdir()` scratch nobody keeps. `--out <path>` still redirects.
     `SCHEMA_MODULE_COMPILE_TEST` declares no `tracked` home and keeps its scratch file — that report is
     evidence, not a contract.

3. **Row 128** — `INTERACTIVITY_AUDIT_PUZZLE_FILL_PREVIEW_FIXTURE_FILE` (`:8328`):
   `…/⏳️precompute/🪣️fill/🧪️fixtures/🔣️.json` → `…/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json`.
   This was the **only** `✏️s/…` path in the root script spelling `🧪️fixtures`. See §3.5 for why it was
   applied late in the pass rather than immediately.

### `📋️project.json`

- `schema-verify` gained `"forwardAllArgs": true`. It is the only `schema-*` target with meaningful flags
  (`--rust-entries-complete`, `--rust-entries <file>`) and no way to pass them; every other flag-carrying
  `schema-*` target already forwards. `inputs: ["schemaSources"]` already covers the new default input —
  `schemaSources` includes `{workspaceRoot}/**/🧬️schema/**/*` and the dump lives at
  `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/…`, so nx hashing stays correct with no `nx.json` edit.
- No target added for the registry crate: `@semio-tech/schema-registry-rs` owns its own `📋️project.json`
  (`test`, `test-quick`, `test-long`, `test-exhaustive`) and nx's repo plugin infers `build`/`check` from
  its `Cargo.toml` (§3.4). Root `📋️project.json` declares root-script commands only.

### `.vscode/launch.json`

**No edit needed — already registered, in both catalogs.** All six targets of the new crate were present on
arrival (staged, absent from `HEAD`), following the existing per-crate naming/grouping exactly:

| name | command | group | order |
|---|---|---|---|
| `📦️build🦀️@semio-tech/schema-registry-rs` | `bun nx run @semio-tech/schema-registry-rs:build` | `4_build` | `501.523` |
| `🔎️check🦀️@semio-tech/schema-registry-rs` | `…:check` | `4_gate` | `501.524` |
| `🧪️test🦀️@semio-tech/schema-registry-rs` | `…:test` | `4_gate` | `501.525` |
| `🧪️test quick🦀️@semio-tech/schema-registry-rs` | `…:test-quick` | `4_gate` | `501.526` |
| `🧪️test long🦀️@semio-tech/schema-registry-rs` | `…:test-long` | `4_gate` | `501.527` |
| `🧪️test exhaustive🦀️@semio-tech/schema-registry-rs` | `…:test-exhaustive` | `4_gate` | `501.528` |

Same shape and neighbouring orders as `@semio-tech/schema-derive-rs` (`501.51`/`501.521`) and
`@semio-tech/framework-schema` (`500.85`/`500.862`). Verified parseable and duplicate-free (§3.6);
`.vscode/🧩️launch.seed.jsonc` carries the same six entries. R-4's proposed name `📇️registry schema test`
was **not** used: it belongs to no family in either catalog, whereas `🧪️test🦀️<project>` is the family all
2 302 entries use for a crate's nx `test` target.

### `package.json`

**Untouched, deliberately** — same reasoning as `📓️wp2d-root-script.md` §5.1, re-measured here.
wp3c R-4 asked for a `schema-registry:test` script. There is no per-crate `*:test` script group in root
`package.json`: of its 93 scripts, 16 name an nx project and **all 16 are `dev:*`/`build:*` app entry
points** — no crate's `test` target has a script, `@semio-tech/framework-schema:test` included. The
`schema:*` group exists because those are *root-script* commands (`workspace:schema-*`). Adding a lone
`schema-registry:test` would create a group of one and break the grouping CLAUDE.md requires following.
Open question §5.1 rather than a silent deviation in either direction.

### `nx.json`

Untouched. `schemaSources` already covers the new default input (above), and the repo plugin's
`**/📋️project.json` include already discovers `@semio-tech/schema-registry-rs` (§3.4).

## 3. Verification (real output)

### 3.1 Row 117 — the tracked config, and why R-3's filename could not work

First run, with R-3's exact path:

```
$ bun ./📜️script.ts schema oracle
✘ [ERROR] Could not resolve "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🧪️vitest.config.ts"

failed to load config from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🧪️vitest.config.ts

⎯⎯⎯⎯⎯⎯⎯ Startup Error ⎯⎯⎯⎯⎯⎯⎯⎯
Error: Build failed with 1 error:
error: Could not resolve "…/🧬️schema/🧪️vitest.config.ts"
    at failureErrorWithLog (…/node_modules/esbuild/lib/main.js:1748:15)
```

The file existed at that path. Reproduced with a *relative* `--config ./🧪️vitest.config.ts` from the module
directory, so it is not a cwd artefact — esbuild, which vitest bundles its config with, cannot resolve the
emoji filename. After the peer's rename to ASCII:

```
$ bun ./📜️script.ts schema oracle
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema

 Test Files  3 passed (3)
      Tests  27 passed (27)
   Start at  21:24:32
   Duration  17.04s (transform 5.55s, setup 0ms, import 12.58s, tests 8.10s, environment 0ms)
```

**3 spec files, not 1.** The retired constant named only `🧪️tests/✅️draft07-oracle/🟦️.ts`; the module
config's `include` also picks up `🧪️tests/🏷️entity-kinds/🟦️.ts` and
`🧪️tests/📤️schema-export-entries/🟦️.ts` — the two oracles wp3c added, which the old invocation silently
never ran. That is the concrete gain of R-3.

### 3.2 Row 118 — `schema verify` with no flags now runs the Rust half

```
$ bun ./📜️script.ts schema verify
[schema verify] stale generated output: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json, 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md. Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.
{"code":"rust-scope-unknown","path":"…/🔣️schema-catalog.json","detail":"The registry declares scope framework.schema, which the catalog does not contain."}
… (30 identical for framework.schema, 3 for framework.schema.entries)
[schema verify] rust entries=33 scopes=2 of 3070 catalogued, findings=33
[schema verify] rust-scope-unknown=33
exit 1
```

Before this change the same invocation printed only the two stale lines and never touched the dump.

**The 33 findings are a real content defect, not a wiring defect** — see §4.1. Both scopes the committed
dump registers (`framework.schema`, `framework.schema.entries`) are **absent from the generated catalog**,
so the Rust↔catalog cross-check cannot go green until the generator catalogues them.

### 3.3 Row 118 — `--rust-entries-complete`

```
$ bun ./📜️script.ts schema verify --rust-entries-complete
[schema verify] stale generated output: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md. Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.
{"code":"rust-scope-unregistered","path":"…/🔣️schema-catalog.json","detail":"♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema declares scope mit-bestand.bericht.documents; no register_scope_schema_exports call publishes it."}
{"code":"rust-scope-unregistered","path":"…/🔣️schema-catalog.json","detail":"✏️s/🔌️plugins/✒️writer/…/✏️editor/🎚️config/🧬️schema declares scope app.writer.writer.config; no register_scope_schema_exports call publishes it."}
… 3 070 lines
[schema verify] rust entries=33 scopes=2 of 3070 catalogued, findings=3103
[schema verify] rust-scope-unknown=33
[schema verify] rust-scope-unregistered=3070
exit 1
```

`3 070 of 3 070` catalogued scopes register nothing — the honest baseline wp3c §8 predicted, and the
acceptance measure for WP4/WP5's registration work. (Note the `🔣️schema-catalog.json` half of the stale
line has disappeared between §3.2 and §3.3: a peer regenerated the catalog in between. Only the `.md`
index is still stale, and neither is a root-script finding.)

### 3.4 Row 117 — the registry crate's nx targets resolve, build and pass

```
$ NX_DAEMON=false bun nx show project @semio-tech/schema-registry-rs --json
targets: [ "build", "check", "test", "test-quick", "test-long", "test-exhaustive" ]
build => {"cwd":".","command":"bun \"…/⚡️caching/🦀️cargo/📜️script.ts\" native cargo build --manifest \"🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml\""}
check => {"cwd":".","command":"bun \"…/⚡️caching/🦀️cargo/📜️script.ts\" native cargo check --manifest \"…/Cargo.toml\""}
test  => {"cwd":"🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust","command":"bun ./📜️script.ts test","forwardAllArgs":true}
```

All six launch entries name a target that exists; `build`/`check` are inferred from `Cargo.toml` by the
repo nx plugin, so `nx.json` needed no edit. wp3c R-4's "not verified — daemon timed out" is now verified.

The target itself, run as the direct script the nx target wraps (private `CARGO_TARGET_DIR` so it could not
contend with the peers' builds on the shared target dir, `RUSTC_WRAPPER=""` to bypass sccache):

```
$ CARGO_TARGET_DIR=<scratch> RUSTC_WRAPPER="" bun ./📜️script.ts test      # first attempt
    Blocking waiting for file lock on package cache
[budget] cargo nextest run … exceeded 15000ms — killed. Trim it, or assign it to a higher level (quick/long/exhaustive).

$ CARGO_TARGET_DIR=<scratch> RUSTC_WRAPPER="" SEMIO_TEST_BUDGET_MS=180000 bun ./📜️script.ts test
 Nextest run ID 8d9085a6-fb8a-4b48-a71d-3aecf21a3d51 with nextest profile: fundamental
    Starting 7 tests across 2 binaries
     Summary [   0.373s] 7 tests run: 7 passed, 0 skipped
```

The crate builds and its 7 assertions pass in 0.373 s. The first attempt's budget kill was the package-cache
lock under `load average 147`, not the crate: the assertions themselves are three orders of magnitude
inside the 15 s `fundamental` budget.

### 3.5 Row 128 — applied, after the directory landed

At the start of this pass the target did not exist, so the literal was deliberately **not** renamed:

```
✏️s/…/🪣️fill/🧪️fixtures/🔣️.json   -> EXISTS
✏️s/…/🪣️fill/🧫️fixtures/🔣️.json   -> MISSING
```

Renaming the literal first would have been silent, not loud: the reader is
`policyReadRustPolicySource` → `policyReadFileSafe`, which returns `""` for a missing file. The audit would
have run on empty input and reported no findings — the same green-by-crashing shape W2d found in
`verify-package-purity`. The plugin owner renamed the directory at `21:15`, and the literal was then applied:

```
✏️s/…/🪣️fill/🧪️fixtures/🔣️.json   -> MISSING
✏️s/…/🪣️fill/🧫️fixtures/🔣️.json   -> EXISTS (6 054 bytes)

$ grep -n "✏️s/" 📜️script.ts | grep "🧪️fixtures" | wc -l
       0
$ grep -c "🧪️fixtures" 📋️project.json package.json nx.json .vscode/launch.json
📋️project.json:0
nx.json:0
package.json:0
.vscode/launch.json:0
```

Three `🧪️fixtures` literals remain in the root script — all `🧰️framework/🔨️modules/🧵️job/🧪️fixtures/…`,
outside row 128's `✏️s` scope, and all three targets still exist under that exact name on disk. They are
recorded as §4.4 so they move when the job module's owner renames the directory.

### 3.6 Catalogs, dispatch and compilation

```
$ bun -e '<jsonc-parser over both launch catalogs>'
launch.json          errors 0  configs 2302   duplicates []
🧩️launch.seed.jsonc  errors 0  configs 1286
```

`📋️project.json` re-parses as valid JSON after the `forwardAllArgs` edit. Router dispatch after the
`entries`/`verify` changes:

```
$ bun ./📜️script.ts schema bogus
error: unknown schema subcommand: "bogus" (expected audit | check | compile | docs | entries | generate | oracle | test | verify).
```

The library's own root-script compile gate, full file:

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
$ bun test "./🧪️tests/⚙️root-script-compiler/🟦️.ts"
 4 pass
 2 fail
 52 expect() calls
Ran 6 tests across 1 file. [37.98s]
```

`Bun and esbuild accept the complete actual root task router` is among the 4 passes — the edited file
compiles under both compilers. The 2 failures are a corrupted fixture vector that predates this pass, are
identical on `HEAD`, and are **not** in this partition; the evidence is in §4.3.

A whole-file `tsc --noEmit` was run as a weak extra signal and is **not** a valid measurement: without the
repo tsconfig it has no Bun globals, so it reports 185 errors in `📜️script.ts` (`Cannot find name 'Bun'`
and friends) and 871 repo-wide. The only usable reading from it is negative evidence for the edited region:

```
$ grep -c "^📜️script.ts(17[3-5][0-9][0-9]," tsc.txt
0
```

Zero diagnostics anywhere in `17300–17599`, the range every edit of this pass falls in.

### 3.7 `schema test` — both halves run, and the oracle half is the new one

```
$ bun ./📜️script.ts schema test
[test schema] 4372 invariant finding(s) over the repository
[test schema]    1971 × schema-export-incomplete
[test schema]    1628 × schema-export-parser-missing
[test schema]     329 × schema-export-unknown
[test schema]     157 × schema-fixture-defines-schema
[test schema]     117 × schema-placement-outside-module
[test schema]      68 × schema-owner-ineligible
[test schema]      48 × schema-ref-unresolved
[test schema]      26 × schema-cross-scope-dependency-forbidden
[test schema]      18 × schema-dialect-not-draft-07
[test schema]       5 × schema-ref-broken-internal
[test schema]       3 × schema-file-missing
[test schema]       1 × schema-placement-forbidden-filename
[test schema]       1 × schema-catalog-malformed
…
[test schema]   … and 4332 more
[test schema] 0/0 schema-bound fixture(s) reached their declared stage

 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema

 Test Files  3 passed (3)
      Tests  27 passed (27)
   Start at  21:43:31
   Duration  9.18s (transform 2.89s, setup 0ms, import 5.95s, tests 4.56s, environment 0ms)

[schema test] harness exit=1, draft-07 oracle exit=0
```

The composite gate behaves as W2c designed it: both halves always run and the exit code is their union.
The oracle half is green on 3 spec files; the harness half is the wave-3 backlog
(`📋️cross-partition-requests.md` row 123 measured 5 166, this run 4 372 — the tree has moved under several
workers). `schema-owner-ineligible × 68` includes the `framework.schema` finding of §4.1.

### 3.8 Row 128 — the renamed literal names a file that exists

`verify interactivity` cannot serve as the proof: its `apps` stage fails far upstream of the puzzle-fill
audit, printing `757 additional all-app discovery failure(s) omitted`,
`19132 descriptors exceed fixed capacity 256` and
`.vscode/launch.json: 2302 configurations exceed fixed capacity 512` (§5.3).
The constant itself, imported from the module under test:

```
[DEBUG] constant = ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json
[DEBUG] exists = true bytes = 6048
[DEBUG] old path exists = false
```

Before the edit the first line named `🧪️fixtures` and `exists` was `false` — i.e. the audit was reading
`""` and finding nothing, exactly the silent state §3.5 describes.

## 4. Cross-partition requests

### 4.1 → W2u library / coordinator — `framework.schema` is not a catalogued scope at all

This is the single blocker for row 118's gate ever going green, and it is not a root-script bug. Probed
with `inventorySchemaScopes(root, loadCatalogTaxonomy())` on the live tree:

```
[DEBUG] framework.schema* scopes: []
[DEBUG] module {"modulePath":"🧰️framework/🔨️modules/🧬️schema","ownerPath":"🧰️framework/🔨️modules","level":null,"scopeId":"framework.schema","facetKindId":"🧬️data"}
[DEBUG] diagnostics: 1
[DEBUG] {"code":"schema-owner-ineligible","path":"🧰️framework/🔨️modules/🧬️schema","detail":"🧰️framework/🔨️modules is not a declared schemaScopeOwnerLevels level."}
```

`🧰️framework/🔨️modules/🧬️schema/🔣️.json` declares
`$id: "https://semio.tech/schema/framework/schema/schema.json"` — scope `framework.schema` — and ten
`$defs` exports, and the module resolves its own scope id correctly. It is dropped because the walker
derives the *owner* as the module directory's parent, and for the framework module that **is itself named
`🧬️schema`** the owner and the module are the same directory: the parent is the ineligible
`🧰️framework/🔨️modules`. Contract §A makes `🧰️framework/🔨️modules/<m>` an eligible framework-module
level, and `<m> = 🧬️schema` is exactly that case. The generator needs the self-owning module rule (a
`🧬️schema` directory that is itself a framework module is its own owner); it must not be fixed by
declaring `🧰️framework/🔨️modules` an owner level, which would make every framework module's parent
eligible.

`framework.schema.entries` — the second scope the Rust dump registers — has no catalogued counterpart
either; whether that is a facet of `framework.schema` or a scope of its own is wp3c's call, but the
catalog must contain whichever it is before `schema verify` can be green.

Acceptance measure, already runnable: `bun ./📜️script.ts schema verify` → `rust-scope-unknown=0`.
The probe is kept as `wp2e-catalog-probe.ts` in this folder (`bun wp2e-catalog-probe.ts`); the row-128
path probe is `wp2e-fixture-path-probe.ts`.

### 4.2 → coordinator — row 128 is closed on the root side

`📋️cross-partition-requests.md` row 128 lists `W5c os + W2w root`. The root half is done (§3.5). The other
two spellers named in that row (`🌊️flow/…/🔬️slider-label/🦀️.rs`,
`📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`) are untouched by this pass and remain with their
owners.

### 4.3 → W2u library — the `⚙️root-script-compiler` fixture vector is path-rewrite corrupted

`📚️library/🧪️tests/⚙️root-script-compiler/🔣️.json` is unchanged since `Sep 4 23:14` and identical to
`HEAD`, so this red is pre-existing and unrelated to this ticket — but it is worth naming because the
corruption signature is a path-rewrite sweep that replaced bare filenames with paths relative to the *test
case directory*:

```
$ bun test "./🧪️tests/⚙️root-script-compiler/🟦️.ts" -t "eager policy vocabulary"
- Expected  - 6
+ Received  + 6
-     "POLICY_TS_COMPONENT_LEAF": "../⚙️root-script-compiler/🟦️.ts",
+     "POLICY_TS_COMPONENT_LEAF": "🟦️.ts",
-     "../⚙️root-script-compiler/🔣️.json",      (×2, inside POLICY_STDIO_TEXT_SPEC_LEAVES / POLICY_FACET_MIRROR_DRIFT_SIBLINGS)
+     "🔣️.json",
```

The same sweep hit `execution.launchName`: the vector expects
`"🧹clean🧩️taxonomy⚙️root-script-compiler"` (the `🧪️` grapheme replaced by the case directory's `⚙️`)
plus a `--skip-nx-cache` suffix on `launchCommand`. Both launch catalogs spell
`🧹clean🧩️taxonomy🧪️root-script-compiler` with no suffix, which is the family all **19** sibling
`🧹clean🧩️taxonomy🧪️<case>` entries use, and only 2 of 2 302 entries use `--skip-nx-cache` at all. **The
launch catalogs are the correct side; the vector is the corrupted side.** I deliberately did not "fix"
`.vscode/launch.json` to satisfy a corrupted expectation — that would have broken the naming family
CLAUDE.md requires following, in my own partition, to make someone else's damaged fixture green.

### 4.4 → framework `🧵️job` module owner — three `🧪️fixtures` literals left in the root script

`📜️script.ts:5874`, `:6009`, `:6081` read
`🧰️framework/🔨️modules/🧵️job/🧪️fixtures/{📇️fixed-operation-registry-law.json,⚖️shared-framework-action-routes-law.json}`.
Both files exist under that name; the taxonomy declares `testFixturesDirName = 🧫️fixtures`, so the
directory is drifted. Outside row 128's `✏️s` scope, so not renamed here — when the job module's owner
renames the directory, those three lines move with it (and only those three: `grep -c "🧪️fixtures"` in the
root script is 3).

### 4.5 → W3e framework schema owner — the tracked vitest config's name is load-bearing

`schema oracle` and `schema test` now depend on `🧰️framework/🔨️modules/🧬️schema/vitest.config.ts`
keeping an **ASCII** filename (§3.1). A future taxonomy sweep that re-emojifies it silently breaks both
commands with a startup error that reads like a missing file. If the taxonomy is ever taught to demand a
leading grapheme for `*.config.ts`, this file needs a declared exception, and the reason is esbuild's
resolver, not vitest's.

## 5. Open questions

1. **`package.json` and per-crate test scripts.** wp3c R-4 wants `schema-registry:test`; the file has no
   per-crate `*:test` group to put it in (§2.5), and `@semio-tech/framework-schema:test` — the crate this
   one was split out of — has no script either. Either every registered crate's `test` target gets a script
   in one pass, or none does. Same shape as W2d's open question 1 about the `verify` family; both are one
   coordinator decision about what root `package.json` is for.
2. **`schema verify` is now red by default, and should stay that way.** Its exit code was previously driven
   only by catalog staleness; it now also fails on 33 `rust-scope-unknown`. Anything that wires
   `schema-verify` into a blocking gate must wait for §4.1, not weaken the default back to opt-in.
3. **`.vscode/launch.json` has outgrown its declared capacity.** `verify interactivity` reports
   `.vscode/launch.json: 2302 configurations exceed fixed capacity 512` and
   `.vscode/🧩️launch.seed.jsonc: 1286 configurations exceed fixed capacity 512`
   (`INTERACTIVITY_ALL_APP_LAUNCH_CAPACITY`, `📜️script.ts:8328` region). The constant is in this
   partition but the number is a policy decision, not a root-script bug: the file is under continuous peer
   growth (2 299 → 2 300 → 2 302 across W2d and this pass) and 512 has been wrong for a long time. It also
   demands six `⚖️gate…` registrations that exist in neither catalog. I did not raise the number or add
   the six entries on my own judgement — the gate is red for reasons no row of this ticket owns, and
   quietly widening a capacity assertion to make it green is the wrong move. Coordinator's call whether
   this becomes a row.
4. **`schema entries` overwrites a tracked file.** That is deliberate (the dump is a committed reference,
   regenerated by the command that produces it) but it means the target is not idempotent with respect to
   the working tree: running it on a machine where fewer crates are linked shrinks the reference. If that
   turns out to matter, the fix is for the entries test binary to fail when it registers fewer scopes than
   the reference already contains — a change in `semio-framework-schema`, not here.
