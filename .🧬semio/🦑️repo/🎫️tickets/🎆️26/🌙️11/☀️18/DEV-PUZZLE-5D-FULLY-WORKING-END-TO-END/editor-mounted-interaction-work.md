# Editor interaction publication Work

`created_part_board_and_world_edits_reopen_with_history` reached `addPartKind 0` and failed while publishing its interaction state. The focused editor binary refused the VCS identity operation because original Work requires its caller loan.

The refusal breakpoint in the rebuilt editor ELF was RVA `0xda61d9d`. Matching its stack's instruction windows to the unstripped V1 ELF located `OriginalWork::prepare`, `NativeEncodeControl::with_effect`, `OperationIdentityCursor::commit`, interaction store replay, `RenderCtx::apply_interaction_writes`, and `TypedCtx::publish_mounted_typed_interaction_unit`. The mounted publication code resumed the correct app-owned identity receipt but left its Work lender empty. The earlier fixture identity correction therefore did not reach this separately resumed authority.

Mounted publication now resumes that receipt at each interaction authoring call or derived-child follow, lends the current turn's native encoder Work, and posts the Work performed before the next publication unit can inspect its budget. It restores receipt and progress before returning even when authoring reports a fault. Original emission supplies its existing caller authority through the same pump. The two routes share the publication units.

Bun original-emission and publication-stall conformance passed all six tests, including the independent SQLite authoring-mode fixture and the ordered current-turn lending and fault settlement law. Rustfmt parsed the edited Rust module and the scoped diff check passed. The native history replay needs to run on the rebuilt artifact to verify the downstream history and reopen assertions.
