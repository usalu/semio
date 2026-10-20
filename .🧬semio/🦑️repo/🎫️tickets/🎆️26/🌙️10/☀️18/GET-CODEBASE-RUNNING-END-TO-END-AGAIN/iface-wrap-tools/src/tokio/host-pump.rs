    /// 📡️ Hands every event of one open document to `on_event`, in order, on `reactor`, until the document closes; no receiver type appears in the signature.
    pub fn pump_events_key(&self, reactor: &DocumentSocketReactor, document_key: &ArtifactDocumentKey, on_event: impl FnMut(ArtifactEvent) + Send + 'static) {
        let events = semio_framework_async::poll::resolve_ready(self.subscribe_key(document_key));
        reactor.spawn(pump_events(events, on_event));
    }

