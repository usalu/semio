//! 📌️ Bitmap mutation — `PinPixel`: pre-assigns one output cell to a palette colour. A hard domain
//! restriction the solve must respect, kept in canonical row-major order so the pin list is
//! point-invertible.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinPixel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinPixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_pixel(x: u32, y: u32, color: u32) -> BitmapMutation {
    BitmapMutation::PinPixel(PinPixel { x, y, color })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for PinPixel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "pixel", kind: "pin-pixel", record: "Fixed" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Pin ({}, {}) to colour {}", self.x, self.y, self.color), &format!("({}, {}) an Farbe {} anheften", self.x, self.y, self.color))
    }
    fn target(&self) -> Vec<String> {
        vec![crate::schema::snapshot::pin_key(self.x, self.y)]
    }
}
//#endregion 🔖️PinPixel
