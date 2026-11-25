# IFC 4x3 / 2x3 Port, Round 4

## Result
- 4x3 `--lib`: 63 passed, 17 failed (round 3: 58/22). Log: ifc50 (scratchpad).
- 2x3: details crate import fixed (`RetainedWorkOutcome`); not yet compiled or run. Test-target ports (11 base, 24 base-details errors) not started.
- Scalar/enum/set commit law: not written. Phase 96 restriction to set/array/list leaves and the scalar commit path are open.

## Done this round
- `type-guidance.rs`: commit-copy phases renumbered to 110-112 (`COMMIT_COPY/TAKE/COMMIT_CLOSE`), removing the collision with SET phases 96-98; `context_matches` now includes `commit_copy.request()` (fixes `staging.stale` at commit). All `[DEBUG]` instrumentation removed.
- 2x3 details `work.rs` ported to `SourceStaging` / `closed_in_source`; `Cargo.toml` depends on ifc-details-wire; receive uses `with_native_receiving_work`.
- Leftover test binaries deleted (`find . -type f -perm -u+x -size +20M -delete` inside the build dirs).

## Open 4x3 failures (17) and diagnosis
- Arena lifetime closure ("UI arena descendants remain borrowed"): revisioned_host_render, mvd native_view_variants, value_select, enum, identity_drafts. Tests must retire all built trees; `tree_window_indexed_rows` must return a typed refusal without headroom.
- "nested UI value owner already released" / "test arena retirement refused": list and measure_select guided tests (double retirement of a built tree).
- `express.receive-dangling-reference`: mixed_select.
- `Option::unwrap() on None`: set_select (line 10 of the test).
- `guided_controller_builds_actual_owned...`: "private donated controller must build a body"; `required_type_input...`: "must complete a body"; `required_type_controller_denied_axes...`: expected error not produced.
- Editor: `domain_insertion...` (`retained_text_bytes() >= retained_bytes`), `actual_publication_wire...` (`ifc-domain-argument-arity` for RemoveValue).
- staging-native: text-birth-axis stock does not complete in one turn; expired-deadline "original Source clock declined new construction"; render_owner "admission birth changed its original Work or Source"; identity_drafts frame left unsettled.

## Blocker observed
Another agent's debug counter in the framework native-receiver-protocol `🦀️.rs:78` briefly broke compilation (uses `self`, `P`, `demand` inside a free fn); it cleared on retry.
