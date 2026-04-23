#[cfg(not(target_arch = "wasm32"))]
pub mod kit_conflict_registry {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use serde::{Deserialize, Serialize};

    use crate::id::Id;
    use crate::kit_checkpoint::KitCheckpoint;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ConflictResolution {
        DropWip,
        ForceOverwriteBackbone,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct KitConflict {
        pub id: Id,
        pub wip_checkpoint: KitCheckpoint,
        pub backbone_tip: Option<Id>,
        pub reason: String,
        pub created_at: String,
    }

    #[derive(Clone, Debug, Default)]
    pub struct ConflictRegistry {
        inner: Arc<Mutex<HashMap<Id, KitConflict>>>,
    }

    impl ConflictRegistry {
        pub fn new() -> Self {
            Self { inner: Arc::new(Mutex::new(HashMap::new())) }
        }

        pub fn insert(&self, c: KitConflict) {
            let mut g = self.inner.lock().expect("conflict registry lock");
            g.insert(c.id.clone(), c);
        }

        pub fn list(&self) -> Vec<KitConflict> {
            let g = self.inner.lock().expect("conflict registry lock");
            let mut v: Vec<KitConflict> = g.values().cloned().collect();
            v.sort_by(|a, b| a.as_str().cmp(b.as_str()));
            v
        }

        pub fn remove(&self, id: &Id) -> Option<KitConflict> {
            let mut g = self.inner.lock().expect("conflict registry lock");
            g.remove(id)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod backbone {
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    use crate::error::{Result, SemioError};
    use crate::id::Id;
    use crate::kit_alternative::KitAlternative;
    use crate::kit_checkpoint::KitCheckpoint;
    use crate::kit_graph::{KitFullDto, KitGraph, KitGraphRef};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BackboneSnapshot {
        pub initial: KitFullDto,
        pub checkpoints: HashMap<Id, KitCheckpoint>,
        pub the_kit_head: Option<Id>,
        pub alternatives: HashMap<Id, KitAlternative>,
        pub children: HashMap<Option<Id>, Vec<Id>>,
    }

    impl BackboneSnapshot {
        pub fn from_graph(g: &KitGraph) -> Self {
            Self {
                initial: g.initial.clone(),
                checkpoints: g.checkpoints.clone(),
                the_kit_head: g.the_kit_head.clone(),
                alternatives: g.alternatives.clone(),
                children: g.children.clone(),
            }
        }

        pub fn verify_linear_mainline_extension(auth: &Self, proposed: &Self) -> std::result::Result<(), String> {
            if auth.initial != proposed.initial {
                return Err("initial kit mismatch".into());
            }
            for (id, cp) in &auth.checkpoints {
                match proposed.checkpoints.get(id) {
                    Some(p) if p == cp => {}
                    _ => return Err(format!("checkpoint {id} missing or diverged")),
                }
            }
            let tip = proposed.the_kit_head.clone().ok_or_else(|| "proposed has no the_kit_head".to_string())?;
            let ncp = proposed.checkpoints.get(&tip).ok_or_else(|| "tip checkpoint missing".to_string())?;
            if ncp.parent != auth.the_kit_head {
                return Err(format!(
                    "checkpoint parent {:?} != backbone tip {:?}",
                    ncp.parent, auth.the_kit_head
                ));
            }
            let new_ids: HashSet<_> = proposed.checkpoints.keys().filter(|k| !auth.checkpoints.contains_key(*k)).cloned().collect();
            if new_ids.len() != 1 || !new_ids.contains(&tip) {
                return Err("expected exactly one new checkpoint on main line".into());
            }
            Ok(())
        }
    }

    pub fn apply_snap_to_graph(kit: &KitGraphRef, snap: &BackboneSnapshot) -> crate::error::SetResult {
        use crate::error::SetError;
        let dto = KitCheckpoint::materialize(&snap.initial, &snap.checkpoints, snap.the_kit_head.as_ref());
        KitGraph::replace_from_full_dto(kit, dto)?;
        let mut g = kit.write().map_err(|_| SetError::LockPoisoned("kit".into()))?;
        g.initial = snap.initial.clone();
        g.checkpoints = snap.checkpoints.clone();
        g.the_kit_head = snap.the_kit_head.clone();
        g.alternatives = snap.alternatives.clone();
        g.children = snap.children.clone();
        g.sessions.clear();
        Ok(())
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum BackboneConfig {
        Dev { path: String },
        Local { folder: String },
        Remote { url: String, session_id: String },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ProposeOutcome {
        Accepted,
        Rejected { current_tip: Option<Id> },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BackboneStatusDto {
        pub attached: bool,
        pub kind: Option<String>,
        pub backbone_tip: Option<String>,
        pub pending_wip_checkpoints: usize,
    }

    pub trait BackboneDriver: Send {
        fn pull(&self) -> Result<BackboneSnapshot>;
        fn persist_full(&self, proposed: &BackboneSnapshot) -> Result<ProposeOutcome>;
        fn set_active_checkpoint(&self, _id: Option<&Id>) -> Result<()> {
            Ok(())
        }
    }

    pub struct DevBackbone {
        path: PathBuf,
    }

    impl DevBackbone {
        pub fn new(path: PathBuf) -> Self {
            Self { path }
        }

        fn read_file(&self) -> Result<BackboneSnapshot> {
            if !self.path.exists() {
                return Err(SemioError::InvalidOperation(format!(
                    "dev backbone file missing: {}",
                    self.path.display()
                )));
            }
            let s = std::fs::read_to_string(&self.path).map_err(|e| SemioError::Other(e.to_string()))?;
            let mut snap: BackboneSnapshot = serde_json::from_str(&s).map_err(|e| SemioError::Other(e.to_string()))?;
            if snap.children.is_empty() && !snap.checkpoints.is_empty() {
                snap.children = KitCheckpoint::rebuild_children_map(&snap.checkpoints);
            }
            Ok(snap)
        }

        fn write_atomic(&self, snap: &BackboneSnapshot) -> Result<()> {
            if let Some(parent) = self.path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).map_err(|e| SemioError::Other(e.to_string()))?;
                }
            }
            let dir = self.path.parent().unwrap_or_else(|| Path::new("."));
            let mut tmp = tempfile::NamedTempFile::new_in(dir).map_err(|e| SemioError::Other(e.to_string()))?;
            let json = serde_json::to_string_pretty(snap).map_err(|e| SemioError::Other(e.to_string()))?;
            std::io::Write::write_all(&mut tmp, json.as_bytes()).map_err(|e| SemioError::Other(e.to_string()))?;
            tmp.flush().map_err(|e| SemioError::Other(e.to_string()))?;
            tmp.persist(&self.path).map_err(|e| SemioError::Other(e.error.to_string()))?;
            Ok(())
        }
    }

    impl BackboneDriver for DevBackbone {
        fn pull(&self) -> Result<BackboneSnapshot> {
            self.read_file()
        }

        fn persist_full(&self, proposed: &BackboneSnapshot) -> Result<ProposeOutcome> {
            let auth = if self.path.exists() {
                self.read_file()?
            } else {
                BackboneSnapshot {
                    initial: proposed.initial.clone(),
                    checkpoints: HashMap::new(),
                    the_kit_head: None,
                    alternatives: HashMap::new(),
                    children: HashMap::new(),
                }
            };
            if BackboneSnapshot::verify_linear_mainline_extension(&auth, proposed).is_err() {
                return Ok(ProposeOutcome::Rejected {
                    current_tip: auth.the_kit_head.clone(),
                });
            }
            if self.path.exists() {
                let auth2 = self.read_file()?;
                if auth2.the_kit_head != auth.the_kit_head {
                    return Ok(ProposeOutcome::Rejected {
                        current_tip: auth2.the_kit_head.clone(),
                    });
                }
            }
            self.write_atomic(proposed)?;
            Ok(ProposeOutcome::Accepted)
        }
    }

    pub struct LocalBackbone {
        folder: PathBuf,
    }

    impl LocalBackbone {
        pub fn new(folder: PathBuf) -> Self {
            Self { folder }
        }
    }

    impl BackboneDriver for LocalBackbone {
        fn pull(&self) -> Result<BackboneSnapshot> {
            let k = KitGraph::load_local_folder(&self.folder)?;
            let g = k.read().map_err(|_| SemioError::LockPoisoned("kit".into()))?;
            Ok(BackboneSnapshot::from_graph(&g))
        }

        fn persist_full(&self, proposed: &BackboneSnapshot) -> Result<ProposeOutcome> {
            let auth = if self.folder.join(".semio").join("kit.db").exists() {
                self.pull()?
            } else {
                BackboneSnapshot {
                    initial: proposed.initial.clone(),
                    checkpoints: HashMap::new(),
                    the_kit_head: None,
                    alternatives: HashMap::new(),
                    children: HashMap::new(),
                }
            };
            if BackboneSnapshot::verify_linear_mainline_extension(&auth, proposed).is_err() {
                return Ok(ProposeOutcome::Rejected {
                    current_tip: auth.the_kit_head.clone(),
                });
            }
            let dto = KitCheckpoint::materialize(&proposed.initial, &proposed.checkpoints, proposed.the_kit_head.as_ref());
            let k = KitGraph::from_full_dto(dto);
            {
                let mut g = k.write().map_err(|_| SemioError::LockPoisoned("kit".into()))?;
                g.initial = proposed.initial.clone();
                g.checkpoints = proposed.checkpoints.clone();
                g.the_kit_head = proposed.the_kit_head.clone();
                g.alternatives = proposed.alternatives.clone();
                g.children = proposed.children.clone();
            }
            k.read().map_err(|_| SemioError::LockPoisoned("kit".into()))?
                .save_local_folder(&self.folder)?;
            Ok(ProposeOutcome::Accepted)
        }

        fn set_active_checkpoint(&self, id: Option<&Id>) -> Result<()> {
            let k = KitGraph::load_local_folder(&self.folder)?;
            let dto = {
                let g = k.read().map_err(|_| SemioError::LockPoisoned("kit".into()))?;
                KitCheckpoint::materialize(&g.initial, &g.checkpoints, id)
            };
            let k2 = KitGraph::from_full_dto(dto);
            k2.read().map_err(|_| SemioError::LockPoisoned("kit".into()))?
                .save_local_folder(&self.folder)?;
            Ok(())
        }
    }

    pub struct RemoteBackbone {
        hub_url: String,
        session_id: String,
    }

    impl RemoteBackbone {
        pub fn new(hub_url: String, session_id: String) -> Self {
            Self { hub_url, session_id }
        }
    }

    impl BackboneDriver for RemoteBackbone {
        fn pull(&self) -> Result<BackboneSnapshot> {
            let k = KitGraph::load_remote_session(&self.hub_url, &self.session_id)?;
            let g = k.read().map_err(|_| SemioError::LockPoisoned("kit".into()))?;
            Ok(BackboneSnapshot::from_graph(&g))
        }

        fn persist_full(&self, _proposed: &BackboneSnapshot) -> Result<ProposeOutcome> {
            Err(SemioError::InvalidOperation(
                "Remote backbone commit requires an authenticated RemoteKitSession (owner token). Use Dev or Local backbone.".into(),
            ))
        }
    }

    pub enum BackboneKind {
        Dev(DevBackbone),
        Local(LocalBackbone),
        Remote(RemoteBackbone),
    }

    impl BackboneKind {
        pub fn from_config(cfg: &BackboneConfig) -> Result<Self> {
            match cfg {
                BackboneConfig::Dev { path } => Ok(Self::Dev(DevBackbone::new(PathBuf::from(path)))),
                BackboneConfig::Local { folder } => Ok(Self::Local(LocalBackbone::new(PathBuf::from(folder)))),
                BackboneConfig::Remote { url, session_id } => Ok(Self::Remote(RemoteBackbone::new(url.clone(), session_id.clone()))),
            }
        }

        pub fn pull(&self) -> Result<BackboneSnapshot> {
            match self {
                Self::Dev(d) => d.pull(),
                Self::Local(l) => l.pull(),
                Self::Remote(r) => r.pull(),
            }
        }

        pub fn persist_full(&self, proposed: &BackboneSnapshot) -> Result<ProposeOutcome> {
            match self {
                Self::Dev(d) => d.persist_full(proposed),
                Self::Local(l) => l.persist_full(proposed),
                Self::Remote(r) => r.persist_full(proposed),
            }
        }

        pub fn set_active_checkpoint(&self, id: Option<&Id>) -> Result<()> {
            match self {
                Self::Local(l) => l.set_active_checkpoint(id),
                _ => Ok(()),
            }
        }

        pub fn kind_name(&self) -> &'static str {
            match self {
                Self::Dev(_) => "dev",
                Self::Local(_) => "local",
                Self::Remote(_) => "remote",
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod wip_kit {
    use std::sync::mpsc::{Receiver, Sender};

    use crate::backbone::BackboneSnapshot;
    use crate::kit_graph::KitGraphRef;
    use crate::kit_store_command::{KitStoreCommand, KitStoreCommandResult};

    pub enum WipMsg {
        Exec {
            cmd: KitStoreCommand,
            reply: Sender<crate::error::Result<KitStoreCommandResult>>,
        },
        ApplyAuth {
            snap: BackboneSnapshot,
            reply: Sender<crate::error::SetResult>,
        },
    }

    pub enum CoordMsg {
        WipProposal {
            proposed: BackboneSnapshot,
            reply: Option<Sender<crate::error::Result<()>>>,
        },
        SyncNow,
        Attach {
            cfg: crate::backbone::BackboneConfig,
            reply: Sender<crate::error::Result<()>>,
        },
        Detach {
            reply: Sender<()>,
        },
        Status {
            reply: Sender<crate::backbone::BackboneStatusDto>,
        },
        SetActiveCheckpoint {
            id: Option<crate::id::Id>,
            reply: Sender<crate::error::Result<()>>,
        },
        ResolveConflict {
            id: crate::id::Id,
            strategy: crate::kit_conflict_registry::ConflictResolution,
            wip: KitGraphRef,
            reply: Sender<crate::error::Result<()>>,
        },
    }

    fn scan_finalize(graph: &KitGraphRef, res: &KitStoreCommandResult, coord: &Sender<CoordMsg>) {
        use crate::kit_draft::KitDraftCommandResult;
        use crate::kit_session::SessionCommandResult;
        if let KitStoreCommandResult::ExecuteSessionCommands { results } = res {
            for sr in results {
                if let SessionCommandResult::ExecuteKitDraftCommands { results } = sr {
                    for dr in results {
                        if let KitDraftCommandResult::FinalizeToKitCheckpoint { .. } = dr {
                            let proposed = {
                                let g = graph.read().expect("wip graph");
                                crate::backbone::BackboneSnapshot::from_graph(&g)
                            };
                            let _ = coord.send(CoordMsg::WipProposal { proposed, reply: None });
                        }
                    }
                }
            }
        }
    }

    pub fn run(graph: KitGraphRef, rx: Receiver<WipMsg>, coord: Sender<CoordMsg>) {
        while let Ok(msg) = rx.recv() {
            match msg {
                WipMsg::Exec { cmd, reply } => {
                    let r = cmd.execute(&graph);
                    if let Ok(ref ok) = r {
                        scan_finalize(&graph, ok, &coord);
                    }
                    let _ = reply.send(r);
                }
                WipMsg::ApplyAuth { snap, reply } => {
                    let r = crate::backbone::apply_snap_to_graph(&graph, &snap);
                    let _ = reply.send(r);
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod kit_coordinator {
    use std::sync::mpsc::Receiver;
    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::backbone::{apply_snap_to_graph, BackboneKind, BackboneSnapshot, ProposeOutcome};
    use crate::error::{Result, SemioError};
    use crate::kit_change::KitChange;
    use crate::kit_checkpoint::KitCheckpoint;
    use crate::kit_conflict_registry::{ConflictRegistry, KitConflict};
    use crate::kit_graph::{KitGraph, KitGraphRef};
    use crate::wip_kit::{CoordMsg, WipMsg};

    fn process_proposal(
        backbone: &BackboneKind,
        sync: &KitGraphRef,
        conflicts: &ConflictRegistry,
        proposed: &BackboneSnapshot,
        wip_tx: &std::sync::mpsc::Sender<WipMsg>,
    ) -> Result<()> {
        let auth = backbone.pull()?;
        apply_snap_to_graph(sync, &auth).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
        let tip = proposed.the_kit_head.clone().ok_or_else(|| SemioError::InvalidOperation("wip has no the_kit_head".into()))?;
        if auth.checkpoints.contains_key(&tip) {
            return Ok(());
        }
        let cp = proposed
            .checkpoints
            .get(&tip)
            .ok_or_else(|| SemioError::InvalidOperation("missing tip checkpoint".into()))?
            .clone();
        for ch in &cp.changes {
            if let Err(e) = KitChange::apply_forward(ch, sync) {
                conflicts.insert(KitConflict {
                    id: cp.id.clone(),
                    wip_checkpoint: cp.clone(),
                    backbone_tip: auth.the_kit_head.clone(),
                    reason: e.to_string(),
                    created_at: Utc::now().to_rfc3339(),
                });
                return Ok(());
            }
        }
        {
            let mut g = sync.write().map_err(|_| SemioError::LockPoisoned("sync"))?;
            g.checkpoints.insert(cp.id.clone(), cp.clone());
            g.children.entry(cp.parent.clone()).or_default().push(cp.id.clone());
            g.children
                .entry(cp.parent.clone())
                .or_default()
                .sort_by(|a, b| a.as_str().cmp(b.as_str()));
            g.the_kit_head = Some(cp.id.clone());
        }
        match backbone.persist_full(proposed)? {
            ProposeOutcome::Accepted => {
                let new_auth = backbone.pull()?;
                let (tx, rx) = std::sync::mpsc::channel();
                wip_tx
                    .send(WipMsg::ApplyAuth {
                        snap: new_auth,
                        reply: tx,
                    })
                    .map_err(|_| SemioError::InvalidOperation("wip channel closed".into()))?;
                rx.recv()
                    .map_err(|_| SemioError::InvalidOperation("wip apply auth".into()))?
                    .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
            }
            ProposeOutcome::Rejected { .. } => {}
        }
        Ok(())
    }

    pub fn run(
        backbone_slot: Arc<Mutex<Option<BackboneKind>>>,
        sync: KitGraphRef,
        pending: Arc<Mutex<usize>>,
        conflicts: Arc<ConflictRegistry>,
        wip_tx: std::sync::mpsc::Sender<WipMsg>,
        rx: Receiver<CoordMsg>,
    ) {
        while let Ok(msg) = rx.recv() {
            match msg {
                CoordMsg::WipProposal { proposed, reply } => {
                    *pending.lock().expect("p") += 1;
                    let res = (|| {
                        let g = backbone_slot.lock().expect("bb");
                        let Some(ref b) = *g else {
                            return Ok(());
                        };
                        process_proposal(b, &sync, &conflicts, &proposed, &wip_tx)
                    })();
                    *pending.lock().expect("p") = pending.lock().expect("p").saturating_sub(1);
                    if let Some(tx) = reply {
                        let _ = tx.send(res);
                    }
                }
                CoordMsg::SyncNow => {
                    let _ = backbone_slot.lock();
                }
                CoordMsg::Attach { cfg, reply } => {
                    let r = BackboneKind::from_config(&cfg).and_then(|k| {
                        let mut g = backbone_slot.lock().expect("bb");
                        *g = Some(k);
                        Ok(())
                    });
                    let _ = reply.send(r);
                }
                CoordMsg::Detach { reply } => {
                    let mut g = backbone_slot.lock().expect("bb");
                    *g = None;
                    let _ = reply.send(());
                }
                CoordMsg::Status { reply } => {
                    let g = backbone_slot.lock().expect("bb");
                    let (attached, kind, tip) = match &*g {
                        None => (false, None, None),
                        Some(b) => {
                            let t = b.pull().ok().and_then(|s| s.the_kit_head.map(|x| x.to_string()));
                            (true, Some(b.kind_name().to_string()), t)
                        }
                    };
                    let pend = *pending.lock().expect("p");
                    let _ = reply.send(crate::backbone::BackboneStatusDto {
                        attached,
                        kind,
                        backbone_tip: tip,
                        pending_wip_checkpoints: pend,
                    });
                }
                CoordMsg::SetActiveCheckpoint { id, reply } => {
                    let r = (|| {
                        let g = backbone_slot.lock().expect("bb");
                        let Some(ref b) = *g else {
                            return Err(SemioError::InvalidOperation("no backbone attached".into()));
                        };
                        b.set_active_checkpoint(id.as_ref())
                    })();
                    let _ = reply.send(r);
                }
                CoordMsg::ResolveConflict { id, strategy, wip, reply } => {
                    let r = (|| {
                        let c = conflicts.remove(&id).ok_or_else(|| SemioError::NotFound { kind: "KitConflict", id: id.clone() })?;
                        match strategy {
                            crate::kit_conflict_registry::ConflictResolution::DropWip => Ok(()),
                            crate::kit_conflict_registry::ConflictResolution::ForceOverwriteBackbone => {
                                let g = backbone_slot.lock().expect("bb");
                                let Some(ref b) = *g else {
                                    return Err(SemioError::InvalidOperation("no backbone".into()));
                                };
                                let auth = b.pull()?;
                                apply_snap_to_graph(&wip, &auth).map_err(|e| SemioError::InvalidOperation(e.to_string()))
                            }
                        }
                    })();
                    let _ = reply.send(r);
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod kit_store {
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread::JoinHandle;

    use async_broadcast::Receiver;

    use crate::backbone::{BackboneConfig, BackboneSnapshot};
    use crate::error::{Result, SemioError};
    use crate::kit_graph::{KitFullDto, KitGraph, KitGraphRef};
    use crate::kit_conflict_registry::{ConflictRegistry, ConflictResolution, KitConflict};
    use crate::kit_store_command::{KitStoreCommand, KitStoreCommandResult};
    use crate::wip_kit::{CoordMsg, WipMsg};

    /// Control plane: wip graph actor + coordinator + optional backbone attachment (native).
    pub struct KitStore {
        graph: KitGraphRef,
        wip_tx: mpsc::Sender<WipMsg>,
        coord_tx: mpsc::Sender<CoordMsg>,
        pub conflicts: Arc<ConflictRegistry>,
        backbone_slot: Arc<Mutex<Option<crate::backbone::BackboneKind>>>,
        pending: Arc<Mutex<usize>>,
        _handles: Vec<JoinHandle<()>>,
    }

    impl KitStore {
        pub fn from_full_dto(dto: KitFullDto) -> Self {
            let graph = KitGraph::from_full_dto(dto);
            let conflicts = Arc::new(ConflictRegistry::new());
            let backbone_slot = Arc::new(Mutex::new(None));
            let pending = Arc::new(Mutex::new(0));
            let init = {
                let g = graph.read().expect("kit");
                g.initial.clone()
            };
            let sync = KitGraph::from_full_dto(init);
            let (wip_tx, wip_rx) = mpsc::channel::<WipMsg>();
            let (coord_tx, coord_rx) = mpsc::channel::<CoordMsg>();
            let g1 = graph.clone();
            let coord_tx_wip = coord_tx.clone();
            let t_wip = std::thread::spawn(move || {
                crate::wip_kit::run(g1, wip_rx, coord_tx_wip);
            });
            let bb = backbone_slot.clone();
            let sync_c = sync.clone();
            let pend = pending.clone();
            let conf = conflicts.clone();
            let wip_t = wip_tx.clone();
            let t_coord = std::thread::spawn(move || {
                crate::kit_coordinator::run(bb, sync_c, pend, conf, wip_t, coord_rx);
            });
            Self {
                graph: graph.clone(),
                wip_tx,
                coord_tx,
                conflicts,
                backbone_slot,
                pending,
                _handles: vec![t_wip, t_coord],
            }
        }

        pub fn graph(&self) -> KitGraphRef {
            self.graph.clone()
        }

        pub fn subscribe(&self) -> Receiver<crate::events::KitEvent> {
            self.graph.read().expect("kit").subscribe()
        }

        pub fn execute_vcs(&self, cmd: KitStoreCommand) -> Result<KitStoreCommandResult> {
            let (tx, rx) = mpsc::channel();
            self.wip_tx
                .send(WipMsg::Exec { cmd, reply: tx })
                .map_err(|_| SemioError::InvalidOperation("wip store closed".into()))?;
            rx.recv()
                .map_err(|_| SemioError::InvalidOperation("wip reply missing".into()))?
        }

        pub fn attach_backbone(&self, cfg: BackboneConfig) -> Result<()> {
            let (tx, rx) = mpsc::channel();
            self.coord_tx
                .send(CoordMsg::Attach { cfg, reply: tx })
                .map_err(|_| SemioError::InvalidOperation("coordinator closed".into()))?;
            rx.recv()
                .map_err(|_| SemioError::InvalidOperation("attach reply missing".into()))?
        }

        pub fn detach_backbone(&self) {
            let (tx, rx) = mpsc::channel();
            if self.coord_tx.send(CoordMsg::Detach { reply: tx }).is_ok() {
                let _ = rx.recv();
            }
        }

        pub fn backbone_status(&self) -> Result<crate::backbone::BackboneStatusDto> {
            let (tx, rx) = mpsc::channel();
            self.coord_tx
                .send(CoordMsg::Status { reply: tx })
                .map_err(|_| SemioError::InvalidOperation("coordinator closed".into()))?;
            rx.recv()
                .map_err(|_| SemioError::InvalidOperation("status reply missing".into()))
        }

        pub fn set_active_checkpoint(&self, id: Option<crate::id::Id>) -> Result<()> {
            let (tx, rx) = mpsc::channel();
            self.coord_tx
                .send(CoordMsg::SetActiveCheckpoint { id, reply: tx })
                .map_err(|_| SemioError::InvalidOperation("coordinator closed".into()))?;
            rx.recv()
                .map_err(|_| SemioError::InvalidOperation("setActive reply missing".into()))?
        }

        pub fn sync_now(&self) -> Result<()> {
            self.coord_tx
                .send(CoordMsg::SyncNow)
                .map_err(|_| SemioError::InvalidOperation("coordinator closed".into()))?;
            Ok(())
        }

        pub fn list_conflicts(&self) -> Vec<KitConflict> {
            self.conflicts.list()
        }

        pub fn resolve_conflict(&self, id: crate::id::Id, strategy: ConflictResolution) -> Result<()> {
            let (tx, rx) = mpsc::channel();
            self.coord_tx
                .send(CoordMsg::ResolveConflict {
                    id,
                    strategy,
                    wip: self.graph.clone(),
                    reply: tx,
                })
                .map_err(|_| SemioError::InvalidOperation("coordinator closed".into()))?;
            rx.recv()
                .map_err(|_| SemioError::InvalidOperation("resolve reply missing".into()))?
        }
    }
}
