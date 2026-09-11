# Lane S2 — Root Script Cache Consolidation (2026-09-11)

Scope: root `📜️script.ts`, root `📋️project.json`, root `package.json`. Part of the single-shared-cache-root
plan (`📓️2026-09-11-single-shared-cache-plan.md`, Wave B lane S2).

## Changes

### 1. sccache bootstrap removed entirely

- `📜️script.ts`: deleted the whole `//#region 🔖️SccacheSetup` block (was at the old line numbers ~294–360):
  `SCCACHE_VERSION`, `ensureSccache()`, `sccacheReleaseAsset()` — the version pin, the GitHub-release
  download/extract-under-`⚡️cache/sccache`/`⚡️cache/extract` logic, and the platform asset matrix.
- `📜️script.ts` `SetupScript.runDependencies` (`deps tools` branch): removed the `ensureSccache();` call.
  `cargo-nextest`/`cargo-llvm-cov` installs are untouched — that target is not sccache-specific.
- `📜️script.ts` top-level `node:fs` import: dropped `chmodSync` and `copyFileSync` — both were used only by
  the removed sccache extraction code (verified no other call site in the file uses either).
- No CLI help text, `project.json` target, or `package.json` script referenced "sccache" — confirmed via
  `rg -ni sccache` across all three files after the edit (zero hits).
- Why: `.cargo/config.toml`'s `rustc-wrapper = "sccache"` was already removed in Wave A; one shared
  fine-grain-locked build-dir makes the wrapper (and its own out-of-repo cache/serialising server)
  redundant.

### 2. Ticket-scoped private `CARGO_TARGET_DIR`/`CARGO_INCREMENTAL` removed

- `📜️script.ts` `VerifyScript`, 4 call sites (`cad-document-contract`, `layout-document-contract`,
  `generation3d-preview-window-transient`, `gis-terrain-window-config`, each under `segments[1] ===
  "native"`, originally around lines 7434, 7447, 7460, 7473): removed
  `process.env.CARGO_TARGET_DIR = join(this.root, ".../🗑️generated/cargo-trinity")` and
  `process.env.CARGO_INCREMENTAL = "0"` before each `runCargo(...)` call.
- Why: `runCargo` (from the library package module) defaults to `process.env`, so with these overrides
  gone it now resolves through Cargo's own precedence — i.e. `.cargo/config.toml`'s
  `build.target-dir`/`build.build-dir` under `.🧬semio/🦑️repo/⚡️cache/cargo/` — instead of a private
  per-ticket target dir. No other `CARGO_TARGET_DIR`, `RUSTC_WRAPPER`, `SCCACHE_*`,
  `CARGO_BUILD_TARGET_DIR`/`CARGO_BUILD_BUILD_DIR`, `"target-dir"`/`'target-dir'`,
  `--target-dir`, or `cargo/browser` reference exists anywhere else in the root script (verified with
  `rg` for each pattern — all zero hits after the edit). The `join(root, "target")`-style directory-name
  filters that remain (e.g. `entry.name === "target"` in various tree walkers/skip-lists at lines like
  8213, 13696, 14853, 18203, 21258, 24471) are unrelated — they are walker skip-lists, not artifact
  lookups, and were left untouched.

### 3. `PLAYWRIGHT_BROWSERS_PATH` routed through the shared cache root

- Added `import { repoCacheDirectory } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";`
  next to the existing library-package import block.
- `📜️script.ts` `SetupScript.runDependencies` (`deps browsers` branch, was line 439): 
  `PLAYWRIGHT_BROWSERS_PATH: join(this.root, "node_modules/.cache/ms-playwright")` →
  `PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(this.root, "tools", "ms-playwright")`.
- `📜️script.ts` `TestScript.runStorybookPlaywright` (was line 14249):
  `process.env.PLAYWRIGHT_BROWSERS_PATH ?? \`${this.root}/node_modules/.cache/ms-playwright\`` →
  `process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(this.root, "tools", "ms-playwright")`.
- No other `node_modules/.cache` or `node_modules/.vite` path exists in the root script (confirmed via
  `rg`, zero hits after the edit).

### 4. `clean` never sweeps the cache root; delegates to `cache prune`; cargo `target*` always stray

- The shared cache root (`.🧬semio/🦑️repo/⚡️cache/`) was **already** structurally protected before this
  change: `cleanProtectedPrefixes` includes `join(getRepoMetaDir(root), CLEAN_CACHE_DIR_NAME)`, and every
  `cleanWalkDirs` visitor (`cleanCollectMisplaced`, `cleanCollectGitignore`-equivalent walk,
  `cleanBuildArtifactRemovals`) and the separate `cleanCollectWindowsIllegal` walk skip
  `name === CLEAN_CACHE_DIR_NAME` at every depth, so `clean` never enters or deletes anything under it.
  That part of the requirement needed no code change — only the "delegate its own maintenance to
  `cache prune`" part was missing, added below.
