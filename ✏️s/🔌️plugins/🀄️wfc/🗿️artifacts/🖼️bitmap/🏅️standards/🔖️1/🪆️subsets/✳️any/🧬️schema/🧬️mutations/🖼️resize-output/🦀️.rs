//! 🖼️ Bitmap mutation — `ResizeOutput`: the extent and periodicity of the bitmap the solve must
//! produce. Pins outside the new extent cascade away, and the inverse re-pins them, because a pin
//! is authored state the solve reads and a resize must not quietly strand it.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ResizeOutput
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ResizeOutput {
    pub width: u32,
    pub height: u32,
    pub periodic: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_output(width: u32, height: u32, periodic: bool) -> BitmapMutation {
    BitmapMutation::ResizeOutput(ResizeOutput { width, height, periodic })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for ResizeOutput {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "output", kind: "resize-output", record: "ResizedOutput" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize output to {}×{}", self.width, self.height)
    }
}
//#endregion 🔖️ResizeOutput
