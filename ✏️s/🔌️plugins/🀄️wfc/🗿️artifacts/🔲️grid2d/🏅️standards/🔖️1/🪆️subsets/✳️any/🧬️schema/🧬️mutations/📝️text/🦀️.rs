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
use crate::schema::snapshot::text::{rule_from_dsl, rule_to_dsl, tile_from_dsl, tile_to_dsl, WfcAdjacencyRule2dDsl, WfcTile2dDsl};
use crate::schema::snapshot::WfcTileMedia2d;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
        media: dsl::DslValue,
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
        Grid2dMutation::ChangeTileMedia(ChangeTileMedia { id, media }) => Grid2dOperationDsl::ChangeTileMedia { id: id.clone(), media: dsl::to_dsl_value(media).unwrap_or(dsl::DslValue::Null) },
        Grid2dMutation::CreateRule(CreateRule { rule }) => Grid2dOperationDsl::CreateRule { rule: rule_to_dsl(rule) },
        Grid2dMutation::DeleteRule(DeleteRule { id }) => Grid2dOperationDsl::DeleteRule { id: id.clone() },
        Grid2dMutation::PinCell(PinCell { x, y, tile_id }) => Grid2dOperationDsl::PinCell { x: *x, y: *y, tile_id: tile_id.clone() },
        Grid2dMutation::UnpinCell(UnpinCell { x, y }) => Grid2dOperationDsl::UnpinCell { x: *x, y: *y },
        Grid2dMutation::MaskCell(MaskCell { x, y }) => Grid2dOperationDsl::MaskCell { x: *x, y: *y },
        Grid2dMutation::UnmaskCell(UnmaskCell { x, y }) => Grid2dOperationDsl::UnmaskCell { x: *x, y: *y },
    }
}

pub fn operation_from_dsl(operation: Grid2dOperationDsl) -> Result<Grid2dMutation, store::TextError> {
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
                dsl::DslValue::Null => WfcTileMedia2d::default(),
                other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid tile media: {error}"), store::TextSpan::at(1, 1)))?,
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown grid2d mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `Grid2dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Grid2dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        operation_from_dsl(<Grid2dOperationDsl as protocol::OpText>::parse_op(line)?)
    }

    fn print_op(&self) -> String {
        <Grid2dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfcgrid2d` mutation line.
pub fn parse_op(line: &str) -> Result<Grid2dMutation, store::TextError> {
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
