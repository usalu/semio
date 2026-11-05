# Current Typed Semio Asset Inventory

Read-only source/header/embedded-schema audit; no actual typed codec execution or asset edits. Paths below are relative to Semio v1 subsets. E schema is stdio.semio for base, otherwise stdio.semio.SUBSET.

| Subset | Source module | Actual included asset | Header | Decoded schema | Duplicate asset copies |
| --- | --- | --- | --- | --- | --- |
| audio | ✉️base/📚️examples/🎵️tone/🦀️.rs | ✉️base/📚️examples/🎵️tone/🖼️assets/🗣️.dsl.semio | semio stdio.semio.audio.dsl v1 | stdio.semio.audio | 🔊️audio/🖼️assets/🎵️tone/🗣️.dsl.semio |
| animation | ✉️base/📚️examples/🚶️walk/🦀️.rs | ✉️base/📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio | semio s.stdio.semio.animation.dsl v1 | s.stdio.semio.animation | 🎞️animation/🖼️assets/🚶️walk/🗣️.dsl.semio |
| brep | ✉️base/📚️examples/🧊️solid/🦀️.rs | ✉️base/📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio | semio stdio.semio.brep.dsl v1 | stdio.semio.brep | 🧊️brep/🖼️assets/🧊️solid/🗣️.dsl.semio |
| cad | ✉️base/📚️examples/📐️drawing/🦀️.rs | ✉️base/📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio | semio stdio.semio.cad.dsl v1 | stdio.semio.cad | 📐️cad/🖼️assets/📐️drawing/🗣️.dsl.semio |
| document | ✉️base/📚️examples/🗒️memo/🦀️.rs | ✉️base/📚️examples/🗒️memo/🖼️assets/🗣️.dsl.semio | semio s.stdio.semio.document.dsl v1 | s.stdio.semio.document | 📑️document/🖼️assets/🗒️memo/🗣️.dsl.semio |
| drawing | ✉️base/📚️examples/🖍️sketch/🦀️.rs | ✉️base/📚️examples/🖍️sketch/🖼️assets/🗣️.dsl.semio | semio stdio.semio.drawing.dsl v1 | stdio.semio.drawing |  |
| image | ✉️base/📚️examples/🖼️swatch/🦀️.rs | ✉️base/📚️examples/🖼️swatch/🖼️assets/🗣️.dsl.semio | semio s.stdio.semio.image.dsl v1 | s.stdio.semio.image |  |
| text | ✉️base/📚️examples/📃️note/🦀️.rs | ✉️base/📚️examples/📃️note/🖼️assets/🗣️.dsl.semio | semio s.stdio.semio.text.dsl v1 | s.stdio.semio.text | 🔤️text/🖼️assets/📃️note/🗣️.dsl.semio |
| value | ✉️base/📚️examples/🕸️graph/🦀️.rs | ✉️base/📚️examples/🕸️graph/🖼️assets/🗣️.dsl.semio | semio stdio.semio.value.dsl v1 | stdio.semio.value | 🔢️value/🖼️assets/🕸️graph/🗣️.dsl.semio |
| mesh | 🔺️mesh/📚️examples/🧊️cube/🦀️.rs | 🔺️mesh/📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio | semio stdio.semio.mesh.dsl v1 | stdio.semio.mesh |  |
| video | ✉️base/📚️examples/🎥️clip/🦀️.rs | ✉️base/📚️examples/🎥️clip/🖼️assets/🗣️.dsl.semio | semio stdio.semio.video.dsl v1 | stdio.semio.video | 🎬️video/🖼️assets/🎥️clip/🗣️.dsl.semio |
| * | ✉️base/📚️examples/🌐️envelope/🦀️.rs | ✉️base/🖼️assets/🌐️envelope/🗣️.dsl.semio | semio stdio.semio.dsl v1 | stdio.semio |  |
| presentation | ✉️base/📚️examples/📽️deck/🦀️.rs | ✉️base/📚️examples/📽️deck/🖼️assets/🗣️.dsl.semio | semio s.stdio.semio.presentation.dsl v1 | s.stdio.semio.presentation | 📽️presentation/🖼️assets/📽️deck/🗣️.dsl.semio |
| flow | ✉️base/📚️examples/🌊️pipeline/🦀️.rs | ✉️base/📚️examples/🌊️pipeline/🖼️assets/🗣️.dsl.semio | semio stdio.semio.flow.dsl v1 | stdio.semio.flow | 🌊️flow/🖼️assets/🌊️pipeline/🗣️.dsl.semio |
| model | ✉️base/📚️examples/🏢️building/🦀️.rs | ✉️base/📚️examples/🏢️building/🖼️assets/🗣️.dsl.semio | semio stdio.semio.model.dsl v1 | stdio.semio.model | 🏛️model/🖼️assets/🏢️building/🗣️.dsl.semio |
| graph | 🕸️graph/📚️examples/🕸️wires/🦀️.rs | 🕸️graph/🖼️assets/🕸️wires/🗣️.dsl.semio | semio s.stdio.semio.graph.dsl v1 | s.stdio.semio.graph |  |
| object | 📦️object/📚️examples/📦️crate/🦀️.rs | 📦️object/🖼️assets/📦️crate/🗣️.dsl.semio | semio stdio.semio.object.dsl v1 | stdio.semio.object |  |
| kit | 🧰️kit/📚️examples/🪑️furniture/🦀️.rs | 🧰️kit/🖼️assets/🪑️furniture/🗣️.dsl.semio | semio stdio.semio.kit.dsl v1 | stdio.semio.kit |  |
| table | 📊️table/📚️examples/📃️sheet/🦀️.rs | 📊️table/🖼️assets/📃️sheet/🗣️.dsl.semio | semio s.stdio.semio.table.dsl v1 | s.stdio.semio.table |  |

Seven typed example schema/header families remain stale: animation, document, image, text, presentation, graph and table. Identical copies listed above must be handcrafted together with exact current typed print/encode evidence; changing the header alone leaves old embedded hex schema. Mutation fixtures may share those exact bytes and are included in duplicate matches. Pack counterparts require independent actual ArtifactPack regeneration/verification, not blind byte substitutions.

Current text/value/animation parse_dsl strips any recognized preamble using split_text_preamble and ignores the returned envelope before parsing body. Value body carries hex(schema) as the first bracketed token; text/animation carry schema=hex. Parsing success therefore does not prove E schema/header identity. Assert both snapshot.schema==E::DOCUMENT_SCHEMA and exact current print/encode content. No current-codec validity claim follows from this inventory.

Exact local example exports: semio_s_artifact_stdio_semio_mesh_examples::examples::cube; graph_examples::examples::wires; object_examples::examples::crate_; kit_examples::examples::furniture; Table crate::examples::sheet. Object uses crate_ because crate is reserved. Mesh editor component Cargo.toml currently lacks mesh-examples; its examples crate depends on mesh-snapshot and base, not mesh-editor/root, so a direct editor→examples dependency introduces no immediate reverse edge. Graph/Object/Kit editor component manifests already declare their exact example owners. Table has no separate examples package and its editor is mounted in table root. Brep has no separate editor component manifest; its root owns the editor and shared base_examples::examples::solid. Do not add source aliases or guessed exports.
