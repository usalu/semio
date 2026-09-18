//! ⚡️ `wfc3d` artifact — the mutation vocabulary's single-line TEXT opcodes + grammar.
//!
//! One keyword per `Wfc3dMutation` variant, in the `KINDS` order the catalog
//! (`../../../🔮️oracles/🔣️.json`) declares. The operation twin exists for the same reason the
//! snapshot's does — `Tile::media` reaches a foreign `store::ArtifactChild` — and is bridged through
//! the snapshot facet's own `slot_to_dsl`/`edge_to_dsl`/`tile_to_dsl`/`rule_to_dsl` pairs, never a
//! second conversion.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::change_seed::ChangeSeed;
use crate::schema::mutations::change_tile_media::ChangeTileMedia;
use crate::schema::mutations::change_tile_weight::ChangeTileWeight;
use crate::schema::mutations::connect_slots::ConnectSlots;
use crate::schema::mutations::create_rule::CreateRule;
use crate::schema::mutations::create_slot::CreateSlot;
use crate::schema::mutations::create_tile::CreateTile;
use crate::schema::mutations::delete_rule::DeleteRule;
use crate::schema::mutations::delete_slot::DeleteSlot;
use crate::schema::mutations::delete_tile::DeleteTile;
use crate::schema::mutations::disconnect_slots::DisconnectSlots;
use crate::schema::mutations::move_slot::MoveSlot;
use crate::schema::mutations::pin_slot::PinSlot;
use crate::schema::mutations::resize_slot::ResizeSlot;
use crate::schema::mutations::unpin_slot::UnpinSlot;
use crate::schema::mutations::Wfc3dMutation;
use crate::schema::snapshot::text::{edge_from_dsl, edge_to_dsl, rule_from_dsl, rule_to_dsl, slot_from_dsl, slot_to_dsl, tile_from_dsl, tile_to_dsl, GraphRuleDsl, Slot3dDsl, SlotEdgeDsl, TileDsl};
use crate::schema::snapshot::TileMedia3d;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum Wfc3dOperationDsl {
    CreateSlot {
        index: usize,
        #[dsl(block)]
        slot: Slot3dDsl,
    },
    DeleteSlot {
        id: String,
    },
    MoveSlot {
        id: String,
        x: f64,
        y: f64,
        z: f64,
    },
    ResizeSlot {
        id: String,
        width: f64,
        height: f64,
        depth: f64,
    },
    ConnectSlots {
        index: usize,
        #[dsl(block)]
        edge: SlotEdgeDsl,
    },
    DisconnectSlots {
        id: String,
    },
    PinSlot {
        id: String,
        tile_id: String,
    },
    UnpinSlot {
        id: String,
    },
    CreateTile {
        index: usize,
        #[dsl(block)]
        tile: TileDsl,
    },
    DeleteTile {
        id: String,
    },
    ChangeTileWeight {
        id: String,
        weight: f64,
    },
    ChangeTileMedia {
        id: String,
        media: dsl::DslValue,
    },
    CreateRule {
        index: usize,
        #[dsl(block)]
        rule: GraphRuleDsl,
    },
    DeleteRule {
        id: String,
    },
    ChangeSeed {
        seed: u64,
    },
}

