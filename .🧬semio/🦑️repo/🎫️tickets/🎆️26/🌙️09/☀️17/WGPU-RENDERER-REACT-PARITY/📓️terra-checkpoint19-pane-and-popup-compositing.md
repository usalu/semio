# Checkpoint 19 Pane and Popup Compositing Audit

## Scope and evidence

This is a read-only production-source review of the current physical WGPU captures. No build, browser action, or production edit was made.

- [WGPU Actions pane](./🗑️generated/astra-runtime/checkpoint19-iab/journey-pane-chip-engagement-toggle-wgpu.jpg) has World grid/reference geometry visible through the Actions body and has no visible action rows.
- The paired [React Actions pane](./🗑️generated/astra-runtime/checkpoint19-iab/journey-pane-chip-engagement-toggle-react.jpg) has an opaque dark body with the full action list.
- [WGPU Appearance](./🗑️generated/astra-runtime/checkpoint19-iab/journey-settings-appearance-open-wgpu.jpg) replays General rows through/over the System, Light, Dark menu. The paired [React Appearance](./🗑️generated/astra-runtime/checkpoint19-iab/journey-settings-appearance-open-react.jpg) does not.

The style token is not transparent: the current dark `ChromePalette.panel` carries alpha `1` at [tokens](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs:675). The visible World through an Actions body therefore establishes an ordering failure, rather than a panel-alpha choice.

## Confirmed cause 1: prepared commands discard UI/World chronology

`DrawList::push_scene_pass` records the current `layer_index`, `ui_watermark`, and `vector_watermark`, precisely to retain the point where the scene pass occurred in the UI stream ([draw types](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:913)). A later `push_solid` is opaque and appends to that active layer ([draw types](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:932)).

The Shell uses that later-solid pattern for Actions and Search: it paints `theme.panel`, then renders the retained document and border in `paint_window_pane_body_step` ([Shell](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:19312)). `render_main_window_step` deliberately reaches these pane bodies after the window body scene (`phase 12` and `13`, [Shell](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23850)). The source thus intends the pane to cover the canvas, exactly as its comment says.

The prepared owner loses that intent. `PreparedRenderJob::next_draw_usage` walks every `DrawLayer` before it starts `PassHeader(0)` ([prepared packet](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2625), [same](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2661)). `advance_pipeline` freezes that flattened sequence into command pages ([prepared packet](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3109)). The real `PreparedGpuPresentPhase::Commands` executes those pages in that order ([GPU presenter](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:639)). Consequently, a body background emitted after the World scene is presented first, then the World is presented over it. This directly explains the physical Actions capture.

This is a production defect. It is unrelated to the current accessibility projection loss: no action-row semantic change can make an already overwritten opaque background or glyph layer visible.

## Confirmed cause 2: glass and foreground are globally reordered

An anchored Shell panel opens a panel glass region and then paints its border, tab bar, and document in the glass-content layer ([Shell](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24180)). `DrawList::begin_glass_content` marks the new layer with `foreground_of` ([draw types](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:991)). A live Select routes its later menu into the Shell overlay; it creates its own `Level::Menu` glass region and content layer ([Select](../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:496)). That is the Appearance menu in the recorded capture.

The real presenter first executes *all* `GlassCommands`, then executes *all* `ForegroundCommands` ([GPU presenter](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:684)). The General panel's foreground glyphs are therefore replayed after the later menu glass. They appear through the popup, exactly as recorded.

The existing `prepared_foreground_scalar_is_enclosed` exception only moves a foreground scalar back into the scene when a later region fully contains the scalar's region ([GPU presenter](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:127)). Its own current test deliberately says that a menu merely clipping a cap leaves the cap foreground ([prepared-GPU test](../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs:193)). That is insufficient for normal CSS stacking: only the part of the lower panel beneath the later menu must be occluded, while panel content outside the menu stays crisp. Sending the entire panel to the scene would blur the wrong pixels and is not a valid repair.

## Smallest coherent repair boundary

The renderer must preserve one bounded **presentation-order stream**, rather than classify the same command pages three global times. Its order must retain all of the following source facts:

1. Each ordinary UI/vector scalar is ordered around a scene pass using its recorded `ui_watermark` and `vector_watermark`.
2. A glass region is emitted at the point `push_glass` was called.
3. Its `begin_glass_content` scalar sequence follows that glass, before any later overlapping glass region.
4. The `overlay` list follows the main list, as the existing preparation walk already establishes.

The current `DrawList` contains the first fact but has no position carried by `GlassRegion`; `push_glass` stores only rect/style ([draw types](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:983)). Extend the draw-owned record with a bounded insertion anchor (active layer plus channel cursors / scene-watermark ordinal) and construct the stream during `PreparedRenderJob` measurement. The GPU cursor then consumes exactly one stream position per opportunity. It should select the correct attachment for that position: normal content before a glass goes to the scene, a glass composites into the composite, and its content immediately follows on the composite. This avoids a Shell or Select special case and preserves the existing per-opportunity fuel, credits, cancellation, and watchdog accounting.

Do not repair this by making `theme.panel` stronger, moving Actions to the overlay list, or adding an Actions-only glass wrapper. Each masks one screenshot while leaving the ordinary `DrawList` order wrong for every UI surface and does not solve a partially overlapping Select popup.

## Fail-first laws

Use the existing native prepared-GPU module [targets-wgpu-gpu-prepared-present](../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs), which already owns real prepared-presentation cursor rules, plus a small neutral fixture under its existing fixture family.

1. **Opaque pane after World.** Build a real `DrawList` with a World scene pass, then a full-rect opaque panel solid and a glyph in the same layer. Its presentation stream must schedule the scene before both later panel scalars. The actual native render/readback harness must sample the panel interior and observe the exact `Theme::panel` output, never the World/reference color. Add a React canvas/DOM visual oracle using the existing Actions pane journey: the sampled interior is panel paint and an action row label has a non-empty raster/semantic witness.
2. **Partial menu occlusion.** Build a main panel glass plus two foreground markers, then a later overlay menu glass overlapping only marker A, followed by its menu marker. The stream must order `panel glass → panel markers → menu glass → menu marker`; the output at A must be menu-owned, and the output at B must remain panel-owned. This must replace the current partial-overlap assertion that treats all of the panel as a late foreground. It proves both occlusion and crisp content outside the popup.
3. **Bounded progress and retirement.** Drive the stream one scalar per `prepared_present_step`, assert the progress signature changes for every new stream cursor, and close mid-panel/mid-menu. The exact packet, GPU cursor, and draw owners must reach their existing terminal proofs without a new frame-local clone or a global scan.

The prior source-string ladder test in `wgpu-renderer-async-boundary` only proves that the current three phases exist. It needs replacement or extension with the two behavior laws above; it cannot certify correct visual ordering.

## Confidence

High. Both defects follow directly from the current production command construction and presenter phases, and each has a matching current physical WGPU symptom. The report does not claim the action definitions are absent: their text may have additional data-path issues, but the compositor already proves why the pane cannot visually cover the World. A fresh paired browser run is still required after the renderer repair to establish complete visual parity.
