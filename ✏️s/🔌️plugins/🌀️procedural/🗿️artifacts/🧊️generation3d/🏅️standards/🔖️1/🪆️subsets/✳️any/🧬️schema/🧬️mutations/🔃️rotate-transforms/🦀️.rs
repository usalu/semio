//! 🔃️ `rotate-transforms` payload — one gumball rotation as intent: the world-axis rotation every addressed rotate
//! operator of the generator graph composes after its own. Editing it in history re-derives the operator on any base.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::generation3d_label_items;
use crate::generation3d_label_number;
use crate::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️RotateTransforms
/// 🔃️ Composes the rotation by `angle` radians about the world axis `(ax, ay, az)` after every addressed operator's own.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
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
        crate::rotate_transforms::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::rotate_transforms::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = generation3d_label_number(self.angle.to_degrees());
        let (items_en, items_de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {items_en} by {en}°"), &format!("{items_de} um {de}° drehen"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️RotateTransforms
