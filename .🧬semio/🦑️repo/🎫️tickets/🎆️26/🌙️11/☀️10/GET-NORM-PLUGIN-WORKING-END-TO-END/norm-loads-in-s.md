# Norm loads in the "s" instance (reg3)

Status: NOT yet proven. The s catalog regenerates, but it is empty because every plugin descriptor is stale (channel 24 vs host 25). Steps 3 and 4 are blocked on step 2 and on graph memory (Follow-up 6).

## Step 1: `repo run @semio-tech/plugin-registry:generate`
- First run: graph read finished (621 owners, about 4.5 h under fleet load), then failed in the catalog projection: `Registry implementation import is not schema-owned: @semio-tech/value-ts/refusal in 🚪️io/⏱️failure-owner/🟦️.ts`. The merge made the registry implementation closure reach `failure-owner`.
- Root fix: three schema-owned workspace bindings added to the plugin-registry `inputDiscovery.workspaceImports` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: `@semio-tech/value-ts/refusal`, `@semio-tech/value-ts/refusal/codec`, `@semio-tech/framework-diagnostic-ts/controlled` (found with the fast driver `reg3-closure.ts`, loop `reg3-bind.py`; closure = 144817 input paths).
- Second run: rc=0.

## Step 2: s catalog (`@semio-tech/s-dev-composition:generate`)
- Run 1 cancelled at the graph memory limit (3961/3961 MB) at owners 713/713. Run 2 (after the host quieted) rc=0.
- Result: `🧩️catalog/🤖️generated/🔌️plugins.json` is `[]`, `🦀️catalog.json` has `plugins: []`. `🩺️diagnostics.json` lists 70 x `stale-channel` (descriptor 24, host 25), including norm (`🌎️hub/🧩️compositions/📕️norm/🧩️component/...`). Dev generate runs with `staleChannel: "exclude"`, so it withholds them silently and succeeds.
- Norm is therefore NOT listed. Required: regenerate descriptors (`repo run workspace:components -- describe norm`, then `--all`).

## Descriptor regeneration (blocked)
- `describe norm` attempts: (1) resident service "unexpected failure" ed025f23; (2,3) `Internal error: ENOSPC`; (4) de0c1c78 after full graph read; (5) cancelled at graph memory limit 3556/3556 MB in a nested 627-owner slice read. Not retried per coordinator until Follow-up 6 lands.
- ENOSPC root cause (repo bug, fixed): `/run/user/1000` (tmpfs, 1.6 GB) was 100 % full. The resident service spills plan slices (254 to 390 MB each) to `plans-<tag>/`, retention was by count (256) and eviction ran after the write, plus failed `.tmp` writes leaked. Fix in `🏡️resident/🏛️service/🌐️host/🟨️.mjs`: `makeRoomV1` (exported) evicts oldest slices and orphan `.tmp` files until `statfsSync` free space covers the next body, `atomicWrite` removes its temp on failure. Test added in `🏡️resident/🧪️tests/🌐️host/🟦️.ts` (not yet run: `repo run repo:test-build-resident` was cancelled by the graph memory limit, so it never executed; it is not a test failure).
- Opaque error: refs ed025f23/de0c1c78 printed no log path and no log containing them was findable (searched `/run/user/1000/semio-repo*`, `~/.cache/semio`, `.cache/b`). Another agent is typing ENOSPC (`🏛️service/🧯️failure/🟨️.mjs`, repo-cas-autocap.md); I did not touch that code.
- Also: the 543 MB journal in `/run/user/1000/semio-repo-1000/d09b06e73429/journal` and 1 GB plan spills share one 1.6 GB tmpfs; no byte budget relates them to the device size.

