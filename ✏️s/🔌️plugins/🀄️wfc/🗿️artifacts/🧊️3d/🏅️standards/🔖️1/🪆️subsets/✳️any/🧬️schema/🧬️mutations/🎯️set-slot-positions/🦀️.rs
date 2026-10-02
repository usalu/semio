//! 🎯️ `wfc3d` mutation — `SetSlotPositions`: puts a set of slots at absolute positions (their MINIMUM corners) in
//! one row. It is the exact undo of a `drag-slots` (every moved slot back at its base position) and of itself, so a
//! multi-slot gesture stays ONE point-invertible row however many slots it moved.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetSlotPositions
/// 📌️ One slot's absolute minimum corner, in document units.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
pub struct Wfc3dSlotPosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSlotPositions {
    pub positions: Vec<Wfc3dSlotPosition>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_slot_positions(positions: Vec<Wfc3dSlotPosition>) -> Wfc3dMutation {
    Wfc3dMutation::SetSlotPositions(SetSlotPositions { positions })
}

impl SetSlotPositions {
    /// 🛂️ Whether the payload is well-formed: at least one position, no slot twice, finite coordinates.
    pub fn holds_invariants(&self) -> bool {
        !self.positions.is_empty()
            && !self.positions.iter().enumerate().any(|(at, position)| self.positions[..at].iter().any(|prior| prior.id == position.id))
            && self.positions.iter().all(|position| [position.x, position.y, position.z].iter().all(|value| value.is_finite()))
    }

    /// 🆔️ The positioned slot ids, in payload order.
    pub fn ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.id.clone()).collect()
    }
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for SetSlotPositions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slot-positions", kind: "set-slot-positions", record: "SetSlotPositions" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.positions.len() {
            1 => semio_framework_ui_locale::LocalizedLabel::native("Set 1 slot position", "1 Slotposition setzen"),
            count => semio_framework_ui_locale::LocalizedLabel::native(&format!("Set {count} slot positions"), &format!("{count} Slotpositionen setzen")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.ids()
    }
}
//#endregion 🔖️SetSlotPositions
