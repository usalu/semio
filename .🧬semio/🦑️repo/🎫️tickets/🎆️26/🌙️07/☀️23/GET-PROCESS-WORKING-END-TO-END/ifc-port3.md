# IFC 4x3 `any` Plugin: Round 3 (owner decisions)

`cargo test --lib -p semio-s-artifact-stdio-ifc-4x3`, native cargo through `ifc-retry.sh` (logs now go to the session scratchpad because other agents delete `🗑️generated/*.txt`). Final run `ifc30`: **58 passed, 22 failed** (round 2: 50 / 30; round 1 start: 42 / 38). All `[DEBUG]` prints are removed again.

## Done this round

1. **Retained edit family (owner decision 1).** `DomainWork` was 37 312 B, so its quote was 152 321 B against the fixture's 32 768 B. The inline `DomainCursor` (fixed 128-frame arrays, a language-agnostic law) now lives behind `Option<Box<DomainCursor>>`, born under a paid receipt (`retained_capacity_bytes`, quoted as `capacity_bytes`) and released in `close_step` (`released_bytes`, quoted as `release_bytes`). The fixture was not raised. Seven of the nine tests pass.
2. **Receipt model (decision 2).** `prepared_native_clone_error_forwards_partial_effects_...` asserts `receipt_refusal` (the effect-counted refusal) instead of `context_refusal`; it passes.
3. **Fresh clone cursors.** `close_demands` of a never-started cursor is zero while `begin_close` needs `retained_close_start_demand`; new `close_quote` (`source_loan.rs`) quotes `begin_close_demand()` first and is used at every IFC call site.
4. **Close stalls.** Ancestors built with `retire_self!` quoted one work unit while a nested source close needs more, so `ControlledRetirement` idled forever (8 guided tests ran 700+ s). `retire_self!` takes an optional `work = |owner, body| ..` quote; `SourceCustody`, `SourceLoan`, `IfcTypeInputSourceWork`, `GuidedWork` and `Ifc4x3RenderWork` quote the nested source work through `RetainedCloneClose::next_owner_work`. `SourceLoan.source` moved into its own `SourceCustody`, closed through `RetainedCloneSource::close_step`. The stalls are gone.
5. **Body build.** `build_body` set no actual extent when its receipt retained capacity (`finishing an allocation requires an actual extent`); it now does. `receive_effect` takes its maximum from `with_native_receiving_work`.
6. **Test environment fixes (assertions not weakened).** Guided, render and staging tests get the UI arena (`FixtureUiArena::admit`) and `tree_probe::warm_ui()` (publishes the built-child and UI-value retirement authorities, as the framework's own tests do), a valid `IFC4X3_ADD2` header, and `retire_staged` retires under the original Source with the native close loan. `key_sorted` compares objects order-insensitively. Header tuple reduction now ticks (`ReductionControl::tick` per value-edit step).

## 2x3 (decision 4)

`🪟️details/effect.rs` `receive` now uses `with_native_receiving_work` with the lent `units` (same fix as 4x3); the 2x3 libraries compile. The 2x3 **test targets do not compile for reasons unrelated to this change**: `semio-s-artifact-stdio-ifc-2x3-base` (11 errors: `crate::viewer`, `serde_json`) and `semio-s-artifact-stdio-ifc-2x3-base-details` (24 errors: private `app_testing`, grant-shaped `RetainedCloneGrant` calls, `admit_borrowed`, `crate::editor`). They need the same test port the 4x3 crate got in round 1, so 2x3 tests were not run.

## Not green (22 failures, causes known)

- **`staging.type-input` refusals (14 guided tests: scalar, enum, select, set, value-select, measure, number-binary, reference-select, list, mixed, multiple, required-type, producer-swap, guided builder).** After candidate validation `type-guidance.rs` phase 80 always enters phase 96 (set-collection commit copy), whose arm calls `set_members` and refuses for a non-set row. The branch `self.state.phase = if self.state.ready { 96 } else { 6 }` must pick 96 only for a set/array/list leaf and take the scalar commit path otherwise; reconstructing that scalar path needs a design decision.
- **`retained_text_bytes` (`domain_insertion_...`) and `ifc-domain-argument-arity` (`actual_publication_wire_...`).** Reached now; the arity refusal comes from `domain_work.rs` for one neutral wire case, not yet root-caused.
- **`revisioned_host_render_...`.** Renders only the first row: `tree_window_indexed_rows` stops silently when `ui_value_headroom().rows() == 0`, so the process-wide UI value arena is exhausted by unretired trees of other tests (order dependent). The mvd header test ends with `UI arena descendants remain borrowed` for the same reason.
- **staging-native:** `..._text_birth_denies_each_independent_axis` (full stock does not complete the construction in one turn), `..._partial_seed_expired_deadline_...` (`original Source clock declined new construction`), `..._render_owner_retires_...` (`admission birth changed its original Work or Source`), `..._identity_drafts_...` (native receiving frame left unsettled after an error).

## Files changed this round

Product: `🔎️details/✏️edit/domain_work.rs`, `🔎️details/work.rs`, `🔎️details/📨️wire/source_loan.rs`, `🔎️details/🧭️profile/retained.rs`, `🔎️details/🧶️guidance/{type-guidance,type_guidance_request,type_guidance_candidate}.rs`, `🔎️details/✏️edit/contextual_edit.rs`, `🔎️details/🔤️type/constituent_roster.rs`, `🏅️standards/🔖️4x3/.../✏️editor/{guided-work,render-work,details}.rs`, `.../staging-native/🦀️.rs`, `.../✍️apply/work.rs`, `.../👁️mvd/🧩️concepts/*.rs`, `🏅️standards/🔖️2x3/.../🪟️details/effect.rs`.

Tests: `✏️editor/{render-work-tests,tree-probe-tests,tests,guided-type-*-tests}.rs`, `✍️apply/staging-native/tests.rs`.

Ticket folder: `ifc-cargo.sh`, `ifc-one.sh`, `ifc-pkg.sh`, `ifc-retry.sh`, `ifc-2x3.sh`, this file.
