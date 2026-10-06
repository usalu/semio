#!/usr/bin/env python3
"""🪆 S5-NESTED wave N3 (design §21.9): composition is recursive — every live maintenance path follows member projections.

Usage: python3 🧪️s5-nested-n3-recursion.py check|land|restore
  check   -> verifies every anchor, on the live files or — while N2 is not on disk — on N2's planned result
  land    -> needs N1 and N2 on disk; keeps pre-images under 🗑️generated/s5-nested/pre-n3/ and writes every file
  restore -> puts every pre-image back
Region and exact-string replacements with counted anchors; any drift fails closed before the first write.
"""
import importlib.util
import pathlib
import shutil
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
PRE = TICKET / "🗑️generated/s5-nested/pre-n3"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUNTIME = f"{PLUGIN}/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"

GENESIS_START = "        /// 🌱️ Opens every composed child the initial snapshot declares and the app derives\n"
GENESIS_END = "        pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {"
GENESIS_NEW = '''        /// 🌱️ Opens every composed member the initial snapshot declares and its owner derives — the app for the document's
        /// own children (`ArtifactApp::genesis_child_pack`), the member roster for the children a member composes
        /// (`MemberFactory::genesis_child_pack`, design §21.9) — through the ordinary restore path, so `children`, the
        /// composition graph and the child content root are complete from the first frame at every depth: a saved archive
        /// of a never-edited document then carries its derivable members instead of relying on the loading side to
        /// re-derive them.
        async fn seed_genesis_children(&mut self) -> Result<(), Fault> {
            match self.follow_member_owners().await? {
                Some(false) => Ok(()),
                Some(true) | None => Err(plugin_sdk_fault("a freshly constructed store waits on no child retirement while its derivable members open")),
            }
        }

        /// 🪆️ Derivable composed members follow their coordinate, at every depth (design §21.9). A content-addressed child is
        /// minted from its owner's own content, so an owner-lane change can re-point a declared coordinate at a child id no
        /// store holds (P8 `composedChildOrphaned`): a parent-lane change for the document's own children, a member-lane
        /// change for the children that member composes. This opens every declared child no store holds that its owner
        /// derives and retires every held child whose slot now names another derived child, so exporters, archives, hub
        /// members, agents and the next verb read the child its owner names. Gated by the parent store's generation AND by
        /// the child-content generation (every member-lane publication advances it), and deferred while a child admission is
        /// in flight or while the named child is still retiring (an undo back to it) — the pass then repeats until that
        /// retirement drained. It also waits, before it changes anything for an owner, while the retirements its own
        /// publications need are not yet admissible ([`Self::reclaim_child_retirements`]): every owner already holds its
        /// edits in order, the child it names follows once maintenance returned them, and the instance stays runnable
        /// meanwhile (`child_follow_awaits_retirement`).
        async fn follow_derivable_children(&mut self) -> Result<(), Fault> {
            let generation = self.store.generation_now();
            if (generation == self.followed_parent_generation && self.child_content_generation == self.followed_member_generation) || self.children.has_admission_in_flight() {
                return Ok(());
            }
            match self.follow_member_owners().await? {
                None => self.child_follow_awaits_retirement = true,
                Some(deferred) => {
                    if !deferred {
                        self.followed_parent_generation = generation;
                        self.followed_member_generation = self.child_content_generation;
                    }
                    self.child_follow_awaits_retirement = deferred;
                }
            }
            Ok(())
        }

        /// 🚶️ One follow pass over every owner, breadth first: the document, then every live member — the ones the pass
        /// itself opens included, each with the member as `owner.parent` of what it derives. Answers `None` when the
        /// retirements one owner's publications need are not admissible yet (that owner is left untouched), else whether a
        /// declared child was still retiring.
        async fn follow_member_owners(&mut self) -> Result<Option<bool>, Fault> {
            let mut owners: Vec<Option<MemberKey>> = vec![None];
            let mut deferred = false;
            let mut next = 0;
            while next < owners.len() {
                let owner = owners[next].clone();
                next += 1;
                let Some(plan) = self.member_follow_plan(owner.as_ref())? else { continue };
                deferred |= plan.deferred;
                let publications = plan.retire.len() + plan.genesis.len();
                if !self.reclaim_child_retirements(publications, publications)? {
                    return Ok(None);
                }
                for key in &plan.retire {
                    self.retire_followed_member(key).await?;
                }
                for (slot, child_id, expected, initial_pack) in plan.genesis {
                    let dialect = expected.dialect.clone();
                    let schema = genesis_member_schema::<M>(&dialect)?;
                    let member_owner = store::OwnerRef { parent: plan.parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
                    let envelope_pack = store::genesis_member_envelope_pack(schema, &expected, &member_owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("derived child {slot}/{child_id} envelope: {error}")))?;
                    self.open_member(owner.as_ref(), slot, child_id, dialect, &envelope_pack).await?;
                }
                owners.extend(self.children.keyed_entries().filter(|(key, _)| key.owner == plan.owner_id).map(|(key, _)| Some(key.to_key())));
            }
            Ok(Some(deferred))
        }

        /// 📋️ What one owner — the document (`None`) or one live member — owes the children its snapshot declares: the
        /// unheld ones it derives (to open), the held ones whose slot now names another derived child (to retire, each
        /// with every member it owns in turn, leaves first), and whether a declared child is still retiring. `None` for an
        /// owner that is no longer live.
        fn member_follow_plan(&self, owner: Option<&MemberKey>) -> Result<Option<MemberFollowPlan>, Fault> {
            let owning = match owner {
                None => None,
                Some(owner) => match self.children.member(owner.borrowed()) {
                    Some(entry) => Some(entry),
                    None => return Ok(None),
                },
            };
            let parent = match owning {
                Some(entry) => entry.reference.clone(),
                None => ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() },
            };
            let projection = match owning {
                Some(entry) => entry.member.child_restore_projection(),
                None => store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()),
            }
            .map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
            let mut plan = MemberFollowPlan { owner_id: owning.map(|entry| entry.reference.artifact_id.clone()).unwrap_or_default(), parent, genesis: Vec::new(), retire: Vec::new(), deferred: false };
            let mut declared: Vec<(&str, &str)> = Vec::with_capacity(projection.len());
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                if self.children.member(MemberKeyRef { owner: &plan.owner_id, slot, child_id: fields.child_id }).is_none() {
                    if self.child_member_retirements.retains_member(fields.artifact_id) {
                        plan.deferred = true;
                    } else {
                        let derived = match owning {
                            Some(entry) => entry.member.genesis_child_pack(slot, fields.child_id),
                            None => A::genesis_child_pack(self.store.snapshot_ref(), slot, fields.child_id),
                        }
                        .map_err(|error| plugin_sdk_fault(error.into_message()))?;
                        if let Some(initial_pack) = derived {
                            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                            plan.genesis.push((slot.to_string(), fields.child_id.to_string(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
                        }
                    }
                }
                declared.push((slot, fields.child_id));
            }
            let mut retire: Vec<MemberKey> = self
                .children
                .keyed_entries()
                .filter(|(key, _)| key.owner == plan.owner_id && !declared.contains(&(key.slot, key.child_id)) && plan.genesis.iter().any(|(slot, _, _, _)| slot.as_str() == key.slot))
                .map(|(key, _)| key.to_key())
                .collect();
            let mut next = 0;
            while next < retire.len() {
                if let Some(owned) = self.children.member(retire[next].borrowed()).map(|entry| entry.reference.artifact_id.as_str()) {
                    let nested: Vec<MemberKey> = self.children.keyed_entries().filter(|(key, _)| key.owner == owned).map(|(key, _)| key.to_key()).collect();
                    retire.extend(nested);
                }
                next += 1;
            }
            retire.reverse();
            plan.retire = retire;
            Ok(Some(plan))
        }

        /// 🍂️ Moves one held member its coordinate left out of every live authority — the member map, the ownership
        /// graph and the published child-content root (the previous root goes to bounded content retirement) —
        /// into bounded child-member retirement, where it stays the disposer of the snapshot leases that previous
        /// root still holds (`ChildContentRetirement::close_step` finds it there) and closes only after them.
        async fn retire_followed_member(&mut self, key: &MemberKey) -> Result<(), Fault> {
            let retirement_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child member retirement generation exhausted"))?;
            if !self.reclaim_child_retirements(0, 1)? {
                return Err(plugin_sdk_fault("child member retirement waits on retirements that are all still borrowed").with_retryable(true));
            }
            let publication_generation = self.admit_child_content_publication()?;
            let next = self.child_content_root.without_member(key.borrowed())?;
            let entry = self.children.remove(key.borrowed())?.ok_or_else(|| plugin_sdk_fault("followed child left the member map before its exact retirement"))?;
            self.composition.graph_mut().await.remove_owns(&entry.reference.artifact_id).await;
            let previous = std::mem::replace(&mut *self.child_content_root, next);
            if !previous.is_empty() {
                self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
            }
            self.child_content_generation = publication_generation;
            self.child_member_retirements.insert_admitted(retirement_generation, ChildMemberRetirement::new(entry));
            self.child_member_retirement_generation = retirement_generation;
            Ok(())
        }

'''

