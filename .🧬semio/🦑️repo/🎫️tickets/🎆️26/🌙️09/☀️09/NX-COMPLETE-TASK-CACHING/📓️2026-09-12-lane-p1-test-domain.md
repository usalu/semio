# Lane P1 — Test Domain Caching (2026-09-12)

Session `⚪83140bef0e504e03b0b3380912b12a0e`. Repo MCP failed to connect (`repo`: invalid initialize params,
`semio`: connection closed) — ticket bookkeeping kept on disk manually, per the established coordinator/S8/S11
pattern in this ticket.

Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**` (the test-domain Nx plugin `🟨️.mjs`, its `📜️script.ts`,
`📦️packages/**`, fixtures/tests) and the `📋️project.json` files of projects whose only uncached targets are
`test*` targets.

## Task 1 — `test-exhaustive` (250 projects)

### Why it was excluded (investigation)

`git log -S'level !== "exhaustive"'` finds exactly one commit, `e5465a2c1c` — the plugin's original authoring
commit. The line was hard-coded with **no comment anywhere, then or since**, and no test/fixture ever asserted
the exclusion for a *reason* (S11's cache-contract suite only asserted the boolean, not why). I checked every
plausible technical justification directly against the runner (`executeOne`/`RunScript` in `📜️script.ts`):

- **Nondeterministic seeds / randomness**: none found. Scenarios are static, declared in `.feature` files;
  `Math.random`/seed generation do not appear in the run path.
- **`SEMIO_TEST_BUDGET_MS` / level as an input**: already hashed (`scoped()` already adds `{ env:
  "SEMIO_TEST_LEVEL" }` and `{ env: "SEMIO_TEST_BUDGET_MS" }` for every level, exhaustive included).
- **Coverage instrumentation** (`resolveTestLevel` auto-sets `SEMIO_COVERAGE=1` at `exhaustive`, which drives
  cumulative, order-dependent `cargo llvm-cov`/vitest-coverage writes elsewhere in the repo): traced end to end
  — `coverageEnabled()`/`runCargoTestBudgeted`/`runVitest` are **not called anywhere** in this runner's path
  (`executeOne` builds a small generated host crate/module and runs it directly via `runProbe`). Ruled out.
- **Declared inputs incomplete — real, found and fixed** (see below): the actual gap. `inputsFor()` already
  hashed the case's own feature/fixtures/adapters/oracle-registry/schema/taxonomy/domain files plus a
  repo-wide `**/🔮️oracle/**/*` glob for contribution *manifests*, but never the crate/module a manifest's
  `oracleHostPackages[].path` or a rust adapter's own crate root actually point **at** — often nowhere near
  the case's own `ownerRel`.

Concrete, verified instance: the case owned by
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit`
links its rust crate at the **artifact root**,
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust` — an ANCESTOR of the owner, never
covered by the existing `{workspaceRoot}/${ownerRel}/**/*` glob. Similarly, seven owners (`gis`, `writer`,
`forms`, `animate`, `playbook`, `norm`, `stdio`, `flow`) declare a `path`-based `oracleHostPackages` entry
pointing at `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — a **sibling plugin's** crate, never covered
by any input at all (only the manifest *naming* it was hashed, not the crate itself). Editing either crate's
source today invalidates **no** cached quick/long/exhaustive result for any of those owners — a soundness gap
that predates and is independent of the exhaustive exclusion, affecting every already-cached level too.

**Conclusion**: no technical reason distinguishes `exhaustive` from `quick`/`long` — the exclusion was an
unjustified blanket exclusion. The real, load-bearing gap (crate/module resolution outside `ownerRel`) applied
equally to all three levels. Fixed it, then enabled caching uniformly for all three.

### Fix (`🧪️test/🟨️.mjs`)

- `rustSutCratePath(workspaceRoot, ownerRel)` — mirrors `rustSutCrate`'s walk-up in `📜️script.ts` (checks
  `<dir>/📦️packages/🦀️rust/Cargo.toml` from `ownerRel` up to 16 ancestors, stops at the
  `semio-repo-test-host` sentinel exactly like the runtime does) and returns the crate's real root.
