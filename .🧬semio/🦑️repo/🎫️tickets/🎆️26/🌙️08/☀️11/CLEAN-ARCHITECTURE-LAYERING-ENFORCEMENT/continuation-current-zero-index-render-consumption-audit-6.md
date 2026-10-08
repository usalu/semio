# Current Zero-Index Render Consumption Audit 6

Observed 2026-10-07T01:41:59.159484+00:00. This is read-only source evidence bound to actual current production bodies and the surviving epoch-6 Board held model. No production write, compiler or GPU rendering command was executed by this audit. The minimum physical index buffer is a staging proposal; current production still equals its admitted before body. This audit does not claim screenshot/pixel proof.

## Bound source identities

| Source | Current SHA-256 | Held epoch-6 SHA-256 | Relation |
| --- | --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` | `f43c0d3a97047f7f64c37f3d93d4c1ffb18535df13dfc87673902f750e875c7d` | `2a58ad3c24802238ffde605670c09875e295718aa0e1a3ae60d39e74f6827731` | live = full proposal before; held = full proposal after |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` | `7431cf848bbfc04be2290d1543dfa4aa3886406d5d09d22177aa69d4e8bbe204` | `1081fa8b1d23fefd307769f4216ececdc8dd6f9434210401f800b36ed6a91de5` | live = full proposal before; held = full proposal after |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs` | `e70b593c61dd8f2264bb78bcd2d1c70df44561ae909b8060c0eb6b93ffe23ba7` | `e70b593c61dd8f2264bb78bcd2d1c70df44561ae909b8060c0eb6b93ffe23ba7` | same full bytes |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` | `84d72fe21b6b61ed6e49d7b4d4115bf920f67b3cddf8ba357315282f16ab1a01` | `84d72fe21b6b61ed6e49d7b4d4115bf920f67b3cddf8ba357315282f16ab1a01` | same full bytes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | `07d5d3c38fe40f59d74173d55fd776fdb2855c2a7ddd28a64a87feb52378660f` | `574e434cf2ec10db193e4e48c188c9cae9921fc5d787c750319152ef56370b0a` | live = full proposal before; held = full proposal after |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs` | `af3338f564831955c67e28116d507f246aa0717d1f892b60d62aa3e744cd702d` | `af3338f564831955c67e28116d507f246aa0717d1f892b60d62aa3e744cd702d` | same full bytes |

The full World/GPU proposal is `cargo-inputs/📥️current-native-world/🧩️world-full-pairs-5.json`; actual source admission is `🗑️generated/current-native-world/source-model-5/admission.json`. Native owns current whole/postguard outcomes. The relevant render functions are unchanged between World before/after; the GPU change is confined to zero-index upload rejection and minimum buffer capacity.

## The original primitives have a separate non-indexed color path

The source chain is complete:

| Boundary | Current source evidence | Consumption |
| --- | --- | --- |
| Original resident topology | World `append_component_overlays` 10509 | Reads the admitted Mesh3d lease schema and actual edge/vertex payload. |
| Primary wire admission | World 10519–10535 | `indices == 0 && edges > 0` makes primary_wire true. This bypasses the optional showEdges/component-mode gate. Each actual edge contributes its two transformed endpoints with original semantic style paint. |
| Primary point admission | World 10605–10625 | `indices == 0 && edges == 0` makes primary_point true after exact topological vertex count admission. This bypasses optional vertex-selection/component-mode gates. Each actual vertex contributes three visible marker line segments with camera/viewport-scaled extent. These markers represent original vertices; they do not add mesh triangles or authored index data. |
| Production scene owner | Public `render_world_3d`, World 13740 and 13878–13892 | Calls append_component_overlays and pushes its nonempty line_vertices into ScenePass3d.line_draws. It is production source, not cfg(test). |
| Prepared scalar authority | UI prepared 2754–2762 | PassLine then PassLineVertex measures each retained line vertex with explicit draw-item and physical LineVertex3d byte demand. The line roster is separate from indexed mesh-instance cursors. |
| GPU command consumer | UI gpu 998–1006 | Each odd line vertex forms an exact two-vertex segment, calls encode_prepared_world_line, and submits the resulting command encoder. It does not fetch or interpret the zero-index mesh's index buffer for this command. |
| Actual non-indexed draw | UI draw 4836–4877 | Requires exactly two vertices, uploads actual line vertices, selects world_line_pipeline and calls `pass.draw(0..2, 0..1)`. No set_index_buffer/draw_indexed occurs in this function. |
| Pipeline topology | UI draw 3340–3358 | world_line_pipeline declares PrimitiveTopology::LineList. |

Therefore the indexed mesh pass may correctly draw an empty logical index range while primary wire/point color geometry is consumed by explicit non-indexed line commands. The proposal's minimum4 physical index-buffer capacity neither fabricates an index nor changes the color path. Logical index_count remains0, the index upload cursor performs zero writes, and the primary geometry path does not depend on that index data.

The primary-wire condition covers both the original wire and the edge-only fixture. Edge-only publication expands only the actual edge endpoints into render vertices while retaining original edge topology and zero indices. Pure points require admitted topological vertex IDs and retain original vertex positions.

## Existing causal authorities retained

The unchanged native law `world_native_primary_geometry_paint_uses_original_style_tokens` at unit 6662 builds actual zero-index bridged wire/edge-only/point scenes in default object mode with showEdgesfalse and vertex/edge/face selection flagsfalse. It calls the same production append_component_overlays function and asserts nonempty geometry with original palette colors across object states and themes. This tests the primary-geometry gate, beyond accepting a resident buffer with index_count0.

The unchanged native GPU law at6698 verifies actual bridged leases with zero indices, actual device upload admission, logical resident index_count0, bounded upload close and resident retirement. Its success by itself would not prove the final color command was encoded; the source chain above supplies the exact consuming path.

The unchanged UI prepared law `a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry` at prepared-unit482 asserts the same PassLineVertex lane remains present. This audit read that law and did not execute it. Native's owning whole/postguards must retain the actual outcomes, and an offscreen pixel/render experiment would be required for stronger image proof. No rendering success is inferred merely from the minimum buffer or source inspection.