ARCHIVE_START = "        /// 🌱️ Completes an archive's member roster with every child the hydrated candidate parent declares,\n"
ARCHIVE_END = "        /// 🧹️ `retire_envelope_uninstalled`, never `retire_envelope` on a temporary catalog: the\n"
ARCHIVE_NEW = '''        /// 🌳️ Every member a candidate root derives and its roster lacks — and, breadth first, every member THOSE derive
        /// from their own genesis pack (`MemberFactory::genesis_children`, design §21.9) — as archive entries owner-stamped
        /// to the member that derives them and numbered after the `archived` ones. A member the archive ships with its
        /// history ships its own children too: they are never re-derived here, so a roster that lacks one stays
        /// `Incomplete`. The fixed member authority and, with `payload_bytes`, the fixed byte authority are re-checked
        /// over the completed roster.
        fn derived_member_entries(parent: &ArtifactRef, snapshot: &A::Snapshot, archived: &[protocol::OwnedDocumentMemberPackEntry], mut payload_bytes: Option<usize>) -> Result<Vec<protocol::OwnedDocumentMemberPackEntry>, Fault> {
            let archive_reference = |reference: &ArtifactRef| protocol::DocumentArchiveArtifactRef { artifact_id: reference.artifact_id.clone(), artifact_kind: reference.dialect.artifact_kind.clone(), standard: reference.dialect.standard.clone(), subset: reference.dialect.subset.clone() };
            let shipped = |owner: &str, slot: &str, child_id: &str| archived.iter().any(|entry| entry.owner.parent.artifact_id == owner && entry.owner.slot == slot && entry.owner.child_id == child_id);
            let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("document genesis child projection failed: {error}")))?;
            let mut pending: std::collections::VecDeque<(ArtifactRef, store::MemberGenesisChild)> = std::collections::VecDeque::new();
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                if shipped(&parent.artifact_id, slot, fields.child_id) {
                    continue;
                }
                let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
                let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                pending.push_back((parent.clone(), store::MemberGenesisChild { slot: slot.to_string(), child_id: fields.child_id.to_string(), reference: ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack }));
            }
            let mut entries = Vec::new();
            while let Some((owner, child)) = pending.pop_front() {
                let ordinal = archived.len() + entries.len();
                if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                    return Err(plugin_sdk_fault("document genesis exceeds its fixed 1024-member authority"));
                }
                let schema = genesis_member_schema::<M>(&child.reference.dialect)?;
                let member_owner = store::OwnerRef { parent: owner, slot: child.slot.clone(), child_id: child.child_id.clone() };
                let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::genesis_member_envelope_pack(schema, &child.reference, &member_owner, &child.initial_pack))
                    .map_err(|error| plugin_sdk_fault(format!("genesis child {}/{} envelope: {error}", child.slot, child.child_id)))?;
                if let Some(bytes) = payload_bytes.as_mut() {
                    *bytes = bytes.checked_add(envelope_pack.len()).filter(|bytes| *bytes <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES).ok_or_else(|| plugin_sdk_fault("document archive genesis exceeds its fixed byte authority"))?;
                }
                for nested in M::genesis_children(&child.reference.dialect, &child.initial_pack).map_err(|error| plugin_sdk_fault(error.to_string()))? {
                    if !shipped(&child.reference.artifact_id, &nested.slot, &nested.child_id) {
                        pending.push_back((child.reference.clone(), nested));
                    }
                }
                entries.push(protocol::OwnedDocumentMemberPackEntry {
                    ordinal: u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("document genesis ordinal exceeds u32"))?,
                    reference: archive_reference(&child.reference),
                    owner: protocol::DocumentArchiveOwnerRef { parent: archive_reference(&member_owner.parent), slot: member_owner.slot, child_id: member_owner.child_id },
                    envelope_pack,
                });
            }
            Ok(entries)
        }

        /// 🌱️ Completes an archive's member roster with every member the hydrated candidate parent declares, the archive
        /// omits, and its owner derives ([`Self::derived_member_entries`]) — the content-addressed members a whole-document
        /// load (`Effect::LoadDocument`, which carries pack+spr only) can never ship, at every depth. A slot nothing
        /// derives is left alone, so closure validation still rejects it as `Incomplete`.
        ///
        /// 🔢️ Archived entries are already reversed for ordinal-ordered popping; ordinals stay
        /// `0..n` for them and continue at `n` for every minted member, whatever the pop order.
        fn complete_document_archive_genesis(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
            let archive = active.archive.as_mut().ok_or_else(|| plugin_sdk_fault("recursive document archive member roster owner is absent"))?;
            let candidate = self
                .store_replacement_jobs
                .get(handle.operation.0)
                .filter(|replacement| replacement.operation == handle.operation && replacement.generation == handle.generation)
                .and_then(ActiveArtifactStoreReplacement::candidate_awaiting_members)
                .ok_or_else(|| plugin_sdk_fault("document archive genesis has no hydrated candidate parent awaiting members"))?;
            let dialect = candidate.envelope().dialect.clone().ok_or_else(|| plugin_sdk_fault("document archive genesis requires the candidate parent's exact dialect"))?;
            let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
            let payload_bytes = archive.members.iter().fold(archive.parent_spr.len(), |total, entry| total + entry.envelope_pack.len());
            for entry in Self::derived_member_entries(&parent, candidate.snapshot_ref(), &archive.members, Some(payload_bytes))? {
                archive.members.push(entry);
                active.total = active.total.saturating_add(1);
            }
            Ok(())
        }

        /// 🌿️ Mints every derivable composed member onto a live envelope replacement's member roster, at every depth —
        /// the JSON ingress path ships only the parent envelope, so the owners' derivations must supply the same members
        /// `seed_genesis_children` opened at boot.
        fn complete_store_replacement_genesis(&mut self, operation_id: u64) -> Result<(), Fault> {
            let active = self.store_replacement_jobs.get(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis lost its exact replacement authority"))?;
            let handle = ArtifactEnvelopeDecodeOperationHandle { operation: active.operation, generation: active.generation };
            let candidate = active.retained_store.as_ref().ok_or_else(|| plugin_sdk_fault("live store replacement genesis has no hydrated candidate parent"))?;
            let dialect = candidate.envelope().dialect.clone().ok_or_else(|| plugin_sdk_fault("live store replacement genesis requires the candidate parent's exact dialect"))?;
            let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
            let entries = Self::derived_member_entries(&parent, candidate.snapshot_ref(), &[], None)?;
            let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis changed before member admission"))?;
            active.begin_members(entries.len(), u64::MAX)?;
            for entry in entries {
                let mut pending = PendingDocumentArchiveMember::new(entry).map_err(|(fault, _)| fault)?;
                while !pending.fill_one_page(store::OWNED_SCHEMA_DECODE_PAGE_BYTES)? {}
                while pending.retire_source_step(store::OWNED_SCHEMA_DECODE_PAGE_BYTES) != PluginCloseStep::Complete {}
                let ingress = pending.take_ingress(handle).map_err(|(fault, _)| fault)?;
                active.admit_member(ingress).map_err(|(fault, _)| fault)?;
            }
            active.seal_members()?;
            Ok(())
        }

'''

