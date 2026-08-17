# WGPU Silhouette Content Handoff

## Implemented

- Added renderer-neutral nested silhouette clips through `DrawList::begin_silhouette_clip` and `DrawList::end_silhouette_clip`.
- Changed the shared depth attachment and UI/vector/world/translucent/line pipeline family to `Depth24PlusStencil8`.
- Added a color-write-disabled silhouette mask pipeline. It writes reference `1` with `Replace`; all content pipelines test `Equal` and preserve stencil.
- Each layer uploads and draws its content once. The mask is the body-and-owned-chip union, so no UI, raster, vector, 3D, overlay, glass-foreground, hover, or hit payload is admitted through a gap.
- Mask reset is bounded to the union bounds of the previous and current masks. Render-pass boundaries clear stencil, and empty/pending clips emit no reference-one mask.
- Preserved ordinary nested scissors by intersecting them into mask geometry. Fractional rectangles use outward pixel coverage to avoid seams.
- Replaced the rectangular Dock cap glass with one glass region per measured tab and one controls glass region. Removed the cap-gap hit target and rectangular cap fill.
- Replaced `WindowSilhouette { tabs_w, controls_w, cap_h }` with a Rust mirror of `window-silhouette-geometry/v1`: normalized arbitrary merged top and bottom spans, constrained depths, body/glass/content regions, containment, safe clearances, and general top/bottom outline painting.
- Dock projects measured tabs and controls into the normalized top edge. Tests use the TypeScript fixture coordinates for top and bottom glass, span merging, safe clearances, and containment.
- Generic/document UiNodes retain the body-safe initial layout. Known edgeless scene/canvas surfaces start at full silhouette bounds. Both use the same silhouette stencil and clipped hit regions.

## GPU Contract

For every clipped layer in a render pass:

1. Clear stencil at the render-pass boundary.
2. Reset only the prior/current mask union bounds with stencil reference `0`.
3. Draw all disjoint body/chip mask rectangles with reference `1` and no color writes.
4. Draw each content batch once with stencil comparison `Equal(1)`.

The implementation is shared by native and WASM source paths; no platform-specific stencil code or manifest flag was added.

## Panel And Pane Audit

The current WGPU panel path at `Shell::render_panels` paints one ordinary rectangular `Level::Panel` glass region. Engagement/measures/search rails likewise paint ordinary rectangular `Level::Pane` cards. No WGPU panel/pane implementation currently owns chip-gap or notched silhouette geometry, so there is no additional panel/pane silhouette path to wire. Coverage is therefore explicit for Dock windows; future notched panel/pane chrome must construct a `WindowSilhouette`-equivalent region union and enter the same `DrawList` clip API.

## Files

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Dock/🧊️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`

## Verification

- `CARGO_TARGET_DIR=<ticket>/🎯️target-wgpu cargo check -p semio-framework-ui --features wgpu-engine`: passed in 4m07s; only pre-existing warnings.
- `CARGO_TARGET_DIR=<ticket>/🎯️target-wgpu cargo check -p semio-framework-os-renderer-wgpu --message-format=short`: a definitive warm rerun passed in 30.61s; only existing warnings were emitted. This supersedes the earlier cold-build `E0432` diagnostic and the inconclusive filtered run.
- `CARGO_TARGET_DIR=<ticket>/🎯️target-wgpu cargo test -p semio-framework-ui --features wgpu-engine silhouette --lib`: blocked before tests by 93 unrelated existing lib-test compile errors in `engine.rs`, `component.rs`, `cursor.rs`, and `reconcile.rs` (label conversions, stale scene fixtures, missing test imports). Four new-test import errors reported in that run were corrected afterward.
- `bun nx run @semio-tech/ui-rs:check`: blocked before Rust compilation by the pre-existing stale generated `ui-axes.ts` artifact.
- The earlier `bun nx run @semio-tech/framework-renderer-wgpu:test-quick` was interrupted with exit 130 while validating the superseded scissor implementation and never reached tests.
- `rustfmt --check` parsed the three changed Rust files but reports repository-style formatting differences.
- WASM compilation was not run because it would require another broad cold build. Runtime GPU/stencil behavior was not visually exercised in this workstream.

## Coordination

- Another workflow staged intermediate snapshots of all three Rust files while this workstream remained active. This workstream ran no modifying Git command; current files are `MM` and include later unstaged stencil/geometry corrections.
- The ticket remains open as requested.