- `📜️script.ts` `CleanScript.run`: after the existing `runWorkspaceClean` sweep + report printing, added
  a call to `this.runCachePrune(dry)`.
- New private method `CleanScript.runCachePrune(dry)`: invokes
  `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts cache prune`, passing
  `--apply` when `dry` is false (mirrors the sibling `DiskPruneScript`'s own `--apply` convention in that
  same module — `dry: !args.includes("--apply")`) and nothing when `dry` is true. Uses `runCmdStatus`
  (not `runCmd`) so a non-zero exit is logged, not thrown — `clean` must keep working for its own
  categories even while `cache prune` doesn't exist yet. Logs
  `[clean] cache-prune ok|unavailable (exit N) <cache-root-path>`.
  **`cache prune` does not exist yet** (S7 in flight concurrently) — live-verified below: the command
  currently errors `unknown command "cache"` from the caching module's `ScriptRouter` (only
  `disk-report`/`disk-prune`/`doctor`/`cache-verify`/etc. are registered there today). `clean` degrades
  gracefully (see verification) and will pick up real pruning automatically once S7 registers the `cache`
  command with a `prune` subcommand. **Assumption to revisit once S7 lands**: the `--apply`/dry-default
  flag convention — I mirrored `DiskPruneScript`'s existing convention in the same file since no `cache
  prune` implementation exists yet to confirm against.
- `cleanIsBuildArtifactDirName`/`cleanBuildArtifactRemovals`: split cargo `target*` dirs from generic
  `dist`/`build`/`out` dirs. New helper `cleanIsCargoTargetDirName(name)` (`name === "target"`,
  `name.startsWith("🎯️target")`, `name.startsWith("target-")`) is used both by
  `cleanIsBuildArtifactDirName` (unchanged membership) and, new, inside `cleanBuildArtifactRemovals`:
  `if (cleanIsCargoTargetDirName(name) || bytes > CLEAN_BUILD_ARTIFACT_MAX_BYTES) out.push(...)` — cargo
  target dirs are now flagged for removal at **any** size (protected-prefix / cache-root skip still
  applies), `dist`/`build`/`out` keep the existing 10GB gate. Why: nothing writes to a `target*` dir
  outside the cache root any more (Wave A moved `build.target-dir`/`build.build-dir` into
  `.🧬semio/🦑️repo/⚡️cache/cargo/`), so any such dir found elsewhere is stale by construction, however
  small.
- Existing tests/fixtures: searched (`rg` across `🧪️tests` folders and the whole repo) for any test
  exercising `cleanBuildArtifactRemovals`, `cleanIsBuildArtifactDirName`, or
  `CLEAN_BUILD_ARTIFACT_MAX_BYTES` directly — none exist (the only clean-related tests,
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts` and the
  `active exact Cargo lease` case in `…/🧪️tests/🦀️exact-cargo-laws/🟦️.ts`, exercise `cleanProjectRemovals`
  / `cleanRemovalProtection` / ticket-generated-output removal, not the build-artifact size gate). Nothing
  to extend.

### 5. Root `project.json` / `package.json`

- `build-storybook` target already declares `"cache": true` and
  `"outputs": ["{workspaceRoot}/storybook-static"]` — no change needed (`git diff` on `project.json` is
  empty; this was already correct before this lane started, contrary to the stale
  `📓️2026-09-11-explore-js-outputs.md` finding).
- `disk-prune`/`disk-report` targets: **do not exist in root `project.json`** (only in the caching
  module's own `project.json`, which is S7's file, not mine) — nothing to remove here.
