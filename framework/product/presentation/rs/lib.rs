//! 🎞️ Presentation deck document + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PRESENTATION_DOCUMENT_SCHEMA: &str = "presentation.deck";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    pub frame: FigureTileFrame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    pub crop: FigureTileFrame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationDeck {
    pub schema: String,
    pub source: FigureTileSource,
    pub tiles: Vec<FigureTileDraft>,
}

pub type PresentationEnvelope = DocumentVcsEnvelope<PresentationDeck, PresentationOp>;
pub type PresentationStore = DocumentVcsStore<PresentationDeck, PresentationOp>;

pub fn empty_presentation_projection() -> PresentationDeck {
    PresentationDeck {
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        source: FigureTileSource {
            src: String::new(),
            kind: "figure".into(),
            frame: FigureTileFrame {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        },
        tiles: Vec::new(),
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum PresentationOp {
    SetSource {
        source: FigureTileSource,
    },
    AddTile {
        tile: FigureTileDraft,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveTile {
        tile_id: String,
    },
    RenameTile {
        tile_id: String,
        name: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationDiff {
    pub source: Option<FigureTileSource>,
    pub tiles: Option<CollectionDiff<String, FigureTileDraftPatch, FigureTileDraft>>,
}

impl OperationDiff<PresentationDeck> for PresentationDiff {
    fn apply(&self, projection: &PresentationDeck) -> PresentationDeck {
        let mut next = projection.clone();
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(tiles) = &self.tiles {
            for id in &tiles.removed {
                next.tiles.retain(|tile| tile.id != *id);
            }
            for patch in &tiles.modified {
                for tile in &mut next.tiles {
                    if tile.id == patch.id {
                        if let Some(name) = &patch.patch.name {
                            tile.name = name.clone();
                        }
                    }
                }
            }
            for added in &tiles.added {
                next.tiles.push(added.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.source.is_some() {
            self.source = other.source;
        }
        match (&mut self.tiles, other.tiles) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.tiles = Some(b),
            _ => {}
        }
    }
}

impl Operation<PresentationDeck> for PresentationOp {
    type Diff = PresentationDiff;

    fn diff(&self, _projection: &PresentationDeck) -> PresentationDiff {
        match self {
            PresentationOp::SetSource { source } => PresentationDiff {
                source: Some(source.clone()),
                ..Default::default()
            },
            PresentationOp::AddTile { tile, .. } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    added: vec![tile.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            PresentationOp::RemoveTile { tile_id } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    removed: vec![tile_id.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            PresentationOp::RenameTile { tile_id, name } => PresentationDiff {
                tiles: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: tile_id.clone(),
                        patch: FigureTileDraftPatch {
                            name: Some(name.clone()),
                        },
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &PresentationDeck) -> Vec<Self> {
        match self {
            PresentationOp::SetSource { .. } => vec![PresentationOp::SetSource {
                source: projection.source.clone(),
            }],
            PresentationOp::AddTile { tile, .. } => vec![PresentationOp::RemoveTile {
                tile_id: tile.id.clone(),
            }],
            PresentationOp::RemoveTile { tile_id } => projection
                .tiles
                .iter()
                .find(|t| t.id == *tile_id)
                .map(|tile| vec![PresentationOp::AddTile {
                    tile: tile.clone(),
                    index: None,
                }])
                .unwrap_or_default(),
            PresentationOp::RenameTile { tile_id, .. } => projection
                .tiles
                .iter()
                .find(|t| t.id == *tile_id)
                .map(|tile| {
                    vec![PresentationOp::RenameTile {
                        tile_id: tile_id.clone(),
                        name: tile.name.clone(),
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
    pub struct PresentationDocumentVcs {
        store: RefCell<PresentationStore>,
    }

    #[wasm_bindgen]
    impl PresentationDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<PresentationDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: PresentationEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    PresentationStore::new(envelope)
                }
                None => PresentationStore::new(create_document_vcs_envelope(
                    PRESENTATION_DOCUMENT_SCHEMA,
                    "presentation",
                    empty_presentation_projection(),
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
    fn presentation_deck_materializes() {
        let mut store = PresentationStore::new(create_document_vcs_envelope(
            PRESENTATION_DOCUMENT_SCHEMA,
            "presentation",
            empty_presentation_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![PresentationOp::AddTile {
                    tile: FigureTileDraft {
                        id: "t1".into(),
                        name: "A".into(),
                        crop: FigureTileFrame {
                            x: 0.0,
                            y: 0.0,
                            width: 1.0,
                            height: 1.0,
                        },
                    },
                    index: None,
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").tiles.len(), 1);
    }
}
//#endregion 🧪Tests
