//! ⚡️ `s.wfc.grid2d` — the mutation vocabulary's single-line TEXT opcodes + grammar.
//!
//! One keyword per `Grid2dMutation` variant, in the `KINDS` order the catalog
//! (`../../../🔮️oracles/🔣️.json`) declares. The operation twin exists for the same reason the
//! snapshot's does — `WfcTile2d::media` is a tagged enum carrying an artifact child — and bridges
//! through the snapshot facet's own `tile_to_dsl`/`rule_to_dsl` pairs, never a second conversion.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::change_cell_size::ChangeCellSize;
use crate::schema::mutations::change_periodicity::ChangePeriodicity;
use crate::schema::mutations::change_seed::ChangeSeed;
use crate::schema::mutations::change_tile_media::ChangeTileMedia;
use crate::schema::mutations::change_tile_weight::ChangeTileWeight;
use crate::schema::mutations::create_rule::CreateRule;
use crate::schema::mutations::create_tile::CreateTile;
use crate::schema::mutations::delete_rule::DeleteRule;
use crate::schema::mutations::delete_tile::DeleteTile;
use crate::schema::mutations::mask_cell::MaskCell;
use crate::schema::mutations::pin_cell::PinCell;
use crate::schema::mutations::resize_grid::ResizeGrid;
use crate::schema::mutations::unmask_cell::UnmaskCell;
use crate::schema::mutations::unpin_cell::UnpinCell;
use crate::schema::mutations::Grid2dMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::{rule_from_dsl, rule_to_dsl, tile_from_dsl, tile_to_dsl, WfcAdjacencyRule2dDsl, WfcTile2dDsl};
use crate::schema::snapshot::WfcTileMedia2d;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum Grid2dOperationDsl {
    ChangeSeed {
        seed: u64,
    },
    ResizeGrid {
        width: u32,
        height: u32,
    },
    ChangeCellSize {
        cell_width: f64,
        cell_height: f64,
    },
    ChangePeriodicity {
        periodic_x: bool,
        periodic_y: bool,
    },
    CreateTile {
        #[dsl(block)]
        tile: WfcTile2dDsl,
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
        #[dsl(block)]
        rule: WfcAdjacencyRule2dDsl,
    },
    DeleteRule {
        id: String,
    },
    PinCell {
        x: u32,
        y: u32,
        tile_id: String,
    },
    UnpinCell {
        x: u32,
        y: u32,
    },
    MaskCell {
        x: u32,
        y: u32,
    },
    UnmaskCell {
        x: u32,
        y: u32,
    },
}

pub fn operation_to_dsl(operation: &Grid2dMutation) -> Grid2dOperationDsl {
    match operation {
        Grid2dMutation::ChangeSeed(ChangeSeed { seed }) => Grid2dOperationDsl::ChangeSeed { seed: *seed },
        Grid2dMutation::ResizeGrid(ResizeGrid { width, height }) => Grid2dOperationDsl::ResizeGrid { width: *width, height: *height },
        Grid2dMutation::ChangeCellSize(ChangeCellSize { cell_width, cell_height }) => Grid2dOperationDsl::ChangeCellSize { cell_width: *cell_width, cell_height: *cell_height },
        Grid2dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y }) => Grid2dOperationDsl::ChangePeriodicity { periodic_x: *periodic_x, periodic_y: *periodic_y },
        Grid2dMutation::CreateTile(CreateTile { tile }) => Grid2dOperationDsl::CreateTile { tile: tile_to_dsl(tile) },
        Grid2dMutation::DeleteTile(DeleteTile { id }) => Grid2dOperationDsl::DeleteTile { id: id.clone() },
        Grid2dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }) => Grid2dOperationDsl::ChangeTileWeight { id: id.clone(), weight: *weight },
        Grid2dMutation::ChangeTileMedia(ChangeTileMedia { id, media }) => Grid2dOperationDsl::ChangeTileMedia { id: id.clone(), media: semio_framework_value::ToValue::to_value(media) },
        Grid2dMutation::CreateRule(CreateRule { rule }) => Grid2dOperationDsl::CreateRule { rule: rule_to_dsl(rule) },
        Grid2dMutation::DeleteRule(DeleteRule { id }) => Grid2dOperationDsl::DeleteRule { id: id.clone() },
        Grid2dMutation::PinCell(PinCell { x, y, tile_id }) => Grid2dOperationDsl::PinCell { x: *x, y: *y, tile_id: tile_id.clone() },
        Grid2dMutation::UnpinCell(UnpinCell { x, y }) => Grid2dOperationDsl::UnpinCell { x: *x, y: *y },
        Grid2dMutation::MaskCell(MaskCell { x, y }) => Grid2dOperationDsl::MaskCell { x: *x, y: *y },
        Grid2dMutation::UnmaskCell(UnmaskCell { x, y }) => Grid2dOperationDsl::UnmaskCell { x: *x, y: *y },
    }
}

