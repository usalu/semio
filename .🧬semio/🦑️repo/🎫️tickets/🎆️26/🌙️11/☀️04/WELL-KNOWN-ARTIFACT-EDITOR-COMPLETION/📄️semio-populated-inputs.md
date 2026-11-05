# Populated Semio Natural Codec Inputs

All19subset snapshot owners already define populated typed demo functions. These are more reliable inputs than editor example selection that parses umbrella base PRIMARY_TEXT and silently falls back to default. They are guarded by cfg(any(test,feature=testkit)); cross-crate tests must enable the actual snapshot owners testkit feature. Direct typed demo content is source-authoritative, but its codec validity still requires execution.

| Subset | Exact source | Function |
| --- | --- | --- |
| 🔊️audio | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🦀️.rs:157 | demo_audio_snapshot |
| 🎞️animation | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/🦀️.rs:155 | demo_animation_snapshot |
| 🧊️brep | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🦀️.rs:483 | demo_brep_snapshot |
| 📑️document | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🦀️.rs:213 | demo_semio_document_snapshot |
| 🖊️drawing | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs:307 | demo_drawing_snapshot |
| 🔢️value | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📸️snapshot/🦀️.rs:197 | demo_semio_value_snapshot |
| 🔤️text | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🦀️.rs:159 | demo_text_snapshot |
| 🔺️mesh | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/🦀️.rs:224 | demo_mesh_snapshot |
| 📐️cad | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/🦀️.rs:125 | demo_cad_snapshot |
| 🎬️video | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/🦀️.rs:159 | demo_video_snapshot |
| 🖼️image | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🦀️.rs:169 | demo_image_snapshot |
| 📽️presentation | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/🦀️.rs:190 | demo_semio_presentation_snapshot |
| 🕸️graph | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🦀️.rs:268 | demo_graph_snapshot |
| 🌊️flow | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:154 | demo_flow_snapshot |
| 📦️object | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🦀️.rs:189 | demo_object_snapshot |
| 🏛️model | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/🦀️.rs:340 | demo_semio_model_snapshot |
| ✉️base | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🦀️.rs:143 | demo_semio_snapshot |
| 🧰️kit | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs:280 | demo_kit_snapshot |
| 📊️table | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/🦀️.rs:162 | demo_table_snapshot |

## Nonempty Evidence

Audio has two sample channels and tags; animation timelines; brep vertices/edges/faces; document styles/images/blocks; drawing canvas/styles/layers with Path/Text/Image nodes; value map and nodes; text three multilingual marked runs; mesh meshes/materials/textures; CAD layers/blocks/entities; video streams; image a2x2RGBA frame withICC and metadata; presentation masters/layouts/slides; graph nodes and edge; flow nodes and edge; object transform and typed child references; model spatial/elements/relations; base wraps a populated Flow subset; kit types/designs/objects/models/representations; table columns and rows. These are not default-only roundtrips.

Text demo159: three runs with plain English, bold English and linked German text. A minimal authored nonempty text input requires schema and one run with content. Image demo169: width2,height2,RGBA8bitDepth8,one16byteframe,ICC andTitle metadata. A minimal image needs nonzero dimensions and width*height*4RGBAbytes in one frame. Drawing demo307:100x50canvas,style s1,layer l0 containing grouped path/text/image primitives. Prefer that typed demo over inventing a tagged DrawNode JSON shape.

Object/kit/model demos contain child references. Natural snapshot bytes proving these fields survive do not establish full recursive child-document archive persistence; that remains a distinct whole-app/archive lifecycle obligation.
