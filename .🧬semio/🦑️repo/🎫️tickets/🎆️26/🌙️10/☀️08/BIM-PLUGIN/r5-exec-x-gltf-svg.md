# r5 execution report: x-gltf-svg (Wave X, glTF 2.0 and SVG 1.1 export of the BIM model)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `IO` = `S/🚪️io/📤️export`.

## 1. Result

Two export leaves are registered in the subset `io()` next to the IFC entry: `s.stdio.gltf@2.0/*` (binary glTF, `ModelIntoGlb`) and `s.stdio.svg@1.1/*` (floor plans at 1:100, `ModelIntoSvg`). Both are `Serializer<ModelSnapshot>` leaves that receive only the snapshot, so they call the same pure inference functions the inference fields run (`compute_element_solids`, `compute_storey_levels`, `compute_plan_linework`). Both are `IoFidelity::Lossy`.

- **glTF**: site, building, storey and element nodes. One node per element that has geometry (name = authored name, else id; `extras` = `{kind, id, name, storey}`), site node lifted by the site elevation, building node at its origin/elevation with the quaternion about the vertical for its rotation, storey node at its inferred elevation, element vertices relative to the storey, so the chain of node transforms reproduces `ElementSolid.placement`. One mesh per element, one primitive per glTF material (indexed triangles, `POSITION` with min/max, `NORMAL`, `u32` indices, welded per primitive). PBR materials: the model material colour (the one the 3D viewer paints, `render::world::group_color`), `Glass` category and unnamed glazing are `alphaMode BLEND` with alpha 0.35 and roughness 0.05, metal is `metallic 1`, faces without a material take the viewer's family colour. Z-up to Y-up is the proper rotation `(x, y, z) -> (x, z, -y)` (handedness and winding survive). GLB container with 4-byte aligned JSON and BIN chunks; no BIN chunk for an empty scene.
- **SVG**: one sheet, `<g class="storey" id="storey-<id>" data-storey data-name data-level aria-label transform>` per storey, stacked top-down per building (highest level first), each with a `<title>`, a title text and three layers (`layer regions`, `layer lines`, `layer texts`). Paper millimetres at 1:100 (`1 m = 10 mm`, y flipped so north is up, coordinates snapped to 0.001 mm), `width`/`height` in `mm` and `viewBox` fitted to the stacked slots. One `<path>` per region (poche, one subpath per ring, `fill-rule: evenodd`) and per polyline, one `<text>` per text anchor (space tags stack number, name and area in `<tspan>`s; grid labels). Stroke classes per line style in an embedded style sheet: `cut` 0.5, `projection` 0.25, `hidden` 0.18 dashed, `annotation` 0.13 (ISO 128 groups); every path also carries its kind class (`wall-cut`, `door-swing`, ...), `data-element` and `data-id` (picking). Bulge arcs become `A` commands: radius `c(1+b^2)/(4|b|)`, large-arc flag `|b| > 1`, sweep flag `b < 0` (the picture is the plan as drawn y-up, so a counter-clockwise positive bulge is SVG sweep 0). A browser render of the committed house shows poche walls, the curved bay wall, window symbols, stacked space tags and grid bubbles.

## 2. Deviation from the recipe, and why

`r3-recipe-io.md` proposes `GltfSnapshot` + `encode_glb` and the `SvgElement` tree + `xml_document_to_text_checked`. That path needs the crates `semio-s-artifact-stdio-gltf`, `-svg`, `-xml` (and `-json`), and none of them compiles today: the repository-wide `ApplyCapability` migration of the stdio crates stopped before them (`cargo check -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-svg`: 9 `E0061` errors in `json` and `xml`, e.g. `🧾️json/…/🧱️base/🚪️io/🦀️.rs:113`, `📰️xml/…/🧱️base/🚪️io/🦀️.rs:114`; the other 49 `MutationDiff::apply` call sites in 30 stdio artifacts are the same migration). x-ifc already had to depend on the contract crate under the ifc name for the same reason. A BIM crate that depends on the broken crates would not compile for anyone, so the writers are local, small and pure:

