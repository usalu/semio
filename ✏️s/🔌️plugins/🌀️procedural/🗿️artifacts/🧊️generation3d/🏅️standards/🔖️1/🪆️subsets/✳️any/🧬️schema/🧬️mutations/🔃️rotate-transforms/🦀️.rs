//! 🔃️ `rotate-transforms` payload — one gumball rotation as intent: the world-axis rotation every addressed rotate
//! operator of the generator graph composes after its own. Editing it in history re-derives the operator on any base.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_items;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_number;

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
pub fn rotate_transforms<Op: From<RotateTransforms>>(targets: Vec<String>, axis: [f64; 3], angle: f64) -> Op {
    Op::from(RotateTransforms { targets, ax: axis[0], ay: axis[1], az: axis[2], angle })
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for RotateTransforms
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "transforms", kind: "rotate-transforms", record: "RotatedTransforms" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = generation3d_label_number(self.angle.to_degrees());
        let (items_en, items_de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {items_en} by {en}°"), &format!("{items_de} um {de}° drehen"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for RotateTransforms
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::rotate_transforms::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::rotate_transforms::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️RotateTransforms
