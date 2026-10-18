//! 🛠️ Db artifact engine: document state, query source, state pages and ledger tail.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as store;

use db_artifact_contract::{ArtifactDurableGroupJournalAppendV1, ArtifactDurableGroupRecoveryDriverCoreV1 as _, ArtifactDurableGroupRecoveryDriverV1, CheckpointPublicationSnapshot, CommandBatch, CommandReceipt, ConflictRecord, DB_PATHMAP_SCHEMA, FOREIGN_HISTORY_TRANSITION_CODE, FOREIGN_HISTORY_TRANSITION_MESSAGE, MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE, MAX_RECENT_TOUCHES, SubmitOptions, UNKNOWN_SUPERSEDE_TARGET_CODE, UNKNOWN_SUPERSEDE_TARGET_MESSAGE, command_touch, committed_durable_group_decision_from_transaction, diff_entries, durable_group_touch, encode_pathmap, entries_to_value, entries_touched, grade_concurrent_write, inverse_entries, remember_recent_touch, to_core_actor_id, to_core_document_id, unseen_concurrent_writes};
use db_artifact_history::{HistoryReplayFuture, HistoryReplayReservation};
use db_durability::{DurabilityClass, Frontier};
use db_ids::{*, ActorId, ArtifactId, DbError, DbLimits, check_len};
use db_io_core::WalStorage;
use db_observe::NullEmit;
use pack::ContentHash;
use protocol::MutationDiff as _;
use semio_framework_value::DslValue;
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, sync::Arc};
#[cfg(test)]
use db_artifact_contract::decode_pathmap_json;
#[cfg(test)]
use db_artifact_history::{HistoryEnvelopeCursor, HistoryEnvelopeField, HistoryFrontierCursor, HistoryFrontierField, HistoryReplayPhase, HistoryReplayTransition, history_capacity_test_lock};
#[cfg(test)]
use std::pin::Pin;

//#region 🔖️Ids
//#region 🔖️State
pub async fn admit_wal_bytes(source: Vec<u8>, maximum: u64, control: &mut db_wal_log::WalCursorControl) -> Result<db_wal_log::WalBytes, DbError> {
    let source = if source.len() as u64 <= maximum && source.capacity() as u64 > maximum { source.into_boxed_slice().into_vec() } else { source };
    match db_wal_log::WalBytes::try_admit(source, maximum, control).await {
        Ok(bytes) => Ok(bytes),
        Err(mut rejected) => {
            control.grant()?;
            let _ = rejected.close_step()?;
            Err(rejected.into_error())
        }
    }
}

pub async fn push_wal_record(records: &mut db_wal_log::WalRecordBatch, record: db_wal_log::WalRecord, control: &mut db_wal_log::WalCursorControl) -> Result<(), DbError> {
    match records.push(record) {
        Ok(()) => Ok(()),
        Err(mut record) => {
            control.grant()?;
            let _ = record.close_step()?;
            drop(record);
            Err(DbError::LimitExceeded("db_artifact fixed wal record batch"))
        }
    }
}

pub async fn close_wal_record_batch(records: &mut db_wal_log::WalRecordBatch) -> Result<(), DbError> {
    let mut control = db_wal_log::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    while !records.terminal_is_empty() {
        control.grant()?;
        if !records.close_step()? && !records.terminal_is_empty() {
            return Err(DbError::Internal("WAL record batch retirement stalled".to_string()));
        }
        semio_framework_async::yield_once().await;
    }
    Ok(())
}
//#region 🔖️StateRetirement
const ARTIFACT_STATE_RETIREMENT_SLOTS: usize = 64;

/// 🧹️ Persists one rejected staging graph and advances exactly one refusal,
/// source, slot, page, or text owner for each maintenance grant.
struct ArtifactStateRetirementCursor {
    pub rejected: Option<db_state::StateEntryRejected>,
    staged: [Option<db_state::StateEntry>; 64],
    pub phase: u8,
    pub slot: u8,
    pub retirement: Option<ArtifactStateRetirementReservation>,
}

impl ArtifactStateRetirementCursor {
    pub fn rejected(retirement: ArtifactStateRetirementReservation, rejected: db_state::StateEntryRejected, staged: [Option<db_state::StateEntry>; 64]) -> Self {
        Self { rejected: Some(rejected), staged, phase: 0, slot: 0, retirement: Some(retirement) }
    }

    pub fn entry(retirement: ArtifactStateRetirementReservation, entry: db_state::StateEntry) -> Self {
        let mut staged = std::array::from_fn(|_| None);
        staged[0] = Some(entry);
        Self { rejected: None, staged, phase: 2, slot: 0, retirement: Some(retirement) }
    }

    fn staged(retirement: ArtifactStateRetirementReservation, staged: [Option<db_state::StateEntry>; 64]) -> Self {
        Self { rejected: None, staged, phase: 2, slot: 0, retirement: Some(retirement) }
    }

    fn empty() -> Self {
        Self { rejected: None, staged: std::array::from_fn(|_| None), phase: 3, slot: 64, retirement: None }
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1)?;
        control.grant()?;
        match self.phase {
            0 => {
                let rejected = self.rejected.as_mut().ok_or_else(|| DbError::Internal("artifact retirement lost refusal owner".to_string()))?;
                if !rejected.close_step()? {
                    self.phase = 1;
                }
                Ok(true)
            }
            1 => {
                let source = self.rejected.take().and_then(db_state::StateEntryRejected::into_source);
                drop(source);
                self.phase = 2;
                Ok(true)
            }
            2 => {
                let Some(entry) = self.staged.get_mut(self.slot as usize) else {
                    self.phase = 3;
                    release_artifact_state_retirement(&mut self.retirement);
                    return Ok(true);
                };
                if let Some(entry) = entry.as_mut() {
                    if entry.close_step()? {
                        return Ok(true);
                    }
                }
                *entry = None;
                self.slot += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == 3 && self.rejected.is_none() && self.staged.iter().all(Option::is_none)
    }
}

static ARTIFACT_STATE_RETIREMENT: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

static ARTIFACT_STATE_RETIREMENT_OVERFLOW: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

static ARTIFACT_STATE_RETIREMENT_QUARANTINE: std::sync::Mutex<[Option<ArtifactStateRetirementCursor>; ARTIFACT_STATE_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; ARTIFACT_STATE_RETIREMENT_SLOTS]);

static ARTIFACT_STATE_RETIREMENT_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];

static ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy)]
struct ArtifactStateRetirementReservation {
    tier: u8,
    pub index: u8,
}

