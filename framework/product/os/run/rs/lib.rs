//! 🕸️ Headless computation of an OS studio's media graph — no UI involved. A `StudioRunner` walks
//! `OsMediaGraph` in topological order, instantiates each node's app through a `MediaNodeHost`, moves
//! `Media` along edges, and skips any node whose inputs and document are unchanged since the last run.
//! Importing media is emitting ops: a headless run is an ordinary editing session (actor `runner`)
//! recorded in each app document's own VCS envelope, so a later UI open sees it as normal history.

//#region 🔖Types
use semio_framework_core::{Media, MediaError, MediaFingerprint};
use semio_framework_os::{OsAppInstance, OsMediaGraph, OsMediaGraphNode};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

/// 🚧 A failure computing a studio's media graph headlessly.
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("unknown media-graph node {0}")]
    UnknownNode(String),
    #[error("unknown app instance {0}")]
    UnknownInstance(String),
    #[error("media-graph edge {edge_id} type mismatch: producer is `{produced}`, consumer accepts `{accepted}`")]
    Incompatible { edge_id: String, produced: String, accepted: String },
    #[error("media graph has a cycle (unreachable nodes: {0:?})")]
    Cycle(Vec<String>),
    #[error("host error: {0}")]
    Host(String),
    #[error("media error: {0}")]
    Media(#[from] MediaError),
    #[error("io error at {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("(de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
//#endregion 🔖Types

//#region 🔖MediaNodeHost
/// 🔌 The one seam `StudioRunner` calls through — every concrete plugin host (native wasmtime,
/// browser worker, or an in-process fake for tests) implements this the same way. `node` is an
/// opaque handle the host mints in `instantiate` and the runner threads back on every later call.
pub trait MediaNodeHost {
    fn instantiate(&mut self, app_id: &str) -> Result<u32, RunError>;
    fn load_document(&mut self, node: u32, document_json: &str) -> Result<(), RunError>;
    fn import_media(&mut self, node: u32, port: &str, media: &Media) -> Result<(), RunError>;
    fn export_media(&mut self, node: u32, port: &str) -> Result<Media, RunError>;
    fn media_fingerprint(&mut self, node: u32, port: &str) -> Result<MediaFingerprint, RunError>;
    fn read_document(&mut self, node: u32) -> Result<String, RunError>;
}
//#endregion 🔖MediaNodeHost

//#region 🔖MediaCache
/// 📦 Content-addressed cache of exported `Media` values, keyed by `MediaFingerprint`. Lets a
/// downstream dirty node import a clean upstream node's last output without re-instantiating that
/// upstream node at all — the whole point of fingerprint-based incrementality.
pub trait MediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media>;
    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media);
}

/// 🧠 Process-local `MediaCache` — sufficient for a single `run()` call; nothing survives the process.
#[derive(Default)]
pub struct InMemoryMediaCache {
    entries: HashMap<String, Media>,
}

impl MediaCache for InMemoryMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        self.entries.get(&fingerprint.0).cloned()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        self.entries.insert(fingerprint.0.clone(), media.clone());
    }
}

/// 💾 Disk-backed `MediaCache` under `<studio>/run/media/<fingerprint>.json` — the persistent
/// counterpart to `InMemoryMediaCache`, so a cold-started runner still skips re-exporting a clean
/// node's output when a prior run already cached it.
pub struct FileMediaCache {
    root: PathBuf,
}

impl FileMediaCache {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn entry_path(&self, fingerprint: &MediaFingerprint) -> PathBuf {
        self.root.join(format!("{}.json", fingerprint.0))
    }
}

impl MediaCache for FileMediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
        let text = std::fs::read_to_string(self.entry_path(fingerprint)).ok()?;
        serde_json::from_str(&text).ok()
    }

    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
        if std::fs::create_dir_all(&self.root).is_err() {
            return;
        }
        if let Ok(text) = serde_json::to_string(media) {
            let _ = std::fs::write(self.entry_path(fingerprint), text);
        }
    }
}
//#endregion 🔖MediaCache

