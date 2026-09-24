"""🐍️ WG7 R1–R3 — the wgpu shell's browser document actor: door dialer, browser wiring, execution-target lease before open."""
from pathlib import Path

ENGINE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = Path(ENGINE + "/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
BRIDGE = Path(ENGINE + "/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs")
DOOR = Path(ENGINE + "/🎯️targets/🧊️wgpu/🔌️socket-door/🦀️.rs")
PANEL_TEST = Path(ENGINE + "/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs")

def swap(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:160])
    return text.replace(old, new, 1)

s = SHELL.read_text(encoding="utf-8")
b = BRIDGE.read_text(encoding="utf-8")
d = DOOR.read_text(encoding="utf-8")
p = PANEL_TEST.read_text(encoding="utf-8")

s = swap(s, '''/// 🏛️ The persistence transports the document host this shell build links actually serves. Both
/// builds link the kernel's `ArtifactHost`; its native actor owns the folder event log and the hub
/// WebSocket, while its browser actor has no filesystem and a hub `connect` with no transport yet.''', '''/// 🏛️ The persistence transports the document host this shell build links actually serves. Both
/// builds link the kernel's `ArtifactHost`; its native actor owns the folder event log and the hub
/// WebSocket, while its browser actor has no filesystem and dials its hub through the page's socket door.''')
s = swap(s, '''pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: false };''', '''pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: true };''')

s = swap(s, '''        #[cfg(target_arch = "wasm32")]
        let document_host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));''', '''        #[cfg(target_arch = "wasm32")]
        let document_host = {
            let host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));
            host.set_document_socket_dialer(std::sync::Arc::new(crate::socket_door::browser::DoorDocumentSocketDialer));
            host
        };''')

s = swap(s, '''                        self.directory_client = Some(client.clone());
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            self.document_host.set_local_hub_credential(credential);
                            self.document_host.set_hub_socket_grant_source(client);
                        }''', '''                        self.directory_client = Some(client.clone());
                        self.document_host.set_local_hub_credential(credential);
                        self.document_host.set_hub_socket_grant_source(client);''')

s = swap(s, '''fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {''', '''/// 🪪️ Admits a hub-selected execution-target lease only for the package this shell actually mounted:
/// the same plugin, the same package, the same component digest, the document's schema and the
/// surface the shell requested. Anything else would lend the document actor a kind identity for
/// code that is not running here.
pub(crate) fn document_execution_target_admitted(lease: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1, plugin_id: &str, package_id: Option<&str>, component_sha256: Option<&str>, artifact_schema: &str, surface_id: &str) -> Result<(), &'static str> {
    if lease.package.plugin_id != plugin_id {
        return Err("document-execution-target.plugin-mismatch");
    }
    if package_id != Some(lease.package.package_id.as_str()) {
        return Err("document-execution-target.package-mismatch");
    }
    if component_sha256 != Some(lease.component.sha256.as_str()) {
        return Err("document-execution-target.component-mismatch");
    }
    if lease.artifact.schema != artifact_schema {
        return Err("document-execution-target.schema-mismatch");
    }
    if lease.surface.surface_id != surface_id {
        return Err("document-execution-target.surface-mismatch");
    }
    Ok(())
}

fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {''')

s = swap(s, '''        bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;''', '''        bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        self.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id).await?;
        let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;''')

s = swap(s, '''    /// 📇️ Default bindings for a document opened against the CURRENTLY mounted session's own''', '''    /// 🪪️ The kind identity of a hub document whose schema this process links no codec for: the
    /// hub-selected execution target's lease for the exact scope and the surface this shell requests,
    /// bound onto the document host only when it names the package this shell mounted
    /// ([`document_execution_target_admitted`]). A kind with a linked codec, or a document with no hub
    /// binding, needs none. Both targets walk this one body.
    async fn bind_document_execution_target(&mut self, document_id: &str, schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {
        let Some(space_id) = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.clone()),
            PersistenceBinding::Folder { .. } => None,
        }) else {
            return Ok(());
        };
        if store_sync::os_store::document_codec(schema).await.map_err(|error| format!("document codec registry: {error}"))?.is_some() {
            return Ok(());
        }
        let client = self.directory_client.clone().ok_or("document open requires a signed-in hub session")?;
        let surface_id = wgpu_document_socket_surface(program, app, window_kind_id)?;
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id),
            requested_surface_id: Some(surface_id.clone()),
            client_instance_id: format!("wgpu-shell-{}", self.shell_session_id),
        };
        let lease = client.document_execution_target_manifest(&self.directory_command_ctx(), &intent).await.map_err(|error| format!("document execution target: {error}"))?;
        document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
        if !self.document_host.set_document_execution_target_lease(&ArtifactDocumentKey::hub(space_id, document_id), lease) {
            return Err("document execution target is already bound for this document".into());
        }
        Ok(())
    }

    /// 📇️ Default bindings for a document opened against the CURRENTLY mounted session's own''')

