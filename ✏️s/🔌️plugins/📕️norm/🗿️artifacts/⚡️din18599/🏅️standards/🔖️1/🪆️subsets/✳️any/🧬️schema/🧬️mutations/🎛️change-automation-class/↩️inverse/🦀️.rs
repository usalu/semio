//! ↩️ `change-automation-class` inverse.

use crate::change_automation_class::ChangeAutomationClass;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeAutomationClass, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_automation_class::ChangeAutomationClass> {
    Ok((|| {
    vec![Op::from(ChangeAutomationClass { new_automation_class: base.automation_class })]

    })())
}
