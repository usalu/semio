# Coordinator Log (2026-09-11, session ⚪83140bef0e504e03b0b3380912b12a0e)

Plan: `📓️2026-09-11-single-shared-cache-plan.md`. Exploration reports: `📓️2026-09-11-explore-*.md` (8 Haiku lanes).
Execution reports: `📓️2026-09-11-lane-s*.md` (Sonnet lanes S1–S10).

## Wave A (coordinator)

- `.cargo/config.toml`: `build.target-dir`/`build.build-dir` under `.🧬semio/🦑️repo/⚡️cache/cargo/`,
  `[unstable] fine-grain-locking / build-dir-new-layout / checksum-freshness`, sccache wrapper removed.
- `nx.json`: `cacheDirectory` → `.🧬semio/🦑️repo/⚡️cache/nx` (564 entries moved with `mv`, hits preserved:
  `semio-framework-hash:build` rerun printed `[local cache]` / "Nx read the output from the cache … 1 out of 1 tasks";
  `.nx/cache` was not recreated), `maxCacheSize` 16GB.
- `policy.json`: cargo hash env −`CARGO_TARGET_DIR`, +`CARGO_PROFILE_WASM_{DEV,RELEASE}_DEBUG`.
- New resolvers: `⚡️caching/🦀️cargo/🟦️.ts` (`cargoDirectories`, `cargoTargetDirectory`, `cargoBuildDirectory`),
  `⚡️caching/🟦️.ts` (`repoCacheDirectory`). Runtime-checked: default → `⚡️cache/cargo/{target,build}`,
  `CARGO_TARGET_DIR=x/y` → target `x/y`, build stays shared.
- Hot paths switched immediately (they would otherwise have read stale wasm from the old `target/`):
  dev `buildPluginCargo`, describe `cargoTargetRoot`.

## Review fixes applied by the coordinator

1. **`clean` would have deleted source folders** (lane S2): `cleanIsCargoTargetDirName` used
   `startsWith("🎯️target")`, matching the taxonomy folder `🎯️targets`; combined with the new "always stray" rule a
   real `clean` deletes source trees. Fixed: exact names + Cargo's `CACHEDIR.TAG` required (`cleanIsCargoTargetDir`).
   Dry run: 97 → 54 rows, 0 `🎯️targets`.