ADMIT_START = "        fn declared_child_reference(&self, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {"
ADMIT_END = "        async fn prepare_child_member("
ADMIT_NEW = '''        /// 🪪️ The identity one owner — the document (`None`) or one live member — declares for the child at
        /// `(slot, child_id)` of its own snapshot: the only authority a member is admitted under (design §21.9).
        fn declared_child_reference(&self, owner: Option<MemberKeyRef<'_>>, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {
            let projection = match owner {
                None => store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()),
                Some(owner) => self.children.member(owner).ok_or_else(|| plugin_sdk_fault("child member identity names an owning member that is not live"))?.member.child_restore_projection(),
            }
            .map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
            let fields = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.child_id == child_id).map(|(_, fields)| fields).ok_or_else(|| plugin_sdk_fault("child member identity is not declared by the current parent"))?;
            if fields.artifact_kind != dialect.artifact_kind || fields.standard != dialect.standard || fields.subset != dialect.subset {
                return Err(plugin_sdk_fault("child member dialect differs from the declared target"));
            }
            Ok(ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() })
        }

        async fn admit_child_member(&mut self, owner: Option<MemberKeyRef<'_>>, slot: String, child_id: String, dialect: ArtifactDialect) -> Result<ChildStoreAdmission, Fault> {
            let expected = self.declared_child_reference(owner, &slot, &child_id, &dialect)?;
            let (parent, key) = match owner {
                None => {
                    let parent_dialect: ArtifactDialect = A::DIALECT.into();
                    if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
                        return Err(plugin_sdk_fault("owned member admission requires the parent's exact declared dialect"));
                    }
                    (ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: parent_dialect }, MemberKey::root(slot, child_id))
                }
                Some(owner) => {
                    let parent = self.children.member(owner).ok_or_else(|| plugin_sdk_fault("owned member admission lost its owning member"))?.reference.clone();
                    let key = MemberKey { owner: parent.artifact_id.clone(), slot, child_id };
                    (parent, key)
                }
            };
            let root_index = self.child_content_root.admit_member(key.borrowed())?;
            let generation = self.admit_child_content_publication()?;
            let graph = self.composition.graph_mut().await.admit_owns(&parent.artifact_id, &key.slot, &expected.artifact_id).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
            let owner = store::OwnerRef { parent, slot: key.slot.clone(), child_id: key.child_id.clone() };
            let member = self.children.admit_identity(key.borrowed())?;
            Ok(ChildStoreAdmission { key, expected, owner, member, graph, root_index, generation })
        }

'''

