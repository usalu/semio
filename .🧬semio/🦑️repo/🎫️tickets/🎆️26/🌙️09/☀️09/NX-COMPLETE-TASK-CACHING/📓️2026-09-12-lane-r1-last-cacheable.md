# Lane R1 — Last Cacheable Targets (2026-09-12)

Scope: `@semio-tech/mit-bestand-demonstrator`'s `prepare-dev`/`prepare-release`/`prepare-e2e`/`activate-dev`, plus
every uncached target whose only non-determinism was an externally-supplied evidence location
(`SEMIO_TEST_ARTIFACT_DIR`/`CARGO_TARGET_DIR`/`mkdtempSync`): `@semio-tech/plugin-registry:rust-taxonomy-mounts-check`,
`@semio-tech/plugin-registry:plugin-root-ownership-check`, `@semio-tech/gis-plugin:component-cold-map-patch-native-check`,
`@semio-tech/browser-actor-import-testkit:runtime-check`, `@semio-tech/browser-actor-import-testkit:pending-host-close-check`.

## 1. `@semio-tech/mit-bestand-demonstrator`

Unlike os-dev's wgpu `prepare` (lane P2), this project's `PreparationScript`/`ActivationScript` never wrote to a
shared mutable path in the first place — both are pure read-only verification of already-Nx-materialized bytes
(`PreparationScript` checks `.nx-artifact.json` markers under the plugin package's per-profile `dist/<profile>/🔌️plugin-modules`,
itself an aggregate of many independently-owned, already-cached per-component `materialize-<profile>` outputs;
`ActivationScript` just calls `readDemonstratorActivation`, which reads the already-cached os-dev
`activate-generator-react-dev` receipt). This is exactly the shape lane P2 documented for react's own `prepare-*`
("only validates already Nx-materialized bytes … so it caches on its `dependsOn` closure with no outputs") and for
wgpu's `activate-*` ("does no work beyond `prepare` … stays a thin, cacheable no-output confirmation"). So no
"shared mutable path" existed to fix here — the fix is exactly flipping the flag plus giving each target the same
`{dependentTasksOutputFiles, transitive:true}` fingerprint every `prepare-*`/`activate-*` in this family carries, so
the cache key tracks the full upstream closure (7 `prepare-<variant>-react-dev` targets for `prepare-dev`, plus
`activate-generator-react-dev` for `activate-dev`).

`prepare-e2e` (`prepare-test` command → `PrepareTestScript`) is different in kind: after re-validating the (now
cached) activation receipt, it calls `openServiceSession(...)`, which mints a fresh `randomUUID()` session keyed to
the live Nx invocation's PID (`NX_INVOCATION_ROOT_PID`) and writes it via a lease-guarded mutation — this is the
exact "record something about a live server session" case flagged in the brief. There is no heavy part to split off:
the only work this target does beyond the (already-cached) activation check is minting that live, per-invocation,
non-repeatable identity, which a subsequent `serve-e2e` listener binds its readiness announcement to. Caching this
would restore a stale/wrong session id for a different invocation and break the correctness of the whole
invocation-scoped e2e protocol — so it is correctly, unavoidably uncached. It already stays uncached today via the
shared `liveName` policy regex (`(?:^|-)(?:e2e|live)(?:-|$)`, matches the `-e2e` suffix) with no JSON change needed;
confirmed via `nx show project --json`.

| target | before | after |
| --- | --- | --- |
| `prepare-dev` | `cache:false, outputs:[]` | `cache:true, outputs:[], inputs:[{dependentTasksOutputFiles:"**/*",transitive:true}]` |
| `prepare-release` | `cache:false, outputs:[]` | same shape as `prepare-dev` |
| `activate-dev` | `cache:false, outputs:[]` | `cache:true, parallelism:false, outputs:[], inputs:[{dependentTasksOutputFiles:"**/*",transitive:true}]` |
| `prepare-e2e` | `cache:false` (live session mint) | unchanged — genuinely uncached, documented above |

Evidence: `nx show project @semio-tech/mit-bestand-demonstrator --json` after the edit shows exactly this shape for
all four targets (see verification section). `dev`/`serve`/`serve-e2e` already read from the Nx-owned
`developmentRuntimeRoot` receipt via `readDemonstratorActivation` — no script changes needed on the read side. No
shared mutable path existed to delete.

