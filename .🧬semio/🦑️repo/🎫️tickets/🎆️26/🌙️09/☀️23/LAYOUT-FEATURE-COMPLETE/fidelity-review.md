# Fidelity Renderer Review

Read-only audit of the current native SVG/PNG renderer and its primary first-party font/image APIs. Native compilation and native tests were not executed: no working native build runner is available. Source inspection and defined tests do not establish a compilation or runtime pass.

## Actionable Findings

### P1 — Fidelity Retirement Is Unreachable From Interactive Close

File: `⏭️advance-commit-and-codec/🦀️.rs`, line 218.

The close dispatcher sends OutputChunks, MediaCredit, Snapshot and SnapshotOwner through close_original_retained_step, but omits Fidelity. The correct Fidelity branch exists in close_paired_turn at line 128. Every normal close reaches Fidelity from Interchange (`🏗️job-admission-and-close/🦀️.rs`, lines 360–364), then takes close_export_step, which returns an error for Fidelity. This blocks terminal retirement after successful export or cancellation, including kinds without any fidelity state. Add Fidelity to the quoted-retirement dispatcher and execute both ordinary completion and cancellation close laws. Verify the reserved-job close route delegates through the same corrected dispatch.

### P1 — Rotated Text Frames Lose Their Transform

File: `📕️pdf-png-and-package-encoding/🦀️.rs`, lines 70–72.

Glyph rotation and center are inferred by finding the containing frame in list.rects. The resolved-frame export builder calls assemble_display_list_items with chrome_blueprint=false (scene `⚙️layout-engine-and-views/🦀️.rs`, lines 127–128). Text frame rectangles are added only when chrome_blueprint=true (scene `🧩️assemble-display-list/🦀️.rs`, lines 62–65). Therefore exported text runs have no corresponding rectangle and silently receive rotation=0 and center=[0,0], even when their authored frame bounds rotate. Carry explicit frame transform with the display run or export item. Add a rotated text frame artifact test with independently expected glyph coordinates/pixels; the present direct-glyph oracle uses rotation=0 and cannot catch this planning defect.

## Repaired Finding

### P1 — Missing Edge Stroke Field (Repaired By Main Agent)

File: `🎨️faithful-primitives/🦀️.rs`, line 13.

Initial inspection found Edge declared only a and b while flattening constructs Edge { a, b, stroke } at lines 161 and 167–171 and sampling reads edge.stroke at line 187. This is a concrete Rust field/type error. Main agent added the boolean field; a fresh source read confirms its presence. This is source-level repair evidence only, not a compilation pass.

## Verified Source Properties And Limits

- Font outlines are genuine first-party GlyphOutlineCursor output rather than glyph rectangles. FontFaceAdmissionJob::new/advance/take_result, AdmittedFont.source/face, FontFace.units_per_em and GlyphOutlineCursor::advance/result/retained_point_count calls match their primary definitions in framework font and outline modules. ExportFont ownership is wrapped behind first-party FontHandle and RetireOwned.
- ImageDecodeInput fields and ImageDecodeJob::new/progress/advance/take_result signatures match the primary pixels image decode implementation. Source RGBA sampling inverse-rotates positions, bilinearly interpolates premultiplied colors and alpha, then performs source-over compositing.
- SVG streams original line/quadratic/cubic commands; PNG uses bounded curve subdivision and 64 edge operations per sample turn. Implicit fill closure is represented separately from stroke edges after the repaired stroke field. Curves, segments, fonts, image data, decoded pixels and outlines have explicit limits. Unsupported image/font sources fail explicitly.
- Painter order follows stack_pieces for each background/frame batch and appends batches in planned frame order. Page clipping is supplied by the SVG viewport and PNG raster bounds. There is no per-frame clipping representation in the inspected DisplayList/PdfItem APIs; the implementation should not claim frame clipping from this audit.
- Whole-frame shaping, display-list conversion and its temporary sorting/cloning remain synchronous within a work unit (encoding file lines 24–25, 33–35 and 46–76). The fidelity preparation and PNG pixel/edge work after that point are incremental. This is already disclosed in fidelity.md and is not a verified strict latency bound for arbitrary admitted frame content.
- Checkpoints store completed work and replay the prefix. Native laws for outline witnesses, artifact exports and checkpoint/cancellation are defined, but are unexecuted here.

## Other Main-Agent Repair

Main agent reported correction of the fidelity corpus native document wire fields from width/height to w/h (including text insets). This report did not independently rerun the native-document decoder and makes no additional decoder-pass claim.
