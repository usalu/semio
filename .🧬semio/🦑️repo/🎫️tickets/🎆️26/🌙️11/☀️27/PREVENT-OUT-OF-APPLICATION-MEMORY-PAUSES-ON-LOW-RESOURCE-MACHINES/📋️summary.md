# 📋️ Summary: Prevent Out of Application Memory Pauses (2026-11-28)

## Incident

On 2026-11-27 at about 17:30:20 a 16 GB Mac paused Claude Code (resumed 18:01:21). At that moment:
- swap was 95 % full (3.06 of 3.22 GB);
- the compressor held 8.26 GB;
- swap-out was 705 MB/s.

The warden saw only its own 2–3 GB of grants, and it restarted 151 times during the day. Details are in `🧾️root-cause.md`.

## Cures (lane reports)

- `🧯️closing-MEM-GUARD.md` (warden):
  - It counts foreign memory.
  - It suspends over-grant holders and vitest workers only on live danger.
  - It hands over to a new instance when its sources change, so there is no gap.
  - It cancels git above 1.5 GB.
  - Swap-free and compressed lines are tied to live signals. The compressed-only regression was fixed the same evening.
  - Footprint is read per process.
- `🪓️closing-MEM-CARVE.md` (units): 9 staffage test cases that parsed the whole catalog now load only their assets. The largest went from 3946 to 1295 MB. The export CLI no longer caches the catalog, and the model-catalog audit frees memory per file.
- `🎚️closing-MEM-ADMIT.md` (admission):
  - Elastic reservations converge to the observed peak plus a margin.
  - Cargo and rustc jobs are derived from memory: 1 job on 8 GB, 3 on 16 GB.
  - Stale estimate trust is kept only downward.
- `🧪️closing-CAP-REDS.md`: 4 red host-capacity test files were cured, none of them caused by this ticket. The `🔼️upgrade` grant cap closes an over-ceiling reservation. 123 of 123 tests are green.

## Open, for the dev

- **Desktop app diff pane:** it diffs against `⛳wip` (`refs/heads/⛳wip` and `refs/remotes/origin/⛳wip` at `277881dca`, 762 commits back, 315,851 files, over 2 GB per run).
  - Mitigation: `git update-ref` both refs to `🐙ueli/⛳wip`, by the dev or the archivist.
  - The warden now cancels such a git process above 1.5 GB.
- **Not carved here:** hub wasm finals (cad about 8 GB, puzzle about 6 GB), owned by `SPLIT-PLUGIN-WASM-INTO-SHARED-FRAMEWORK-COMPONENT-AND-THIN-PLUGIN-COMPONENTS`.
- **Not measured:** `🧊️catalog-silhouette` and `🧊️model-batch` (command in `🪓️closing-MEM-CARVE.md`).
- **Other lane's call:** the `🔼️upgrade` spare-job question is for the lane that owns that change (`🧪️closing-CAP-REDS.md`).
- **Draw note:** `🔗️from-draw-e2e-warden-deadlock.md`, from 19:45 to 20:14, describes the compressed-only hold that MEM-GUARD's 20:05 fix removes. Suspensions since then occur only under kernel pressure warn, and they resume when it is calm.