//#region 🔖RunState
/// 📇 Everything the runner remembers about one media-graph node between runs: the document
/// fingerprint that produced its current outputs, and the fingerprints of its inputs and outputs at
/// that time. A node is dirty iff any of these three no longer match reality.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRunRecord {
    pub document_fingerprint: String,
    pub input_fingerprints: BTreeMap<String, String>,
    pub output_fingerprints: BTreeMap<String, String>,
}

/// 🗄️ The runner's persisted incremental-recompute state for one studio bundle, keyed by media-graph
/// node id (not instance id — a node's record is tied to its position in the graph).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunState {
    pub nodes: BTreeMap<String, NodeRunRecord>,
}

impl RunState {
    pub fn load(path: &Path) -> Result<Self, RunError> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(serde_json::from_str(&text)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(source) => Err(RunError::Io { path: path.to_path_buf(), source }),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), RunError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(path, text).map_err(|source| RunError::Io { path: path.to_path_buf(), source })
    }
}
//#endregion 🔖RunState

//#region 🔖StudioBundle
/// 📁 The on-disk shape of a studio: `studio.os.json` (the `OsDocument` VCS envelope), one plain
/// document per app instance under `documents/`, and the runner's own `run/state.json` +
/// `run/media/` cache. Ids only — no paths inside `studio.os.json` itself — so the bundle is
/// relocatable and syncs the same way over `file://` or a hub backbone.
pub struct StudioBundle {
    root: PathBuf,
}

impl StudioBundle {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn studio_document_path(&self) -> PathBuf {
        self.root.join("studio.os.json")
    }

    pub fn document_path(&self, document_id: &str) -> PathBuf {
        self.root.join("documents").join(format!("{document_id}.json"))
    }

    pub fn run_state_path(&self) -> PathBuf {
        self.root.join("run").join("state.json")
    }

    pub fn media_cache_dir(&self) -> PathBuf {
        self.root.join("run").join("media")
    }

    pub fn read_studio_document(&self) -> Result<String, RunError> {
        let path = self.studio_document_path();
        std::fs::read_to_string(&path).map_err(|source| RunError::Io { path, source })
    }

    pub fn read_document(&self, document_id: &str) -> Result<String, RunError> {
        let path = self.document_path(document_id);
        std::fs::read_to_string(&path).map_err(|source| RunError::Io { path, source })
    }

    pub fn write_document(&self, document_id: &str, document_json: &str) -> Result<(), RunError> {
        let path = self.document_path(document_id);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&path, document_json).map_err(|source| RunError::Io { path, source })
    }

    pub fn load_run_state(&self) -> Result<RunState, RunError> {
        RunState::load(&self.run_state_path())
    }

    pub fn save_run_state(&self, state: &RunState) -> Result<(), RunError> {
        state.save(&self.run_state_path())
    }

    pub fn media_cache(&self) -> FileMediaCache {
        FileMediaCache::new(self.media_cache_dir())
    }
}
//#endregion 🔖StudioBundle

//#region 🔖Topology
/// 🔢 Deterministic topological order (Kahn's algorithm, lexicographically-smallest-ready-node-first)
/// over `graph`'s nodes. `Err(RunError::Cycle)` names whichever nodes never became ready — the media
/// graph's own `validate_media_graph` should be called first to reject cycles with a friendlier
/// message; this is the runner's authoritative order once that check has passed.
fn topological_order(graph: &OsMediaGraph) -> Result<Vec<String>, RunError> {
    let mut indegree: BTreeMap<String, usize> = graph.nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        *indegree.entry(edge.target_node_id.clone()).or_insert(0) += 1;
        outgoing.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut ready: BTreeSet<String> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| id.clone()).collect();
    let mut order = Vec::with_capacity(graph.nodes.len());
    while let Some(node_id) = ready.iter().next().cloned() {
        ready.remove(&node_id);
        order.push(node_id.clone());
        for next in outgoing.get(&node_id).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(next) {
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(next.clone());
                }
            }
        }
    }
    if order.len() != graph.nodes.len() {
        let unreached: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).filter(|id| !order.contains(id)).collect();
        return Err(RunError::Cycle(unreached));
    }
    Ok(order)
}
//#endregion 🔖Topology