## Why all 70 descriptors fell behind (appChannelVersion 25 vs 24)
- Pin: `🧑‍💻dev/🔖️channel-version/📌️pin/🔣️.json` = 25; descriptor schema const (`🛂️descriptor-verification/🧬️schema/🔣️.json`) = 25 since commit 3904c31bd05 (2026-11-05, was 23). Committed descriptors (`**/🧩️component/🔣️.json`, tracked in git) stay 24: they are only rewritten by `workspace:components describe` (cache false, side-effecting, heavy cargo+wasm build).
- Missing edge: the `channel-version check-descriptors` verb already exists (reports `stale-descriptor-channel` for every committed descriptor not at the pin) but NO target ran it. Nothing depends on the pin to flag descriptors, and the dev catalog generator excludes stale plugins without failing, so the bump went unnoticed.
- Fix made (graph cannot rebuild descriptors automatically; the right edge is a fail-fast gate): new target `@semio-tech/framework-os-dev:channel-version-descriptors-check` (inputs: pin and every `🧩️component/🔣️.json`; cached) in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`, and `@semio-tech/s-dev-composition:generate` now `dependsOn` it (`✏️s/🧑‍💻dev/📋️project.json`). A channel bump now fails the s catalog generate naming the regenerate command instead of emitting an empty catalog. Unverified (needs a graph run).
- Follow-up suggestion: also make `catalog-check` fail when `🩺️diagnostics.json` is non-empty or `🔌️plugins.json` is empty.

## Step 3: semantic-vocabulary and allowlist (analysis only, no graph run)
Driver `reg3-vocab.ts` calls `policySemanticVocabularyBreaches` directly (no graph): 21248 files scanned, 2300 breaches, 78 `high` (SetSnapshot / NoMutation / CollectionMutation in 78 files; 0 in norm), the rest `medium` advisory `Set*` names.
- The lane (`runSemanticVocabulary`) never consults the allowlist; it fails on any `high` breach. `policyDiscoveredAllowlist` is used only by one test. The allowlist (342 entries): 219 point at nonexistent paths, 80 more exist but no longer breach, 25 match a current high breach but have no effect. So resync = delete the file's 342 entries (all dead or inert), and the lane stays red on 78 high breaches that must be fixed in source (writer/vcs/fem/draw/forms/cad/stdio plugins, mostly `🧬️mutations/🦀️.rs` and `🎮️commands`), not allowlisted. Not done: 78-file semantic rewrite is outside this task's scope; decision needed on whether the allowlist should be wired into the lane or removed.

## Step 4: catalog-check, verify-catalog, test-catalog-publication
Targets exist (`@semio-tech/s-dev-composition:catalog-check`, `verify-catalog`, `test-catalog-publication`). Not run: blocked on descriptors (empty catalog) and graph memory.

## Files changed
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (3 bindings)
- `.../🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs`, `.../🏡️resident/🧪️tests/🌐️host/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`, `✏️s/🧑‍💻dev/📋️project.json`
- Ticket inputs: `reg3-closure.ts`, `reg3-bind.py`, `reg3-vocab.ts`; logs `🗑️generated/reg3-*.log`.
- Deleted by me: a stale 857 MB `s5.heapsnapshot` in the session scratchpad and two stale 254 MB plan spills in `/run/user/1000` (regenerable cache).

## Next (after Follow-up 6 lands)
1. `repo run workspace:components -- describe norm`, then `describe --all`; commit-level descriptors change.
2. Re-run `@semio-tech/s-dev-composition:generate`; confirm norm in `🔌️plugins.json`.
3. `repo run workspace:semantic-vocabulary`; decide allowlist wiring; fix 78 high breaches in source.
4. catalog-check, verify-catalog, test-catalog-publication; `repo run repo:test-build-resident` for the eviction test.

## reg4 (BUILD PAUSE #3, host quiet: rustc count 1 after 12 min wait)
Status: STILL NOT PROVEN. Step 1 cancelled at the graph memory limit; steps 2 to 5 not run (per instruction: record, stop, no loop).
- `repo run workspace:components -- describe norm` (log `🗑️generated/reg4-describe-norm.log`): cancelled after about 5600 s in the OWNER phase at owners 510 of 627, running owner `@semio-tech/framework-os-dev` (project, kind language:typescript, 2279 s, 3651 rows, no learned peak for that kind, 256 MB assumed). Message: "the service holds 5411 MB of its 5411 MB memory limit (safety margin 256 MB)". Peak = 5411 MB = limit.
- The root `workspace` owner (formerly the 4.6 GB blocker) passed this time under the 5411 MB limit; the new blocker is the slice as a whole: memory kept growing across 510 owners (headroom fell from 755 to 444 MB while one TypeScript project ran for 38 min), i.e. memory is retained across owners (not released per owner) and the `framework-os-dev` project is itself very slow (2279 s, 3651 rows).
- Next (graph owner, not a norm defect): find what accumulates per completed owner in the slice worker (leak/retention), and why framework-os-dev takes 38 min; then retry step 1 and steps 2 to 5 in order (describe norm, describe all, s-dev-composition:generate, catalog-check, verify-catalog, test-catalog-publication, workspace:semantic-vocabulary, repo:test-build-resident).

## reg5 (after graph build completed)
Status: STILL NOT PROVEN. Steps 2 to 5 not run (blocked on step 1).

- describe norm run 1 (`🗑️generated/reg5-describe-norm.log`, about 2600 s): graph loaded from cache (owners 217 of 217, assembly 660 of 660), then the Nx run `workspace:components` ran its three generate dependencies (ui-rs, ui-styling-tokens, dsl-derive-rs; "Successfully ran target components ... 3 skipped") and then failed with `NX Failed to send request: error sending request for url (http://127.0.0.1:6090/main/v1/cache/<hash>)`, rc=1. The describe script's own output was not shown (nx hides it), and no norm descriptor changed.
- Root cause of the rc=1 (not a describe-script or norm defect): port 6090 is the remote cache endpoint hosted INSIDE the resident graph service (`startHostedRemoteCacheV1`, `⌨️command/🌐️host/🟦️.ts`). The service process was gone by the time Nx wrote its cache entries: `/run/user/1000/semio-repo/1000/exit.log` records the service process (pid 3865479, rss 2449 MB) ending with `reason exit, code 124`, i.e. the universal Script stall watchdog (`🐕️watchdog/🟦️.ts`, "made no progress for 10 min and has no live child") killed it at 14:31:03, right after the graph build finished and before Nx started (14:31:45). The service main thread is idle by design after publishing; the watchdog only exempts declared waits and treats worker-thread stalls as aborts, but a main thread with idle workers exits. FIX NEEDED (graph owner): a resident service host must not be subject to the Script no-progress exit while it is serving (or must note progress on every served request and on the host heartbeat), and describe/nx runs must not depend on a service that can exit under them. Not yet changed because it could not be verified under the current queue.
- describe norm run 2 (`🗑️generated/reg5-describe-norm2.log`) is queued: the service restart waited about 26 min behind other agents' leases (`native-fixed-image` exclusive queue), then the graph plan waits for memory headroom (needs 5700 MB reserved, other agents hold about 6000 to 6300 MB continuously; 755 s and counting). It is still running in the background. It cannot be admitted while the fleet keeps that reservation; a build pause is needed for it to proceed.
- Next once admitted: if the service survives past the Nx step, expect the real describe output; if it dies again, apply the watchdog exemption above. Then: `describe --all` (or the bulk target), `@semio-tech/s-dev-composition:generate` (expect non-empty `🔌️plugins.json` with norm, empty diagnostics), `catalog-check`, `verify-catalog`, `test-catalog-publication`, `workspace:semantic-vocabulary`, `workspace:verify-build-owners-sync` (11 norm split crates), `repo:test-build-resident`.
