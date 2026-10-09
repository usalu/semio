//! 📨️ `engagementSubmit` (Enter in the entry field): gives the typed line to the armed utility of the addressed window as a click at the point it names, so every placement tool can be
//! driven from the keyboard alone. `x, y` is absolute, `@dx, dy` and `length<angle` are measured from the point the gesture hangs on, a bare length goes towards where the pointer
//! last was, and an empty line finishes the gesture. A line that names no point is refused and stays in the field; an accepted one empties it.

use crate::editor::bim::gestures::{keep_entry, run_typed};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: String,
}

pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let emit = run_typed(ctx, doc, &payload.value)?;
    keep_entry(ctx, "");
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
