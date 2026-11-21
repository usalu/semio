    async fn advance_original_atomic_backbone(
        &mut self, inbox: &mut store::os_store::backbone::atomic_transport::AtomicBackboneInbox,
        original: &mut Option<Vec<u8>>, binding: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<Option<protocol::MergeReport>, OriginalTurnFailure> {
        let before = self.stores.store.generation();
        let report = semio_plugin_reactor_app::atomic_document::fold_atomic_inbox(&mut self.stores.store, inbox, original, binding, cx).await?;
        if self.stores.store.generation() != before {
            self.dispatch_ctx().queue_base_moved();
            self.history.cache = None;
        }
        Ok(report)
    }

    fn original_atomic_outbound_pending(&self) -> bool {
        semio_plugin_reactor_app::atomic_document::original_atomic_outbox(self.stores.store.original_outbound_message())
    }

    fn advance_original_atomic_outbound(
        &mut self, delivery: &mut Option<store::os_store::backbone::atomic_transport::AtomicBackboneDelivery>,
        binding: [u8; 32], cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        semio_plugin_reactor_app::atomic_document::drive_atomic_outbox(cx, |effect| self.stores.store.advance_bound_atomic_outbound(binding, delivery, effect))
    }
