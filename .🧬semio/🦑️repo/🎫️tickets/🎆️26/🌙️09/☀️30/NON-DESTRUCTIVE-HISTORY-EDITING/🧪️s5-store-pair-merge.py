#!/usr/bin/env python3
"""🫱️ S5-STORE wave PM (design §22.22, live fault F4 of probe batch H): a folder read-back of THIS document merges its log into
the live store instead of replacing the document.

- `ArtifactStore::merge_persisted_history(pack, history) -> PairMerge { merged, ahead }` (the decoded-history entry S5-LOAD's
  stepped archive machine calls) and `merge_persisted_pair(pack, spr)` = decode + that. Every event the store lacks is
  ingested like a remote event; the reader's head, its own edits and an open replay stay; the pair's viewer head is never
  read. A pair of another document is refused before anything is parsed.
- `SpaceMember::merge_persisted_envelope(envelope_pack)`: the member half. The default refuses (a member without an event
  log has nothing to merge); `ArtifactStore` merges its pair. The `space_members!` arm is S5-NESTED's.
- `AppliedMutation.unit` (S5-RUNTIME's ask): the cross-artifact unit on every applied row, computed once per edit
  (`applied_edit_units`) instead of one located prefix fold per operation; `unit_id` / `unit_operations` read it.

Every edit is keyed on anchors that must occur exactly once; the file is written only when all resolved. Idempotent.
`--check` writes nothing; `--emit <dir>` writes the edited file into `<dir>`.

    python3 🧪️s5-store-pair-merge.py [--check | --emit <dir>]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"

MERGE_ANCHOR = "//#endregion 🔖️ArtifactStore\n\n//#region 🔖️EffectiveForwards\n"
MERGE_REGION = '''//#endregion 🔖️ArtifactStore

//#region 🔖️PairMerge
/// 🫱️ What merging a persisted pair into a live store found ([`ArtifactStore::merge_persisted_history`]): `merged` events the
/// store took from the pair, and `ahead` events the store holds that the pair lacks — what a writer persists again for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PairMerge {
    pub merged: usize,
    pub ahead: usize,
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🫸️ [`Self::merge_persisted_history`] over the pair's undecoded `.spr` bytes.
    pub async fn merge_persisted_pair(&mut self, pack: &[u8], spr: &[u8]) -> Result<PairMerge, VcsError> {
        let history = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.merge_persisted_history(pack, history).await
    }

    /// 🪭️ Merges the event log of a persisted pair — the genesis `pack` and its decoded `history`, another writer's copy of
    /// THIS document — into this live store. Every operation and history transition the store lacks is ingested exactly like
    /// a remote event ([`Self::ingest_remote`]): this replica keeps its head, its own edits and whatever replay is in flight,
    /// and the pair's viewer head is never read — only a cold load ([`parse_document_pack`]) stands where a pair says. A pair
    /// of another document (id, schema or genesis) is refused before anything is parsed: replacing a document is a load, not
    /// a merge. An ingest refusal stops the merge; what was taken before it stays, as with any remote batch (every ingest is
    /// atomic).
    pub async fn merge_persisted_history(&mut self, pack: &[u8], history: crate::os_spr::HistoryLog) -> Result<PairMerge, VcsError> {
        self.ensure_durable_group_idle()?;
        if history.doc_id != self.envelope.id || history.schema != self.envelope.schema || artifact_initial_digest_of_pack(pack) != self.initial_digest {
            return Err(VcsError::ValidationFailed(format!("the persisted pair holds document {}, not {}: a merge takes this document's own log only", history.doc_id, self.envelope.id)));
        }
        let ParsedDocumentText { envelope: foreign, snapshot } = parse_decoded_document_spr::<P, Mutation>(pack, history).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        retire_replayed_projection::<P, Mutation>(snapshot);
        let mut failure = None;
        let mut events = Vec::new();
        for edit in foreign.vcs.edits.iter() {
            match crate::os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, 0, &ArtifactId(foreign.id.clone()), &SchemaId(foreign.schema.clone())) {
                Ok(envelopes) => events.extend(envelopes),
                Err(error) => {
                    failure = Some(VcsError::Serialize(error.to_string()));
                    break;
                }
            }
        }
        events.extend(foreign.transitions.iter().cloned());
        foreign.retire_unadopted();
        if let Some(error) = failure {
            return Err(error);
        }
        let known: HashSet<MutationId> = self.event_log()?.into_iter().map(|event| event.mutation_id).collect();
        let offered: HashSet<MutationId> = events.iter().map(|event| event.mutation_id.clone()).collect();
        let mut merge = PairMerge { merged: 0, ahead: known.iter().filter(|id| !offered.contains(*id)).count() };
        for event in events.into_iter().filter(|event| !known.contains(&event.mutation_id)) {
            if self.ingest_remote(event).await?.accepted {
                merge.merged += 1;
            }
        }
        Ok(merge)
    }
}
//#endregion 🔖️PairMerge

//#region 🔖️EffectiveForwards
'''

UNIT_START = "    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit. An operation that\n"
UNIT_END = "    /// 🕰️ The projection right before operation `mutation_id` with `drafts` laid over the effective inputs — nothing\n"
UNIT_BLOCK = '''    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit
    /// ([`Self::applied_edit_units`]). A unit is superseded in all of its documents at once, which no replica can do yet
    /// ([`VcsError::UnitSpansDocuments`]).
    pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {
        let (position, index) = self.locate_mutation(mutation_id)?;
        Ok(self.applied_edit_units(position)?.swap_remove(index))
    }

    /// 🪬️ This store's part of the unit of `mutation_id`, in operation order: every operation its edit recorded under the
    /// unit's group, or the operation alone when it is its own unit ([`Self::unit_id`]).
    pub fn unit_operations(&self, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> {
        let (position, index) = self.locate_mutation(mutation_id)?;
        let edit = self.applied_edit(position)?;
        let group = edit.mutation_meta.get(index).and_then(|meta| meta.group_id.as_ref());
        if group.is_none() || self.applied_edit_units(position)?[index].is_none() {
            return Ok(vec![mutation_id.clone()]);
        }
        Ok(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().enumerate().filter(|(member, _)| edit.mutation_meta.get(*member).and_then(|meta| meta.group_id.as_ref()) == group).map(|(_, member)| member).collect())
    }

    /// 🫴️ The cross-artifact unit of every operation of the applied edit at `position`, in operation order; `None` for an
    /// operation that is its own unit. An operation that plans foreign steps and whose plan on the projection right before
    /// it is not empty changed other documents too, and an operation of `Transaction` origin is such a step of another
    /// document's operation. The unit is named by the gesture's group where this replica holds one, else by the operation.
    /// The edit is folded once from the nearest prefix projection, and only when one of its operations may plan foreign
    /// steps; nothing is retained.
    fn applied_edit_units(&self, position: usize) -> Result<Vec<Option<String>>, VcsError> {
        let edit = self.applied_edit(position)?;
        let ids = crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit);
        let mut plans = vec![false; edit.forwards.len()];
        if edit.forwards.iter().any(|operation| operation.may_emit_foreign_steps()) {
            let applied: Vec<String> = self.applied_edit_ids.to_vec();
            let (base, recorded) = self.prefix_state_recorded(&applied, position, self.supersessions(), Self::prefix_stride(applied.len()))?;
            for (_, snapshot) in recorded {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            let mut running = base;
            for (index, operation) in edit.forwards.iter().enumerate() {
                plans[index] = operation.may_emit_foreign_steps() && !operation.foreign_steps(&running).is_empty();
                let mutation_id = ids.get(index).cloned().unwrap_or_else(|| MutationId(format!("{}#{index}", edit.id)));
                let effective = effective_operation::<P, Mutation>(operation, index as u32, mutation_id, &self.envelope.schema, self.supersessions());
                if let Some(folded) = effective.operation() {
                    if let (Some(next), _) = fold_operation::<P, Mutation>(&running, folded, index as u32) {
                        <Arc<P> as FoldProjection<P, Mutation>>::advance(&mut running, next);
                    }
                }
            }
            retire_shared_projection::<P, Mutation>(running);
        }
        Ok((0..edit.forwards.len())
            .map(|index| {
                let meta = edit.mutation_meta.get(index);
                let spans = plans[index] || matches!(meta.map(|meta| &meta.origin), Some(crate::os_spr::MutationOrigin::Transaction { .. }));
                spans.then(|| meta.and_then(|meta| meta.group_id.clone()).or_else(|| ids.get(index).map(|id| id.0.clone())).unwrap_or_else(|| format!("{}#{index}", edit.id)))
            })
            .collect())
    }

'''

EDITS = [
    (
        "row-field",
        "    pub supersession: Option<&'a protocol::EffectiveSupersession>,\n    pub effective: &'static crate::os_spr::MutationLeafDescriptor,\n}\n",
        "    pub supersession: Option<&'a protocol::EffectiveSupersession>,\n    pub effective: &'static crate::os_spr::MutationLeafDescriptor,\n"
        "    /// 🫷️ The cross-artifact unit the operation belongs to, `None` when it is its own unit ([`ArtifactStore::unit_id`]): an\n"
        "    /// operation of a unit takes no supersession in one document alone.\n"
        "    pub unit: Option<String>,\n}\n",
        "    pub unit: Option<String>,\n}\n",
    ),
    (
        "row-units",
        "        let edit = self.applied_edit(position)?;\n        let mut operations = Vec::with_capacity(edit.forwards.len());\n",
        "        let edit = self.applied_edit(position)?;\n        let units = self.applied_edit_units(position)?;\n        let mut operations = Vec::with_capacity(edit.forwards.len());\n",
        "        let units = self.applied_edit_units(position)?;\n        let mut operations = Vec::with_capacity(edit.forwards.len());\n",
    ),
    (
        "row-build",
        "            operations.push(AppliedMutation { mutation_id, edit_id: edit.id.as_str(), position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective });\n",
        "            operations.push(AppliedMutation { mutation_id, edit_id: edit.id.as_str(), position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective, unit: units.get(index).cloned().flatten() });\n",
        "supersession, effective, unit: units.get(index).cloned().flatten() });\n",
    ),
    (
        "member-trait",
        "    /// `ChildStoreFactory::open` on reload. The full history, not just the current content.\n    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;\n",
        "    /// `ChildStoreFactory::open` on reload. The full history, not just the current content.\n    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;\n"
        "    /// 🪼️ Merges the event log of `envelope_pack` — another writer's persisted copy of THIS member — into it\n"
        "    /// ([`ArtifactStore::merge_persisted_pair`]): the member keeps its head and its own edits. The default refuses: a member\n"
        "    /// without an event log of its own has nothing to merge, and its owner falls back to a load.\n"
        "    async fn merge_persisted_envelope(&mut self, _envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {\n"
        '        Err(VcsError::ValidationFailed("this member holds no event log a persisted envelope merges into".into()))\n'
        "    }\n",
        "    async fn merge_persisted_envelope(&mut self, _envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {\n        Err(",
    ),
    (
        "member-store",
        "    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {\n        let files = print_document_pack(self.envelope()).await?;\n        Ok(encode_document_pack_bytes(&files.pack, &files.spr).await)\n    }\n",
        "    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {\n        let files = print_document_pack(self.envelope()).await?;\n        Ok(encode_document_pack_bytes(&files.pack, &files.spr).await)\n    }\n"
        "\n"
        "    async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {\n"
        "        let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;\n"
        "        self.merge_persisted_pair(&pack, &spr).await\n"
        "    }\n",
        "    async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {\n        let (pack, spr)",
    ),
]


def main():
    text = STORE.read_text(encoding="utf-8")
    pending = []
    if "    pub async fn merge_persisted_history(" not in text:
        if text.count(MERGE_ANCHOR) != 1:
            raise SystemExit(f"anchor `merge` occurs {text.count(MERGE_ANCHOR)} times (expected 1): re-derive the wave")
        text = text.replace(MERGE_ANCHOR, MERGE_REGION)
        pending.append("merge")
    if "    fn applied_edit_units(&self, position: usize)" not in text:
        if text.count(UNIT_START) != 1 or text.count(UNIT_END) != 1:
            raise SystemExit(f"unit block anchors occur {text.count(UNIT_START)} / {text.count(UNIT_END)} times (expected 1 / 1): re-derive the wave")
        start, end = text.index(UNIT_START), text.index(UNIT_END)
        if not start < end or "fn plans_foreign_steps" not in text[start:end]:
            raise SystemExit("the unit block is not the one wave FW landed: re-derive the wave")
        text = text[:start] + UNIT_BLOCK + text[end:]
        pending.append("units")
    for name, old, new, marker in EDITS:
        if marker in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--emit" in sys.argv[1:]:
        target = Path(sys.argv[sys.argv.index("--emit") + 1])
        target.mkdir(parents=True, exist_ok=True)
        (target / "store.rs").write_text(text, encoding="utf-8")
        print(f"emitted to {target}")
        return
    if "--check" in sys.argv[1:] or not pending:
        return
    STORE.write_text(text, encoding="utf-8")
    print("applied")


main()
