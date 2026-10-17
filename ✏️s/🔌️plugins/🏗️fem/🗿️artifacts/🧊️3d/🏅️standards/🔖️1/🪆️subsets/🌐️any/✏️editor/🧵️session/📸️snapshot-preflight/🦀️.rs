//! 📸️ Snapshot preflight of the FEM 3D session: the current publication, the preflight stages and the pending snapshot. See [`super`].

use super::*;

//#region 💼️MountedSession
#[derive(Clone, Copy)]
pub(super) struct Current {
    pub(super) app_instance_id: u32,
    pub(super) shell: u16,
    pub(super) identity: Identity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SnapshotPreflightStage {
    Nodes,
    Elements,
    Regions,
    Supports,
    LoadCases,
    Loads,
    Complete,
}

#[derive(Clone, Copy)]
pub(super) struct SnapshotPreflight {
    pub(super) stage: SnapshotPreflightStage,
    pub(super) outer: usize,
    pub(super) inner: usize,
    load_count: usize,
    page_items: [u16; FEM3D_VISUAL_PAGES],
    page_bytes: [u32; FEM3D_VISUAL_PAGES],
    pub(super) item_count: u32,
    pub(super) byte_count: u32,
    pub(super) draw_count: u32,
    pub(super) draw_bytes: u32,
}

impl SnapshotPreflight {
    pub(super) fn new() -> Self {
        let mut preflight = Self { stage: SnapshotPreflightStage::Nodes, outer: 0, inner: 0, load_count: 0, page_items: [0; FEM3D_VISUAL_PAGES], page_bytes: [0; FEM3D_VISUAL_PAGES], item_count: 0, byte_count: 0, draw_count: 0, draw_bytes: 0 };
        let _ = preflight.charge(0, 1, 3, 0, 3);
        let _ = preflight.charge(20, 1, 2 + FEM3D_VISUAL_LABEL_EN.len(), 0, 0);
        let _ = preflight.charge(20, 1, 2 + FEM3D_VISUAL_LABEL_DE.len(), 0, 0);
        preflight
    }

    fn charge(&mut self, page: usize, items: u16, bytes: usize, draws: u32, draw_bytes: usize) -> Result<(), ()> {
        let page_items = self.page_items[page].checked_add(items).ok_or(())?;
        let page_bytes = self.page_bytes[page].checked_add(u32::try_from(bytes).map_err(|_| ())?).ok_or(())?;
        let item_count = self.item_count.checked_add(u32::from(items)).ok_or(())?;
        let byte_count = self.byte_count.checked_add(u32::try_from(bytes).map_err(|_| ())?).ok_or(())?;
        let draw_count = self.draw_count.checked_add(draws).ok_or(())?;
        let draw_bytes = self.draw_bytes.checked_add(u32::try_from(draw_bytes).map_err(|_| ())?).ok_or(())?;
        if usize::from(page_items) > WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY
            || page_bytes as usize > WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY
            || item_count as usize > FEM3D_VISUAL_PAGES * WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY
            || byte_count as usize > FEM3D_VISUAL_PAGES * WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY
        {
            return Err(());
        }
        self.page_items[page] = page_items;
        self.page_bytes[page] = page_bytes;
        self.item_count = item_count;
        self.byte_count = byte_count;
        self.draw_count = draw_count;
        self.draw_bytes = draw_bytes;
        Ok(())
    }

    pub(super) fn credit(self) -> Result<Fem3dPageCredit, ()> {
        (self.stage == SnapshotPreflightStage::Complete).then_some(Fem3dPageCredit { item_count: self.item_count, byte_count: self.byte_count, draw_count: self.draw_count, draw_bytes: self.draw_bytes }).ok_or(())
    }

    fn next_stage(&mut self, stage: SnapshotPreflightStage) {
        self.stage = stage;
        self.outer = 0;
        self.inner = 0;
    }

    pub(super) fn step_one(&mut self, snapshot: &Fem3dSnapshot) -> Result<bool, ()> {
        match self.stage {
            SnapshotPreflightStage::Nodes => {
                let Some(node) = snapshot.nodes.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Elements);
                    return Ok(false);
                };
                let page = 2 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY;
                self.charge(page, 1, node.id.len(), 1, node.id.len())?;
                self.charge(10 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(12 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(14 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(16 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.charge(18 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, node.id.len(), 1, node.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Elements => {
                let Some(element) = snapshot.elements.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Regions);
                    return Ok(false);
                };
                let id = element_id(element);
                self.charge(4 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, id.len(), 1, id.len())?;
                self.charge(6 + self.outer / WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY, 1, id.len(), 1, id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Regions => {
                let Some(solid) = snapshot.solids.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Supports);
                    return Ok(false);
                };
                let kind_bytes = if solid.layers == 1 { "tetrahedron".len() } else { "hexahedron".len() };
                self.charge(1, 1, solid.id.len().checked_add(kind_bytes).ok_or(())?, 1, solid.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::Supports => {
                let Some(support) = snapshot.supports.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::LoadCases);
                    return Ok(false);
                };
                self.charge(9, 1, support.id.len(), 1, support.id.len())?;
                self.outer += 1;
            }
            SnapshotPreflightStage::LoadCases => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.next_stage(SnapshotPreflightStage::Loads);
                    return Ok(false);
                };
                let next = self.load_count.checked_add(case.loads.len()).ok_or(())?;
                if next > MAXIMUM_LOADS {
                    return Err(());
                }
                self.load_count = next;
                self.outer += 1;
            }
            SnapshotPreflightStage::Loads => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.stage = SnapshotPreflightStage::Complete;
                    return Ok(false);
                };
                let Some(load) = case.loads.get(self.inner) else {
                    self.outer += 1;
                    self.inner = 0;
                    return Ok(false);
                };
                let id = load_id(load);
                self.charge(8, 1, id.len(), 1, id.len())?;
                self.inner += 1;
            }
            SnapshotPreflightStage::Complete => return Ok(true),
        }
        Ok(false)
    }
}

#[derive(Clone, Copy)]
pub(super) struct PendingSnapshot {
    pub(super) render: AppRenderOperationContext,
    pub(super) preflight: SnapshotPreflight,
}
//#endregion 💼️MountedSession
