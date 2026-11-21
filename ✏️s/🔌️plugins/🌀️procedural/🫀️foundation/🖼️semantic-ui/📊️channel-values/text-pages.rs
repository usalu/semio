//! 📃️ Bounded random access to decoded UTF-8 channel text.
const TEXT_PAGE_BYTES: usize = 256;

pub(super) struct TextPages<'a>(pub &'a str);

impl<'a> TextPages<'a> {
    pub(super) fn len(&self) -> usize {
        self.0.len().div_ceil(TEXT_PAGE_BYTES)
    }

    pub(super) fn get(&self, index: usize) -> Option<&'a str> {
        let offset = index.checked_mul(TEXT_PAGE_BYTES)?;
        if offset >= self.0.len() {
            return None;
        }
        let boundary = |mut at| {
            while !self.0.is_char_boundary(at) {
                at -= 1;
            }
            at
        };
        let end = offset.saturating_add(TEXT_PAGE_BYTES).min(self.0.len());
        self.0.get(boundary(offset)..boundary(end))
    }
}