OPEN_START = "        fn validate_parent_child_restore(&self, slot: &str, child_id: &str, expected: &ArtifactRef) -> Result<(), Fault> {"
OPEN_END = "        /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of\n"
OPEN_NEW = '''        fn validate_parent_child_restore(&self, owner: Option<MemberKeyRef<'_>>, slot: &str, child_id: &str, expected: &ArtifactRef) -> Result<(), Fault> {
            let projection = match owner {
                None => {
                    let parent_dialect: ArtifactDialect = A::DIALECT.into();
                    if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
                        return Err(plugin_sdk_fault("child restore requires the parent's exact declared dialect"));
                    }
                    store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref())
                }
                Some(owner) => self.children.member(owner).ok_or_else(|| plugin_sdk_fault("child restore lost its owning member"))?.member.child_restore_projection(),
            }
            .map_err(|error| plugin_sdk_fault(format!("child restore projection failed: {error:?}")))?;
            if !(0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot && fields.child_id == child_id && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.dialect.artifact_kind && fields.standard == expected.dialect.standard && fields.subset == expected.dialect.subset) {
                return Err(plugin_sdk_fault("child restore is not declared by the loaded parent snapshot"));
            }
            Ok(())
        }

        /// 🌱️ Restores one exact member the document itself owns and publishes its graph, map and snapshot together.
        pub async fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8]) -> Result<(), Fault> {
            self.open_member(None, slot.into(), child_id.into(), dialect, envelope_pack).await
        }

        /// 🪴️ [`Self::open_child`] under any owner: the document (`None`) or one live member, whose own snapshot declares the
        /// child and whose reference becomes the child's `owner.parent` (design §21.9).
        async fn open_member(&mut self, owner: Option<&MemberKey>, slot: String, child_id: String, dialect: ArtifactDialect, envelope_pack: &[u8]) -> Result<(), Fault> {
            let owner = owner.map(MemberKey::borrowed);
            let expected = self.declared_child_reference(owner, &slot, &child_id, &dialect)?;
            self.validate_parent_child_restore(owner, &slot, &child_id, &expected)?;
            let parent_generation = self.store.generation_now();
            let abort_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("failed child retirement generation exhausted"))?;
            if !self.reclaim_child_retirements(0, 1)? {
                return Err(plugin_sdk_fault("failed child retirement waits on retirements that are all still borrowed").with_retryable(true));
            }
            let admission = self.admit_child_member(owner, slot, child_id, dialect).await?;
            let mut member = match M::open(&admission.expected, Some(&admission.owner), envelope_pack).await {
                Ok(member) => member,
                Err(error) => {
                    assert!(self.children.cancel_admission(&admission.member), "failed child open retains its exact admission until cancellation");
                    return Err(plugin_sdk_fault(error.to_string()));
                }
            };
            let prepared = if self.store.generation_now() != parent_generation {
                Err(plugin_sdk_fault("loaded parent changed during child restore"))
            } else {
                match self.validate_parent_child_restore(owner, &admission.key.slot, &admission.key.child_id, &admission.expected) {
                    Ok(()) => self.prepare_child_member(&admission, &mut member, true).await,
                    Err(fault) => Err(fault),
                }
            };
            let (root, pin_index) = match prepared {
                Ok(prepared) => prepared,
                Err(fault) => {
                    assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                    self.child_member_retirements.insert_admitted(abort_generation, ChildMemberRetirement::new(ChildMemberEntry { reference: admission.expected, owner: admission.owner, member }));
                    self.child_member_retirement_generation = abort_generation;
                    return Err(fault);
                }
            };
            self.commit_child_member(admission, member, root, pin_index).await;
            Ok(())
        }

'''

