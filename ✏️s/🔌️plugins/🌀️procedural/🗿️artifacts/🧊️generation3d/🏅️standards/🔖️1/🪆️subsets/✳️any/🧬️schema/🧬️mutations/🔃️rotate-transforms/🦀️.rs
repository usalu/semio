//! 🔃️ `rotate-transforms` payload — one gumball rotation as intent: the world-axis rotation every addressed rotate
//! operator of the generator graph composes after its own. Editing it in history re-derives the operator on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_items, generation3d_label_number, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️RotateTransforms
/// 🔃️ Composes the rotation by `angle` radians about the world axis `(ax, ay, az)` after every addressed operator's own.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RotateTransforms {
    pub targets: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rotate_transforms(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Generation3dMutation {
    Generation3dMutation::RotateTransforms(RotateTransforms { targets, ax: axis[0], ay: axis[1], az: axis[2], angle })
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for RotateTransforms {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "transforms", kind: "rotate-transforms", record: "RotatedTransforms" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> protocol::LocalizedLabel {
        let (en, de) = generation3d_label_number(self.angle.to_degrees());
        let (items_en, items_de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        protocol::LocalizedLabel::native(&format!("Rotate {items_en} by {en}°"), &format!("{items_de} um {de}° drehen"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️RotateTransforms