fn reserve_artifact_state_retirement() -> Option<ArtifactStateRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..ARTIFACT_STATE_RETIREMENT_SLOTS as u8 {
            let bit = 1u64 << index;
            if ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(ArtifactStateRetirementReservation { tier, index });
            }
            ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_artifact_state_retirement(reservation: &mut Option<ArtifactStateRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        ARTIFACT_STATE_RETIREMENT_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn release_artifact_state_retirements(reservations: &mut [Option<ArtifactStateRetirementReservation>; 64]) {
    for reservation in reservations {
        release_artifact_state_retirement(reservation);
    }
}

fn artifact_state_vacant_retirement_slot(tier: usize, slots: &[Option<ArtifactStateRetirementCursor>]) -> Option<usize> {
    let reserved = ARTIFACT_STATE_RETIREMENT_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_artifact_state_owner(owner: ArtifactStateRetirementCursor) {
    let reservation = owner.retirement.unwrap_or(ArtifactStateRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

#[cfg(test)]
fn retire_artifact_state_owner(owner: ArtifactStateRetirementCursor) -> Result<(), ArtifactStateRetirementCursor> {
    if owner.retirement.is_some() {
        install_reserved_artifact_state_owner(owner);
        return Ok(());
    }
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = artifact_state_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = artifact_state_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = artifact_state_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

/// 🧹️ Advances one parked state retirement — a replaced or removed value, or a refused
/// staging graph. Every state apply drives it until nothing is parked, so a replaced value's pages
/// and its I/O operation never outlive the write that replaced it; parked owners used to wait for a
/// maintenance pass nothing ran, until the process ran out of I/O operations and refused every write.
pub fn artifact_state_retirement_maintenance_step() -> Result<bool, DbError> {
    let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = artifact_state_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("artifact quarantine retirement changed owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = artifact_state_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("artifact overflow retirement changed owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("artifact retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for ArtifactStateRetirementCursor {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            install_reserved_artifact_state_owner(std::mem::replace(self, Self::empty()));
        }
    }
}
//#endregion 🔖️StateRetirement

/// 🏗️ A document's materialized state: a flat `db_state::PMap` from path to raw value
/// bytes, plus a per-path last-writer map for `submit`'s local, path-granular conflict detection
/// (see `🔖️Conflict`'s doc on why this stays local rather than `db_conflict`-backed). `values` uses
/// `PMap` (not a mutable `HashMap`) specifically so `content_hash` — the `Frontier.chain_hash`
/// source — is a real content-addressed digest of the whole state, not an incidental byte count,
/// and so `PMap::iter` gives `snapshot_now`/`query` a cheap, complete enumeration.
struct DocumentState {
    values: db_state::RetainedStateMap,
    last_writer: db_state::PMap<String, protocol::MutationId>,
}

/// 🗺️ One envelope planned by `ArtifactEngine::plan_one` and not applied yet: its path entries with every value
/// already encoded as the state stores it (`None` deletes), the regions it touches and the conflicts it meets.
struct PlannedEntries {
    pub entries: Vec<(String, Option<Vec<u8>>)>,
    pub touched: db_state::TouchedSet,
    pub conflicts: Vec<ConflictRecord>,
}

impl DocumentState {
    // 🚫️async: E1 pure constructor, `db_state::PMap::new` is sync — see R9
    pub fn new() -> DocumentState {
        DocumentState { values: db_state::RetainedStateMap::new(), last_writer: db_state::PMap::new() }
    }

    pub fn get(&self, path: &str) -> Option<&db_io_core::DbIoPages> {
        self.values.get(path)
    }

    async fn content_hash(&self) -> Result<ContentHash, DbError> {
        let mut control = db_state::StateCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        self.values.content_hash(&mut control).await
    }

    /// 🧭️ What applying one envelope's flattened path-value entries would touch and conflict with, decided
    /// without applying anything: a path's last writer is `written`'s (an earlier envelope of the same batch) or else
    /// the state's own, and a conflict is a last writer that is neither `mutation_id` itself nor a declared
    /// `dependencies` member. `written` gains this envelope's paths.
    fn plan_entries(&self, mutation_id: &protocol::MutationId, dependencies: &[protocol::MutationId], entries: &[(String, Option<DslValue>)], written: &mut HashMap<String, protocol::MutationId>) -> Result<(db_state::TouchedSet, Vec<ConflictRecord>), DbError> {
        check_len(entries.len() as u64, 64, "db_artifact::retained_state_mutations")?;
        let mut touched = db_state::TouchedSet::new();
        let mut conflicts = Vec::new();
        for (path, _) in entries {
            if let Some(previous_writer) = written.get(path).or_else(|| self.last_writer.get(path)) {
                if previous_writer != mutation_id && !dependencies.contains(previous_writer) {
                    conflicts.push(ConflictRecord { command_id: mutation_id.clone(), conflicting_with: previous_writer.clone(), path: path.clone() });
                }
            }
            touched.record(db_state::TouchedRegion::write(path.clone()));
            written.insert(path.clone(), mutation_id.clone());
        }
        Ok((touched, conflicts))
    }

    /// ✍️ Applies one planned envelope's entries ([`Self::plan_entries`]): every value, already encoded, set at
    /// its path, every `None` a deletion, and `mutation_id` recorded as each path's last writer.
    async fn apply_entries(&mut self, mutation_id: &protocol::MutationId, entries: Vec<(String, Option<Vec<u8>>)>) -> Result<(), DbError> {
        while artifact_state_retirement_maintenance_step()? {}
        if entries.is_empty() {
            return Ok(());
        }
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let mut staged: [Option<db_state::StateEntry>; 64] = std::array::from_fn(|_| None);
        let mut retirements: [Option<ArtifactStateRetirementReservation>; 64] = [None; 64];
        for index in 0..entries.len() {
            let Some(reservation) = reserve_artifact_state_retirement() else {
                release_artifact_state_retirements(&mut retirements);
                return Err(DbError::Unavailable("artifact state retirement pressure refused admission".to_string()));
            };
            retirements[index] = Some(reservation);
        }
        let mut paths = Vec::with_capacity(entries.len());
        for (index, (path, value)) in entries.into_iter().enumerate() {
            if let Some(bytes) = value {
                match db_state::StateEntry::try_admit(&path, bytes, MAX_STATE_PAGE_VALUE_BYTES, &mut control).await {
                    Ok(entry) => staged[index] = Some(entry),
                    Err(rejected) => {
                        let error = format!("retained state admission failed: {}", rejected.error());
                        let error = if matches!(rejected.error(), DbError::Unavailable(_)) { DbError::Unavailable(error) } else { DbError::InvalidArgument(error) };
                        let retirement = retirements[index].take().ok_or_else(|| DbError::Internal("retained state refusal lost retirement preflight".to_string()))?;
                        release_artifact_state_retirements(&mut retirements);
                        install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::rejected(retirement, rejected, staged));
                        while artifact_state_retirement_maintenance_step()? {}
                        return Err(error);
                    }
                }
            }
            paths.push(path);
        }
        for (index, path) in paths.into_iter().enumerate() {
            let retirement = retirements[index].take().ok_or_else(|| DbError::Internal("retained state staging lost retirement admission".to_string()))?;
            match staged[index].take() {
                Some(entry) => {
                    let replaced = match self.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(rejected) => {
                            staged[index] = Some(rejected);
                            release_artifact_state_retirements(&mut retirements);
                            install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::staged(retirement, staged));
                            return Err(DbError::LimitExceeded("retained state entries"));
                        }
                    };
                    match replaced {
                        Some(replaced) => install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::entry(retirement, replaced)),
                        None => release_artifact_state_retirement(&mut Some(retirement)),
                    }
                }
                None => match self.values.remove(&path) {
                    Some(removed) => install_reserved_artifact_state_owner(ArtifactStateRetirementCursor::entry(retirement, removed)),
                    None => release_artifact_state_retirement(&mut Some(retirement)),
                },
            }
            self.last_writer = self.last_writer.insert(path, mutation_id.clone());
        }
        while artifact_state_retirement_maintenance_step()? {}
        Ok(())
    }
}
//#endregion 🔖️State
//#region 🔖️Engine
/// ⚙️ Construction-time configuration for one `ArtifactEngine`: `db_engine` sets `limits` and
/// `preview_ttl_ms` and spreads `..Default::default()` for the rest.
pub struct ArtifactEngineConfig {
    pub limits: DbLimits,
    /// 🔐️ The real authz/dedupe/DoS-budget gate `submit` admits every not-yet-committed
    /// envelope through and records its committed ones in — see `db_security::SecurityGate::admit_commands`
    /// and `record_committed`. Keyed per-envelope by a `Principal`
    /// synthesized from that envelope's own `actor` (a permissive `"member"` role, `"default"`
    /// tenant) — `SubmitOptions` stays durability-only (see its doc) so this crate's dedupe/authz
    /// story does not require a caller to separately authenticate every submit call.
    pub security: db_security::SecurityGate,
    // 🔀️ dedyn-emit-runtime, O1/R11(c): every real call site (this crate's own `default()`,
    // `db_engine::document_engine_config`'s `other_defaults.emit` spread) constructs `NullEmit` and
    // nothing else — the field is stored but never actually called (`grep '.emit(' this crate: zero
    // hits). Unlike `db_security::SecurityGate` (which genuinely needs `E: Emit` generic so its own
    // tests can inject a `RecordingEmit`), there is no second implementor anywhere in this crate's
    // call graph, so O1 takes the "exactly one impl" branch: concrete `NullEmit`, no `dyn`, no
    // generic param added to this config type.
    pub emit: Arc<NullEmit>,
    pub preview_ttl_ms: u64,
    /// 🧬️ Projection factory: `submit`'s project step registers a fresh
    /// `db_projection::ProjectionEngine` from this on every call it needs one (see `🔖️Engine`'s doc
    /// for why a factory rather than a stored, already-built engine — `db_projection::
    /// ProjectionEngine::new`'s borrowed-`IndexStorage` + owned-`Vec<E>` shape does not
    /// semio_compose_rs with `ArtifactEngine` owning its storage as `Arc<dyn DbStorage>` without
    /// becoming self-referential). Defaults to no projections registered — `db_projection::
    /// NoProjections` (dedyn-fw-os-guestruntime, O1/R1: no first-party `dyn ErasedProjection`
    /// trait object) is uninhabited, so this factory can never actually return anything today; not
    /// one call site repo-wide overrides it with anything else. The day a real caller wants to
    /// register a projection here, it swaps `NoProjections` for its own closed `ErasedProjection`
    /// enum (R11) — this field's own `dyn Fn` closure stays (`dyn Fn` is R1-legal, a std trait).
    pub projections: Arc<dyn Fn() -> Vec<db_projection::NoProjections> + Send + Sync>,
}

impl Default for ArtifactEngineConfig {
    fn default() -> Self {
        let limits = DbLimits::default();
        let policy = db_security::RoleBasedPolicy::new().with_grant(db_security::Grant::allow("member", &["**"], &[db_security::Action::Read, db_security::Action::Write]));
        ArtifactEngineConfig {
            preview_ttl_ms: limits.max_preview_ttl_ms,
            limits,
            security: db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 4_096), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit)),
            emit: Arc::new(NullEmit),
            projections: Arc::new(Vec::new),
        }
    }
}

/// 🎭️ The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews and index backlog — everything
/// `ArtifactAuthority` (the `db_actor`-mailbox wrapper below) drives in finite process-pool turns.
/// The engine is moved only between serialized turns; callers can also use it directly wherever a
/// mailbox is unnecessary (for example this crate's own tests).
pub struct ArtifactEngine {
    pub document: ArtifactId,
    protocol_document: protocol::ArtifactId,
    pub storage: Arc<db_storage::DbBackend>,
    pub wal: db_wal::ArtifactWal,
    pub state: DocumentState,
    applied: HashMap<String, protocol::MutationEnvelope>,
    applied_receipts: HashMap<String, CommandReceipt>,
    applied_receipt_order: VecDeque<String>,
    durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
    durable_group_edit_ids: HashSet<String>,
    recent_targets: HashMap<String, Vec<String>>,
    actor_seq: HashMap<String, u64>,
    pub frontier: Frontier,
    pub head_edit_id: Option<protocol::MutationId>,
    previews: db_preview::PreviewStore,
    recent_touches: VecDeque<db_conflict::CommandTouch>,
    live_queries: HashMap<u64, db_query::LiveQuery>,
    next_live_query_id: u64,
    index_backlog: ArtifactIndexBacklog,
    pub config: ArtifactEngineConfig,
}

/// 🗂️ Index entries of commits that are durable in the WAL but not yet written as index runs.
/// A commit appends to it; after the commit's receipt was sent the runner writes every full run
/// (`db_index::RUN_ENTRIES_MAX` entries) through the kinds' owned runs — no listing, level folds
/// bounded per append — so the four index kinds cost one run write per 64 entries and no Ack waits
/// for index I/O. The WAL stays the source of truth: nothing here is durable, and opening a document
/// refills the backlog from the WAL suffix past what each kind's newest run already records.
#[derive(Default)]
struct ArtifactIndexBacklog {
    commands: VecDeque<(u64, db_index::RecordLocation)>,
    inverses: VecDeque<(u64, db_index::RecordLocation)>,
    actor_seqs: VecDeque<(ActorId, u64, u64)>,
    frontiers: VecDeque<Frontier>,
    runs: Option<ArtifactIndexRuns>,
}

/// 🗂️ The four kinds' owned runs, listed once on the backlog's first write and kept by its appends; dropped after
/// a failed write, so the next write lists again what storage really holds.
struct ArtifactIndexRuns {
    commands: db_index::OwnedRuns,
    inverses: db_index::OwnedRuns,
    actor_seqs: db_index::OwnedRuns,
    frontiers: db_index::OwnedRuns,
}

/// 🧾️ How far each index kind of one document already records (its newest run's highest
/// command or commit seq): the replay on open re-queues exactly what lies beyond.
#[derive(Clone, Copy, Default)]
struct ArtifactIndexWatermarks {
    commands: u64,
    inverses: u64,
    actor_seqs: u64,
    frontiers: u64,
}

/// 🛑️ The most entries one index kind may keep waiting for a run write: a `submit` that finds more writes
/// them first, and refuses transiently — before any WAL write — while index storage keeps failing.
const INDEX_BACKLOG_ENTRIES_MAX: usize = 64 * db_index::RUN_ENTRIES_MAX;

/// 🧾️ How many batch receipts the engine keeps for a whole-batch resend to be answered with its original
/// receipt; an older resend is answered by the per-envelope dedupe with the current frontier.
const APPLIED_RECEIPTS_MAX: usize = 1_024;

impl ArtifactIndexBacklog {
    async fn watermarks(storage: &impl db_io_core::IndexStorage, document: &ArtifactId) -> Result<ArtifactIndexWatermarks, DbError> {
        Ok(ArtifactIndexWatermarks {
            commands: db_index::CommandIndex::new(storage, document.clone()).await.indexed_through().await?,
            inverses: db_index::InverseIndex::new(storage, document.clone()).await.indexed_through().await?,
            actor_seqs: db_index::ActorSeqIndex::new(storage, document.clone()).await.indexed_through().await?,
            frontiers: db_index::FrontierIndex::new(storage, document.clone()).await.indexed_through().await?,
        })
    }

    fn queue_command(&mut self, watermarks: &ArtifactIndexWatermarks, seq: u64, location: db_index::RecordLocation, actor: ActorId, actor_seq: u64) {
        if seq > watermarks.commands {
            self.commands.push_back((seq, location));
        }
        if seq > watermarks.inverses {
            self.inverses.push_back((seq, location));
        }
        if seq > watermarks.actor_seqs {
            self.actor_seqs.push_back((actor, actor_seq, seq));
        }
    }

    fn queue_frontier(&mut self, watermarks: &ArtifactIndexWatermarks, frontier: &Frontier) {
        if frontier.commit_seq > watermarks.frontiers {
            self.frontiers.push_back(frontier.clone());
        }
    }