FOLLOW_PLAN_STRUCT = '''    /// 📋️ What one owner owes its declared children in one follow pass (`VcsArtifactApp::member_follow_plan`): the owner's
    /// artifact id as member keys carry it (empty for the document) and its reference, the children to open as
    /// `(slot, child_id, reference, initial pack)`, the members to retire leaves first, and whether a declared child is
    /// still retiring.
    struct MemberFollowPlan {
        owner_id: String,
        parent: ArtifactRef,
        genesis: Vec<(String, String, ArtifactRef, Vec<u8>)>,
        retire: Vec<MemberKey>,
        deferred: bool,
    }

'''

RUNTIME_REGIONS = [
    (GENESIS_START, GENESIS_END, GENESIS_NEW),
    (ARCHIVE_START, ARCHIVE_END, ARCHIVE_NEW),
    (ADMIT_START, ADMIT_END, ADMIT_NEW),
    (OPEN_START, OPEN_END, OPEN_NEW),
]
RUNTIME_REPLACEMENTS = [
    ("    struct ChildStoreAdmission {\n        key: MemberKey,\n", FOLLOW_PLAN_STRUCT + "    struct ChildStoreAdmission {\n        key: MemberKey,\n", 1),
    ("        followed_parent_generation: u64,\n", "        followed_parent_generation: u64,\n        followed_member_generation: u64,\n", 1),
    ("                followed_parent_generation: 0,\n", "                followed_parent_generation: 0,\n                followed_member_generation: 0,\n", 1),
    (
        "            this.followed_parent_generation = this.store.generation_now();\n",
        "            this.followed_parent_generation = this.store.generation_now();\n            this.followed_member_generation = this.child_content_generation;\n",
        1,
    ),
    (
        "            self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation\n",
        "            self.child_follow_awaits_retirement && (self.store.generation_now() != self.followed_parent_generation || self.child_content_generation != self.followed_member_generation)\n",
        1,
    ),
    (
        "            let admission = match self.admit_child_member(slot.into(), child_id.into(), dialect).await {\n",
        "            let admission = match self.admit_child_member(None, slot.into(), child_id.into(), dialect).await {\n",
        1,
    ),
    (
        "            self.follow_derivable_children().await?;\n            let folded_member_lanes = self.fold_member_inbound().await?;\n",
        "            self.follow_derivable_children().await?;\n            let folded_member_lanes = self.fold_member_inbound().await?;\n            if folded_member_lanes != 0 {\n                self.followed_member_generation = u64::MAX;\n                self.follow_derivable_children().await?;\n            }\n",
        1,
    ),
]

