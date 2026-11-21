//! 🪟️ Samples a baseline JPEG canvas preview through the original coefficient loan.

use super::{BaselineJpgCursor, BaselineJpgMetadata, JpgByteSource, JpgError, ZIGZAG_TO_NATURAL, idct_8x8};

const MAX_PREVIEW_EDGE: usize = 256;

#[derive(Clone, Copy)]
pub struct JpgPreviewSize {
    pub source: [usize; 2],
    pub display: [usize; 2],
    pub channels: u8,
}

impl JpgPreviewSize {
    fn valid(self) -> bool {
        matches!(self.channels, 1 | 3)
            && (0..2).all(|axis| {
                (1..=u16::MAX as usize).contains(&self.source[axis])
                    && (1..=self.source[axis].min(MAX_PREVIEW_EDGE)).contains(&self.display[axis])
            })
    }

    fn sample(self, axis: usize, output: usize, sampling: usize, maximum: usize) -> usize {
        let source = (2 * output + 1) * self.source[axis] / (2 * self.display[axis]);
        source * sampling / maximum
    }

    fn first(self, axis: usize, mut target: usize, sampling: usize, maximum: usize) -> usize {
        target *= 8;
        let (mut low, mut high) = (0, self.display[axis]);
        while low < high {
            let middle = low + (high - low) / 2;
            if self.sample(axis, middle, sampling, maximum) < target { low = middle + 1; }
            else { high = middle; }
        }
        low
    }
}

pub struct BaselineJpgPreview {
    size: JpgPreviewSize,
    inner: BaselineJpgCursor,
}

impl BaselineJpgPreview {
    /// 🧭️ Creates one cursor for the admitted image extent and destination sample grid.
    pub fn new(length: usize, size: JpgPreviewSize) -> Result<Self, JpgError> {
        if !size.valid() {
            return Err(JpgError::Malformed("JPEG preview dimensions exceed the admitted sample grid".into()));
        }
        Ok(Self { size, inner: BaselineJpgCursor::new(length) })
    }

    /// 🔆️ Writes at most one coefficient block to the private preview, skipping inverse transforms outside the grid.
    pub fn step(
        &mut self,
        source: &dyn JpgByteSource,
        write_component: &mut dyn FnMut(usize, usize, usize, u8),
    ) -> Result<Option<BaselineJpgMetadata>, JpgError> {
        let size = self.size;
        self.inner.step(source, &mut |block| {
            if block.component >= usize::from(size.channels) { return; }
            let x0 = size.first(0, block.block_x, block.sampling_x, block.maximum_sampling_x);
            let x1 = size.first(0, block.block_x + 1, block.sampling_x, block.maximum_sampling_x);
            if x0 == x1 { return; }
            let y0 = size.first(1, block.block_y, block.sampling_y, block.maximum_sampling_y);
            let y1 = size.first(1, block.block_y + 1, block.sampling_y, block.maximum_sampling_y);
            if y0 == y1 { return; }
            let mut natural = [0.0; 64];
            for (index, &position) in ZIGZAG_TO_NATURAL.iter().enumerate() {
                natural[position] = f64::from(block.zigzag[index]) * f64::from(block.quantization[index]);
            }
            let spatial = idct_8x8(&natural);
            for y in y0..y1 {
                let row = size.sample(1, y, block.sampling_y, block.maximum_sampling_y) % 8;
                for x in x0..x1 {
                    let col = size.sample(0, x, block.sampling_x, block.maximum_sampling_x) % 8;
                    let value = (spatial[8 * row + col] + 128.0).round().clamp(0.0, 255.0) as u8;
                    write_component(x, y, block.component, value);
                }
            }
        })?;
        let Some(metadata) = self.inner.complete() else { return Ok(None); };
        if size.source != [metadata.width as usize, metadata.height as usize] || size.channels != metadata.components {
            return Err(JpgError::Malformed("JPEG preview source and admitted dimensions differ".into()));
        }
        Ok(Some(metadata))
    }
}
