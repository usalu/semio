# Store original close work review

Source-only review; no runtime verification. Paths below are relative to framework products OS store modules.

- replay operation Rust source, lines 214–217: the terminal boxed preparation cursor is dropped and intrinsic last_turn_work is set to one without original prepare or commit. Child close at lines 219–222 supplies its own last_turn_work.
- replay cancel Rust source, lines 101–102: preparation-to-retirement handoff and the terminal child slot clearing at line 98 are local owner actions around controlled child delegation. EditReplayResult lines 203–223 transfer outputs into a potentially original-controlled residual; line 226 clears its empty residual. Both ErasedSnapshotRetirement adapters infer work from physical progress at lines 167–168 and 237–238, losing child error work.
- replay operation derived Rust source, lines 124–127 and 133–134: task handoff and terminal task-close shell drop are local actions. Child task and retirement outcomes have actual last_turn_work or turn.work; line 160 substitutes nonempty physical progress when work remains zero, including losing work for failures before that fallback.
- read snapshot-alias Rust source, lines 14 and 19–20: controlled ticket result is detached from its actual work and replaced by physical-progress inference. The local branches need their own work accounting when selected; actual original-control requirement depends on the active child.
- read derived-snapshots-replays-and-history-read-plan Rust source: preview lines 282–294 and raw replay lines 325–329 perform local cancellation, terminal owner removal and handoffs. Lines 338–339 infer work from effects; line 355 discards the controlled ticket work. Physical copied-item normalization at lines 356–359 is not computational work.

The selected action must quote and reserve its own intrinsic work before local effects, commit actual work even on refusal, and forward each child actual work without re-reserving an already controlled child. Control requirements and action priority change with ownership state; inspect the selected branch rather than treating all listed branches as simultaneously original-controlled.
