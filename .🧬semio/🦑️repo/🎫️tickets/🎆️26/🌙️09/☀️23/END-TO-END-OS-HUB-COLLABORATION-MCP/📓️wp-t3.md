# WP-T3: Real Plugin Import/Export (Stub Serializers)

Slice: T3 (session 10). Captures: `.tmp-ticket/wp-t3/generated/`. Probe (kept input): `.tmp-ticket/wp-t3/stub-probe.ts`
(runs the test platform's own `stubSerializerBreaches`, prints one row per stub).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. baseline | 131 stub-serializer rows live (R5 `contract-3.txt` had 131 serializer rows) | `stub-0.txt` |
| 1. gate honesty | Gate stripped doc comments (4 real writers were flagged for prose that named the stub they replaced) and gained the import half `stub-deserializer`: **127 serializer + 122 deserializer = 249** live rows | `stub-1.txt`, `stub-2.txt` |
| 3. every owner | landed: **249 → 0** live stub rows (127 serializer + 122 deserializer at the start of the honest gate) | `stub-2.txt` → `stub-12.txt`, probe re-run at the end prints `0 stub rows` |
| 2. shared writers (stdio semio) | `drawing → png` (own AA rasterizer), `drawing → pdf` (real vector ops, was text-only), `mesh → png` (isometric view), plus `encode_drawing`/`decode_drawing`/`encode_mesh`/`decode_mesh` so every domain artifact has ONE writer per format. 370/370 drawing+mesh subset tests; oracles: resvg 0.45.1 renders the svg leaf, hayro 0.4.0 renders the pdf leaf, analytic hexagon for the cube view | `semio-drawing-1.txt` (13/13), `semio-drawing-mesh-2.txt` (370/370) |

## Shared writers (one per format, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`)

| Piece | Where | What it does | Oracle |
|---|---|---|---|
| drawing → png | `🖊️drawing/🚪️io/📤️export/…/📷️png/🔖️1.2` | own exact-area AA scanline rasterizer (non-zero fill, butt caps, miter joins limit 4, group transforms, png images) | resvg 0.45.1 renders the svg leaf's output of the same drawing: mean premultiplied diff < 0.006 |
| drawing → pdf | `…/📖️pdf/🔖️1.7` (rewritten) | real content-stream vector ops (paths, fills, strokes, ExtGState alpha, `cm` per group, Helvetica text) — was text-only | hayro 0.4.0 rasterizes the pdf; compared with our rasterizer |
| drawing → dxf / dwg | existing leaves, fixed | group transforms now applied (were dropped); circle normal form → real `CIRCLE`/`Circle` under a similarity | round trips in owner tests |
| drawing → svg | existing leaf, fixed | a style without fill now writes `fill="none"` (open polylines rendered filled black before) | resvg |
| mesh → png | `🔺️mesh/…/📷️png/🔖️1.2` | isometric z-up view, painter's order, two-sided Lambert, via the drawing rasterizer | analytic: unit cube silhouette = √3·s² within 1 % |
| mesh → gltf | existing leaf, fixed | POSITION `min`/`max` (spec-required) and a default scene with one node per mesh (viewers showed nothing) | the `gltf` 1.4.1 crate refused the file before the fix |
| `encode_drawing`/`decode_drawing`, `encode_mesh`/`decode_mesh` | the two `🚪️io/🦀️.rs` | one call per format for every owner | — |
| `diagram_drawing` (`SemioDiagram`) | `🖊️drawing/🚪️io/🦀️.rs` | node-link boards (circles/rectangles, boundary-to-boundary links, frames, labels) | unit tests |
| `encode_document_archive`/`decode_document_archive` | `🎒️zip/…/🧱️base/🚪️io/🦀️.rs` | zip carrier: `snapshot.<ext>.semio` + `snapshot.json`, Exact | the `zip` 6 crate reads the member back |

## Per-owner decisions

| Owner | Export (real now) | Import | Removed (no honest path) | Tests |
|---|---|---|---|---|
| gis/gismap | svg, pdf, png (page drawing, now north-up); dxf, dwg (world drawing: raw lon/lat, positions as circles) | dxf, dwg → features (circles → positions, closed → regions, open → routes); txt | import svg, pdf, png (page/raster coordinates are not map coordinates); dead `dwg_projection` | 155/155, `geo` area oracle on the dxf/dwg round trip (`gismap-2.txt`) |
| gis/gisterrain | stl, obj, ply, gltf, las (surface mesh); txt | json, txt | import stl, obj, ply, gltf, las, dwg, png (the surface is derived from the document; no mesh can be read into it); dead png/dwg export leaves | 55/55, `gltf` crate oracle (`gisterrain-4.txt`) |
| lowpoly | stl, obj, ply, gltf, las, dwg without the hex-DSL side channel (a re-import preferred the stale embedded DSL over edited geometry); png = isometric view | stl, obj, ply, gltf, dwg geometry | import las (fabricated a triangle fan over a point cloud), import png (only read the side channel) | 299/299, `gltf` crate oracle (`lowpoly-1.txt`) |
| puzzle/2d | svg, pdf, png, dxf, dwg (board diagram); txt | json, txt | import svg, dxf, pdf, png, dwg (a drawing is not a puzzle board) | 604+3 (`puzzle2d-2/3.txt`) |
| puzzle/3d | txt | json, txt | export+import stl, obj, ply, gltf, las, dwg, png: objects reference external `mesh_url` assets a serializer cannot resolve | 287/287 (`puzzle3d-1.txt`) |
| puzzle/5d | png (parts' 2D board), zip (document archive), txt | json, zip, txt | export+import stl, obj (external `mesh_url`), import png | 375+3, `zip` crate oracle (`puzzle5d-2/3.txt`) |
| procedural/generation2d | svg, pdf, png, dxf from the EVALUATED program (flow drawing kernel scenes → drawing, every node kind); txt | json, txt | import svg, dxf, pdf, png, dwg (a drawing is not a generative program); dead dwg export (generation3d owns the claim) | 176/176 (`generation2d-6.txt`). Honest gap: the scene converter is tested on committed scene JSON; the bundled example itself evaluates `math.add` as an unknown kind in a bare host, so no live end-to-end drawing was proven |
| procedural/generation3d | (already real) | stl, obj, ply, gltf, dwg (existing brep import neurons) | import las, png (were declared, always Err, and "withheld" from the picker — the withheld concept is gone with them); dead png export | lib 158 + io-round-trip 41 (`generation3d-3.txt`), incl. its parry3d oracle |
| shooting | — | json, txt | export+import png, jpg, gif, tiff, bmp, svg, pdf, dwg: they reinterpreted pack bytes; a scene of external asset URLs rendered by the host leaves through the `photos:out` port, not as an io format | 357/357 (`shooting-2.txt`) |
| cad | — (the editor's kernel-backed `export_solids_as` obj/stl/step stays the real path) | json | export+import stl, obj, gltf, png, dwg, step, ifc: they printed the DSL; `CadSnapshot` holds only model CHILD handles, unresolvable in a serializer | 433/433 (`semio-s-artifact-cad-cad-1.txt`) |
| process/process3d | txt | json, txt | export+import stl, obj, gltf, png, dwg, step, ifc: they reinterpreted pack bytes; the machined solid is a child brep the kernel host resolves | 360/360 (`process3d-3.txt`) |
| block/2d, 3d, 5d | (zip/json/txt were already real) | same | export+import stl, obj, png: registered hops that could only refuse (a node kind definition has no geometry or raster) | 2d 222/222, 3d 300/300 (`block2d-1.txt`, `block3d-1.txt`); 5d 328/328 |
| architect/program | csv = the register table via `export_registers_csv` (was one DSL cell); csv import via the editor's `import_registers_csv` (was `Default`) | csv, json, txt | import zip, xlsx (serde coercions into an empty program; the multi-table exports stay) | 2092/2092, `csv` crate oracle (`architect-3.txt`) |
| forms | csv/xlsx = question grid (the hex DSL row/cell side channel is gone), zip = document archive (was a pack mislabelled zip) | json, txt, zip | import csv, xlsx (only read the side channel) | 200/200, `csv` + `zip` crate oracles (`forms-1.txt`) |
| energy, space/home, demonstrator/playground | zip = document archive; txt carriers (were `Err("not yet implemented")`) | json, txt, zip | csv, xlsx both ways (DSL cell / serde coercion into an empty sheet) | energy 6292/6292, home 28/28, playground 41/41 |
| vcs | csv/xlsx = one named-column record (was serde coercion), zip = archive | csv, xlsx, json, txt, zip | — | see Crate test counts |
| fem/2d, 3d | csv = node coordinate table (was DSL envelope) | json, txt | md both ways, csv import, stl/obj imports (refusing hops); TS twin `FEM*_IO_ENTRIES` updated, `implemented` flag removed (always true now) | see Crate test counts |
| layout | svg, png, dxf, dwg of the spreads (`layout_snapshot_to_semio_drawing`; svg used to print the DSL and parse it as XML); png/dxf/dwg composers + capability rows registered | svg, dxf (trace → page frames + background via the generalised `layout_document_json_from_drawing`), dwg, json | pdf both ways (DSL text in a page; its contract test tested the stub), png import | see Crate test counts |
| draw | dxf, png, dwg via `drawing_document_to_semio_drawing` (were refusals) | json, txt | import svg, dxf, pdf, png, dwg (all returned an empty drawing) | see Crate test counts |
| dag, flow, trinity jack/rewriting, reasoning wires, imperative procedure, playbook, animate presentation | — | json, txt | md/docx/pdf/svg/png/csv/pptx both ways: the content lives in a composed child a serializer cannot resolve; each child's own subset (semio document/graph/flow/presentation) already has real exports | dag 211, jack 167, wires 193, procedure 149, playbook 158, rewriting 96, presentation 329; flow 255/256 — the 1 failure is `flow_render_fixture_projection_retires_populated_and_rejected_pages` (editor render, not io) |
| raster | (png etc. already real) | dwg kept (real) | pdf both ways, dwg export (registered refusals) | see Crate test counts |
| sourcing/curation | zip = document archive | zip, json, txt | png, stl, obj both ways (pack reinterpretation) | 153/153 |
| stdio docx, xlsx, pptx, bcf | — | — | xml hops (returned `XmlDocument::default()`; an OPC package is not one XML document) | see Crate test counts |
| stdio semio/cad | dwg = real entity mapping (was a refusal) | dwg = real entity mapping from the logical drawing (was always empty — its doc said the codec never decodes entities, stale) | — | 414/414 incl. `a_plan_survives_real_dwg_bytes` |
| txt carriers | flow, architect, procedure, energy, jack, home, process3d, puzzle 2d/3d/5d, gis, generation2d, shooting: exact DSL carrier | | | |

## Gate changes (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`)

- `withoutLineComments`: prose that names the stub a leaf replaced no longer flags a real writer.
- `stubDeserializerBreaches` (new, wired next to `stubSerializerBreaches`): an import leaf that never
  reads its input, parses the artifact's DSL out of the file, or serde-coerces the format snapshot.
  Exempt: txt/json/binary@raw carriers, the shared zip document archive, and a format's own snapshot
  wire (`<MdSnapshot as ArtifactDsl>::parse_dsl`).
- `printsItsInputAsDsl`: only printing the INPUT as DSL is a stub; printing the target format's own
  snapshot (the io-mechanism hop shape) is not.
- Coercion detection now also sees the method form `X::from_value(from.to_value())` (dag, vcs, curation hid behind it).

## Crate test counts (native, `cargo test -p <crate> --lib`, private target)

| Crate | Result | Capture |
|---|---|---|
| stdio-semio (drawing+mesh+cad subsets, conversion-cad,mesh) | 414/414 | `semio-cad-drawing-mesh-4.txt` |
| gis-gismap / gis-gisterrain | 155/155 / 55/55 | `gismap-2.txt`, `gisterrain-4.txt` |
| lowpoly | 299/299 | `lowpoly-1.txt` |
| puzzle 2d / 3d / 5d | 604+3 / 287 / 375 (+3 io) | `puzzle2d-2/3`, `puzzle3d-1`, `puzzle5d-2/3` |
| procedural generation2d / generation3d | 176/176 / 158 + io-round-trip 41 | `generation2d-6`, `generation3d-3` |
| shooting / cad / process3d | 357 / 433 / 360 | `shooting-2`, `semio-s-artifact-cad-cad-1`, `process3d-3` |
| block 2d / 3d / 5d | 222 / 300 / 328 | `block2d-1`, `block3d-1`, `batch-semio-s-artifact-block-5d` |
| architect / forms / energy / home / playground / vcs | 2092 / 200 / 6292 / 28 / 41 / 125 | `architect-3`, `forms-1`, batch-1/2 captures |
| dag / jack / wires / procedure / playbook / rewriting / presentation / curation / layout / fem-3d | 211 / 167 / 193 / 149 / 158 / 96 / 329 / 153 / 395 / 956 | `batch-*` |
| stdio docx / xlsx / bcf | 72 / 66 / 37 | `batch-*` |
| **not green, not io** | flow 255/256 (editor render projection), fem-2d 1036/1038 (two 8 ms timing laws under peer load: 39 ms), draw 279/282 (store arena/retained-envelope faults), raster 220/227 (arena/poisoned test lock), pptx 75/76 (missing `temp/…pptx` fixture) | `batch-*` |
| plugin crates (26) | assembly green everywhere; only `descriptor_is_fresh` fails in 21 of them — expected until W1's `describe` | `batch-3-summary.txt` |

## Guests needing rebuild (for W1)

Filed in `wp-w1/requests/t3.txt` (requests 1 and 2): gis, lowpoly, puzzle, procedural, shooting, cad,
process, block, architect, forms, energy, space, vcs, fem, layout, draw, dag, flow, trinity, reasoning,
imperative, playbook, animate, raster, sourcing, demonstrator, and stdio. W1 acknowledged request 1.

## Removed declarations (no honest implementation path)

Listed per owner in the tables above. The recurring reasons: the content is a composed child a
serializer cannot resolve (cad, process3d, dag, flow, jack, wires, procedure, playbook, presentation);
the format carries no model of the artifact (shooting images, gisterrain mesh imports, puzzle3d/5d
meshes with external `mesh_url`s, block geometry/raster, fem md, draw/layout/puzzle2d/gismap raster or
page imports, las→lowpoly); or it only ever refused or returned `Default` (block, fem stl/obj imports,
generation3d las/png, raster pdf/dwg export, stdio OPC→xml hops).

## Honest gaps

- gis declares no GeoJSON; nothing was added (not requested by a declaration).
- generation2d: the scene→drawing converter is tested on committed scene JSON; the bundled example
  evaluates `math.add` as an unknown kind in a bare host, so no live evaluated drawing was proven.
- No third-party DWG reader exists as a test dependency; DWG round trips go through the codec's own
  writer and its independent decoder only.
- Raster's dwg import was removed by my bulk drop and restored from `HEAD` (leaf + wiring); any peer
  edit to that leaf after `HEAD` would be lost — none showed in the diff before deletion.
- Descriptors (`🛂️.descriptor.semio`/`🔣️.json`) are stale in 21 plugins until W1 runs `describe`.

## Files changed

- Gate: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`.
- stdio semio: `🖊️drawing/🚪️io/{🦀️.rs, 📤️export/…/{📷️png,📖️pdf,🔄️dxf,🖊️dwg,🎨️svg}}` + tests, `🔺️mesh/🚪️io/{🦀️.rs, 📤️export/…/{📷️png,🎬️gltf}}` + tests, `📐️cad/🚪️io/{📤️export,📥️import}/…/🖊️dwg` + tests, crate root module tree, `📦️packages/🦀️rust/Cargo.toml` (png in conversion-drawing, mesh⇒drawing, resvg/hayro dev-deps).
- stdio zip `🧱️base/🚪️io/🦀️.rs` (document archive); stdio docx/xlsx/pptx/bcf io (xml hops removed).
- Owners' `🚪️io/**`, artifact roots (module trees, kind lists, composer rows), `📦️packages/🦀️rust/Cargo.toml` (stdio-semio features, test-only oracles `gltf`, `csv`, `zip`), TS twins (fem), fixture `generation3d/🧫️fixtures/🚪️io/🗿️artifact-surface.json` and its `🟦️.ts` test, removed `layout …/🔬️pdf-contract-vectors`, `gismap …/🔬️dwg-projection-unit`.
- Ticket inputs (kept): `wp-t3/{stub-probe.ts, leaf-tool.py, io-edit.py, drop-formats.py, leaves.py, run-batch.sh, batch-*.txt}`.
