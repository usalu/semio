//! 📤️ Posts the already prepared atomic frame as an original owned actor-port wire buffer.

use super::*;

impl AtomicBackboneDelivery {
    pub fn needs_actor_wire(&self) -> bool { !self.sent && self.frame.is_some() }

    pub fn advance_actor_wire(
        &mut self, endpoint: &Backbones, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, VcsError> {
        let Some(frame) = self.frame.as_ref().filter(|_| !self.sent) else { return Ok(Default::default()); };
        let Backbones::Port(port) = endpoint else { return Err(VcsError::InverseRefused(invalid())); };
        let Some(BackboneChannelPorts::Actor(channel)) = port.channel.as_deref() else { return Err(VcsError::InverseRefused(invalid())); };
        let mut prefix = [1, 1, 0, 1, 0, 8, 0, 0, 0];
        let mut len = frame.len();
        let mut width = 6;
        while len >= 128 {
            prefix[width] = (len as u8 & 127) | 128;
            width += 1;
            len >>= 7;
        }
        prefix[width] = len as u8;
        width += 1;
        let extent = width + frame.len();
        let refusal = VcsError::InverseRefused;
        if self.actor_wire.is_none() {
            let demand = RetirementDemand { copy_bytes: width + size_of::<Option<Vec<u8>>>(), capacity_bytes: extent, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(refusal)? {
                return Ok(Default::default());
            }
            let mut wire = Vec::new();
            wire.try_reserve_exact(extent).map_err(|_| refusal(invalid()))?;
            wire.extend_from_slice(&prefix[..width]);
            let bytes = wire.capacity();
            self.actor_wire = ManuallyDrop::new(Some(wire));
            return Ok(RetainedCloneProgress { retained_capacity_bytes: bytes, ..progress(demand) });
        }
        let wire = self.actor_wire.as_mut().unwrap();
        if wire.len() != extent {
            let count = (extent - wire.len()).min(4096);
            let demand = RetirementDemand { copy_bytes: count, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(count as u64).map_err(refusal)? {
                return Ok(Default::default());
            }
            let from = wire.len() - width;
            wire.extend_from_slice(&frame[from..from + count]);
            return Ok(progress(demand));
        }
        let mut state = channel.state.lock().map_err(|_| refusal(invalid()))?;
        if !state.accepting || state.uri != port.uri { return Err(refusal(invalid())); }
        let next_bytes = state.outbound_bytes.checked_add(extent).ok_or_else(|| refusal(invalid()))?;
        if state.outbound.len() >= super::super::BACKBONE_CHANNEL_MAXIMUM_MESSAGES || next_bytes > super::super::BACKBONE_CHANNEL_MAXIMUM_BYTES {
            return Ok(Default::default());
        }
        let demand = RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>() + size_of::<usize>() + size_of::<bool>(), depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1).map_err(refusal)? {
            return Ok(Default::default());
        }
        state.outbound.push_back(self.actor_wire.take().unwrap());
        state.outbound_bytes = next_bytes;
        self.sent = true;
        Ok(progress(demand))
    }

    pub(super) fn actor_wire_close_demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>(), release_bytes: self.actor_wire.as_ref().map_or(0, Vec::capacity), depth: 1, ..Default::default() }
    }

    pub(super) fn close_actor_wire(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = self.actor_wire_close_demand();
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok(Default::default());
        }
        drop(self.actor_wire.take());
        Ok(progress(demand))
    }
}
