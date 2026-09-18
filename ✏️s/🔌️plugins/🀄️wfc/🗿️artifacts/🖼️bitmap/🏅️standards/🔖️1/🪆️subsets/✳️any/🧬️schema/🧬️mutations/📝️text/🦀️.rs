//! ⚡️ Bitmap artifact — the mutation vocabulary's single-line TEXT opcodes + grammar.
//!
//! One keyword per `BitmapMutation` variant, in the `KINDS` order the catalog
//! (`../../../🔮️oracles/🔣️.json`) declares. The operation twin exists for the same reason the
//! snapshot's does: the schema tree states WHAT a mutation is, this facet states how it is spelled.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::mutations::add_palette_color::AddPaletteColor;
use crate::schema::mutations::change_model::ChangeModel;
use crate::schema::mutations::change_palette_color::ChangePaletteColor;
use crate::schema::mutations::change_seed::ChangeSeed;
use crate::schema::mutations::pin_pixel::PinPixel;
use crate::schema::mutations::remove_palette_color::RemovePaletteColor;
use crate::schema::mutations::resize_input::ResizeInput;
use crate::schema::mutations::resize_output::ResizeOutput;
use crate::schema::mutations::set_input_pixels::SetInputPixels;
use crate::schema::mutations::unpin_pixel::UnpinPixel;
use crate::schema::mutations::BitmapMutation;
use crate::schema::snapshot::text::{color_from_dsl, color_to_dsl, BitmapColorDsl};

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
pub enum BitmapOperationDsl {
    ChangeSeed {
        seed: u64,
    },
    ResizeInput {
        width: u32,
        height: u32,
    },
    SetInputPixels {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        pixels: String,
    },
    AddPaletteColor {
        index: usize,
        #[dsl(block)]
        color: BitmapColorDsl,
    },
    ChangePaletteColor {
        index: usize,
        #[dsl(block)]
        color: BitmapColorDsl,
    },
    RemovePaletteColor {
        index: usize,
    },
    ResizeOutput {
        width: u32,
        height: u32,
        periodic: bool,
    },
    ChangeModel {
        pattern_size: u32,
        symmetry: u32,
        periodic_input: bool,
        ground: Option<u32>,
    },
    PinPixel {
        x: u32,
        y: u32,
        color: u32,
    },
    UnpinPixel {
        x: u32,
        y: u32,
    },
}

pub fn operation_to_dsl(operation: &BitmapMutation) -> BitmapOperationDsl {
    match operation {
        BitmapMutation::ChangeSeed(ChangeSeed { seed }) => BitmapOperationDsl::ChangeSeed { seed: *seed },
        BitmapMutation::ResizeInput(ResizeInput { width, height }) => BitmapOperationDsl::ResizeInput { width: *width, height: *height },
        BitmapMutation::SetInputPixels(SetInputPixels { x, y, width, height, pixels }) => BitmapOperationDsl::SetInputPixels { x: *x, y: *y, width: *width, height: *height, pixels: pixels.clone() },
        BitmapMutation::AddPaletteColor(AddPaletteColor { index, color }) => BitmapOperationDsl::AddPaletteColor { index: *index, color: color_to_dsl(color) },
        BitmapMutation::ChangePaletteColor(ChangePaletteColor { index, color }) => BitmapOperationDsl::ChangePaletteColor { index: *index, color: color_to_dsl(color) },
        BitmapMutation::RemovePaletteColor(RemovePaletteColor { index }) => BitmapOperationDsl::RemovePaletteColor { index: *index },
        BitmapMutation::ResizeOutput(ResizeOutput { width, height, periodic }) => BitmapOperationDsl::ResizeOutput { width: *width, height: *height, periodic: *periodic },
        BitmapMutation::ChangeModel(ChangeModel { pattern_size, symmetry, periodic_input, ground }) => BitmapOperationDsl::ChangeModel { pattern_size: *pattern_size, symmetry: *symmetry, periodic_input: *periodic_input, ground: *ground },
        BitmapMutation::PinPixel(PinPixel { x, y, color }) => BitmapOperationDsl::PinPixel { x: *x, y: *y, color: *color },
        BitmapMutation::UnpinPixel(UnpinPixel { x, y }) => BitmapOperationDsl::UnpinPixel { x: *x, y: *y },
    }
}

pub fn operation_from_dsl(operation: BitmapOperationDsl) -> BitmapMutation {
    match operation {
        BitmapOperationDsl::ChangeSeed { seed } => BitmapMutation::ChangeSeed(ChangeSeed { seed }),
        BitmapOperationDsl::ResizeInput { width, height } => BitmapMutation::ResizeInput(ResizeInput { width, height }),
        BitmapOperationDsl::SetInputPixels { x, y, width, height, pixels } => BitmapMutation::SetInputPixels(SetInputPixels { x, y, width, height, pixels }),
        BitmapOperationDsl::AddPaletteColor { index, color } => BitmapMutation::AddPaletteColor(AddPaletteColor { index, color: color_from_dsl(&color) }),
        BitmapOperationDsl::ChangePaletteColor { index, color } => BitmapMutation::ChangePaletteColor(ChangePaletteColor { index, color: color_from_dsl(&color) }),
        BitmapOperationDsl::RemovePaletteColor { index } => BitmapMutation::RemovePaletteColor(RemovePaletteColor { index }),
        BitmapOperationDsl::ResizeOutput { width, height, periodic } => BitmapMutation::ResizeOutput(ResizeOutput { width, height, periodic }),
        BitmapOperationDsl::ChangeModel { pattern_size, symmetry, periodic_input, ground } => BitmapMutation::ChangeModel(ChangeModel { pattern_size, symmetry, periodic_input, ground }),
        BitmapOperationDsl::PinPixel { x, y, color } => BitmapMutation::PinPixel(PinPixel { x, y, color }),
        BitmapOperationDsl::UnpinPixel { x, y } => BitmapMutation::UnpinPixel(UnpinPixel { x, y }),
    }
}
//#endregion 🔖️OpTextMirror

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ Handcrafted `OpText` — `dsl::DslOps`/`dsl::DslEnum` emit `DslVariants` only.
impl protocol::OpText for BitmapOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown bitmap mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ `BitmapMutation`'s compact single-line op encoding, bridged through the twin above.
impl protocol::OpText for BitmapMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(operation_from_dsl(<BitmapOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <BitmapOperationDsl as protocol::OpText>::print_op(&operation_to_dsl(self))
    }
}
//#endregion 🔖️HandcraftedOpCodecs

/// 📖️ Parses one `.wfcbitmap` mutation line.
pub fn parse_op(line: &str) -> Result<BitmapMutation, store::TextError> {
    <BitmapMutation as protocol::OpText>::parse_op(line)
}

/// 🖨️ Prints one `BitmapMutation` back to its single-line form.
pub fn print_op(operation: &BitmapMutation) -> String {
    protocol::OpText::print_op(operation)
}

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_op`/`print_op` speak.
pub type BitmapMutationText = String;
//#endregion 🚚️Carrier