b = swap(b, '''pub struct ProgramBridgeEntry {
    pub plugin_id: String,
    pub package_id: Option<String>,''', '''pub struct ProgramBridgeEntry {
    pub plugin_id: String,
    pub package_id: Option<String>,
    /// 🪪️ The SHA-256 of the component this entry mounts, as its verified package descriptor names it —
    /// what a hub-selected execution-target lease must name for this entry to carry its document.
    pub component_sha256: Option<String>,''')

b = swap(b, '''        let manifest: PluginManifest = serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        Ok(Self {
            plugin_id,
            package_id: None,
            manifest,''', '''        let manifest: PluginManifest = serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        let identity_json = get_fn(&handle, "packageIdentity")?.call0(&JsValue::NULL).map_err(|_| "packageIdentity call failed")?.as_string().ok_or("packageIdentity not string")?;
        let identity: ProgramPackageIdentityV1 = serde_json::from_str(&identity_json).map_err(|err| format!("packageIdentity parse: {err}"))?;
        Ok(Self {
            plugin_id,
            package_id: Some(identity.package_id),
            component_sha256: Some(identity.component_sha256),
            manifest,''')

b = swap(b, '''    pub fn from_wasm(plugin_id: String, package_id: Option<String>, wasm_path: std::path::PathBuf, manifest: PluginManifest) -> Result<Self, String> {
        Ok(Self {
            plugin_id: plugin_id.clone(),
            package_id,''', '''    pub fn from_wasm(plugin_id: String, package_id: Option<String>, component_sha256: Option<String>, wasm_path: std::path::PathBuf, manifest: PluginManifest) -> Result<Self, String> {
        Ok(Self {
            plugin_id: plugin_id.clone(),
            package_id,
            component_sha256,''')

b = swap(b, '''        entries.push(ProgramBridgeEntry::from_wasm(module.plugin_id, Some(descriptor.package_id), modules_root.join(module.wasm_path), descriptor.manifest)?);''', '''        entries.push(ProgramBridgeEntry::from_wasm(module.plugin_id, Some(descriptor.package_id), Some(module.wasm_sha256), modules_root.join(module.wasm_path), descriptor.manifest)?);''')

b = swap(b, '''enum ProgramBridgeBackend {''', '''/// 🪪️ The package a browser program mounts, as the JS bridge's `packageIdentity` reads it off the
/// served package descriptor the module was admitted with.
#[cfg(target_arch = "wasm32")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProgramPackageIdentityV1 {
    package_id: String,
    component_sha256: String,
}

enum ProgramBridgeBackend {''')

p = swap(p, '''ProgramBridgeEntry::from_wasm("space".into(), None, std::path::PathBuf::from("missing-host-panel-guest.wasm"), manifest)''', '''ProgramBridgeEntry::from_wasm("space".into(), None, None, std::path::PathBuf::from("missing-host-panel-guest.wasm"), manifest)''')

d = swap(d, '''    impl Drop for BrowserDoorSocket {
        fn drop(&mut self) {
            self.close();
        }
    }
}''', '''    impl Drop for BrowserDoorSocket {
        fn drop(&mut self) {
            self.close();
        }
    }

    /// ☎️ The browser document actor's dialer: every hub document socket is one page-owned door socket,
    /// the same lane the directory stream and the agent bridge ride.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DoorDocumentSocketDialer;

    impl store_sync::sync::DocumentSocketDialer for DoorDocumentSocketDialer {
        fn dial(&self, url: &str, protocols: &[String]) -> Result<Box<dyn store_sync::sync::DocumentSocket>, String> {
            BrowserDoorSocket::open(url, protocols).map(|socket| Box::new(DoorDocumentSocket { socket }) as Box<dyn store_sync::sync::DocumentSocket>)
        }
    }

    /// 🔌️ One hub document socket read through the duplex door. Its wire is binary frames only; a text
    /// frame is off-contract and skipped, and a loss the lane counted is reported, never swallowed.
    struct DoorDocumentSocket {
        socket: BrowserDoorSocket,
    }

    impl store_sync::sync::DocumentSocket for DoorDocumentSocket {
        fn is_open(&self) -> bool {
            self.socket.lane().is_open()
        }

        fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), String> {
            self.socket.send(SocketMessage::Binary(bytes))
        }

        fn poll(&mut self) -> store_sync::sync::DocumentSocketPoll {
            let dropped = self.socket.lane().dropped();
            if dropped > 0 {
                return store_sync::sync::DocumentSocketPoll::Lost(dropped);
            }
            loop {
                match self.socket.try_recv() {
                    Some(SocketMessage::Binary(bytes)) => return store_sync::sync::DocumentSocketPoll::Frame(bytes),
                    Some(SocketMessage::Text(_)) => continue,
                    None => break,
                }
            }
            let lane = self.socket.lane();
            if lane.is_closed() {
                return store_sync::sync::DocumentSocketPoll::Closed(lane.close_code());
            }
            store_sync::sync::DocumentSocketPoll::Pending
        }

        fn close(&mut self) {
            self.socket.close();
        }
    }
}''')

SHELL.write_text(s, encoding="utf-8")
BRIDGE.write_text(b, encoding="utf-8")
DOOR.write_text(d, encoding="utf-8")
PANEL_TEST.write_text(p, encoding="utf-8")
print("r1-r3: applied")
