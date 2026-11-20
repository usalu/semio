//! 🎯️ The original conflict path is borrowed from the mutation or rendered into fixed scratch.

use super::Grid2dMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

enum Segment<'a> {
    Id(&'a str),
    Cell(u32, u32),
}

fn absent() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Grid2d conflict path segment is absent")
}

impl Grid2dMutation {
    fn target_segment(&self) -> Option<Segment<'_>> {
        use Grid2dMutation::*;
        match self {
            CreateTile(operation) => Some(Segment::Id(&operation.tile.id)),
            DeleteTile(operation) => Some(Segment::Id(&operation.id)),
            ChangeTileWeight(operation) => Some(Segment::Id(&operation.id)),
            ChangeTileMedia(operation) => Some(Segment::Id(&operation.id)),
            CreateRule(operation) => Some(Segment::Id(&operation.rule.id)),
            DeleteRule(operation) => Some(Segment::Id(&operation.id)),
            PinCell(operation) => Some(Segment::Cell(operation.x, operation.y)),
            UnpinCell(operation) => Some(Segment::Cell(operation.x, operation.y)),
            MaskCell(operation) => Some(Segment::Cell(operation.x, operation.y)),
            UnmaskCell(operation) => Some(Segment::Cell(operation.x, operation.y)),
            ChangeSeed(_) | ResizeGrid(_) | ChangeCellSize(_) | ChangePeriodicity(_) => None,
        }
    }

    fn with_target_segment<T>(&self, index: usize, read: impl FnOnce(&[u8]) -> Result<T, ValueError>) -> Result<T, ValueError> {
        if index != 0 {
            return Err(absent());
        }
        match self.target_segment().ok_or_else(absent)? {
            Segment::Id(id) => read(id.as_bytes()),
            Segment::Cell(x, y) => {
                use std::io::Write;
                // Two decimal u32 coordinates have at most ten digits each, plus a comma.
                let mut scratch = [0; 21];
                let mut out = std::io::Cursor::new(&mut scratch[..]);
                write!(out, "{x},{y}").map_err(|_| ValueError::invariant("Grid2d conflict coordinate exceeded fixed scratch"))?;
                let length = out.position() as usize;
                read(&scratch[..length])
            }
        }
    }
}

impl ArtifactOperationTargets for Grid2dMutation {
    fn len(&self) -> usize {
        usize::from(self.target_segment().is_some())
    }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.with_target_segment(index, |segment| Ok(segment.len()))
    }

    fn read_segment(&self, index: usize, offset: usize, to: &mut [u8]) -> Result<usize, ValueError> {
        self.with_target_segment(index, |segment| {
            let rest = segment.get(offset..).ok_or_else(absent)?;
            let copied = to.len().min(rest.len());
            to[..copied].copy_from_slice(&rest[..copied]);
            Ok(copied)
        })
    }
}
