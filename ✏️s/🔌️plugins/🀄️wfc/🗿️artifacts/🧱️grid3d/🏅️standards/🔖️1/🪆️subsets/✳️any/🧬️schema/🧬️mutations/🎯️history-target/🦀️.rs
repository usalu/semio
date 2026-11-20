//! 🎯️ The original conflict path is borrowed from the mutation or rendered into fixed scratch.

use super::Grid3dMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

enum Segment<'a> {
    Id(&'a str),
    Cell(u32, u32, u32),
}

fn absent() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Grid3d conflict path segment is absent")
}

impl Grid3dMutation {
    fn target_segment(&self) -> Option<Segment<'_>> {
        use Grid3dMutation::*;
        match self {
            CreateTile(operation) => Some(Segment::Id(&operation.tile.id)),
            DeleteTile(operation) => Some(Segment::Id(&operation.id)),
            ChangeTileWeight(operation) => Some(Segment::Id(&operation.tile_id)),
            ChangeTileMedia(operation) => Some(Segment::Id(&operation.tile_id)),
            CreateRule(operation) => Some(Segment::Id(&operation.rule.id)),
            DeleteRule(operation) => Some(Segment::Id(&operation.id)),
            PinCell(operation) => Some(Segment::Cell(operation.pinned.x, operation.pinned.y, operation.pinned.z)),
            UnpinCell(operation) => Some(Segment::Cell(operation.x, operation.y, operation.z)),
            MaskCell(operation) => Some(Segment::Cell(operation.cell.x, operation.cell.y, operation.cell.z)),
            UnmaskCell(operation) => Some(Segment::Cell(operation.x, operation.y, operation.z)),
            ChangeSeed(_) | ResizeGrid(_) | ChangeCellSizes(_) | ChangePeriodicity(_) => None,
        }
    }

    fn with_target_segment<T>(&self, index: usize, read: impl FnOnce(&[u8]) -> Result<T, ValueError>) -> Result<T, ValueError> {
        if index != 0 {
            return Err(absent());
        }
        match self.target_segment().ok_or_else(absent)? {
            Segment::Id(id) => read(id.as_bytes()),
            Segment::Cell(x, y, z) => {
                use std::io::Write;
                // Three decimal u32 coordinates have at most ten digits each, plus two colons.
                let mut scratch = [0; 32];
                let mut out = std::io::Cursor::new(&mut scratch[..]);
                write!(out, "{x}:{y}:{z}").map_err(|_| ValueError::invariant("Grid3d conflict coordinate exceeded fixed scratch"))?;
                let length = out.position() as usize;
                read(&scratch[..length])
            }
        }
    }
}

impl ArtifactOperationTargets for Grid3dMutation {
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