STORE_FACTORY_OLD = '''pub trait MemberFactory: MemberVisit + Sized {
    const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration];
    type Open: MemberOpenOperation<Member = Self> + Send;
    fn begin_open(request: MemberOpenRequest) -> Result<Self::Open, MemberOpenAdmissionError>;
    async fn create(id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError>;
    async fn open(expected: &crate::os_io::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8]) -> Result<Self, VcsError>;
}
'''
STORE_FACTORY_NEW = '''/// 🌱️ One child a composed member derives from its own content: where its snapshot declares it, the identity declared
/// there and the initial pack it is minted from.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberGenesisChild {
    pub slot: String,
    pub child_id: String,
    pub reference: crate::os_io::ArtifactRef,
    pub initial_pack: Vec<u8>,
}

/// 🧮️ The derivation authority of one member artifact: the function its own app answers `genesis_child_pack` with, named
/// on the member roster so a member composed under another document still derives the children it composes (design §21.9).
pub type MemberDerivation<P> = fn(&P, &str, &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError>;

/// 🫙️ The derivation of a member artifact that derives no child.
pub fn no_member_derivation<P>(_snapshot: &P, _slot: &str, _child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
    Ok(None)
}

/// 🌿️ Every child `snapshot` declares and `derive` answers an initial pack for, in declaration order.
pub fn member_genesis_children<P: semio_framework_schema_composition::ArtifactCompositionFields>(snapshot: &P, derive: MemberDerivation<P>) -> Result<Vec<MemberGenesisChild>, VcsError> {
    let projection = ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
    let mut children = Vec::new();
    for index in 0..projection.len() {
        let Some((slot, fields)) = projection.get(index) else { break };
        let Some(initial_pack) = derive(snapshot, slot, fields.child_id).map_err(|error| VcsError::ValidationFailed(error.into_message()))? else { continue };
        let dialect = crate::os_io::ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
        children.push(MemberGenesisChild { slot: slot.to_string(), child_id: fields.child_id.to_string(), reference: crate::os_io::ArtifactRef { artifact_id: fields.artifact_id.to_string(), dialect }, initial_pack });
    }
    Ok(children)
}

/// 🧩️ The derivation a `space_members!` variant names, or [`no_member_derivation`] when it names none.
#[doc(hidden)]
#[macro_export]
macro_rules! __space_member_derivation {
    () => {
        $crate::os_store::no_member_derivation
    };
    ($derive:expr) => {
        $derive
    };
}

pub trait MemberFactory: MemberVisit + Sized {
    const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration];
    type Open: MemberOpenOperation<Member = Self> + Send;
    fn begin_open(request: MemberOpenRequest) -> Result<Self::Open, MemberOpenAdmissionError>;
    async fn create(id: &str, dialect: &crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Self, VcsError>;
    async fn open(expected: &crate::os_io::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8]) -> Result<Self, VcsError>;
    /// 🌱️ The initial pack of the child this LIVE member derives at `(slot, child_id)` of its own snapshot — the member's
    /// own derivation authority (design §21.9); `None` for a child it does not derive. A roster that names no derivation
    /// derives nothing.
    fn genesis_child_pack(&self, _slot: &str, _child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        Ok(None)
    }
    /// 🌿️ Every child a member of `dialect` freshly minted from `initial_pack` declares and derives — what completes a
    /// roster below a minted member before any store of it is open.
    fn genesis_children(_dialect: &crate::os_io::ArtifactDialect, _initial_pack: &[u8]) -> Result<Vec<MemberGenesisChild>, VcsError> {
        Ok(Vec::new())
    }
}
'''

