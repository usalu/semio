# L32 3D Native Results (full run, 2026-11-24 04:03)

The batch built after one retry. The first attempt hit a concurrent `phase_witness` edit by another agent. Every target ran. Log: `🗑️generated/l32-3d.log`. Failures by target: `🗑️generated/l32-3d-failed.txt`. Some test names appear in more than one target.

| target | result | before (l29) |
|---|---|---|
| puzzle-3d unit | 10/10 ok | 10/10 |
| snapshot-sqlite | 14/14 ok | 14/14 |
| editor unit | 1/1 ok | 1/1 |
| action-producer | 1 ok, 1 failed (settlement ledger) | 0/2 |
| budgets | 70 ok, 3 failed | 64/9 |
| defects | 50 ok, 3 failed (the same 3 as budgets) | 42/11 |
| gestures | 45 ok, 4 failed | about 46/6, partial |
| manifest | 58 ok, 5 failed | not run |
| panels | 83 ok, 9 failed | not run |
| surface (🔬️unit) | 52 ok, 14 failed | not run |
| task-resume | 1 ok, 3 failed | not run |

## Confirmed fixed
- `context::closes_mid_reserved_commit` and `context::conflict_history_edits_end_to_end` now pass.
- The interactive-ceiling tests `accept_suggestion_*` and `set_active_example_*` pass (`l31`).
- The source-string laws and the import wire budget (`l30`) pass: the import, open and reimport tests are ok.

## Open clusters and the lane on each
- Import stock overflow (2 tests), `open_vortex` close custody, settlement ledger: `l35`.
- Clipboard cancel ingress, topology counter, mutation latency: `l36`. The first two are fixed in source; mutation latency needs an instrumented run.
- Brush search never leaves stage 0 (dominant surface cluster), hover storm faults, residue class, agent lane, relocate scan: `l37`.
- Fill-job projection frame, ordered-map drop, window pack receiver, backbone convergence, local interaction reads, task-resume: `l38`.