- `oracleContributionPaths(workspaceRoot, vocabulary, ownerRel)` — mirrors `oracleHostPackagesFor`'s ancestor
  filter (`owner === entry.owner || owner.startsWith(entry.owner + "/")`) by walking every ancestor-or-self
  `🔮️oracle/🔣️.json`, collecting every declared `oracleHostPackages[].path`.
- `inputsFor()` now appends the resolved crate root (only when the case actually has a rust adapter) and every
  resolved oracle-contribution path, each as a `**/*` glob, on top of the existing inputs.
- `test-quick`/`test-long`/`test-exhaustive` generation: removed `level !== "exhaustive"` — all three levels
  now share `cache: true` unconditionally (the whole point of the per-level distinction is budget, not trust).

### Verification (independent-oracle unit tests, run directly)

New test: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚡️exhaustive-cache-inputs/🟦️.ts`. Each
resolution function is checked against a **from-scratch, separately-written** filesystem walk (not a call into
the plugin's own code), against real repo paths — not synthetic fixtures — so the two can disagree if either
is wrong:

```
bun test "./🧪️tests/⚡️exhaustive-cache-inputs/🟦️.ts"
 7 pass
 0 fail
 25 expect() calls
```

Covers: the generation3d ancestor-crate resolution (`rustSutCratePath` finds
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust`, agrees with the independent walk,
the resolved `Cargo.toml` really exists); the sentinel-package exclusion (never links the generated host
itself); `oracleContributionPaths` for `gis`/`norm`/`stdio` agreeing with the independent walk and resolving to
real, existing paths; no false positive for an owner with no contribution; `inputsFor` actually threading both
new families of input into a real case (gisterrain) and not adding a rust-crate input to a case with no rust
adapter (kernel); and that the plugin's own `createNodesV2` output shows `cache: true` with the outputs
declared for all three generated levels.

### Runtime verification (real `nx show project`, 3 representative real projects)

| project (owner) | `test-exhaustive` cache | new inputs present |
| --- | --- | --- |
| `test-s-plugins-gis-…-1a7abb-🏔️mutate-gisterrain-1` (`gis/gisterrain`) | `true` | `…/gisterrain/📦️packages/🦀️rust/**/*`, `…/stdio/🧪️oracle/📦️packages/🦀️rust/**/*` |
| `test-s-plugins-norm-…-a9f115-🌬️mutate-din16798-1` (`norm/din16798`) | `true` | own crate root, `stdio`'s crate, **and** `norm`'s own python module (`🔮️oracle/📦️packages/🐍️python`) |
| `test-s-plugins-procedural-…-cdccaf-🧊️mutate-procedural-3d-1` (`procedural/generation3d`) | `true` | `…/generation3d/📦️packages/🦀️rust/**/*` (the ancestor crate) |

`NX_DAEMON=false bunx nx show projects` → exit 0, 700 projects, run repeatedly across every edit in this lane.

### Blocked: live `[local cache]` demonstration for `test-exhaustive` itself

