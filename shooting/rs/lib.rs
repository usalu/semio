//! 📸 Shooting scene document + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const SHOOTING_DOCUMENT_SCHEMA: &str = "shooting.scene/v1";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingEntity {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingScene {
    pub schema: String,
    pub id: String,
    pub entities: Vec<ShootingEntity>,
}

pub type ShootingEnvelope = DocumentVcsEnvelope<ShootingScene, ShootingOp>;
pub type ShootingStore = DocumentVcsStore<ShootingScene, ShootingOp>;

pub fn empty_shooting_projection() -> ShootingScene {
    ShootingScene {
        schema: SHOOTING_DOCUMENT_SCHEMA.into(),
        id: "shooting".into(),
        entities: Vec::new(),
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum ShootingOp {
    AddEntity {
        entity: ShootingEntity,
    },
    RemoveEntity {
        entity_id: String,
    },
    RenameEntity {
        entity_id: String,
        label: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingEntityPatch {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingDiff {
    pub entities: Option<CollectionDiff<String, ShootingEntityPatch, ShootingEntity>>,
}

impl OperationDiff<ShootingScene> for ShootingDiff {
    fn apply(&self, projection: &ShootingScene) -> ShootingScene {
        let mut next = projection.clone();
        if let Some(entities) = &self.entities {
            for id in &entities.removed {
                next.entities.retain(|entity| entity.id != *id);
            }
            for patch in &entities.modified {
                for entity in &mut next.entities {
                    if entity.id == patch.id {
                        if let Some(label) = &patch.patch.label {
                            entity.label = label.clone();
                        }
                    }
                }
            }
            for added in &entities.added {
                next.entities.push(added.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.entities, other.entities) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.entities = Some(b),
            _ => {}
        }
    }
}

impl Operation<ShootingScene> for ShootingOp {
    type Diff = ShootingDiff;

    fn diff(&self, projection: &ShootingScene) -> ShootingDiff {
        match self {
            ShootingOp::AddEntity { entity } => ShootingDiff {
                entities: Some(CollectionDiff {
                    added: vec![entity.clone()],
                    ..Default::default()
                }),
            },
            ShootingOp::RemoveEntity { entity_id } => ShootingDiff {
                entities: Some(CollectionDiff {
                    removed: vec![entity_id.clone()],
                    ..Default::default()
                }),
            },
            ShootingOp::RenameEntity { entity_id, label } => ShootingDiff {
                entities: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: entity_id.clone(),
                        patch: ShootingEntityPatch {
                            label: Some(label.clone()),
                        },
                    }],
                    ..Default::default()
                }),
            },
        }
    }

    fn backwards(&self, projection: &ShootingScene) -> Vec<Self> {
        match self {
            ShootingOp::AddEntity { entity } => vec![ShootingOp::RemoveEntity {
                entity_id: entity.id.clone(),
            }],
            ShootingOp::RemoveEntity { entity_id } => projection
                .entities
                .iter()
                .find(|e| e.id == *entity_id)
                .map(|entity| vec![ShootingOp::AddEntity { entity: entity.clone() }])
                .unwrap_or_default(),
            ShootingOp::RenameEntity { entity_id, .. } => projection
                .entities
                .iter()
                .find(|e| e.id == *entity_id)
                .map(|entity| {
                    vec![ShootingOp::RenameEntity {
                        entity_id: entity_id.clone(),
                        label: entity.label.clone(),
                    }]
                })
                .unwrap_or_default(),
        }
    }
}
//#endregion 🔖Ops

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ShootingDocumentVcs {
        store: RefCell<ShootingStore>,
    }

    #[wasm_bindgen]
    impl ShootingDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<ShootingDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ShootingEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ShootingStore::new(envelope)
                }
                None => ShootingStore::new(create_document_vcs_envelope(
                    SHOOTING_DOCUMENT_SCHEMA,
                    "shooting",
                    empty_shooting_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shooting_projection_round_trip() {
        let mut store = ShootingStore::new(create_document_vcs_envelope(
            SHOOTING_DOCUMENT_SCHEMA,
            "shooting",
            empty_shooting_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![ShootingOp::AddEntity {
                    entity: ShootingEntity {
                        id: "e1".into(),
                        label: "Camera".into(),
                        asset_id: None,
                    },
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").entities.len(), 1);
    }
}
//#endregion 🧪Tests
