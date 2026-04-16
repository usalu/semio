//! Collaborative async kit backbones (native targets only).
//!
//! Pattern: optimistic local [`Kit`] + authoritative snapshot on the backbone with a linear
//! commit ledger. Sessions carry an interaction timeout chosen by the backbone; when it fires,
//! collaborators are disconnected and [`InteractionLockWarning`] signals UI to offer reconnect vs.
//! offline save (see [`KitBackboneHandle::interaction_warning`]).
//!
//! Change flow: a client submits a validated [`KitGraphChange`]; the backbone asks every other
//! session-holder to validate and agree (`AgreementNeeded` retries until the consensus deadline).
//!
//! ### `RemoteKitBackbone` wire protocol (`semio-kit-backbone/1`)
//!
//! Text WebSocket frames carry JSON ([`RemoteWireMsg`]). A semio-hub endpoint should persist the
//! authoritative kit + ledger in Postgres / object storage; this crate implements the client peer.
//!
//! [`Kit`] graphs are domain-mutable and are not `Send`; the authoritative backbone therefore runs
//! on a dedicated OS thread while [`KitStateSnapshot`] exposes `Arc<str>` kit JSON for `watchers.

use crate::{
    export_dev_kit, export_local_kit, import_dev_kit,
    import_local_kit, remap_kit_file_bytes, remove_stale_local_assets,
    Kit, KitGraphChange, Result, SemioError, SemioUtil,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    thread,
};
use tokio::sync::{oneshot, watch, Mutex};
use tokio_tungstenite::tungstenite::Message as WsMessage;

type StdResult<T, E> = std::result::Result<T, E>;

// ——— Public config / surface warnings —————————————————————————————————————————

/// Backbone-defined timeouts (interaction idle + consensus retry window).
#[derive(Debug, Clone)]
pub struct BackboneRuntimeConfig {
    pub interaction_timeout: Duration,
    pub consensus_deadline: Duration,
    pub consensus_broadcast_interval: Duration,
}

impl Default for BackboneRuntimeConfig {
    fn default() -> Self {
        Self {
            interaction_timeout: Duration::from_secs(120),
            consensus_deadline: Duration::from_secs(120),
            consensus_broadcast_interval: Duration::from_millis(750),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionLockWarning {
    None,
    IdleTimeout,
    Disconnected,
}

#[derive(Clone)]
pub struct KitStateSnapshot {
    kit_json: Arc<str>,
    pub revision: u64,
    pub interaction_lock: InteractionLockWarning,
}

impl KitStateSnapshot {
    pub fn kit(&self) -> Result<Kit> {
        Kit::from_json_str(self.kit_json.as_ref())
    }

    pub fn kit_json(&self) -> Arc<str> {
        Arc::clone(&self.kit_json)
    }
}

#[derive(Debug, Clone)]
pub struct SessionStarted {
    pub client_id: String,
    pub interaction_timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum BackboneEvent {
    AgreementNeeded {
        candidate_id: String,
        change: KitGraphChange,
    },
    Committed {
        revision: u64,
        change: KitGraphChange,
    },
    SessionIdleTimeout,
    Disconnected {
        reason: String,
    },
}

type ClientId = String;

// ——— Ledger ————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerFile {
    revision: u64,
    entries: Vec<LedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerEntry {
    seq: u64,
    change: KitGraphChange,
}

fn dev_ledger_path(kit_path: &Path) -> PathBuf {
    PathBuf::from(format!("{}.ledger.json", kit_path.display()))
}

fn local_ledger_path(folder: &Path) -> PathBuf {
    folder.join(".semio").join("kit.ledger.json")
}

fn load_ledger(path: &Path) -> Result<LedgerFile> {
    if !path.exists() {
        return Ok(LedgerFile {
            revision: 0,
            entries: vec![],
        });
    }
    let s = std::fs::read_to_string(path).map_err(|e| SemioError::Database {
        message: format!("read ledger {}: {}", path.display(), e),
    })?;
    serde_json::from_str(&s).map_err(|e| SemioError::Serialization {
        message: format!("ledger {}: {}", path.display(), e),
    })
}

fn save_ledger(path: &Path, ledger: &LedgerFile) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| SemioError::Database {
            message: format!("mkdir {}: {}", parent.display(), e),
        })?;
    }
    let json = serde_json::to_string_pretty(ledger).map_err(|e| SemioError::Serialization {
        message: format!("serialize ledger: {}", e),
    })?;
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, json).map_err(|e| SemioError::Database {
        message: format!("write {}: {}", tmp.display(), e),
    })?;
    std::fs::rename(&tmp, path).map_err(|e| SemioError::Database {
        message: format!("rename ledger: {}", e),
    })?;
    Ok(())
}

