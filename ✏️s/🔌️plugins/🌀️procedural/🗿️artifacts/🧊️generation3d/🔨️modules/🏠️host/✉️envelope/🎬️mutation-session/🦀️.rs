//! 🎬️ Mutation session of the owned envelope catalog: the session phases and the bounded mutation decode session with its drop guard. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationSessionPhase {
    Format,
    Ordinal,
    Body,
    Ready,
    Published,
    Closing,
    Closed,
}

pub struct Generation3dMutationSession {
    phase: Generation3dMutationSessionPhase,
    expected_bytes: usize,
    pub(super) maximum_items: usize,
    admitted: usize,
    pending: Option<(u64, u8)>,
    pub(super) ordinal: u64,
    ordinal_shift: u32,
    ordinal_bytes: u8,
    body_bytes: u64,
    sealed: bool,
    pub(super) body: std::mem::ManuallyDrop<Option<store::mounted_pack_rt::RetainedRecordBodyCursor>>,
    pub(super) owner: std::mem::ManuallyDrop<Option<Generation3dRetainedMutationOwner>>,
}

impl Generation3dMutationSession {
    pub fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if !(3..=GENERATION3D_OWNER_BYTES).contains(&expected_bytes) || maximum_items == 0 {
            return Err("generation3d-mutation.exact-credits");
        }
        Ok(Self {
            phase: Generation3dMutationSessionPhase::Format,
            expected_bytes,
            maximum_items,
            admitted: 0,
            pending: None,
            ordinal: 0,
            ordinal_shift: 0,
            ordinal_bytes: 0,
            body_bytes: 0,
            sealed: false,
            body: std::mem::ManuallyDrop::new(None),
            owner: std::mem::ManuallyDrop::new(None),
        })
    }

    pub fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if self.pending.is_some() || self.sealed || self.admitted == self.expected_bytes {
            return Err(value);
        }
        self.pending = Some((self.admitted as u64, value));
        self.admitted += 1;
        Ok(())
    }

    pub fn ingress_ready(&self) -> bool {
        self.pending.is_none()
            && (self.phase != Generation3dMutationSessionPhase::Body
                || self.body.as_ref().is_some_and(store::mounted_pack_rt::RetainedRecordBodyCursor::ingress_ready))
    }

    pub fn seal(&mut self) -> Result<(), &'static str> {
        if self.pending.is_some() || self.admitted != self.expected_bytes || self.phase != Generation3dMutationSessionPhase::Body {
            return Err("generation3d-mutation.exact-byte-seal");
        }
        self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.seal(self.body_bytes).map_err(|_| "generation3d-mutation.body-seal")?;
        self.sealed = true;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Generation3dMutationSessionPhase::Ready | Generation3dMutationSessionPhase::Published) {
            return Ok(true);
        }
        if let (Some(owner), Some(body)) = (self.owner.as_mut(), self.body.as_ref()) {
            if owner.grant_symbol(body)? {
                return Ok(false);
            }
        }
        match self.phase {
            Generation3dMutationSessionPhase::Format => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.format-input")?;
                if byte != dsl::variants_binary::OP_BINARY_FORMAT {
                    return Err("generation3d-mutation.format");
                }
                self.phase = Generation3dMutationSessionPhase::Ordinal;
            }
            Generation3dMutationSessionPhase::Ordinal => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.ordinal-input")?;
                if self.ordinal_bytes >= 10 || (self.ordinal_bytes == 9 && (byte & 0xfe) != 0) {
                    return Err("generation3d-mutation.ordinal-overflow");
                }
                self.ordinal |= u64::from(byte & 0x7f) << self.ordinal_shift;
                self.ordinal_shift += 7;
                self.ordinal_bytes += 1;
                if byte & 0x80 == 0 {
                    if self.ordinal_bytes > 1 && byte & 0x7f == 0 {
                        return Err("generation3d-mutation.ordinal-noncanonical");
                    }
                    let ordinal = u8::try_from(self.ordinal).map_err(|_| "generation3d-mutation.variant")?;
                    let limits = store::mounted_pack_rt::PackLimits {
                        max_file_len: self.expected_bytes as u64,
                        max_segment_len: self.expected_bytes as u64,
                        max_symbols: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u32,
                        max_depth: GENERATION3D_RETAINED_STACK_CAPACITY as u16,
                        max_items: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u64,
                        max_total_alloc: GENERATION3D_MAXIMUM_DOMAIN_BYTES as u64,
                    };
                    *self.body = Some(
                        store::mounted_pack_rt::RetainedRecordBodyCursor::try_new(
                            limits,
                            self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS),
                            self.expected_bytes,
                            self.expected_bytes,
                            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,
                        )
                        .map_err(|_| "generation3d-mutation.body-preflight")?,
                    );
                    *self.owner = Some(Generation3dRetainedMutationOwner::new(ordinal)?);
                    self.phase = Generation3dMutationSessionPhase::Body;
                }
            }
            Generation3dMutationSessionPhase::Body => {
                if self.pending.is_some() && self.body.as_ref().ok_or("generation3d-mutation.body-owner")?.ingress_ready() {
                    let (_, byte) = self.pending.take().ok_or("generation3d-mutation.body-input")?;
                    self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.admit_byte(self.body_bytes, byte).map_err(|(_, byte)| if byte == 0 { "generation3d-mutation.body-handback-zero" } else { "generation3d-mutation.body-handback" })?;
                    self.body_bytes += 1;
                }
                if let Some(store::mounted_pack_rt::RetainedRecordBodyToken::Value(token)) = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.grant().map_err(|_| "generation3d-mutation.body-malformed")? {
                    let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                    let body = self.body.as_ref().expect("P3 retained mutation body");
                    self.owner.as_mut().expect("P3 retained mutation owner").accept(token, body)?;
                    if complete {
                        self.phase = Generation3dMutationSessionPhase::Ready;
                        return Ok(true);
                    }
                }
            }
            _ => return Err("generation3d-mutation.session-state"),
        }
        Ok(false)
    }

    pub fn take(&mut self) -> Option<Generation3dMutation> {
        if self.phase != Generation3dMutationSessionPhase::Ready {
            return None;
        }
        let value = self.owner.as_mut()?.take()?;
        self.phase = Generation3dMutationSessionPhase::Published;
        Some(value)
    }

    pub fn next_retained_allocation_bytes(&mut self) -> Result<Option<usize>, &'static str> {
        let Some(body) = self.body.as_mut() else { return Ok(None) };
        body.next_allocation_bytes().map_err(|_| "generation3d-mutation.body-allocation")
    }

    pub fn reserve_retained_allocation(&mut self, maximum_bytes: usize) -> Result<(bool, usize), &'static str> {
        let body = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?;
        body.reserve_allocation(maximum_bytes)
            .map(|step| (step.progressed, step.allocated_bytes))
            .map_err(|_| "generation3d-mutation.body-allocation")
    }

    pub fn retained_allocated_bytes(&self) -> usize {
        self.body.as_ref().map_or(0, store::mounted_pack_rt::RetainedRecordBodyCursor::allocated_bytes)
    }

    pub fn next_retained_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() || self.owner.is_some() {
            return None;
        }
        self.body.as_ref()?.next_release_allocation_bytes().ok().flatten()
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::mounted_pack_rt::RetainedPackCloseStep, store::PackRefusal> {
        use store::mounted_pack_rt::RetainedPackCloseStep;
        if self.phase == Generation3dMutationSessionPhase::Closed {
            return Ok(RetainedPackCloseStep::Complete);
        }
        if maximum_items == 0 && (self.pending.is_some() || self.owner.is_some() || self.body.is_none()) {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closing;
        if maximum_items != 0 && self.pending.take().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.as_mut() {
            if !owner.close_step() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(self.owner.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(body) = self.body.as_mut() {
            match body.close_step(maximum_items, maximum_bytes)? {
                RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                    return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                }
                RetainedPackCloseStep::Complete => {
                    if maximum_items == 0 {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                }
            }
            drop(self.body.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closed;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == Generation3dMutationSessionPhase::Closed && self.owner.is_none() && self.body.is_none() && self.pending.is_none()
    }
}

impl Drop for Generation3dMutationSession {
    /// 🔒️ Fail-closed on a live drop, but never while the thread is already unwinding — a second
    /// panic in a destructor during cleanup aborts the process instead of reporting the first
    /// failure. Same guard `OrderedMap::drop` and the framework's retirement roots carry.
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Generation3d mutation session reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