    fn pending(&self) -> usize {
        self.commands.len().max(self.inverses.len()).max(self.actor_seqs.len()).max(self.frontiers.len())
    }

    /// 🚚️ Writes this backlog's runs: every full one, or everything including a partial tail
    /// (`drain`) before WAL history may be deleted. Each written run leaves the backlog at once, so a
    /// failed write resumes exactly where it stopped, and forgets the owned runs, which the next write
    /// lists again.
    async fn flush(&mut self, storage: &impl db_io_core::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let written = self.flush_owned(storage, document, drain).await;
        if written.is_err() {
            self.runs = None;
        }
        written
    }

    async fn flush_owned(&mut self, storage: &impl db_io_core::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let full = db_index::RUN_ENTRIES_MAX;
        let take = |len: usize| if len >= full { full } else if drain { len } else { 0 };
        if [self.commands.len(), self.inverses.len(), self.actor_seqs.len(), self.frontiers.len()].into_iter().all(|len| take(len) == 0) {
            return Ok(());
        }
        let commands = db_index::CommandIndex::new(storage, document.clone()).await;
        let inverses = db_index::InverseIndex::new(storage, document.clone()).await;
        let actor_seqs = db_index::ActorSeqIndex::new(storage, document.clone()).await;
        let frontiers = db_index::FrontierIndex::new(storage, document.clone()).await;
        let runs = match self.runs.as_mut() {
            Some(runs) => runs,
            None => self.runs.insert(ArtifactIndexRuns { commands: commands.owned_runs().await?, inverses: inverses.owned_runs().await?, actor_seqs: actor_seqs.owned_runs().await?, frontiers: frontiers.owned_runs().await? }),
        };
        while take(self.commands.len()) > 0 {
            let count = take(self.commands.len());
            let run: Vec<_> = self.commands.iter().take(count).copied().collect();
            commands.record_owned_run(&mut runs.commands, &run).await?;
            self.commands.drain(..count);
        }
        while take(self.inverses.len()) > 0 {
            let count = take(self.inverses.len());
            let run: Vec<_> = self.inverses.iter().take(count).copied().collect();
            inverses.record_owned_run(&mut runs.inverses, &run).await?;
            self.inverses.drain(..count);
        }
        while take(self.actor_seqs.len()) > 0 {
            let count = take(self.actor_seqs.len());
            let run: Vec<_> = self.actor_seqs.iter().take(count).cloned().collect();
            actor_seqs.record_owned_run(&mut runs.actor_seqs, &run).await?;
            self.actor_seqs.drain(..count);
        }
        while take(self.frontiers.len()) > 0 {
            let count = take(self.frontiers.len());
            let run: Vec<_> = self.frontiers.iter().take(count).cloned().collect();
            frontiers.record_owned_run(&mut runs.frontiers, &run).await?;
            self.frontiers.drain(..count);
        }
        Ok(())
    }
}

/// 🧭️ Consistency resolution of a mounted document: its current frontier is the engine's
/// own, and a historical frontier is found among the not-yet-written backlog before the index.
struct ArtifactConsistencyResolver<'a, S: db_io_core::IndexStorage> {
    pub current: Frontier,
    pending: &'a VecDeque<Frontier>,
    pub commits: db_index::CommitIndex<'a, S>,
    frontiers: db_index::FrontierIndex<'a, S>,
}

impl<S: db_io_core::IndexStorage> db_query::ConsistencyResolver for ArtifactConsistencyResolver<'_, S> {
    async fn current_frontier(&self) -> Result<Frontier, DbError> {
        Ok(self.current.clone())
    }

    async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
        let command_seq = self.commits.lookup(commit_id).await?.ok_or_else(|| DbError::NotFound(format!("unknown commit id {commit_id:?}")))?;
        if let Some(frontier) = self.pending.iter().find(|frontier| frontier.commit_seq == command_seq) {
            return Ok(frontier.clone());
        }
        self.frontiers.lookup(command_seq).await?.ok_or_else(|| DbError::NotFound(format!("no frontier recorded at command_seq {command_seq}")))
    }
}

/// 🧯️ Engine construction rejection that keeps every acquired WAL owner until bounded terminal close.
#[must_use = "engine construction rejection must reach terminal retained cleanup"]
pub enum ArtifactEngineOpenRejected {
    BeforeWal(DbError),
    WalOpen(db_wal::ArtifactWalOpenRejected),
    RetainedWal { cause: DbError, close_error: Option<DbError>, wal: db_wal::ArtifactWal },
}

impl ArtifactEngineOpenRejected {
    fn retained_wal(cause: DbError, wal: db_wal::ArtifactWal) -> Self {
        Self::RetainedWal { cause, close_error: None, wal }
    }

    fn retained_wal_after_cleanup_error(cause: DbError, close_error: DbError, wal: db_wal::ArtifactWal) -> Self {
        Self::RetainedWal { cause, close_error: Some(close_error), wal }
    }

    pub fn error(&self) -> &DbError {
        match self {
            Self::BeforeWal(cause) => cause,
            Self::WalOpen(rejected) => rejected.error(),
            Self::RetainedWal { cause, .. } => cause,
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::WalOpen(rejected) => rejected.release_error(),
            Self::RetainedWal { close_error, .. } => close_error.as_ref(),
            Self::BeforeWal(_) => None,
        }
    }

    pub fn has_retained_writer(&self) -> bool {
        match self {
            Self::BeforeWal(_) => false,
            Self::WalOpen(rejected) => rejected.has_release_owner(),
            Self::RetainedWal { .. } => true,
        }
    }

    pub async fn retry_close(self) -> Result<DbError, ArtifactEngineOpenRejected> {
        match self {
            Self::BeforeWal(cause) => Ok(cause),
            Self::WalOpen(rejected) => rejected.retry_close().await.map_err(Self::WalOpen),
            Self::RetainedWal { cause, close_error: _, mut wal } => match wal.close().await {
                Ok(()) => Ok(cause),
                Err(error) => Err(Self::RetainedWal { cause, close_error: Some(error), wal }),
            },
        }
    }
}

impl From<DbError> for ArtifactEngineOpenRejected {
    fn from(error: DbError) -> Self {
        Self::BeforeWal(error)
    }
}

impl From<db_wal::ArtifactWalOpenRejected> for ArtifactEngineOpenRejected {
    fn from(rejected: db_wal::ArtifactWalOpenRejected) -> Self {
        Self::WalOpen(rejected)
    }
}

impl std::fmt::Debug for ArtifactEngineOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ArtifactEngineOpenRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).field("has_retained_writer", &self.has_retained_writer()).finish()
    }
}

impl ArtifactEngine {
    /// 🔕️ One close step: progress wakes the caller's waker, a pending writer release parks it
    /// in the release signal until the backend's terminal notification.
    pub fn poll_close(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), DbError>> {
        if self.wal.poll_close(context)?.is_pending() {
            return std::task::Poll::Pending;
        }
        if self.state.values.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        std::task::Poll::Ready(Ok(()))
    }

    #[cfg(test)]
    pub async fn close(&mut self) -> Result<(), DbError> {
        std::future::poll_fn(|context| self.poll_close(context)).await
    }

    pub fn close_flush_pending(&self) -> bool {
        self.wal.has_pending()
    }

    /// 🅿️ The engine rests between turns: its WAL parks the document's writer (unless a group commit still waits), so a
    /// backend with more open documents than writers lends it to whichever document writes next.
    pub fn park_writer(&self) {
        self.wal.park_writer();
    }

    pub async fn close_flush(&mut self) -> Result<(), DbError> {
        let wal_facet = self.storage.wal().await;
        self.wal.close_flush(&wal_facet).await
    }

    /// 🌱️ Retained constructor used by the document authority. Every storage wait remains
    /// represented by this future so a pool worker only polls it once before yielding.
    pub async fn create_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<ArtifactEngine, ArtifactEngineOpenRejected> {
        let core_id = to_core_document_id(&document).await;
        let wal = db_wal::ArtifactWal::create(&storage.wal().await, core_id.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await.map_err(ArtifactEngineOpenRejected::WalOpen)?;
        Ok(ArtifactEngine::assemble(document, core_id, storage, wal, config).await)
    }

    /// 🌱️ Creates a brand-new document: a genesis WAL (segment 0) and an empty state.
    /// Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    /// Process/test entry-point convenience; live document authorities use `create_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<ArtifactEngine, ArtifactEngineOpenRejected> {
        db_actor::block_on(Self::create_retained(document, storage, config, now_ms))
    }

