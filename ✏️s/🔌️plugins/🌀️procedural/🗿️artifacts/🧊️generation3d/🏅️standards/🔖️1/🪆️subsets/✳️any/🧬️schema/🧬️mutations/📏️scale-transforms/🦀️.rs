//! 📏️ `scale-transforms` payload — one gumball scaling as intent: the per-axis factors every addressed scale operator of
//! the generator graph multiplies into its own. Editing it in history re-derives the operator on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_items, generation3d_label_number, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ScaleTransforms
/// 📏️ Multiplies `(sx, sy, sz)` into the factors of every addressed scale operator.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ScaleTransforms {
    pub targets: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_transforms(targets: Vec<String>, factors: [f64; 3]) -> Generation3dMutation {
    Generation3dMutation::ScaleTransforms(ScaleTransforms { targets, sx: factors[0], sy: factors[1], sz: factors[2] })
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ScaleTransforms {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "transforms", kind: "scale-transforms", record: "ScaledTransforms" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = [self.sx, self.sy, self.sz].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {en} by ({x_en}, {y_en}, {z_en})"), &format!("{de} um ({x_de}; {y_de}; {z_de}) skalieren"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️ScaleTransforms
