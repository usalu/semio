//! ♻️ Wires document retirement transfers each exact child materialization before retiring its handle.

use crate::op::WiresMutation;
use crate::{WiresSnapshot, WiresWorkingScene};
use std::{mem::ManuallyDrop, sync::Arc};
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, RetirementStep, SharedValueRetirementFactory};

store::artifact_retire_struct!(WiresWorkingScene { nodes, edges });

struct SceneRoot(ManuallyDrop<Option<Arc<WiresWorkingScene>>>);
impl RetirementCursor for SceneRoot {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.take().and_then(Arc::into_inner).map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl Drop for SceneRoot {
    fn drop(&mut self) {
        assert!(self.0.is_none(), "Wires scene root retired before terminal-empty");
    }
}

struct SnapshotRetirement(ManuallyDrop<Option<WiresSnapshot>>);
impl RetirementCursor for SnapshotRetirement {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        let Some(value) = self.0.as_mut() else {
            return RetirementStep::Complete;
        };
        let scene = match value.content.take_local_owner::<WiresWorkingScene>() {
            Ok(scene) => scene,
            Err(_) => return RetirementStep::BudgetExhausted,
        };
        let WiresSnapshot { wires_fixture, content, meta } = self.0.take().expect("exact Wires snapshot remains owned");
        RetirementStep::Child(store::retirement::sequence(vec![wires_fixture.retirement(), content.retirement(), meta.retirement(), Box::new(SceneRoot(ManuallyDrop::new(scene)))]))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl Drop for SnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.0.is_none(), "Wires snapshot retired before terminal-empty");
    }
}
impl RetireOwned for WiresSnapshot {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(SnapshotRetirement(ManuallyDrop::new(Some(self))))
    }
}

impl RetireOwned for WiresMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CreateNode(value) => value.node.retirement(),
            Self::DeleteNode(value) => value.node_id.retirement(),
            Self::MoveNode(value) => (value.node_id, value.new_x, value.new_y).retirement(),
            Self::ResizeNode(value) => store::artifact_retirement_sequence![value.node_id, value.new_radius, value.new_width, value.new_height],
            Self::ChangeNodeKind(value) => (value.node_id, value.new_node_kind).retirement(),
            Self::ChangeNodeShape(value) => (value.node_id, value.new_shape).retirement(),
            Self::EditNodeText(value) => (value.node_id, value.new_text).retirement(),
            Self::SetNodeRoot(value) => (value.node_id, value.new_root).retirement(),
            Self::ConnectNodes(value) => (value.edge, value.relationship).retirement(),
            Self::DisconnectNodes(value) => value.edge_id.retirement(),
        }
    }
}

/// 🗃️ Installs the document's concrete root and mutation retirement authorities.
pub fn document_store_owners() -> store::DocumentStoreOwners<WiresSnapshot, WiresMutation> {
    store::DocumentStoreOwners::new(
        Arc::new(SharedValueRetirementFactory::<WiresSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<WiresSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<WiresMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<WiresSnapshot, WiresMutation>::new()),
    )
}