    /// 🚑️ Retained materialization as initial ⊕ snapshot ⊕ WAL suffix.
    pub async fn open_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<(ArtifactEngine, MaterializeReport), ArtifactEngineOpenRejected> {
        let core_id = to_core_document_id(&document).await;
        let mut report = MaterializeReport::default();

        let mut state = DocumentState::new();
        let mut applied_head_seq = 0u64;
        let snapshot_facet = storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        if let Some((generation, descriptor)) = snapshot_manager.load_latest(&core_id).await? {
            report.from_snapshot = true;
            report.snapshot_generation = Some(generation);
            let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
            let mut snapshot_cursor = snapshot_manager.chain_cursor(&core_id, generation, control);
            for hash in &descriptor.roots {
                let mut page_bytes = snapshot_cursor.read_page(*hash).await?;
                let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
                let mut decoder = StatePageDecodeCursor::new(&page_bytes, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                while let Some(entry) = decoder.next().await? {
                    let replaced = match state.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(mut rejected) => {
                            let _ = rejected.close_step()?;
                            return Err(ArtifactEngineOpenRejected::BeforeWal(DbError::LimitExceeded("retained state entries")));
                        }
                    };
                    if let Some(mut replaced) = replaced {
                        let _ = replaced.close_step()?;
                        drop(replaced);
                    }
                }
                let _ = decoder.close_step()?;
                drop(decoder);
                let _ = page_bytes.close_step()?;
                drop(page_bytes);
            }
            let _ = snapshot_cursor.close_step()?;
            applied_head_seq = descriptor.head_seq;
        }
        drop(snapshot_manager);
        drop(snapshot_facet);

        let (wal, wal_recovery) = db_wal::ArtifactWal::open(&storage.wal().await, core_id.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await.map_err(ArtifactEngineOpenRejected::WalOpen)?;
        report.torn_tail_bytes = wal_recovery.torn_tail_bytes;
        let mut engine = ArtifactEngine::assemble(document, core_id.clone(), storage.clone(), wal, config).await;
        engine.state = state;
        engine.frontier.head_seq = applied_head_seq;

        let replay = async {
            let wal_facet = storage.wal().await;
            let replay_cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let replay_control = db_wal_log::WalCursorControl::new(replay_cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
            let watermarks = ArtifactIndexBacklog::watermarks(&storage.index().await, &core_id).await?;
            let mut records = db_wal_log::replay_committed_document(&wal_facet, &core_id, replay_control).await?;
            let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(Arc::new(std::sync::atomic::AtomicBool::new(false)), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
            let mut turn = db_wal_log::WalTurn::start();
            let mut batch_ids: HashSet<String> = HashSet::new();
            let mut seen: u64 = 0;
            let mut replay_frontier = Frontier::genesis(core_id.clone());
            let mut replay_head_edit_id = None;
            let mut replay_projected = false;
            let result = async {
                loop {
                    records.renew_step()?;
                    decode_control.renew_step()?;
                    let mut transaction = match records.next_transaction_step().await? {
                        db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                        db_wal_log::WalCommittedStep::Yield => {
                            turn.step().await;
                            continue;
                        }
                        db_wal_log::WalCommittedStep::Done => break,
                    };
                    let transaction_id = transaction.transaction_id();
                    let segment_index = transaction.segment_index();
                    batch_ids.clear();
                    let mut frontier_seen = false;
                    let mut body_records = 0usize;
                    let mut durable_group = None;
                    loop {
                        let record = match transaction.next_record_step()? {
                            db_wal_log::WalCommittedRecordStep::Record(record) => record,
                            db_wal_log::WalCommittedRecordStep::Yield => {
                                turn.step().await;
                                continue;
                            }
                            db_wal_log::WalCommittedRecordStep::Done => break,
                        };
                        if frontier_seen {
                            return Err(DbError::Corrupt("artifact committed frontier is not terminal".to_string()));
                        }
                        body_records = body_records.checked_add(1).ok_or_else(|| DbError::LimitExceeded("artifact committed body records"))?;
                        if durable_group.is_some() {
                            return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                        }
                        match record {
                            db_wal_log::WalRecord::Command(bytes) => {
                                let envelope = db_sync::decode_wal_command(bytes, &mut decode_control)?;
                                if envelope.document_id.0 != core_id.0 {
                                    return Err(DbError::Corrupt("artifact envelope document differs".to_string()));
                                }
                                replay_head_edit_id = Some(envelope.mutation_id.clone());
                                seen += 1;
                                batch_ids.insert(envelope.mutation_id.0.clone());
                                let actor = to_core_actor_id(&envelope.actor).await;
                                if seen <= applied_head_seq {
                                    *engine.actor_seq.entry(envelope.actor.0.clone()).or_insert(0) += 1;
                                    engine.applied.insert(envelope.mutation_id.0.clone(), envelope.clone());
                                } else {
                                    let touched = match engine.plan_one(&envelope, &batch_ids, &mut HashMap::new()).await? {
                                        Some(plan) => {
                                            engine.commit_one(&envelope, plan.entries).await?;
                                            plan.touched
                                        }
                                        None => db_state::TouchedSet::new(),
                                    };
                                    let touch = command_touch(&envelope, &touched);
                                    engine.remember_target(&envelope);
                                    remember_recent_touch(&mut engine.recent_touches, touch);
                                    report.commands_replayed += 1;
                                }
                                let actor_seq = *engine.actor_seq.get(&envelope.actor.0).unwrap_or(&0);
                                engine.index_backlog.queue_command(&watermarks, seen, db_index::RecordLocation { segment: segment_index, offset: seen, len: 1 }, actor, actor_seq);
                            }
                            db_wal_log::WalRecord::Frontier(frontier) => {
                                if frontier.document != core_id {
                                    return Err(DbError::Corrupt("artifact frontier document differs".to_string()));
                                }
                                engine.index_backlog.queue_frontier(&watermarks, frontier);
                                frontier_seen = true;
                                replay_frontier = frontier.clone();
                                replay_projected = true;
                            }
                            db_wal_log::WalRecord::Event(bytes) => {
                                if body_records != 1 {
                                    return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                                }
                                let mut canonical_pack = Vec::with_capacity(bytes.len());
                                for fragment in bytes.fragments() {
                                    canonical_pack.extend_from_slice(fragment);
                                }
                                let record = store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?;
                                if record.document().artifact_id != core_id.0 {
                                    return Err(DbError::Corrupt("committed durable group decision targets a different replay document".to_string()));
                                }
                                durable_group = Some(record);
                            }
                            _ => {}
                        }
                        while transaction.close_record_step()? {
                            turn.step().await;
                        }
                        turn.step().await;
                    }
                    if !batch_ids.is_empty() && !frontier_seen {
                        return Err(DbError::Corrupt("artifact committed commands have no frontier".to_string()));
                    }
                    if let Some(record) = durable_group {
                        if body_records != 1 {
                            return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                        }
                        seen = seen.checked_add(1).ok_or_else(|| DbError::LimitExceeded("artifact replay head sequence"))?;
                        let receipt = store::durable_group::DurableOwnedGroupJournalReceiptV1 { anchor_sha256: record.anchor_sha256().to_string(), decision_sha256: record.decision_sha256().to_string(), transaction_id, segment_index };
                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
                        engine.durable_group_edit_ids.insert(record.parent_edit_id().to_string());
                        if !duplicate {
                            replay_frontier = Frontier {
                                document: core_id.clone(),
                                head_seq: replay_frontier.head_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group replay head sequence"))?,
                                commit_seq: replay_frontier.commit_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group replay commit sequence"))?,
                                chain_hash: record.parent_post_revision(),
                                epoch: replay_frontier.epoch,
                            };
                            replay_head_edit_id = Some(protocol::MutationId(record.parent_edit_id().to_string()));
                            remember_recent_touch(&mut engine.recent_touches, durable_group_touch(record.parent_edit_id()));
                            replay_projected = true;
                        }
                    }
                    transaction.finish()?;
                }
                if replay_projected {
                    engine.frontier = replay_frontier;
                    engine.head_edit_id = replay_head_edit_id;
                }
                Ok::<(), DbError>(())
            }
            .await;
            let closed = async {
                while records.close_owner_step()? {
                    semio_framework_async::yield_once().await;
                }
                Ok::<(), DbError>(())
            }
            .await;
            drop(records);
            drop(wal_facet);
            result.and(closed)
        }
        .await;
        if let Err(error) = replay {
            let state_close = async {
                while engine.state.values.close_step()? {
                    semio_framework_async::yield_once().await;
                }
                Ok::<(), DbError>(())
            }
            .await;
            let ArtifactEngine { wal, .. } = engine;
            if let Err(close_error) = state_close {
                return Err(ArtifactEngineOpenRejected::retained_wal_after_cleanup_error(error, close_error, wal));
            }
            return Err(ArtifactEngineOpenRejected::retained_wal(error, wal));
        }
        Ok((engine, report))
    }

    /// 🚑️ Process/test entry-point convenience; live authorities use `open_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(document: protocol::ArtifactId, storage: &Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<(ArtifactEngine, MaterializeReport), ArtifactEngineOpenRejected> {
        db_actor::block_on(Self::open_retained(document, storage.clone(), config, now_ms))
    }

    async fn assemble(protocol_document: protocol::ArtifactId, core_id: ArtifactId, storage: Arc<db_storage::DbBackend>, wal: db_wal::ArtifactWal, config: ArtifactEngineConfig) -> ArtifactEngine {
        let preview_budgets = db_preview::PreviewBudgets { default_ttl_ms: config.preview_ttl_ms, max_ttl_ms: config.preview_ttl_ms, ..db_preview::PreviewBudgets::default() };
        ArtifactEngine {
            document: core_id.clone(),
            protocol_document,
            storage,
            wal,
            state: DocumentState::new(),
            applied: HashMap::new(),
            applied_receipts: HashMap::new(),
            applied_receipt_order: VecDeque::new(),
            durable_group_receipts: HashMap::new(),
            durable_group_edit_ids: HashSet::new(),
            recent_targets: HashMap::new(),
            actor_seq: HashMap::new(),
            frontier: Frontier::genesis(core_id.clone()),
            head_edit_id: None,
            previews: db_preview::PreviewStore::new(core_id, preview_budgets),
            recent_touches: VecDeque::new(),
            live_queries: HashMap::new(),
            next_live_query_id: 0,
            index_backlog: ArtifactIndexBacklog::default(),
            config,
        }
    }

    /// 🧭️ Plans `envelope` against this engine and the batch so far without applying anything: `None` when it is
    /// already applied (the per-envelope half of the dedupe law), else its decoded entries — values encoded as the state
    /// stores them — with the regions it touches and the conflicts it meets. Refuses an envelope whose dependency is
    /// neither applied, nor earlier in `batch_ids`, nor folded by a durable group decision.
    async fn plan_one(&self, envelope: &protocol::MutationEnvelope, batch_ids: &HashSet<String>, written: &mut HashMap<String, protocol::MutationId>) -> Result<Option<PlannedEntries>, DbError> {
        if self.applied.contains_key(&envelope.mutation_id.0) {
            return Ok(None);
        }
        for dependency in &envelope.dependencies {
            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) && !self.names_durable_group_operation(&dependency.0) {
                return Err(DbError::InvalidArgument(format!("operation {} depends on unseen operation {}", envelope.mutation_id.0, dependency.0)));
            }
        }
        let entries = diff_entries(&envelope.diff).await?;
        check_len(entries.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::diff_entries")?;
        let (touched, conflicts) = self.state.plan_entries(&envelope.mutation_id, &envelope.dependencies, &entries, written)?;
        let entries = entries.into_iter().map(|(path, value)| (path, value.map(|value| store::pack_rt::encode_wire_value(&value)))).collect();
        Ok(Some(PlannedEntries { entries, touched, conflicts }))
    }

    /// ✍️ Applies one planned envelope to the document state and records it as applied.
    async fn commit_one(&mut self, envelope: &protocol::MutationEnvelope, entries: Vec<(String, Option<Vec<u8>>)>) -> Result<(), DbError> {
        self.state.apply_entries(&envelope.mutation_id, entries).await?;
        self.applied.insert(envelope.mutation_id.0.clone(), envelope.clone());
        *self.actor_seq.entry(envelope.actor.0.clone()).or_insert(0) += 1;
        Ok(())
    }

    /// 🪪️ The event log's twin of `protocol::fold_history`'s admission, so the log never holds a history transition every
    /// replica refuses: refused before the WAL, whatever the merge policy, with one typed message.
    /// - A payload that does not decode: [`MALFORMED_HISTORY_TRANSITION_CODE`].
    /// - An undo belongs to its author: a `Revert` or `Reinstate` naming an operation this document's log (or an envelope of
    ///   the same batch) records under another actor: [`FOREIGN_HISTORY_TRANSITION_CODE`]. An operation the log does not
    ///   know belongs to anyone (the replicas' fold says the same).
    /// - A `Supersede` has no ownership rule: editing history is non-destructive and reversible, and write permission is the
    ///   security gate's. It must depend on exactly its targets ([`MALFORMED_HISTORY_TRANSITION_CODE`]), and every target
    ///   must be committed, `earlier` in its batch, or folded by a durable group decision ([`UNKNOWN_SUPERSEDE_TARGET_CODE`]).
    ///
    /// 📋️ `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §2, §9.1.
    fn history_transition_refusal(&self, envelope: &protocol::MutationEnvelope, batch: &[protocol::MutationEnvelope], earlier: &HashSet<String>) -> Option<protocol::MutationMessage> {
        let transition = match protocol::history_transition_from_envelope(envelope) {
            Ok(Some(transition)) => transition,
            Ok(None) => return None,
            Err(_) => return Some(protocol::MutationMessage::error(MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE)),
        };
        match transition {
            protocol::HistoryTransition::Revert { mutation_ids } | protocol::HistoryTransition::Reinstate { mutation_ids } => {
                let author = |id: &protocol::MutationId| self.applied.get(&id.0).map(|applied| &applied.actor).or_else(|| batch.iter().find(|earlier| &earlier.mutation_id == id).map(|earlier| &earlier.actor));
                mutation_ids.iter().any(|id| author(id).is_some_and(|actor| actor != &envelope.actor)).then(|| protocol::MutationMessage::error(FOREIGN_HISTORY_TRANSITION_CODE, FOREIGN_HISTORY_TRANSITION_MESSAGE))
            }
            protocol::HistoryTransition::Supersede(supersede) => {
                let targets = supersede.targets();
                if envelope.dependencies != targets {
                    return Some(protocol::MutationMessage::error(MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE));
                }
                targets
                    .into_iter()
                    .find(|target| !self.applied.contains_key(&target.0) && !earlier.contains(&target.0) && !self.names_durable_group_operation(&target.0))
                    .map(|target| protocol::MutationMessage::error(UNKNOWN_SUPERSEDE_TARGET_CODE, UNKNOWN_SUPERSEDE_TARGET_MESSAGE).at([target.0]))
            }
            protocol::HistoryTransition::Commit(_) | protocol::HistoryTransition::Branch { .. } | protocol::HistoryTransition::Checkout { .. } | protocol::HistoryTransition::Repin { .. } => None,
        }
    }

    /// 👁️ Whether `dependency` names an operation a committed durable group decision folded: the
    /// decision's edit itself, or `<edit>#<position>`.
    fn names_durable_group_operation(&self, dependency: &str) -> bool {
        self.durable_group_edit_ids.contains(dependency) || dependency.rsplit_once('#').is_some_and(|(edit, position)| !position.is_empty() && position.bytes().all(|byte| byte.is_ascii_digit()) && self.durable_group_edit_ids.contains(edit))
    }

    /// 🎯️ Keeps `envelope`'s declared target while its touch is in the recent commit window, so a later
    /// concurrent write can be graded against it; targets that left the window are dropped with it.
    fn remember_target(&mut self, envelope: &protocol::MutationEnvelope) {
        if self.recent_targets.len() >= MAX_RECENT_TOUCHES {
            let window: HashSet<&str> = self.recent_touches.iter().map(|touch| touch.command_id.0.as_str()).collect();
            self.recent_targets.retain(|command, _| window.contains(command.as_str()));
        }
        self.recent_targets.insert(envelope.mutation_id.0.clone(), envelope.target.clone());
    }

    /// 🚦️ The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →
    /// conflict → execute → WAL append → durability → publish → project →
    /// preview-reconcile → live-query notify → receipt.
    // 🔒️ `batch` is taken by value deliberately, not just because the current body happens not to
    // move it: submitting a batch is the one place in this API where the caller's copy is
    // logically spent (it becomes part of durable history), matching `ArtifactMessage::Submit`'s
    // owned payload on the actor-mailbox side one level up. Keeping ownership here leaves room for
    // a future zero-copy WAL append that moves `batch.envelopes` directly into the write path.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn submit(&mut self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        // admit
        check_len(batch.envelopes.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::batch_commands")?;
        for envelope in &batch.envelopes {
            if envelope.document_id != self.protocol_document {
                return Err(DbError::InvalidArgument(format!("envelope targets document {:?} but this actor owns {:?}", envelope.document_id, self.protocol_document)));
            }
        }
        let command_id = batch.envelopes.last().expect("CommandBatch::new guarantees at least one envelope").mutation_id.clone();

        for envelope in &batch.envelopes {
            if let Some(applied) = self.applied.get(&envelope.mutation_id.0) {
                if applied.diff != envelope.diff || applied.inverse != envelope.inverse || applied.dependencies != envelope.dependencies {
                    return Err(DbError::Conflict(format!("mutation id {} is already committed with different content", envelope.mutation_id.0)));
                }
            }
        }

        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }

        // index backlog bound: write what earlier commits queued before this one adds more; refuse
        // transiently while index storage fails, before the WAL holds anything of this batch
        if self.index_backlog.pending() > INDEX_BACKLOG_ENTRIES_MAX {
            if let Err(error) = self.maintain_index().await {
                return Err(DbError::Unavailable(format!("index backlog of {} entries cannot be written: {error}", self.index_backlog.pending())));
            }
        }

        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut written: HashMap<String, protocol::MutationId> = HashMap::new();
        let mut planned: Vec<(&protocol::MutationEnvelope, PlannedEntries)> = Vec::new();
        let mut commands: Vec<Vec<u8>> = Vec::new();
        for envelope in &batch.envelopes {
            if batch_ids.contains(&envelope.mutation_id.0) {
                return Err(DbError::Conflict(format!("mutation id {} appears twice in one batch", envelope.mutation_id.0)));
            }
            if self.applied.contains_key(&envelope.mutation_id.0) {
                batch_ids.insert(envelope.mutation_id.0.clone());
                continue;
            }

            // authz: the `db_security::SecurityGate`, keyed by a permissive principal synthesized from the
            // envelope's own actor (see `ArtifactEngineConfig::security`'s doc). An already-committed
            // envelope (content-checked above) skipped it: it is an idempotent resend.
            let principal = db_security::Principal::new(envelope.actor.clone(), db_security::TenantId::from("default"), vec!["member".to_string()]);
            self.config.security.admit_commands(&principal, &db_security::TenantId::from("default"), &envelope.document_id, &envelope.diff.schema.0, &[(&envelope.actor, &envelope.mutation_id)], now_ms).await?;
            if let Some(refusal) = self.history_transition_refusal(envelope, &batch.envelopes, &batch_ids) {
                return Err(DbError::Rejected { policy: options.policy, worst: semio_framework_diagnostic::Severity::Error, messages: vec![refusal] });
            }

            // 🎯️ W5: `WalRecord::Command`'s bytes are `protocol::encode_envelope`'s binary record now
            // (M-C's "storage AND communication both binary") — `db_sync::replay_sync_state` reads
            // these same WAL records via `decode_command_envelope` (the same binary codec) to serve
            // semio_hub bootstrap/catch-up, so this crate's own write-and-read-back convention below must
            // agree with it byte-for-byte.
            let mut envelope_bytes = Vec::new();
            protocol::encode_envelope(envelope, &mut envelope_bytes);
            check_len(envelope_bytes.len() as u64, self.config.limits.max_command_bytes, "db_artifact::envelope_bytes")?;

            let plan = self.plan_one(envelope, &batch_ids, &mut written).await?;
            batch_ids.insert(envelope.mutation_id.0.clone());
            if let Some(plan) = plan {
                planned.push((envelope, plan));
                commands.push(envelope_bytes);
            }
        }

        if planned.is_empty() {
            // Every envelope in this (re-)submitted batch was already durable individually — a
            // full no-op commit, per-envelope half of the dedupe law (see `plan_one`'s doc).
            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await?), messages: Vec::new() };
            self.remember_receipt(&receipt);
            return Ok(receipt);
        }

