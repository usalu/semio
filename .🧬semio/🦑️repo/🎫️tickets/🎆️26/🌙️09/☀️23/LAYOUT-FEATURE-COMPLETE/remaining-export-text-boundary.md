# Remaining Export Shaping And Teardown Boundary

After the mutation derive repair, reread current source owners. The remaining export job issue is upstream of the already bounded faithful primitive painter, not the per-pixel/glyph outline loop.

## Concrete Current Boundary

Layout export `📤️export/📕️pdf-png-and-package-encoding/🦀️.rs::plan_pdf_one` still calls `build_display_list_for_resolved_frame`, then `append_pdf_display_items`. The latter sorts the complete display-list stack and copies every glyph or stroke in one job turn. Frame construction reaches `🎬️scene/🔠️style-spans-and-preview-marks/🦀️.rs::style_spans`, which allocates an owner entry per UTF-8 byte and loops across every overlapping style range, and `story_run_glyphs`, which shapes every span in one call. No per-frame outer cursor bounds this inner work.

The actual framework UI TextSystem `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖋️text/🧶️shaping-and-glyph-atlas/🦀️.rs::shape_paragraph` calls external Parley builder.build(text), break_all_lines, align and complete extract_shaped synchronously. Its public contract has no continuation or controlled disposal. `TextSystem::new` also registers all supplied fonts synchronously and allocates 2048×2048 alpha and RGBA atlas buffers even for CPU export. Layout export `PdfEngine` close currently takes/drops that complete TextSystem while reporting only the LayoutEngine stack size. That is not a bounded heap-retirement proof.

## Existing First-Party Building Blocks

The separate framework first-party font run owner `◻️2d/📝️text/🔤️font/🧵️run/🦀️.rs` exposes FontOutlineRunCursor with granted advance, real Unicode normalization/graphemes/bidi and OpenType face/layout/mark/outline children. It uses controlled paged ownership and retirement. This is the appropriate foundation for an export text plan, but its current FontRunStyle carries size/origin/relative line height/kerning only; it does not yet provide the existing UI paragraph wrapping width, alignment, tracking, or heterogeneous style spans. Replacing the current call with fixed text chunks would lose joining, bidi and line-breaking context; wrapping the synchronous call in an outer job would preserve the unbounded turn.

## Required Next Owner Work

A proper repair needs a first-party paragraph cursor over borrowed immutable story bytes and style ranges, reusing the controlled font-run children; bounded style resolution, line-breaking/alignment and glyph publication; retained original font identities for the existing fidelity stage; then direct one-glyph/primitive export publication without whole-frame DisplayList materialization or a UI atlas. All owned intermediate children must close through their original quoted retirement demands. Portable mixed-script/style/wrapping fixtures must compare against existing Parley/Unicode/font test oracles before replacement. Text byte chunking is not a substitute.

No renderer or text-system source was edited during this inspection. Native compile/runtime verification remains unavailable through the current repo frontend; the last executed repair evidence is in mutation-derive-repair.md.