fn snapshot_from_kit(kit: &Kit, revision: u64, lock: InteractionLockWarning) -> Result<KitStateSnapshot> {
    Ok(KitStateSnapshot {
        kit_json: Arc::from(kit.to_json_pretty()?.into_boxed_str()),
        revision,
        interaction_lock: lock,
    })
}

// ——— Authority —————————————————————————————————————————————————————————————

enum PersistKind {
    Dev { kit_path: PathBuf },
    Local {
        folder: PathBuf,
        files: HashMap<String, Vec<u8>>,
    },
}

struct ClientSession {
    active: bool,
    last_touch: Instant,
    notify: Sender<BackboneEvent>,
}

struct PendingConsensus {
    id: String,
    change: KitGraphChange,
    votes: HashMap<ClientId, Option<bool>>,
    deadline: Instant,
    last_broadcast: Instant,
    finish: oneshot::Sender<Result<u64>>,
}

struct AuthorityState {
    /// Authoritative kit as JSON so the backbone thread stays `Send` (`Kit` uses interior mutability).
    kit_json: String,
    revision: u64,
    ledger: LedgerFile,
    persist: PersistKind,
    clients: HashMap<ClientId, ClientSession>,
    pending: Option<PendingConsensus>,
    config: BackboneRuntimeConfig,
}

impl AuthorityState {
    fn kit(&self) -> Result<Kit> {
        Kit::from_json_str(&self.kit_json)
    }

    fn set_kit(&mut self, kit: &Kit) -> Result<()> {
        self.kit_json = kit.to_json_pretty()?;
        Ok(())
    }

