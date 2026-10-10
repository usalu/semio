//! 🌡️ `applyConditions` and `clearConditions`: copy the thermal conditions of one space to the other given or selected spaces, and remove them. Copying states the whole record of the source on each target through
//! one ordinary `set-space-conditions` (a field the source leaves unstated is cleared on the target, so the target ends up with exactly the source's conditions); clearing is one `remove-space-conditions` per space that has
//! conditions. Both are one gesture and so one history row, and both refuse as a whole instead of touching part of the targets.

use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::remove_space_conditions::RemoveSpaceConditions;
use crate::mutations::set_space_conditions::SetSpaceConditions;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "apply-conditions")]
pub struct ApplyConditions {
    pub source: String,
    pub ids: Vec<String>,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "clear-conditions")]
pub struct ClearConditions {
    pub ids: Vec<String>,
}

/// 🏘️ The spaces among the explicit ids, else among the selection, each once, in the order given.
pub fn spaces_of<'a>(snapshot: &ModelSnapshot, ids: &'a [String], selected: &'a [String]) -> Vec<&'a String> {
    let wanted = if ids.is_empty() { selected } else { ids };
    wanted.iter().fold(Vec::new(), |mut spaces, id| {
        if snapshot.spaces.contains_key(id) && !spaces.contains(&id) {
            spaces.push(id);
        }
        spaces
    })
}

/// 🌡️ The space whose conditions are copied: the named one, else the first of the given spaces that states conditions.
pub fn source_of<'a>(snapshot: &ModelSnapshot, source: &'a str, spaces: &[&'a String]) -> Option<&'a str> {
    if source.trim().is_empty() {
        return spaces.iter().find(|id| snapshot.space_conditions.contains_key(id.as_str())).map(|id| id.as_str());
    }
    let source = source.trim();
    snapshot.space_conditions.contains_key(source).then_some(source)
}

/// 🌡️ A command on the conditions of spaces: what it emits for a snapshot and the current selection.
pub trait Conditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault>;
}

impl Conditions for ApplyConditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        apply(self, snapshot, selected)
    }
}

impl Conditions for ClearConditions {
    fn emit(&self, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        clear(self, snapshot, selected)
    }
}

fn apply(payload: &ApplyConditions, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let spaces = spaces_of(snapshot, &payload.ids, selected);
    let source = source_of(snapshot, &payload.source, &spaces).ok_or_else(|| fault("bim.conditions.source-missing", "no space with conditions to copy from among the source, the given spaces and the selection"))?;
    let record = &snapshot.space_conditions[source];
    let targets: Vec<&&String> = spaces.iter().filter(|id| id.as_str() != source).collect();
    if targets.is_empty() {
        return Err(fault("bim.conditions.target-missing", "no other space to copy the conditions to"));
    }
    let mutations: Vec<ModelMutation> = targets.into_iter().filter(|id| snapshot.space_conditions.get(id.as_str()) != Some(record)).map(|id| ModelMutation::SetSpaceConditions(SetSpaceConditions::stating(id, record))).collect();
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

fn clear(payload: &ClearConditions, snapshot: &ModelSnapshot, selected: &[String]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let spaces = spaces_of(snapshot, &payload.ids, selected);
    if spaces.is_empty() {
        return Err(fault("bim.conditions.target-missing", "no space to clear the conditions of among the targets"));
    }
    let mutations: Vec<ModelMutation> = spaces.into_iter().filter(|id| snapshot.space_conditions.contains_key(id.as_str())).map(|id| ModelMutation::RemoveSpaceConditions(RemoveSpaceConditions { id: id.clone() })).collect();
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

pub fn handle<P: Conditions>(payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    payload.emit(doc.snapshot, &ctx.selected)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
