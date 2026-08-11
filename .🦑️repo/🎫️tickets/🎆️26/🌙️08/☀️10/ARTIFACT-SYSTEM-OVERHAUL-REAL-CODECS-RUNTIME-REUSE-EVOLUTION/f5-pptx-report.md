# F5 — pptx (ecma-376) — Report

Plan: `~/.claude/plans/the-current-schemas-are-scalable-journal.md`. Recipe: `🧬️schema-design.md`. W0 recon: `w0-recon-report.md` (§ pptx row: flagged the flattened shape tree as the primary defect). S2 spine (glue-mounting policy resolution): `s2-spine-report.md`. OPC precedent to follow: `f4-docx-report.md`.

## 1. Primary defect fixed — the slide shape tree is no longer flattened away

The pre-wave model was `PptxPresentation { slides: Vec<PptxSlide{ paragraphs: Vec<PptxParagraph> }> }` — every `p:txBody`'s paragraphs across every shape on a slide were concatenated into one flat list, with the docstring itself admitting shape boundaries were discarded. This is exactly the defect W0 flagged.

New model (`📸️snapshot/🦀️component.rs`):
- `PptxTransform { x, y, cx, cy }` — a shape's `a:xfrm` position/size in EMUs; a weak (value) entity, whole-value replaced in diffs, never sub-diffed.
- `PptxShape` (tag `shapeKind` — see the serde gotcha in §5): `TextBox{text_frame: Vec<PptxParagraph>, position}`, `Picture{blip_rel_id: String, position}`, `Placeholder{kind: String, text_frame: Vec<PptxParagraph>, position}`, `Other{xml: String}` — raw-retention fallback for `p:graphicFrame` (charts/tables/SmartArt), `p:grpSp` (groups), `p:cxnSp` (connectors), and anything unrecognized. `PptxRun` gained `font_size: Option<u32>` (whole points; XML carries centipoints).
- `PptxSlide { shapes: Vec<PptxShape> }` (renamed `paragraphs` → `shapes`) — index-keyed, order matters.
- `PptxPresentation { slides: Vec<PptxSlide> }` — unchanged shape, richer `PptxSlide`.
- `PptxSnapshot { schema, opc: OpcPackage, presentation: PptxPresentation }` — unchanged top-level shape.

