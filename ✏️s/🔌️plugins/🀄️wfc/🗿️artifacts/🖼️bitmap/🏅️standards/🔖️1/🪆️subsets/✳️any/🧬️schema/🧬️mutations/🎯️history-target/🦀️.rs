//! 🎯️ Original conflict paths borrow the authored mutation target.
use super::BitmapMutation;

use semio_framework_value::{ValueError, ValueRefusalKind};
use store::ArtifactOperationTargets;

fn absent_target() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Bitmap conflict path segment is absent")
}

impl BitmapMutation {
    fn with_target_segment<T>(&self, index: usize, read: impl FnOnce(&[u8]) -> Result<T, ValueError>) -> Result<T, ValueError> {
        use std::io::Write;
        use BitmapMutation::*;
        if index != 0 {
            return Err(absent_target());
        }
        let mut scratch = [0; 21];
        let mut out = std::io::Cursor::new(&mut scratch[..]);
        let written = match self {
            PinSolution(_) | RestorePins(_) => return read(b"pinned"),
            PinPixel(op) => write!(out, "{}:{}", op.x, op.y),
            UnpinPixel(op) => write!(out, "{}:{}", op.x, op.y),
            RemovePaletteColor(op) => write!(out, "{}", op.index),
            ChangeSeed(_) | ResizeInput(_) | SetInputPixels(_) | AddPaletteColor(_) |
            ChangePaletteColor(_) | ResizeOutput(_) | ChangeModel(_) | PaintInputStroke(_) => return Err(absent_target()),
        };
        written.map_err(|_| ValueError::invariant("Bitmap conflict path exceeded fixed scratch"))?;
        let length = out.position() as usize;
        read(&scratch[..length])
    }
}

impl ArtifactOperationTargets for BitmapMutation {
    fn len(&self) -> usize {
        usize::from(self.with_target_segment(0, |_| Ok(())).is_ok())
    }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.with_target_segment(index, |segment| Ok(segment.len()))
    }

    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, ValueError> {
        self.with_target_segment(index, |segment| {
            let rest = segment.get(offset..).ok_or_else(absent_target)?;
            let copied = output.len().min(rest.len());
            output[..copied].copy_from_slice(&rest[..copied]);
            Ok(copied)
        })
    }
}