- `🧊️gltf/🧱️container` (GLB framing, 60 lines), `🧬️document` (typed document to JSON + buffer, `DslValue` printed by `semio_framework_pack_json`), no runtime library is added.
- `🎨️svg/📝️markup` (escaped XML element tree, one element per line).

Follow-up once the stdio crates compile: add the two path dependencies, cross-check `container::split_glb` against `decode_glb` and the SVG against `parse_svg_xml`, and decide whether to replace the local container and markup by `encode_glb` and the typed tree (the typed `GltfModel` and the `Element` tree are the only seams).

## 3. Files

Created under `IO`:
- `🧊️gltf/🦀️.rs` (dialect, `ModelIntoGlb`, `model_to_gltf`, `export_glb`), `🧱️container`, `🧬️document`, `🎨️materials`, `🌳️scene`, `📏️projection` (each with `🧪️tests/🔬️unit`), `🧪️tests/🧰️testkit`, `🧪️tests/🔬️unit`.
- `🎨️svg/🦀️.rs` (dialect, `ModelIntoSvg`, `plans_to_svg`, `export_svg`), `📝️markup`, `✒️path`, `🎚️style`, `📐️sheet`, `🖍️drawing`, `📏️projection` (each with `🧪️tests/🔬️unit`), `🧪️tests/🧰️testkit`, `🧪️tests/🔬️unit`.

Created under `S`: fixtures `🧫️fixtures/🚪️gltf/🏠️house/{🏠️house.glb, 🔬️measure/🔣️.json}`, `🧫️fixtures/🚪️svg/🏠️house/{🏠️house.svg, 🔬️measure/🔣️.json}` (both reuse the committed house snapshot `🧫️fixtures/🚪️ifc/🏠️house`); cases `🧪️tests/🧊️export-bim-1-gltf/{🥒️.feature, 🟦️.ts, 🦀️.rs}` and `🧪️tests/🎨️export-bim-1-svg/{🥒️.feature, 🐍️.py, 🦀️.rs}`.

Updated (surgical `Edit`s): `IO/🦀️.rs` (two `pub mod`), `S/🚪️io/🦀️.rs` (two `serializer_entry` rows), `S/🔮️oracles/🔣️.json` (oracles `bim-1-three-gltf`, `bim-1-lxml-shapely-svg`), `✏️s/🔌️plugins/🏙️bim/🔮️oracles/🔣️.json` (host package `lxml 6.1.3`), `📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `gltf 1.4.1` with `utils, names, extras` and `quick-xml 0.39.4`, both already in `Cargo.lock`).

## 4. Oracles (third party, run and green through the platform)

| Case | Oracle | What it reproduces from the committed file |
|---|---|---|
| `export-bim-1-gltf` | three.js 0.182.0 `GLTFLoader` (`bun 🟦️.ts check|write <S/🧫️fixtures/🚪️gltf>`) | 30 nodes, 24 meshes, 37 primitives (one `Mesh` each), 11 676 triangles from the index buffers, 7 materials, element nodes per kind and per storey, world bounds of every vertex through the site/building/storey chain (`Box3.setFromObject(scene, true)`); self-checks: scene-graph node count equals `parser.json.nodes`, used materials equal defined materials |
| `export-bim-1-svg` | lxml 6.1.3 (libxml2) + shapely 2.1.2 (GEOS) (`python 🐍️.py check|write <S/🧫️fixtures/🚪️svg>`) | root SVG 1.1 and mm size equal to the viewBox, one `g.storey` per model storey (unique id, name, level, title, three layers), regions/lines/texts per storey, paths per style class, arc commands, even-odd area of the straight cut poche and length of the straight lines per style converted to metres; arcs are sampled (SVG endpoint-to-centre conversion) for an audit value committed beside the table |

The subject side of both cases reports the same table from its typed document (`export::gltf::projection`, `export::svg::projection`); the Rust unit tests compare it with the committed oracle tables at 1e-9. The three bounds agree to 1e-9 because the subject works from the `f32` vertices it wrote, which is what the loader reads.

## 5. Commands and results

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-svg` (gate) | FAILS in `json`/`xml` (peers, section 2) |
| `cargo check -p semio-s-artifact-bim-model` (gate, native) | green, no warning from the new files |
| `cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2` (gate) | green |
| `cargo test -p semio-s-artifact-bim-model --lib -- export::gltf export::svg` (gate, first run, `BIM_BLESS=1`) | 70 passed, 5 failed (4 tests that need the oracle tables, 1 wrong expectation of mine) |
| same filter on the private mirror (`r5-x-gltf-svg-mirror.ts`, gate slot), final run | **76 passed, 0 failed** (gltf: container 4, document 5, materials 4, scene 12, projection 6, root 6; svg: markup 3, path 8, style 3, sheet 4, drawing 6, projection 7, root 8; third-party readers inside: the `gltf` crate, quick-xml) |
| `bun 🧊️export-bim-1-gltf/🟦️.ts write` then `check` | wrote 30 nodes / 11 676 triangles; `oracle agrees (three 0.182.0)` |
| `python -I -X utf8 🎨️export-bim-1-svg/🐍️.py write` then `check` | wrote 4 storeys; `oracle agrees (lxml 6.1.3, shapely 2.1.2)` |
| `bun 📜️script.ts oracle quick --case 🧊️export-bim-1-gltf` (platform) | `executed=1 passed=1 failed=0` |
| `bun 📜️script.ts oracle quick --case 🎨️export-bim-1-svg` (platform, after the final re-bless) | `executed=1 passed=1 failed=0` |