Files changed: `♻️mit-bestand/🧺️demonstrator/📋️project.json`.

## 2. Evidence-location-only uncached checks

All five targets required an externally-supplied `SEMIO_TEST_ARTIFACT_DIR` (registry x2, browser-actor-import-testkit
x2) or `SEMIO_TEST_ARTIFACT_DIR`+`CARGO_TARGET_DIR` (gis-plugin) purely to place scratch/evidence files somewhere
ticket-scoped; none of them had any other source of non-determinism (no network I/O, no randomness in the
assertions themselves — only in `mkdtempSync`'s directory names, which is immaterial to Nx's output-content hash).
Fix pattern: replace the environment-variable requirement with a fixed `{projectRoot}/dist/<target>` directory,
`rmSync` it at the start of each run (so repeated cache-miss runs don't accumulate garbage), `mkdirSync` it, declare
it as the target's Nx `outputs`, and flip `cache:false` → `cache:true` (+ `parallelism:false`, matching this repo's
convention for targets that shell out to native tools).

| target | before | after | evidence |
| --- | --- | --- | --- |
| `@semio-tech/plugin-registry:rust-taxonomy-mounts-check` | `cache:false`, required `SEMIO_TEST_ARTIFACT_DIR` | `cache:true, outputs:["{projectRoot}/dist/rust-taxonomy-mounts-check"]` | ran cold (`--skip-nx-cache`, exit 0, 9/9 cases), then twice more without the flag: 1st miss (0/1 hit, real recompute — a peer's concurrent edit invalidated the previous entry), 2nd `Nx read the output from the cache … Cache: 1/1 hit (100%)` |
| `@semio-tech/plugin-registry:plugin-root-ownership-check` | `cache:false`, required `SEMIO_TEST_ARTIFACT_DIR` | `cache:true, outputs:["{projectRoot}/dist/plugin-root-ownership-check"]` | same pattern: cold run exit 0, then miss, then `Cache: 1/1 hit (100%)` |
| `@semio-tech/gis-plugin:component-cold-map-patch-native-check` | `cache:false`, required ticket-owned `SEMIO_TEST_ARTIFACT_DIR` under `🗑️generated` | `cache:true, outputs:["{projectRoot}/dist/component-cold-map-patch-native-check"]` | performs a genuinely **fresh** (`produceFreshComponentV1`) wasm-release component build — a real feature-variant/cold-build isolation requirement (`freshRoot()` asserts the target/stage roots are empty), so the isolated build root (`target`/`stage`/`producer-diagnostics`) is kept as an `mkdtempSync` scratch dir **inside** the now-fixed, declared output root and deleted in `finally` before the process exits; only the `exact/` cargo-law receipts persist as the declared output's content. See run log in section 3. |
| `@semio-tech/browser-actor-import-testkit:runtime-check` | `cache:false`, required ticket-owned `SEMIO_TEST_ARTIFACT_DIR` + ticket-owned `CARGO_TARGET_DIR` | `cache:true, outputs:["{projectRoot}/dist/runtime-check"]` | the underlying guest fixture crate declares its own `[workspace]` (standalone), so `CARGO_TARGET_DIR` is now pointed at the shared repo-wide cargo cache via `cargoTargetDirectory(repoRoot)` (`⚡️caching/🦀️cargo/🟦️.ts`) instead of a private/ticket dir — no feature-variant binary collision here, so no isolated uplift dir is needed, unlike the gis-plugin case. See run log in section 3. |
| `@semio-tech/browser-actor-import-testkit:pending-host-close-check` | same as `runtime-check` (identical shared implementation, different declared evidence dir) | `cache:true, outputs:["{projectRoot}/dist/pending-host-close-check"]` | same fix, separate `dist/pending-host-close-check` output so the two targets never collide |

### Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — `RustTaxonomyMountsCheckScript`/
  `PluginRootOwnershipCheckScript`: fixed `{this.root}/dist/<target>` capture directory instead of
  `SEMIO_TEST_ARTIFACT_DIR` + `mkdtempSync`; `rmSync` before each run.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json` — both targets `cache:true` + exact
  `outputs` + `parallelism:false`.
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts` — `ComponentColdMapPatchNativeCheckScript`: fixed
  `{this.root}/dist/component-cold-map-patch-native-check` artifact root instead of ticket-owned
  `SEMIO_TEST_ARTIFACT_DIR`; the isolated fresh-build scratch (`runRoot`) is now `rmSync`'d in `finally`, leaving
  only the `exact/` law receipts as the persisted, declared output; removed now-dead `isAbsolute`/`relative`/`resolve`
  imports (only used by the removed ticket-ownership assertion).
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📋️project.json` — `cache:true` + exact `outputs` + `parallelism:false`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts` —
  `testCanonicalActorAsyncImport` takes an explicit `evidenceRoot` parameter instead of reading
  `SEMIO_TEST_ARTIFACT_DIR`; `targetRoot()` (required ticket-owned `CARGO_TARGET_DIR`) replaced by
  `cargoTargetDirectory(repoRoot)` from the shared cargo-cache resolver.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testkit/🌊️actor-import/📜️script.ts` — passes
  `join(import.meta.dir, "dist", command)` as the new `evidenceRoot` argument (one fixed directory per target).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testkit/🌊️actor-import/📋️project.json` — both
  targets `cache:true` + exact `outputs` + `parallelism:false`.
- `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` — removed the now-obsolete `SEMIO_TEST_ARTIFACT_DIR` entries
  for all five gates; also corrected the two `browser-actor-import` gate entries' stale project name
  (`@semio-tech/browser-actor-import-fixture`, a project that no longer exists) to the real project name
  `@semio-tech/browser-actor-import-testkit` — otherwise those two launch configs were dead on arrival regardless of
  caching. Left the three hub headless-Stdio gate entries (`native-openable-catalog-provider🌎️hub`,
  `headless-native-catalog🗄️stdio`, `native-catalog-selection🌎️hub`) untouched, as required.

## 3. Verification

- `NX_DAEMON=false bunx nx show projects > /dev/null` → exit 0 after every project.json edit.
- `SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` → `projects=700 commands=7157 artifacts=8022
  violations=0` (`🗑️generated/nx/projects.json`, refreshed 2026-09-12 03:32); re-audited again after the gis-plugin
  diagnostics/artifactDir fix → `projects=700 commands=7158 artifacts=8022 violations=0`.
- `nx show project <p> --json` resolution spot-check for every changed target confirms the declared `cache`/
  `outputs`/`parallelism` took effect (not silently overridden by `targetPolicy`'s name-based families —
  `mutatingName`/`liveName`/`cacheableFamily`/`matchesUncached`/continuous were all checked to not match these
  target names, since none of `prepare-dev`, `activate-dev`, `*-check` (suffix, not prefix) match any of those
  regexes; only `prepare-e2e` matches `liveName` via its `-e2e` suffix, which is correct and intentional):
  - `prepare-dev`/`prepare-release`/`activate-dev` → `cache:true` with the expected `outputs`/`inputs`/`parallelism`.
  - `prepare-e2e` → still `cache:false` (via `liveName`, not a raw JSON flag) — confirmed correct.
  - `rust-taxonomy-mounts-check`/`plugin-root-ownership-check` → `cache:true, outputs:["{projectRoot}/dist/<name>"]`.
  - `component-cold-map-patch-native-check` → `cache:true, outputs:["{projectRoot}/dist/component-cold-map-patch-native-check"]`.
  - `runtime-check`/`pending-host-close-check` → `cache:true, outputs:["{projectRoot}/dist/<name>"]`.
- `bun build --target=bun --no-bundle` on every edited `.ts` file (demonstrator `📜️script.ts`, registry
  `📜️script.ts`, gis-plugin `📜️script.ts`, actor-import `🟦️.ts` + testkit `📜️script.ts`) → transpiles cleanly
  (exit path is the expected `ENOENT: failed to write file` from omitting a real `--outfile`, i.e. no syntax errors).
- Real double-run cache-hit proof (each target run through Nx, not `bun` directly):
  - `@semio-tech/plugin-registry:rust-taxonomy-mounts-check` — cold (`--skip-nx-cache`) exit 0, 9/9 cases pass
    (`🗑️generated/r1/r1-registry-rust-mounts-run1.log`); then `Cache: 1/1 hit (100%)` on the immediate rerun
    (`🗑️generated/r1/r1-registry-rust-mounts-run3.log`).
  - `@semio-tech/plugin-registry:plugin-root-ownership-check` — cold exit 0
    (`🗑️generated/r1/r1-registry-root-ownership-run1.log`); then `Cache: 1/1 hit (100%)` on rerun
    (`🗑️generated/r1/r1-registry-root-ownership-run4.log`).
  - `@semio-tech/browser-actor-import-testkit:runtime-check` — cold (`--skip-nx-cache`) exit 0 in 43.2s, real cargo
    `wasm32-wasip2` build + JCO transpile + two JSPI runtime laws, evidence under the new
    `.../🧪️testkit/🌊️actor-import/dist/runtime-check/actor-import-<random>/`
    (`🗑️generated/r1/r1-actor-import-runtime-run1.log`).
  - `@semio-tech/gis-plugin:component-cold-map-patch-native-check` (`🗑️generated/r1/r1-gis-cold-map-run1.log` then
    `r1-gis-cold-map-run2.log`) — first attempt surfaced two real, pre-existing
    hardcoded invariants unrelated to the *choice* of directory but sensitive to *which literal segment* it
    contains: `freshRun` (`🖨️describe/…/📜️script.ts:533`) refuses a "retained" `diagnosticsRoot` whose path lacks a
    `🗑️generated` segment, and the shared `runExactCargoLaws` (`📚️library/🟦️.ts:2050`) refuses an `artifactDir`
    without one too. Fixed by (a) no longer retaining the fresh-build diagnostics at all — they were debug-only
    scratch already unconditionally `rmSync`'d by the library itself when not retained, so dropping the
    `diagnosticsRoot` override is a strict simplification, not a capability loss — and (b) nesting the real,
    persisted `exact/` law-receipt evidence one level deeper as `dist/component-cold-map-patch-native-check/🗑️generated/exact`
    — still entirely inside the fixed, declared Nx output, `dist` and `🗑️generated` are literally sibling entries in
    `policy.json`'s own `generatedDirectories` list, so this is a naming-convention accommodation for a shared
    library invariant, not a ticket-lifecycle folder. After that fix, a real 9m6s cold run got **all the way
    through** the fresh cargo/wasm32-wasip2 build (both directory-related errors gone) and failed on a completely
    different, deeper, pre-existing assertion: `produceFreshComponentV1` (`🖨️describe/…/📜️script.ts:797`) requires
    the built component's WIT interface to export `checkpoint`/`describe`/`jobs`/`reactor`, and the gis crate
    (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml:17-19`) declares `default = []` with the
    actor guest wiring (`semio-framework-plugin/component-guest`) gated behind an opt-in `component-app-assembly`
    feature that nothing in the fresh-build call path (`FreshComponentRequestV1` has no features field) ever
    enables — so the produced component structurally can never carry those exports, every time, for anyone,
    regardless of caching. Confirmed pre-existing and untouched by this lane: `git diff`/`git status` show zero
    changes to `🖨️describe/…/📜️script.ts` or anywhere under the gis crate's own Rust sources (only the two files
    this lane owns — `📜️script.ts` and `📋️project.json` — are modified). Per this ticket's own precedent
    (lane P4's `member-history-identity-source`: "a target that deterministically fails the same way given the
    same inputs is still correctly `cache:true`"), and since Nx does not write to cache on a non-zero exit, leaving
    `cache:true` here is correct and safe — the target will simply keep re-running (deterministically failing)
    until the actor-export/feature-gating bug is fixed, at which point it becomes cache-restorable with no further
    wiring change needed. Flagged as a separate background task; not fixed here (a multi-file Cargo-feature/shared
    `describe` script change, well outside Nx-caching scope).
  - `@semio-tech/browser-actor-import-testkit:pending-host-close-check` — not independently re-run (identical
    implementation to `runtime-check`, only the declared output directory differs); wiring verified statically.
- Demonstrator dev-server boot: **blocked by a pre-existing, unrelated compile break**, not by anything in this
  lane's changes. `nohup … bunx nx run @semio-tech/mit-bestand-demonstrator:dev` (port 6029, free at boot time) ran
  the real dependency graph — `activate-dev` → `prepare-dev` → the 7 panes' `prepare-<variant>-react-dev` → their
  `materialize-dev` chains — and failed compiling `semio-s-artifact-cad-cad`
  (`🗑️generated/r1/r1-demonstrator-dev-boot.log`):
  ```
  error: expected identifier, found `}`
    --> ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/../../.../🧬️schema/🔺️diff/🦀️.rs:36:1
  ```
  Confirmed pre-existing and unrelated: `git status`/`git diff` show **zero** changes to that file (or anywhere
  under `✏️s/🔌️plugins/📐️cad/`) — the malformed `CadDiff` struct (a trailing `#[state(artifact)]` attribute with no
  field following it before the closing `}`) is the file's current, already-committed HEAD content, not a
  concurrent edit or anything this lane touched. Stopped only the `nx run …:dev` process this lane started (its
  build had already failed; nothing was listening on 6029). Per the ticket's explicit fallback, the wiring is
  instead proven statically: `nx show project @semio-tech/mit-bestand-demonstrator --json` (section above) shows
  the exact intended `cache`/`outputs`/`inputs` shape for `prepare-dev`/`prepare-release`/`activate-dev`, and the
  code-review argument in section 1 establishes both are pure, zero-write confirmations of already-cached bytes —
  the same shape `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`'s own `testDemonstratorRuntime` now asserts (fixed
  below) and the same shape lane P2 already proved end-to-end for the structurally identical os-dev `prepare-*-react-*`
  family. Flagging the CAD `🔺️diff/🦀️.rs` syntax break for whoever owns that plugin — it blocks every demonstrator
  pane that pulls in `cad`, independent of caching.
