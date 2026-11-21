    /// 🎟️ Advances an authenticated atomic outbox cursor before exposing a physical frame.
    pub fn advance_atomic_outbound(
        &mut self, binding: [u8; 32],
        delivery: &mut Option<semio_os_store_backbone::atomic_transport::AtomicBackboneDelivery>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<Option<RetainedCloneProgress>, ValueError> {
        use semio_os_store_backbone::atomic_transport::AtomicBackboneDelivery;
        use semio_os_spr::atomic_group::AtomicBackboneSender;
        let Some(original) = self.wire.original_outbox.front() else { return Ok(None); };
        if !AtomicBackboneSender::matches_prepared(original) { return Ok(None); }
        let grant = effect.grant();
        let admits = semio_framework_value::retained_clone::retained_clone_grant_admits;
        let header = size_of::<Option<AtomicBackboneDelivery>>();
        let Some(owner) = delivery.as_mut() else {
            let demand = semio_framework_value::RetirementDemand { copy_bytes: header, depth: 1, ..Default::default() };
            if !admits(grant, demand) || !effect.performed(1)? { return Ok(Some(Default::default())); }
            *delivery = Some(AtomicBackboneDelivery::begin(original, binding)?);
            return Ok(Some(RetainedCloneProgress { copied_items: 1, copied_bytes: header, ..Default::default() }));
        };
        owner.check_binding(&binding)?;
        if !owner.terminal_is_empty() { return owner.advance(original, effect).map(Some); }
        let mut demand = self.wire.original_outbox.acknowledgement_demands()?;
        demand.copy_bytes += header;
        if !admits(grant, demand) || !effect.performed(1)? { return Ok(Some(Default::default())); }
        let Some(mut receipt) = self.wire.original_outbox.acknowledge_delivered(grant)? else { return Ok(Some(Default::default())); };
        drop(delivery.take());
        receipt.copied_bytes += header;
        Ok(Some(receipt))
    }

    /// 📤️ Exposes only the completed next physical frame through the authenticated attached endpoint.
    pub async fn send_ready_atomic_outbound(
        &mut self, binding: &[u8; 32],
        delivery: &mut semio_os_store_backbone::atomic_transport::AtomicBackboneDelivery,
    ) -> Result<bool, VcsError> {
        delivery.check_binding(binding).map_err(VcsError::InverseRefused)?;
        let Some(backbone) = self.wire.backbone.as_mut() else { return Ok(false); };
        delivery.send_ready(backbone).await
    }

    /// 📥️ Advances one original atomic part from the bound document actor's private ingress.
    pub fn advance_atomic_inbound(
        &self, binding: [u8; 32], actor: &semio_os_spr::ActorId,
        frame: &mut Option<semio_os_spr::CausalPayload>,
        receiver: &mut Option<semio_os_store_backbone::atomic_transport::AtomicBackboneReceive>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(Option<semio_os_spr::VerifiedAtomicEdit>, RetainedCloneProgress), VcsError> {
        use semio_os_store_backbone::atomic_transport::AtomicBackboneReceive;
        if let Some(owner) = receiver.as_mut() {
            if frame.is_some() {
                let receipt = owner.push(&binding, frame, effect).map_err(VcsError::InverseRefused)?;
                return Ok((None, receipt));
            }
            return owner.seal(&binding, &self.log.envelope.id, actor.0.as_str(), effect);
        }
        let Some(original) = frame.as_ref() else { return Ok((None, Default::default())); };
        let Some((owner, receipt)) = AtomicBackboneReceive::admit(original, binding, effect).map_err(VcsError::InverseRefused)? else {
            return Ok((None, Default::default()));
        };
        *receiver = Some(owner);
        Ok((None, receipt))
    }

    /// 🧲 Extracts only the next authenticated multipart carrier before the ordinary causal pump.
    pub fn take_front_atomic_backbone(&self) -> Result<Option<semio_os_spr::CausalPayload>, VcsError> {
        let Some(backbone) = self.wire.backbone.as_ref() else { return Ok(None); };
        semio_os_store_backbone::atomic_transport::take_atomic_front(backbone)
    }

