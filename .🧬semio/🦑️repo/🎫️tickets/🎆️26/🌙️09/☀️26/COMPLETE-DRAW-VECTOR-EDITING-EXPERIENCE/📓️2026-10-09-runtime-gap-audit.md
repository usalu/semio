# Draw Runtime Gap Audit — 2026-10-09

Read-only current-source audit. No tasks, tests or live application were run. Parent owns ticket coordination. Historical acceptance reports were not treated as current runtime evidence.

Paths below are relative to C:/git/semio. D = ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any. N = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs.

## Confirmed Source Gaps

1. PNG document export is absent from the end-user command. D/✏️editor/🎮️commands/📤️export-document/🦀️.rs:39–52 dispatches only PDF and SVG and rejects every other format. The artifact-level export catalogue separately advertises stdio.png (drawing/🦀️.rs:474), and a PNG serializer exists. These are distinct capabilities; presence of serializer is not evidence that the command can download PNG. Add a command-owned bounded raster/PNG job with progress/cancellation and declare PNG in the format argument options.

2. Add Image creates an unresolved resource reference. D/✏️editor/🎮️commands/➕️add-layer/🦀️.rs:31–37 calls the generic kind factory; D/🧬️schema/🦀️.rs:836 creates the image with key image-source. Handler lines 59–65 emit only create-layer and selection, with no asset admission/import. Existing image admission/emission jobs in D/🚪️io/🖼️image/🦀️.rs are genuine lower-level facilities but do not turn this UI command into encoded image import. A proper action must obtain a source, admit samples, publish import-image-asset and create-layer atomically, and select the result.

3. Native image paint loses affine geometry. N:7284–7294 computes transformed corners, then reduces them to an axis-aligned bounding rectangle for push_raster_quad. Picking uses the transformed quadrilateral in D/🧬️schema/🎬️scene/🎨️paint/📋️prepare/🎯️query/🦀️.rs:54. Rotated/sheared images can therefore paint outside their pick region; replacing the rectangle with a quad carrying affine UVs is an independent native renderer lane.

4. Native text paint loses rotation/shear and reduces gradient paint. N:7264–7282 maps each line origin and derives scalar font pixels, then invokes draw_text without the layer affine transform. Linear/radial text fills use one sampled midpoint colour (7270). Picking explicitly uses TextFallback quadrilaterals, not shaped glyph ink: D/🧬️schema/🎬️scene/🎨️paint/📋️prepare/🦀️.rs:42 estimates columns × size × 0.6 and lines × size × 1.2; query line 54 accepts this quad. This is concrete semantic/rendering mismatch, not a claim that text is wholly absent.

## Already Present; Do Not Reimplement Without Runtime Evidence

D/✏️editor/🦀️.rs:2259 declares window_kind_initial_utility(selectDirect). Lines 92–100 prefer addressed window ownership, focused ownership, flat field then fallback. Framework 🛂️manifest/🪛️utilities/🌅️initial/🟦️.ts:14–17 preserves an explicitly owned null and refuses unknown initial utilities. Thus the older assertion that Draw has no initial utility API is stale. This audit did not prove host boot or overflow behaviour; no concrete overflow source defect was identified. Narrow host runtime checks are required before changing these.

Native image source upload, native text drawing, semantic image samples and source admission already exist. Avoid replacing these with another pipeline.

## Clean Next Execution Lanes and Evidence

Prioritize PNG export command and encoded image import as separate schema-first command lanes. Native affine raster/text paint can proceed independently in N and renderer tests; it does not require editing scene algorithms. Keep utility initialisation/overflow as runtime verification rather than speculative rewrite.

Language-agnostic fixtures should cover: PNG selection/full-page framing, transparency and cancel before publication; image import with missing/corrupt source and atomic no-mutation refusal; a four-colour 2×2 raster rotated 30 degrees and sheared, with points inside the AABB but outside the true quad; multiline text with rotation, nonuniform scale, Unicode and gradients; fresh canvas default utility, explicit cleared null, second pane ownership, narrow toolbar overflow choice and focus preservation. Independently decode exported PNG with an existing third-party decoder; independently compare raster affine sampling to platform Canvas2D; use font shaping reference for text rather than duplicating the 0.6 estimate.

Runtime must capture actual native image/text screenshots and coordinate picks plus [DEBUG] command publication/cancellation logs, then verify saved/downloaded output. No currently passing tests or working runtime behaviour is claimed here.