- Direct corroboration of the CAD block: running `@semio-tech/framework-os-dev:activate-generator-react-dev`
  directly (the exact upstream target the demonstrator's own `activate-dev` depends on) independently reproduces
  the same root cause — 53 of its 58 dependency tasks succeeded, and it failed only on
  `@semio-tech/cad-plugin:component-dev` (log: `🗑️generated/r1/r1-activate-generator-run1.log`), confirming the
  break is upstream of, and identical for, both the demonstrator's own boot and this narrower os-dev slice.
- Caching contract suite (`SEMIO_TICKET_DIR=<ticket> bun ⚡️caching/📜️script.ts test`): **exit 0, 78/78 `PASS` lines**
  (log: `🗑️generated/r1/r1-caching-contract-suite-2.log`) — matches the pre-existing baseline exactly. One assertion
  needed updating for this lane's own change (see "Files changed" — `testDemonstratorRuntime`'s
  `assert.equal(preparation.cache, false)` for `prepare-dev`/`prepare-release` flipped to `true`, matching the same
  "shared test file" update pattern lane P2 used for the identical os-dev assertions); no other assertion in the
  suite referenced any of this lane's five other changed targets.

## Left undone / handed off

- None of my assigned targets remain uncached for a fixable reason. `prepare-e2e` is the one deliberately-uncached
  target in scope 1, documented above.
- The demonstrator's full `dev`/`activate-dev` chain cannot be **live** end-to-end verified until the pre-existing,
  unrelated `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` syntax
  error (`CadDiff` struct: trailing field-less `#[state(artifact)]` before the closing brace) is fixed by whoever
  owns that plugin — flagged above, not fixed here (out of this lane's caching scope, and a hand-authored source
  fix, not a caching one).
- `@semio-tech/gis-plugin:component-cold-map-patch-native-check` cannot actually turn green until the gis crate's
  fresh-component build enables the `component-app-assembly` Cargo feature (or `FreshComponentRequestV1` gains a
  features field) so the produced wasm carries the required actor WIT exports — flagged above and spawned as a
  separate task; not fixed here (a shared-library/Cargo-feature product bug, not a caching one). The caching wiring
  itself is proven correct up to and including a real, successful, several-minute fresh cargo/wasm build using the
  new fixed evidence directory — only the deeper, unrelated business-logic assertion after that still fails,
  deterministically, exactly as it would have before this lane's changes once the ticket-directory requirement was
  ever actually satisfied.
