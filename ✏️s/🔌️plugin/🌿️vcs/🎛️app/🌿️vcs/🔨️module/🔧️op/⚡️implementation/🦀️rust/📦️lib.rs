//! ⚡️ VCS app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use vcs::VcsDemoProjection;

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum VcsDemoOperation {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum VcsDemoDiff {
    #[default]
    Empty,
    SetCounter {
        counter: i64,
    },
    SetTitle {
        title: String,
    },
    SetNotes {
        notes: String,
    },
    SetStatus {
        status: String,
    },
    AddTag {
        tag: String,
    },
    RemoveTag {
        tag: String,
    },
}

impl OperationDiff<VcsDemoProjection> for VcsDemoDiff {
    fn apply(&self, projection: &VcsDemoProjection) -> VcsDemoProjection {
        let operation = match self {
            VcsDemoDiff::Empty => return projection.clone(),
            VcsDemoDiff::SetCounter { counter } => VcsDemoOperation::SetCounter { counter: *counter },
            VcsDemoDiff::SetTitle { title } => VcsDemoOperation::SetTitle { title: title.clone() },
            VcsDemoDiff::SetNotes { notes } => VcsDemoOperation::SetNotes { notes: notes.clone() },
            VcsDemoDiff::SetStatus { status } => VcsDemoOperation::SetStatus { status: status.clone() },
            VcsDemoDiff::AddTag { tag } => VcsDemoOperation::AddTag { tag: tag.clone() },
            VcsDemoDiff::RemoveTag { tag } => VcsDemoOperation::RemoveTag { tag: tag.clone() },
        };
        apply_vcs_demo_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, VcsDemoDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<VcsDemoProjection> for VcsDemoOperation {
    type Diff = VcsDemoDiff;

    fn diff(&self, _projection: &VcsDemoProjection) -> Self::Diff {
        match self {
            VcsDemoOperation::SetCounter { counter } => VcsDemoDiff::SetCounter { counter: *counter },
            VcsDemoOperation::SetTitle { title } => VcsDemoDiff::SetTitle { title: title.clone() },
            VcsDemoOperation::SetNotes { notes } => VcsDemoDiff::SetNotes { notes: notes.clone() },
            VcsDemoOperation::SetStatus { status } => VcsDemoDiff::SetStatus { status: status.clone() },
            VcsDemoOperation::AddTag { tag } => VcsDemoDiff::AddTag { tag: tag.clone() },
            VcsDemoOperation::RemoveTag { tag } => VcsDemoDiff::RemoveTag { tag: tag.clone() },
        }
    }

    fn backwards(&self, projection: &VcsDemoProjection) -> Vec<Self> {
        match self {
            VcsDemoOperation::SetCounter { .. } => vec![VcsDemoOperation::SetCounter { counter: projection.counter }],
            VcsDemoOperation::SetTitle { .. } => vec![VcsDemoOperation::SetTitle { title: projection.title.clone() }],
            VcsDemoOperation::SetNotes { .. } => vec![VcsDemoOperation::SetNotes { notes: projection.notes.clone() }],
            VcsDemoOperation::SetStatus { .. } => vec![VcsDemoOperation::SetStatus { status: projection.status.clone() }],
            VcsDemoOperation::AddTag { tag } => vec![VcsDemoOperation::RemoveTag { tag: tag.clone() }],
            VcsDemoOperation::RemoveTag { tag } => vec![VcsDemoOperation::AddTag { tag: tag.clone() }],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn apply_vcs_demo_operation(projection: &VcsDemoProjection, operation: &VcsDemoOperation) -> VcsDemoProjection {
    let mut next = projection.clone();
    match operation {
        VcsDemoOperation::SetCounter { counter } => next.counter = *counter,
        VcsDemoOperation::SetTitle { title } => next.title = title.clone(),
        VcsDemoOperation::SetNotes { notes } => next.notes = notes.clone(),
        VcsDemoOperation::SetStatus { status } => next.status = status.clone(),
        VcsDemoOperation::AddTag { tag } => {
            if !next.tags.contains(tag) {
                next.tags.push(tag.clone());
            }
        }
        VcsDemoOperation::RemoveTag { tag } => next.tags.retain(|entry| entry != tag),
    }
    next
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ConfigOperations
/// 🧮️ `vcs_engine::VcsDemoConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `VcsPlayApp` field writes/deleted `ViewState.locale`), plus a generic `Snapshot` every
/// variant's `backwards()` returns (see `shooting_op::ShootingConfigOperation`'s identical doc for why
/// this whole-config-snapshot-undo shape is correct and sufficient here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum VcsDemoConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: vcs_engine::VcsDemoConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { checkpoint_ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<vcs_engine::VcsDemoConfig> for VcsDemoConfigOperation {
    type Diff = vcs_engine::VcsDemoConfig;

    fn diff(&self, base: &vcs_engine::VcsDemoConfig) -> vcs_engine::VcsDemoConfig {
        let mut next = base.clone();
        match self {
            VcsDemoConfigOperation::Snapshot { config } => return config.clone(),
            VcsDemoConfigOperation::SetSelection { checkpoint_ids } => next.selected_checkpoint_ids = checkpoint_ids.clone(),
            VcsDemoConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &vcs_engine::VcsDemoConfig) -> Vec<Self> {
        vec![VcsDemoConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetCounter { counter: 3 });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetTitle { title: "Untitled".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::AddTag { tag: "draft".into() });
    }

    /// 🧮️ Round-trip law per `VcsDemoConfigOperation` variant (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-
    /// SCHEMA-FLOW-CONFIG-ON-NODE).
    #[test]
    fn vcs_demo_config_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::Snapshot { config: vcs_engine::VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "de-DE".into() } });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::SetSelection { checkpoint_ids: vec!["checkpoint-1".into(), "checkpoint-2".into()] });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` always returns a `Snapshot` of the pre-operation config, so applying it after
    /// the forward op exactly restores the original — the "whole-config-snapshot-undo" law.
    #[test]
    fn vcs_demo_config_operation_backwards_restores_the_base_config() {
        let base = vcs_engine::VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "en-US".into() };
        let operation = VcsDemoConfigOperation::SetLocale { value: "de-DE".into() };
        let forward = operation.diff(&base);
        assert_eq!(forward.locale, "de-DE");
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![VcsDemoConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
