//! 🔀️ Resumable persisted-pair merge: the cursor behind the bounded per-turn document-archive merge. See the design note beside the patch (`1105b-tools/store/merge/design.md`).
//!
//! 🪭️ [`ArtifactStore::merge_persisted_history`] does the whole merge in one call. This module does the same merge as a state machine whose
//! [`ArtifactStore::persisted_merge_step`] performs priced units of work until its budget is spent and reports the work it really did, so a host
//! turn can interleave it with everything else, cancel it between steps and retire what it holds. The cursor owns only the decoded
//! `HistoryLog` and id censuses; every event is ingested through the unchanged atomic [`ArtifactStore::ingest_remote`].
use super::*;
use semio_framework_value::RetainedWorkOutcome;
use semio_os_spr::ColdMutation;

/// 💱️ The authored price list (`persisted-merge-pricing.json`); a law keeps this constant equal to that file.
pub const PERSISTED_MERGE_PRICING: PersistedMergePricing = PersistedMergePricing { decode_base: 2, decode_per_operation: 1, census_per_id: 1, order_per_comparison: 1, ingest_base: 8, ingest_per_dependency: 1, preflight_decode: true };

/// 💱️ Prices of one unit of each kind of merge work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PersistedMergePricing {
    pub decode_base: u64,
    pub decode_per_operation: u64,
    pub census_per_id: u64,
    pub order_per_comparison: u64,
    pub ingest_base: u64,
    pub ingest_per_dependency: u64,
    pub preflight_decode: bool,
}

/// 🏷️ The phase a merge cursor stands in; a merge visits them in this order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PersistedMergePhase {
    #[default]
    Preflight,
    PreflightTransitions,
    Census,
    CensusTransitions,
    Order,
    IngestEdits,
    IngestTransitions,
    Complete,
}

impl PersistedMergePhase {
    const ALL: [Self; 8] = [Self::Preflight, Self::PreflightTransitions, Self::Census, Self::CensusTransitions, Self::Order, Self::IngestEdits, Self::IngestTransitions, Self::Complete];
    fn code(self) -> u8 { Self::ALL.iter().position(|phase| *phase == self).map_or(0, |index| index as u8) }
    fn from_code(code: u8) -> Self { Self::ALL.get(usize::from(code)).copied().unwrap_or(Self::Complete) }
}

/// 📊️ Local-only progress of the current phase and the running totals (`persisted-merge-progress.schema.json`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PersistedMergeProgress {
    pub phase: PersistedMergePhase,
    pub completed: u64,
    pub total: u64,
    pub merged: u64,
    pub ahead: u64,
}

/// 🧵️ The resumable merge of one persisted pair into one live store. It holds the pair's decoded log until it is retired.
#[derive(semio_framework_value::RetireOwned)]
pub struct PersistedMergeCursor {
    history: semio_os_spr::HistoryLog,
    document: String,
    schema: String,
    phase: u8,
    index: usize,
    offset: usize,
    stage: u8,
    census_total: usize,
    offered: HashSet<MutationId>,
    known: HashSet<MutationId>,
    keys: Vec<u64>,
    order: Vec<u32>,
    merged: usize,
    ahead: usize,
}

impl PersistedMergeCursor {
    fn new(history: semio_os_spr::HistoryLog) -> Self {
        let (document, schema) = (history.doc_id.clone(), history.schema.clone());
        Self { history, document, schema, phase: 0, index: 0, offset: 0, stage: 0, census_total: 0, offered: HashSet::new(), known: HashSet::new(), keys: Vec::new(), order: Vec::new(), merged: 0, ahead: 0 }
    }

    pub fn phase(&self) -> PersistedMergePhase { PersistedMergePhase::from_code(self.phase) }

    pub fn is_complete(&self) -> bool { self.phase() == PersistedMergePhase::Complete }

    /// 🫱️ What the merge took and what the store held beyond the pair; final once [`Self::is_complete`].
    pub fn outcome(&self) -> PairMerge { PairMerge { merged: self.merged, ahead: self.ahead } }