STORE_MACRO_FACTORY_OLD = '''            async fn open(expected: &$crate::os_io::ArtifactRef, owner: Option<&$crate::os_store::OwnerRef>, envelope_pack: &[u8]) -> Result<Self, $crate::os_store::VcsError> {
                match (expected.dialect.artifact_kind.as_str(), expected.dialect.standard.as_str(), expected.dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::open_member_store($schema, expected, owner, envelope_pack).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", expected.dialect.to_coordinate(), stringify!($enum_name)))),
                }
            }
'''
STORE_MACRO_FACTORY_NEW = STORE_MACRO_FACTORY_OLD + '''            fn genesis_child_pack(&self, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, ::semio_framework_value::ValueError> {
                match self {
                    $(Self::$variant(m) => {
                        let derive: $crate::os_store::MemberDerivation<$snapshot> = $crate::__space_member_derivation!($($derive)?);
                        derive(m.snapshot_ref(), slot, child_id)
                    }),+
                }
            }
            fn genesis_children(dialect: &$crate::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Vec<$crate::os_store::MemberGenesisChild>, $crate::os_store::VcsError> {
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => {
                        let derive: $crate::os_store::MemberDerivation<$snapshot> = $crate::__space_member_derivation!($($derive)?);
                        let snapshot = <$snapshot as $crate::os_store::ArtifactPack>::decode_pack(initial_pack).map_err(|error| $crate::os_store::VcsError::Deserialize(format!("{error:?}")))?;
                        $crate::os_store::member_genesis_children(&snapshot, derive)
                    })+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", dialect.to_coordinate(), stringify!($enum_name)))),
                }
            }
'''

STORE_REPLACEMENTS = [
    (STORE_FACTORY_OLD, STORE_FACTORY_NEW, 1),
    (
        "    (pub enum $enum_name:ident, $open_name:ident { $($variant:ident($kind:literal, $standard:literal, $subset:literal, $schema:literal) => ($snapshot:ty, $mutation:ty)),+ $(,)? }) => {\n",
        "    (pub enum $enum_name:ident, $open_name:ident { $($variant:ident($kind:literal, $standard:literal, $subset:literal, $schema:literal) => ($snapshot:ty, $mutation:ty $(, $derive:expr)?)),+ $(,)? }) => {\n",
        1,
    ),
    (
        "///         Sketch(\"s.note.sketch\", \"1\", \"*\", \"note.sketch/v1\") => (SketchSnapshot, SketchMutation),\n",
        "///         Sketch(\"s.note.sketch\", \"1\", \"*\", \"note.sketch/v1\") => (SketchSnapshot, SketchMutation, <SketchApp as ArtifactApp>::genesis_child_pack),\n",
        1,
    ),
    (
        "/// method), and `impl MemberFactory for NoteMembers` with an exact coordinate per typed store.\n",
        "/// method), and `impl MemberFactory for NoteMembers` with an exact coordinate per typed store. A variant's optional third\n/// entry is the member artifact's own derivation ([`MemberDerivation`]): the children that member composes follow it\n/// wherever the member itself is composed (design §21.9); a variant without one derives nothing.\n",
        1,
    ),
    (STORE_MACRO_FACTORY_OLD, STORE_MACRO_FACTORY_NEW, 1),
]

