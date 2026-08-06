import fs from "fs";

const path = process.argv[2];
let text = fs.readFileSync(path, "utf8");

const sessionStruct = `//#region 🔖️DocumentSession
/// 🧾 Host-owned per-document generation counters and engine cache — guests hold handles only.
pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub engines: store::EngineCache,
}

impl DocumentSession {
    /// 🏗️ Empty session with a fresh engine cache under \`budget_bytes\`.
    pub fn new(budget_bytes: usize) -> Self {
        Self { generation: 0, command_log_len: 0, engines: store::EngineCache::new(budget_bytes) }
    }
}

const DEFAULT_ENGINE_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;
//#endregion 🔖️DocumentSession

`;

if (!text.includes("pub struct DocumentSession")) {
  if (!text.includes("//#region 🔖️HostState\n")) {
    throw new Error("HostState region marker missing");
  }
  text = text.replace("//#region 🔖️HostState\n", sessionStruct + "//#region 🔖️HostState\n");
  console.log("inserted DocumentSession");
} else {
  console.log("DocumentSession already present");
}

const blobField = `    blob_store: Option<Arc<dyn store::BlobStore>>,
}`;
const blobFieldWithSession = `    blob_store: Option<Arc<dyn store::BlobStore>>,
    /// @emoji 🧾 Host-authoritative document session (generation + engine cache).
    session: DocumentSession,
}`;

if (!text.includes("session: DocumentSession")) {
  if (!text.includes(blobField)) {
    throw new Error("blob_store field terminator not found");
  }
  text = text.replace(blobField, blobFieldWithSession);
  console.log("added session field");
} else {
  console.log("session field already present");
}

// has_engine_access after has_backbone_access
const backboneAccess = `    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Backbone && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }
`;
const engineAccess = `    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Backbone && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }

    fn has_engine_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Engine && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }
`;
if (!text.includes("fn has_engine_access")) {
  if (!text.includes(backboneAccess)) throw new Error("has_backbone_access not found");
  text = text.replace(backboneAccess, engineAccess);
  console.log("added has_engine_access");
}

const backboneStatus = `    fn backbone_status(&mut self, uri: String) -> Result<String, Vec<u8>> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
    }
}
//#endregion 🔖️HostState
`;

const withEngineStubs = `    fn backbone_status(&mut self, uri: String) -> Result<String, Vec<u8>> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
    }

    fn engine_derive(&mut self, engine_id: String, input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Invoke) {
            return Err(host_fault_bytes("os.host.engine-derive", "engine invoke capability missing"));
        }
        let handle = self
            .session
            .engines
            .derive(&engine_id, &input)
            .map_err(|error| host_fault_bytes("os.host.engine-derive", error.to_string()))?;
        Ok(handle.key.0.to_vec())
    }

    fn engine_read(&mut self, engine_id: String, key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Read) {
            return Err(host_fault_bytes("os.host.engine-read", "engine read capability missing"));
        }
        let key_bytes: [u8; 32] = key
            .as_slice()
            .try_into()
            .map_err(|_| host_fault_bytes("os.host.engine-read", format!("engine key must be 32 bytes, got {}", key.len())))?;
        let handle = store::EngineHandle {
            key: store::EngineKey(key_bytes),
            engine_id,
        };
        self.session
            .engines
            .read(&handle)
            .map_err(|error| host_fault_bytes("os.host.engine-read", error.to_string()))
    }
}
//#endregion 🔖️HostState
`;

if (!text.includes("fn engine_derive")) {
  if (!text.includes(backboneStatus)) throw new Error("backbone_status block not found");
  text = text.replace(backboneStatus, withEngineStubs);
  console.log("added engine_derive/engine_read");
} else {
  console.log("engine stubs already present");
}

// host_state constructor
const oldCtor = `    fn host_state(plugin_id: &str, manifest: &PluginManifest) -> HostState {
        HostState { wasi: WasiCtxBuilder::new().build(), table: ResourceTable::new(), granted_capabilities: manifest.capabilities.clone(), plugin_id: plugin_id.to_string(), backbones: HashMap::new(), blob_store: None }
    }`;
const newCtor = `    fn host_state(plugin_id: &str, manifest: &PluginManifest) -> HostState {
        HostState {
            wasi: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
            granted_capabilities: manifest.capabilities.clone(),
            plugin_id: plugin_id.to_string(),
            backbones: HashMap::new(),
            blob_store: None,
            session: DocumentSession::new(DEFAULT_ENGINE_CACHE_BUDGET_BYTES),
        }
    }`;
if (!text.includes("session: DocumentSession::new")) {
  if (!text.includes(oldCtor)) throw new Error("host_state ctor not found");
  text = text.replace(oldCtor, newCtor);
  console.log("updated host_state ctor");
} else {
  console.log("host_state ctor already has session");
}

// register_engine helper after deregister_host_blob_store
const deregisterBlob = `    pub fn deregister_host_blob_store(&self) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().blob_store = None;
        Ok(())
    }
`;
const withRegisterEngine = `    pub fn deregister_host_blob_store(&self) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().blob_store = None;
        Ok(())
    }

    /// @emoji ⚙️ Registers a compute kernel on the host \`EngineCache\` under its \`ENGINE_ID\`.
    pub fn register_engine<E: store::Engine>(&self, engine: E) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().session.engines.register(engine);
        Ok(())
    }
`;
if (!text.includes("fn register_engine")) {
  if (!text.includes(deregisterBlob)) throw new Error("deregister_host_blob_store not found");
  text = text.replace(deregisterBlob, withRegisterEngine);
  console.log("added register_engine");
} else {
  console.log("register_engine already present");
}

fs.writeFileSync(path, text);
console.log("wrote", path);
