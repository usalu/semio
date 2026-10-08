//! 📓 `update-spread` — renames a spread.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutSpreadsDelta, LayoutSpreadsModification, SpreadPatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateSpread {
    pub id: String,
    pub name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateSpread {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "spread", kind: "update-spread", record: "UpdatedSpread" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_spread(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_update_spread(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update spread \"{}\"", self.name), &format!("Druckbogen \"{}\" aktualisieren", self.name)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_update_spread(payload: &UpdateSpread, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(spread) = base.spreads.iter().find(|spread| spread.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Spread \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A spread needs a name.", std::iter::empty::<String>());
    }
    if spread.name == payload.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Spread is already named that.");
    }
    protocol::MutationOutcome::new(LayoutDiff { spreads: Some(LayoutSpreadsDelta { modified: vec![LayoutSpreadsModification { id: payload.id.clone(), patch: SpreadPatch { name: Some(payload.name.clone()) } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_update_spread(payload: &UpdateSpread, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(spread) = base.spreads.iter().find(|spread| spread.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::UpdateSpread(UpdateSpread { id: spread.id.clone(), name: spread.name.clone() })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
