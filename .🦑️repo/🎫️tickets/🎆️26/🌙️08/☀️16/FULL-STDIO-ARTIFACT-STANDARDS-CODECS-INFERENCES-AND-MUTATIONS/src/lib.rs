//! 🧪️ Ticket-local executable harness for the unmounted create-scene leaf.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfScene {
    #[serde(default)]
    pub nodes: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GltfDocument {
    pub scene: Option<usize>,
    pub scenes: Vec<GltfScene>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GltfSnapshot {
    pub schema: String,
    pub document: GltfDocument,
}

pub mod artifacts {
    pub mod gltf {
        pub use crate::GltfSnapshot;

        pub mod schema {
            pub mod snapshot {
                pub use crate::GltfScene;
            }

            pub mod mutations {
                use crate::GltfSnapshot;

                #[derive(Clone, Debug, PartialEq, Eq)]
                pub struct GltfMutationLeafError {
                    pub code: String,
                    pub path: String,
                    pub detail: String,
                }

                pub struct GltfMutationLeafPlan {
                    pub diff_payload: Vec<u8>,
                    pub inverse_payload: Vec<u8>,
                    pub touched_paths: Vec<String>,
                }

                pub struct GltfMutationLeafApplication {
                    pub snapshot: GltfSnapshot,
                    pub touched_paths: Vec<String>,
                }

                #[derive(Clone, Copy)]
                pub struct GltfMutationLeafDescriptor {
                    pub command_id: &'static str,
                    pub version: u16,
                    pub plan: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError>,
                    pub plan_inverse: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafPlan, GltfMutationLeafError>,
                    pub apply_diff: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError>,
                    pub apply_inverse: fn(&[u8], &GltfSnapshot) -> Result<GltfMutationLeafApplication, GltfMutationLeafError>,
                }

                pub mod top_level_collections_private {
                    use crate::{GltfDocument, GltfSnapshot};
                    use serde::{Deserialize, Serialize};

                    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
                    #[serde(rename_all = "camelCase")]
                    pub struct GltfTopLevelMutationRejection {
                        pub code: String,
                        pub path: String,
                        pub detail: String,
                    }

                    pub fn reject(code: impl Into<String>, path: impl Into<String>, detail: impl Into<String>) -> GltfTopLevelMutationRejection {
                        GltfTopLevelMutationRejection { code: code.into(), path: path.into(), detail: detail.into() }
                    }

                    #[derive(Clone, Copy)]
                    pub enum GltfTopLevelFamily {
                        Scenes,
                    }

                    pub enum Change {
                        Insert(usize),
                        Delete(usize),
                    }

                    pub fn repair(document: &mut GltfDocument, family: GltfTopLevelFamily, change: &Change) -> Result<(), GltfTopLevelMutationRejection> {
                        if !matches!(family, GltfTopLevelFamily::Scenes) {
                            return Ok(());
                        }
                        document.scene = match (document.scene, change) {
                            (Some(scene), Change::Insert(position)) if scene >= *position => Some(scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond usize"))?),
                            (Some(scene), Change::Delete(position)) if scene == *position => None,
                            (Some(scene), Change::Delete(position)) if scene > *position => Some(scene - 1),
                            (scene, _) => scene,
                        };
                        Ok(())
                    }

                    pub fn scenes_op(snapshot: &mut GltfSnapshot, family: GltfTopLevelFamily, index: usize, position: Option<usize>, order: Option<&[usize]>) -> Result<(), GltfTopLevelMutationRejection> {
                        if !matches!(family, GltfTopLevelFamily::Scenes) || position.is_some() || order.is_some() || index >= snapshot.document.scenes.len() {
                            return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "position must address the created scene"));
                        }
                        repair(&mut snapshot.document, family, &Change::Delete(index))?;
                        snapshot.document.scenes.remove(index);
                        Ok(())
                    }
                }

                #[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/🔺️diff/🦀️component.rs"]
                pub mod create_scene_diff;
                #[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/↩️inverse/🦀️component.rs"]
                pub mod create_scene_inverse;
                #[path = "../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/🦠️mutation/🦀️component.rs"]
                pub mod create_scene_mutation;

                pub mod create_scene {
                    pub use super::create_scene_diff as diff;
                    pub use super::create_scene_inverse as inverse;
                    pub use super::create_scene_mutation as mutation;

                    #[path = "../../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/🦀️component.rs"]
                    mod component;
                    pub use component::*;

                    #[cfg(test)]
                    #[path = "../../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/create-scene/🧪️contract/🦀️component.rs"]
                    pub mod contract;
                }
            }
        }
    }
}