CONTRACT_TEST = f"{PLUGIN}/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
TIME_TRAVEL_TEST = f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs"
CONTRACT_TEST_REPLACEMENTS = [
    ('Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation),', 'Child("s.test.child", "native", "*", "semio.test/v1") => (TestSnapshot, TestMutation, test_nested_derivation),', 1),
    (
        "        const DIALECT: Dialect = TEST_APP_DIALECT;\n",
        "        const DIALECT: Dialect = TEST_APP_DIALECT;\n        fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {\n            test_nested_derivation(snapshot, slot, child_id)\n        }\n",
        1,
    ),
    (
        '    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧩️composition/🦀️.rs"));\n',
        '    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧩️composition/🦀️.rs"));\n\n    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧪️nested-composition/🦀️.rs"));\n',
        1,
    ),
]
TIME_TRAVEL_TEST_REPLACEMENTS = [
    (
        "//#endregion 📁️FolderReloadRoute\n",
        '//#endregion 📁️FolderReloadRoute\n\n//#region 🪆️NestedMember\n#[path = "../🧪️time-travel-nested-member/🦀️.rs"]\nmod nested_member;\n//#endregion 🪆️NestedMember\n',
        1,
    ),
]
NEW_FILES = [
    ("🧪️s5-nested-n3-law-nested-composition.rs", f"{PLUGIN}/🧪️tests/🧪️nested-composition/🦀️.rs"),
    ("🧪️s5-nested-n3-law-time-travel-nested-member.rs", f"{PLUGIN}/🧪️tests/🧪️time-travel-nested-member/🦀️.rs"),
]

FILES = [
    (RUNTIME, RUNTIME_REPLACEMENTS, RUNTIME_REGIONS, "async fn follow_member_owners(&mut self)"),
    (STORE, STORE_REPLACEMENTS, [], "pub struct MemberGenesisChild {"),
    (CONTRACT_TEST, CONTRACT_TEST_REPLACEMENTS, [], "test_nested_derivation(snapshot, slot, child_id)"),
    (TIME_TRAVEL_TEST, TIME_TRAVEL_TEST_REPLACEMENTS, [], "mod nested_member;"),
]


def load(name: str):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), TICKET / f"🧪️{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def base_text(n1, n2, relative: str) -> tuple[str, bool]:
    text = (REPO / relative).read_text(encoding="utf-8")
    landed = True
    for candidate, replacements, regions, marker in n1.FILES:
        if candidate == relative and marker not in text:
            text, landed = n1.planned(relative, text, replacements, regions), False
    wave = {n2.RUNTIME: (n2.RUNTIME_REPLACEMENTS + n2.FRAME_REPLACEMENTS, "pub fn owner_text(&self) -> String {"), n2.STORE: (n2.STORE_REPLACEMENTS, "BackboneMessage::Member { owner: owner.to_string(),")}
    if relative in wave and wave[relative][1] not in text:
        text, landed = n2.planned(relative, text, wave[relative][0]), False
    return text, landed


def planned(name: str, text: str, replacements, regions) -> str:
    for start, end, new in regions:
        if text.count(start) != 1 or text.count(end) != 1:
            raise SystemExit(f"{name}: region anchor drift ({text.count(start)} start, {text.count(end)} end): {start[:90]!r}")
        head, tail = text.index(start), text.index(end)
        if head >= tail:
            raise SystemExit(f"{name}: region anchors out of order: {start[:90]!r}")
        text = text[:head] + new + text[tail:]
    for old, new, count in replacements:
        found = text.count(old)
        if found != count:
            raise SystemExit(f"{name}: anchor drift: expected {count}, found {found}: {old[:140]!r}")
        text = text.replace(old, new)
    return text


def pre_image(relative: str) -> pathlib.Path:
    return PRE / relative.replace("/", "__")


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "restore":
        for relative, _, _, _ in FILES:
            if pre_image(relative).exists():
                shutil.copyfile(pre_image(relative), REPO / relative)
                print(f"restored {relative}")
        for _, target in NEW_FILES:
            if (REPO / target).exists():
                (REPO / target).unlink()
                (REPO / target).parent.rmdir()
                print(f"removed {target}")
        return
    n1, n2 = load("s5-nested-n1-keys"), load("s5-nested-n2-wire")
    results = []
    on_disk = True
    for relative, replacements, regions, marker in FILES:
        text, landed = base_text(n1, n2, relative)
        on_disk = on_disk and landed
        if marker in text:
            print(f"already landed: {relative}")
            continue
        results.append((relative, text, planned(relative, text, replacements, regions)))
        print(f"{relative}: {sum(count for _, _, count in replacements)} replacements, {len(regions)} regions on {'the live file' if landed else 'the planned predecessor'}; {len(text)} -> {len(results[-1][2])} bytes")
    if mode == "land":
        if not on_disk:
            raise SystemExit("N1 and N2 are not both on disk: land them first")
        PRE.mkdir(parents=True, exist_ok=True)
        for relative, text, result in results:
            pre_image(relative).write_text(text, encoding="utf-8")
            (REPO / relative).write_text(result, encoding="utf-8")
            print(f"landed {relative}")
        for source, target in NEW_FILES:
            (REPO / target).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(TICKET / source, REPO / target)
            print(f"created {target}")


if __name__ == "__main__":
    main()
