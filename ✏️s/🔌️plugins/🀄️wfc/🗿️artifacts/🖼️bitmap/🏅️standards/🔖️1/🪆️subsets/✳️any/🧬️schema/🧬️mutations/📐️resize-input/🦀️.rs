//! 📐️ Bitmap mutation — `ResizeInput`: changes the authored sample bitmap's extent. Growing pads
//! the new right/bottom margin with palette index `0`; shrinking drops the pixels outside the new
//! extent, which is why the inverse is a resize PLUS a full-buffer restore rather than a resize
//! alone.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeInput
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeInput {
    pub width: u32,
    pub height: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_input(width: u32, height: u32) -> BitmapMutation {
    BitmapMutation::ResizeInput(ResizeInput { width, height })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for ResizeInput {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "input", kind: "resize-input", record: "ResizedInput" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Resize input to {}×{}", self.width, self.height), &format!("Größe der Eingabe auf {}×{} ändern", self.width, self.height))
    }
}
//#endregion 🔖️ResizeInput
