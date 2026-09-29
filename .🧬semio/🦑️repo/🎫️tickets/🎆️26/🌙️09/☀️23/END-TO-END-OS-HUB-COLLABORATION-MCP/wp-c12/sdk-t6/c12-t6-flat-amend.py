"""📈️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` (coordinator 11:1x): a coalesced amend costs
amortized O(1) per key. Root cause measured by the burst law (900 s timeout): every amend's `bump()` reconciles the revision
accumulator, whose tail check re-encoded the WHOLE coalesced edit as canonical JSON — twice — to re-derive its digest, so a
typing run of n keys cost O(n²). Now an edit's revision identity stays a pure function of the edit — a single-operation edit
keeps its canonical-JSON digest (the one-item byte sealer streams exactly that), a coalesced edit that grew past one
operation hashes its header fields and one running chain per operation list — and the accumulator keeps the applied tail's
chains, so an amend extends them by the operations it appended. The burst law goes back to ONE uninterrupted 10 000-key run
and asserts a flat per-key cost (median of the last 1 000 keys ≤ 3 × the first 1 000); a store law pins purity (the amended
revision equals the from-scratch revision of the same envelope). Usage: python3 c12-t6-flat-amend.py"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATCH = os.path.join(HERE, "c12-sdk-composition-patch.py")
text = open(PATCH, encoding="utf-8").read()
if "DIGEST_OLD = " in text:
    print("already amended")
    raise SystemExit(0)


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:90], text.count(old))
    text = text.replace(old, new)


swap("""   Laws: SDK `child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting` (10 000 publications without a
   maintenance turn: none faults, each lands in order, occupancy within the fixed slots, pressure drains below the quarter
   bound); writer `a_ten_thousand_keystroke_burst_applies_every_key_in_order` (10 000 keys with corrections, no pressure
   drain between keys, projection equals the model after every key, pressure drains afterwards).
""", """   Laws: SDK `child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting` (10 000 publications without a
   maintenance turn: none faults, each lands in order, occupancy within the fixed slots, pressure drains below the quarter
   bound); writer `a_ten_thousand_keystroke_burst_applies_every_key_in_order` (ONE uninterrupted 10 000-key run with
   corrections, no pressure drain between keys, projection equals the model after every key, per-key cost flat — median of
   the last 1 000 keys ≤ 3 × the first 1 000 —, pressure drains afterwards).
   (c) Coordinator (11:1x): a coalesced amend is amortized O(1) per key. Every amend's `bump()` reconciled the revision
   accumulator, whose tail check re-encoded the WHOLE coalesced edit as canonical JSON (twice) to re-derive its digest — a
   typing run of n keys cost O(n²) (the burst law timed out at 900 s). An edit's revision identity stays a pure function of
   the edit: a single-operation edit keeps its canonical-JSON digest (what the one-item byte sealer streams), a coalesced
   edit that grew past one operation hashes its header fields and one running chain per operation list, and the
   accumulator keeps the applied tail's chains, so an amend extends them by exactly the operations it appended. Store law
   `an_amended_edit_revision_is_its_own_from_scratch_digest` pins purity (the amended revision equals the revision a store
   loaded from the same envelope derives).
""")

swap('''SDK_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
''', '''SDK_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
DURABLE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs"
STORE_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
''')

swap("""/// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): a 10 000-keystroke burst at the maximum rate — one full-text `text-edit` per
/// key with corrections and a paste (`set-text`) every 256 keys, no pressure drain between keys — applies every key in order
/// and never faults. Every key re-points the document slot at a new content-addressed child and so admits child-root and
/// child-member retirements faster than the maintenance rotation returns them; publication and the follow pass wait on
/// those retirements (the 65th key answered `interactive-job.child-root-retirement-saturated` before). The pastes end the
/// coalesced typing run the way a user's paste does, so each run stays one bounded undo step. Afterwards maintenance
/// returns the retirements below their pressure bound.""", """/// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): ONE uninterrupted 10 000-keystroke typing run at the maximum rate — one
/// full-text `text-edit` per key with corrections, no pressure drain between keys — applies every key in order, never faults,
/// and costs the same per key at its end as at its start. Every key re-points the document slot at a new content-addressed
/// child and so admits child-root and child-member retirements faster than the maintenance rotation returns them;
/// publication and the follow pass wait on those retirements (the 65th key answered
/// `interactive-job.child-root-retirement-saturated` before). Every key also amends the run's one coalesced edit, which
/// re-encoded the whole run per key before (the median key of the last 1 000 must stay within 3 × the first 1 000).
/// Afterwards maintenance returns the retirements below their pressure bound.""")
swap("""    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for index in 0..10_000usize {""", """    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut key_nanos = Vec::with_capacity(10_000);
    for index in 0..10_000usize {""")
