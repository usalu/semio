//! ↩️ `change-automation-class` inverse.

use crate::mutations::change_automation_class::ChangeAutomationClass;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeAutomationClass, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeAutomationClass(ChangeAutomationClass { new_automation_class: base.automation_class })]
}