        // outcome step (contract §C9): union this batch's own `db_conflict::ConflictDetector`
        // findings (probed against recent commit history) into graded `protocol::MutationMessage`s,
        // then let `options.policy` decide before touching `self.recent_touches`/`self.outbox`/the
        // WAL at all — a rejected batch must leave every one of those untouched.
        let batch_touches: Vec<db_conflict::CommandTouch> = planned.iter().map(|(envelope, plan)| command_touch(envelope, &plan.touched)).collect();
        let mut messages: Vec<protocol::MutationMessage> = Vec::new();
        for (index, (envelope, _)) in planned.iter().enumerate() {
            for unseen in unseen_concurrent_writes(&self.recent_touches, &batch_touches[..index], &batch_touches[index], envelope) {
                let unseen_target = self.recent_targets.get(&unseen.command_id.0).map(Vec::as_slice).or_else(|| planned[..index].iter().find(|(earlier, _)| earlier.mutation_id == unseen.command_id).map(|(earlier, _)| earlier.target.as_slice())).unwrap_or(&[]);
                messages.extend(grade_concurrent_write(&batch_touches[index], &envelope.target, unseen, unseen_target).await);
            }
        }
        if let Some(worst) = protocol::worst_level(&messages) {
            if options.policy.rejects(worst) {
                return Err(DbError::Rejected { policy: options.policy, worst, messages });
            }
        }

