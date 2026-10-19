//! 🗄️ Fixed packet pages and lease: the visual freshness, the fixed packet pages and order and the mounted visual lease. See [`super`].

use super::*;

//#region 👁️LiveVisualLanguage
pub(super) const FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES: usize = semio_framework_ui_scene::CANVAS2D_SNAPSHOT_PAGE_CAPACITY * CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY;

const FEM2D_MOUNTED_VISUAL_PAGE_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fem2dVisualFreshness {
    pub app_instance_id: u32,
    pub model_revision: u64,
    pub document_generation: u64,
    pub operation: u64,
    pub numerical_preview_sequence: u64,
    pub surface_generation: u64,
    pub renderer_scene_generation: u64,
}

pub(super) const FEM2D_MOUNTED_VISUAL_PAGE_COUNT: usize = FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES / FEM2D_MOUNTED_VISUAL_PAGE_BYTES;

#[derive(Debug)]
pub(super) struct Fem2dFixedPacketPages {
    pub(super) pages: [Option<Canvas2dSnapshotPage>; FEM2D_MOUNTED_VISUAL_PAGE_COUNT],
    write_page: usize,
    pub(super) len: usize,
}

impl Fem2dFixedPacketPages {
    pub(super) fn new() -> Self {
        Self { pages: std::array::from_fn(|_| None), write_page: 0, len: 0 }
    }

    pub(super) fn admit_page(&mut self, page: usize) -> Result<(), &'static [u8]> {
        let Some(owner) = self.pages.get_mut(page) else { return Err(b"fem2d.visual-page-index") };
        if owner.is_some() {
            return Err(b"fem2d.visual-page-duplicate");
        }
        *owner = Some(Canvas2dSnapshotPage::new());
        Ok(())
    }

    pub(super) fn take_page(&mut self, page: usize) -> Option<Canvas2dSnapshotPage> {
        self.pages.get_mut(page)?.take()
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) else { return (true, 0, 0) };
        if maximum_bytes < FEM2D_MOUNTED_VISUAL_PAGE_BYTES {
            return (false, 0, 0);
        }
        *page = None;
        (self.pages.iter().all(Option::is_none), 1, FEM2D_MOUNTED_VISUAL_PAGE_BYTES)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
}

impl Write for Fem2dFixedPacketPages {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        if value.len() > FEM2D_MOUNTED_VISUAL_PAGE_BYTES || self.len.checked_add(value.len()).is_none_or(|len| len > FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES) {
            return Err(std::fmt::Error);
        }
        if self.pages.get(self.write_page).and_then(Option::as_ref).is_none_or(|page| page.remaining() < value.len()) {
            self.write_page += 1;
        }
        self.pages.get_mut(self.write_page).and_then(Option::as_mut).ok_or(std::fmt::Error)?.push(value.as_bytes()).map_err(|_| std::fmt::Error)?;
        self.len += value.len();
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Fem2dFixedOrder<const N: usize> {
    pub(super) slots: [Option<usize>; N],
    pub(super) len: usize,
}

impl<const N: usize> Fem2dFixedOrder<N> {
    pub(super) fn new() -> Self {
        Self { slots: [None; N], len: 0 }
    }

    pub(super) fn push(&mut self, value: usize) -> Result<(), ()> {
        let Some(slot) = self.slots.get_mut(self.len) else { return Err(()) };
        *slot = Some(value);
        self.len += 1;
        Ok(())
    }

    pub(super) fn get(&self, index: usize) -> Option<usize> {
        self.slots.get(index).copied().flatten()
    }

    pub(super) fn swap(&mut self, left: usize, right: usize) {
        self.slots.swap(left, right);
    }

    pub(super) fn pop(&mut self) -> Option<usize> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.slots[index].take()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<const N: usize> Default for Fem2dFixedOrder<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct Fem2dMountedVisualLease {
    pub(super) app_instance_id: u32,
    pub(super) base_revision: u64,
    pub(super) generation: u64,
    pub(super) operation: u64,
    pub(super) preview_sequence: u64,
    pub(super) surface_generation: u64,
    pub(super) renderer_scene_generation: u64,
    pub(super) snapshot: Canvas2dSnapshotLease,
    pub(super) close_started: bool,
    pub(super) region_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_REGIONS>,
    pub(super) assembly_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_ELEMENTS>,
    pub(super) field_order: Fem2dFixedOrder<FEM2D_VISUAL_MAXIMUM_FIELDS>,
}

impl Fem2dMountedVisualLease {
    pub(crate) fn matches(&self, app_instance_id: u32, base_revision: u64, generation: u64) -> bool {
        self.app_instance_id == app_instance_id && self.base_revision == base_revision && self.generation == generation
    }

    pub(crate) fn snapshot(&self) -> Canvas2dSnapshotLease {
        self.snapshot
    }

    pub fn matches_freshness(&self, freshness: Fem2dVisualFreshness) -> bool {
        self.app_instance_id == freshness.app_instance_id
            && self.base_revision == freshness.model_revision
            && self.generation == freshness.document_generation
            && self.operation == freshness.operation
            && self.preview_sequence == freshness.numerical_preview_sequence
            && self.surface_generation == freshness.surface_generation
            && self.renderer_scene_generation == freshness.renderer_scene_generation
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if maximum_bytes < size_of::<usize>() { return (false, 0, 0); }
        if self.region_order.pop().is_some() || self.assembly_order.pop().is_some() || self.field_order.pop().is_some() {
            return (false, 1, size_of::<usize>());
        }
        if maximum_bytes < CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY {
            return (false, 0, 0);
        }
        if !self.close_started {
            if canvas2d_snapshot_begin_close(self.snapshot).is_err() {
                return (false, 0, 0);
            }
            self.close_started = true;
            return (false, 1, 0);
        }
        match canvas2d_snapshot_close_step(self.snapshot) {
            Ok(terminal) => (terminal, usize::from(!terminal), usize::from(!terminal) * CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY),
            Err(_) => (false, 0, 0),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        canvas2d_snapshot_terminal_is_empty(self.snapshot) && self.region_order.is_empty() && self.assembly_order.is_empty() && self.field_order.is_empty()
    }
}
//#endregion 👁️LiveVisualLanguage