swap("""        let key = if index % 256 == 255 { WriterCommand::SetText(set_text::SetText { text: model.clone() }) } else { WriterCommand::TextEdit(super::TextEdit { text: model.clone() }) };
        dispatch(&mut app, key).await;
        assert_eq!(writer_text(&app.snapshot().expect("projection")), model, "key {index} applies in order");
    }""", """        let started = std::time::Instant::now();
        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: model.clone() })).await;
        key_nanos.push(started.elapsed().as_nanos());
        assert_eq!(writer_text(&app.snapshot().expect("projection")), model, "key {index} applies in order");
    }
    let median = |window: &[u128]| {
        let mut sorted = window.to_vec();
        sorted.sort_unstable();
        sorted[sorted.len() / 2]
    };
    let (first, last) = (median(&key_nanos[..1_000]), median(&key_nanos[9_000..]));
    assert!(last <= first.saturating_mul(3), "per-key cost stays flat over one uninterrupted run: the median key of the last 1 000 took {last} ns against {first} ns for the first 1 000");""")

NEW_HUNKS = r'''
DIGEST_OLD = """#[derive(Clone)]
struct CursorRevisionAccumulator {
    identity_digest: [u8; 32],
    applied: Vec<CursorRevisionRecord>,
    redo: Vec<CursorRevisionRecord>,
}
"""
DIGEST_NEW = """#[derive(Clone)]
struct CursorRevisionAccumulator {
    identity_digest: [u8; 32],
    applied: Vec<CursorRevisionRecord>,
    redo: Vec<CursorRevisionRecord>,
    applied_tail_chains: Option<([u8; 32], EditDigestChains)>,
}

/// 🔗️ Running per-list digests of a coalesced edit's operations: what lets an amend extend its edit's revision identity by
/// the operations it appended instead of re-encoding the whole run (a typing run amends one edit per key).
#[derive(Clone, Copy, Default)]
struct EditDigestChains {
    forwards: usize,
    inverse: usize,
    meta: usize,
    forwards_digest: [u8; 32],
    inverse_digest: [u8; 32],
    meta_digest: [u8; 32],
}

impl EditDigestChains {
    /// 🔗️ The chains over every operation of `edit`, continued from these; `None` when `edit` is not an extension of the lists
    /// these were taken over (a list shrank).
    fn extended<Mutation: ToValue>(mut self, edit: &Edit<Mutation>) -> Option<Self> {
        if self.forwards > edit.forwards.len() || self.inverse > edit.inverse.len() || self.meta > edit.mutation_meta.len() {
            return None;
        }
        for operation in &edit.forwards[self.forwards..] {
            self.forwards_digest = CursorRevisionAccumulator::hash_record(b"edit-forward", &[&self.forwards_digest, crate::os_pack::json::to_json_string(operation).as_bytes()]);
        }
        for operation in &edit.inverse[self.inverse..] {
            self.inverse_digest = CursorRevisionAccumulator::hash_record(b"edit-inverse", &[&self.inverse_digest, crate::os_pack::json::to_json_string(operation).as_bytes()]);
        }
        for meta in &edit.mutation_meta[self.meta..] {
            self.meta_digest = CursorRevisionAccumulator::hash_record(b"edit-meta", &[&self.meta_digest, crate::os_pack::json::to_json_string(meta).as_bytes()]);
        }
        self.forwards = edit.forwards.len();
        self.inverse = edit.inverse.len();
        self.meta = edit.mutation_meta.len();
        Some(self)
    }
}
"""
NEW_OLD = "        Self { identity_digest, applied: Vec::with_capacity(capacity), redo: Vec::with_capacity(capacity) }\n"
NEW_NEW = "        Self { identity_digest, applied: Vec::with_capacity(capacity), redo: Vec::with_capacity(capacity), applied_tail_chains: None }\n"
EDIT_DIGEST_OLD = """    fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
        let encoded = crate::os_pack::json::to_json_string(edit).into_bytes();
        Self::hash_record(b"edit", &[edit.id.as_bytes(), &encoded])
    }

    fn reconcile_stack<Mutation: ToValue>(records: &mut Vec<CursorRevisionRecord>, ids: &[String], edits: &ArtifactHistoryLedger<Edit<Mutation>>, domain: &[u8], identity_digest: [u8; 32]) -> Vec<String> {
        let mut common = 0;
        while common < records.len().min(ids.len()) && records[common].id_digest == Self::hash_record(b"edit-id", &[ids[common].as_bytes()]) {
            common += 1;
        }
        while records.len() > common {
            records.pop().expect("revision suffix record remains present");
        }
        if common == ids.len() && common != 0 {
            let id = &ids[common - 1];
            let edit = edits.iter().find(|edit| edit.id == *id).expect("validated cursor edit exists");
            let edit_digest = Self::edit_digest(edit);
            if records[common - 1].edit_digest != edit_digest {
                records.pop().expect("validated revision record remains present");
                common -= 1;
            }
        }
        for id in &ids[common..] {
            let edit = edits.iter().find(|edit| edit.id == *id).expect("validated cursor edit exists");
            let edit_digest = Self::edit_digest(edit);
            let previous = records.last().map_or(identity_digest, |record| record.prefix_digest);
            let prefix_digest = Self::hash_record(domain, &[&previous, &edit_digest]);
            records.push(CursorRevisionRecord { id_digest: Self::hash_record(b"edit-id", &[id.as_bytes()]), edit_digest, prefix_digest });
        }
        Vec::new()
    }

    fn reconcile<Mutation: ToValue>(&mut self, applied_ids: &[String], redo_ids: &[String], edits: &ArtifactHistoryLedger<Edit<Mutation>>) -> (Vec<String>, Vec<String>) {
        let applied = Self::reconcile_stack(&mut self.applied, applied_ids, edits, b"applied", self.identity_digest);
        let redo = Self::reconcile_stack(&mut self.redo, redo_ids, edits, b"redo", self.identity_digest);
        (applied, redo)"""
EDIT_DIGEST_NEW = """    /// 🔏️ Revision identity of one edit — a pure function of the edit. A single-operation edit hashes its canonical JSON
    /// (exactly what the one-item byte sealer streams); a coalesced edit that grew past one operation hashes its header
    /// fields and one running chain per operation list ([`EditDigestChains`]), which an amend extends in O(appended).
    fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
        Self::edit_digest_extending(edit, None).0
    }

    /// 🔏️ [`Self::edit_digest`], continuing `known` chains when the edit only grew since they were taken (an amend).
    fn edit_digest_extending<Mutation: ToValue>(edit: &Edit<Mutation>, known: Option<EditDigestChains>) -> ([u8; 32], Option<EditDigestChains>) {
        if edit.forwards.len() <= 1 {
            let encoded = crate::os_pack::json::to_json_string(edit).into_bytes();
            return (Self::hash_record(b"edit", &[edit.id.as_bytes(), &encoded]), None);
        }
        let chains = known.and_then(|chains| chains.extended(edit)).unwrap_or_else(|| EditDigestChains::default().extended(edit).expect("empty chains extend every edit"));
        let tag = |value: Option<&String>| [u8::from(value.is_some())];
        let digest = Self::hash_record(
            b"edit-chained",
            &[
                edit.id.as_bytes(),
                &tag(edit.actor.as_ref()),
                edit.actor.as_deref().unwrap_or_default().as_bytes(),
                &tag(edit.description.as_ref()),
                edit.description.as_deref().unwrap_or_default().as_bytes(),
                &tag(edit.coalesce_key.as_ref()),
                edit.coalesce_key.as_deref().unwrap_or_default().as_bytes(),
                &edit.sequence_number.to_be_bytes(),
                edit.started_at.as_bytes(),
                &tag(edit.finished_at.as_ref()),
                edit.finished_at.as_deref().unwrap_or_default().as_bytes(),
                &(chains.forwards as u64).to_be_bytes(),
                &chains.forwards_digest,
                &(chains.inverse as u64).to_be_bytes(),
                &chains.inverse_digest,
                &(chains.meta as u64).to_be_bytes(),
                &chains.meta_digest,
            ],
        );
        (digest, Some(chains))
    }

    /// 🧮️ Re-derives one cursor stack's records for `ids`, keeping the common prefix. `tail_chains` carries the running
    /// chains of the stack's tail edit between calls, so the amend case (same ids, tail grew) costs O(appended operations).
    fn reconcile_stack<Mutation: ToValue>(
        records: &mut Vec<CursorRevisionRecord>,
        ids: &[String],
        edits: &ArtifactHistoryLedger<Edit<Mutation>>,
        domain: &[u8],
        identity_digest: [u8; 32],
        tail_chains: &mut Option<([u8; 32], EditDigestChains)>,
    ) -> Vec<String> {
        let mut common = 0;
        while common < records.len().min(ids.len()) && records[common].id_digest == Self::hash_record(b"edit-id", &[ids[common].as_bytes()]) {
            common += 1;
        }
        while records.len() > common {
            records.pop().expect("revision suffix record remains present");
        }
        let mut regrown = None;
        if common == ids.len() && common != 0 {
            let id = &ids[common - 1];
            let edit = edits.iter().find(|edit| edit.id == *id).expect("validated cursor edit exists");
            let known = tail_chains.filter(|(id_digest, _)| *id_digest == records[common - 1].id_digest).map(|(_, chains)| chains);
            let (edit_digest, chains) = Self::edit_digest_extending(edit, known);
            *tail_chains = chains.map(|chains| (records[common - 1].id_digest, chains));
            if records[common - 1].edit_digest != edit_digest {
                records.pop().expect("validated revision record remains present");
                common -= 1;
                regrown = Some(edit_digest);
            }
        }
        for id in &ids[common..] {
            let id_digest = Self::hash_record(b"edit-id", &[id.as_bytes()]);
            let edit_digest = match regrown.take() {
                Some(edit_digest) => edit_digest,
                None => {
                    let edit = edits.iter().find(|edit| edit.id == *id).expect("validated cursor edit exists");
                    let (edit_digest, chains) = Self::edit_digest_extending(edit, None);
                    *tail_chains = chains.map(|chains| (id_digest, chains));
                    edit_digest
                }
            };
            let previous = records.last().map_or(identity_digest, |record| record.prefix_digest);
            let prefix_digest = Self::hash_record(domain, &[&previous, &edit_digest]);
            records.push(CursorRevisionRecord { id_digest, edit_digest, prefix_digest });
        }
        Vec::new()
    }

    fn reconcile<Mutation: ToValue>(&mut self, applied_ids: &[String], redo_ids: &[String], edits: &ArtifactHistoryLedger<Edit<Mutation>>) -> (Vec<String>, Vec<String>) {
        let applied = Self::reconcile_stack(&mut self.applied, applied_ids, edits, b"applied", self.identity_digest, &mut self.applied_tail_chains);
        let redo = Self::reconcile_stack(&mut self.redo, redo_ids, edits, b"redo", self.identity_digest, &mut None);
        (applied, redo)"""
INIT_ACC_OLD = "            revision: std::mem::ManuallyDrop::new(CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision }),\n"
INIT_ACC_NEW = "            revision: std::mem::ManuallyDrop::new(CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None }),\n"
BUILD_ACC_OLD = "        let mut revision_accumulator = CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision };\n"
BUILD_ACC_NEW = "        let mut revision_accumulator = CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None };\n"
RETIRED_ACC_OLD = """                        identity_digest: self.revision_accumulator.identity_digest,
                        applied: Vec::new(),
                        redo: retired_redo,
                    })));"""
RETIRED_ACC_NEW = """                        identity_digest: self.revision_accumulator.identity_digest,
                        applied: Vec::new(),
                        redo: retired_redo,
                        applied_tail_chains: None,
                    })));"""
DURABLE_ACC_OLD = """        redo: match retained_revision_stack(&[]) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
    };"""
DURABLE_ACC_NEW = """        redo: match retained_revision_stack(&[]) {
            Ok(values) => values,
            Err(error) => return reject(error, outcome),
        },
        applied_tail_chains: None,
    };"""
STORE_LAW_ANCHOR = """    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot after undo").n, Some(0), "one undo reverts the whole 50-step coalesced gesture");
}
"""
STORE_LAW = """
/// 🔏️ LAW (ticket 26/09/23 C12): a coalesced edit's revision identity is a pure function of the edit. Each amend extends the
/// tail's digest chains by only the operations it appended, yet after a long run the live revision equals the revision a
/// store loaded from the same envelope derives from scratch, and every amend still moves the revision.
#[semio_framework_async_macros::async_test]
async fn an_amended_edit_revision_is_its_own_from_scratch_digest() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None);
    let mut store = ArtifactStore::new(envelope).await;
    let mut revisions = std::collections::HashSet::new();
    for n in 1..=256 {
        store.dispatch(ArtifactCommand::AmendLast { mutations: vec![DemoMutation::SetN(SetN { n })], coalesce_key: Some("typing".into()) }).await.expect("amend");
        assert!(revisions.insert(store.content_revision_now()), "amend {n} moves the revision");
    }
    assert_eq!(store.envelope().vcs.edits.len(), 1, "the run is one coalesced edit");
    let reloaded = ArtifactStore::new(store.envelope().clone()).await;
    assert_eq!(reloaded.content_revision_now(), store.content_revision_now(), "the incrementally extended revision equals the from-scratch one");
}
"""
'''
swap('\nDAG_OLD = ', NEW_HUNKS + '\nDAG_OLD = ')
swap('''    SDK_LAWS: [(SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW)],''', '''    SDK_LAWS: [(SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW)],
    STORE: [(DIGEST_OLD, DIGEST_NEW), (NEW_OLD, NEW_NEW), (EDIT_DIGEST_OLD, EDIT_DIGEST_NEW), (INIT_ACC_OLD, INIT_ACC_NEW), (BUILD_ACC_OLD, BUILD_ACC_NEW), (RETIRED_ACC_OLD, RETIRED_ACC_NEW)],
    DURABLE: [(DURABLE_ACC_OLD, DURABLE_ACC_NEW)],
    STORE_LAWS: [(STORE_LAW_ANCHOR, STORE_LAW_ANCHOR + STORE_LAW)],''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