        let head_seq = self.frontier.head_seq.checked_add(planned.len() as u64).ok_or(DbError::LimitExceeded("db_artifact::head_seq"))?;
        let mut records = db_wal_log::WalRecordBatch::new();
        let _ = records.push(db_wal_log::WalRecord::Frontier(Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: [0; 32], epoch: self.frontier.epoch }));
        self.wal.preflight_submit(&commands, &records)?;
        let _ = records.close_step()?;
        let wal_facet = self.storage.wal().await;
        self.wal.begin_write(&wal_facet).await?;
        drop(wal_facet);

        let mut touched_all = db_state::TouchedSet::new();
        let mut conflicts_all: Vec<ConflictRecord> = Vec::new();
        let mut applied_actor_seqs: Vec<u64> = Vec::with_capacity(planned.len());
        let mut newly_applied: Vec<(&protocol::MutationEnvelope, db_state::TouchedSet)> = Vec::with_capacity(planned.len());
        for (envelope, plan) in planned {
            self.commit_one(envelope, plan.entries).await?;
            applied_actor_seqs.push(*self.actor_seq.get(&envelope.actor.0).unwrap_or(&0));
            for region in &plan.touched.regions {
                touched_all.record(region.clone());
            }
            conflicts_all.extend(plan.conflicts);
            newly_applied.push((envelope, plan.touched));
        }

        // Bookkeeping for `preview_conflicts`'s real, additive `db_conflict::ConflictDetector`
        // integration (see its own doc) — `submit`'s own returned `ConflictRecord`s stay this
        // crate's original path-granular last-writer detection above (see `🔖️Conflict`'s doc). Only
        // reached once the outcome step above has accepted the batch.
        for (envelope, _) in &newly_applied {
            self.remember_target(envelope);
        }
        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }

        // publish: compute + WAL-append the new frontier in the same transaction as its commands
        let new_frontier = Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: self.state.content_hash().await?.0, epoch: self.frontier.epoch };
        let mut records = db_wal_log::WalRecordBatch::new();
        let _ = records.push(db_wal_log::WalRecord::Frontier(new_frontier.clone()));

        // WAL append + durability (ArtifactWal::submit wraps `records` in its own TxBegin/TxCommit)
        let wal_facet = self.storage.wal().await;
        let appended = self.wal.submit(&wal_facet, &commands, &records, options.durability, now_ms).await?;
        drop(wal_facet);
        self.config.security.record_committed(&newly_applied.iter().map(|(envelope, _)| (&envelope.actor, &envelope.mutation_id)).collect::<Vec<_>>(), now_ms);
        let _ = records.close_step()?;
        drop(records);
        self.frontier = new_frontier.clone();

        // publish: index entries join the backlog; the runner writes its full runs after the receipt
        let base_seq = self.frontier.head_seq - newly_applied.len() as u64;
        let queued = ArtifactIndexWatermarks::default();
        for (offset, ((envelope, _), actor_seq)) in newly_applied.iter().zip(&applied_actor_seqs).enumerate() {
            let seq = base_seq + offset as u64 + 1;
            let location = db_index::RecordLocation { segment: appended.segment_index, offset: seq, len: 1 };
            self.index_backlog.queue_command(&queued, seq, location, to_core_actor_id(&envelope.actor).await, *actor_seq);
        }
        self.index_backlog.queue_frontier(&queued, &new_frontier);

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let index_facet = self.storage.index().await;
            let engine = db_projection::ProjectionEngine::new(&index_facet, self.document.clone(), projection_classes).await?;
            for (offset, (envelope, touched)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched).await?;
            }
        }

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all }, &db_preview::DbConflictOracle::default());
        self.head_edit_id = Some(command_id.clone());

        // live-query notify
        // 🩹️ R13: was `let _ = self.refresh_live_queries();` — silently dropped the future, so
        // this notify step never actually ran. `.await`ed now; the `Vec<(u64, QueryDiff)>` result
        // is genuinely unneeded here (callers observe diffs via their own subscription poll), so
        // `let _ =` on the awaited, resolved value is the correct fire-and-forget shape.
        let _ = self.refresh_live_queries().await;

        // receipt
        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };
        self.remember_receipt(&receipt);
        Ok(receipt)
    }

    /// 🧾️ Keeps `receipt` for a whole-batch resend, forgetting the oldest beyond [`APPLIED_RECEIPTS_MAX`].
    fn remember_receipt(&mut self, receipt: &CommandReceipt) {
        if self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone()).is_none() {
            self.applied_receipt_order.push_back(receipt.command_id.0.clone());
        }
        while self.applied_receipt_order.len() > APPLIED_RECEIPTS_MAX {
            if let Some(oldest) = self.applied_receipt_order.pop_front() {
                self.applied_receipts.remove(&oldest);
            }
        }
    }

    /// 🗂️ Writes every full index run the commits so far queued — their WAL records are the durable truth
    /// ([`ArtifactIndexBacklog`]). The document runner calls it after a commit's receipt was sent, so no Ack waits
    /// for index I/O; `submit` calls it first only when the backlog outgrew [`INDEX_BACKLOG_ENTRIES_MAX`].
    pub async fn maintain_index(&mut self) -> Result<(), DbError> {
        let index_facet = self.storage.index().await;
        self.index_backlog.flush(&index_facet, &self.document, false).await
    }

    /// ↩️ The inverse-undo pipeline: looks up `target`'s already-applied envelope, flips its
    /// `inverse` into a compensating envelope's `diff` (and vice versa, so the compensating
    /// envelope's OWN inverse can re-undo the undo), and submits it as a fresh, ordinary command
    /// depending on `target` — undo is just another commit, not a WAL rewrite. This is the crate's
    /// "inverse undo, using protocol's inverse-operation machinery".
    pub async fn undo(&mut self, target: &protocol::MutationId, undo_mutation_id: protocol::MutationId, actor: protocol::ActorId, now_ms: u64) -> Result<CommandReceipt, DbError> {
        let original = self.applied.get(&target.0).cloned().ok_or_else(|| DbError::NotFound(format!("operation {} not found for undo", target.0)))?;
        let undo_diff_entries = inverse_entries(&original.inverse).await?;
        let redo_inverse_entries = diff_entries(&original.diff).await?;
        let compensating = protocol::MutationEnvelope {
            mutation_id: undo_mutation_id,
            document_id: self.protocol_document.clone(),
            actor,
            dependencies: vec![target.clone()],
            observed: None,
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: original.inverse.schema.clone(), payload: encode_pathmap(&entries_to_value(&undo_diff_entries)).await },
            inverse: protocol::InverseMutation { schema: original.diff.schema, payload: encode_pathmap(&entries_to_value(&redo_inverse_entries)).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
            transaction: None, verb: None, line: None,
        };
        self.submit(CommandBatch::new(vec![compensating]).await?, SubmitOptions::default(), now_ms).await
    }

    pub async fn get(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let Some(value) = self.state.get(path) else { return Ok(None) };
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        Ok(Some(db_query::QueryBytes::copy_from_pages(value, &mut control).await?))
    }

    pub async fn frontier(&self) -> Frontier {
        self.frontier.clone()
    }

    /// 📍️ Captures the frontier and its exact committed tip in one serialized actor turn.
    pub async fn checkpoint_publication_snapshot(&self) -> CheckpointPublicationSnapshot {
        CheckpointPublicationSnapshot { frontier: self.frontier.clone(), head_edit_id: self.head_edit_id.clone() }
    }

    pub fn history_replay(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> HistoryReplayFuture {
        HistoryReplayFuture::new(self.storage.clone(), self.document.clone(), operation_generation, cancelled, reservation)
    }

    /// 📓️ Appends one Store-admitted fixed-three decision as a forced-Fsync event on this
    /// authority's already-retained WAL writer. Admission and capacity rejection complete before
    /// storage I/O; every error returned by `submit` remains an uncertain durable outcome.
    pub async fn append_durable_group_decision(&mut self, record: store::durable_group::DurableOwnedGroupJournalRecordV1, cancelled: Arc<std::sync::atomic::AtomicBool>, now_ms: u64) -> Result<ArtifactDurableGroupJournalAppendV1, DbError> {
        if let Some(receipt) = self.durable_group_receipts.get(record.decision_sha256()) {
            return Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt.clone()));
        }
        if cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(ArtifactDurableGroupJournalAppendV1::Absent);
        }
        let head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
        let next_frontier = Frontier {
            document: self.document.clone(),
            head_seq: self.frontier.head_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group head sequence"))?,
            commit_seq: self.frontier.commit_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group commit sequence"))?,
            chain_hash: record.parent_post_revision(),
            epoch: self.frontier.epoch,
        };
        let (canonical_pack, decision_sha256, anchor_sha256, document) = record.into_parts();
        if document.artifact_id != self.protocol_document.0 {
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(DbError::InvalidArgument("durable group decision anchor targets a different document".to_string())));
        }
        let mut control = db_wal_log::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let event = match admit_wal_bytes(canonical_pack, db_io_core::DB_IO_MAX_READ_BYTES, &mut control).await {
            Ok(event) => event,
            Err(error) => return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(error)),
        };
        let mut records = db_wal_log::WalRecordBatch::new();
        if let Err(error) = push_wal_record(&mut records, db_wal_log::WalRecord::Event(event), &mut control).await {
            let close = close_wal_record_batch(&mut records).await;
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(close.err().unwrap_or(error)));
        }
        if let Err(error) = self.wal.preflight_submit(&[], &records) {
            let close = close_wal_record_batch(&mut records).await;
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(close.err().unwrap_or(error)));
        }
        if cancelled.load(std::sync::atomic::Ordering::Acquire) {
            close_wal_record_batch(&mut records).await?;
            return Ok(ArtifactDurableGroupJournalAppendV1::Absent);
        }
        let wal_facet = self.storage.wal().await;
        let append = self.wal.submit(&wal_facet, &[], &records, DurabilityClass::Fsync, now_ms).await;
        drop(wal_facet);
        let close = close_wal_record_batch(&mut records).await;
        let receipt = match (append, close) {
            (Ok(receipt), Ok(())) => receipt,
            (Err(error), Ok(())) => return Err(error),
            (Ok(_), Err(error)) => return Err(error),
            (Err(error), Err(close_error)) => return Err(DbError::Internal(format!("durable group WAL append failed: {error}; record retirement failed: {close_error}"))),
        };
        if !receipt.committed {
            return Err(DbError::Internal("forced-Fsync durable group decision did not commit".to_string()));
        }
        let receipt = store::durable_group::DurableOwnedGroupJournalReceiptV1 { anchor_sha256, decision_sha256, transaction_id: receipt.tx_id, segment_index: receipt.segment_index };
        self.frontier = next_frontier;
        self.durable_group_edit_ids.insert(head_edit_id.0.clone());
        remember_recent_touch(&mut self.recent_touches, durable_group_touch(&head_edit_id.0));
        self.head_edit_id = Some(head_edit_id);
        self.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt.clone());
        Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt))
    }

    /// 🧭 Replays every committed fixed-three Event in physical WAL order into one opaque
    /// retained target. The target, cursor pages, transaction gate and current Store recovery
    /// never cross the caller reply boundary.
    pub async fn recover_durable_group_decisions(&mut self, mut driver: ArtifactDurableGroupRecoveryDriverV1) -> Result<(), DbError> {
        if driver.owner.document() != &self.document {
            return Err(DbError::InvalidArgument("durable group recovery target differs from the document authority".to_string()));
        }
        while !driver.owner.drive_recovery_step()? {
            semio_framework_async::yield_once().await;
        }
        let wal_facet = self.storage.wal().await;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = db_wal_log::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
        let mut replay = db_wal_log::replay_committed_document(&wal_facet, &self.document, control).await?;
        let outcome = async {
            loop {
                replay.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
                let transaction = match replay.next_transaction_step().await? {
                    db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                    db_wal_log::WalCommittedStep::Yield => {
                        semio_framework_async::yield_once().await;
                        continue;
                    }
                    db_wal_log::WalCommittedStep::Done => break,
                };
                if let Some(witness) = committed_durable_group_decision_from_transaction(&self.document, transaction).await? {
                    driver.owner.accept(witness)?;
                    while !driver.owner.drive_recovery_step()? {
                        semio_framework_async::yield_once().await;
                    }
                }
            }
            driver.owner.finish_scan()
        }
        .await;
        loop {
            replay.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
            if !replay.close_owner_step()? {
                break;
            }
            semio_framework_async::yield_once().await;
        }
        if !replay.terminal_is_empty() {
            return Err(DbError::Internal("durable group WAL recovery cursor did not retire terminally".to_string()));
        }
        outcome
    }

    /// 🧹 Runs compaction inside this document authority while its WAL writer stays retained.
    pub async fn compact_retained(
        &mut self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<db_compact::CompactionReport, DbError> {
        let index_facet = self.storage.index().await;
        self.index_backlog.flush(&index_facet, &self.document, true).await?;
        drop(index_facet);
        db_compact::retained_compaction_with_wal(self.storage.clone(), &mut self.wal, holder, consolidate_snapshots, budget, now_ms, cancelled).await
    }

    //#region 🔖️Snapshot
    /// 📸️ Publishes a new `db_snapshot` generation of the whole current `DocumentState` —
    /// new this revision; the counterpart `open` reads back to accelerate materialization.
    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let encoder = StatePageEncodeCursor::try_new(&self.state.values, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let page = db_state::Page::try_from_pages(encoder.finish().await?).await?;
        let snapshot_facet = self.storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        let origin = if snapshot_manager.load_latest(&self.document).await?.is_some() { db_snapshot::SnapshotOrigin::Incremental } else { db_snapshot::SnapshotOrigin::FullBaseline };
        let body = db_snapshot::SnapshotBody {
            head_seq: self.frontier.head_seq,
            commit_seq: self.frontier.commit_seq,
            epoch: self.frontier.epoch,
            chain_hash: self.frontier.chain_hash,
            protocol_version: 1,
            base_pack_hash: None,
            roots: vec![page.hash],
            created_at_ms: now_ms,
        };
        snapshot_manager.publish(&self.document, origin, &[page], body).await
    }
    //#endregion 🔖️Snapshot

    //#region 🔖️Query
    /// 🔎️ One-shot query over the document's current materialized state, resolved under
    /// `consistency` via `db_index`'s `CommitIndex`/`FrontierIndex`. `StateQuerySource` always reads
    /// the CURRENT canonical state regardless of what `consistency` resolved to — a true
    /// point-in-time replay is `db_engine`'s documented deferred extension (see its own module doc).
    // 🔒️ Mirrors `submit`'s ownership rationale above: `ArtifactMessage::RunQuery` already owns
    // `query`/`consistency` one level up on the actor-mailbox side, and query descriptors are
    // small, cheap-to-move value types — taking them by reference here would only push the
    // ownership question onto every caller for no benefit.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        let index_facet = self.storage.index().await;
        let resolver = ArtifactConsistencyResolver {
            current: self.frontier.clone(),
            pending: &self.index_backlog.frontiers,
            commits: db_index::CommitIndex::new(&index_facet, self.document.clone()).await,
            frontiers: db_index::FrontierIndex::new(&index_facet, self.document.clone()).await,
        };
        // A fresh document has no recorded frontier yet; canonical reads still succeed via the
        // in-memory frontier, so only consult the resolver for modes that truly need the index.
        if !matches!(consistency, db_query::Consistency::Canonical) {
            db_query::resolve_consistency(&consistency, &resolver).await?;
        }
        let source = StateQuerySource(&self.state.values);
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        db_query::execute(&query, &source, None::<&db_query::NoFullTextLookup>, &db_query::QueryLimits::default(), &mut control).await
    }

    /// 📡️ Registers a live query, returning its subscription id — new this revision.
    pub async fn subscribe(&mut self, spec: db_query::LiveQuerySpec) -> u64 {
        let id = self.next_live_query_id;
        self.next_live_query_id += 1;
        self.live_queries.insert(id, db_query::LiveQuery::new(spec).await);
        id
    }

    pub async fn unsubscribe(&mut self, id: u64) {
        self.live_queries.remove(&id);
    }

    /// 📡️ Live-query notify: re-evaluates every registered live query and returns what
    /// changed. Called automatically at the end of `submit`; also callable directly.
    pub async fn refresh_live_queries(&mut self) -> Vec<(u64, db_query::QueryDiff)> {
        let source = StateQuerySource(&self.state.values);
        let limits = db_query::QueryLimits::default();
        let mut diffs = Vec::new();
        for (id, live_query) in self.live_queries.iter_mut() {
            let mut control = match db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536) {
                Ok(control) => control,
                Err(_) => continue,
            };
            if let Ok(diff) = live_query.refresh(&source, None::<&db_query::NoFullTextLookup>, &limits, &mut control).await {
                if !diff.added.is_empty() || !diff.removed.is_empty() || !diff.updated.is_empty() {
                    diffs.push((*id, diff));
                }
            }
        }
        diffs
    }
    //#endregion 🔖️Query

    //#region 🔖️Advisory
    /// 🔮️ Advisory-only, real `db_conflict::ConflictDetector` integration: runs `batch`'s
    /// envelopes' touched regions against recent commit history WITHOUT executing anything, using
    /// the family's real bloom-filter/kind-matrix machinery — a caller (e.g. a UI) can call this
    /// before `submit` to preview likely conflicts. `submit`'s own returned `ConflictRecord`s stay
    /// this crate's original path-granular last-writer detection (see `🔖️Conflict`'s doc on why
    /// `db_conflict`'s per-command-pair shape can't replace it without breaking `db_engine`'s frozen
    /// `CommandReceipt.conflicts` element type).
    pub async fn preview_conflicts(&self, batch: &CommandBatch) -> Result<Vec<db_conflict::ConflictRecord>, DbError> {
        let mut probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().collect();
        for envelope in &batch.envelopes {
            let entries = diff_entries(&envelope.diff).await?;
            let touched = entries_touched(&entries);
            probe.push(command_touch(envelope, &touched));
        }
        Ok(db_conflict::ConflictDetector::new().detect(&probe))
    }
    //#endregion 🔖️Advisory

    //#region 🔖️Preview
    /// 🌫️ Publishes a new preview overlaying the CURRENT committed state — never durable
    /// (never touches the WAL), per the contract's preview law. Backed by a real
    /// `db_preview::PreviewStore` this revision (previously a local, minimal stand-in).
    pub async fn publish_preview(&mut self, entries: &[(String, Option<serde_json::Value>)], now_ms: u64) -> Result<db_preview::PreviewId, DbError> {
        let dsl_entries: Vec<(String, Option<DslValue>)> = entries.iter().map(|(path, value)| (path.clone(), value.as_ref().map(semio_framework_value::DslValue::from))).collect();
        let touched = entries_touched(&dsl_entries);
        let envelope = protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(format!("preview-{}", entries.len())),
            document_id: self.protocol_document.clone(),
            actor: protocol::ActorId("preview".to_string()),
            dependencies: Vec::new(),
            observed: None,
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&entries_to_value(&dsl_entries)).await },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&semio_framework_value::DslValue::Object(vec![])).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
            transaction: None, verb: None, line: None,
        };
        self.previews.publish(db_preview::PublishPreviewRequest { document: self.document.clone(), actor: ActorId("preview".to_string()), key: format!("preview-{now_ms}"), base: self.frontier.clone(), envelope, touched, ttl_ms: None, now_ms })
    }

    /// 🌫️ The value a preview would show at `path`: the preview's own diff if it touches
    /// `path`, else falling through to the committed state.
    pub async fn preview_get(&self, id: &db_preview::PreviewId, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let preview = self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?;
        for (entry_path, value) in diff_entries(&preview.envelope.diff).await? {
            if entry_path == path {
                return match value {
                    Some(dsl_value) => {
                        let bytes = store::pack_rt::encode_wire_value(&dsl_value);
                        let mut writer = db_io_core::DbIoPageWriter::try_reserve(bytes.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
                        let mut offset = 0;
                        while offset < bytes.len() {
                            offset += writer.write_fragment(&bytes[offset..])?;
                        }
                        let pages = writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
                        Ok(Some(db_query::QueryBytes::from_pages(pages).map_err(|(error, mut pages)| {
                            let _ = pages.close_step();
                            drop(pages);
                            error
                        })?))
                    }
                    None => Ok(None),
                };
            }
        }
        self.get(path).await
    }

    pub async fn preview_status(&self, id: &db_preview::PreviewId) -> Result<db_preview::PreviewState, DbError> {
        Ok(self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?.state)
    }

    pub async fn withdraw_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.withdraw(id)
    }

    pub async fn reject_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.reject(id)
    }

    pub async fn expire_previews(&mut self, now_ms: u64) -> Vec<db_preview::PreviewId> {
        self.previews.sweep_expired(now_ms)
    }
    //#endregion 🔖️Preview
}
//#endregion 🔖️Engine
//#region 🔖️QuerySource
/// 🚰️ The `db_query::QuerySource` this crate supplies over its own `DocumentState`: one row
/// per stored path, `{"path": <path>, "value": <text-or-bytes>}`.
struct StateQuerySource<'a>(&'a db_state::RetainedStateMap);

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map (QuerySource::scan's row builder) — see R9
async fn path_row_value(path: &str, bytes: &db_io_core::DbIoPages, control: &mut db_query::QueryCursorControl) -> Result<db_query::Value, DbError> {
    let mut map = std::collections::BTreeMap::new();
    map.insert("path".to_string(), db_query::Value::Text(path.to_string()));
    map.insert("value".to_string(), db_query::Value::Bytes(db_query::QueryBytes::copy_from_pages(bytes, control).await?));
    Ok(db_query::Value::Map(map))
}

impl<'a> db_query::QuerySource for StateQuerySource<'a> {
    async fn scan(&self, control: &mut db_query::QueryCursorControl) -> Result<db_query::QueryRows, DbError> {
        let mut rows = db_query::QueryRows::new();
        for (index, entry) in self.0.iter().enumerate() {
            control.grant()?;
            rows.preflight_push()?;
            let row = db_query::QueryRow::new(db_query::RowId(index as u64), path_row_value(entry.key(), entry.value(), control).await?);
            rows.push_preflighted(row);
        }
        Ok(rows)
    }
}
//#endregion 🔖️QuerySource
//#region 🔖️Snapshot
const MAX_STATE_PAGE_ENTRIES: u64 = 10_000_000;

const MAX_STATE_PAGE_PATH_BYTES: u64 = db_io_core::DbIoText::maximum_capacity() as u64;

const MAX_STATE_PAGE_VALUE_BYTES: u64 = 256 * 1024 * 1024;

fn state_page_varint_len(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

fn state_page_varint(mut value: u64, output: &mut [u8; 10]) -> &[u8] {
    let mut len = 0;
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        output[len] = byte;
        len += 1;
        if value == 0 {
            return &output[..len];
        }
    }
}

struct StatePageGrant {
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    fuel: usize,
}

impl StatePageGrant {
    fn consume(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("state page cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("state page cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("state page cursor fuel"))?;
        Ok(())
    }
}

struct StatePageEncodeCursor<'state> {
    values: &'state db_state::RetainedStateMap,
    pub writer: db_io_core::DbIoPageWriter,
    pub grant: StatePageGrant,
}