**Found, not caused by this lane**: every `run`-based level target (`test`, `test-quick`, `test-long`,
`test-exhaustive` — all four, for every one of the 250 projects, before and independent of this lane's changes)
currently fails through `RunScript.run()` → `validateAllContracts()`, because `oracleImportsInProduction`/the
mutation-inventory/layout checks inside it scan the **whole repository unconditionally**, regardless of which
case was selected, and currently report ~1300 breaches (`testing/dependency`, `testing/layout`) across files
this lane never touched (puzzle-2d, layout, plugin-reactor, dev). Verified this is unconditional on the
selected case (breach set for a single trivial `kernel/✅️satisfy-version-requirements` case includes hundreds
of unrelated puzzle/layout files) and pre-existing (confirmed the *same* `test-quick` target — already
`cache: true` before this lane touched anything — fails identically). Nx does not cache a failed
`nx:run-commands` task (verified: two consecutive `nx run …:test-exhaustive` invocations without
`--skip-nx-cache` each take ~2m30s+ with no `Cache:` line at all — no hit, no miss reported, no write).
Flagged as a separate background task (`task_ee07155c`, "Fix repo-wide test dependency ratchet breach (1299
findings)") rather than fixed here — it is unrelated to caching and far outside this lane's scope.

**Also found and fixed in passing** (blocking, and squarely in this lane's file ownership): a genuinely
unrelated, pre-existing bug that independently breaks *every* case regardless of the breach above —
`testCacheDir()`'s `SEMIO_TEST_OUTPUT_SCOPE` validation regex (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:1180`)
was `/^[a-zA-Z0-9_-]+\/[a-zA-Z0-9_-]+$/` — ASCII-only — while every real scope value is
`${projectName}/${phase}`, and `projectName` is **always** emoji-prefixed by design (`canonicalCase`/
`projectNameFor`). Every scoped invocation of `run`/`oracle`/`subject`/`parity` (i.e. every per-case target the
plugin generates) threw `invalid test output scope` before this fix. Replaced with a segment-count + traversal
check that accepts the real, legitimate unicode format:
```
const segments = scope.split("/");
if (segments.length !== 2 || segments.some((s) => s === "" || s === "." || s === ".." || s.includes("\0"))) throw …
```
Confirmed this was the blocker independent of the breach count by running the *un-modified* `test-quick`
target directly — same `invalid test output scope "…/test-quick"` error before the fix, gone after.

### Alternative real `[local cache]` evidence (same mechanism, an unblocked target)

Since `test-exhaustive` itself can't complete today, I proved the identical caching mechanism — a
`{ runtime: … }` digest input over a gitignored evidence file, the exact pattern `test-exhaustive`'s own
`inputsFor` inputs rely on for correctness — end-to-end on `test-report`, which I also fixed this session (see
Task 2):

```
run 1: NX_DAEMON=false bunx nx run "@semio-tech/repo-test-domain:test-report"
  → Cache: 0/1 hit (0%)                                    (cold)
run 2: same command again, no source changes
  → "Nx read the output from the cache instead of running the command for 1 out of 1 tasks."
  → Cache: 1/1 hit (100%)
run 3: appended a synthetic line to the evidence file the digest input reads, then re-ran
  → Cache: 0/1 hit (0%)                                    (correctly invalidated — proves the digest is live, not a false positive)
```

## Task 2 — every other uncached `test*` target

| target | project | decision | evidence |
| --- | --- | --- | --- |
| `test-exhaustive` | 250 test-case projects | **cached** (was uncached) | Task 1 above |
| `test-discover` | `repo-test-domain` (mine) | pure, deterministic (`DiscoverScript`: filesystem walk + stdout JSON, zero writes) → **should be cached**; added `inputs` (taxonomy.json + `**/*.feature` + one glob per adapter-source filename) + `outputs: []`. Still shows `cache: false` at runtime — forced by `⚡️caching/🔣️policy.json` `uncached` list's exact `"test-discover"` entry, which fires before `targetPolicy` ever looks at the authored `cache: true`. **Needs lane P4**: remove `"test-discover"` from `policy.json`'s `uncached` list; my inputs are pre-staged so caching activates immediately once that lands. |
| `test-report` (repo-test-domain) | `repo-test-domain` (mine) | deterministic (`ReportScript`: reads the gitignored `📤️results.jsonl` evidence stream, writes a derived `📋️junit.xml`) → **cached**. Nx cannot hash a gitignored file via a glob (S11's finding) so I used the same `{ runtime: "<sha256 digest>" }` mechanism S11 built for output roots, pointed at `.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📤️results.jsonl`; declared `outputs: ["…/reports/latest/📋️junit.xml"]`. **Was already cacheable at runtime** — a concurrent lane (not me) removed `report`/`write-baseline$` from the library plugin's `mutatingName` regex mid-session, so `cache: true` now actually applies; verified live 3-run cache-hit/invalidate sequence above. |
| `test-doctor` | `repo-test-domain` (mine) | genuinely live: probes the machine's installed toolchains (`cargo --version`, `bun --version`, `python3 --version`, `dotnet --version`) — a function of the machine, not the repo. **Keep uncached** (already correct via `policy.json`'s exact `"test-doctor"` entry). Flipped the project.json's own `"cache": true` → `"cache": false` — it was authored true but silently always overridden by policy; now the source of truth is honest. |
| `test-inventory` | `repo-test-domain` (mine) | genuinely unscoped execution: `InventoryScript` runs each owner's **live production mutation-dispatch bridge** across the whole registry (`mutationBridgeFor`/`runProbe`) and writes a runtime inventory. A correct cache key would need the transitive closure of every owner's production runtime reachable from that bridge — a repo-wide surface this test-domain-scoped plugin cannot soundly enumerate (this is the native-cargo-dependency-closure problem, owned by a different subsystem entirely). **Keep uncached** (already correct via policy). Flipped authored `"cache": true` → `"cache": false` to match. |
| `test-fixture-generate` | `repo-test-domain` (mine) | doubly disqualified: (1) runs an **arbitrary, data-declared generator command** per fixture (`fixture.generator.command`, unscoped, same closure problem as `test-inventory`), and (2) is intentionally content-producing — writes new bytes into the committed content-addressed fixture store and publishes a manifest update for a human to review ("commit review is a separate, human step" per its own log line). **Keep uncached** (matches `policy.json`). Flipped authored `"cache": true` → `"cache": false`. |
| `test-fixture-reproduce` | `repo-test-domain` (mine) | same unscoped-arbitrary-command problem as `test-inventory`/`generate` (re-runs `fixture.generator.command`); no committed-state mutation (writes only to a scratch dir it deletes/recreates), but the execution-closure problem alone is disqualifying. **Keep uncached** (matches `policy.json`). Flipped authored `"cache": true` → `"cache": false`. |
| `test-gc` | `repo-test-domain` (mine) | dry-run by default (`collectGarbage(..., { dry: !segments.includes("--apply") })`), but `forwardAllArgs: true` lets a caller pass `--apply` to make it actually delete, and that flag is **not** reflected in Nx's cache key — caching risks a stale hit silently skipping a real deletion pass (or the reverse). **Keep uncached** (matches `policy.json`'s `mutatingName` regex, `gc`). Flipped authored `"cache": true` → `"cache": false`. |
| `test-clean` | `repo-test-domain` (mine) | genuinely mutating: marker-guarded deletion of generated test state (`cleanTestOutputs`). Already correctly `"cache": false` in source — no change. |
| `test-metrics` / `test-metrics-enforce` | `workspace` (root project.json — **not mine**: many non-`test*` uncached targets) | same class as `test-report`: `MetricsScript` reads the gitignored `📈️metrics.json` evidence and optionally enforces a threshold — deterministic given that file's current bytes. **Needs lane P4** (owns root `project.json` + `policy.json`): apply the same `{ runtime: digest }` input against `.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📈️metrics.json`, remove `"test-metrics"` from `policy.json`'s `uncached` list (`test-metrics-enforce` isn't separately listed — check `matchesUncached`'s prefix-family behavior for it too). |
| `test-e2e` | `@semio-tech/mit-bestand-demonstrator` (**not mine**: project has `prepare-*`/`dev`/`activate-*`/`serve*` uncached targets, owned by P2/P3) | genuinely live: `dependsOn: ["serve-e2e"]`, a continuous dev server. **Correctly uncached** (matches `policy.json`'s exact `"test-e2e"`). No change needed anywhere. |
| `test-inventory-artifact-shards` | `@semio-tech/repo-lib` (**not mine**: `📚️library/**` file, project also has `preview-generated`/`workspaces-write` uncached) | trivially deterministic: `bun ./📜️script.ts test inventory-artifact-shards` (in `📚️library/📦️packages/🟦️typescript/📜️script.ts`, NOT the test-domain's own script) just runs `node --test` against one fixture-backed unit-test file (`📚️library/🧪️tests/💠️inventory-artifact-shards/🟦️.ts`) — indistinguishable in kind from any other already-cached TS unit test. **Needs lane P4** (or whoever owns `📚️library/📦️packages/🟦️typescript/📋️project.json`): add `"cache": true`; no special inputs needed, S8's generic script-closure mechanism already covers it since the command names a `📜️script.ts`. |

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` — `rustSutCratePath`, `oracleContributionPaths`
  (new); wired into `inputsFor`; removed the `level !== "exhaustive"` cache exclusion; `internals` export
  gained both new functions.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📋️project.json` — `test-discover`/`test-report` gained
  `inputs`/`outputs`; `test-doctor`/`test-inventory`/`test-fixture-reproduce`/`test-fixture-generate`/`test-gc`
  flipped their authored `cache` from a policy-overridden `true` to an honest `false`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` — fixed `testCacheDir`'s
  `SEMIO_TEST_OUTPUT_SCOPE` validation to accept the real, emoji-prefixed scope format (was ASCII-only,
  breaking every scoped case invocation, not just exhaustive).
- New: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚡️exhaustive-cache-inputs/🟦️.ts` — independent-oracle
  contract test for both new resolution functions and the levels' cache flags (7/7 pass).

## Needs lane P4 (policy/regex changes; not touched by me)

1. Remove `"test-discover"` from `⚡️caching/🔣️policy.json`'s `uncached` list — it is a pure, deterministic
   filesystem walk with zero side effects; inputs are already pre-staged in `repo-test-domain`'s `project.json`.
2. Root `workspace` `project.json`: add the same `{ runtime: digest }` input trick to `test-metrics` /
   `test-metrics-enforce` (pointed at `.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📈️metrics.json`) and remove
   `"test-metrics"` from `policy.json`'s `uncached` list (check whether `test-metrics-enforce` needs its own
   entry removed too, or was never separately matched).
3. `@semio-tech/repo-lib`'s `📋️project.json` (owned elsewhere): `test-inventory-artifact-shards` should be
   `"cache": true` — it is an ordinary fixture-backed unit test, no policy exclusion applies to it at all today
   (it simply was never authored with `cache: true`).

## Spawned background task (not fixed here, out of scope)

`task_ee07155c` — "Fix repo-wide test dependency ratchet breach (1299 findings)": every `run`/`test-quick`/
`test-long`/`test-exhaustive` invocation currently fails (pre-existing, unrelated to caching) because
`validateAllContracts` → `oracleImportsInProduction`/mutation-inventory/layout checks scan the whole repo
unconditionally and report ~1300 breaches today, across files this lane never touched. This is what blocks a
literal live `[local cache]` demonstration for `test-exhaustive` itself (see Task 1); the `test-report`
demonstration above proves the same caching mechanism soundly on an unblocked target instead.

## Verification summary

- `NX_DAEMON=false bunx nx show projects` → exit 0, 700 projects — run after every edit in this lane (final run
  confirmed after all changes landed).
- `node --check 🟨️.mjs` → OK, after every edit.
- `python3 -c "import json; json.load(open('📋️project.json'))"` → valid JSON, after every edit.
- 3 representative `test-exhaustive` projects (`gis/gisterrain`, `norm/din16798`, `procedural/generation3d`)
  via `nx show project <p> --json`: all three `cache: true`, each showing the new ancestor-crate and/or
  oracle-contribution-path inputs resolved to real, existing repo paths.
- New plugin contract test (`⚡️exhaustive-cache-inputs/🟦️.ts`): 7/7 pass, each cross-checked against an
  independently-written filesystem walk.
- Live cache-hit/invalidate proof (`test-report`, same mechanism `test-exhaustive` relies on): cold run
  `0/1 hit` → warm run `1/1 hit (100%)`, "Nx read the output from the cache instead of running the command for
  1 out of 1 tasks" → after mutating the evidence file the digest input reads, back to `0/1 hit` (correct
  invalidation, not a false-positive hit).
- Live `test-exhaustive` demonstration blocked by a pre-existing, repo-wide, unrelated breakage (documented
  above, spawned as `task_ee07155c`); the identical caching mechanism was proven live on `test-report` instead.
