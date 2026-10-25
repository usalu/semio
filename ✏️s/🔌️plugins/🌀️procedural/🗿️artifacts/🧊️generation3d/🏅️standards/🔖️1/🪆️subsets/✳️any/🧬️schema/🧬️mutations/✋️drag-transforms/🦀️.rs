//! ✋️ `drag-transforms` payload — one gumball drag as intent: the offset every addressed translate operator of the
//! generator graph adds to its own offset. The splice that inserts a missing operator rides the same transaction as
//! absolute rows; this leaf carries what the user did, so editing it in history re-derives the operator on any base.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_items;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_number;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️DragTransforms
/// ✋️ Adds `(dx, dy, dz)` to the offset of every addressed translate operator.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DragTransforms {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_transforms<Op: From<DragTransforms>>(targets: Vec<String>, offset: [f64; 3]) -> Op {
    Op::from(DragTransforms { targets, dx: offset[0], dy: offset[1], dz: offset[2] })
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for DragTransforms
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "transforms", kind: "drag-transforms", record: "DraggedTransforms" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = [self.dx, self.dy, self.dz].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({x_en}, {y_en}, {z_en})"), &format!("{de} um ({x_de}; {y_de}; {z_de}) ziehen"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for DragTransforms
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::drag_transforms::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::drag_transforms::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DragTransforms
