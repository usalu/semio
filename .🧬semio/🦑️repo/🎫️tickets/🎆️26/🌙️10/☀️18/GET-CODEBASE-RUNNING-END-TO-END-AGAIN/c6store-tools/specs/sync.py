from specs.lib import *
F = STORE + '/🔄️sync/🦀️.rs'
CH = [
    ch('🚨', 'sync-errors-actor-identity-and-envelope-serde', 'sync_errors_actor_identity_and_envelope_serde', [(18, 92)],
       'Sync errors, actor identity and envelope serde: the repository root check, the sync error, the actor identity admission and the envelope serde adapter.'),
    ch('🗄', 'persistence-binding-actor-config-and-messages', 'persistence_binding_actor_config_and_messages', [(96, 260)],
       'Persistence binding, actor config and messages: the persistence data class and binding, the actor config, the document key and the actor message set.'),
    ch('📬', 'artifact-mailbox', 'artifact_mailbox', [(262, 473)],
       'Artifact mailbox: the mailbox limits, send error, slots, state and authority and the bounded sender, receiver and close handle pair.'),
    ch('📡', 'actor-message-bytes-status-and-events', 'actor_message_bytes_status_and_events', [(475, 698)],
       'Actor message bytes, status and events: the exact byte measure of an actor message, the remote state, the sync status and the artifact event set.'),
    ch('🧭', 'backbone-retention-bootstrap-and-command-acks', 'backbone_retention_bootstrap_and_command_acks', [(700, 865)],
       'Backbone retention, bootstrap and command acks: the document backbone retention, the bootstrap identity, the frontier reach, the command rejection codes and the command ack outcome.'),
    ch('🧵', 'backbone-worker-wire-and-history-envelopes', 'backbone_worker_wire_and_history_envelopes', [(869, 1098)],
       'Backbone worker wire and history envelopes: the backbone worker wire, the operation ids, the history edit and envelope conversions and the hub socket url.'),
    ch('🔌', 'document-socket-link-and-echo-suppression', 'document_socket_link_and_echo_suppression', [(1102, 1471)],
       'Document socket link and echo suppression: the document socket connect, the commands frames, the link shortage policy and states, the echo suppression and the document socket door.'),
    ch('🌉', 'wire-bridge-sync-session-and-presence-heartbeat', 'wire_bridge_sync_session_and_presence_heartbeat', [(1475, 1744)],
       'Wire bridge, sync session and presence heartbeat: the rollback envelope, the history transition refusal, the presence bytes, the session stamp, the sync session and the presence heartbeat producer.'),
    ch('🏠', 'artifact-host-state-and-core', 'artifact_host_state_and_core', [(1746, 1954)],
       'Artifact host state and core: the channels, the open document, the host state and the host core with its runner operations.'),
]
SPEC = dict(orig='sync.rs', file=F, children=CH, wiring='block', keep_empty=('NativeActor', 'WasmActor', 'FolderStorage', 'BlobStoreImpl'), external=mounted(F, orig='sync.rs'))
