# Geometry Whole-Trial and Repository Schema Census

Current read-only `rg --files` JSON census screened 5,214 schema-shaped documents whose paths contain schema or the schema facet marker, excluding ticket/generated/third-party/build/cache and ordinary fixture directories. The structural nested input-plus-expected/copy-grant filter returned six documents: placement, loops, mesh, bulge, section and regions below. This is a bounded structural screen, not proof that every possible corpus grammar is absent. Separate direct review additionally confirms skeleton and triangulation, whose expected values use domain-specific names.

Confirmed complete trial authorities:

- `🧰️framework/🔨️modules/📐️geometry/🧭️placement/🧬️schema/🔣️.json`: grouped placements/z_planes; named operations and input point plus expected result.
- `🧰️framework/🔨️modules/📐️geometry/➰️loops/🧬️schema/🔣️.json`: loops/offsets families; named vertices plus expected area/perimeter/centroid/bounds/containment.
- `🧰️framework/🔨️modules/📐️geometry/🕸️mesh/🧬️schema/🔣️.json`: extrusions/loop_extrusions/prisms/sweeps/walls families; named input geometry plus expected computed output.
- `🧰️framework/🔨️modules/📐️geometry/🌙️bulge/🧬️schema/🔣️.json`: arcs/offsets/intersections/closest/three_points/bands/corners families; named source geometry plus expected results.
- `🧰️framework/🔨️modules/📐️geometry/🔪️section/🧬️schema/🔣️.json`: array requiring name/outer/holes/bottom/top/z/expected.
- `🧰️framework/🔨️modules/📐️geometry/🦴️skeleton/🧬️schema/🔣️.json`: named input outer/holes/speeds paired with peak/faces/levels/oracles.
- `🧰️framework/🔨️modules/📐️geometry/🔺️triangulation/🧬️schema/🔣️.json`: named input outer/holes paired with expected area and triangle count.
- `🧰️framework/🔨️modules/◻️2d/🧱️regions/🧬️schema/🔣️.json`: offsets/booleans family envelope; name/input/distance/join plus expected outcomes.

These are complete examples even though the IDs use geometry domain names. They do not define an independently produced individual geometry value. No schema document reader was found in the bounded geometry TS/Rust search. Existing native unit tests instead parse plain examples and execute real algorithms per row: section16–17, loops unit fixture reader4–5, mesh38/70/88/101/127, bulge26/71/103/140/152/222/240. AEC independent Rust tests read bulge84, loops174, mesh251, section298 and triangulation321; AEC TypeScript imports plain bulge/loops/triangulation/mesh at4–7 and uses third-party Three. Preserve every plain input, actual geometry computation, native assertions and independent oracle.

The earlier five OS whole-law authorities are documented separately in `📓️oct8-post-ui-current-whole-law-census.md`; Root is retiring that batch concurrently, so absence from this later screen is not authored credit. Genuine controlled diagnostics, official compiler/test grammars and runtime individual-value DTOs remain outside these retirement recommendations. No builds/tests/generation or source edits were performed.
