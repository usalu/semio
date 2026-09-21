# Prepared Command-Stream Compositor, Clip, and Depth Audit

## Scope and evidence

Read-only source audit on 2026-09-21 after the command-stream presentation change. This report does not claim a fresh browser result. It identifies source-proven behavior and separates generic risks from a current physical regression.

The audited runtime path is the production prepared path:

- [GPU cursor](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:539) executes one prepared command at a time.
- [Prepared command order](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2625) comes from `DrawMeasureCursor`.
- [Draw list boundaries](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:914) own scene, glass, scissor, and silhouette state.

## What the new glass stream gets right

For an authored glass cursor, [Commands](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:565) switches to `SnapshotBackdrop`; that phase copies the current composite into the scene source, and only then advances optional blur mips. [CompositeGlass](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:584) draws that one glass region back into the composite before the next authored command. Later glass commands therefore see earlier UI, scenes, and glass output. The code does not reuse a frame-start snapshot.

That is the required retained ownership boundary: one authored glass owns one snapshot point; its foreground is authored after its glass command. The existing `prepared-glass-stacking` fixture and `prepared_glass_and_foreground_follow_authored_partial_and_nested_stacking` law are the nearest prepared-order route.

## Confirmed command-order defect: overlay before scene

`DrawList::push_scene_pass` assigns the scene to the current layer and creates a following layer ([types](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:930)). The intended authored sequence for:

1. normal A,
2. overlay-routed B,
3. scene,
4. following normal C,

is **A → scene → B → C**. B belongs to the pre-scene layer and C belongs to the following layer.

The prepared cursor currently walks all six layer channels—normal UI/vector/raster, then overlay UI/vector/raster—before it reaches `next_after_layer` and emits `PassHeader` ([prepared](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2834)). It serializes **A → B → scene → C**. `ui_watermark` and `vector_watermark` are recorded on the scene, but this cursor does not consume either.

A retained overlay node is a real producer: [UI paint](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2251) opens and closes the overlay route. I did not establish a fresh browser case where one such overlay contains a scene, so that browser manifestation remains unproven. The DrawList protocol itself is concrete and must not encode the wrong order.

**Fail-first law.** Extend [`a_scene_pass_is_prepared_between_the_ui_scalars_authored_around_it`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1042) and its neutral `prepared-scene-ui-stacking` fixture with an `overlay-solid` operation. The prepared command cursors must order `A, scene, B, C`; an independent TinySkia oracle should sample an overlap pixel for that same order. The repair belongs in prepared cursor staging, not in a global post-frame overlay replay.

Sol Tree owns this order law and repair packet.

## Confirmed production clip omission

The Dock uses a disjoint silhouette union for live body content: [`render_stack`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1687) enters `begin_silhouette_clip`, pushes a full content-bounds fill, renders the active body, and exits it. The exact union intentionally excludes the cap gap; the existing Dock law records that geometry ([test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:165)).

In production prepared rendering, UI and vector encoders receive only `layer.scissor` ([GPU dispatch](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:658); [UI encoder](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:4082)). They never receive `layer.clip`. The existing effective-clip and stencil-mask implementation is test-only ([draw](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2809), [pipeline](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2986)). Therefore a prepared Dock body can paint the full bounding rectangle through a silhouette cutout.

This is a production-reachable source defect. A fresh physical screenshot is still needed to measure its current severity after compositor changes.

## Glass and scene clip boundaries

`GlassRegion` stores only its layer index and visual style ([type](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:28)). Its encoder accepts no scissor or clip and draws the complete rectangle ([glass encoder](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:4594)). The builder deliberately captures current scissor/clip in the glass layer ([builder](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs:993)), so this is a lost retained constraint. I found no current Shell caller that pushes a glass region while already clipped; context-menu content starts its scissor after the glass. The generic DrawList contract is nevertheless broken.

Likewise, scene color encoders scope only to `ScenePass3d.viewport` ([world instance](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:4266)), never to the owning DrawLayer's scissor or silhouette. A clipped component scene needs that layer boundary propagated to its color work. Shadow work remains one full shadow job: a screen-space clip must not multiply or clip the light-space shadow map.

## Bounded clip-piece repair contract

The proposed cursor-owned `clip_piece` protocol is correct if it follows these rules.

- Resolve the current command's owner layer. A missing clip yields exactly one piece: its inherited layer scissor. A present empty union yields zero pieces: make no color, snapshot, blur, or glass submission and advance the command.
- For each nonempty union member, derive one logical `layer.scissor ∩ clip_piece`. Convert once with the existing logical-to-physical helper. This preserves current fractional-DPR policy and avoids double rounding.
- A World color scalar additionally intersects the logical scene viewport before that conversion. Apply it to opaque, material, textured, grid, and line color commands. Render the shadow begin/casters exactly once, outside the screen clip.
- Snapshot and blur a nonempty glass command once. Composite every disjoint clip piece against that same snapshot; increment its command only after the final piece. `GlassRegion.layer_index` identifies the owner layer.
- `ClipRegion` requires non-overlapping members, so repeated alpha or translucent world work cannot double blend a pixel. Preserve that invariant at construction.

The cursor progress signature must include `clip_piece`; otherwise the presentation watchdog can report a non-moving command during a valid multi-piece command.

**Neutral fail-first fixture.** Add rows to the existing `prepared-gpu-opportunity` fixture and GPU prepared-present test suite:

| Case | Required observation |
| --- | --- |
| no clip | one color encode |
| two disjoint pieces | two color encodes, one command advance |
| empty clip | zero color encodes and no snapshot |
| scissor ∩ two pieces at DPR 1.5 | exact one-time physical conversion, no seam outside either result |
| one glass plus two pieces | one snapshot, optional blur chain once, two glass composites using that snapshot |
| World viewport ∩ piece | no encoded pixel outside both rectangles; shadow begins once |

Use the existing Dock silhouette fixture as the first host-level producer and the existing React Dock geometry oracle for the same cap-gap samples.

## Depth boundary

A prepared frame clears shared depth/stencil once ([GPU](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:550)). Each World color scalar subsequently loads it ([world instance](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:4272)). UI is safe after World because the UI pipeline uses `depth_compare=Always` and does not write depth ([pipeline](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2926)).

Two overlapping ScenePass viewports still share depth. The later scene can fail its depth test against the earlier scene rather than cover it according to DrawList order. I found no current main-window overlap witness, so this is a generic source risk, not a declared current runtime bug. If overlapping scene hosts are supported, add one per-ScenePass viewport depth/stencil clear before its first color scalar, once per pass and never per clip piece. A two-overlapping-scene GPU pixel law is the admission test.

## Required runtime checks after implementation

1. Open a multi-tab Dock and sample one cap-gap pixel. It must equal the surrounding background, never the active body's canvas fill or World output. Compare the same coordinates in React and WGPU at DPR 1 and 1.5.
2. Open a menu or dialog above World content. Its glass background must sample the already-composited backdrop, and its labels must remain above the scene.
3. Exercise nested glass surfaces. The inner surface must include the outer glass result in its backdrop; closing either must not reveal stale glass.
4. Use a fixture with an overlay-routed node and a World scene to verify **A → scene → B → C** in both command receipt and pixels.
5. If overlapping World hosts are valid, verify the later host covers the first independent of camera depth.

No source changes or tests were run for this audit.

