//! 📸set-snapshot sparse diff leaf.
use crate::artifacts::present::diff::diff_set_snapshot;
use crate::artifacts::present::mutations::PresentMutation;

pub fn diff(mutation: &PresentMutation) -> crate::artifacts::present::diff::PresentDiff {
    match mutation {
        PresentMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        _ => unreachable!("set-snapshot diff only for SetSnapshot"),
    }
}
