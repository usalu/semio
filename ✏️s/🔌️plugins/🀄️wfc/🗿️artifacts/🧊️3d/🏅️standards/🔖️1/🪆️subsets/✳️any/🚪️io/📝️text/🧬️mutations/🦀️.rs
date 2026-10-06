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
use crate::schema::mutations::drag_slots::DragSlots;
use crate::schema::mutations::move_slot::MoveSlot;
use crate::schema::mutations::pin_slot::PinSlot;
use crate::schema::mutations::resize_slot::ResizeSlot;
use crate::schema::mutations::set_slot_positions::{SetSlotPositions, Wfc3dSlotPosition};
use crate::schema::mutations::unpin_slot::UnpinSlot;
use crate::schema::mutations::Wfc3dMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::{edge_from_dsl, edge_to_dsl, rule_from_dsl, rule_to_dsl, slot_from_dsl, slot_to_dsl, tile_from_dsl, tile_to_dsl, GraphRuleDsl, Slot3dDsl, SlotEdgeDsl, TileDsl};
use crate::schema::snapshot::TileMedia3d;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
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
        media: semio_framework_value::DslValue,
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
    DragSlots {
        targets: Vec<String>,
        dx: f64,
        dy: f64,
        dz: f64,
    },
    SetSlotPositions {
        ids: Vec<String>,
        xs: Vec<f64>,
        ys: Vec<f64>,
        zs: Vec<f64>,
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
        Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media }) => Wfc3dOperationDsl::ChangeTileMedia { id: id.clone(), media: semio_framework_value::ToValue::to_value(media) },
        Wfc3dMutation::CreateRule(CreateRule { index, rule }) => Wfc3dOperationDsl::CreateRule { index: *index, rule: rule_to_dsl(rule) },
        Wfc3dMutation::DeleteRule(DeleteRule { id }) => Wfc3dOperationDsl::DeleteRule { id: id.clone() },
        Wfc3dMutation::ChangeSeed(ChangeSeed { seed }) => Wfc3dOperationDsl::ChangeSeed { seed: *seed },
        Wfc3dMutation::DragSlots(DragSlots { targets, dx, dy, dz }) => Wfc3dOperationDsl::DragSlots { targets: targets.clone(), dx: *dx, dy: *dy, dz: *dz },
        Wfc3dMutation::SetSlotPositions(SetSlotPositions { positions }) => Wfc3dOperationDsl::SetSlotPositions {
            ids: positions.iter().map(|position| position.id.clone()).collect(),
            xs: positions.iter().map(|position| position.x).collect(),
            ys: positions.iter().map(|position| position.y).collect(),
            zs: positions.iter().map(|position| position.z).collect(),
        },
    }
}

pub fn operation_from_dsl(operation: Wfc3dOperationDsl) -> Result<Wfc3dMutation, semio_framework_diagnostic::TextError> {
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
                semio_framework_value::DslValue::Null => TileMedia3d::default(),
                other => semio_framework_value::FromValue::from_value(other).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid tile media: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
            };
            Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media })
        }
        Wfc3dOperationDsl::CreateRule { index, rule } => Wfc3dMutation::CreateRule(CreateRule { index, rule: rule_from_dsl(rule) }),
        Wfc3dOperationDsl::DeleteRule { id } => Wfc3dMutation::DeleteRule(DeleteRule { id }),
        Wfc3dOperationDsl::ChangeSeed { seed } => Wfc3dMutation::ChangeSeed(ChangeSeed { seed }),
        Wfc3dOperationDsl::DragSlots { targets, dx, dy, dz } => Wfc3dMutation::DragSlots(DragSlots { targets, dx, dy, dz }),
        Wfc3dOperationDsl::SetSlotPositions { ids, xs, ys, zs } => {
            if xs.len() != ids.len() || ys.len() != ids.len() || zs.len() != ids.len() {
                return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "set-slot-positions carries one x, y and z per id", semio_framework_diagnostic::TextSpan::at(1, 1)));
            }
            let positions = ids.into_iter().zip(xs).zip(ys).zip(zs).map(|(((id, x), y), z)| Wfc3dSlotPosition { id, x, y, z }).collect();
            Wfc3dMutation::SetSlotPositions(SetSlotPositions { positions })
        }
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for Wfc3dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown wfc3d mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// ⚡️ `Wfc3dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Wfc3dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        operation_from_dsl(<Wfc3dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Wfc3dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfc3d` mutation line.
pub fn parse_op(line: &str) -> Result<Wfc3dMutation, semio_framework_diagnostic::TextError> {
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

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
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
use crate::standards::v1::subsets::any::schema::mutations::set_slot_positions::{set_slot_positions, Wfc3dSlotPosition};
use crate::standards::v1::subsets::any::schema::mutations::unpin_slot::unpin_slot;

/// 🌉️ The language-neutral report of one committed specification vector — decoded, diffed, applied and inverted
/// through this subset's production JSON codec and `Mutation` implementation — that the `mutate-wfc3d` case's
/// subject half judges with `law::vector`. Its signature names only `str`, so a generated test host reaches it.
/// @see store::os_store::test_support::mutation_report_json
pub fn wfc3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Wfc3dSnapshot, Wfc3dMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_codec::*;
