//! 📍️ Snapshot admission and registry: the current session, the pending admissions, the snapshot admission cursor and the mounted registry. See [`super`].

use super::*;

//#region 🧰️FixedArena
#[derive(Clone, Copy)]
pub(super) struct CurrentSession {
    pub(super) app_instance_id: u32,
    pub(super) shell: u16,
    pub(super) identity: MountedIdentity,
}

#[derive(Clone, Copy)]
pub(super) struct PendingAdmission {
    pub(super) app_instance_id: u32,
    pub(super) shell: u16,
    pub(super) identity: MountedIdentity,
    pub(super) admitted_items: usize,
}

#[derive(Clone, Copy)]
pub(super) struct SnapshotAdmissionCursor {
    lane: u8,
    pub(super) outer: usize,
    inner: usize,
    deep: usize,
    owner_opened: bool,
    pub(super) items: usize,
    pub(super) bytes: usize,
    visual_loads: usize,
}

impl SnapshotAdmissionCursor {
    pub(super) fn new() -> Self {
        Self { lane: 0, outer: 0, inner: 0, deep: 0, owner_opened: false, items: 1, bytes: size_of::<Fem2dSnapshot>(), visual_loads: 0 }
    }

    pub(super) fn charge(&mut self, items: usize, bytes: usize) -> Result<(), &'static [u8]> {
        let items = self.items.checked_add(items).ok_or(b"fem2d.session-item-overflow" as &'static [u8])?;
        let bytes = self.bytes.checked_add(bytes).ok_or(b"fem2d.session-byte-overflow" as &'static [u8])?;
        if items > SESSION_MAXIMUM_INPUT_ITEMS || bytes > SESSION_MAXIMUM_INPUT_BYTES {
            return Err(b"fem2d.session-admission-exceeded");
        }
        self.items = items;
        self.bytes = bytes;
        Ok(())
    }

    fn reset_lane(&mut self) {
        self.lane += 1;
        self.outer = 0;
        self.inner = 0;
        self.deep = 0;
        self.owner_opened = false;
    }

    pub(super) fn step_one(&mut self, snapshot: &Fem2dSnapshot) -> Result<bool, &'static [u8]> {
        if !self.owner_opened && !matches!(self.lane, 4 | 9 | 11) {
            let bytes = match self.lane {
                0 => snapshot.nodes.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode>(),
                1 => snapshot.elements.capacity() * size_of::<FemElement>(),
                2 => snapshot.regions.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion>(),
                3 => 0,
                5 => snapshot.materials.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemMaterial>(),
                6 => snapshot.sections.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection>(),
                7 => snapshot.supports.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport>(),
                8 => snapshot.load_cases.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase>(),
                10 => snapshot.combinations.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombination>(),
                _ => 0,
            };
            self.owner_opened = true;
            self.charge(1, bytes)?;
            return Ok(false);
        }
        let item = match self.lane {
            0 => {
                if snapshot.nodes.len() > SESSION_MAXIMUM_NODES {
                    return Err(b"fem2d.session-node-capacity");
                }
                match snapshot.nodes.get(self.outer) {
                    Some(node) => Some(bounded_string_capacities(&[&node.id])?),
                    None => None,
                }
            }
            1 => {
                if snapshot.elements.len() > SESSION_MAXIMUM_ELEMENTS {
                    return Err(b"fem2d.session-element-capacity");
                }
                match snapshot.elements.get(self.outer) {
                    Some(FemElement::Bar { id, start, end, material_id, section_id }) | Some(FemElement::Beam { id, start, end, material_id, section_id }) => Some(bounded_string_capacities(&[id, start, end, material_id, section_id])?),
                    None => None,
                }
            }
            2 => {
                if snapshot.regions.len() > 1 {
                    return Err(b"fem2d.session-region-capacity");
                }
                match snapshot.regions.get(self.outer) {
                    Some(region) => Some(if region.outline.len() > SESSION_MAXIMUM_BOUNDARY_POINTS || region.holes.len() > SESSION_MAXIMUM_REGION_HOLES {
                        SESSION_MAXIMUM_INPUT_BYTES + 1
                    } else {
                        bounded_string_capacities(&[&region.id, &region.name, &region.material_id])? + region.outline.capacity() * 16 + region.holes.capacity() * size_of::<Vec<[f64; 2]>>()
                    }),
                    None => None,
                }
            }
            3 => {
                let Some(region) = snapshot.regions.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if self.inner < region.outline.len() {
                    self.inner += 1;
                    Some(0)
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    return Ok(false);
                }
            }
            4 => {
                let Some(region) = snapshot.regions.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                let Some(hole) = region.holes.get(self.inner) else {
                    self.outer += 1;
                    self.inner = 0;
                    self.deep = 0;
                    self.owner_opened = false;
                    return Ok(false);
                };
                if hole.len() > SESSION_MAXIMUM_BOUNDARY_POINTS {
                    return Err(b"fem2d.session-hole-capacity");
                }
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, hole.capacity() * 16)?;
                    return Ok(false);
                }
                if self.deep < hole.len() {
                    self.deep += 1;
                    Some(0)
                } else {
                    self.inner += 1;
                    self.deep = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            5 => match snapshot.materials.get(self.outer) {
                Some(material) => Some(bounded_string_capacities(&[&material.id, &material.name])?),
                None => None,
            },
            6 => match snapshot.sections.get(self.outer) {
                Some(section) => Some(bounded_string_capacities(&[&section.id, &section.name])?),
                None => None,
            },
            7 => {
                if snapshot.supports.len() > SESSION_MAXIMUM_SUPPORTS {
                    return Err(b"fem2d.session-support-capacity");
                }
                match snapshot.supports.get(self.outer) {
                    Some(support) => Some(bounded_string_capacities(&[&support.id, &support.node_id])? + support.fixed.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof>()),
                    None => None,
                }
            }
            8 => match snapshot.load_cases.get(self.outer) {
                Some(case) => Some(bounded_string_capacities(&[&case.id, &case.name])?),
                None => None,
            },
            9 => {
                let Some(case) = snapshot.load_cases.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, case.loads.capacity() * size_of::<FemLoad>())?;
                    return Ok(false);
                }
                if let Some(load) = case.loads.get(self.inner) {
                    self.visual_loads = self.visual_loads.checked_add(1).ok_or(b"fem2d.visual-load-count-overflow" as &'static [u8])?;
                    if self.visual_loads > SESSION_MAXIMUM_VISUAL_LOADS {
                        return Err(b"fem2d.visual-load-maximum-plus-one");
                    }
                    self.inner += 1;
                    Some(match load {
                        FemLoad::Nodal { id, node_id, .. } => bounded_string_capacities(&[id, node_id])?,
                        FemLoad::MemberUdl { id, element_id, .. } => bounded_string_capacities(&[id, element_id])?,
                        FemLoad::Area { id, region_id, .. } => bounded_string_capacities(&[id, region_id])?,
                    })
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            10 => match snapshot.combinations.get(self.outer) {
                Some(combination) => Some(bounded_string_capacities(&[&combination.id, &combination.name])?),
                None => None,
            },
            11 => {
                let Some(combination) = snapshot.combinations.get(self.outer) else {
                    self.reset_lane();
                    return Ok(false);
                };
                if !self.owner_opened {
                    self.owner_opened = true;
                    self.charge(1, combination.terms.capacity() * size_of::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm>())?;
                    return Ok(false);
                }
                if let Some(term) = combination.terms.get(self.inner) {
                    self.inner += 1;
                    Some(bounded_string_capacities(&[&term.case_id])?)
                } else {
                    self.outer += 1;
                    self.inner = 0;
                    self.owner_opened = false;
                    return Ok(false);
                }
            }
            _ => return Ok(true),
        };
        if let Some(bytes) = item {
            self.charge(1, bytes)?;
            if matches!(self.lane, 0 | 1 | 2 | 5 | 6 | 7 | 8 | 10) {
                self.outer += 1;
            }
            return Ok(false);
        }
        self.reset_lane();
        Ok(self.lane > 11)
    }

    pub(super) fn process_credit(self) -> Result<(usize, usize), &'static [u8]> {
        let (items, bytes) = MountedProcessOwnerCatalog::fixed().credit(self.items, self.bytes)?;
        if items > SESSION_MAXIMUM_ITEMS || bytes > SESSION_MAXIMUM_BYTES {
            return Err(b"fem2d.session-process-admission-exceeded");
        }
        Ok((items, bytes))
    }
}

