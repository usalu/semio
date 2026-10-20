    pub(crate) fn watch_hub_document(&self, reactor: &'static store::sync::DocumentSocketReactor, document_key: store::sync::ArtifactDocumentKey, relay: Arc<HubRelay>) {
        let host = self.artifact_host.clone();
        let actor = self.actor_label();
        let pumped_key = document_key.clone();
        self.artifact_host.pump_events_key(reactor, &pumped_key, move |event| {
            if agent_presence_moment(&event) {
                host.presence_heartbeat_key(&document_key, now_ms(), agent_presence_peer(&actor));
            }
            match event {
                store::sync::ArtifactEvent::DocumentArchiveReplaced { archive } => relay.deliver(HubInbound::Archive(archive)),
                store::sync::ArtifactEvent::DocumentBackbone { message } => relay.deliver(HubInbound::Backbone(message)),
                store::sync::ArtifactEvent::Status(status) => relay.record(Some(status), None),
                store::sync::ArtifactEvent::Conflict(message) if document_link_terminal_code(&message.code.0) => relay.record_terminal(&message.code.0, &message.message),
                store::sync::ArtifactEvent::Conflict(message) => relay.record(None, Some(format!("{}: {}", message.code.0, message.message))),
                _ => {}
            }
        });
    }
}
