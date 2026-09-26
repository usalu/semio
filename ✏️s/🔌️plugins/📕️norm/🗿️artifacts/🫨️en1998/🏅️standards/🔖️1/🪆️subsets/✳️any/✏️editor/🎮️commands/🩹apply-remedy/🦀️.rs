//! Norm command — `apply-remedy` (coerces Exactly→bool when the target leaf is boolean).

use crate::document::{cached_report_for, NormFamily, RemedyBound};
use crate::editor::en1998::En1998Family;
use crate::op::En1998Mutation;
use crate::En1998Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-remedy")]
pub struct ApplyRemedy {
    pub check_id: String,
    pub remedy_index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1998Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1998Mutation, NoConfigMutation>, Fault> {
    let report = cached_report_for::<En1998Family>(doc.snapshot).unwrap_or_else(|| <En1998Family as NormFamily>::evaluate(doc.snapshot));
    let check = report
        .checks
        .iter()
        .find(|c| c.id == payload.check_id)
        .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-check"), format!("check '{}'", payload.check_id)))?;
    let remedy = check
        .remedies
        .get(payload.remedy_index as usize)
        .ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("norm.apply-remedy-missing-remedy"), format!("remedy {}", payload.remedy_index)))?
        .clone();
    let path = remedy.target.path.clone();
    crate::app_surface::commit_value_tree_edit(
        doc.snapshot,
        "applyRemedy",
        move |tree| {
            let current = crate::app_surface::get_value_at_path(tree, &path)?;
            let value = match current {
                dsl::DslValue::Bool(_) => dsl::DslValue::Bool(remedy.required.value >= 0.5),
                _ if matches!(remedy.bound, RemedyBound::OneOf) => {
                    let option = remedy.options.first().cloned().unwrap_or_default();
                    dsl::DslValue::String(option)
                }
                _ => dsl::DslValue::float(remedy.required.value),
            };
            crate::app_surface::set_value_at_path(tree, &path, value)
        },
        |base, target| En1998Mutation::from_snapshot(base, target),
    )
}
//#endregion 🔖️Handler
