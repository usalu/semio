# L21: 3D mutation latency (184 turns vs 8)

Status: root cause NOT confirmed; no fix applied. The runner was stopped (tmpfs full) before my test ran. No source files remain changed (a temporary [DEBUG] eprintln in the pump was added and removed again).

## The test
`editor-gestures` target: `-p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`
(source: `.../✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs`, law B54, `LADDER_TURN_CEILING = 8`).
Last recorded failure (l12 log, before L17b's fixture edit):
`translate=184turns/1655units [store=1531 page=95 retirement=10 worker=19]`, ladder = 165 (translate) / 76 (delete) on 1 object; nakagin 180 objects: 87 / 70.

## Findings
- The ceiling is not wrong by design: the B54 report (`PUZZLE-3D-END-TO-END/📓️2026-09-13-wave-B54-mutation-latency-2.md`) measured exactly 17 store + 3 page + 1 retirement units per mutation. Today it is 1531 store units, about 90 times more, still document-independent (1531 vs 1532), so the regression is in the ladder's per-mutation step count, not in the document.
- Turns / units = 184 / 1655, about 9 units per turn although `TYPED_OPERATION_PUBLICATION_PUMPS` is 256 and the harness grants 256 items. The pump (`🧰️framework/.../🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs`, `publish_mounted_typed_operation_pump`) bounds each turn by `original_plugin_turn_grant(self.mounted_policy.preparation, retained)`, which is `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` = 64 items / 32 KiB copy. Each store unit spends several retained items (about 7), so the 64-item per-turn grant ends the turn after ~9 units. The grant is the turn limiter; the unit count is the amplifier.
- The harness `host_turn` (raw-app-fixture) runs publication under `policy.maintenance` (L17b change), i.e. the same 64/32 KiB grant, so the host-side 256-item grant never reaches the pump.
- Candidate for the 90x unit amplification (unverified): each unit's receipt/cursor is paged per retained item under the 32 KiB copy / 64 item grant instead of completing the B54 constant ladder (stage, prepare, fold, retire owner, validate cursor, preflight, commit, receipt).

## Needed measurement
Add a temporary `[DEBUG]` eprintln at the top of the pump loop (route booleans, `cx.fuel_remaining()`, `remaining_plugin_turn_grant(turn,*spent).maximum_items`, `spent.copied_items`, `mounted.stage`) and run the test line above with `--nocapture`; the per-unit item cost and the unit kinds that repeat will name the amplifier. Then fix by making that unit complete in one grant (framework) rather than raising the ceiling.

## Test line for the next runner
`-p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`