    fn session_ids(&self) -> Vec<ClientId> {
        self.clients
            .iter()
            .filter(|(_, s)| s.active)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn persist_snapshot(&mut self) -> Result<()> {
        let kit = self.kit()?;
        match &mut self.persist {
            PersistKind::Dev { kit_path } => {
                export_dev_kit(&kit, kit_path.to_str().ok_or_else(|| {
                    SemioError::InvalidOperation {
                        message: "Dev kit path is not UTF-8".into(),
                    }
                })?)?;
                save_ledger(&dev_ledger_path(kit_path), &self.ledger)?;
            }
            PersistKind::Local { folder, files } => {
                let folder_s = folder.to_str().ok_or_else(|| SemioError::InvalidOperation {
                    message: "Local folder path is not UTF-8".into(),
                })?;
                export_local_kit(&kit, files, folder_s)?;
                save_ledger(&local_ledger_path(folder), &self.ledger)?;
            }
        }
        Ok(())
    }

    fn apply_committed_change(&mut self, change: &KitGraphChange) -> Result<()> {
        let mut kit = self.kit()?;
        let before = kit.clone();
        let diff = change
            .validation
            .diff
            .as_ref()
            .unwrap_or(&change.forward)
            .clone();
        kit.apply_diff(&diff);

        if let PersistKind::Local { files, folder } = &mut self.persist {
            let prev = files.clone();
            *files = remap_kit_file_bytes(&before, &kit, files)?;
            let folder_s = folder.to_str().unwrap();
            remove_stale_local_assets(folder_s, &prev, files)?;
        }

        self.set_kit(&kit)?;

        self.revision += 1;
        let seq = self.revision;
        self.ledger.revision = self.revision;
        self.ledger.entries.push(LedgerEntry {
            seq,
            change: change.clone(),
        });
        self.persist_snapshot()?;
        Ok(())
    }
}

enum AuthRequest {
    Register {
        reply: oneshot::Sender<(ClientId, Receiver<BackboneEvent>, watch::Receiver<KitStateSnapshot>)>,
    },
    StartSession {
        id: ClientId,
        reply: oneshot::Sender<Result<SessionStarted>>,
    },
    Touch {
        id: ClientId,
    },
    Propose {
        id: ClientId,
        change: KitGraphChange,
        reply: oneshot::Sender<Result<u64>>,
    },
    Vote {
        id: ClientId,
        candidate_id: String,
        agree: bool,
    },
}

fn broadcast_kit(
    state: &AuthorityState,
    snapshot_tx: &watch::Sender<KitStateSnapshot>,
) -> Result<()> {
    let snap = snapshot_from_kit(&state.kit()?, state.revision, InteractionLockWarning::None)?;
    let _ = snapshot_tx.send(snap);
    Ok(())
}

fn authority_main_loop(
    rx: Receiver<AuthRequest>,
    mut state: AuthorityState,
    snapshot_tx: watch::Sender<KitStateSnapshot>,
) {
    let tick = Duration::from_millis(250);
    let _ = broadcast_kit(&state, &snapshot_tx);

    loop {
        match rx.recv_timeout(tick) {
            Ok(req) => handle_auth_request(req, &mut state, &snapshot_tx),
            Err(RecvTimeoutError::Timeout) => {
                tick_idle_and_consensus(&mut state, &snapshot_tx);
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn handle_auth_request(
    req: AuthRequest,
    state: &mut AuthorityState,
    snapshot_tx: &watch::Sender<KitStateSnapshot>,
) {
    match req {
        AuthRequest::Register { reply } => {
            let id = SemioUtil::generate_guid();
            let (tx, rx_ev) = mpsc::channel::<BackboneEvent>();
            let snap = snapshot_tx.subscribe();
            let _ = reply.send((id.clone(), rx_ev, snap));
            state.clients.insert(
                id,
                ClientSession {
                    active: false,
                    last_touch: Instant::now(),
                    notify: tx,
                },
            );
        }
        AuthRequest::StartSession { id, reply } => {
            let res = if let Some(s) = state.clients.get_mut(&id) {
                s.active = true;
                s.last_touch = Instant::now();
                Ok(SessionStarted {
                    client_id: id.clone(),
                    interaction_timeout: state.config.interaction_timeout,
                })
            } else {
                Err(SemioError::InvalidOperation {
                    message: "Unknown backbone client".into(),
                })
            };
            let _ = reply.send(res);
        }
        AuthRequest::Touch { id } => {
            if let Some(s) = state.clients.get_mut(&id) {
                s.last_touch = Instant::now();
            }
        }
        AuthRequest::Vote {
            id,
            candidate_id,
            agree,
        } => {
            let Some(p) = state.pending.as_mut() else {
                return;
            };
            if p.id != candidate_id || !p.votes.contains_key(&id) {
                return;
            }
            p.votes.insert(id.clone(), Some(agree));

            let any_no = p.votes.values().any(|v| matches!(v, Some(false)));
            if any_no {
                let pending = state.pending.take().unwrap();
                let _ = pending.finish.send(Err(SemioError::InvalidOperation {
                    message: "Consensus rejected the candidate".into(),
                }));
                return;
            }

            let unanimous_yes = p.votes.values().all(|v| matches!(v, Some(true)));
            if unanimous_yes {
                let pending = state.pending.take().unwrap();
                let change = pending.change.clone();
                match state.apply_committed_change(&change) {
                    Ok(()) => {
                        let rev = state.revision;
                        let _ = broadcast_kit(state, snapshot_tx);
                        let committed = BackboneEvent::Committed {
                            revision: rev,
                            change: change.clone(),
                        };
                        for (_, c) in &state.clients {
                            let _ = c.notify.send(committed.clone());
                        }
                        let _ = pending.finish.send(Ok(rev));
                    }
                    Err(e) => {
                        let _ = pending.finish.send(Err(e));
                    }
                }
            }
        }
        AuthRequest::Propose {
            id,
            change,
            reply,
        } => {
            let holders = state.session_ids();
            if !holders.contains(&id) {
                let _ = reply.send(Err(SemioError::InvalidOperation {
                    message: "start_session required before proposing".into(),
                }));
                return;
            }

            let kit = match state.kit() {
                Ok(k) => k,
                Err(e) => {
                    let _ = reply.send(Err(e));
                    return;
                }
            };
            let validation = kit.validate_diff(&change.forward, false);
            if !validation.ok || !validation.errors.is_empty() {
                let _ = reply.send(Err(SemioError::Validation {
                    message: validation
                        .errors
                        .first()
                        .map(|e| e.message.clone())
                        .unwrap_or_else(|| "validation failed".into()),
                }));
                return;
            }

            let mut touch = change.clone();
            touch.validation = validation;

            if state.pending.is_some() {
                let _ = reply.send(Err(SemioError::InvalidOperation {
                    message: "Another change is awaiting consensus".into(),
                }));
                return;
            }

            let voters: HashSet<ClientId> = holders.iter().cloned().collect();
            if voters.is_empty() {
                let _ = reply.send(Err(SemioError::InvalidOperation {
                    message: "No active sessions".into(),
                }));
                return;
            }

            if voters.len() == 1 && voters.contains(&id) {
                match state.apply_committed_change(&touch) {
                    Ok(()) => {
                        let rev = state.revision;
                        let _ = broadcast_kit(state, snapshot_tx);
                        let committed = BackboneEvent::Committed {
                            revision: rev,
                            change: touch.clone(),
                        };
                        for (_, c) in &state.clients {
                            let _ = c.notify.send(committed.clone());
                        }
                        let _ = reply.send(Ok(rev));
                    }
                    Err(e) => {
                        let _ = reply.send(Err(e));
                    }
                }
                return;
            }

            let candidate_id = SemioUtil::generate_guid();
            let mut votes = HashMap::new();
            for v in &voters {
                votes.insert(v.clone(), None);
            }

            state.pending = Some(PendingConsensus {
                id: candidate_id.clone(),
                change: touch.clone(),
                votes,
                deadline: Instant::now() + state.config.consensus_deadline,
                last_broadcast: Instant::now(),
                finish: reply,
            });

            let ev = BackboneEvent::AgreementNeeded {
                candidate_id: candidate_id.clone(),
                change: touch.clone(),
            };
            for (cid, c) in &state.clients {
                if voters.contains(cid) {
                    let _ = c.notify.send(ev.clone());
                }
            }
        }
    }
}

fn tick_idle_and_consensus(state: &mut AuthorityState, snapshot_tx: &watch::Sender<KitStateSnapshot>) {
    let now = Instant::now();
    let timeout = state.config.interaction_timeout;

    let mut timed_out = Vec::new();
    for (cid, s) in &state.clients {
        if s.active && now.duration_since(s.last_touch) > timeout {
            timed_out.push(cid.clone());
        }
    }
    for cid in timed_out {
        if let Some(mut s) = state.clients.remove(&cid) {
            s.active = false;
            let _ = s.notify.send(BackboneEvent::SessionIdleTimeout);
        }
        match state.kit() {
            Ok(ref kit) => {
                if let Ok(snap) = snapshot_from_kit(
                    kit,
                    state.revision,
                    InteractionLockWarning::IdleTimeout,
                ) {
                    let _ = snapshot_tx.send(snap);
                }
            }
            Err(_) => {}
        }
    }

    if let Some(p) = state.pending.as_ref() {
        if now > p.deadline {
            let pending = state.pending.take().unwrap();
            let _ = pending.finish.send(Err(SemioError::InvalidOperation {
                message: "Consensus timed out".into(),
            }));
            return;
        }
    }

    if let Some(p) = state.pending.as_mut() {
        if now.duration_since(p.last_broadcast) >= state.config.consensus_broadcast_interval {
            p.last_broadcast = now;
            let ev = BackboneEvent::AgreementNeeded {
                candidate_id: p.id.clone(),
                change: p.change.clone(),
            };
            for (cid, vote) in &p.votes {
                if vote.is_none() {
                    if let Some(c) = state.clients.get(cid) {
                        let _ = c.notify.send(ev.clone());
                    }
                }
            }
        }
    }
}

fn spawn_agreement_worker(
    client_id: ClientId,
    rx_ev: Receiver<BackboneEvent>,
    auth_tx: Sender<AuthRequest>,
    snapshot_rx: watch::Receiver<KitStateSnapshot>,
) {
    thread::spawn(move || {
        while let Ok(ev) = rx_ev.recv() {
            let BackboneEvent::AgreementNeeded {
                candidate_id,
                change,
            } = ev
            else {
                continue;
            };
            let snap = snapshot_rx.borrow().clone();
            if snap.interaction_lock != InteractionLockWarning::None {
                let _ = auth_tx.send(AuthRequest::Vote {
                    id: client_id.clone(),
                    candidate_id,
                    agree: false,
                });
                continue;
            }
            let kit = match snap.kit() {
                Ok(k) => k,
                Err(_) => {
                    let _ = auth_tx.send(AuthRequest::Vote {
                        id: client_id.clone(),
                        candidate_id,
                        agree: false,
                    });
                    continue;
                }
            };
            let v = kit.validate_diff(&change.forward, false);
            let agree = v.ok && v.errors.is_empty();
            let _ = auth_tx.send(AuthRequest::Vote {
                id: client_id.clone(),
                candidate_id,
                agree,
            });
        }
    });
}

/// Handle to the collaborative backbone (clone for additional peers in-process).
#[derive(Clone)]
pub struct KitBackboneHandle {
    auth_tx: Sender<AuthRequest>,
    client_id: ClientId,
    kit_state: watch::Receiver<KitStateSnapshot>,
}

impl KitBackboneHandle {
    pub async fn start_session(&self) -> Result<SessionStarted> {
        let (tx, rx) = oneshot::channel();
        self.auth_tx
            .send(AuthRequest::StartSession {
                id: self.client_id.clone(),
                reply: tx,
            })
            .map_err(|_| SemioError::InvalidOperation {
                message: "Backbone authority stopped".into(),
            })?;
        rx.await.map_err(|_| SemioError::InvalidOperation {
            message: "Backbone authority stopped".into(),
        })?
    }

    pub fn touch(&self) -> Result<()> {
        self.auth_tx
            .send(AuthRequest::Touch {
                id: self.client_id.clone(),
            })
            .map_err(|_| SemioError::InvalidOperation {
                message: "Backbone authority stopped".into(),
            })
    }

    pub fn interaction_warning(&self) -> InteractionLockWarning {
        self.kit_state.borrow().interaction_lock
    }

    pub fn subscribe_kit_state(&self) -> watch::Receiver<KitStateSnapshot> {
        self.kit_state.clone()
    }

    pub async fn propose_change(&self, change: KitGraphChange) -> Result<u64> {
        let (tx, rx) = oneshot::channel();
        self.auth_tx
            .send(AuthRequest::Propose {
                id: self.client_id.clone(),
                change,
                reply: tx,
            })
            .map_err(|_| SemioError::InvalidOperation {
                message: "Backbone authority stopped".into(),
            })?;
        rx.await.map_err(|_| SemioError::InvalidOperation {
            message: "Backbone authority stopped".into(),
        })?
    }
}

/// In-process collaborative hub (OS thread). Connect peers via [`KitBackboneHub::connect`].
pub struct KitBackboneHub {
    tx: Sender<AuthRequest>,
    _join: thread::JoinHandle<()>,
}

impl KitBackboneHub {
    pub async fn connect(&self) -> Result<KitBackboneHandle> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(AuthRequest::Register { reply: tx })
            .map_err(|_| SemioError::InvalidOperation {
                message: "Backbone authority stopped".into(),
            })?;
        let (client_id, rx_ev, kit_state) = rx.await.map_err(|_| SemioError::InvalidOperation {
            message: "Backbone authority stopped".into(),
        })?;
        let auth_tx = self.tx.clone();
        spawn_agreement_worker(client_id.clone(), rx_ev, auth_tx.clone(), kit_state.clone());
        Ok(KitBackboneHandle {
            auth_tx,
            client_id,
            kit_state,
        })
    }
}

fn spawn_authority(state: AuthorityState, snapshot_tx: watch::Sender<KitStateSnapshot>) -> KitBackboneHub {
    let (tx, rx) = mpsc::channel::<AuthRequest>();
    let join = thread::spawn(move || {
        authority_main_loop(rx, state, snapshot_tx);
    });
    KitBackboneHub {
        tx,
        _join: join,
    }
}

// ——— Dev —————————————————————————————————————————————————————————————————————

pub struct DevKitBackbone;

impl DevKitBackbone {
    pub async fn spawn(
        kit_json_path: impl AsRef<Path>,
        initial_kit_if_missing: Option<Kit>,
        config: BackboneRuntimeConfig,
    ) -> Result<KitBackboneHub> {
        let kit_path = kit_json_path.as_ref().to_path_buf();
        let path_str = kit_path.to_str().ok_or_else(|| SemioError::InvalidOperation {
            message: "Dev kit path is not UTF-8".into(),
        })?;

        let (kit, ledger) = if kit_path.exists() {
            let k = import_dev_kit(path_str)?;
            let ledger = load_ledger(&dev_ledger_path(&kit_path))?;
            (k, ledger)
        } else if let Some(k) = initial_kit_if_missing {
            export_dev_kit(&k, path_str)?;
            save_ledger(
                &dev_ledger_path(&kit_path),
                &LedgerFile {
                    revision: 0,
                    entries: vec![],
                },
            )?;
            (
                import_dev_kit(path_str)?,
                load_ledger(&dev_ledger_path(&kit_path))?,
            )
        } else {
            return Err(SemioError::InvalidOperation {
                message: "Dev kit file missing and no initial kit provided".into(),
            });
        };

        let revision = ledger.revision.max(ledger.entries.len() as u64);

        let kit_json = kit.to_json_pretty()?;
        let state = AuthorityState {
            kit_json,
            revision,
            ledger,
            persist: PersistKind::Dev {
                kit_path: kit_path.clone(),
            },
            clients: HashMap::new(),
            pending: None,
            config,
        };

        let init_snap =
            snapshot_from_kit(&state.kit()?, state.revision, InteractionLockWarning::None)?;
        let (snapshot_tx, _) = watch::channel(init_snap);

        Ok(spawn_authority(state, snapshot_tx))
    }
}

// ——— Local ———————————————————————————————————————————————————————————————————

pub struct LocalKitBackbone;

impl LocalKitBackbone {
    pub async fn spawn(
        folder_path: impl AsRef<Path>,
        initial_kit_if_missing: Option<Kit>,
        config: BackboneRuntimeConfig,
    ) -> Result<KitBackboneHub> {
        let folder = folder_path.as_ref().to_path_buf();
        let folder_s = folder.to_str().ok_or_else(|| SemioError::InvalidOperation {
            message: "Local folder path is not UTF-8".into(),
        })?;

        std::fs::create_dir_all(&folder).map_err(|e| SemioError::Database {
            message: format!("mkdir {}: {}", folder.display(), e),
        })?;

        let (kit, files, ledger) = if folder.join(".semio").join("kit.db").exists()
            || folder.join(".semio").join("kit.ledger.json").exists()
        {
            let imported = import_local_kit(folder_s)?;
            let ledger = load_ledger(&local_ledger_path(&folder))?;
            (imported.kit, imported.files, ledger)
        } else if let Some(k) = initial_kit_if_missing {
            let empty = HashMap::new();
            export_local_kit(&k, &empty, folder_s)?;
            save_ledger(
                &local_ledger_path(&folder),
                &LedgerFile {
                    revision: 0,
                    entries: vec![],
                },
            )?;
            let imported = import_local_kit(folder_s)?;
            (imported.kit, imported.files, load_ledger(&local_ledger_path(&folder))?)
        } else {
            return Err(SemioError::InvalidOperation {
                message: "Local kit missing and no initial kit provided".into(),
            });
        };

        let revision = ledger.revision.max(ledger.entries.len() as u64);

        let kit_json = kit.to_json_pretty()?;
        let state = AuthorityState {
            kit_json,
            revision,
            ledger,
            persist: PersistKind::Local { folder, files },
            clients: HashMap::new(),
            pending: None,
            config,
        };

        let init_snap =
            snapshot_from_kit(&state.kit()?, state.revision, InteractionLockWarning::None)?;
        let (snapshot_tx, _) = watch::channel(init_snap);

        Ok(spawn_authority(state, snapshot_tx))
    }
}

// ——— Remote (WebSocket client) —————————————————————————————————————————————————

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RemoteWireMsg {
    #[serde(rename = "hello")]
    Hello {
        protocol: String,
        role: String,
    },
    #[serde(rename = "welcome")]
    Welcome {
        interaction_timeout_ms: u64,
        revision: u64,
        kit_json: String,
    },
    #[serde(rename = "session_start")]
    SessionStart,
    #[serde(rename = "session_ack")]
    SessionAck {
        interaction_timeout_ms: u64,
        client_id: String,
    },
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "propose")]
    Propose {
        candidate_id: String,
        change: KitGraphChange,
    },
    #[serde(rename = "agreement_request")]
    AgreementRequest {
        candidate_id: String,
        change: KitGraphChange,
    },
    #[serde(rename = "vote")]
    Vote {
        candidate_id: String,
        agree: bool,
    },
    #[serde(rename = "committed")]
    Committed {
        #[serde(default)]
        candidate_id: Option<String>,
        revision: u64,
        change: KitGraphChange,
    },
    #[serde(rename = "reject")]
    Reject {
        candidate_id: String,
        reason: String,
    },
    #[serde(rename = "session_timeout")]
    SessionTimeout,
    #[serde(rename = "disconnect")]
    Disconnect {
        reason: String,
    },
}

type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WsSink = futures::stream::SplitSink<WsStream, WsMessage>;

#[derive(Clone)]
pub struct RemoteKitSession {
    write: Arc<Mutex<WsSink>>,
    kit_state: watch::Receiver<KitStateSnapshot>,
    interaction_timeout: Duration,
    client_id: String,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<Result<u64>>>>>,
}

impl RemoteKitSession {
    pub fn interaction_warning(&self) -> InteractionLockWarning {
        self.kit_state.borrow().interaction_lock
    }

    pub fn subscribe_kit_state(&self) -> watch::Receiver<KitStateSnapshot> {
        self.kit_state.clone()
    }

    pub fn remote_client_id(&self) -> &str {
        &self.client_id
    }

    pub async fn start_session(&self) -> Result<SessionStarted> {
        Ok(SessionStarted {
            client_id: self.client_id.clone(),
            interaction_timeout: self.interaction_timeout,
        })
    }

    pub async fn touch(&self) -> Result<()> {
        self.send_json(&RemoteWireMsg::Heartbeat).await
    }

    pub async fn propose_change(&self, change: KitGraphChange) -> Result<u64> {
        let candidate_id = SemioUtil::generate_guid();
        let (tx, rx) = oneshot::channel();
        {
            let mut g = self.pending.lock().await;
            g.insert(candidate_id.clone(), tx);
        }
        self.send_json(&RemoteWireMsg::Propose {
            candidate_id,
            change,
        })
        .await?;
        rx.await.map_err(|_| SemioError::InvalidOperation {
            message: "Remote backbone closed during propose".into(),
        })?
    }

    async fn send_json(&self, msg: &RemoteWireMsg) -> Result<()> {
        let txt = serde_json::to_string(msg).map_err(|e| SemioError::Serialization {
            message: format!("{}", e),
        })?;
        let mut w = self.write.lock().await;
        w.send(WsMessage::Text(txt.into()))
            .await
            .map_err(|e| SemioError::Database {
                message: format!("ws send: {}", e),
            })
    }
}

pub struct RemoteKitBackbone;

impl RemoteKitBackbone {
    pub async fn connect(
        websocket_url: &str,
    ) -> Result<(RemoteKitSession, tokio::task::JoinHandle<Result<()>>)> {
        let url = url::Url::parse(websocket_url).map_err(|e| SemioError::InvalidOperation {
            message: format!("Bad WebSocket URL: {}", e),
        })?;
        let (ws, _) = tokio_tungstenite::connect_async(url.as_str())
            .await
            .map_err(|e| SemioError::Database {
                message: format!("WebSocket connect failed: {}", e),
            })?;
        let (mut write, mut read) = ws.split();

        let hello = RemoteWireMsg::Hello {
            protocol: "semio-kit-backbone/1".into(),
            role: "client".into(),
        };
        let txt = serde_json::to_string(&hello).map_err(|e| SemioError::Serialization {
            message: format!("{}", e),
        })?;
        write
            .send(WsMessage::Text(txt.into()))
            .await
            .map_err(|e| SemioError::Database {
                message: format!("ws send: {}", e),
            })?;

        let welcome = loop {
            let msg = read.next().await.ok_or_else(|| SemioError::Database {
                message: "WebSocket closed before welcome".into(),
            })?;
            let msg = msg.map_err(|e| SemioError::Database {
                message: format!("ws: {}", e),
            })?;
            if let WsMessage::Text(t) = msg {
                let m: RemoteWireMsg = serde_json::from_str(&t).map_err(|e| {
                    SemioError::Serialization {
                        message: format!("welcome parse: {}", e),
                    }
                })?;
                if let RemoteWireMsg::Welcome {
                    interaction_timeout_ms,
                    revision,
                    kit_json,
                } = m
                {
                    break (interaction_timeout_ms, revision, kit_json);
                }
            }
        };

        let kit_json = welcome.2.clone();
        let kit = Kit::from_json_str(&kit_json)?;
        let init_snap = snapshot_from_kit(&kit, welcome.1, InteractionLockWarning::None)?;
        let (snapshot_tx, snapshot_rx) = watch::channel(init_snap);

        let sess_start = RemoteWireMsg::SessionStart;
        let txt = serde_json::to_string(&sess_start).map_err(|e| SemioError::Serialization {
            message: format!("{}", e),
        })?;
        write
            .send(WsMessage::Text(txt.into()))
            .await
            .map_err(|e| SemioError::Database {
                message: format!("ws send: {}", e),
            })?;

        let session_ack = loop {
            let msg = read.next().await.ok_or_else(|| SemioError::Database {
                message: "WebSocket closed before session_ack".into(),
            })?;
            let msg = msg.map_err(|e| SemioError::Database {
                message: format!("ws: {}", e),
            })?;
            if let WsMessage::Text(t) = msg {
                let m: RemoteWireMsg = serde_json::from_str(&t).map_err(|e| {
                    SemioError::Serialization {
                        message: format!("session_ack parse: {}", e),
                    }
                })?;
                if let RemoteWireMsg::SessionAck {
                    interaction_timeout_ms,
                    client_id,
                } = m
                {
                    break (interaction_timeout_ms, client_id);
                }
            }
        };

        let write = Arc::new(Mutex::new(write));
        let pending: Arc<Mutex<HashMap<String, oneshot::Sender<Result<u64>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let session = RemoteKitSession {
            write: write.clone(),
            kit_state: snapshot_rx,
            interaction_timeout: Duration::from_millis(session_ack.0.max(1)),
            client_id: session_ack.1,
            pending: pending.clone(),
        };

        let reader = tokio::spawn(remote_reader_loop(
            read,
            write,
            snapshot_tx,
            welcome.1,
            kit_json,
            pending,
        ));

        Ok((session, reader))
    }
}

async fn remote_reader_loop<S>(
    mut read: S,
    write: Arc<Mutex<WsSink>>,
    snapshot_tx: watch::Sender<KitStateSnapshot>,
    mut revision: u64,
    mut kit_json: String,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<Result<u64>>>>>,
) -> Result<()>
where
    S: StreamExt<Item = StdResult<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{

    while let Some(msg) = read.next().await {
        let msg = msg.map_err(|e| SemioError::Database {
            message: format!("ws read: {}", e),
        })?;
        if let WsMessage::Text(t) = msg {
            let m: RemoteWireMsg = serde_json::from_str(&t).map_err(|e| {
                SemioError::Serialization {
                    message: format!("remote msg: {}", e),
                }
            })?;
            match m {
                RemoteWireMsg::AgreementRequest {
                    candidate_id,
                    change,
                } => {
                    let agree = match Kit::from_json_str(&kit_json) {
                        Ok(kit) => {
                            let v = kit.validate_diff(&change.forward, false);
                            v.ok && v.errors.is_empty()
                        }
                        Err(e) => return Err(e),
                    };
                    let vote = RemoteWireMsg::Vote {
                        candidate_id,
                        agree,
                    };
                    let txt = serde_json::to_string(&vote).map_err(|e| SemioError::Serialization {
                        message: format!("{}", e),
                    })?;
                    let mut w = write.lock().await;
                    let _ = w.send(WsMessage::Text(txt.into())).await;
                }
                RemoteWireMsg::Committed {
                    candidate_id,
                    revision: rev,
                    change,
                } => {
                    revision = rev;
                    let diff = change
                        .validation
                        .diff
                        .as_ref()
                        .unwrap_or(&change.forward)
                        .clone();
                    let complete = candidate_id.clone();
                    {
                        let mut kit = Kit::from_json_str(&kit_json)?;
                        kit.apply_diff(&diff);
                        kit_json = kit.to_json_pretty()?;
                        if let Ok(snap) =
                            snapshot_from_kit(&kit, revision, InteractionLockWarning::None)
                        {
                            let _ = snapshot_tx.send(snap);
                        }
                    }
                    if let Some(cid) = complete {
                        let tx_opt = pending.lock().await.remove(&cid);
                        if let Some(tx) = tx_opt {
                            let _ = tx.send(Ok(revision));
                        }
                    }
                }
                RemoteWireMsg::Reject {
                    candidate_id,
                    reason,
                } => {
                    if let Some(tx) = pending.lock().await.remove(&candidate_id) {
                        let _ = tx.send(Err(SemioError::InvalidOperation { message: reason }));
                    }
                }
                RemoteWireMsg::SessionTimeout => {
                    let kit = Kit::from_json_str(&kit_json)?;
                    if let Ok(snap) = snapshot_from_kit(
                        &kit,
                        revision,
                        InteractionLockWarning::IdleTimeout,
                    ) {
                        let _ = snapshot_tx.send(snap);
                    }
                }
                RemoteWireMsg::Disconnect { reason } => {
                    let kit = Kit::from_json_str(&kit_json)?;
                    if let Ok(snap) = snapshot_from_kit(
                        &kit,
                        revision,
                        InteractionLockWarning::Disconnected,
                    ) {
                        let _ = snapshot_tx.send(snap);
                    }
                    return Err(SemioError::InvalidOperation { message: reason });
                }
                _ => {}
            }
        }
    }
    Ok(())
}
