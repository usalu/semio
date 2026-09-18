//! ⚡️ `s.wfc.grid3d` — the mutation vocabulary's single-line TEXT opcodes + grammar. One keyword per
//! `Grid3dMutation` variant, in the `KINDS` order the oracle catalog declares.
//!
//! The dispatch enum derives `dsl::Mutations` (one unnamed field per variant), which emits no
//! keyworded record, so this leaf carries a flat `Grid3dOperationDsl` mirror converted at the
//! `OpText`/`OpBinary` boundary only — every field is a local type that already binds
//! `dsl::DslField`, so no twin RECORD is needed, only a twin ENUM.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::change_cell_sizes::ChangeCellSizes;
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
use crate::schema::mutations::Grid3dMutation;
use crate::schema::snapshot::{Grid3dAxis, Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dTile, Grid3dTileMedia};

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum Grid3dOperationDsl {
    ChangeSeed {
        seed: u64,
    },
    ResizeGrid {
        width: u32,
        height: u32,
        depth: u32,
    },
    ChangeCellSizes {
        axis: Grid3dAxis,
        sizes: Vec<f64>,
    },
    ChangePeriodicity {
        periodic_x: bool,
        periodic_y: bool,
        periodic_z: bool,
    },
    CreateTile {
        #[dsl(block)]
        tile: Grid3dTile,
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
        #[dsl(statements, block)]
        media: Grid3dTileMedia,
    },
    CreateRule {
        #[dsl(block)]
        rule: Grid3dRule,
    },
    DeleteRule {
        id: String,
    },
    PinCell {
        #[dsl(block)]
        pinned: Grid3dPinnedCell,
    },
    UnpinCell {
        x: u32,
        y: u32,
        z: u32,
    },
    MaskCell {
        #[dsl(block)]
        cell: Grid3dCell,
    },
    UnmaskCell {
        x: u32,
        y: u32,
        z: u32,
    },
}

pub fn operation_to_dsl(operation: &Grid3dMutation) -> Grid3dOperationDsl {
    match operation {
        Grid3dMutation::ChangeSeed(ChangeSeed { seed }) => Grid3dOperationDsl::ChangeSeed { seed: *seed },
        Grid3dMutation::ResizeGrid(ResizeGrid { width, height, depth }) => Grid3dOperationDsl::ResizeGrid { width: *width, height: *height, depth: *depth },
        Grid3dMutation::ChangeCellSizes(ChangeCellSizes { axis, sizes }) => Grid3dOperationDsl::ChangeCellSizes { axis: *axis, sizes: sizes.clone() },
        Grid3dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y, periodic_z }) => Grid3dOperationDsl::ChangePeriodicity { periodic_x: *periodic_x, periodic_y: *periodic_y, periodic_z: *periodic_z },
        Grid3dMutation::CreateTile(CreateTile { tile }) => Grid3dOperationDsl::CreateTile { tile: tile.clone() },
        Grid3dMutation::DeleteTile(DeleteTile { id }) => Grid3dOperationDsl::DeleteTile { id: id.clone() },
        Grid3dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight }) => Grid3dOperationDsl::ChangeTileWeight { tile_id: tile_id.clone(), weight: *weight },
        Grid3dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media }) => Grid3dOperationDsl::ChangeTileMedia { tile_id: tile_id.clone(), media: media.clone() },
        Grid3dMutation::CreateRule(CreateRule { rule }) => Grid3dOperationDsl::CreateRule { rule: rule.clone() },
        Grid3dMutation::DeleteRule(DeleteRule { id }) => Grid3dOperationDsl::DeleteRule { id: id.clone() },
        Grid3dMutation::PinCell(PinCell { pinned }) => Grid3dOperationDsl::PinCell { pinned: pinned.clone() },
        Grid3dMutation::UnpinCell(UnpinCell { x, y, z }) => Grid3dOperationDsl::UnpinCell { x: *x, y: *y, z: *z },
        Grid3dMutation::MaskCell(MaskCell { cell }) => Grid3dOperationDsl::MaskCell { cell: *cell },
        Grid3dMutation::UnmaskCell(UnmaskCell { x, y, z }) => Grid3dOperationDsl::UnmaskCell { x: *x, y: *y, z: *z },
    }
}

pub fn operation_from_dsl(operation: Grid3dOperationDsl) -> Grid3dMutation {
    match operation {
        Grid3dOperationDsl::ChangeSeed { seed } => Grid3dMutation::ChangeSeed(ChangeSeed { seed }),
        Grid3dOperationDsl::ResizeGrid { width, height, depth } => Grid3dMutation::ResizeGrid(ResizeGrid { width, height, depth }),
        Grid3dOperationDsl::ChangeCellSizes { axis, sizes } => Grid3dMutation::ChangeCellSizes(ChangeCellSizes { axis, sizes }),
        Grid3dOperationDsl::ChangePeriodicity { periodic_x, periodic_y, periodic_z } => Grid3dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y, periodic_z }),
        Grid3dOperationDsl::CreateTile { tile } => Grid3dMutation::CreateTile(CreateTile { tile }),
        Grid3dOperationDsl::DeleteTile { id } => Grid3dMutation::DeleteTile(DeleteTile { id }),
        Grid3dOperationDsl::ChangeTileWeight { tile_id, weight } => Grid3dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight }),
        Grid3dOperationDsl::ChangeTileMedia { tile_id, media } => Grid3dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media }),
        Grid3dOperationDsl::CreateRule { rule } => Grid3dMutation::CreateRule(CreateRule { rule }),
        Grid3dOperationDsl::DeleteRule { id } => Grid3dMutation::DeleteRule(DeleteRule { id }),
        Grid3dOperationDsl::PinCell { pinned } => Grid3dMutation::PinCell(PinCell { pinned }),
        Grid3dOperationDsl::UnpinCell { x, y, z } => Grid3dMutation::UnpinCell(UnpinCell { x, y, z }),
        Grid3dOperationDsl::MaskCell { cell } => Grid3dMutation::MaskCell(MaskCell { cell }),
        Grid3dOperationDsl::UnmaskCell { x, y, z } => Grid3dMutation::UnmaskCell(UnmaskCell { x, y, z }),
    }
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslEnum` emits `DslVariants` only (P6).
impl protocol::OpText for Grid3dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown wfc grid3d mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `Grid3dMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for Grid3dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(operation_from_dsl(<Grid3dOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Grid3dOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfcgrid3d` mutation line.
pub fn parse_op(line: &str) -> Result<Grid3dMutation, store::TextError> {
    <Grid3dMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `Grid3dMutation` back to its single-line form.
pub fn print_op(operation: &Grid3dMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type Grid3dMutationText = String;
//#endregion 🚚️Carrier
