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
use crate::schema::mutations::drag_slots::DragSlots;
use crate::schema::mutations::move_slot::MoveSlot;
use crate::schema::mutations::pin_slot::PinSlot;
use crate::schema::mutations::resize_slot::ResizeSlot;
use crate::schema::mutations::set_slot_positions::{SetSlotPositions, Wfc2dSlotPosition};
use crate::schema::mutations::unpin_slot::UnpinSlot;
use crate::schema::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dTileMedia;
use crate::standards::v1::subsets::any::io::text::snapshot::{
    edge_from_dsl, edge_to_dsl, rule_from_dsl, rule_to_dsl, slot_from_dsl, slot_to_dsl, tile_from_dsl, tile_to_dsl, Wfc2dRuleDsl, Wfc2dSlotDsl, Wfc2dSlotEdgeDsl, Wfc2dTileDsl,
};

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
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
        media: semio_framework_value::DslValue,
    },
    CreateRule {
        #[dsl(block)]
        rule: Wfc2dRuleDsl,
    },
    DeleteRule {
        id: String,
    },
    DragSlots {
        targets: Vec<String>,
        dx: f64,
        dy: f64,
    },
    SetSlotPositions {
        ids: Vec<String>,
        xs: Vec<f64>,
        ys: Vec<f64>,
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
        Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media }) => Wfc2dOperationDsl::ChangeTileMedia { tile_id: tile_id.clone(), media: semio_framework_value::ToValue::to_value(media) },
        Wfc2dMutation::CreateRule(CreateRule { rule }) => Wfc2dOperationDsl::CreateRule { rule: rule_to_dsl(rule) },
        Wfc2dMutation::DeleteRule(DeleteRule { id }) => Wfc2dOperationDsl::DeleteRule { id: id.clone() },
        Wfc2dMutation::DragSlots(DragSlots { targets, dx, dy }) => Wfc2dOperationDsl::DragSlots { targets: targets.clone(), dx: *dx, dy: *dy },
        Wfc2dMutation::SetSlotPositions(SetSlotPositions { positions }) => Wfc2dOperationDsl::SetSlotPositions {
            ids: positions.iter().map(|position| position.id.clone()).collect(),
            xs: positions.iter().map(|position| position.x).collect(),
            ys: positions.iter().map(|position| position.y).collect(),
        },
    }
}

pub fn operation_from_dsl(operation: Wfc2dOperationDsl) -> Result<Wfc2dMutation, semio_framework_diagnostic::TextError> {
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
                semio_framework_value::DslValue::Null => Wfc2dTileMedia::default(),
                other => semio_framework_value::FromValue::from_value(other).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid tile media: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
            };
            Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media })
        }
        Wfc2dOperationDsl::CreateRule { rule } => Wfc2dMutation::CreateRule(CreateRule { rule: rule_from_dsl(rule) }),
        Wfc2dOperationDsl::DeleteRule { id } => Wfc2dMutation::DeleteRule(DeleteRule { id }),
        Wfc2dOperationDsl::DragSlots { targets, dx, dy } => Wfc2dMutation::DragSlots(DragSlots { targets, dx, dy }),
        Wfc2dOperationDsl::SetSlotPositions { ids, xs, ys } => {
            if xs.len() != ids.len() || ys.len() != ids.len() {
                return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "set-slot-positions carries one x and one y per id", semio_framework_diagnostic::TextSpan::at(1, 1)));
            }
            let positions = ids.into_iter().zip(xs).zip(ys).map(|((id, x), y)| Wfc2dSlotPosition { id, x, y }).collect();
            Wfc2dMutation::SetSlotPositions(SetSlotPositions { positions })
        }
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for Wfc2dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown wfc2d mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// ⚡️ `Wfc2dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Wfc2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        operation_from_dsl(<Wfc2dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Wfc2dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfc2d` mutation line.
pub fn parse_op(line: &str) -> Result<Wfc2dMutation, semio_framework_diagnostic::TextError> {
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

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::standards::v1::subsets::any::schema::mutations::connect_slots::connect_slots;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_slot::create_slot;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::standards::v1::subsets::any::schema::mutations::delete_slot::delete_slot;
use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_slots::disconnect_slots;
use crate::standards::v1::subsets::any::schema::mutations::drag_slots::drag_slots;
use crate::standards::v1::subsets::any::schema::mutations::move_slot::move_slot;
use crate::standards::v1::subsets::any::schema::mutations::pin_slot::pin_slot;
use crate::standards::v1::subsets::any::schema::mutations::resize_slot::resize_slot;
use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::{set_slot_positions, Wfc2dSlotPosition};
use crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot;

/// 🌉️ The language-neutral report of one committed specification vector — decoded, diffed, applied and inverted
/// through this subset's production JSON codec and `Mutation` implementation — that the `mutate-wfc2d` case's
/// subject half judges with `law::vector`. Its signature names only `str`, so a generated test host reaches it.
/// @see store::os_store::test_support::mutation_report_json
pub fn wfc2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Wfc2dSnapshot, Wfc2dMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_codec::*;