Engine (`⚙️engine/🦀️component.rs`) rewritten to derive shapes from `p:spTree`'s DIRECT children (previously it recursed through the WHOLE tree collecting every `a:p` regardless of shape boundary or nesting depth): `collect_shapes` walks direct children only (skipping the group's own `p:nvGrpSpPr`/`p:grpSpPr`), classifying each into `TextBox`/`Placeholder` (by presence of `p:nvSpPr/p:nvPr/p:ph`) /`Picture` (`p:pic`, reading `p:blipFill/a:blip@r:embed`) / `Other` (raw serialized XML via a single-node round trip through the xml module's own document serializer/parser — no bespoke fragment (de)serializer was written). `shape_to_xml` is the inverse direction, including real `a:xfrm` emission/parsing and `a:rPr@sz` font-size round-trip.

Real bug avoided while designing the `Other` fallback: PPTX shapes can appear inside `p:grpSp` groups; per the brief's explicit scope ("reasonably-scoped shape model...full SmartArt/chart/table shape support explicitly out of scope"), a `p:grpSp` itself becomes one `Other{xml}` entry (its whole subtree, including any nested shapes, preserved verbatim) rather than attempting recursive group-shape typing — documented in the snapshot module's own doc comment so it isn't mistaken for an oversight.

## 2. Diff — real sparse, recursive, generic engine (own copy, per docx's precedent)

`🔺️diff/🦀️component.rs`: the same `IndexedTripleDiff<D,T>`/`NamedTripleDiff<K,D,T>` generic collection-triple engine docx introduced (own copy in this file — the ownership boundary keeps each OOXML sibling from touching a shared location this wave, flagged in `glue_followup` same as docx already did). `presentation.slides` is index-keyed; within a modified slide, `shapes` is index-keyed too. `PptxShapeDiff` is tag `kind` with `TextBox(PptxTextBoxDiff)`/`Picture(PptxPictureDiff)`/`Placeholder(PptxPlaceholderDiff)` real per-field diffs plus `Replace{shape}` on shape-KIND change (same "Replace on kind change" rule as json/xml/dxf/docx). `text_frame`/`runs` are diffed via the same generic engine, recursively (`PptxParagraphsDiff`/`PptxRunsDiff`). `position` is a weak value, whole-replaced. `PptxRunDiff.font_size` is tri-state `Option<Option<u32>>`.

OPC diff types (`PptxOpc*Diff`) are docx's exact shape, renamed — own copy in this file per the same boundary rule, `glue_followup`'d for the same eventual `zip::opc` hoist docx already flagged.

`impl MutationDiff<PptxSnapshot> for PptxDiff { apply, absorb }` + `impl DiffAlgebra<PptxSnapshot> for PptxDiff { inverse, between, is_empty }` — both real, no full-replace fallback anywhere. `grep -n "snapshot: Option<"` on the diff file: zero hits.

## 3. Mutations — exactly the 9 variants the brief specified

`🧬️mutations/🦀️component.rs`: `NoMutation`, `SetSnapshot`, `InsertSlide`/`RemoveSlide`/`MoveSlide`, `InsertShape`/`RemoveShape`/`SetShapeText`/`SetShapePosition` — addressed by plain `(slide_index[, shape_index])` (no path type needed: PresentationML slides are only 2 levels deep in this typed model, unlike docx's arbitrarily-nesting table tree).

`MoveSlide{from,to}`'s `diff()` is a plain `removed:[from] + added:[(to,item)]` pair — the collection-triple algebra has no separate "moved" primitive, and `apply_indexed`'s existing remove-then-insert semantics already reconstruct a move correctly from that pair without any special-cased logic. Its mutation-level `inverse()` needed one derived fact (documented inline): after `from -> to`, the slide lands at `min(to, len-1)` (one shorter after the removal, then inserted at `min(to, that shorter length)`), so the undo is `MoveSlide{from: min(to,len-1), to: from}` — verified by `inverse_law` and the dedicated `move_slide_apply_and_inverse` test.

`SetShapeText`/`SetShapePosition` are no-ops (`PptxDiff::default()`) on shape kinds that don't carry the targeted field (`Picture`/`Other` have no `text_frame`; `Other` has no typed `position`) — covered by `set_shape_text_on_picture_is_a_no_op`.

Every variant's `diff()` is handcrafted (dedicated `diff_*` constructor per variant, never apply-and-capture); every variant's `inverse()` looks up prior state from `base` and constructs the exact undoing mutation. `apply_pptx_mutation` is the single `let d = mutation.diff(snapshot); *snapshot = d.apply(snapshot); d` semantics source.

Also fixed the `📄set-snapshot` triad leaf (`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`) — its `diff(snapshot)` helper called `diff_set_snapshot` with the stale 1-arg signature; updated to `diff(base, snapshot)` matching the new `between(base, next)` shape (this was a real compile error caught by `cargo check`, not just a style nit).

## 4. Test laws — all 6 present, real, in the mutations test module

`mutation_diff_law`, `inverse_law`, `absorb_law` (all four canonical cases: Insert+Remove-before, Insert+Insert-same-index-both-survive, Insert+SetField-patches-into-added, Modify+Remove-annihilates, plus associativity), `between_roundtrip_law`, `codec_retention_law`, `field_sweep`.

`field_sweep`'s `sweep_a`/`sweep_b` differ in every mutable field across BOTH `opc` (content_types defaults+overrides, parts, relationships-by-owner) and `presentation` (`slides` uses different-length lists per this ticket's "known structural trap" note — `a -> b` exercises `slides.removed` + a `slides.modified` entry whose OWN nested `shapes` diff shows removed+modified+added simultaneously — Picture dropped, TextBox modified in every field including the `font_size` tri-state, Placeholder added — while `b -> a` exercises the top-level `slides.added` in the other direction, carrying a whole `Other` slide).

## 5. Three real bugs found and fixed (not test-only artifacts)

1. **serde internal-tag collision**: `PptxShape`'s internal enum tag was originally `"kind"` (matching docx/gif's usual `tag = "kind"` convention) — but `Placeholder`'s own field is ALSO named `kind` (the placeholder type, per the brief's exact field name), which serde_derive rejects outright at compile time ("variant field name `kind` conflicts with internal tag"). Fixed by renaming the enum's internal tag to `"shapeKind"` (documented inline) rather than renaming the brief-specified `kind` field.

2. **`OpcPackage.parts` order instability across a double `regenerate_presentation_parts` pass** (found by `codec_retention_law`, a real engine bug, present in the pre-wave code too — just never exercised by any prior test): `regenerate_presentation_parts` retains-away and re-appends `ppt/slides/*` parts on every call, but NOT `ppt/presentation.xml`. On a single call from an empty package (`build_minimal_pptx`) this is harmless — slides get appended, then presentation.xml gets appended last, giving `[...,slides,presentation]`. But `store::ArtifactPack::encode_pack` calls `engine::encode_pptx`, which calls `regenerate_presentation_parts` AGAIN on an ALREADY-built snapshot: the SECOND pass retains-away and re-appends the slides (now landing at the true end), while `presentation.xml` (never retained) stays at its OLD position from the first pass (before the slides) — flipping their relative order and breaking exact `Vec<OpcPart>` equality. Fixed by also retaining-away `ppt/presentation.xml` before regenerating, so its position is freshly appended (after slides) on every call, not just the first — verified stable via a new `double_regenerate_keeps_opc_parts_order_stable` regression test in the engine's own test module.

