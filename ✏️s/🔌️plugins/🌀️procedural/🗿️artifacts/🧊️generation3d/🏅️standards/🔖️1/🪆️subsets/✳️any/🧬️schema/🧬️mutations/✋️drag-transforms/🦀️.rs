//! ✋️ `drag-transforms` payload — one gumball drag as intent: the offset every addressed translate operator of the
//! generator graph adds to its own offset. The splice that inserts a missing operator rides the same transaction as
//! absolute rows; this leaf carries what the user did, so editing it in history re-derives the operator on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_items,generation3d_label_number,Generation3dMutation};

use crate::Generation3dSnapshot;

//#region 🔖️DragTransforms
/// ✋️ Adds `(dx, dy, dz)` to the offset of every addressed translate operator.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DragTransforms {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_transforms(targets: Vec<String>, offset: [f64; 3]) -> Generation3dMutation {
    Generation3dMutation::DragTransforms(DragTransforms { targets, dx: offset[0], dy: offset[1], dz: offset[2] })
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for DragTransforms {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "transforms", kind: "drag-transforms", record: "DraggedTransforms" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de), (z_en, z_de)] = [self.dx, self.dy, self.dz].map(generation3d_label_number);
        let (en, de) = generation3d_label_items(self.targets.len(), "shape(s)", "Form(en)");
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({x_en}, {y_en}, {z_en})"), &format!("{de} um ({x_de}; {y_de}; {z_de}) ziehen"))
    }

    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️DragTransforms