#[derive(Clone, Copy)]
pub(super) struct PendingSnapshotAdmission {
    pub(super) app_instance_id: u32,
    pub(super) render: AppRenderOperationContext,
    pub(super) cursor: SnapshotAdmissionCursor,
}

#[derive(Clone, Copy)]
pub(super) struct PendingSnapshotFault {
    pub(super) app_instance_id: u32,
    pub(super) render: AppRenderOperationContext,
    pub(super) detail: &'static str,
    pub(super) emitted: bool,
}

pub(super) struct MountedRegistry {
    pub(super) shells: [Rc<RefCell<Option<MountedState>>>; SESSION_SHELL_CAPACITY],
    pub(super) current: [Option<CurrentSession>; SESSION_ACTIVE_CAPACITY],
    pub(super) preflight: [Option<PendingSnapshotAdmission>; SESSION_ACTIVE_CAPACITY],
    pub(super) preflight_fault: [Option<PendingSnapshotFault>; SESSION_ACTIVE_CAPACITY],
    pub(super) pending: [Option<PendingAdmission>; SESSION_ACTIVE_CAPACITY],
    pub(super) retiring: [Option<u16>; SESSION_SHELL_CAPACITY],
    free: [u16; SESSION_SHELL_CAPACITY],
    free_read: usize,
    pub(super) free_len: usize,
    pub(super) credit_items: [usize; SESSION_SHELL_CAPACITY],
    pub(super) credit_bytes: [usize; SESSION_SHELL_CAPACITY],
    pub(super) reserved_items: usize,
    pub(super) reserved_bytes: usize,
    pub(super) next_job: u64,
}