2. **Whole-workspace Nx graph outage** (lane S8's generic command-closure hashing): one unresolvable import in
   `📓️print/…/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` (broken since 2026-09-09) made
   `relativeScriptInputs` throw for every project → `nx show projects` exit 1 for all sessions. Fixed: an unresolvable
   specifier contributes no file (the importing file stays hashed, so repairing the import still changes the key).
   `NX_DAEMON=false bunx nx show projects` → exit 0.
3. **Pruning consolidated into one command**: lane S7 added `cache-report`/`cache-prune` next to the pre-existing
   `disk-prune` (test-evidence retention under the same cache root). `disk-prune` (script class, Nx target,
   policy uncached entry) removed; `cache-report`/`cache-prune` now also run test-evidence retention
   (`storage.tests.unusedAgeMs`, schema-first in `🧬️schema/🔣️.json` + `policy.json`).
4. **`clean` → `cache-prune` interface mismatch** (S2 called `cache prune --apply`, S7 implemented `cache-prune --dry-run`):
   fixed in root `📜️script.ts` `CleanScript.runCachePrune`. `clean --dry` → `[cache-prune] would delete 0 units` /
   `[clean] cache-prune ok`.

## Deletions (all regenerable Cargo caches; `lsof` showed no process holding them)

| path | size | why |
| --- | ---: | --- |
| `target/` | 74 G | old shared target; nothing writes there since Wave A |
| `.🧬semio/🦑️repo/⚡️cache/cargo/browser` | 54 G | second private target (`wasmBuildEnvironment`), removed by S1 |
| `.🧬semio/🦑️repo/⚡️cache/agents/local/cargo-test-hosts` | 7.2 G | per-agent test-host target, removed by S1 |
| 54 per-package `target*` dirs (all with `CACHEDIR.TAG`) | 28.8 G | standalone workspaces now build into the shared dirs |
| `agents/local`, `agents/plugin-host-lifecycle-sol` | 75 M | first real `cache-prune` run on the live cache |

Relocated (not deleted): `node_modules/.cache/ms-playwright` → `⚡️cache/tools/ms-playwright`.

Not touched (not ours; surfaced to the dev): session scratchpads `9f5f6952…` (186 G) and `9e1e818a…` (58 G) — private
cargo targets of dead sessions last written 2026-09-10 12:44; `~/Library/Caches/Mozilla.sccache` (10 G, user-level,
no longer used by this repo).

## Verification evidence

- Fine-grain locking probes: `🐚️fine-grain-lock-probe.sh` (see plan "Key evidence").
- Pruner: `testCachePrune` → planner matches fixture + Python oracle; real-disk scan/cancel/delete PASS.
- Live `cache-report`: cargo 35.35 GiB / 2110 units, nothing inside the 48 h guard scheduled.
- Live `cache-prune`: deleted 2 stale agent units (74.68 MiB), lease store kept, test GC ran.
- Concurrent package builds through Nx on the real shared build-dir:
  `bunx nx run-many -t check -p @semio-tech/framework-os-kernel,@semio-tech/ui-rs --parallel=2` → exit 0 in 107 s,
  both projects succeeded, **zero** `Blocking waiting for file lock` lines (previously one directory lock serialised them).
- Package-level cache hit: rerun `bunx nx run @semio-tech/framework-os-kernel:check` →
  `[existing outputs match the cache, left as is]`, "Nx read the output from the cache … 1 out of 1 tasks".
- Finding: `@semio-tech/ui-rs:check` is a generator `checkTarget` freshness guard and stays `cache: false` by design
  (🟨️.mjs ~652/674) because generated outputs are excluded from `default`; follow-up lane makes guards cacheable with
  the generated outputs as explicit inputs.

## Review fix 5: legitimate private target dirs restored (after lane S10)

Lane S6 stripped every `CARGO_TARGET_DIR` from `.vscode/launch.json` / `🧩️launch.seed.jsonc`, including the three
headless-Stdio gate entries that `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `proveHeadlessStdioLaunchIsolation` requires
(`CARGO_TARGET_DIR = ${SEMIO_TEST_ARTIFACT_DIR}/cargo-target`). That isolation is legitimate under the shared cache: the
gates build `--features test-support` binary variants whose UPLIFTED deliverables share names, so a shared target-dir
lets a concurrent variant replace the binary before it runs; a per-command target-dir holds only those small uplifted
files while all intermediates stay in the shared build-dir (disk stays bounded). Restored the three entries in both
files with `🧩️restore-headless-gate-target-dirs.mjs`; both files still parse as JSONC (2482 / 1392 configurations);
the transpiled law `proveHeadlessStdioLaunchIsolation(repoRoot)` passes.

Rule going forward: a private `CARGO_TARGET_DIR` is allowed only to isolate uplifted feature-variant deliverables of one
command; it never needs `RUSTC_WRAPPER`, never a private build-dir, and must live in a ticket/`🗑️generated` root.

## Review fix 6: last audit findings + cached git/launcher commands (after lane S9)

S9 took `repo:audit` from 322 CACHE-06 findings to 13 (all in lanes S2/S3 files). Closing them exposed a real defect:
root router commands `commit`, `micro-commit`, `os`, `semio`, `scale-fixture` resolved as **cached** (policy default
branch), so a cache hit would have skipped a git commit or a nested `generate`. `matchesUncached` treated every policy
entry as a prefix family except a hard-coded `format`; adding `os` would have uncached every `os-*` target.

- Schema-first: `⚡️caching/🧬️schema/🔣️.json` + `policy.json` gain `uncachedExact` (`commit`, `format`, `micro-commit`,
  `os`, `parity`, `scale-fixture`, `semio`); `format` moved out of the prefix list.
- `🟨️.mjs` `matchesUncached(name, policy)` = exact list ∪ prefix families (hard-coded `format` special case gone);
  `rootCommandTargets` declares `outputs: []` (router output is terminal output; written files belong to their own
  targets); contract test extended (`commit`/`os` uncached, `os-dev`/`format-check` not, `setup-git` uncached).
- `outputs: []` for verified read-only targets: os-dev `layer-lint`, `index-lint`, `host-handle-lint`,
  `preview-distribution`, `preview-generated` (stdout generator protocol), `scale-fixture-check`; root `scale-fixture-check`.
- Evidence: `nx show project workspace` → commit/micro-commit/os/semio/scale-fixture `cache=false`, examples and
  scale-fixture-check `cache=true outputs=[]`; `bunx nx run repo:audit --skip-nx-cache` →
  `projects=700 commands=7134 artifacts=7766 violations=0`.

## Review fix 7: Vite stale-optimized-dep reload under the moved cacheDir (after lane S11)

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` `isPlaygroundOptimizedDepUrl` hard-coded `/node_modules/.vite/deps/`,
so `playgroundStaleOptimizeDepPlugin` (full reload on a 504 stale prebundle) could never match a Vite server whose
`cacheDir` lives elsewhere — already silently dead for os-dev/demonstrator (`node_modules/.vite-os-dev/…`) and
impossible under `⚡️cache/vite/<consumer>` (emoji path, percent-encoded in `req.url`). New
`playgroundOptimizedDepUrlPrefix(root, cacheDir)` derives Vite's real prefix (root-relative inside `root`, `/@fs/`
outside) from `server.config`; matching runs on the decoded URL. Test updated (classic, encoded emoji, malformed,
outside-root). Assertions executed directly against the module: pass.

