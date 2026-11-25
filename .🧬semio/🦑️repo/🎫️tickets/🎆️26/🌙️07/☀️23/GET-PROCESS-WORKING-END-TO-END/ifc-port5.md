# IFC 4x3 / 2x3 Port, Round 5

## Result
- 4x3 `--lib` (log ifc54, run under ifc-guard-suite.sh with a 15 minute silence guard): 69 passed, 11 failed (round 4: 63/17). No hang. The set-select test that looked hung in ifc51 passes alone in 71 s and inside the suite; it is slow, not stalled. Not yet profiled to a per-step cause.
- 2x3: details crate compiles after the `RetainedWorkOutcome` import fix; test-target ports (11 base, 24 base-details errors) not started.
- Scalar/enum/set commit law: not written. Phase 96 is already restricted to set/array/list leaves (`set` requires `Set|Array|List`); enum goes to 103, binary to 104.

## Product fixes this round
- Window kit `tree_window_indexed_rows`: a failing subtree census returns `ui.tree-window.subtree-census` instead of silently shortening the window.
- Snapshot-details renderer: each row pays for its own fixed controls from the node ledger (`charge_fixed_nodes`), refusing with `ui.fixed-capacity` when the ledger cannot pay; collection child windows take their open flag from the new provider hook `SnapshotDetailsProvider::default_open`. The IFC provider keeps `header` collapsed while the snapshot has entities, because the open header spent the whole first-paint ledger and starved the entities row ("Coordinates" missing). Reversible UI decision.
- `constituent_roster.rs`: the primary-payload guard used `value_cursor.terminal_is_empty()`, which a fresh derived clone cursor never satisfies (it requires `closing`). It now refuses only a busy cursor (`output_ready` or `close_started`) or an occupied value. Fixed the "IFC primary payload owner occupied" family (required_type_input, list, measure_select).
- `⚖️validation/local.rs`: `express.receive-*` refusals from the population receiver make the candidate `Invalid` (draft stays editable, `ready:false`) instead of aborting the render (mixed_select dangling reference).
- Test helper `retired_tree_serialization` retires its own tree and drains the built-node close queue until that queue reports empty; it no longer waits on process-global emptiness.
- Guided tests print the refusal and the case name when a body is not built.

## Fixture change
`type-value-select-input.json` and schema: the four `domain: null` cases carry an explicit `ready: false` (no physical value is built for them); the Rust test asserts `domain` when set, else the case's explicit `ready`. Not yet run after this edit.

## Open 4x3 failures (11) and diagnosis
- enum `full53`: rules engine returns Incomplete because evaluating the IFCDERIVEDUNIT rules looks up the enum type name `IFCDERIVEDUNITENUM` as an attribute of SELF and raises `express.unbound` (`machine-population.rs`, identity attribute lookup). Evaluator semantics fix needed (identifier that is an enum type name must fall through to global name resolution).
- guided_controller: "retained source original authority is closing or transferred" (`try_authority` after the owner was taken). Not traced.
- denied_axes: `result.is_err()` expected for the action swap, got Ok. Not traced.
- required_type_input_uses: assertions pass; arena teardown refuses ("test arena retirement refused") in some orders. Not traced.
- value_select: changed assertion (see fixture change), pending run.
- Editor: `domain_insertion...` (`retained_text_bytes() >= retained_bytes`), `actual_publication_wire...` (`ifc-domain-argument-arity` for RemoveValue).
- staging-native (4): text-birth-axis stock; expired-deadline "original Source clock declined new construction"; render_owner "admission birth changed its original Work or Source"; identity_drafts frame left unsettled.

## Scripts (ticket folder)
`ifc-guard.sh <label> <filter> <seconds>` single-test watchdog; `ifc-guard-suite.sh <label> <idle-seconds>` full-suite watchdog.
