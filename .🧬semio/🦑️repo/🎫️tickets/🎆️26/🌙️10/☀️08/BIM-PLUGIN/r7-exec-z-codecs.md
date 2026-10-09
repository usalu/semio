# r7 execution report: z-codecs (BIM glTF/SVG onto the stdio codecs, io() declared, IFC reads the inference)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `A` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`. `S` = `A/🏅️standards/🔖️1/🪆️subsets/✳️any`, `X` = `S/🚪️io/📤️export`. Stdio = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>`.

## 1. Stdio crates and codecs

Findings when I started: json and xml failed with 9 `E0061` (`MutationDiff::apply` without the capability), svg with 6 more, gltf with a different, larger break. All fixed in the minimal way, none mints a capability.

| Crate | Files (all leaf-external central-apply call sites) | Change |
|---|---|---|
| json | `🧱️base/🚪️io/🦀️.rs`, `🛜️i-json/🚪️io/🦀️.rs`, `🌍️geojson/🚪️io/🦀️.rs` (`absorb`), `🧱️base/🧬️schema/🧬️mutations/🦀️.rs`, `🛜️i-json/🧬️schema/🧬️mutations/🦀️.rs` (`apply_*_mutation`) | `protocol::apply_diff(&diff, &base)` |
| xml | `🧱️base/🚪️io/🦀️.rs`, `✅️valid/🚪️io/🦀️.rs`, `🧱️base/🧬️schema/🧬️mutations/🦀️.rs`, `✅️valid/🧬️schema/🧬️mutations/🦀️.rs` | same |
| svg | `🧱️base`, `🔰️basic`, `🔬️tiny` each `🚪️io/🦀️.rs` and `🧬️schema/🧬️mutations/🦀️.rs` | same |
| gltf | `🏅️standards/🔖️2.0/…/🧬️schema/🔺️diff/🦀️.rs`: the block `PrimitiveDiff … AnimationDiff` (513 lines) was present three times (E0428/E0119, 108 errors), the two copies removed; `🧬️mutations/🎬️scene/{🌱️create,🗑️delete}/🦀️.rs`: stale imports `insert_empty_scene`, `scenes_op` dropped (functions no longer exist, the leaves already use `plan`/`rewire`) | |
| framework authority | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json`: the gltf block still registered `📸️snapshot/{📸️set,🩹️patch}` whose leaves were deleted by the diff-only wave, which tripped the const check "Mutations requires every explicitly registered component domain operation"; the two rows removed | |

For the DIFF-ONLY owner: these are the files above; `cargo check --manifest-path <artifact>/Cargo.toml` is green for json, xml, svg and gltf.

## 2. BIM exports onto the stdio codecs (local writers deleted)

- **SVG**: `X/🎨️svg/🧱️codec/🦀️.rs` is the only place naming `SvgElement`, `typed_to_svg_document` and `xml_document_to_text_checked`; `🖍️drawing` builds `SvgElement::{Group,Path,Text,Tspan}` with `CommonAttrs`, `✒️path::commands` returns `PathCommand`s (the arc flags and radii rule is unchanged), `🦀️.rs::plans_to_svg` builds the `Svg` root with `ViewBox` and returns `Result`. `📝️markup` (local XML writer) deleted. Deps `semio-s-artifact-stdio-svg`, `-xml` added to the package manifest.
- **glTF**: `X/🧊️gltf/🧱️codec/🦀️.rs` is the only place naming `GltfSnapshot` and `encode_glb`/`decode_glb`; `🧬️document::GltfModel::to_snapshot()` lowers the BIM scene (nodes, meshes, materials, one buffer) into a typed snapshot, `to_glb()` calls `encode_glb`. `🧱️container` (local GLB framing) and the local JSON writer deleted. Dep `semio-s-artifact-stdio-gltf` added.
- Ancillary: the owner's commit 675 moved `io_mechanism` from `semio_framework::io` to `semio_framework_os_kernel::io`; all BIM io files updated (9 files).

Outputs and oracles: the SVG bytes are not changed by the stdio writer on the committed expectations I blessed before the commit and `lxml + shapely` agree (`python 🐍️.py check`). The GLB bytes changed legitimately (key order and number formatting of the stdio JSON writer, `scene` index, no `buffers` for an empty scene): `🧫️fixtures/🧊️gltf/🏠️house/🏠️house.glb` re-blessed (1 007 576 bytes), the three.js measure table `🔬️measure/🔣️.json` is unchanged and the oracle still agrees. One degenerate behaviour change: a model with no element writes a scene without `nodes`, which the stdio writer omits (valid per spec) and the `gltf` crate refuses; the unit test now reads that case with the stdio decoder.

## 3. `io()` reachable from the host

The artifact now declares `ArtifactDeclaration<A>` the way drawing does: `S/🦀️.rs` (`subset::<A>()` with `io: io::io()`, viewer, editor, examples), `A/🏅️standards/🔖️1/🦀️.rs` (`standard()`), `A/🦀️.rs` (`artifact::<A: BimApplication>()`, trait `BimApplication`, `standards::v1` mounts). The old `declaration()` builder is deleted, the editor's `examples()` override moved to the subset, the hub root `🌎️hub/🧩️compositions/🏙️bim/🦀️.rs` uses `.declare_artifact(crate::artifacts::model::artifact())` (editor/viewer registration now comes from the tree). Tests: `S/🚪️io/🧪️tests/🔬️unit` `the_io_declaration_lists_the_native_pack_and_text_codec_and_every_foreign_hop` (pack + text round trip of the house, native codec schema, and the four hops IFC export, glTF export, SVG export, IFC import in that order); hub `🧪️tests/🔬️surface` `bim_declares_its_native_codec_and_the_ifc_gltf_and_svg_hops_through_the_declaration_tree`.

## 4. IFC export reads the inference (F08, F09)

`Export::new(model, &ModelInference)`; `model_to_part21` runs `ModelInference::infer(model)` once and `inferred_to_part21(model, &inferred)` writes. Levels, wall layouts, solids, rooms, stair runs come from the fields (no `compute_*` left in the exporter). All `Qto_*BaseQuantities` (wall, slab, roof, column, beam, window, door, curtain wall, railing, space) come from `ModelInference.quantities.elements[id]` through `Export::quantify`; the local formulas (opening areas, `NetVolume`, slab areas, profile area, `ProjectedArea`, railing length) are deleted, roof layer volumes map the take-off rows to the solid groups (`horizontal::group_volumes`). Storey `GrossHeight` stays authored (no take-off row). New tests: document equals the one built from a given inference; every written base quantity equals the take-off value (10 families); a wall's `NetSideArea` follows the take-off (an opening moved out of the wall stops discounting); roof layer volumes equal the take-off and the written brep; `group_volumes` mapping.

## 5. Commands and results

| Command (all through the gate, manifests of the per-artifact workspaces) | Result |
|---|---|
| `cargo check` json, xml, svg, gltf (`<artifact>/Cargo.toml`) | exit 0 each |
| `cargo check -p semio-s-artifact-bim-model` | exit 0 |
| same, `--target wasm32-wasip2` | exit 0 |
| `cargo test -p semio-s-artifact-bim-model --lib -- io::` | **180 passed, 0 failed** |
| `cargo test … --lib` (whole crate) | 3678 passed, 9 failed, 1 ignored; all 9 are editor tests (`editor::bim::component::unit_tests` x3, `editor::bim::gestures::component::app_tests` x6), the known close-stall family, none in io |
| `BIM_BLESS=1 … the_committed_house_file_is_the_current_export` | gltf blessed; ifc and svg unchanged |
| `bun 📜️script.ts oracle quick --case` `🧊️export-bim-1-gltf`, `🎨️export-bim-1-svg`, `🏗️export-bim-1-ifc` | each `executed=1 passed=1` |
| `python 🐍️.py check` svg (lxml, shapely) and ifc (ifcopenshell) | "oracle agrees" |
| `cargo test --manifest-path 🌎️hub/Cargo.toml -p semio-hub-bim --lib -- surface` | NOT RUN to a verdict: `semio-framework-artifact-workflow-run` does not compile at HEAD (`RunStatus: BorrowedDslField`, "approved semantic verb"), baseline work of `z-baseline` |

## 6. Open issues

- The hub composition (`.declare_artifact`) and its new surface test are written but unverified until the workflow-run crate compiles; the trait bounds mirror drawing/writer (`NoMembers` default).
- Platform `subject`/`parity` roles of the export cases were not run (they build the generated host outside the gate).
- Quantities that have no take-off row (storey `GrossHeight`/`NetHeight`, IFC stair-flight `Length`/`Width`) are still authored/run values.
- Generated scratch under `T/🗑️generated/z-codecs/` removed.
