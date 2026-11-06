//! ↩️ Restores the exact prior pin sequence as one inverse event.
use crate::diff::BitmapDiff;
use crate::schema::snapshot::{BitmapPinnedPixel, BitmapSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RestorePins {
    pub pins: Vec<BitmapPinnedPixel>,
}

/// 🏗️ Wraps the complete pin payload in its canonical event.
pub fn restore_pins<Op>(pins: Vec<BitmapPinnedPixel>) -> Op
where
    Op: From<RestorePins>,
{
    Op::from(RestorePins { pins })
}

impl<Op> MutationKind<BitmapSnapshot, Op> for RestorePins
where
    Op: protocol::Mutation<BitmapSnapshot, Diff = BitmapDiff> + From<RestorePins> + From<crate::standards::v1::subsets::any::schema::mutations::restore_pins::RestorePins>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "restore", entity: "pins", kind: "restore-pins", record: "PinsRestored" };
    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Restore Pins", "Anheftungen wiederherstellen")
    }
    fn target(&self) -> Vec<String> {
        vec!["pinned".into()]
    }
}

impl<Op> protocol::ColdMutationKind<BitmapSnapshot, Op> for RestorePins
where
    Op: protocol::ColdMutation<BitmapSnapshot, ReferenceDiff = BitmapDiff> + From<RestorePins> + From<crate::standards::v1::subsets::any::schema::mutations::restore_pins::RestorePins>,
{
    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