impl MountedRegistry {
    pub(super) fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            current: [None; SESSION_ACTIVE_CAPACITY],
            preflight: [None; SESSION_ACTIVE_CAPACITY],
            preflight_fault: [None; SESSION_ACTIVE_CAPACITY],
            pending: [None; SESSION_ACTIVE_CAPACITY],
            retiring: [None; SESSION_SHELL_CAPACITY],
            free: std::array::from_fn(|index| index as u16),
            free_read: 0,
            free_len: SESSION_SHELL_CAPACITY,
            credit_items: [0; SESSION_SHELL_CAPACITY],
            credit_bytes: [0; SESSION_SHELL_CAPACITY],
            reserved_items: 0,
            reserved_bytes: 0,
            next_job: 0,
        }
    }

    pub(super) fn allocate(&mut self) -> Option<u16> {
        if self.free_len == 0 {
            return None;
        }
        let shell = self.free[self.free_read];
        self.free_read = (self.free_read + 1) % SESSION_SHELL_CAPACITY;
        self.free_len -= 1;
        Some(shell)
    }

    pub(super) fn release(&mut self, shell: u16) {
        assert_eq!(self.credit_items[shell as usize], 0, "mounted FEM shell released before its process item credit");
        assert_eq!(self.credit_bytes[shell as usize], 0, "mounted FEM shell released before its process byte credit");
        let write = (self.free_read + self.free_len) % SESSION_SHELL_CAPACITY;
        self.free[write] = shell;
        self.free_len += 1;
    }

    pub(super) fn reserve_credit(&mut self, shell: u16, admitted_items: usize, admitted_bytes: usize) -> bool {
        if self.credit_items[shell as usize] != 0 || self.credit_bytes[shell as usize] != 0 || admitted_items == 0 || admitted_bytes == 0 || admitted_items > SESSION_MAXIMUM_ITEMS || admitted_bytes > SESSION_MAXIMUM_BYTES {
            return false;
        }
        let Some(items) = self.reserved_items.checked_add(admitted_items) else { return false };
        let Some(bytes) = self.reserved_bytes.checked_add(admitted_bytes) else { return false };
        if items > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_ITEMS || bytes > SESSION_SHELL_CAPACITY * SESSION_MAXIMUM_BYTES {
            return false;
        }
        self.credit_items[shell as usize] = admitted_items;
        self.credit_bytes[shell as usize] = admitted_bytes;
        self.reserved_items = items;
        self.reserved_bytes = bytes;
        true
    }

    pub(super) fn release_credit(&mut self, shell: u16) {
        let items = std::mem::take(&mut self.credit_items[shell as usize]);
        let bytes = std::mem::take(&mut self.credit_bytes[shell as usize]);
        assert!(items != 0 && bytes != 0, "mounted FEM shell released a missing process credit");
        self.reserved_items -= items;
        self.reserved_bytes -= bytes;
    }

    pub(super) fn retain_retirement(&mut self, shell: u16) -> bool {
        let Some(slot) = self.retiring.iter_mut().find(|slot| slot.is_none()) else { return false };
        *slot = Some(shell);
        true
    }
}
//#endregion 🧰️FixedArena