impl<'state> StatePageEncodeCursor<'state> {
    pub fn try_new(values: &'state db_state::RetainedStateMap, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        check_len(values.len() as u64, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
        let mut len = state_page_varint_len(values.len() as u64);
        for entry in values.iter() {
            let path = entry.key();
            let value = entry.value();
            check_len(path.len() as u64, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
            check_len(value.len() as u64, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
            len = len.checked_add(state_page_varint_len(path.len() as u64) + path.len() + 1 + state_page_varint_len(value.len() as u64) + value.len()).ok_or(DbError::LimitExceeded("state page encoded bytes"))?;
        }
        let pages = len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        if pages > db_io_core::DB_IO_OPERATION_PAGES {
            return Err(DbError::LimitExceeded("state page reservation"));
        }
        let writer = db_io_core::DbIoPageWriter::try_reserve(pages).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        Ok(Self { values, writer, grant: StatePageGrant { cancelled, deadline, fuel } })
    }

    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), DbError> {
        let mut offset = 0;
        while offset < bytes.len() {
            self.grant.consume()?;
            offset += self.writer.write_fragment(&bytes[offset..])?;
            semio_framework_async::yield_once().await;
        }
        Ok(())
    }

    pub async fn finish(mut self) -> Result<db_io_core::DbIoPages, DbError> {
        let mut varint = [0u8; 10];
        self.write(state_page_varint(self.values.len() as u64, &mut varint)).await?;
        for entry in self.values.iter() {
            let path = entry.key();
            let value = entry.value();
            self.write(state_page_varint(path.len() as u64, &mut varint)).await?;
            self.write(path.as_bytes()).await?;
            self.write(&[1]).await?;
            self.write(state_page_varint(value.len() as u64, &mut varint)).await?;
            for fragment in value.fragments() {
                self.write(fragment).await?;
            }
        }
        self.writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }
}

struct StatePageDecodeCursor<'pages> {
    pub pages: &'pages db_io_core::DbIoPages,
    pub page: u8,
    pub offset: usize,
    pub remaining: Option<u64>,
    pub grant: StatePageGrant,
    pub closed: bool,
}

