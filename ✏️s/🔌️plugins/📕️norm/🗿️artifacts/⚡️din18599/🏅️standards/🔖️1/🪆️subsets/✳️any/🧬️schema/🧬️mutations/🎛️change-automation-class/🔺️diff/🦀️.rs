//! 🔺️ `change-automation-class` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_automation_class::ChangeAutomationClass;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeAutomationClass, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.automation_class == payload.new_automation_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "automation-class already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { automation_class: Some(payload.new_automation_class), ..Default::default() })
}