pub fn operation_from_dsl(operation: Grid2dOperationDsl) -> Result<Grid2dMutation, semio_framework_diagnostic::TextError> {
    Ok(match operation {
        Grid2dOperationDsl::ChangeSeed { seed } => Grid2dMutation::ChangeSeed(ChangeSeed { seed }),
        Grid2dOperationDsl::ResizeGrid { width, height } => Grid2dMutation::ResizeGrid(ResizeGrid { width, height }),
        Grid2dOperationDsl::ChangeCellSize { cell_width, cell_height } => Grid2dMutation::ChangeCellSize(ChangeCellSize { cell_width, cell_height }),
        Grid2dOperationDsl::ChangePeriodicity { periodic_x, periodic_y } => Grid2dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y }),
        Grid2dOperationDsl::CreateTile { tile } => Grid2dMutation::CreateTile(CreateTile { tile: tile_from_dsl(tile)? }),
        Grid2dOperationDsl::DeleteTile { id } => Grid2dMutation::DeleteTile(DeleteTile { id }),
        Grid2dOperationDsl::ChangeTileWeight { id, weight } => Grid2dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }),
        Grid2dOperationDsl::ChangeTileMedia { id, media } => {
            let media: WfcTileMedia2d = match media {
                semio_framework_value::DslValue::Null => WfcTileMedia2d::default(),
                other => semio_framework_value::FromValue::from_value(other).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("invalid tile media: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?,
            };
            Grid2dMutation::ChangeTileMedia(ChangeTileMedia { id, media })
        }
        Grid2dOperationDsl::CreateRule { rule } => Grid2dMutation::CreateRule(CreateRule { rule: rule_from_dsl(rule)? }),
        Grid2dOperationDsl::DeleteRule { id } => Grid2dMutation::DeleteRule(DeleteRule { id }),
        Grid2dOperationDsl::PinCell { x, y, tile_id } => Grid2dMutation::PinCell(PinCell { x, y, tile_id }),
        Grid2dOperationDsl::UnpinCell { x, y } => Grid2dMutation::UnpinCell(UnpinCell { x, y }),
        Grid2dOperationDsl::MaskCell { x, y } => Grid2dMutation::MaskCell(MaskCell { x, y }),
        Grid2dOperationDsl::UnmaskCell { x, y } => Grid2dMutation::UnmaskCell(UnmaskCell { x, y }),
    })
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for Grid2dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown grid2d mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// ⚡️ `Grid2dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Grid2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        operation_from_dsl(<Grid2dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Grid2dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfcgrid2d` mutation line.
pub fn parse_op(line: &str) -> Result<Grid2dMutation, semio_framework_diagnostic::TextError> {
    <Grid2dMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `Grid2dMutation` back to its single-line form.
pub fn print_op(operation: &Grid2dMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type Grid2dMutationText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::Grid2dDiff;
use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcTile2d};
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::change_cell_size::change_cell_size;
use crate::standards::v1::subsets::any::schema::mutations::change_periodicity::change_periodicity;
use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_media::change_tile_media;
use crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::change_tile_weight;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::create_rule;
use crate::standards::v1::subsets::any::schema::mutations::create_tile::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::delete_rule::delete_rule;
use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::standards::v1::subsets::any::schema::mutations::mask_cell::mask_cell;
use crate::standards::v1::subsets::any::schema::mutations::pin_cell::pin_cell;
use crate::standards::v1::subsets::any::schema::mutations::resize_grid::resize_grid;
use crate::standards::v1::subsets::any::schema::mutations::unmask_cell::unmask_cell;
use crate::standards::v1::subsets::any::schema::mutations::unpin_cell::unpin_cell;

/// 🌉️ The language-neutral report of one committed specification vector — decoded, diffed, applied and inverted
/// through this subset's production JSON codec and `Mutation` implementation — that the `mutate-grid2d` case's
/// subject half judges with `law::vector`. Its signature names only `str`, so a generated test host reaches it.
/// @see store::os_store::test_support::mutation_report_json
pub fn grid2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<Grid2dSnapshot, Grid2dMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_codec::*;