pub fn operation_to_dsl(operation: &Wfc3dMutation) -> Wfc3dOperationDsl {
    match operation {
        Wfc3dMutation::CreateSlot(CreateSlot { index, slot }) => Wfc3dOperationDsl::CreateSlot { index: *index, slot: slot_to_dsl(slot) },
        Wfc3dMutation::DeleteSlot(DeleteSlot { id }) => Wfc3dOperationDsl::DeleteSlot { id: id.clone() },
        Wfc3dMutation::MoveSlot(MoveSlot { id, x, y, z }) => Wfc3dOperationDsl::MoveSlot { id: id.clone(), x: *x, y: *y, z: *z },
        Wfc3dMutation::ResizeSlot(ResizeSlot { id, width, height, depth }) => Wfc3dOperationDsl::ResizeSlot { id: id.clone(), width: *width, height: *height, depth: *depth },
        Wfc3dMutation::ConnectSlots(ConnectSlots { index, edge }) => Wfc3dOperationDsl::ConnectSlots { index: *index, edge: edge_to_dsl(edge) },
        Wfc3dMutation::DisconnectSlots(DisconnectSlots { id }) => Wfc3dOperationDsl::DisconnectSlots { id: id.clone() },
        Wfc3dMutation::PinSlot(PinSlot { id, tile_id }) => Wfc3dOperationDsl::PinSlot { id: id.clone(), tile_id: tile_id.clone() },
        Wfc3dMutation::UnpinSlot(UnpinSlot { id }) => Wfc3dOperationDsl::UnpinSlot { id: id.clone() },
        Wfc3dMutation::CreateTile(CreateTile { index, tile }) => Wfc3dOperationDsl::CreateTile { index: *index, tile: tile_to_dsl(tile) },
        Wfc3dMutation::DeleteTile(DeleteTile { id }) => Wfc3dOperationDsl::DeleteTile { id: id.clone() },
        Wfc3dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }) => Wfc3dOperationDsl::ChangeTileWeight { id: id.clone(), weight: *weight },
        Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media }) => Wfc3dOperationDsl::ChangeTileMedia { id: id.clone(), media: dsl::to_dsl_value(media).unwrap_or(dsl::DslValue::Null) },
        Wfc3dMutation::CreateRule(CreateRule { index, rule }) => Wfc3dOperationDsl::CreateRule { index: *index, rule: rule_to_dsl(rule) },
        Wfc3dMutation::DeleteRule(DeleteRule { id }) => Wfc3dOperationDsl::DeleteRule { id: id.clone() },
        Wfc3dMutation::ChangeSeed(ChangeSeed { seed }) => Wfc3dOperationDsl::ChangeSeed { seed: *seed },
    }
}

pub fn operation_from_dsl(operation: Wfc3dOperationDsl) -> Result<Wfc3dMutation, store::TextError> {
    Ok(match operation {
        Wfc3dOperationDsl::CreateSlot { index, slot } => Wfc3dMutation::CreateSlot(CreateSlot { index, slot: slot_from_dsl(slot) }),
        Wfc3dOperationDsl::DeleteSlot { id } => Wfc3dMutation::DeleteSlot(DeleteSlot { id }),
        Wfc3dOperationDsl::MoveSlot { id, x, y, z } => Wfc3dMutation::MoveSlot(MoveSlot { id, x, y, z }),
        Wfc3dOperationDsl::ResizeSlot { id, width, height, depth } => Wfc3dMutation::ResizeSlot(ResizeSlot { id, width, height, depth }),
        Wfc3dOperationDsl::ConnectSlots { index, edge } => Wfc3dMutation::ConnectSlots(ConnectSlots { index, edge: edge_from_dsl(edge) }),
        Wfc3dOperationDsl::DisconnectSlots { id } => Wfc3dMutation::DisconnectSlots(DisconnectSlots { id }),
        Wfc3dOperationDsl::PinSlot { id, tile_id } => Wfc3dMutation::PinSlot(PinSlot { id, tile_id }),
        Wfc3dOperationDsl::UnpinSlot { id } => Wfc3dMutation::UnpinSlot(UnpinSlot { id }),
        Wfc3dOperationDsl::CreateTile { index, tile } => Wfc3dMutation::CreateTile(CreateTile { index, tile: tile_from_dsl(tile)? }),
        Wfc3dOperationDsl::DeleteTile { id } => Wfc3dMutation::DeleteTile(DeleteTile { id }),
        Wfc3dOperationDsl::ChangeTileWeight { id, weight } => Wfc3dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }),
        Wfc3dOperationDsl::ChangeTileMedia { id, media } => {
            let media: TileMedia3d = match media {
                dsl::DslValue::Null => TileMedia3d::default(),
                other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
            };
            Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media })
        }
        Wfc3dOperationDsl::CreateRule { index, rule } => Wfc3dMutation::CreateRule(CreateRule { index, rule: rule_from_dsl(rule) }),
        Wfc3dOperationDsl::DeleteRule { id } => Wfc3dMutation::DeleteRule(DeleteRule { id }),
        Wfc3dOperationDsl::ChangeSeed { seed } => Wfc3dMutation::ChangeSeed(ChangeSeed { seed }),
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for Wfc3dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown wfc3d mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `Wfc3dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Wfc3dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        operation_from_dsl(<Wfc3dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Wfc3dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfc3d` mutation line.
pub fn parse_op(line: &str) -> Result<Wfc3dMutation, store::TextError> {
    <Wfc3dMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `Wfc3dMutation` back to its single-line form.
pub fn print_op(operation: &Wfc3dMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type Wfc3dMutationText = String;
//#endregion 🚚️Carrier