Runtime evidence (live peer dev server `serve-generation3d-react-dev`, port 6018, restarted by Vite on config change):
optimizer cache at `.🧬semio/🦑️repo/⚡️cache/vite/os-dev/generation3d-react` (59 MB); the prebundled chunk
`deps/@dnd-kit_core.js` is served with HTTP 200 at both URLs the helper computes (root-relative and `/@fs/`).

Also: stale `sccache` rationale in root `Cargo.toml` profile comment rewritten for the shared build-dir.

## Final state

- `repo:audit`: `projects=700 commands=7134 artifacts=7766 violations=0` (was 322 CACHE-06 at the start of the day).
- `NX_DAEMON=false bunx nx show projects`: exit 0.
- Repo-wide scan for `sccache|SCCACHE_|RUSTC_WRAPPER|cargo/browser|node_modules/.cache/ms-playwright|node_modules/.vite|.nx/cache`:
  only isolated-sandbox fixtures, a discovery-skip fixture and the Vite default-layout fixture remain (intentional).
- Shared caches in use: `⚡️cache/cargo/{build,target}` (≈35 GiB warm), `⚡️cache/nx`, `⚡️cache/vite/*`,
  `⚡️cache/tools/ms-playwright`; bounded by `repo:cache-prune` (also auto-triggered ≤1/h after native cargo runs) and
  Nx `maxCacheSize`.

## Phase 2 (2026-09-12) — see `📓️2026-09-12-phase2-every-step-cached-plan.md`

- Old session builds deleted (~244 GB, 8 `target*` dirs in two dead scratchpads; free 46 → 250 GB).
- P6: incremental state got its own budget (24 GiB / 24 h / 1 h guard, `-working` sessions protected); live prune
  130 → 78 GB build-dir.
- P2: `activate-*` + wgpu `prepare-*` cached into per-variant Nx-owned roots; wgpu runner reads the Nx output.
- P4: 21 tail targets cached, 92 kept uncached with reasons; `mutatingName`/`liveName` narrowed.

### Review fix 8: domain carve-out moved from code to schema-first policy data

P4 hard-coded `!/^hub-live-catalog/` into the domain-neutral plugin's `liveName`. Replaced by `policy.json`
`cachedExact` (schema: names verified deterministic that merely collide with the mutating/live heuristics) checked in
`targetPolicy` after the uncached lists and before the heuristics; `liveName` is domain-neutral again. The nx-contract
vectors asserted authored-independence for every row, which contradicts owner-classified names outside every family
(`catalog-smoke`, `artifact-field-parity-report`, `reset-document-ownership` — the pre-existing `catalog-smoke` failure
S11 reported): schema-first optional `authored` flag on policy rows; test and the 2026-09-09 ticket probe
(`verify-cache-contract-policy.ts`, also fixed for the `matchesUncached(name, policy)` signature) now assert it.
Evidence: 34/34 vectors match; probe PASS; graph exit 0; `repo:audit` violations=0.

### Review fix 9: lock-freshness guard soundness (trunk-lockfile suite failure)

The full cache-contract suite, unblocked by fix 8, failed in `🔒️trunk-lockfile`: a lane had cached the wgpu
`lockfile-check` (`native cargo metadata --locked`). Caching is right, but `--locked` validates `Cargo.lock` against
EVERY workspace manifest while the target only hashed its own/dependency sources + root manifest → a stale lock caused
by an unrelated member would replay a pass. New domain-neutral plugin rule `nativeLockInputs(command)`: every
`native cargo metadata` target also hashes `{workspaceRoot}/**/Cargo.toml` + `Cargo.lock`. Test asserts the resolved
policy (`cache: true`) and the rule (positive + negative case). `nx show project` → `lockfile-check cache true,
workspace manifests hashed true`. Suite then advanced to 16 PASS and stopped on the next stale-contract assertion
(`check-browser-worker`), which exposed a real bug class handed to lane Q1: cached freshness checks lose sight of the
generated files they verify because declared outputs are excluded from `default`.

- P1: exhaustive tests cached (real input gap fixed: walk-up SUT crates + shared oracle crates now hashed).
- P3: 135 continuous targets audited; 4 inline-build gaps fixed; os-hub dev cargo build deferred → lane Q2.

### Review fix 10: evidence-location invariants are policy-driven, not a folder name (after lane R1)

R1 satisfied two hard-coded "path must contain `🗑️generated`" invariants (library `runExactCargoLaws`, describe
`freshRun`) by nesting gis evidence as `dist/<target>/🗑️generated/exact`. The invariants' intent is "evidence lives in
a disposable generated location"; the policy already declares that set once (`generatedDirectories`: `dist`,
`🗑️generated`, `target`, …). New `isGeneratedPath()` in `⚡️caching/🟦️.ts` (lazy policy read — a module-load read broke
the bootstrap sandbox, which copies only the import closure) is used by both invariants; gis evidence is
`dist/component-cold-map-patch-native-check/exact` again. Evidence: Node strip-types + Bun load OK; exact-cargo-laws
26/26 pass; cache-contract suite exit 0, 78 PASS; `repo:audit` violations=0; graph exit 0.
