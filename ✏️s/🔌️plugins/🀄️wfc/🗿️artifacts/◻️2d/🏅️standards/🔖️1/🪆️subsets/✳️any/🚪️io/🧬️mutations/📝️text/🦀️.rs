//! ⚡️ WFC 2D artifact — the mutation vocabulary's single-line TEXT opcodes + grammar.
//!
//! One keyword per `Wfc2dMutation` variant, in the `KINDS` order the catalog
//! (`../../../🔮️oracles/🔣️.json`) declares. The operation twin exists for the same reason the
//! snapshot's does — `Wfc2dTile::media` is a data-carrying enum — and is bridged through the
//! snapshot facet's own `slot_to_dsl`/`tile_to_dsl`/`edge_to_dsl`/`rule_to_dsl` pairs, never a
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
use crate::schema::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dTileMedia;
use crate::standards::v1::subsets::any::io::snapshot::text::{
    edge_from_dsl, edge_to_dsl, rule_from_dsl, rule_to_dsl, slot_from_dsl, slot_to_dsl, tile_from_dsl, tile_to_dsl, Wfc2dRuleDsl, Wfc2dSlotDsl, Wfc2dSlotEdgeDsl, Wfc2dTileDsl,
};

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum Wfc2dOperationDsl {
    ChangeSeed {
        seed: u64,
    },
    CreateSlot {
        #[dsl(block)]
        slot: Wfc2dSlotDsl,
    },
    DeleteSlot {
        id: String,
    },
    MoveSlot {
        id: String,
        x: f64,
        y: f64,
    },
    ResizeSlot {
        id: String,
        width: f64,
        height: f64,
    },
    ConnectSlots {
        #[dsl(block)]
        edge: Wfc2dSlotEdgeDsl,
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
        #[dsl(block)]
        tile: Wfc2dTileDsl,
    },
    DeleteTile {
        id: String,
    },
    ChangeTileWeight {
        tile_id: String,
        weight: f64,
    },
    ChangeTileMedia {
        tile_id: String,
        media: dsl::DslValue,
    },
    CreateRule {
        #[dsl(block)]
        rule: Wfc2dRuleDsl,
    },
    DeleteRule {
        id: String,
    },
}

pub fn operation_to_dsl(operation: &Wfc2dMutation) -> Wfc2dOperationDsl {
    match operation {
        Wfc2dMutation::ChangeSeed(ChangeSeed { seed }) => Wfc2dOperationDsl::ChangeSeed { seed: *seed },
        Wfc2dMutation::CreateSlot(CreateSlot { slot }) => Wfc2dOperationDsl::CreateSlot { slot: slot_to_dsl(slot) },
        Wfc2dMutation::DeleteSlot(DeleteSlot { id }) => Wfc2dOperationDsl::DeleteSlot { id: id.clone() },
        Wfc2dMutation::MoveSlot(MoveSlot { id, x, y }) => Wfc2dOperationDsl::MoveSlot { id: id.clone(), x: *x, y: *y },
        Wfc2dMutation::ResizeSlot(ResizeSlot { id, width, height }) => Wfc2dOperationDsl::ResizeSlot { id: id.clone(), width: *width, height: *height },
        Wfc2dMutation::ConnectSlots(ConnectSlots { edge }) => Wfc2dOperationDsl::ConnectSlots { edge: edge_to_dsl(edge) },
        Wfc2dMutation::DisconnectSlots(DisconnectSlots { id }) => Wfc2dOperationDsl::DisconnectSlots { id: id.clone() },
        Wfc2dMutation::PinSlot(PinSlot { id, tile_id }) => Wfc2dOperationDsl::PinSlot { id: id.clone(), tile_id: tile_id.clone() },
        Wfc2dMutation::UnpinSlot(UnpinSlot { id }) => Wfc2dOperationDsl::UnpinSlot { id: id.clone() },
        Wfc2dMutation::CreateTile(CreateTile { tile }) => Wfc2dOperationDsl::CreateTile { tile: tile_to_dsl(tile) },
        Wfc2dMutation::DeleteTile(DeleteTile { id }) => Wfc2dOperationDsl::DeleteTile { id: id.clone() },
        Wfc2dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight }) => Wfc2dOperationDsl::ChangeTileWeight { tile_id: tile_id.clone(), weight: *weight },
        Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media }) => Wfc2dOperationDsl::ChangeTileMedia { tile_id: tile_id.clone(), media: dsl::to_dsl_value(media).unwrap_or(dsl::DslValue::Null) },
        Wfc2dMutation::CreateRule(CreateRule { rule }) => Wfc2dOperationDsl::CreateRule { rule: rule_to_dsl(rule) },
        Wfc2dMutation::DeleteRule(DeleteRule { id }) => Wfc2dOperationDsl::DeleteRule { id: id.clone() },
    }
}

