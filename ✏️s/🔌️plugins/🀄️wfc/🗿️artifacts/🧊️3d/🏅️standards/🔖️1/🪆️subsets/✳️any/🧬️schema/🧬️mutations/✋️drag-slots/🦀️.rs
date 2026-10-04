//! ✋️ `wfc3d` mutation — `DragSlots`: a relative, parametric drag of a set of slots by one common offset, in
//! document units. The gesture's own inputs (which slots, which offset) are the payload, so editing the drag in
//! history re-derives every position from whatever base it replays on. The editor's node-drag tool yields it, one
//! per released drag, with `dz = 0` (the graph canvas is a plan; design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §13.3); its exact undo is
//! one absolute `set-slot-positions`.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DragSlots
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DragSlots {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_slots(targets: Vec<String>, dx: f64, dy: f64, dz: f64) -> Wfc3dMutation {
    Wfc3dMutation::DragSlots(DragSlots { targets, dx, dy, dz })
}

impl DragSlots {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, a finite offset.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && [self.dx, self.dy, self.dz].iter().all(|value| value.is_finite())
    }

    /// 🧘️ Whether the offset moves nothing.
    pub fn is_zero(&self) -> bool {
        (self.dx, self.dy, self.dz) == (0.0, 0.0, 0.0)
    }
}

/// 🔢️ A coordinate as a label shows it: two decimals at most, trailing zeros dropped, `(en, de)`.
pub fn wfc3d_offset_text(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for DragSlots {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "slots", kind: "drag-slots", record: "DraggedSlots" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de), (dz_en, dz_de)) = (wfc3d_offset_text(self.dx), wfc3d_offset_text(self.dy), wfc3d_offset_text(self.dz));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 slot".to_string(), "1 Slot".to_string()),
            count => (format!("{count} slots"), format!("{count} Slots")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en}, {dz_en})"), &format!("{items_de} um ({dx_de}; {dy_de}; {dz_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️DragSlots
