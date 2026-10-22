//! 📏️ `scale-transforms` payload — one gumball scaling as intent: the per-axis factors every addressed scale operator of
//! the generator graph multiplies into its own. Editing it in history re-derives the operator on any base.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_items;
use semio_s_artifact_procedural_3d_mut_widgets::generation3d_label_number;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️ScaleTransforms
/// 📏️ Multiplies `(sx, sy, sz)` into the factors of every addressed scale operator.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct ScaleTransforms {
    pub targets: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_transforms<Op: From<ScaleTransforms>>(targets: Vec<String>, factors: [f64; 3]) -> Op {
    Op::from(ScaleTransforms { targets, sx: factors[0], sy: factors[1], sz: factors[2] })
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ScaleTransforms
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "transforms", kind: "scale-transforms", record: "ScaledTransforms" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = [self.sx, self.sy, self.sz].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by ({x_en}, {y_en}, {z_en})"), &format!("{de} um ({x_de}; {y_de}; {z_de}) skalieren"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for ScaleTransforms
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::scale_transforms::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::scale_transforms::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ScaleTransforms
