//! 🎯️ Original conflict paths borrow the authored mutation target.
use super::Wfc3dMutation;

use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

fn absent_target() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Wfc3d conflict path segment is absent")
}

impl Wfc3dMutation {
    fn target_segment(&self, index: usize) -> Option<&str> {
        use Wfc3dMutation::*;
        match self {
            DragSlots(op) => op.targets.get(index).map(String::as_str),
            SetSlotPositions(op) => op.positions.get(index).map(|position| position.id.as_str()),
            _ if index != 0 => None,
            CreateSlot(op) => Some(&op.slot.id),
            DeleteSlot(op) => Some(&op.id),
            MoveSlot(op) => Some(&op.id),
            ResizeSlot(op) => Some(&op.id),
            ConnectSlots(op) => Some(&op.edge.id),
            DisconnectSlots(op) => Some(&op.id),
            PinSlot(op) => Some(&op.id),
            UnpinSlot(op) => Some(&op.id),
            CreateTile(op) => Some(&op.tile.id),
            DeleteTile(op) => Some(&op.id),
            ChangeTileWeight(op) => Some(&op.id),
            ChangeTileMedia(op) => Some(&op.id),
            CreateRule(op) => Some(&op.rule.id),
            DeleteRule(op) => Some(&op.id),
            ChangeSeed(_) => None,
        }
    }
}

impl ArtifactOperationTargets for Wfc3dMutation {
    fn len(&self) -> usize {
        match self {
            Self::DragSlots(op) => op.targets.len(),
            Self::SetSlotPositions(op) => op.positions.len(),
            _ => usize::from(self.target_segment(0).is_some()),
        }
    }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.target_segment(index).map(str::len).ok_or_else(absent_target)
    }

    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, ValueError> {
        let segment = self.target_segment(index).ok_or_else(absent_target)?;
        let rest = segment.as_bytes().get(offset..).ok_or_else(absent_target)?;
        let copied = output.len().min(rest.len());
        output[..copied].copy_from_slice(&rest[..copied]);
        Ok(copied)
    }
}