    pub fn progress(&self) -> PersistedMergeProgress {
        let (edits, transitions) = (self.history.edits.len(), self.history.transitions.len());
        let (completed, total) = match self.phase() {
            PersistedMergePhase::Preflight | PersistedMergePhase::IngestEdits => (self.index, edits),
            PersistedMergePhase::PreflightTransitions | PersistedMergePhase::IngestTransitions => (self.index, transitions),
            PersistedMergePhase::Census | PersistedMergePhase::CensusTransitions => (self.index, self.census_total),
            PersistedMergePhase::Order => (self.index, transitions),
            PersistedMergePhase::Complete => (1, 1),
        };
        PersistedMergeProgress { phase: self.phase(), completed: completed as u64, total: total as u64, merged: self.merged as u64, ahead: self.ahead as u64 }
    }

    fn enter(&mut self, phase: PersistedMergePhase) {
        self.phase = phase.code();
        self.index = 0;
        self.offset = 0;
        self.stage = 0;
        self.census_total = 0;
    }

    fn transition_less(&self, left: u32, right: u32) -> bool {
        let key = |at: u32| { let base = at as usize * 3; (self.keys[base], self.keys[base + 1], self.keys[base + 2]) };
        let id = |at: u32| self.history.transitions.get(at as usize).map(|record| record.id.as_str());
        (key(left), id(left)) < (key(right), id(right))
    }

    fn sift_down(&mut self, mut root: usize, end: usize) -> u64 {
        let mut compares = 0;
        loop {
            let left = 2 * root + 1;
            if left >= end { break compares }
            let mut child = left;
            if left + 1 < end { compares += 1; if self.transition_less(self.order[left], self.order[left + 1]) { child = left + 1 } }
            compares += 1;
            if self.transition_less(self.order[root], self.order[child]) { self.order.swap(root, child); root = child } else { break compares }
        }
    }
}

fn affordable(work: u64, budget: u64, price: u64) -> bool { work == 0 || work.saturating_add(price) <= budget }

