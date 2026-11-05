# Semio19 Actual SDK Type Map

Neutral corpus at rootartifact/🧪️tests/📄️natural/🧫️fixtures/{🔣️.json,📋️schema.json}; scope-aware Bun metadata law in sibling🟦️.ts. These inputs describe metadata and nonempty discriminators; actual Rust SDK roundtrip is still required.

| Subset | Actual editor type export | Snapshot owner/testkit package key | Snapshot type | Populated demo |
| --- | --- | --- | --- | --- |
| 🔊️audio | semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor | semio-s-artifact-stdio-semio-audio-model | SemioAudioSnapshot | schema::snapshot::demo_audio_snapshot |
| 🎞️animation | semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor | semio-s-artifact-stdio-semio-animation-schema | SemioAnimationSnapshot | schema::snapshot::demo_animation_snapshot |
| 🧊️brep | semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor | semio-s-artifact-stdio-semio-brep-snapshot | SemioBrepSnapshot | schema::snapshot::demo_brep_snapshot |
| 📑️document | semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor | semio-s-artifact-stdio-semio-document-snapshot | SemioDocumentSnapshot | schema::snapshot::demo_semio_document_snapshot |
| 🖊️drawing | semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor | semio-s-artifact-stdio-semio-drawing-snapshot | SemioDrawingSnapshot | schema::snapshot::demo_drawing_snapshot |
| 🔢️value | semio_s_artifact_stdio_semio_value::editor::SemioValueEditor | semio-s-artifact-stdio-semio-value-model | SemioValueSnapshot | schema::snapshot::demo_semio_value_snapshot |
| 🔤️text | semio_s_artifact_stdio_semio_text::editor::SemioTextEditor | semio-s-artifact-stdio-semio-text-model | SemioTextSnapshot | schema::snapshot::demo_text_snapshot |
| 🔺️mesh | semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor | semio-s-artifact-stdio-semio-mesh-snapshot | SemioMeshSnapshot | schema::snapshot::demo_mesh_snapshot |
| 📐️cad | semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor | semio-s-artifact-stdio-semio-cad-snapshot | SemioCadSnapshot | schema::snapshot::demo_cad_snapshot |
| 🎬️video | semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor | semio-s-artifact-stdio-semio-video-model | SemioVideoSnapshot | schema::snapshot::demo_video_snapshot |
| 🖼️image | semio_s_artifact_stdio_semio_image::editor::SemioImageEditor | semio-s-artifact-stdio-semio-image-snapshot | SemioImageSnapshot | schema::snapshot::demo_image_snapshot |
| 📽️presentation | semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor | semio-s-artifact-stdio-semio-presentation-snapshot | SemioPresentationSnapshot | schema::snapshot::demo_semio_presentation_snapshot |
| 🕸️graph | semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor | semio-s-artifact-stdio-semio-graph-snapshot | SemioGraphSnapshot | schema::snapshot::demo_graph_snapshot |
| 🌊️flow | semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor | semio-s-artifact-stdio-semio-flow-snapshot | SemioFlowSnapshot | schema::snapshot::demo_flow_snapshot |
| 📦️object | semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor | semio-s-artifact-stdio-semio-object-snapshot | SemioObjectSnapshot | schema::snapshot::demo_object_snapshot |
| 🏛️model | semio_s_artifact_stdio_semio_model::editor::SemioModelEditor | semio-s-artifact-stdio-semio-model-snapshot | SemioModelSnapshot | schema::snapshot::demo_semio_model_snapshot |
| ✉️base | semio_s_artifact_stdio_semio_editor::editor::SemioAnyEditor | semio-s-artifact-stdio-semio-snapshot | SemioSnapshot | schema::snapshot::demo_semio_snapshot |
| 🧰️kit | semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor | semio-s-artifact-stdio-semio-kit-snapshot | SemioKitSnapshot | schema::snapshot::demo_kit_snapshot |
| 📊️table | semio_s_artifact_stdio_semio_table::editor::SemioTableEditor | semio-s-artifact-stdio-semio-table-model | SemioTableSnapshot | schema::snapshot::demo_table_snapshot |

Each snapshot owner above is derived from the editor explicit import or the actual snapshot component Cargo manifest, not guessed from the subset name. Enable testkit explicitly on that owner (dev dependencies only) to expose demo. Root editor packages export the actual editor module used by source composition. The base foundation package does not export SemioAnyEditor. Use its actual editor component package semio-s-artifact-stdio-semio-editor, whose module is editor; the umbrella artifact root also exports it.

Initial actual Bun red:2pass1fail6expectations685ms, all17misplaced factory macros detected; Mesh/Brep were correctly inside ArtifactEditor. Root moved17macros and reports actualgreen3pass0fail23expectations167ms. No Cargo/Rust edits by this agent.

After repair a source-scope census finds91ArtifactEditor implementations across90stdio+energy: all91have native metadata, encode and decode declarations inside the actual Eimplementation (or correctly placed Semio macro).65factory-containing sources have no misplaced natural codec method/macro. This is declaration scope evidence, not executed codec validity.

Independent verification after the source repair: `repo tools exec --light bun -- test ./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/📄️natural/🟦️.ts` exited 0: 3 passed, 0 failed, 23 expectations, 182 ms. This validates schema, exact source roster and actual implementation scope only.