Why a mirror: for the whole time after the first run the shared crate did not compile its test target because of peers' in-flight work (editor gesture tests, `PropertyValue` enum conversion, a missing mutation fixture; last seen: `✏️editor/🧪️tests/🔬️unit/🦀️.rs:495 BimApp: PluginApp…` and `🧬️inferences/🧪️tests/🔬️unit/🦀️.rs:47` unreadable file). `r5-x-gltf-svg-mirror.ts` (adapted from `r4-x-examples-mirror.ts`) copies the artifact into `T/🗑️generated/x-gltf-svg-mirror`, strips every `cfg(test)` module mount except those under `🧊️gltf` and `🎨️svg`, and runs them in a one-member private workspace that shares the gate slot's build cache. It is a verification harness only; the committed code is the real tree. The real gate command (`cargo test … -p semio-s-artifact-bim-model --lib`, whole crate) was re-run last at 12:12 and still stopped on the two peer errors above, so it is not claimed green here.

Bugs the tests caught on the way: SVG arc sweep flag (the first version used `b > 0`; the independent endpoint-to-centre test showed counter-clockwise bulges need sweep 0), arc radius precision near half circles (3 decimals moved the centre by 0.05 mm, now 6 decimals), CSS newlines flattened by attribute-style escaping (text nodes now keep line feeds, attributes use character references).

## 6. Open issues

- Run `cargo test … -p semio-s-artifact-bim-model --lib` (whole crate) again once the editor, inference and snapshot work of peers compiles; the 76 export tests are expected to pass unchanged.
- The platform `subject` and `parity` roles of the two cases were not run: the generated host builds with its own cargo invocation outside the gate. The `oracle` role is green and the unit tests compare the subject report with both committed oracle tables at 1e-9.
- Section 2: local container/markup instead of the stdio crates until they compile.
- `serializer_entry` rows are in `io()`, but x-ifc notes that the hub `declaration()` does not read `io()` yet; the exports are reachable through `io()` only until that is wired.
- The GLB fixture is 1.0 MB (11 676 flat-shaded triangles as the inference stores them).
- A disk-full incident (C: at 0 bytes, ENOSPC while writing) hit this task once; every file I had written was verified intact afterwards.