//#region 🔖StudioRunner
/// 📊 What actually happened in one `run()` call — which nodes were recomputed and which were left
/// untouched because neither their document nor their inputs changed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunReport {
    pub recomputed: Vec<String>,
    pub clean: Vec<String>,
}

/// 🩺 Computes which nodes `StudioRunner::run` would recompute, without instantiating a single host
/// — the `--dry` plan. Reuses exactly the dirty check `run` applies, so the plan can never drift
/// from what an actual run would do.
pub fn plan(graph: &OsMediaGraph, documents: &BTreeMap<String, String>, state: &RunState) -> Result<RunReport, RunError> {
    StudioRunner::<NullHost>::validate_edge_kinds(graph)?;
    let order = topological_order(graph)?;
    let node_by_id: HashMap<&str, &OsMediaGraphNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming: HashMap<&str, Vec<&semio_framework_os::OsMediaGraphEdge>> = HashMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
    }
    let mut report = RunReport::default();
    for node_id in &order {
        let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
        let document_json = documents.get(&node.instance_id).cloned().unwrap_or_default();
        let document_fingerprint = semio_framework_hash::hash_bytes(document_json.as_bytes());
        let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
        for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
            let fingerprint = state.nodes.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.get(&edge.source_port_id)).cloned().unwrap_or_default();
            input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
        }
        let dirty = match state.nodes.get(node_id.as_str()) {
            None => true,
            Some(record) => record.document_fingerprint != document_fingerprint || record.input_fingerprints != input_fingerprints,
        };
        if dirty {
            report.recomputed.push(node_id.clone());
        } else {
            report.clean.push(node_id.clone());
        }
    }
    Ok(report)
}

/// 🚫 A `MediaNodeHost` that always errors — only ever used as `plan`'s unreachable type parameter
/// so it can call `StudioRunner`'s edge-validation helper without needing a real host.
pub struct NullHost;
impl MediaNodeHost for NullHost {
    fn instantiate(&mut self, _app_id: &str) -> Result<u32, RunError> {
        Err(RunError::Host("NullHost never instantiates".into()))
    }
    fn load_document(&mut self, _node: u32, _document_json: &str) -> Result<(), RunError> {
        Err(RunError::Host("NullHost never loads".into()))
    }
    fn import_media(&mut self, _node: u32, _port: &str, _media: &Media) -> Result<(), RunError> {
        Err(RunError::Host("NullHost never imports".into()))
    }
    fn export_media(&mut self, _node: u32, _port: &str) -> Result<Media, RunError> {
        Err(RunError::Host("NullHost never exports".into()))
    }
    fn media_fingerprint(&mut self, _node: u32, _port: &str) -> Result<MediaFingerprint, RunError> {
        Err(RunError::Host("NullHost never fingerprints".into()))
    }
    fn read_document(&mut self, _node: u32) -> Result<String, RunError> {
        Err(RunError::Host("NullHost never reads".into()))
    }
}

/// 🕸️ Computes one studio's media graph against a `MediaNodeHost`. Node dirtiness is decided purely
/// from `NodeRunRecord`: the document's own fingerprint (did the app's document change since last
/// run — e.g. a UI edit) and its resolved input fingerprints (did anything upstream change). A clean
/// node is never instantiated at all; its cached output fingerprints feed straight into its consumers.
pub struct StudioRunner<H: MediaNodeHost> {
    host: H,
}

impl<H: MediaNodeHost> StudioRunner<H> {
    pub fn new(host: H) -> Self {
        Self { host }
    }

    pub fn into_host(self) -> H {
        self.host
    }

