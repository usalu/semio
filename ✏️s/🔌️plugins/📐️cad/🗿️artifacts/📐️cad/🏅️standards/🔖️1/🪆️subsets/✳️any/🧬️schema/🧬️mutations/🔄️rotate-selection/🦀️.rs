//! 🔄️ CAD mutation — `RotateSelection`: a relative, parametric turn of one pane's objects, each in place about one world
//! axis by one angle. Editing the turn in history re-derives every orientation from whatever base it replays on. The
//! gumball rotate and the `transform.rotate` interaction yield it.

use crate::diff::CadDiff;
use crate::mutations::{cad_selection_items, cad_selection_number, CadMutation};
use crate::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔄️ `rotate-selection` payload — the pane, the objects it turns, the world axis and the right-handed angle (radians).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rotate-selection")]
pub struct RotateSelection {
    pub pane: CadPaneId,
    pub targets: Vec<String>,
    pub axis: [f64; 3],
    pub angle: f64,
}

impl RotateSelection {
    /// 🧭️ The unit `(x, y, z, w)` quaternion this turn composes onto every object — `None` for a non-finite payload or
    /// a zero-length axis (the declared `axis-nonzero` invariant).
    pub fn delta(&self) -> Option<[f64; 4]> {
        let length = (self.axis[0] * self.axis[0] + self.axis[1] * self.axis[1] + self.axis[2] * self.axis[2]).sqrt();
        if !length.is_finite() || length == 0.0 || !self.angle.is_finite() {
            return None;
        }
        let (sin, cos) = (self.angle * 0.5).sin_cos();
        Some([self.axis[0] / length * sin, self.axis[1] / length * sin, self.axis[2] / length * sin, cos])
    }
}

impl MutationKind<CadSnapshot, CadMutation> for RotateSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "selection", kind: "rotate-selection", record: "RotatedSelection" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (items_en, items_de) = cad_selection_items(self.targets.len());
        let (degrees_en, degrees_de) = cad_selection_number(self.angle.to_degrees());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {items_en} by {degrees_en}°"), &format!("{items_de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
