//! 🎯️ WFC 2D mutation — `SetSlotPositions`: puts a set of slots at absolute positions in one row. It is the exact
//! undo of a `drag-slots` (every moved slot back at its base position) and of itself, so a multi-slot gesture stays
//! ONE point-invertible row however many slots it moved.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️SetSlotPositions
/// 📌️ One slot's absolute lower corner, in document units.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dSlotPosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSlotPositions {
    pub positions: Vec<Wfc2dSlotPosition>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_slot_positions(positions: Vec<Wfc2dSlotPosition>) -> Wfc2dMutation {
    Wfc2dMutation::SetSlotPositions(SetSlotPositions { positions })
}

impl SetSlotPositions {
    /// 🛂️ Whether the payload is well-formed: at least one position, no slot twice, finite coordinates.
    pub fn holds_invariants(&self) -> bool {
        !self.positions.is_empty()
            && !self.positions.iter().enumerate().any(|(at, position)| self.positions[..at].iter().any(|prior| prior.id == position.id))
            && self.positions.iter().all(|position| position.x.is_finite() && position.y.is_finite())
    }

    /// 🆔️ The positioned slot ids, in payload order.
    pub fn ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.id.clone()).collect()
    }
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for SetSlotPositions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slot-positions", kind: "set-slot-positions", record: "SetSlotPositions" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Wfc2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
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
