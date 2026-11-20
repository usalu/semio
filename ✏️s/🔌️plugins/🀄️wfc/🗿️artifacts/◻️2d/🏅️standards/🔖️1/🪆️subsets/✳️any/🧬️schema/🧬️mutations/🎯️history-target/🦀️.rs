//! 🎯️ Original conflict paths borrow the authored mutation target.
use super::Wfc2dMutation;

use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

fn absent_target() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Wfc2d conflict path segment is absent")
}

impl Wfc2dMutation {
    fn target_segment(&self, index: usize) -> Option<&str> {
        use Wfc2dMutation::*;
        match self {
            DragSlots(op) => op.targets.get(index).map(String::as_str),
            SetSlotPositions(op) => op.positions.get(index).map(|position| position.id.as_str()),
            _ => None,
        }
    }
}

impl ArtifactOperationTargets for Wfc2dMutation {
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