- `package.json` `scripts`: every script calls `bun nx run workspace:...` (or a named project) except the
  bootstrap `"nx"` script itself; none reference sccache, `disk-prune`/`disk-report`,
  `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, or `node_modules/.cache` — confirmed via `rg`, zero hits. No change
  needed.

## Verification (all run from `/Users/ueli/Documents/semio`)

1. `bun build --target=bun --no-bundle 📜️script.ts > /dev/null` — **exit 0**, no compiler errors (only
   normal `console.error`/`throw` source lines matched a post-hoc `grep -i error` sanity check on the
   captured output).
2. `rg -ni sccache 📜️script.ts 📋️project.json package.json` — **zero matches** (confirms full removal).
3. `rg -n "CARGO_TARGET_DIR|RUSTC_WRAPPER|CARGO_INCREMENTAL|SCCACHE_|CARGO_BUILD_TARGET_DIR|CARGO_BUILD_BUILD_DIR" 📜️script.ts`
   — **zero matches** after the edit (was 10, all in the 4 ticket-scoped `VerifyScript` blocks).
4. `bun ./📜️script.ts setup` — prints `[setup] locked dependency environments are ready through the Nx
   prerequisite graph` (exit 0; unrelated to the edited paths but confirms the file still runs).
5. `bun ./📜️script.ts setup deps tools` — **exit 0**, no output (both `cargo-nextest 0.9.140` and
   `cargo-llvm-cov 0.8.7` were already at the pinned version so `ensureCargoTool` short-circuited; no
   sccache install attempted, confirming the removed call site is gone at runtime, not just in source).
6. `bun ./📜️script.ts clean coverage --dry` — unaffected, prints the 5 expected `would-remove` coverage
   dirs.
7. `bun ./📜️script.ts clean --dry` (full run, ~75s — repo-wide byte accounting over many `target*` dirs,
   not a regression from this change) — **exit 0**. Output highlights:
   - `[clean] build-artifact: 97 (bytes=29341991533)` — 97 stray build-artifact dirs flagged, ~27.3GB.
     Confirms the new "cargo target dirs regardless of size" rule is live: many flagged entries are tiny
     `🎯️targets` dirs (e.g. 995, 1048, 1155, 1607, 2046, 2231 bytes) that the old 10GB gate would never
     have caught.
   - `.🧬semio/🦑️repo/⚡️cache/**` never appears in the walk or the removal list (still fully protected).
   - Root `./target` does not appear — it no longer exists on disk (already migrated/removed upstream of
     this lane; verified with `test -d target` → missing).
   - Final two lines: `unknown command "cache"` /
     `usage: bun ./📜️script.ts <test|audit|policy-check|artifact-check|artifact-package-contract|graph-check|doctor|disk-report|disk-prune|cache-verify> [args…]`
     followed by `[clean] cache-prune unavailable (exit 1) /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache`
     — exactly the documented degrade-gracefully path: `cache prune` isn't registered yet, `clean` logs it
     and still exits 0.
8. `bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts"` —
   **1 pass, 0 fail**. This test calls `new CleanScript(root, root).run([])` with `dry=false` against an
   isolated temp fixture, so it exercises the new `runCachePrune(false)` path for real: it logs
   `error: Module not found ".../🧰️framework/.../⚡️caching/📜️script.ts"` (the fixture root has no
   framework tree) and `[clean] cache-prune unavailable (exit 1) <fixture>/.🧬semio/🦑️repo/⚡️cache`,
   then still passes — confirms `runCmdStatus` (not `runCmd`) was the right choice so `clean` never throws
   on a missing/absent `cache prune`.
9. Did **not** run `…/🧪️tests/🦀️exact-cargo-laws/🟦️.ts` in full (needs `SEMIO_TEST_ARTIFACT_DIR`, runs 19+
   heavyweight cargo-law cases unrelated to this lane's edits) — its one `CleanScript`-calling case
   (`active exact Cargo lease protects ticket evidence from workspace cleanup`) only asserts ticket-lease
   protection, not build-artifact or cache-prune behaviour, so it was judged out of scope rather than run.

## Left undone / follow-ups for other lanes

- `cache prune` (and `cache report`) are not implemented yet (S7's file, `📚️library/⚡️caching/📜️script.ts`
  only registers `disk-report`/`disk-prune`/`doctor`/`cache-verify`/etc. today). `clean` calls
  `bun …/⚡️caching/📜️script.ts cache prune [--apply]` and logs a clear "unavailable" line until S7 lands
  it — no stub was added, per instructions. Once S7 registers the `cache` command, re-verify the exact
  dry/apply flag name against this lane's `--apply` assumption (mirrors `DiskPruneScript`'s own
  convention) and adjust `CleanScript.runCachePrune` if S7 chose a different flag.
- Everything else assigned to lane S2 (sccache bootstrap, ticket-scoped `CARGO_TARGET_DIR`/
  `CARGO_INCREMENTAL`, `PLAYWRIGHT_BROWSERS_PATH`, clean's cache-root delegation + cargo-target-always-stray
  rule, root `project.json`/`package.json` checks) is complete.

## Files touched

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/📋️project.json` — inspected only, no change needed.
- `/Users/ueli/Documents/semio/package.json` — inspected only, no change needed.

## Coordinator review fix (2026-09-11)

`cleanIsCargoTargetDirName` matched `name.startsWith("🎯️target")`, which includes the source taxonomy folder
`🎯️targets` (e.g. `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`). With the new "always stray" rule a real `clean`
would have deleted source trees (the tiny `🎯️targets` hits in item 7 were exactly that). Fixed in `📜️script.ts`:
names are exact (`target`, `🎯️target`, `target-*`, `🎯️target-*`) and the always-stray rule additionally requires
Cargo's `CACHEDIR.TAG` (`cleanIsCargoTargetDir`). Re-ran `bun ./📜️script.ts clean --dry`: 54 build-artifact rows
(28.8 GB), 0 containing `🎯️targets`, all real Cargo targets.