async fn edit_envelopes<P, Mutation>(history: &semio_os_spr::HistoryLog, index: usize, document: &str, schema: &str) -> Result<Vec<semio_os_spr::MutationEnvelope>, VcsError>
where
    P: Clone + 'static,
    Mutation: OpText + OpBinary + self::Mutation<P> + ColdMutation<P> + 'static,
{
    let record = history.edits.get(index).ok_or_else(|| VcsError::Deserialize("the persisted merge lost its edit position".to_string()))?;
    let edit = semio_os_store_text_io::text_format::decode_history_edit_record::<P, Mutation>(index, record, schema).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let envelopes = semio_os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(&edit, 0, &ArtifactId(document.to_string()), &SchemaId(schema.to_string())).map_err(|error| VcsError::Serialize(error.to_string()));
    semio_os_store_ledger::retire_scratch_edits::<P, Mutation>([edit]);
    envelopes
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState + Clone,
    Mutation: semio_os_spr::StoreMutationFor<P> + Clone + ColdMutation<P>,
{
    /// 🚪️ Opens a merge of the pair's decoded `history` (its genesis `pack` names the document): the same identity checks as
    /// [`Self::merge_persisted_history`], nothing parsed. A refusal hands the history back so its caller retires it.
    pub fn persisted_merge_open(&self, pack: &[u8], history: semio_os_spr::HistoryLog) -> Result<PersistedMergeCursor, (VcsError, semio_os_spr::HistoryLog)> {
        if let Err(error) = self.ensure_durable_group_idle() { return Err((error, history)) }
        if history.doc_id != self.log.envelope.id || history.schema != self.log.envelope.schema || artifact_initial_digest_of_pack(pack) != self.log.initial_digest {
            let detail = format!("the persisted pair holds document {}, not {}: a merge takes this document's own log only", history.doc_id, self.log.envelope.id);
            return Err((VcsError::ValidationFailed(detail), history));
        }
        Ok(PersistedMergeCursor::new(history))
    }

    /// 🪜️ Performs priced units of the merge until `budget` fuel is spent (the first unit of a step always runs, so every step
    /// progresses) and answers where the cursor stands with the work it really did. A refusal leaves the cursor at the refused
    /// unit; what earlier units took stays, as with any remote batch. Between steps the store may change: every event is
    /// ingested idempotently and the document identity is checked at each step.
    pub async fn persisted_merge_step(&mut self, cursor: &mut PersistedMergeCursor, budget: u64) -> RetainedWorkOutcome<PersistedMergeProgress, VcsError> {
        let mut work = 0;
        let result = self.persisted_merge_run(cursor, budget, &mut work).await;
        RetainedWorkOutcome { result, work }
    }

    async fn persisted_merge_run(&mut self, cursor: &mut PersistedMergeCursor, budget: u64, work: &mut u64) -> Result<PersistedMergeProgress, VcsError> {
        if cursor.document != self.log.envelope.id || cursor.schema != self.log.envelope.schema {
            return Err(VcsError::ValidationFailed("the store under a persisted merge is no longer the merged document".to_string()));
        }
        self.ensure_durable_group_idle()?;
        let price = PERSISTED_MERGE_PRICING;
        let (edits, transitions) = (cursor.history.edits.len(), cursor.history.transitions.len());
        loop {
            match cursor.phase() {
                PersistedMergePhase::Preflight => {
                    if !price.preflight_decode || cursor.index >= edits { cursor.enter(PersistedMergePhase::PreflightTransitions); continue }
                    let Some(record) = cursor.history.edits.get(cursor.index) else { cursor.enter(PersistedMergePhase::PreflightTransitions); continue };
                    let unit = price.decode_base + price.decode_per_operation * (record.ops.len() + record.inverse.len()) as u64;
                    if !affordable(*work, budget, unit) { break }
                    let envelopes = edit_envelopes::<P, Mutation>(&cursor.history, cursor.index, &cursor.document, &cursor.schema).await?;
                    cursor.offered.extend(envelopes.iter().map(|envelope| envelope.mutation_id.clone()));
                    *work += unit;
                    cursor.index += 1;
                }
                PersistedMergePhase::PreflightTransitions => {
                    if cursor.index >= transitions { cursor.enter(PersistedMergePhase::Census); continue }
                    if !affordable(*work, budget, price.census_per_id) { break }
                    let id = cursor.history.transitions.get(cursor.index).map(|record| record.to_envelope(&cursor.document).mutation_id);
                    cursor.offered.extend(id);
                    *work += price.census_per_id;
                    cursor.index += 1;
                }
                PersistedMergePhase::Census => {
                    if cursor.census_total == 0 && cursor.stage == 0 { cursor.census_total = self.event_log_edit_count(); cursor.stage = 1; *work += cursor.census_total as u64 / 64 + 1 }
                    if cursor.index >= cursor.census_total { cursor.enter(PersistedMergePhase::CensusTransitions); continue }
                    let remaining = budget.saturating_sub(*work);
                    let page = usize::try_from((remaining / (price.census_per_id * 4).max(1)).max(1)).unwrap_or(usize::MAX);
                    let ids = self.event_log_edit_ids_page(cursor.index, page);
                    *work += (ids.len() as u64).max(1) * price.census_per_id;
                    cursor.ahead += ids.iter().filter(|id| !cursor.offered.contains(*id)).count();
                    cursor.known.extend(ids);
                    cursor.index = cursor.index.saturating_add(page);
                    if !affordable(*work, budget, price.census_per_id) { break }
                }
                PersistedMergePhase::CensusTransitions => {
                    if cursor.census_total == 0 && cursor.stage == 0 { cursor.census_total = self.event_log_transition_count(); cursor.stage = 1 }
                    if cursor.index >= cursor.census_total { cursor.enter(PersistedMergePhase::Order); continue }
                    let remaining = budget.saturating_sub(*work);
                    let page = usize::try_from((remaining / price.census_per_id.max(1)).max(1)).unwrap_or(usize::MAX);
                    let ids = self.event_log_transition_ids_page(cursor.index, page);
                    *work += (ids.len() as u64).max(1) * price.census_per_id;
                    cursor.ahead += ids.iter().filter(|id| !cursor.offered.contains(*id)).count();
                    cursor.known.extend(ids);
                    cursor.index = cursor.index.saturating_add(page);
                    if !affordable(*work, budget, price.census_per_id) { break }
                }
                PersistedMergePhase::Order => {
                    if cursor.stage == 0 {
                        while cursor.keys.len() / 3 < transitions {
                            if !affordable(*work, budget, price.census_per_id) { return Ok(cursor.progress()) }
                            let at = cursor.keys.len() / 3;
                            let key = cursor.history.transitions.get(at).map(|record| record.to_envelope(&cursor.document).timestamp.cmp_key()).unwrap_or((0, 0, 0));
                            cursor.keys.extend([key.0, key.1, key.2]);
                            cursor.order.push(at as u32);
                            *work += price.census_per_id;
                        }
                        cursor.stage = 1;
                        cursor.index = transitions / 2;
                    }
                    let log = (usize::BITS - transitions.max(1).leading_zeros()) as u64 + 1;
                    let unit = 2 * log * price.order_per_comparison;
                    if cursor.stage == 1 {
                        if cursor.index == 0 { cursor.stage = 2; cursor.index = transitions.saturating_sub(1); continue }
                        if !affordable(*work, budget, unit) { break }
                        let compares = cursor.sift_down(cursor.index - 1, transitions);
                        *work += compares.max(1) * price.order_per_comparison;
                        cursor.index -= 1;
                    } else {
                        if cursor.index == 0 || transitions < 2 { cursor.enter(PersistedMergePhase::IngestEdits); continue }
                        if !affordable(*work, budget, unit) { break }
                        cursor.order.swap(0, cursor.index);
                        let compares = cursor.sift_down(0, cursor.index);
                        *work += compares.max(1) * price.order_per_comparison;
                        cursor.index -= 1;
                    }
                }
                PersistedMergePhase::IngestEdits => {
                    if cursor.index >= edits { cursor.enter(PersistedMergePhase::IngestTransitions); continue }
                    let record_price = cursor.history.edits.get(cursor.index).map_or(1, |record| price.decode_base + price.decode_per_operation * (record.ops.len() + record.inverse.len()) as u64);
                    if !affordable(*work, budget, record_price) { break }
                    let envelopes = edit_envelopes::<P, Mutation>(&cursor.history, cursor.index, &cursor.document, &cursor.schema).await?;
                    *work += record_price;
                    let mut stopped = false;
                    for (position, event) in envelopes.into_iter().enumerate().skip(cursor.offset) {
                        let unit = if cursor.known.contains(&event.mutation_id) { price.census_per_id } else { price.ingest_base + price.ingest_per_dependency * event.dependencies.len() as u64 };
                        if !affordable(*work, budget, unit) { cursor.offset = position; stopped = true; break }
                        if !cursor.known.contains(&event.mutation_id) && self.ingest_remote(event).await?.accepted { cursor.merged += 1 }
                        *work += unit;
                    }
                    if stopped { break }
                    cursor.index += 1;
                    cursor.offset = 0;
                }
                PersistedMergePhase::IngestTransitions => {
                    if cursor.index >= transitions { cursor.enter(PersistedMergePhase::Complete); continue }
                    let at = cursor.order.get(cursor.index).copied().map_or(cursor.index, |at| at as usize);
                    let Some(event) = cursor.history.transitions.get(at).map(|record| record.to_envelope(&cursor.document)) else { return Err(VcsError::Deserialize("the persisted merge lost its transition position".to_string())) };
                    let unit = if cursor.known.contains(&event.mutation_id) { price.census_per_id } else { price.ingest_base + price.ingest_per_dependency * event.dependencies.len() as u64 };
                    if !affordable(*work, budget, unit) { break }
                    if !cursor.known.contains(&event.mutation_id) && self.ingest_remote(event).await?.accepted { cursor.merged += 1 }
                    *work += unit;
                    cursor.index += 1;
                }
                PersistedMergePhase::Complete => break,
            }
        }
        Ok(cursor.progress())
    }
}
