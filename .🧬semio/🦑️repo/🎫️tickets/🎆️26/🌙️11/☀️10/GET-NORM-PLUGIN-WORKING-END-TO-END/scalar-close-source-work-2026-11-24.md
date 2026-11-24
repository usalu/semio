# Scalar close Source/Work and controlled retirement (2026-11-24)

Status: symptom 2 no longer reproduces and symptom 1 could not be reproduced in-process. One real blocker for every Rust test of the plugin stack was found and fixed. No framework retirement invariant was changed. The full `semio-hub-norm` suite is NOT verified because another agent's in-flight edit broke `semio-framework-value-native-record-machine` (E0310/E0311, `N` may not live long enough) during my run.

## Results

| Check | Result |
| --- | --- |
| `semio-hub-norm --test surface_render` (`norm_public_surfaces_render_all_declared_bodies`, `every_norm_editor_action_is_migrated_onto_the_shared_owned_factory`) | 2 passed (run at ~02:25, after my natural-file move) |
| Same run, earlier targets of the full `semio-hub-norm` suite (12, 1, 2 tests) | passed; last target not built (record-machine compile break by another agent) |
| `semio-framework-value-native-schema-engine --lib` | 3 passed, 1 failed (`schema_key_vacant_seed_fields_stay_individually_bounded`: seed parts 74 vs `key-query.json` 90, unrelated key-frame fixture drift) |
| en1997 editor integration | not re-run; the last log (`en1997-editor-r5.log`) has 3 failures, see below |

## Blocker found and fixed: `semio-plugin-decl-app` source boundary

Every `repo cargo` run (mine and the en1997/en1999 agents') stopped before the compiler with `semio-plugin-decl-app exceeds its source boundary: 28 of 32 files, 264887 of 262144 bytes`. I summed the real file set from the crate's dep-info: the 37-line `NaturalFileDecodeCursor::close_chained` default method (2743 bytes) added to `🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs` pushed the crate exactly over `maxSourceBytes`. That method was only used by `📥️natural-file/🦀️.rs`, which belongs to `semio-plugin-decl-surface` (about 202 KB of 262 KB used).

Fix (no behaviour change): `close_chained` and `NATURAL_CLOSE_CHAIN_ACTIONS` moved out of the trait into a private free function in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs` (`close_chained(&mut **owner, effect)`), the trait in `🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs` is back to its committed content. The grant arithmetic is the same (one `left` closure replaces the two identical literals). Another agent was simultaneously extracting a `semio_plugin_decl_natural_file_media` crate from that file (a transient `E0432` in `🧩️app/🦀️.rs:157` appeared while the crate was not yet linked); that extraction is theirs and the move above stays compatible with it because the trait no longer carries the method.

## Symptom 2: controlled retirement abandoned before terminal-empty

The panic is the `Drop` guard of `ControlledRetirement` (`♻️retirement/🎮️controlled/🦀️.rs:389`). It fires when an error return (here the scalar close refusal of symptom 1, or a transient mismatch in the in-flight loan/receiving edits) drops a retirement owner that has not been retired; it hides the first error. After the `01:20` run (`normtest-rs-test-h6.log`) other agents changed `work-action/🤝️loan`, `🌳️source/🦀️.rs` (their `[DEBUG] scalar close` print came and went; the committed `close_dispatch` is unchanged) and the paged/list code. With that state plus my move, `surface_render` passes, so the abandonment was a consequence of the refused close, not a separate defect in `ControlledRetirement`. I did not weaken the `Drop` assertion.

## Symptom 1: `scalar close changed Source or Work`

Analysis (from `close_dispatch`, `🏭️producer-engine`, `🫴️borrow`, `🧵️job/⏱️context`):

- `invocation.source_turn()` is the Source the outer producer action was created with (`drive`: `producer.source_turn(context)`, which must equal `effect.work_source_turn()`). It can only differ from the effect's Work Source if the outer producer's context reports a different Source than the Work loan, or reports `None`.
- `close_dispatch` additionally refuses `invocation.source_turn().is_none()`. The retirement producers of the job path (`StepContext::with_native_retirement`, `🧵️job/⏱️context/🦀️.rs:229`) bind the Work loan to `self.original_source_turn_identity()`, which is `None` for any `StepContext` created by `borrow_context` instead of `borrow_context_for_turn`. `FactoryOwnedProducer` (`♻️retirement/🏭️factory/📦️owned/🦀️.rs:21`) forwards `effect.work_source_turn()` unchanged. So a Source-less retirement effect reaches the scalar close and is refused; and `OriginalWorkBindingIdentity::follow_between_actions` (`🫴️borrow/🦀️.rs:18`) only re-binds a resident holder when the new effect carries a Source, so a Source-less holder cannot follow the next turn's scope/issuer either (the en1997 agent's note).
- I could not reproduce it in-process: the in-process runs use Work recipients with a Source (`EffectBank`, `OriginalJobNativeWork`), and the failing guests (`en1990/1991/1997/1998`) were built from an earlier state. A fix that relaxes `is_none()` or makes `follow_between_actions` accept Source-less holders changes the semantics of about 20 crates (`🔗️read`, `📽️projection`, `🧬️retained-clone`) without a failing test, so I did not make it. If the rebuilt guests still refuse, the capturing call is whichever `StepContext` is created through `borrow_context` for the native-return close; either thread the actor Source into that context (`borrow_context_for_turn`), or allow a Source-less holder to follow a Source-less effect (`effect.work_source_turn().is_some() || self.source.is_none()` in `follow_between_actions`/`admits_between_actions`), with a unit test in `semio-framework-value-control-contract`.
- No unit test was added: nothing fails in-process, and a test that asserts the current refusal would pin the regression instead of fixing it.

## en1997 editor failures

`undo_redo_round_trips_through_the_wrapper` (settlement stops after 4098 turns), `history_edits_end_to_end` and `conflict_history_edits_end_to_end` (archive load did not settle) are turn-budget/settlement failures of the archive load, not scalar close refusals; they are the target of the chained close (`close_chained`) the other agent introduced. Not re-run after the move.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs`: added `NATURAL_CLOSE_CHAIN_ACTIONS` and `close_chained` (lines ~42-62), call site `close_chained(&mut **owner,effect)` (~line 628), imports of `NaturalFileDecodeCursor` and `retained_clone_grant_admits`.
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🌳️source/🧪️tests/key-settlement.rs`: removed the stale `use crate::record::{settlement_of,Settle};` (the symbols come from `use super::*`); the lib test target of `semio-framework-value-native-schema-engine` did not compile before.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs`: the other agent's `close_chained` removed again (back to committed content).

## Guests needing a rebuild

All Play guests whose routes failed (`norm-eurocode`: en1990, en1991, en1997, en1998), since they embed the framework value/retirement code built before the current state.
