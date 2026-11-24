# L41 3D Native Results (plain flags, partial, 2026-11-24)

`l41-wide-spill.sh`: plain flags, so most crates come from the lower cache, and every `semio-s-*` crate is spilled to `/dev/shm`. 3D compiled in minutes instead of hours. Tests run about 8× slower at opt-level 0. Claude Code's low-memory reaper stopped the run during `editor-panels`, so the surface (🔬️unit), task-resume, 5D, hub and 2D batches did not run. Log: `🗑️generated/l41-3d.log`. Failures: `🗑️generated/l41-3d-failed.txt`.

| target | L41 | L32 |
|---|---|---|
| action-producer | 1 ok, 1 failed (settlement) | 1/1 |
| budgets | 72 ok, 1 failed | 70/3 |
| defects | 52 ok, 1 failed | 50/3 |
| gestures | 47 ok, 2 failed | 45/4 |
| manifest | 58 ok, 5 failed | 58/5 |
| panels | 85 ok, 6 failed before the stop (no panic texts) | 83/9 |

## Confirmed by L41
- `l35`: both large-import stock tests and `open_vortex` close custody pass.
- `l36`: clipboard ingress release passes (the slot assertion is the next failure, fixed in `l42`). The topology counter passes (`selection_scale` ok).
- `l38`: `set_object_kind_weight` (unordered-map retirement), `fill_tool_wins` (ordered-map drop) and `window_options` (Pack receiver) pass.

## Remaining
- `set_active_example…` budget and defects: worst turn 2.6–3.1 ms against the 2 ms unoptimized budget. This is an opt-level 0 timing artifact; it passed at opt-level 1 in L32.
- Clipboard slot restore: fixed in source (`l42-clipboard-slot.md`).
- Mutation latency: wide seal window, store-unit runs and a `[DEBUG]` histogram are in source (`l43-mutation-latency.md`).
- Manifest local reads and render pressure: test host-loop receipt fix in source (`l44-3d-local-reads.md`).
- Panels fill run: finalize, set count, raise/lower, engagement abort and cancelling. Not diagnosed yet in this run; the earlier analysis is `l38` A and `l37` (`original_tool_run` migration).
- `two_instances_converge…via_backbone`: still stuck on the OwnedStores rung.
- Settlement ledger: an O(document) clone (`l35` §3).
- `[DEBUG] tool-run gate` printed nothing before the stop. The brush tests are in the surface target, which did not run.