impl<'pages> StatePageDecodeCursor<'pages> {
    pub fn new(pages: &'pages db_io_core::DbIoPages, cancelled: Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if pages.operation() == 0 || fuel == 0 {
            return Err(DbError::InvalidArgument("state page decode authority".to_string()));
        }
        Ok(Self { pages, page: 0, offset: 0, remaining: None, grant: StatePageGrant { cancelled, deadline, fuel }, closed: false })
    }

    pub fn byte(&mut self) -> Result<u8, DbError> {
        self.grant.consume()?;
        loop {
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page ended early".to_string()))?;
            if let Some(byte) = fragment.get(self.offset) {
                self.offset += 1;
                return Ok(*byte);
            }
            self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
            self.offset = 0;
        }
    }

    fn varint(&mut self) -> Result<u64, DbError> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(DbError::Corrupt("state page varint overflow".to_string()))
    }

    fn fixed_field<const N: usize>(&mut self, len: usize) -> Result<[u8; N], DbError> {
        if len > N {
            return Err(DbError::LimitExceeded("state page fixed field"));
        }
        let mut output = [0u8; N];
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page field ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            output[written..written + copied].copy_from_slice(&fragment[self.offset..self.offset + copied]);
            written += copied;
            self.offset += copied;
        }
        Ok(output)
    }

    async fn retained_field(&mut self, len: usize) -> Result<db_io_core::DbIoPages, DbError> {
        let pages = len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(self.pages.operation(), pages).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        let mut written = 0;
        while written < len {
            self.grant.consume()?;
            let fragment = self.pages.page(self.page).ok_or_else(|| DbError::Corrupt("state page value ended early".to_string()))?;
            if self.offset == fragment.len() {
                self.page = self.page.checked_add(1).ok_or(DbError::LimitExceeded("state page fragment cursor"))?;
                self.offset = 0;
                continue;
            }
            let copied = (len - written).min(fragment.len() - self.offset);
            let accepted = writer.write_fragment(&fragment[self.offset..self.offset + copied])?;
            if accepted != copied {
                return Err(DbError::Internal("state page value writer made partial progress".to_string()));
            }
            self.offset += copied;
            written += copied;
            semio_framework_async::yield_once().await;
        }
        writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub async fn next(&mut self) -> Result<Option<db_state::StateEntry>, DbError> {
        let remaining = match self.remaining {
            Some(remaining) => remaining,
            None => {
                let count = self.varint()?;
                check_len(count, MAX_STATE_PAGE_ENTRIES, "db_artifact::snapshot_page_entries")?;
                self.remaining = Some(count);
                count
            }
        };
        if remaining == 0 {
            return Ok(None);
        }
        let path_len = self.varint()?;
        check_len(path_len, MAX_STATE_PAGE_PATH_BYTES, "db_artifact::snapshot_page_path")?;
        let path = self.fixed_field::<1024>(path_len as usize)?;
        let path = std::str::from_utf8(&path[..path_len as usize]).map_err(|_| DbError::Corrupt("snapshot page path is not valid utf-8".to_string()))?;
        let value = match self.byte()? {
            1 => {
                let len = self.varint()?;
                check_len(len, MAX_STATE_PAGE_VALUE_BYTES, "db_artifact::snapshot_page_value")?;
                self.retained_field(len as usize).await?
            }
            _ => return Err(DbError::Corrupt("state page value tag".to_string())),
        };
        let entry = db_state::StateEntry::try_new(path, value).map_err(|(error, mut value)| {
            let _ = value.close_step();
            drop(value);
            error
        })?;
        self.remaining = Some(remaining - 1);
        semio_framework_async::yield_once().await;
        Ok(Some(entry))
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        self.grant.consume()?;
        if self.closed {
            return Ok(false);
        }
        self.remaining = Some(0);
        self.closed = true;
        Ok(true)
    }
}

/// 📋️ What `ArtifactEngine::open` did to materialize state — "initial ⊕ snapshot ⊕ WAL
/// suffix" made observable. New this revision (was `db_wal::WalRecoveryReport` alone before).
#[derive(Clone, Debug, Default)]
pub struct MaterializeReport {
    pub from_snapshot: bool,
    pub snapshot_generation: Option<u64>,
    pub torn_tail_bytes: u64,
    pub commands_replayed: u64,
}
//#endregion 🔖️Snapshot
//#region 🔖️LedgerTail
/// 🧮️ Ceiling on the committed transactions one ledger tail may carry, equal to the hub
/// authority's own `AUTHORITY_MAX_OPERATIONS`.
pub const ARTIFACT_LEDGER_TAIL_MAX_COMMITS: usize = 16_384;

/// 📍️ One exact point of a document's committed ledger: the counters and content chain the
/// document authority publishes plus the edit id at its tip (`None` only for genesis).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactLedgerPoint {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub head_edit_id: Option<protocol::MutationId>,
}

impl ArtifactLedgerPoint {
    /// 🌱️ The point of a document nothing was ever committed to.
    pub const fn genesis() -> Self {
        Self { head_seq: 0, commit_seq: 0, chain_hash: [0; 32], head_edit_id: None }
    }

    /// 📸️ The point an actor-serialized snapshot names.
    pub fn of_snapshot(snapshot: &CheckpointPublicationSnapshot) -> Self {
        Self { head_seq: snapshot.frontier.head_seq, commit_seq: snapshot.frontier.commit_seq, chain_hash: snapshot.frontier.chain_hash, head_edit_id: snapshot.head_edit_id.clone() }
    }

    fn is_genesis(&self) -> bool {
        *self == Self::genesis()
    }
}

/// 📜️ One committed command transaction of the ledger tail: its envelopes in WAL order and the
/// point the document authority published for it.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactLedgerCommit {
    pub point: ArtifactLedgerPoint,
    pub envelopes: Vec<protocol::MutationEnvelope>,
}

/// 📜️ The committed command transactions strictly after `after` through exactly `through`, read
/// from the same WAL and with the same point derivation `ArtifactEngine::open` replays: a command
/// transaction's point is its terminal `Frontier` record plus its last command's id, and a durable
/// group decision advances the point by one head and one commit to its parent post revision. Both
/// points must be ledger points exactly, `after` must precede `through`, and a durable group
/// decision inside the range is refused: its content is an approval's, published with its own
/// checkpoint, and never an envelope a replica folds. `cancelled` stops the walk at the next record.
///
/// See `🌎️hub/🗿️artifact-authority/📌️check-in` for the hub Check In that consumes it.
pub async fn artifact_ledger_tail(wal: &impl WalStorage, document: &ArtifactId, after: &ArtifactLedgerPoint, through: &ArtifactLedgerPoint, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<Vec<ArtifactLedgerCommit>, DbError> {
    if through.head_seq <= after.head_seq || through.commit_seq <= after.commit_seq || through.head_edit_id.is_none() {
        return Err(DbError::InvalidArgument("ledger tail must end at an edited point after its start".to_string()));
    }
    let control = db_wal_log::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut records = db_wal_log::replay_committed_document(wal, document, control).await?;
    let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(cancelled.clone(), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut turn = db_wal_log::WalTurn::start();
    let result = async {
        let mut point = ArtifactLedgerPoint::genesis();
        let mut decisions: HashSet<String> = HashSet::new();
        let mut started = after.is_genesis();
        let mut commits = Vec::new();
        loop {
            if cancelled.load(std::sync::atomic::Ordering::Acquire) {
                return Err(DbError::Unavailable("ledger tail cancelled".to_string()));
            }
            records.renew_step()?;
            decode_control.renew_step()?;
            let mut transaction = match records.next_transaction_step().await? {
                db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                db_wal_log::WalCommittedStep::Yield => {
                    turn.step().await;
                    continue;
                }
                db_wal_log::WalCommittedStep::Done => return Err(DbError::NotFound("ledger tail end is not a committed ledger point".to_string())),
            };
            let mut envelopes = Vec::new();
            let mut frontier = None;
            let mut decision = None;
            loop {
                let record = match transaction.next_record_step()? {
                    db_wal_log::WalCommittedRecordStep::Record(record) => record,
                    db_wal_log::WalCommittedRecordStep::Yield => {
                        turn.step().await;
                        continue;
                    }
                    db_wal_log::WalCommittedRecordStep::Done => break,
                };
                match record {
                    db_wal_log::WalRecord::Command(bytes) => {
                        let envelope = db_sync::decode_wal_command(bytes, &mut decode_control)?;
                        if envelope.document_id.0 != document.0 {
                            return Err(DbError::Corrupt("ledger envelope document differs".to_string()));
                        }
                        envelopes.push(envelope);
                    }
                    db_wal_log::WalRecord::Frontier(committed) => {
                        if committed.document != *document {
                            return Err(DbError::Corrupt("ledger frontier document differs".to_string()));
                        }
                        frontier = Some(committed.clone());
                    }
                    db_wal_log::WalRecord::Event(bytes) => {
                        let mut canonical_pack = Vec::with_capacity(bytes.len());
                        for fragment in bytes.fragments() {
                            canonical_pack.extend_from_slice(fragment);
                        }
                        decision = Some(store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?);
                    }
                    _ => {}
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            let next = match (decision, frontier, envelopes.last()) {
                (Some(record), None, None) => {
                    if !decisions.insert(record.decision_sha256().to_string()) {
                        continue;
                    }
                    if started {
                        return Err(DbError::Conflict("ledger tail contains a durable group decision".to_string()));
                    }
                    ArtifactLedgerPoint {
                        head_seq: point.head_seq.checked_add(1).ok_or(DbError::LimitExceeded("ledger tail head sequence"))?,
                        commit_seq: point.commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("ledger tail commit sequence"))?,
                        chain_hash: record.parent_post_revision(),
                        head_edit_id: Some(protocol::MutationId(record.parent_edit_id().to_string())),
                    }
                }
                (None, Some(frontier), Some(last)) => ArtifactLedgerPoint { head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash, head_edit_id: Some(last.mutation_id.clone()) },
                (None, None, None) => continue,
                _ => return Err(DbError::Corrupt("ledger transaction is neither one command batch with its frontier nor one durable group decision".to_string())),
            };
            if next.head_seq <= point.head_seq || next.commit_seq <= point.commit_seq {
                return Err(DbError::Corrupt("ledger points do not advance".to_string()));
            }
            point = next;
            if !started {
                if point == *after {
                    started = true;
                } else if point.commit_seq >= after.commit_seq {
                    return Err(DbError::NotFound("ledger tail start is not a committed ledger point".to_string()));
                }
                continue;
            }
            if commits.len() == ARTIFACT_LEDGER_TAIL_MAX_COMMITS {
                return Err(DbError::LimitExceeded("ledger tail commits"));
            }
            let reached = point == *through;
            if !reached && point.commit_seq >= through.commit_seq {
                return Err(DbError::NotFound("ledger tail end is not a committed ledger point".to_string()));
            }
            commits.push(ArtifactLedgerCommit { point: point.clone(), envelopes });
            if reached {
                return Ok(commits);
            }
        }
    }
    .await;
    while records.close_owner_step()? {
        semio_framework_async::yield_once().await;
    }
    drop(records);
    result
}
//#endregion 🔖️LedgerTail
//#endregion 🔖️Ids

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