    /// 🩹 Baseline wire-compatibility check: plain `resource_kind` string equality. `OsMediaPort`
    /// doesn't carry a typed `MediaType` yet (that unification is a separate, concurrently in-flight
    /// ticket) — once it does, this is where `media_types_compatible` conversion-insertion lands.
    fn validate_edge_kinds(graph: &OsMediaGraph) -> Result<(), RunError> {
        let node_by_id: HashMap<&str, &OsMediaGraphNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        for edge in &graph.edges {
            let produced = node_by_id
                .get(edge.source_node_id.as_str())
                .and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id))
                .ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
            let accepted = node_by_id
                .get(edge.target_node_id.as_str())
                .and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id))
                .ok_or_else(|| RunError::UnknownNode(edge.target_node_id.clone()))?;
            if produced.resource_kind != accepted.resource_kind {
                return Err(RunError::Incompatible { edge_id: edge.id.clone(), produced: produced.resource_kind.clone(), accepted: accepted.resource_kind.clone() });
            }
        }
        Ok(())
    }

    /// 🕸️ Runs every dirty node in `graph`'s topological order, importing media across each edge and
    /// persisting mutated documents back into `documents`. `documents` maps app-instance id → current
    /// document json; the returned map has the same keys, updated wherever a node actually ran.
    pub fn run(
        &mut self,
        graph: &OsMediaGraph,
        instances: &[OsAppInstance],
        documents: &BTreeMap<String, String>,
        state: &mut RunState,
        cache: &mut dyn MediaCache,
    ) -> Result<(BTreeMap<String, String>, RunReport), RunError> {
        Self::validate_edge_kinds(graph)?;
        let order = topological_order(graph)?;
        let node_by_id: HashMap<&str, &OsMediaGraphNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let instance_by_id: HashMap<&str, &OsAppInstance> = instances.iter().map(|instance| (instance.id.as_str(), instance)).collect();
        let mut incoming: HashMap<&str, Vec<&semio_framework_os::OsMediaGraphEdge>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
        }

        let mut documents_out = documents.clone();
        let mut report = RunReport::default();
        let mut live: HashMap<String, u32> = HashMap::new();

        for node_id in &order {
            let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
            let instance = *instance_by_id.get(node.instance_id.as_str()).ok_or_else(|| RunError::UnknownInstance(node.instance_id.clone()))?;
            let document_json = documents_out.get(&instance.id).cloned().unwrap_or_default();
            let document_fingerprint = semio_framework_hash::hash_bytes(document_json.as_bytes());

            let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let source_record = state.nodes.get(&edge.source_node_id);
                let fingerprint = source_record.and_then(|record| record.output_fingerprints.get(&edge.source_port_id)).cloned().unwrap_or_default();
                input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
            }

            let previous = state.nodes.get(node_id.as_str());
            let dirty = match previous {
                None => true,
                Some(record) => record.document_fingerprint != document_fingerprint || record.input_fingerprints != input_fingerprints,
            };

            if !dirty {
                report.clean.push(node_id.clone());
                continue;
            }
            report.recomputed.push(node_id.clone());

            let handle = *live
                .entry(node_id.clone())
                .or_insert(self.host.instantiate(&instance.app_id).map_err(|error| RunError::Host(error.to_string()))?);
            self.host.load_document(handle, &document_json)?;

            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = MediaFingerprint(input_fingerprints.get(&edge.target_port_id).cloned().unwrap_or_default());
                let media = match cache.get(&fingerprint) {
                    Some(media) => media,
                    None => {
                        let source_handle = *live.entry(edge.source_node_id.clone()).or_insert({
                            let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
                            let source_instance = *instance_by_id.get(source_node.instance_id.as_str()).ok_or_else(|| RunError::UnknownInstance(source_node.instance_id.clone()))?;
                            let source_handle = self.host.instantiate(&source_instance.app_id)?;
                            self.host.load_document(source_handle, documents_out.get(&source_instance.id).map(String::as_str).unwrap_or_default())?;
                            source_handle
                        });
                        let media = self.host.export_media(source_handle, &edge.source_port_id)?;
                        cache.put(&fingerprint, &media);
                        media
                    }
                };
                self.host.import_media(handle, &edge.target_port_id, &media)?;
            }

            let mut output_fingerprints = BTreeMap::new();
            for port in &node.outputs {
                let fingerprint = self.host.media_fingerprint(handle, &port.id)?;
                output_fingerprints.insert(port.id.clone(), fingerprint.0.clone());
            }

            let mutated_document = self.host.read_document(handle)?;
            documents_out.insert(instance.id.clone(), mutated_document);
            state.nodes.insert(node_id.clone(), NodeRunRecord { document_fingerprint, input_fingerprints, output_fingerprints });
        }

        Ok((documents_out, report))
    }
}
//#endregion 🔖StudioRunner

