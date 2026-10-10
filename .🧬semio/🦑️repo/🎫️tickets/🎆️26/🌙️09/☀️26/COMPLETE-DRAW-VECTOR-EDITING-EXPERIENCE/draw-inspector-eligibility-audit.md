# Draw Inspector Eligibility Audit

Read-only source audit; no tests executed. Production source remained unchanged. Repository MCP unavailable; used the existing open ticket.

## Actionable Findings

- **P2 — Arrangement counts selected descendants twice.** `📌️panels/🔍️properties/🎬️actions/🦀️.rs:17,32–33` uses raw selected count, while `🎮️commands/🎛️edit-selection/🦀️.rs` `plan_arrangement` skips descendants of selected ancestors. Selecting a group and its child offers alignment although the planner has only one effective item and rejects; selecting group, child, and another sibling offers distribution although only two effective items remain. Derive effective arrangement count through an ancestor-aware traversal. Add actual-tree cases for both examples and compare offered operations with planner results.
- **P2 — Missing IDs are discarded before eligibility but ungroup rejects them.** Properties `render` resolves `selected_drawing_layers(document, ids)` before constructing eligibility. For IDs `[existingGroup, missing]`, ungroup appears from the resolved group, while `ungroup::plan` rejects the unique raw-ID/resolved-layer count mismatch. Include selection completeness in eligibility, accounting for accepted tree-row IDs and duplicate IDs; cover mixed existing/missing and all-missing input.
- **P2 — Hidden selections advertise arrangement.** Eligibility checks inherited locks but not inherited visibility. `plan_arrangement` rejects any chosen item hidden directly or through a hidden ancestor. Add a cheap arrangement-editability flag and actual-tree cases for direct/inherited hidden selections.

## Other Findings

- Inherited locks are correctly delegated to the existing first-party `drawing_layer_is_locked` helper, matching command handling. Mixed-parent filtering matches the current edit-selection planner for grouping, duplicate, delete and stack commands. `toPath` correctly permits shape/path mixtures across parents. Ungroup checks isolation, opacity and blend mode exactly like its planner; nested selected groups are intentionally supported by that planner.
- Geometry validity, finite transforms and arrangement capacity remain planner-level failures. The eligibility documentation describes known structural preconditions, so these are limits rather than additional defects in this short audit.
- Computing parent locations and inherited lock state performs repeated full-document traversals: approximately O(selection × document) beyond existing selection resolution. A shared single traversal could compute parent, inherited lock/visibility and effective arrangement count together, especially for large selections. No benchmark was run.
- The SQLite oracle independently executes the boolean rule table but mirrors the intended eligibility rules; it does not validate document-to-flags extraction or planner compatibility. The bilingual native test builds real trees, but generates them from abstract flags and therefore misses ancestor/descendant overlap, missing IDs, shape/path mixtures and hidden ancestry. Its unresolved-compositing case covers opacity only, leaving isolation and non-normal blend mode uncovered. It asserts action-ID presence and immutable document state, not command outcomes.

## Scope

Inspected Rust/TypeScript eligibility, neutral fixtures and SQLite oracle, Properties projection/native test, schema selection/lock helpers, edit-selection planner, arrangement and ungroup planner. No runtime or passing-test claim is made.