pub fn operation_from_dsl(operation: Wfc2dOperationDsl) -> Result<Wfc2dMutation, store::TextError> {
    Ok(match operation {
        Wfc2dOperationDsl::ChangeSeed { seed } => Wfc2dMutation::ChangeSeed(ChangeSeed { seed }),
        Wfc2dOperationDsl::CreateSlot { slot } => Wfc2dMutation::CreateSlot(CreateSlot { slot: slot_from_dsl(slot) }),
        Wfc2dOperationDsl::DeleteSlot { id } => Wfc2dMutation::DeleteSlot(DeleteSlot { id }),
        Wfc2dOperationDsl::MoveSlot { id, x, y } => Wfc2dMutation::MoveSlot(MoveSlot { id, x, y }),
        Wfc2dOperationDsl::ResizeSlot { id, width, height } => Wfc2dMutation::ResizeSlot(ResizeSlot { id, width, height }),
        Wfc2dOperationDsl::ConnectSlots { edge } => Wfc2dMutation::ConnectSlots(ConnectSlots { edge: edge_from_dsl(edge) }),
        Wfc2dOperationDsl::DisconnectSlots { id } => Wfc2dMutation::DisconnectSlots(DisconnectSlots { id }),
        Wfc2dOperationDsl::PinSlot { id, tile_id } => Wfc2dMutation::PinSlot(PinSlot { id, tile_id }),
        Wfc2dOperationDsl::UnpinSlot { id } => Wfc2dMutation::UnpinSlot(UnpinSlot { id }),
        Wfc2dOperationDsl::CreateTile { tile } => Wfc2dMutation::CreateTile(CreateTile { tile: tile_from_dsl(tile)? }),
        Wfc2dOperationDsl::DeleteTile { id } => Wfc2dMutation::DeleteTile(DeleteTile { id }),
        Wfc2dOperationDsl::ChangeTileWeight { tile_id, weight } => Wfc2dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight }),
        Wfc2dOperationDsl::ChangeTileMedia { tile_id, media } => {
            let media: Wfc2dTileMedia = match media {
                dsl::DslValue::Null => Wfc2dTileMedia::default(),
                other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
            };
            Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media })
        }
        Wfc2dOperationDsl::CreateRule { rule } => Wfc2dMutation::CreateRule(CreateRule { rule: rule_from_dsl(rule) }),
        Wfc2dOperationDsl::DeleteRule { id } => Wfc2dMutation::DeleteRule(DeleteRule { id }),
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for Wfc2dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown wfc2d mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `Wfc2dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Wfc2dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        operation_from_dsl(<Wfc2dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Wfc2dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfc2d` mutation line.
pub fn parse_op(line: &str) -> Result<Wfc2dMutation, store::TextError> {
    <Wfc2dMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `Wfc2dMutation` back to its single-line form.
pub fn print_op(operation: &Wfc2dMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type Wfc2dMutationText = String;
//#endregion 🚚️Carrier