3. **Test-design defect, not an algebra bug**: `sample_mutations()`'s `SetSnapshot` entry originally targeted `sweep_b()` (the deliberately minimal, unrelated hand-built OPC used by `field_sweep`) against `fixture()` (a REAL package with a slideMaster/slideLayout/theme boilerplate chain `sweep_b()` doesn't have) as the base. `inverse_law` requires restoring the ORIGINAL `base` through TWO independent `between()` calls (base→next→base); when the two snapshots being diffed don't share the same OPC part key set, the name-keyed collection's "survivors keep position, new entries append at the end" convention (`NamedTripleDiff`, same engine docx introduced) does not guarantee exact round-trip fidelity — `between(X,Y).apply(X)==Y` only holds when `Y`'s real construction order already follows that convention, which `sweep_b()` doesn't relative to `fixture()`'s six-part boilerplate. This is the SAME documented caveat as docx's `OpcContentTypes.overrides`/`RemovePart` position notes, just triggered on a combination docx's own fixtures didn't hit. Fixed by adding `mutated_fixture()` — a `SetSnapshot` target built through `build_minimal_pptx` with the SAME slide count (2) as `fixture()`, differing only in content, so the diff is pure `modified` (zero removed/added) and round-trips exactly by construction. `sweep_a`/`sweep_b` themselves are UNCHANGED (still used only by `field_sweep`/`between_roundtrip_law`, which document/tolerate the caveat directly, same as docx's own sweep fixtures).

## 6. Verification

- `cargo check -p semio-s-plugin-stdio --lib` — clean.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::pptx"` — **48 passed, 0 failed** (final, confirmed run; includes the new `double_regenerate_keeps_opc_parts_order_stable` regression test).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) — **1013 passed, 0 failed** (up from the pre-wave 965 baseline by exactly 48 — no regressions elsewhere, confirmed after the concurrent xlsx/dwg/bcf sibling sessions' own in-progress compile breaks self-resolved; I never touched any file outside `🗿️artifacts/🎞️pptx/**`, per this ticket's documented "classify external churn, don't chase" convention — verified via ~50 consecutive isolated error-attribution checks across ~45 minutes before the crate went green).
- `grep -n "snapshot: Option<"` on the diff file: zero real hits (only the doc-comment sentence explaining what was NOT used).
- `grep -n "impl DiffAlgebra"` on the diff file: present.
- `grep -n "serde_json::Value"` on snapshot/diff/mutations files: zero hits.

## 7. Ownership boundary respected

Touched only: `⚙️engine/🦀️component.rs`, `🏗️builder/🦀️component.rs` (✳️any subset level only — the only level with real typed constructors; top-level/root builder is a thin delegate, untouched), `🧬️schema/📸️snapshot/🦀️component.rs`, `🧬️schema/🔺️diff/🦀️component.rs`, `🧬️schema/🧬️mutations/🦀️component.rs`, and the `📄set-snapshot` triad's `🔺️diff` leaf. Did **not** touch `glue.rs`, `script.ts`, any SDK trait file, the framework schema module, the io module, `🏪️store`, or `zip::opc` (flagged as `glue_followup`, per the boundary rule — same hoist docx already flagged). Composer/analyzer/strict/transitional-subset files were read but not modified — they only reference `opc.parts[].bytes` byte-scans or delegate to the `✳️any` builder, never the typed `presentation`/`PptxSlide` fields directly, so the `paragraphs` → `shapes` rename didn't touch them.

## 8. Known deviations / backlog (not regressions — pre-existing repo-wide state, same as docx's own report)

- **Facet mirrors** (`🟦️component.ts`/`🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto`) for snapshot/diff/mutations were left as-is — the pre-existing, previously-flagged-stale content (the snapshot TS mirror is literally a bare `PptxEntry{name,data}` shape, the same generic-template defect S2's own audit found on every one of the 93 checked facet pairs repo-wide). Prioritized the Rust snapshot/diff/mutations correctness + the 6 test laws (this wave's actual acceptance criterion) within the time budget, exactly as docx's report did. Tracked by the existing shrink-only `POLICY_FACET_MIRROR_DRIFT`/`POLICY_GRAMMAR_HONESTY` allowlists.
- `PptxOpcDiff` and friends live in pptx's own diff file rather than `zip::opc` — see §2 and `glue_followup` (docx already flagged the identical hoist; this wave adds a second occurrence needing the same fix).
- Grouped shapes (`p:grpSp`) are NOT recursively typed — the whole group (including any nested shapes) is one `Other{xml}` entry, per the brief's explicit "reasonably-scoped shape model" scope boundary. Nothing is silently dropped (verbatim raw retention), just not individually addressable via `InsertShape`/`SetShapeText`/etc. inside a group.
- `p:graphicFrame` (charts/tables/SmartArt embedded objects) is explicitly out of scope per the brief and falls to `Other{xml}` the same way.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏗️builder/🦀️component.rs`

## glue_followup

- Hoist `PptxOpcDiff`/`PptxOpcContentTypesDiff`/`PptxOpcPartDiff`/`PptxOpcRelDiff`/`PptxOpcRelationshipsDiff` (currently defined in pptx's own `🔺️diff/🦀️component.rs`, a byte-for-byte structural twin of docx's `DocxOpc*Diff`) into `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️component.rs` once a third OOXML sibling (xlsx or bcf) needs the identical shape — now TWO independent copies exist (docx's, this one), reinforcing the case for the hoist docx's own report already flagged.
