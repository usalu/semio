# F82: volume reclaim without a human (wasmtime cache, shared-store staging, volume floor)

State: all TypeScript parts landed and tested; the Rust parts are written and NOT compiled or run (seal window): "pending Rust tests".

## 1. Wasmtime compiled cache (plugin host engine)
- Policy in JSON, not code: `🔌️plugin/🖥️host/🔧️engine/🧹️cache-retirement/⚙️configuration/🔣️.json` (cap 8 GiB, age 14 d, never-retire-if-used-within 1 h, staging grace 6 h, run interval 6 h, touch interval 10 min, scan bound 100000), schema in `🧬️schema/🔣️.json`.
- `🧹️cache-retirement/🦀️.rs`: pure `select_retired` (staging past grace, complete files past age, then LRU by modification time, ties by path, until the rest fits the cap; a file used within the idle minimum is never selected), `scan_compiled_cache`, `retire_compiled_cache`, `retire_compiled_cache_if_due` (stamp file in the cache root, renewed before the scan so concurrent engine starts skip), `touch_compiled_cache_entry` (a cache hit refreshes mtime at most every touch interval = the LRU clock). Only `.cwasm` and `.scratch` two levels down are looked at (lock file and stamp are not); deletions are single `remove_file` (a mapped file on Windows refuses and stays). Writes were already temp + rename.
- Hook: `🔧️engine/🦀️.rs` new `prepared_compiled_cache_root()` (retire if due, then root); `🎭️guest-runtime/🐎️wasmtime-runtime` uses it at runtime construction (engine start of every host and shard child); `load_compiled_component` touches on a hit.
- No new runtime dependency: the engine crate reads the flat JSON with a 20-line parser; the Rust test cross-checks it against `serde_json` (dev-dependency). No Cargo.toml change.
- Tests: language-agnostic fixture `🧪️tests/🧫️fixtures/🔣️.json` (8 cases) judged by the Rust selector (`🧹️cache-retirement/🧪️tests/🦀️.rs`, 5 tests incl. a real temp cache root, stamp rate limit, touch interval, absent root) and by an independent TS oracle (`🧪️tests/🟦️.ts`, 4 pass, run with `repo test-file`). **Rust tests pending (not compiled).**

## 2. Shared-store staging `semio-store/v1/tmp`
- New `🤝️shared-worktree/📒️journal/🧹️staging-trees/` (`🔣️.json` schema + policy: grace 1 h for trees, 6 h for loose files, walk bound 1,000,000 entries; `🟦️.ts`; `🧪️tests/🟦️.ts`, 6 pass).
- `createStagingTreeV1(store, originId, label)` makes `<originId>.<label>.<pid>.<startMs>.<hex16>`: the owner is the shared process identity (`🏃️process/🔖️identity`, pid + start time), never the pid alone. `reclaimStagingTreesV1` keeps any directory whose registered owner is this machine's origin and proven alive by `processHolderAliveV1` (whatever its age); anything else (dead owner, reused pid, another machine's origin, no registered owner) goes when nothing below it was modified for the grace; trees above the walk bound, links, files and `.nfs*` are kept; dry run; cancellation.
- Creators: the dirs found by F80 (`procedural3d-cold-units-*`, `draw-guest-*`) are created by no code in the checkout (grepped `*.rs/*.ts/*.mjs/*.json/*.py/*.sh`, tickets excluded: the names exist only in `build-owners.json` crate names and ticket notes), i.e. ad-hoc lane scripts with `TMPDIR` or an explicit path inside the store. They register no owner, so they fall under the age rule (1 h without a write) and are reclaimed anyway; any future creator must use `createStagingTreeV1`.
- Hook: disk-recovery registry row `fleet-store-staging` (priority 64, `staging-sweep`, `{store}/v1/tmp`, bilingual bound) bound in `🔌️real/🟦️.ts` (trees + loose files, `{store}` from `.cache/f/machine.json`, nothing without a store). The registry feeds the readiness supervisor (`recoverNeedV1`, `sweepDiskV1`), the build-admission disk guard (`recoverDiskV1`) and `repo cache gc` (`collectMachineBoundsV1`), which is the existing per-machine periodic maintenance; liveness is by identity and age, never by directory mtime alone (lock-aware rule).

## 3. Volume floor
- Registry (`🌡️host-capacity/🧹️disk-recovery/🧬️schema/🔣️.json`): new optional row flag `floor` (typed in `🟦️.ts`), set on `leaked-staging`, `wasmtime-compiled-cache` (priority 63, root `{home}/.semio/cache/wasmtime`, SEMIO_HOME honoured) and `fleet-store-staging` (64); all derivable + host relief and ordered before every content/store/image row (cas-lru is 70). New `Bounds.compiledCache` (same numbers as the engine config; a test asserts equality). TS selector/retirer `🧺️bounds/📦️compiled-cache/🟦️.ts` (same rule as the Rust one, judged on the engine's fixture).
- `relieveVolumeFloorV1` (disk-recovery) runs only `floor` rows by priority until a target free level on the volume (volume re-read after each, failing collector never stops the next, cancellable); `relieveMachineVolumeV1` (`🔌️real`) binds it to the real writers and the store's partition measurement.
- CAS guard: new port `relieve` and policy value `volumeReliefReserveFactor` = 2 (CAS schema + type + validation). `casEnforceTickV1` asks the relief for the bytes up to 2 x the role's hard reserve BEFORE its own sweep/collection when the partition is below that level, re-reads the volume, and goes on (a failing relief is swallowed, a cancelled one rethrows). So the volume is defended before the CAS reserve is breached, with the derivable classes first and the content cache last.
- Wiring: `🌐️remote-cache/🖥️server` new option `relieveVolume` passed to the tick; the fleet cache sidecar (`🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts`) binds it to `relieveMachineVolumeV1` of its workspace (the machine that 507'd).

## Tests (all `repo test-file`, one at a time)
- new: staging-trees 6/0, volume-floor 8/0, cas volume-relief 4/0, server volume-relief 2/0, engine cache-retirement (TS) 4/0.
- re-run unchanged suites: cas 72/0, replica-store 10/0, disk 8/0, disk-recovery 23/0, journal 132/0, remote-cache 26/0 (1 skip), sidecar 7/0.
- `repo typecheck` of every touched path: no error in any touched or new file (the 70-odd reported errors are older lines of other test files). `dangling-imports.py` on all touched folders: exit 0.

## Landing locks
Took cas, host-capacity, remote-cache, replica, remote-execution (alphabetical; F81 held remote-execution until 10:20). The first TS edits to cas and host-capacity files were made before I took their locks (no other lane held them, nothing conflicted); released all after the tests.

## Not verified
- Rust: nothing compiled or run (seal): crate `semio-plugin-host-engine` tests `cache_retirement_tests` and the existing capacity test, plus a compile of `semio-plugin-host-guest-runtime`. Clippy lints of the workspace not run.
- No live run on the real `~/.semio/cache/wasmtime` or the real store (no live experiments); the F80 cleanup already freed those.
- The relief is only wired in the fleet cache sidecar; other machines get the same classes through the readiness supervisor / `repo cache gc` registry rows.
