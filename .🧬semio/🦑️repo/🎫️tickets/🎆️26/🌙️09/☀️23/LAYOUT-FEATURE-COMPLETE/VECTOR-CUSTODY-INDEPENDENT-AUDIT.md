# Independent Vector And Custody Audit

Read-only source audit of the genuine Layout Drawing graph consumer, original child preparation, and qualified custody provider. Native compilation remains unrun; findings below distinguish declaration-level errors from source-level budget defects. No production sources, AGENTS.md, or git state were changed.

## Findings

1. **P1 — Native resumed text checking calls a method absent from its trait.** Drawing composition native source line 74 calls `plan.content.as_ptr()`. DrawingTextPlan.content (drawing native line 15) is `&(dyn Utf8Text + Sync)`; the authoritative framework value paged Utf8Text trait declares only text_bytes, text_chunk_count, and text_chunk. This is a type-check failure, not a grammar issue. capture_label at composition line 66 already uses an explicit trait-object data-pointer cast, so the resumed comparison also must use an actual supported identity operation.

2. **P1 — Native text-copy quote cannot fund the quoted turn.** Composition next_demand phase 2 at line 67 takes the planner loan demand and sets copy_bytes to max(loan demand, min(4, remaining text length)). The phase 2 implementation at line 74 first pays cursor.text, then copies original text using the remaining grant. With the exact advertised copy-byte grant, the paid graph/text loan consumes the entire copy budget, leaving zero bytes for any nonempty chunk; end equals source_chunk_at and the funded turn returns an authority error. The quote needs the sum of the original loan and at least the next complete UTF-8 scalar, with the corresponding item budget, or loan and copy must become separate funded turns. Test a nonempty ASCII and a multibyte original label with exact next_demand grants.

3. **P1 — Native planner drops paid preparation failures from its own receipt.** Drawing native line 47 (Prepare) and line 48 (Walk/Assets) invoke preparation.advance with `?` before self.pay. The same happens in text at line 52 and close_step at line 56. OriginalChildVectorPreparation deliberately attaches actual child progress plus its wrapper movement to errors (preparation lines 16, 20–30). When it returns a paid error, planner.last remains zero, its own known header movement is absent, and composition lines 74/81 reads that zero last_receipt and converts the original ValueError to String. The operation ledger loses real copy/allocation/release progress on error. Capture the error receipt, add planner movement, publish last, and retain that receipt through the consumer error path. Existing preparation paid-error laws do not cover this outer consumer propagation.

4. **P2 — Portable text loan omits parent-source validation.** Drawing portable text at line 54 names its parent argument _snapshot and returns activeText without the original-parent checks used by advance line 50. A funded text call with a substituted parent snapshot or changed original member handle returns the previous text rather than refusing substitution. Perform the same snapshot/member validation before returning that loan; add a parent substitution law during an active measureText pause.

## Exact Sources

- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🖼️composition/🦀️.rs
- C:/git/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🟦️.ts
- C:/git/semio/🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧵️vector/🧳️prepare/🦀️.rs
- C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪆️child/🎯️editor/📖️read/🧩️admission/🦀️.rs
- C:/git/semio/🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs

The provider static registration bound was checked against RetireOwned: its supertrait already requires static, so no missing generic lifetime defect is reported.

The independent audit did not run Native tests or claim full type checking. Known host read injection, boolean/trace/raster resolution, historical raw-codec test migration, and original command-binding gaps were deliberately excluded.
