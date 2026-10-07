# Native Wire and Point Paint Audit

Read-only source audit on 2026-10-06. No runtime, tests, build, or GREEN claim. Concurrent repairs may subsequently move these lines. `World` below means `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`; `Draw` means `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`; `Math` means `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`.

## Observed Source Facts

- **Pure points:** Lower schema now accepts nonempty Positions plus VertexIds and zero edges/indices (`Math:833`). World inline admission also permits zero indices with nonempty positions (`World:9532`). However original point paint is gated by `selection_targets.vertex || granularity == "vertex"` (`World:10524`), then requires a real VertexId, and suppresses neutral points when vertex targeting is false. Default object-mode has no unconditional point marker submission. Indexed surface paint cannot make zero-index points visible.
- **Wires with positions:** Original `append_component_overlays` (`World:10442`) already emits real `mesh.edge` endpoints through `LineVertex3d`. Its outer gate is paint mode, component mode, environment outline, edge targeting, or selected mesh components. `show_edges` defaults true (`World:2219`) but only enables an environment-derived outline color. In ordinary object mode, setting showEdges false can suppress wire-only geometry entirely; a true toggle still needs the outline color to resolve. The semantic edge-ID check is only bypassed when outline color exists.
- **Edge-only MeshData without positions:** World inline admission rejects `vertex_items == 0` (`World:9533`), and lower schema rejects zero packed vertices (`Math:833`). `WorldMeshSurface::position` reads only source positions (`World:9951`). Existing edge stream writes remain separate from Positions. Sampling genuine edge endpoints into the original packed stream is needed before this path can admit such data.
- **No synthetic triangles in lower zero-index submission:** Upload preserves `schema.indices` as GPU `index_count` (`Draw:711`); it creates an index buffer of zero requested bytes when the schema has zero indices (`Draw:677`). The original World index phase explicitly skips writing when `index_items == 0` (`World:9711`). Normal retained instance/shadow draws use `draw_indexed(0..mesh.index_count)` (`Draw:4510`, `4575`), and material draws clamp their ranges then draw only `first < end` (`Draw:4721`). These source paths do not turn Positions into implicit triangles. GPU acceptance of zero-size buffers and binding remains runtime-unverified.
- **Existing original primitives:** Point crosses already use `vertex_marker_half_extent` and three `push_line_segment` calls (`World:10220`, `10549`), keeping a 6px base marker / 11px emphasis marker. `vertex-marker` is also a pinned original placeholder mesh (`World:10183`, `8557`), but the audited component point paint uses line crosses. World submits overlays into the original ScenePass3d `line_draws` (`World:13674`, `13826`). Lower original LineList pipeline exists (`Draw:3358`), and the production scalar encoder accepts exactly two endpoints and submits `draw(0..2, 0..1)` (`Draw:4835–4877`). The bulk prepare/draw helpers around `Draw:3960–4113` are test-gated; they must not be mistaken for production proof.

## Recommendations for Original Owners

1. In `WorldPlaceholderMeshCursor::inline/begin` and original position sourcing, append/sample real edge endpoints into packed Positions when source Positions are absent; retain zero triangle indices and retain real EdgeIds separately. Do not fabricate VertexIds for endpoint samples. Ensure all packed per-vertex fields and AABB follow the admitted render vertex count.
2. In `append_component_overlays`, paint `schema.indices == 0 && schema.edges > 0` as primary geometry independently of showEdges and component selection. Keep showEdges as a surface-outline preference. Reuse original line submission and original object styling/colors.
3. In that same owner, paint real identified points for `schema.indices == 0 && schema.edges == 0` independently of selection targets/granularity; reuse the existing screen-size cross marker and require genuine VertexIds. Preserve original selection/hover emphasis as overlays.
4. Observe actual default object-mode runtime separately for all three supplied cases, including showEdges false for wires, zero-index GPU admission, visible marker size, and framing/culling. Source admission and line encoder existence alone do not establish visible runtime paint.

## Precise Source Seams

- `World:9529–9560`: inline admission, schema counts, original source vertex mapping.
- `World:9668–9717` and `9951`: packed Positions and guarded zero-index writer.
- `World:10442–10557`: wire gate and point marker gate/neutral suppression.
- `World:13674` and `13826`: overlays to original ScenePass3d line submission.
- `Draw:677–711`, `4575`, `4721–4724`, `4835–4877`: exact index counts, indexed paint, production line encoding.
