//! 🪑️ The commands of the family browser: `placeComponent` selects a placeable family in the framework `library` domain and arms the component utility in the addressed window (a panel addresses none, then the
//! selection alone tells the tool which family to place), `openFamily` selects the family and opens the family editor, `searchFamilies` selects the placeable families a text matches. Selection and windows are framework
//! state, so none of them writes a mutation.

use crate::editor::bim::entities::components::{matching, placeable};
use crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::modes::edit::windows::family;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🪑️ The utility that places components.
pub const COMPONENT_UTILITY: &str = "component";

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "place-component")]
pub struct PlaceComponent {
    pub family: String,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "open-family")]
pub struct OpenFamily {
    pub family: String,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "search-families")]
pub struct SearchFamilies {
    pub query: String,
    pub category: String,
}

fn select_family(id: &str) -> Effect {
    select_effect(BIM_LIBRARY_DOMAIN, &[("family".to_string(), id.to_string())], "replace")
}

fn available(snapshot: &ModelSnapshot, family: &str) -> Result<(), Fault> {
    placeable(snapshot).iter().any(|(id, _)| id == family).then_some(()).ok_or_else(|| fault("bim.place.family-unavailable", format!("'{family}' is no family that can be placed")))
}

/// 🪑️ A command of the family browser: what it asks of the document and the addressed window.
pub trait Browse {
    fn browse(&self, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault>;
}

/// 🪑️ Selects the family and arms the component utility in the addressed window.
impl Browse for PlaceComponent {
    fn browse(&self, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        available(snapshot, &self.family)?;
        let mut emit = Emit::default();
        emit.effects.push(select_family(&self.family));
        if let Some(window) = ctx.view.as_ref().and_then(|view| view.window_id.clone()) {
            emit.effects.push(Effect::SetActiveUtility { window_id: window, utility_id: COMPONENT_UTILITY.to_string() });
        }
        Ok(emit)
    }
}

/// 🧬️ Selects the family and opens the family editor.
impl Browse for OpenFamily {
    fn browse(&self, snapshot: &ModelSnapshot, _ctx: &BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        if !snapshot.families.contains_key(&self.family) {
            return Err(fault("bim.family.missing", format!("the family '{}' does not exist", self.family)));
        }
        let mut emit = Emit::default();
        emit.effects.push(select_family(&self.family));
        emit.effects.push(Effect::OpenWindow { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), kind: semio_framework::kernel::WindowKindId(family::WINDOW_KIND_ID.to_string()), params: semio_framework_plugin::DslValue::Null });
        Ok(emit)
    }
}

/// 🔍️ Selects the placeable families the text matches; an empty text clears the selection, a text that matches nothing is refused.
impl Browse for SearchFamilies {
    fn browse(&self, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        let mut emit = Emit::default();
        if self.query.trim().is_empty() {
            emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &[], "replace"));
            return Ok(emit);
        }
        let category = self.category.trim().to_lowercase();
        let found: Vec<String> = matching(snapshot, &self.query, ctx.labels()).into_iter().filter(|id| category.is_empty() || snapshot.families.get(id).is_some_and(|family| format!("{:?}", family.category).to_lowercase() == category)).collect();
        if found.is_empty() {
            return Err(fault("bim.browser.no-match", format!("no placeable family matches '{}'", self.query.trim())));
        }
        let targets: Vec<(String, String)> = found.into_iter().map(|id| ("family".to_string(), id)).collect();
        emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &targets, "replace"));
        Ok(emit)
    }
}

pub fn handle<P: Browse>(payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    payload.browse(doc.snapshot, ctx)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
