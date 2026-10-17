//! 💡️ Cad inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

//#region 🧩️Modules
#[path = "."]
pub mod bounds {
    #[path = "📦bounds/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod component {
                        #[path = "."]
                        pub mod binary {
                            #[path = "../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                            pub mod inferences;
                        }
                        #[path = "."]
                        pub mod text {
                            #[path = "../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                            pub mod inferences;
                        }
                    }
                    pub use component::*;
                }
            }
        }
    }
}
//#endregion 🧩️Modules

use crate::{bounds::object_count, bounds::scene_bounds, bounds::vertex_count};
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a cad snapshot. Today: object/brep-vertex counts and the 3d
/// bounding box across every pane's object origins and vertex positions (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, a full
/// scan over the document is cheap at cad scale.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad.inference")]
pub struct CadInference {
    #[derived]
    pub object_count: usize,
    #[derived]
    pub vertex_count: usize,
    #[derived]
    pub bounds: Option<CadBounds>,
}

impl protocol::Inference<CadSnapshot> for CadInference {
    fn infer(snapshot: &CadSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { object_count: object_count(snapshot), vertex_count: vertex_count(snapshot), bounds: scene_bounds(snapshot) }

        })
    }
}

impl protocol::InferenceSpec<CadSnapshot> for CadInference {
    fn inference_schema_id() -> &'static str {
        "s.cad.cad.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.objectCount", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.vertexCount", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
            protocol::InferenceFieldSpec { id: "s.cad.cad.inference.bounds", reads: &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.cad.cad.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `cad_artifact_schema_descriptor`'s registration.
pub fn cad_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.cad.cad.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔄️DeriveTransformation
// 🐛️ Relocated verbatim from the deleted `⚙️engine/🔄️transformation/🦀️.rs` (ticket
// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- pure `kernel + [CadObject]` derived
// classification/reclassification (rule 2: pure fn snapshot/objects -> derived objects), not
// stateful app behaviour.
#[path = "🔄️derive-transformation/🦀️.rs"]
mod derive_transformation;
pub use derive_transformation::*;
//#endregion 🔄️DeriveTransformation

//#region 🔍️ConstructQuery
// 🐛️ Relocated verbatim from the deleted `⚙️engine/🔍️construct/🦀️.rs` -- a read-only
// Jack `QueryableGraph` adapter over one `CadGeometry` pane (rule 2/CAD-map: query -> D4
// inference-shaped derived compute).
#[path = "🔍️construct-query/🦀️.rs"]
mod construct_query;
pub use construct_query::*;
//#endregion 🔍️ConstructQuery

//#region 🖥️SceneCompute
// 🐛️ Relocated from the deleted `⚙️engine/🦀️.rs`'s `🔖️Compute`/`🔖️Register` regions
// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) -- pure derived compute over
// `CadSnapshot`/`CadObject`/`CadCamera` (rule 2/3), never io bytes, never an app type. The
// io-bound half of that file (native solid export, file-payload import, foreign-format bridges)
// moved to `🚪️io/🦀️.rs` instead; the interaction statechart moved to the app's own
// `⚙️engine` (D5 behavioural).
#[path = "🖥️scene-compute/🦀️.rs"]
mod scene_compute;
pub use scene_compute::*;
//#endregion 🖥️SceneCompute

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::bounds::CadBounds;
//#endregion 🔁️Re-exports