//#region 🔖WasmtimeNodeHost
/// 🧩 Native `MediaNodeHost` over `semio-framework-plugin-host`'s wasmtime runtime. `instantiate` /
/// `load_document` / `read_document` are real today. `import_media`/`export_media`/
/// `media_fingerprint` are **not yet wired** — `world.wit` and both plugin hosts don't expose those
/// three calls on the wire yet (a deliberately separate, follow-up ticket once the concurrently
/// in-flight media-lattice/reconcile tickets land); they return `RunError::Host` naming the gap
/// rather than silently doing nothing.
#[cfg(not(target_arch = "wasm32"))]
pub struct WasmtimeNodeHost {
    runtimes: HashMap<String, semio_framework_plugin_host::WasmPluginRuntime>,
    plugin_path_for_app: HashMap<String, PathBuf>,
    next_handle: u32,
    instances: HashMap<u32, (String, u32)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl WasmtimeNodeHost {
    /// 🗺️ `plugin_path_for_app` maps an app id to the compiled `.wasm` component path the dev-shell
    /// build already produces under `framework/product/os/dev/plugin-modules/<app>/`.
    pub fn new(plugin_path_for_app: HashMap<String, PathBuf>) -> Self {
        Self { runtimes: HashMap::new(), plugin_path_for_app, next_handle: 1, instances: HashMap::new() }
    }

    fn runtime_for(&mut self, app_id: &str) -> Result<&semio_framework_plugin_host::WasmPluginRuntime, RunError> {
        if !self.runtimes.contains_key(app_id) {
            let path = self.plugin_path_for_app.get(app_id).ok_or_else(|| RunError::Host(format!("no compiled plugin registered for app `{app_id}`")))?;
            let runtime = semio_framework_plugin_host::WasmPluginRuntime::load(path).map_err(|error| RunError::Host(error.to_string()))?;
            self.runtimes.insert(app_id.to_string(), runtime);
        }
        Ok(self.runtimes.get(app_id).expect("just inserted"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl MediaNodeHost for WasmtimeNodeHost {
    fn instantiate(&mut self, app_id: &str) -> Result<u32, RunError> {
        let instance_id = self.runtime_for(app_id)?.create_app(app_id).map_err(|error| RunError::Host(error.to_string()))?;
        let handle = self.next_handle;
        self.next_handle += 1;
        self.instances.insert(handle, (app_id.to_string(), instance_id));
        Ok(handle)
    }

    fn load_document(&mut self, node: u32, document_json: &str) -> Result<(), RunError> {
        let (app_id, instance_id) = self.instances.get(&node).ok_or_else(|| RunError::Host(format!("unknown node handle {node}")))?;
        self.runtimes.get(app_id).ok_or_else(|| RunError::Host(format!("no runtime for `{app_id}`")))?.load_app_document(*instance_id, document_json).map_err(|error| RunError::Host(error.to_string()))
    }

    fn import_media(&mut self, _node: u32, port: &str, _media: &Media) -> Result<(), RunError> {
        Err(RunError::Host(format!("import-media (`{port}`) is not yet on the plugin WIT surface — see HEADLESS-MEDIA-CONTRACT follow-up")))
    }

    fn export_media(&mut self, _node: u32, port: &str) -> Result<Media, RunError> {
        Err(RunError::Host(format!("export-media (`{port}`) is not yet on the plugin WIT surface — see HEADLESS-MEDIA-CONTRACT follow-up")))
    }

    fn media_fingerprint(&mut self, _node: u32, port: &str) -> Result<MediaFingerprint, RunError> {
        Err(RunError::Host(format!("media-fingerprint (`{port}`) is not yet on the plugin WIT surface — see HEADLESS-MEDIA-CONTRACT follow-up")))
    }

    fn read_document(&mut self, node: u32) -> Result<String, RunError> {
        let (app_id, instance_id) = self.instances.get(&node).ok_or_else(|| RunError::Host(format!("unknown node handle {node}")))?;
        self.runtimes.get(app_id).ok_or_else(|| RunError::Host(format!("no runtime for `{app_id}`")))?.read_app_document(*instance_id).map_err(|error| RunError::Host(error.to_string()))
    }
}
//#endregion 🔖WasmtimeNodeHost

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_core::MediaPayload;
    use semio_framework_os::{placeholder_media_contract, OsMediaGraphEdge, OsMediaPort};

    /// 🧪 A fake `MediaNodeHost` for tests: no wasm at all, just a per-instance document string and
    /// a fixed structured output per port, so `StudioRunner`'s dirty/clean bookkeeping can be
    /// exercised without a real plugin.
    #[derive(Default)]
    /// 🧪 Outputs are keyed by app id, not by handle — a real app's export is a function of its
    /// document/logic, not of the ephemeral instance handle a host happens to mint this call, and a
    /// node genuinely does get re-instantiated (a fresh handle) on every dirty re-run.
    struct FakeHost {
        documents: HashMap<u32, String>,
        handle_app: HashMap<u32, String>,
        outputs: HashMap<(String, String), Media>,
        next: u32,
        imported: Vec<(u32, String, Media)>,
    }

    impl FakeHost {
        fn set_output(&mut self, app_id: &str, port: &str, json: &str) {
            self.outputs.insert((app_id.to_string(), port.to_string()), Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: json.into() } });
        }
    }

    fn fake_media_type() -> semio_framework_core::MediaType {
        semio_framework_core::MediaType { class: semio_framework_core::MediaClass::Data, form: semio_framework_core::MediaForm::Value }
    }

    impl MediaNodeHost for FakeHost {
        fn instantiate(&mut self, app_id: &str) -> Result<u32, RunError> {
            self.next += 1;
            self.handle_app.insert(self.next, app_id.to_string());
            Ok(self.next)
        }
        fn load_document(&mut self, node: u32, document_json: &str) -> Result<(), RunError> {
            self.documents.insert(node, document_json.to_string());
            Ok(())
        }
        fn import_media(&mut self, node: u32, port: &str, media: &Media) -> Result<(), RunError> {
            self.imported.push((node, port.to_string(), media.clone()));
            Ok(())
        }
        fn export_media(&mut self, node: u32, port: &str) -> Result<Media, RunError> {
            let app_id = self.handle_app.get(&node).cloned().unwrap_or_default();
            self.outputs.get(&(app_id, port.to_string())).cloned().ok_or_else(|| RunError::Host("no output".into()))
        }
        fn media_fingerprint(&mut self, node: u32, port: &str) -> Result<MediaFingerprint, RunError> {
            self.export_media(node, port).map(|media| MediaFingerprint::of(&media))
        }
        fn read_document(&mut self, node: u32) -> Result<String, RunError> {
            Ok(self.documents.get(&node).cloned().unwrap_or_default())
        }
    }

    fn two_node_graph() -> (OsMediaGraph, Vec<OsAppInstance>) {
        let source = OsMediaGraphNode {
            id: "node-a".into(),
            instance_id: "instance-a".into(),
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: Vec::new(),
            outputs: vec![OsMediaPort { id: "out".into(), resource_kind: "data.value".into(), direction: "out".into() }],
        };
        let target = OsMediaGraphNode {
            id: "node-b".into(),
            instance_id: "instance-b".into(),
            x: 1.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            inputs: vec![OsMediaPort { id: "in".into(), resource_kind: "data.value".into(), direction: "in".into() }],
            outputs: Vec::new(),
        };
        let edge = OsMediaGraphEdge { id: "edge-1".into(), source_node_id: "node-a".into(), source_port_id: "out".into(), target_node_id: "node-b".into(), target_port_id: "in".into(), contract: placeholder_media_contract("data.value") };
        let graph = OsMediaGraph { schema: "s.media-graph".into(), nodes: vec![source, target], edges: vec![edge] };
        let instances = vec![
            OsAppInstance { id: "instance-a".into(), program_id: "program".into(), app_id: "app-a".into(), label: "A".into(), yields: "data.value".into(), document: semio_framework_os::OsDocumentRef { document_id: "instance-a".into(), schema: "app-a.document".into() } },
            OsAppInstance { id: "instance-b".into(), program_id: "program".into(), app_id: "app-b".into(), label: "B".into(), yields: "".into(), document: semio_framework_os::OsDocumentRef { document_id: "instance-b".into(), schema: "app-b.document".into() } },
        ];
        (graph, instances)
    }

    #[test]
    fn topological_order_respects_edges() {
        let (graph, _) = two_node_graph();
        let order = topological_order(&graph).expect("acyclic");
        assert_eq!(order, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn detects_cycles() {
        let (mut graph, _) = two_node_graph();
        graph.edges.push(OsMediaGraphEdge { id: "edge-2".into(), source_node_id: "node-b".into(), source_port_id: "in".into(), target_node_id: "node-a".into(), target_port_id: "out".into(), contract: placeholder_media_contract("data.value") });
        assert!(matches!(topological_order(&graph), Err(RunError::Cycle(_))));
    }

    #[test]
    fn first_run_recomputes_every_node_second_run_is_a_no_op() {
        let (graph, instances) = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-a", "out", "\"hello\"");
        let mut runner = StudioRunner::new(host);
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents: BTreeMap<String, String> = [("instance-a".to_string(), "{}".to_string()), ("instance-b".to_string(), "{}".to_string())].into();

        let (documents_1, report_1) = runner.run(&graph, &instances, &documents, &mut state, &mut cache).expect("first run");
        assert_eq!(report_1.recomputed, vec!["node-a".to_string(), "node-b".to_string()]);
        assert!(report_1.clean.is_empty());

        let (_, report_2) = runner.run(&graph, &instances, &documents_1, &mut state, &mut cache).expect("second run");
        assert!(report_2.recomputed.is_empty(), "unchanged documents must not re-trigger recompute: {:?}", report_2.recomputed);
        assert_eq!(report_2.clean, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn editing_upstream_document_dirties_downstream_only_through_the_wire() {
        let (graph, instances) = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-a", "out", "\"hello\"");
        let mut runner = StudioRunner::new(host);
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents: BTreeMap<String, String> = [("instance-a".to_string(), "{}".to_string()), ("instance-b".to_string(), "{}".to_string())].into();
        let (documents_1, _) = runner.run(&graph, &instances, &documents, &mut state, &mut cache).expect("first run");

        let mut documents_2 = documents_1;
        documents_2.insert("instance-a".to_string(), "{\"edited\":true}".to_string());
        let (_, report_2) = runner.run(&graph, &instances, &documents_2, &mut state, &mut cache).expect("second run");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string(), "node-b".to_string()], "node-a's document changed, and node-a's fixed FakeHost output means node-b's input fingerprint is unchanged, but node-a itself must still recompute");
    }

    #[test]
    fn rejects_mismatched_edge_resource_kinds() {
        let (mut graph, instances) = two_node_graph();
        graph.nodes[1].inputs[0].resource_kind = "other.kind".into();
        let host = FakeHost::default();
        let mut runner = StudioRunner::new(host);
        let mut state = RunState::default();
        let mut cache = InMemoryMediaCache::default();
        let documents: BTreeMap<String, String> = [("instance-a".to_string(), "{}".to_string()), ("instance-b".to_string(), "{}".to_string())].into();
        let result = runner.run(&graph, &instances, &documents, &mut state, &mut cache);
        assert!(matches!(result, Err(RunError::Incompatible { .. })));
    }
}
//#endregion 🔖Tests
