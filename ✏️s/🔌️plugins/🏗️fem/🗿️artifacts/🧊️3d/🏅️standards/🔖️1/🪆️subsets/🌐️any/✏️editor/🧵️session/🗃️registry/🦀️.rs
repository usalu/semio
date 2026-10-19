//! 🗃️ Registry of the FEM 3D session: the slot arrays of the mounted apps and the thread-local registry. See [`super`].

use super::*;

//#region 💼️MountedSession
pub(super) struct Registry {
    pub(super) app_slots: semio_framework_value::FixedKeySlots<u32, ACTIVE_CAPACITY>,
    pub(super) requests: semio_framework_value::FixedRequestOwners<semio_framework_value::ContentRequestKey, ConsumerKey, ACTIVE_CAPACITY, 64>,
    pub(super) verified_sources: [Option<(semio_framework_value::RequestCandidateVersion, RequestKey)>; 64],
    pub(super) request_specs: [Option<PhysicalRequestSpec>; ACTIVE_CAPACITY],
    pub(super) shell_request: [Option<semio_framework_value::FixedKeySlot>; SHELL_CAPACITY],
    pub(super) request_cursor: usize,
    pub(super) consumer_cursor: usize,
    pub(super) shells: [Rc<RefCell<Option<MountedState>>>; SHELL_CAPACITY],
    pub(super) recoveries: [Rc<MountedRecoverySlot>; SHELL_CAPACITY],
    pub(super) current: [Option<Current>; ACTIVE_CAPACITY],
    pub(super) retained_results: [Option<Current>; ACTIVE_CAPACITY],
    pub(super) pending: [Option<PendingSnapshot>; ACTIVE_CAPACITY],
    pub(super) requested: [Option<(u32, Fem3dResultRequest)>; ACTIVE_CAPACITY],
    pub(super) retiring: [bool; SHELL_CAPACITY],
    pub(super) retiring_app: [u32; SHELL_CAPACITY],
    pub(super) retiring_count: [usize; ACTIVE_CAPACITY],
    pub(super) retiring_owner: [u32; ACTIVE_CAPACITY],
    pub(super) free: [u16; SHELL_CAPACITY],
    pub(super) free_len: usize,
    pub(super) maintenance_cursor: usize,
    pub(super) recovery_cursor: usize,
    pub(super) next_job: u64,
    pub(super) credit_items: [usize; SHELL_CAPACITY],
    pub(super) credit_bytes: [usize; SHELL_CAPACITY],
    pub(super) reserved_items: usize,
    pub(super) reserved_bytes: usize,
}

impl Registry {
    pub(super) fn new() -> Self {
        Self {
            app_slots: Default::default(),
            requests: Default::default(),
            verified_sources: [None; 64],
            request_specs: [None; ACTIVE_CAPACITY],
            shell_request: [None; SHELL_CAPACITY],
            request_cursor: 0,
            consumer_cursor: 0,
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            recoveries: std::array::from_fn(|_| Rc::new(MountedRecoverySlot::new())),
            current: std::array::from_fn(|_| None),
            retained_results: std::array::from_fn(|_| None),
            pending: std::array::from_fn(|_| None),
            requested: std::array::from_fn(|_| None),
            retiring: [false; SHELL_CAPACITY],
            retiring_app: [0; SHELL_CAPACITY],
            retiring_count: [0; ACTIVE_CAPACITY],
            retiring_owner: [0; ACTIVE_CAPACITY],
            free: std::array::from_fn(|index| (SHELL_CAPACITY - 1 - index) as u16),
            free_len: SHELL_CAPACITY,
            maintenance_cursor: 0,
            recovery_cursor: 0,
            next_job: 0,
            credit_items: [0; SHELL_CAPACITY],
            credit_bytes: [0; SHELL_CAPACITY],
            reserved_items: 0,
            reserved_bytes: 0,
        }
    }

    pub(super) fn allocate(&mut self) -> Option<u16> {
        let next = self.free_len.checked_sub(1)?;
        self.free_len = next;
        Some(self.free[next])
    }

    pub(super) fn release(&mut self, shell: u16) {
        assert_eq!(self.credit_items[shell as usize], 0, "FEM3D shell released before its process item credit");
        assert_eq!(self.credit_bytes[shell as usize], 0, "FEM3D shell released before its process byte credit");
        assert!(self.recoveries[shell as usize].reserved.get().is_none(), "FEM3D shell released before its recovery authority");
        if self.free_len < SHELL_CAPACITY {
            self.free[self.free_len] = shell;
            self.free_len += 1;
        }
    }

    pub(super) fn reserve_credit(&mut self, shell: u16) -> bool {
        let slot = shell as usize;
        if self.credit_items[slot] != 0 || self.credit_bytes[slot] != 0 {
            return false;
        }
        let Some(items) = self.reserved_items.checked_add(FEM3D_PROCESS_BACKING_ITEMS).filter(|items| *items <= SHELL_CAPACITY * FEM3D_PROCESS_BACKING_ITEMS) else { return false };
        let Some(bytes) = self.reserved_bytes.checked_add(FEM3D_PROCESS_BACKING_BYTES).filter(|bytes| *bytes <= SHELL_CAPACITY * FEM3D_PROCESS_BACKING_BYTES) else { return false };
        self.credit_items[slot] = FEM3D_PROCESS_BACKING_ITEMS;
        self.credit_bytes[slot] = FEM3D_PROCESS_BACKING_BYTES;
        self.reserved_items = items;
        self.reserved_bytes = bytes;
        true
    }

    pub(super) fn release_credit(&mut self, shell: u16) {
        let items = std::mem::take(&mut self.credit_items[shell as usize]);
        let bytes = std::mem::take(&mut self.credit_bytes[shell as usize]);
        assert_eq!((items, bytes), (FEM3D_PROCESS_BACKING_ITEMS, FEM3D_PROCESS_BACKING_BYTES), "FEM3D shell returned a mismatched process credit");
        self.reserved_items -= items;
        self.reserved_bytes -= bytes;
    }

    pub(super) fn retain_retiring(&mut self, app_instance_id: u32, shell: u16) -> bool {
        let Some(slot) = self.app_slots.lookup(app_instance_id).map(|slot| slot.index()) else { return false };
        if self.retiring[shell as usize] || (self.retiring_count[slot] != 0 && self.retiring_owner[slot] != app_instance_id) {
            return false;
        }
        self.retiring[shell as usize] = true;
        self.retiring_app[shell as usize] = app_instance_id;
        self.retiring_owner[slot] = app_instance_id;
        self.retiring_count[slot] += 1;
        true
    }

    pub(super) fn retire_retained_results(&mut self, app_instance_id: u32, slot: usize) -> bool {
        let Some(retained) = self.retained_results[slot].filter(|retained| retained.app_instance_id == app_instance_id) else { return true };
        if !self.retain_retiring(app_instance_id, retained.shell) { return false; }
        self.retained_results[slot] = None;
        true
    }
}

thread_local! {
    pub(super) static MOUNTED: RefCell<Registry> = RefCell::new(Registry::new());
}
//#endregion 💼️MountedSession
