#!/usr/bin/env python3
"""🪜 S5-NESTED wave N1 (design §21.9): composed members are keyed by their owner edge, addressed by their owner path.

Usage: python3 🧪️s5-nested-n1-keys.py check|land|restore
  check   -> verifies every anchor count against the live files, writes nothing
  land    -> keeps pre-images under 🗑️generated/s5-nested/pre-n1/ and writes every file (also lands the P1 fixture)
  restore -> puts every pre-image back (and the P1 fixture's)
Exact-string replacements with counted anchors; any drift fails closed before the first write.
"""
import pathlib
import shutil
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TICKET = pathlib.Path(__file__).resolve().parent
PRE = TICKET / "🗑️generated/s5-nested/pre-n1"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUNTIME = f"{PLUGIN}/🦀️.rs"
TIME_TRAVEL = f"{PLUGIN}/⏪️time-travel/🦀️.rs"
TOOL_RUN = f"{PLUGIN}/⏯️tool-run/🦀️.rs"
REGISTRY_TEST = f"{PLUGIN}/🧪️tests/🔬️app-child-member-registry/🦀️.rs"
COMPOSITION_TEST = f"{PLUGIN}/🧪️tests/🧩️composition/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
P1 = TICKET / "🧪️s5-nested-p1-fixture.py"

TYPES = '''    /// 🪜️ The owner edge that identifies one composed member inside a document (design §21.9): the artifact id of the
    /// MEMBER that owns it — empty when the document itself does — and its `(slot, child_id)` step under that owner. It is
    /// the identity a member's `.spr` stamp and archive entry persist (`store::OwnerRef`), so members of any depth share
    /// the flat registry and the flat content root without ever colliding on a step. Hosts, agents and history rows
    /// address the same member by its [`MemberPath`].
    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MemberKey {
        pub owner: String,
        pub slot: String,
        pub child_id: String,
    }

    impl MemberKey {
        /// 🌰️ A member the document itself owns.
        pub fn root(slot: impl Into<String>, child_id: impl Into<String>) -> Self {
            Self { owner: String::new(), slot: slot.into(), child_id: child_id.into() }
        }

        /// 🤲️ The borrowed form every lookup takes.
        pub fn borrowed(&self) -> MemberKeyRef<'_> {
            MemberKeyRef { owner: &self.owner, slot: &self.slot, child_id: &self.child_id }
        }
    }

    /// 🗝️ A [`MemberKey`] borrowed for one lookup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct MemberKeyRef<'a> {
        pub owner: &'a str,
        pub slot: &'a str,
        pub child_id: &'a str,
    }

    impl<'a> MemberKeyRef<'a> {
        /// 🌾️ A member the document itself owns.
        pub fn root(slot: &'a str, child_id: &'a str) -> Self {
            Self { owner: "", slot, child_id }
        }

        /// 🧳️ The owned key.
        pub fn to_key(self) -> MemberKey {
            MemberKey { owner: self.owner.to_string(), slot: self.slot.to_string(), child_id: self.child_id.to_string() }
        }
    }

    /// 👣️ One step of a [`MemberPath`]: the slot a member is declared in and its child id there.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MemberStep {
        pub slot: String,
        pub child_id: String,
    }

    /// 🛤️ The owner path of one composed member — every step from the document down to it — as hosts, agents and history
    /// rows address it (the `store` of a history row, a tool run's member, an inference dependency): `slot/childId` per
    /// step, steps joined by `/`, `%25` for `%` and `%2F` for `/` inside a component. A member of the document whose id
    /// holds neither reads exactly `<slot>/<childId>`.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MemberPath(Vec<MemberStep>);

    impl MemberPath {
        /// 🌲️ The path of a member the document itself owns.
        pub fn root(slot: impl Into<String>, child_id: impl Into<String>) -> Self {
            Self(vec![MemberStep { slot: slot.into(), child_id: child_id.into() }])
        }

        /// 🪵️ The path of the member this one owns at `(slot, child_id)`.
        pub fn join(mut self, slot: impl Into<String>, child_id: impl Into<String>) -> Self {
            self.0.push(MemberStep { slot: slot.into(), child_id: child_id.into() });
            self
        }

        /// 🥾️ Every step from the document down to the member.
        pub fn steps(&self) -> &[MemberStep] {
            &self.0
        }

        /// 📜️ Reads the canonical text; `None` for anything [`std::fmt::Display`] never writes.
        pub fn parse(text: &str) -> Option<Self> {
            let mut components = Vec::new();
            for component in text.split('/') {
                components.push(Self::unescape(component)?);
            }
            if components.len() % 2 != 0 {
                return None;
            }
            let mut steps = Vec::with_capacity(components.len() / 2);
            let mut components = components.into_iter();
            while let (Some(slot), Some(child_id)) = (components.next(), components.next()) {
                steps.push(MemberStep { slot, child_id });
            }
            Some(Self(steps))
        }

        fn unescape(component: &str) -> Option<String> {
            if component.is_empty() {
                return None;
            }
            let mut text = String::with_capacity(component.len());
            let mut rest = component;
            while let Some(position) = rest.find('%') {
                text.push_str(&rest[..position]);
                match rest.get(position..position + 3)? {
                    "%25" => text.push('%'),
                    "%2F" => text.push('/'),
                    _ => return None,
                }
                rest = &rest[position + 3..];
            }
            text.push_str(rest);
            Some(text)
        }
    }

    impl std::fmt::Display for MemberPath {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (index, component) in self.0.iter().flat_map(|step| [step.slot.as_str(), step.child_id.as_str()]).enumerate() {
                if index != 0 {
                    formatter.write_str("/")?;
                }
                for character in component.chars() {
                    match character {
                        '%' => formatter.write_str("%25")?,
                        '/' => formatter.write_str("%2F")?,
                        other => std::fmt::Write::write_char(formatter, other)?,
                    }
                }
            }
            Ok(())
        }
    }

'''

REGISTRY_LOOKUP_OLD_START = "        fn locate(&self, slot: &str, child_id: &str) -> Result<Result<usize, usize>, Fault> {\n            if !self.allocation_admitted {"
REGISTRY_LOOKUP_OLD_END = "        pub(crate) fn entries(&self) -> impl Iterator<Item = &ChildMemberEntry<M>> {"
REGISTRY_LOOKUP_NEW = '''        fn nested(&self, index: usize) -> bool {
            self.nested[index / 64] & (1 << (index % 64)) != 0
        }

        /// 🧷️ The owner edge of the member at one occupied slot.
        fn key_at(&self, index: usize) -> Option<MemberKeyRef<'_>> {
            let entry = self.entry(index)?;
            Some(MemberKeyRef { owner: if self.nested(index) { entry.owner.parent.artifact_id.as_str() } else { "" }, slot: &entry.owner.slot, child_id: &entry.owner.child_id })
        }

        fn locate(&self, key: MemberKeyRef<'_>) -> Result<Result<usize, usize>, Fault> {
            if !self.allocation_admitted {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
            }
            let start = Self::hash(key)?;
            for offset in 0..CHILD_CONTENT_SLOTS {
                let index = (start + offset) % CHILD_CONTENT_SLOTS;
                match self.key_at(index) {
                    Some(candidate) if candidate == key => return Ok(Ok(index)),
                    Some(_) => {}
                    None if self.reserved(index) => {}
                    None => return Ok(Err(index)),
                }
            }
            Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-capacity"), "fixed child-member registry is saturated"))
        }

        fn admit_identity(&mut self, key: MemberKeyRef<'_>) -> Result<ChildMemberAdmission, Fault> {
            if !self.allocation_admitted {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
            }
            match self.locate(key)? {
                Ok(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-duplicate"), "fixed child-member identity is already occupied")),
                Err(index) => {
                    let generation = self.next_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-generation"), "fixed child-member admission generation exhausted"))?;
                    self.next_generation = generation;
                    self.generations[index] = generation;
                    self.reserved[index / 64] |= 1 << (index % 64);
                    Ok(ChildMemberAdmission { index, generation, nested: !key.owner.is_empty() })
                }
            }
        }

        fn admit(&mut self, key: &(String, String)) -> Result<ChildMemberAdmission, Fault> {
            self.admit_identity(MemberKeyRef::root(&key.0, &key.1))
        }

        #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
        fn insert_admitted(&mut self, admission: ChildMemberAdmission, reference: ArtifactRef, owner: store::OwnerRef, member: M) {
            assert!(self.reserved(admission.index) && self.generations[admission.index] == admission.generation && !self.occupied(admission.index), "exact child-member admission changed while its exclusive app owner was suspended");
            self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
            self.slots[admission.index].write(ChildMemberEntry { reference, owner, member });
            self.occupied[admission.index / 64] |= 1 << (admission.index % 64);
            if admission.nested {
                self.nested[admission.index / 64] |= 1 << (admission.index % 64);
            }
            self.dense_slots[self.len] = admission.index;
            self.ordinal_by_slot[admission.index] = self.len;
            self.len += 1;
        }

        /// 🌻️ The member the document itself owns at one `(slot, child_id)` step.
        pub(crate) fn get(&self, key: &(String, String)) -> Option<&ChildMemberEntry<M>> {
            self.member(MemberKeyRef::root(&key.0, &key.1))
        }

        pub(crate) fn get_mut(&mut self, key: &(String, String)) -> Option<&mut ChildMemberEntry<M>> {
            self.member_mut(MemberKeyRef::root(&key.0, &key.1))
        }

        /// 🪆️ The member at one exact owner edge, whatever its depth.
        pub(crate) fn member(&self, key: MemberKeyRef<'_>) -> Option<&ChildMemberEntry<M>> {
            let Ok(Ok(index)) = self.locate(key) else { return None };
            self.entry(index)
        }

        pub(crate) fn member_mut(&mut self, key: MemberKeyRef<'_>) -> Option<&mut ChildMemberEntry<M>> {
            let Ok(Ok(index)) = self.locate(key) else { return None };
            self.entry_mut(index)
        }

        /// 🧾️ Every live member with its owner edge, in admission order.
        pub(crate) fn keyed_entries(&self) -> impl Iterator<Item = (MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
            (0..self.len).filter_map(|ordinal| self.keyed_entry_by_ordinal(ordinal))
        }

        fn keyed_entry_by_ordinal(&self, ordinal: usize) -> Option<(MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
            let index = *self.dense_slots[..self.len].get(ordinal)?;
            Some((self.key_at(index)?, self.entry(index)?))
        }

        /// 🧭️ The member an owner path names: one lookup per step, each under the member the step before resolved.
        pub(crate) fn resolve(&self, path: &MemberPath) -> Option<(MemberKey, &ChildMemberEntry<M>)> {
            let mut owner = "";
            let mut resolved = None;
            for step in path.steps() {
                let entry = self.member(MemberKeyRef { owner, slot: &step.slot, child_id: &step.child_id })?;
                resolved = Some((MemberKey { owner: owner.to_string(), slot: step.slot.clone(), child_id: step.child_id.clone() }, entry));
                owner = entry.reference.artifact_id.as_str();
            }
            resolved
        }

        /// 🗺️ The owner path of the member at `key`: its own step under the steps of every member that owns it; `None`
        /// while a member on the way up is not live.
        pub(crate) fn path_of(&self, key: MemberKeyRef<'_>) -> Option<MemberPath> {
            let mut steps = vec![MemberStep { slot: key.slot.to_string(), child_id: key.child_id.to_string() }];
            let mut owner = key.owner;
            while !owner.is_empty() {
                if steps.len() > self.len {
                    return None;
                }
                let (above, _) = self.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == owner)?;
                steps.push(MemberStep { slot: above.slot.to_string(), child_id: above.child_id.to_string() });
                owner = above.owner;
            }
            steps.reverse();
            Some(MemberPath(steps))
        }

        /// 🏷️ Every live member with the text of its owner path — the store id the history wire names it by.
        pub(crate) fn addressed_entries(&self) -> impl Iterator<Item = (String, &ChildMemberEntry<M>)> {
            self.keyed_entries().filter_map(|(key, entry)| Some((self.path_of(key)?.to_string(), entry)))
        }

'''

REGISTRY_REMOVE_OLD = '''        fn remove(&mut self, slot: &str, child_id: &str) -> Result<Option<ChildMemberEntry<M>>, Fault> {
            if self.has_admission_in_flight() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-removal-admission"), "a child member cannot leave the fixed registry while an admission is in flight"));
            }
            let Ok(mut hole) = self.locate(slot, child_id)? else { return Ok(None) };
            let entry = self.take_at(hole).expect("located child member remains occupied");
            let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
            while let Some(moved) = self.entry(cursor) {
                let home = Self::hash(&moved.owner.slot, &moved.owner.child_id)?;
'''
REGISTRY_REMOVE_NEW = '''        fn remove(&mut self, key: MemberKeyRef<'_>) -> Result<Option<ChildMemberEntry<M>>, Fault> {
            if self.has_admission_in_flight() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-removal-admission"), "a child member cannot leave the fixed registry while an admission is in flight"));
            }
            let Ok(mut hole) = self.locate(key)? else { return Ok(None) };
            let entry = self.take_at(hole).expect("located child member remains occupied");
            let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
            while let Some(moved) = self.key_at(cursor) {
                let home = Self::hash(moved)?;
'''

CONTENT_LOOKUP_OLD_START = "        fn hash(slot: &str, child_id: &str) -> Result<usize, Fault> {\n            if slot.len() > CHILD_CONTENT_ID_BYTES"
CONTENT_LOOKUP_OLD_END = "        fn insert_entry_admitted(&self, index: usize, entry: ChildContentEntry) -> Self {"
CONTENT_LOOKUP_NEW = '''        fn hash(key: MemberKeyRef<'_>) -> Result<usize, Fault> {
            if key.owner.len() > CHILD_CONTENT_ID_BYTES || key.slot.len() > CHILD_CONTENT_ID_BYTES || key.child_id.len() > CHILD_CONTENT_ID_BYTES {
                return Err(plugin_sdk_fault("child slot or id exceeds the fixed immutable-root identity bound"));
            }
            let mut hash = 0xcbf29ce484222325u64;
            let owner = key.owner.bytes().chain(std::iter::once(0xfe)).filter(|_| !key.owner.is_empty());
            for byte in owner.chain(key.slot.bytes()).chain(std::iter::once(0xff)).chain(key.child_id.bytes()) {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
            Ok(hash as usize % CHILD_CONTENT_SLOTS)
        }

        fn entry_at(root: &ChildContentRoot, index: usize) -> Option<&std::sync::Arc<ChildContentEntry>> {
            root.pages[index / CHILD_CONTENT_PAGE_SLOTS].as_ref()?.entries[index % CHILD_CONTENT_PAGE_SLOTS].as_ref()
        }

        fn locate(root: &ChildContentRoot, key: MemberKeyRef<'_>) -> Result<Result<usize, usize>, Fault> {
            let start = Self::hash(key)?;
            for offset in 0..CHILD_CONTENT_SLOTS {
                let index = (start + offset) % CHILD_CONTENT_SLOTS;
                match Self::entry_at(root, index) {
                    Some(entry) if entry.key() == key => return Ok(Ok(index)),
                    Some(_) => {}
                    None => return Ok(Err(index)),
                }
            }
            Err(plugin_sdk_fault("fixed immutable child-content root is saturated"))
        }

        fn admit_member(&self, key: MemberKeyRef<'_>) -> Result<usize, Fault> {
            match self.root.as_deref() {
                Some(root) => Self::locate(root, key).map(|location| match location {
                    Ok(index) | Err(index) => index,
                }),
                None => Self::hash(key),
            }
        }

'''

CONTENT_READS_OLD_START = "        /// 🪞️ The same immutable root with one child's `snapshot` read in its place — a history edit's preview of a composed\n        /// member (design §12), never the published root.\n        pub(crate) fn with_member_read("
CONTENT_READS_OLD_END = "        /// 🈳️ Whether this document has any live children at all."
CONTENT_READS_NEW = '''        /// 🪞️ The same immutable root with one member's `snapshot` read in its place — a history edit's preview of a composed
        /// member (design §12), never the published root.
        pub(crate) fn with_member_read(&self, key: MemberKeyRef<'_>, dialect: &ArtifactDialect, revision: [u8; 32], snapshot: store::ErasedSnapshotRead) -> Result<Self, Fault> {
            let artifact_id = self.find(key)?.map(|entry| entry.artifact_id.clone()).ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {} child {}", key.slot, key.child_id)))?;
            let index = self.admit_member(key)?;
            Ok(self.insert_entry_admitted(index, ChildContentEntry { owner: key.owner.to_string(), slot: key.slot.to_string(), child_id: key.child_id.to_string(), artifact_id, dialect: dialect.clone(), revision, snapshot }))
        }

        async fn with_member<M: SpaceMember>(&self, key: MemberKeyRef<'_>, reference: &ArtifactRef, member: &M) -> Result<Self, Fault> {
            let index = self.admit_member(key)?;
            self.capture_member_admitted(index, key, reference, member).await
        }

        async fn capture_member_admitted<M: SpaceMember>(&self, index: usize, key: MemberKeyRef<'_>, reference: &ArtifactRef, member: &M) -> Result<Self, Fault> {
            let revision = member.content_revision().await;
            let snapshot = member.snapshot_read_erased().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
            Ok(self.insert_entry_admitted(index, ChildContentEntry { owner: key.owner.to_string(), slot: key.slot.to_string(), child_id: key.child_id.to_string(), artifact_id: reference.artifact_id.clone(), dialect: reference.dialect.clone(), revision, snapshot }))
        }

        /// 🧬️ Fixed-size, restart-stable identity paired with the captured child snapshot.
        pub fn revision(&self, slot: &str, child_id: &str) -> Result<[u8; 32], Fault> {
            self.find(MemberKeyRef::root(slot, child_id))?.map(|entry| entry.revision).ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {slot} child {child_id}")))
        }

        /// 🧵️ Selects an immutable child snapshot capability without exposing concrete ownership.
        pub fn typed_read<S: ArtifactPack + Send + Sync + 'static>(&self, slot: &str, child_id: &str) -> Result<store::SnapshotReadRef<'_, S>, Fault> {
            self.typed_read_at(MemberKeyRef::root(slot, child_id))
        }

        /// 🪡️ [`Self::typed_read`] of the member at one exact owner edge, whatever its depth.
        pub fn typed_read_at<S: ArtifactPack + Send + Sync + 'static>(&self, key: MemberKeyRef<'_>) -> Result<store::SnapshotReadRef<'_, S>, Fault> {
            let (slot, child_id) = (key.slot, key.child_id);
            let snapshot = &self.find(key)?.ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {slot} child {child_id}")))?.snapshot;
            snapshot.typed::<S>().ok_or_else(|| plugin_sdk_fault(format!("child snapshot type mismatch for slot {slot} child {child_id}")))
        }

        /// 🎯️ The dialect a child materializes as, for a caller that must route by kind.
        pub fn dialect(&self, slot: &str, child_id: &str) -> Option<ArtifactDialect> {
            self.dialect_at(MemberKeyRef::root(slot, child_id))
        }

        /// 🎳️ [`Self::dialect`] of the member at one exact owner edge, whatever its depth.
        pub fn dialect_at(&self, key: MemberKeyRef<'_>) -> Option<ArtifactDialect> {
            self.find(key).ok().flatten().map(|entry| entry.dialect.clone())
        }

        /// 📋️ Every `(slot, child_id)` the document itself owns and holds live.
        pub fn slots(&self) -> Vec<(String, String)> {
            let Some(root) = self.root.as_deref() else { return Vec::new() };
            let mut slots = Vec::with_capacity(root.len);
            for index in 0..CHILD_CONTENT_SLOTS {
                if let Some(entry) = Self::entry_at(root, index).filter(|entry| entry.owner.is_empty()) {
                    slots.push((entry.slot.clone(), entry.child_id.clone()));
                }
            }
            slots
        }

        /// 🗂️ The owner edge of every member held live, whatever its depth.
        pub fn keys(&self) -> Vec<MemberKey> {
            let Some(root) = self.root.as_deref() else { return Vec::new() };
            (0..CHILD_CONTENT_SLOTS).filter_map(|index| Self::entry_at(root, index)).map(|entry| entry.key().to_key()).collect()
        }

'''

RETIREMENT_LOOKUP_OLD_START = "        /// 🍂️ Whether one exact child identity is still retiring (its member has not closed yet).\n        fn retains_member("
RETIREMENT_LOOKUP_OLD_END = "    impl ArtifactFixedRegistry<ChildContentRetirement> {"
RETIREMENT_LOOKUP_NEW = '''        /// 🍂️ Whether the member with one exact artifact id is still retiring (it has not closed yet).
        fn retains_member(&self, artifact_id: &str) -> bool {
            (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| self.entry(index).is_some_and(|(_, retirement)| retirement.retires(artifact_id)))
        }

        /// 🍂️ The retiring member with one exact artifact id — the disposer of the snapshot leases retired roots still hold.
        fn retiring_member_mut(&mut self, artifact_id: &str) -> Option<&mut M> {
            let id = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| self.entry(index).and_then(|(id, retirement)| retirement.retires(artifact_id).then_some(*id)))?;
            self.get_mut(id)?.entry.as_mut().map(|entry| &mut entry.member)
        }
    }

    /// 🪪️ The member that disposes one child snapshot lease: the live member at its owner edge, else the retiring member
    /// of that artifact id, which its coordinate left while a retired root still lent its snapshot.
    fn child_snapshot_owner<'a, M>(children: &'a mut ChildMemberRegistry<M>, retiring: Option<&'a mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>, key: MemberKeyRef<'_>, artifact_id: &str) -> Option<&'a mut M> {
        if let Some(entry) = children.member_mut(key) {
            return Some(&mut entry.member);
        }
        retiring?.retiring_member_mut(artifact_id)
    }

'''

PUBLISH_OLD_START = "        pub(crate) async fn publish_child_content_member(&mut self, generation: u64, slot: &str, child_id: &str) -> Result<(), Fault> {"
PUBLISH_OLD_END = "        fn peer_roster_slot(generation: u64) -> usize {"
PUBLISH_NEW = '''        pub(crate) async fn publish_child_content_member(&mut self, generation: u64, slot: &str, child_id: &str) -> Result<(), Fault> {
            self.publish_member_content(generation, MemberKeyRef::root(slot, child_id)).await
        }

        /// 📰️ Republishes the immutable child-content root with the member at one exact owner edge captured anew.
        pub(crate) async fn publish_member_content(&mut self, generation: u64, key: MemberKeyRef<'_>) -> Result<(), Fault> {
            if generation != self.child_content_generation.saturating_add(1) || !self.child_content_retirements.can_insert(generation) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-publication-authority"), "immutable child-content publication lost its exact admitted generation"));
            }
            let entry = self.children.member(key).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-member"), "immutable child-content publication lost its exact live member"))?;
            let next = self.child_content_root.with_member(key, &entry.reference, &entry.member).await?;
            let previous = std::mem::replace(&mut *self.child_content_root, next);
            if previous.root.as_ref().is_some_and(|root| root.len != 0) {
                self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, false));
            }
            self.child_content_generation = generation;
            Ok(())
        }

        /// 📮️ Sends one member's events on the document's backbone under that member's lane. The lane of a member another
        /// member owns has no address on the wire before channel 23 (design §21.9, wave N2): its events are refused here,
        /// never sent under a root member's lane and never dropped.
        async fn send_member_lane(&mut self, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
            if payload.is_empty() || self.store.backbone_ref().is_none() {
                return Ok(());
            }
            if !lane.owner.is_empty() {
                return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
            }
            self.store.send_member_mutations(&lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
        }

'''

ANNOUNCE_OLD = '''            let mut lanes: Vec<(String, String)> = Vec::with_capacity(self.children.len());
            for entry in self.children.entries() {
                lanes.push((entry.owner.slot.clone(), entry.owner.child_id.clone()));
            }
            for (slot, child_id) in lanes {
                let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else { continue };
                let payload = store::SpaceMember::event_log_payload(&entry.member).await.map_err(|error| error.into_fault())?;
                self.store.send_member_mutations(&slot, &child_id, payload).await.map_err(|error| error.into_fault())?;
            }
'''
ANNOUNCE_NEW = '''            let lanes: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
            for lane in lanes {
                let Some(entry) = self.children.member_mut(lane.borrowed()) else { continue };
                let payload = store::SpaceMember::event_log_payload(&entry.member).await.map_err(|error| error.into_fault())?;
                self.send_member_lane(&lane, payload).await?;
            }
'''

CHECKPOINT_UNIT_OLD = '''        async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], slot: &str, child_id: &str) -> Result<Option<vcs::CompositionPin>, Fault> {
            let publication_generation = self.admit_child_content_publication()?;
            let (reference, checkpoint_id) = {
                let entry = self.children.get_mut(&(slot.to_string(), child_id.to_string())).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
'''
CHECKPOINT_UNIT_NEW = '''        async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], key: MemberKeyRef<'_>) -> Result<Option<vcs::CompositionPin>, Fault> {
            let publication_generation = self.admit_child_content_publication()?;
            let (reference, checkpoint_id) = {
                let entry = self.children.member_mut(key).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
'''

CHECKOUT_UNIT_OLD = '''            let child_key = self.children.entries().find(|entry| entry.reference.artifact_id == pin.child_ref.artifact_id).map(|entry| (entry.owner.slot.clone(), entry.owner.child_id.clone()));
            let Some((slot, child_id)) = child_key else {
                self.pending_child_pins.push(pin.clone());
                return Ok(());
            };
            let publication_generation = self.admit_child_content_publication()?;
            let entry = self.children.get_mut(&(slot.clone(), child_id.clone())).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
            let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
            let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id).await;
            self.publish_child_content_member(publication_generation, &slot, &child_id).await
'''
CHECKOUT_UNIT_NEW = '''            let child_key = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == pin.child_ref.artifact_id).map(|(key, _)| key.to_key());
            let Some(key) = child_key else {
                self.pending_child_pins.push(pin.clone());
                return Ok(());
            };
            let publication_generation = self.admit_child_content_publication()?;
            let entry = self.children.member_mut(key.borrowed()).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
            let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
            let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id).await;
            self.publish_member_content(publication_generation, key.borrowed()).await
'''

GROUP_HISTORY_OLD = '''                let Some((slot, child_id)) = self.children.entries().find(|entry| entry.reference.artifact_id == reference.artifact_id).map(|entry| (entry.owner.slot.clone(), entry.owner.child_id.clone())) else {
                    return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
                };
                let publication_generation = self.admit_child_content_publication()?;
                self.publish_child_content_member(publication_generation, &slot, &child_id).await?;
'''
GROUP_HISTORY_NEW = '''                let Some(key) = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                    return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
                };
                let publication_generation = self.admit_child_content_publication()?;
                self.publish_member_content(publication_generation, key.borrowed()).await?;
'''

RETIRE_PENDING_OLD = '''            let Some(member) = child_snapshot_owner(children, retiring, &entry.slot, &entry.child_id) else {
                *self.pending = Some(entry);
                return Ok(PluginCloseStep::Blocked { reason: "retired child snapshot has no exact live member disposer owner" });
            };
            let ChildContentEntry { slot, child_id, dialect, revision, snapshot } = entry;
            match member.retire_snapshot_read_erased(snapshot) {
                Ok(owner) => {
                    *self.active_member = Some((slot, child_id));
                    drop(dialect);
                    *self.active = Some(owner);
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(rejected) => {
                    *self.pending = Some(ChildContentEntry { slot, child_id, dialect, revision, snapshot: rejected.snapshot });
'''
RETIRE_PENDING_NEW = '''            let Some(member) = child_snapshot_owner(children, retiring, entry.key(), &entry.artifact_id) else {
                *self.pending = Some(entry);
                return Ok(PluginCloseStep::Blocked { reason: "retired child snapshot has no exact live member disposer owner" });
            };
            let ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot } = entry;
            match member.retire_snapshot_read_erased(snapshot) {
                Ok(owner) => {
                    *self.active_member = Some((MemberKey { owner: member_owner, slot, child_id }, artifact_id));
                    drop(dialect);
                    *self.active = Some(owner);
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
                Err(rejected) => {
                    *self.pending = Some(ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot: rejected.snapshot });
'''

FOLLOW_HELD_OLD = '''                for entry in self.children.entries() {
                    let key = (entry.owner.slot.clone(), entry.owner.child_id.clone());
                    if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
'''
FOLLOW_HELD_NEW = '''                for (member, _) in self.children.keyed_entries().filter(|(member, _)| member.owner.is_empty()) {
                    let key = (member.slot.to_string(), member.child_id.to_string());
                    if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
'''

CANDIDATE_VIEW_ENTRY_OLD = "ChildContentEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.clone(), revision, snapshot }));"
CANDIDATE_VIEW_ENTRY_NEW = "ChildContentEntry { owner: key.owner.to_string(), slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), artifact_id: entry.reference.artifact_id.clone(), dialect: entry.reference.dialect.clone(), revision, snapshot }));"

RUNTIME_REPLACEMENTS = [
    ("    pub(crate) struct ChildMemberEntry<M> {\n", TYPES + "    pub(crate) struct ChildMemberEntry<M> {\n", 1),
    ("    struct ChildMemberAdmission {\n        index: usize,\n        generation: u64,\n    }\n", "    struct ChildMemberAdmission {\n        index: usize,\n        generation: u64,\n        nested: bool,\n    }\n", 1),
    ("        reserved: [u64; CHILD_CONTENT_SLOTS / 64],\n        generations: [u64; CHILD_CONTENT_SLOTS],\n", "        reserved: [u64; CHILD_CONTENT_SLOTS / 64],\n        nested: [u64; CHILD_CONTENT_SLOTS / 64],\n        generations: [u64; CHILD_CONTENT_SLOTS],\n", 1),
    ("                reserved: [0; CHILD_CONTENT_SLOTS / 64],\n                generations: [0; CHILD_CONTENT_SLOTS],\n", "                reserved: [0; CHILD_CONTENT_SLOTS / 64],\n                nested: [0; CHILD_CONTENT_SLOTS / 64],\n                generations: [0; CHILD_CONTENT_SLOTS],\n", 1),
    ("        fn hash(slot: &str, child_id: &str) -> Result<usize, Fault> {\n            ChildContentView::hash(slot, child_id)\n        }\n", "        fn hash(key: MemberKeyRef<'_>) -> Result<usize, Fault> {\n            ChildContentView::hash(key)\n        }\n", 1),
    (REGISTRY_REMOVE_OLD, REGISTRY_REMOVE_NEW, 1),
    ("            self.occupied[index / 64] &= !(1 << (index % 64));\n            self.len -= 1;\n            let ordinal = self.ordinal_by_slot[index];\n", "            self.occupied[index / 64] &= !(1 << (index % 64));\n            self.nested[index / 64] &= !(1 << (index % 64));\n            self.len -= 1;\n            let ordinal = self.ordinal_by_slot[index];\n", 1),
    ("            self.occupied[to / 64] |= 1 << (to % 64);\n            self.generations[to] = self.generations[from];\n", "            self.occupied[to / 64] |= 1 << (to % 64);\n            if self.nested(from) {\n                self.nested[to / 64] |= 1 << (to % 64);\n            } else {\n                self.nested[to / 64] &= !(1 << (to % 64));\n            }\n            self.nested[from / 64] &= !(1 << (from % 64));\n            self.generations[to] = self.generations[from];\n", 1),
    (
        "    pub(crate) struct ChildContentEntry {\n        slot: String,\n        child_id: String,\n        dialect: ArtifactDialect,\n        revision: [u8; 32],\n        pub(crate) snapshot: store::ErasedSnapshotRead,\n    }\n",
        "    pub(crate) struct ChildContentEntry {\n        owner: String,\n        slot: String,\n        child_id: String,\n        artifact_id: String,\n        dialect: ArtifactDialect,\n        revision: [u8; 32],\n        pub(crate) snapshot: store::ErasedSnapshotRead,\n    }\n\n    impl ChildContentEntry {\n        fn key(&self) -> MemberKeyRef<'_> {\n            MemberKeyRef { owner: &self.owner, slot: &self.slot, child_id: &self.child_id }\n        }\n    }\n",
        1,
    ),
    ("        fn find(&self, slot: &str, child_id: &str) -> Result<Option<&ChildContentEntry>, Fault> {\n            let Some(root) = self.root.as_deref() else { return Ok(None) };\n            match Self::locate(root, slot, child_id)? {\n", "        fn find(&self, key: MemberKeyRef<'_>) -> Result<Option<&ChildContentEntry>, Fault> {\n            let Some(root) = self.root.as_deref() else { return Ok(None) };\n            match Self::locate(root, key)? {\n", 1),
    (
        "        fn without_member(&self, slot: &str, child_id: &str) -> Result<Self, Fault> {\n            let Some(current) = self.root.as_deref() else { return Ok(self.clone()) };\n            let Ok(mut hole) = Self::locate(current, slot, child_id)? else { return Ok(self.clone()) };\n",
        "        fn without_member(&self, key: MemberKeyRef<'_>) -> Result<Self, Fault> {\n            let Some(current) = self.root.as_deref() else { return Ok(self.clone()) };\n            let Ok(mut hole) = Self::locate(current, key)? else { return Ok(self.clone()) };\n",
        1,
    ),
    ("                let home = Self::hash(&entry.slot, &entry.child_id)?;\n", "                let home = Self::hash(entry.key())?;\n", 1),
    (
        "                for byte in entry\n                    .slot\n                    .bytes()\n                    .chain(std::iter::once(0xff))\n",
        "                for byte in entry\n                    .owner\n                    .bytes()\n                    .chain(std::iter::once(0xfb))\n                    .filter(|_| !entry.owner.is_empty())\n                    .chain(entry.slot.bytes())\n                    .chain(std::iter::once(0xff))\n",
        1,
    ),
    ("            self.find(&entry.slot, &entry.child_id).ok().flatten().is_some_and(|candidate| std::ptr::eq(candidate, entry.as_ref()))\n", "            self.find(entry.key()).ok().flatten().is_some_and(|candidate| std::ptr::eq(candidate, entry.as_ref()))\n", 1),
    ("        active_member: std::mem::ManuallyDrop<Option<(String, String)>>,\n", "        active_member: std::mem::ManuallyDrop<Option<(MemberKey, String)>>,\n", 1),
    (
        "            if let Some((slot, child_id)) = self.active_member.as_ref() {\n                let Some(member) = child_snapshot_owner(children, retiring, slot, child_id) else {\n",
        "            if let Some((key, artifact_id)) = self.active_member.as_ref() {\n                let Some(member) = child_snapshot_owner(children, retiring, key.borrowed(), artifact_id) else {\n",
        1,
    ),
    (RETIRE_PENDING_OLD, RETIRE_PENDING_NEW, 1),
    ("    struct ChildStoreAdmission {\n        key: (String, String),\n", "    struct ChildStoreAdmission {\n        key: MemberKey,\n", 1),
    (
        "        fn retires(&self, slot: &str, child_id: &str) -> bool {\n            self.entry.as_ref().is_some_and(|entry| entry.owner.slot == slot && entry.owner.child_id == child_id)\n        }\n",
        "        fn retires(&self, artifact_id: &str) -> bool {\n            self.entry.as_ref().is_some_and(|entry| entry.reference.artifact_id == artifact_id)\n        }\n",
        1,
    ),
    ("        CheckpointChildren { message: String, authors: Vec<vcs::Author>, keys: Vec<(String, String)>, next: usize, pins: Vec<vcs::CompositionPin> },\n", "        CheckpointChildren { message: String, authors: Vec<vcs::Author>, keys: Vec<MemberKey>, next: usize, pins: Vec<vcs::CompositionPin> },\n", 1),
    (
        "                        let admission = match candidate.admit_identity(&owner.slot, &owner.child_id) {\n",
        "                        let owner_key = if self.retained_store.as_ref().is_some_and(|parent| parent.envelope().id == owner.parent.artifact_id) { \"\" } else { owner.parent.artifact_id.as_str() };\n                        let admission = match candidate.admit_identity(MemberKeyRef { owner: owner_key, slot: &owner.slot, child_id: &owner.child_id }) {\n",
        1,
    ),
    ("            let Some(entry) = children.entry_by_ordinal(self.view_member_ordinal) else {\n", "            let Some((key, entry)) = children.keyed_entry_by_ordinal(self.view_member_ordinal) else {\n", 1),
    ("            let index = match content.admit_member(&entry.owner.slot, &entry.owner.child_id) {\n", "            let index = match content.admit_member(key) {\n", 1),
    (CANDIDATE_VIEW_ENTRY_OLD, CANDIDATE_VIEW_ENTRY_NEW, 1),
    ("                        if self.child_member_retirements.retains_member(slot, fields.child_id) {\n", "                        if self.child_member_retirements.retains_member(fields.artifact_id) {\n", 1),
    (FOLLOW_HELD_OLD, FOLLOW_HELD_NEW, 1),
    (
        "            let next = self.child_content_root.without_member(slot, child_id)?;\n            let entry = self.children.remove(slot, child_id)?.ok_or_else(",
        "            let next = self.child_content_root.without_member(MemberKeyRef::root(slot, child_id))?;\n            let entry = self.children.remove(MemberKeyRef::root(slot, child_id))?.ok_or_else(",
        1,
    ),
    ("            let root_index = self.child_content_root.admit_member(&slot, &child_id)?;\n", "            let root_index = self.child_content_root.admit_member(MemberKeyRef::root(&slot, &child_id))?;\n", 1),
    ("            let key = (slot, child_id);\n            let member = self.children.admit(&key)?;\n", "            let key = MemberKey::root(slot, child_id);\n            let member = self.children.admit_identity(key.borrowed())?;\n", 1),
    (
        "self.child_content_root.capture_member_admitted(admission.root_index, &admission.key.0, &admission.key.1, &admission.expected.dialect, member).await?;",
        "self.child_content_root.capture_member_admitted(admission.root_index, admission.key.borrowed(), &admission.expected, member).await?;",
        1,
    ),
    ("match self.validate_parent_child_restore(&admission.key.0, &admission.key.1, &admission.expected) {", "match self.validate_parent_child_restore(&admission.key.slot, &admission.key.child_id, &admission.expected) {", 1),
    (ANNOUNCE_OLD, ANNOUNCE_NEW, 1),
    (CHECKPOINT_UNIT_OLD, CHECKPOINT_UNIT_NEW, 1),
    ("            self.publish_child_content_member(publication_generation, slot, child_id).await?;\n            Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: reference, checkpoint_id }))\n", "            self.publish_member_content(publication_generation, key).await?;\n            Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: reference, checkpoint_id }))\n", 1),
    (CHECKOUT_UNIT_OLD, CHECKOUT_UNIT_NEW, 1),
    (GROUP_HISTORY_OLD, GROUP_HISTORY_NEW, 2),
    (
        "                        Some(entry) => next.with_member(&child_emit.slot, &child_emit.child_id, &entry.reference.dialect, &entry.member).await,\n",
        "                        Some(entry) => next.with_member(MemberKeyRef::root(&child_emit.slot, &child_emit.child_id), &entry.reference, &entry.member).await,\n",
        1,
    ),
    (
        "                        let keys: Vec<(String, String)> = self.children.entries().map(|entry| (entry.owner.slot.clone(), entry.owner.child_id.clone())).collect();\n",
        "                        let keys: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();\n",
        1,
    ),
    (
        "                    if let Some((slot, child_id)) = keys.get(next) {\n                        if let Some(pin) = self.checkpoint_child_unit(&message, &authors, slot, child_id).await? {\n",
        "                    if let Some(key) = keys.get(next) {\n                        if let Some(pin) = self.checkpoint_child_unit(&message, &authors, key.borrowed()).await? {\n",
        1,
    ),
]
RUNTIME_REGIONS = [
    (REGISTRY_LOOKUP_OLD_START, REGISTRY_LOOKUP_OLD_END, REGISTRY_LOOKUP_NEW),
    (CONTENT_LOOKUP_OLD_START, CONTENT_LOOKUP_OLD_END, CONTENT_LOOKUP_NEW),
    (CONTENT_READS_OLD_START, CONTENT_READS_OLD_END, CONTENT_READS_NEW),
    (RETIREMENT_LOOKUP_OLD_START, RETIREMENT_LOOKUP_OLD_END, RETIREMENT_LOOKUP_NEW),
    (PUBLISH_OLD_START, PUBLISH_OLD_END, PUBLISH_NEW),
]

TIME_TRAVEL_REPLACEMENTS = [
    (
        "/// 🧩️ The composed member store a session edits (design §12): its slot, child id and dialect, the typed owners of the\n",
        "/// 🧩️ The composed member store a session edits (design §12): its owner edge, owner path and dialect, the typed owners of the\n",
        1,
    ),
    ("pub(crate) struct TimeTravelMemberSubject {\n    pub slot: String,\n    pub child_id: String,\n", "pub(crate) struct TimeTravelMemberSubject {\n    pub key: MemberKey,\n    pub path: MemberPath,\n", 1),
    (
        "    /// 🧩️ The member store id the history wire names it by: `<slot>/<childId>`.\n    pub(crate) fn store(&self) -> String {\n        format!(\"{}/{}\", self.slot, self.child_id)\n    }\n",
        "    /// 🧩️ The member store id the history wire names it by: the text of its owner path (`<slot>/<childId>` for a member of the document).\n    pub(crate) fn store(&self) -> String {\n        self.path.to_string()\n    }\n",
        1,
    ),
    ("        for entry in self.children.entries() {\n            let store = format!(\"{}/{}\", entry.owner.slot, entry.owner.child_id);\n", "        for (store, entry) in self.children.addressed_entries() {\n", 1),
    (
        "            .entries()\n            .map(|entry| {\n                let store = format!(\"{}/{}\", entry.owner.slot, entry.owner.child_id);\n",
        "            .addressed_entries()\n            .map(|(store, entry)| {\n",
        2,
    ),
    (
        "                    for member in self.children.entries() {\n                        let store = format!(\"{}/{}\", member.owner.slot, member.owner.child_id);\n",
        "                    for (store, member) in self.children.addressed_entries() {\n",
        1,
    ),
    ("                let entry = self.children.get(&(member.slot.clone(), member.child_id.clone()))?;\n", "                let entry = self.children.member(member.key.borrowed())?;\n", 1),
    ("                let key = (member.slot.clone(), member.child_id.clone());\n                let entry = children.get_mut(&key).ok_or_else(", "                let key = member.key.clone();\n                let entry = children.member_mut(key.borrowed()).ok_or_else(", 1),
    (
        "                let key = (member.slot.clone(), member.child_id.clone());\n                if let (Some(owners), Some(entry)) = (member.owners.as_deref_mut(), children.get(&key)) {\n",
        "                let key = member.key.clone();\n                if let (Some(owners), Some(entry)) = (member.owners.as_deref_mut(), children.member(key.borrowed())) {\n",
        1,
    ),
    ("        let (slot, child_id, dialect) = (member.slot.clone(), member.child_id.clone(), member.dialect.clone());\n", "        let (key, dialect) = (member.key.clone(), member.dialect.clone());\n", 1),
    ("Some(self.child_content_root.with_member_read(&slot, &child_id, &dialect, revision, read)?),", "Some(self.child_content_root.with_member_read(key.borrowed(), &dialect, revision, read)?),", 1),
    (
        "        let (slot, child_id) = (member.slot.clone(), member.child_id.clone());\n        let label = TimeTravelLabel::MemberEdited.localized(LocalizedLabel::native);\n",
        "        let key = member.key.clone();\n        let label = TimeTravelLabel::MemberEdited.localized(LocalizedLabel::native);\n",
        1,
    ),
    (
        "        self.publish_child_content_member(generation, &slot, &child_id).await?;\n        self.store.send_member_mutations(&slot, &child_id, authored).await.map_err(|error| error.into_fault())\n",
        "        self.publish_member_content(generation, key.borrowed()).await?;\n        self.send_member_lane(&key, authored).await\n",
        1,
    ),
    (
        "                    let Some(entry) = store.split_once('/').and_then(|(slot, child_id)| self.children.get(&(slot.to_string(), child_id.to_string()))) else {\n",
        "                    let Some((path, key, entry)) = MemberPath::parse(store).and_then(|path| self.children.resolve(&path).map(|(key, entry)| (path, key, entry))) else {\n",
        1,
    ),
    (
        "                    Some(TimeTravelMemberSubject { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.clone(), owners: None, children: None })\n",
        "                    Some(TimeTravelMemberSubject { key, path, dialect: entry.reference.dialect.clone(), owners: None, children: None })\n",
        1,
    ),
]

TOOL_RUN_REPLACEMENTS = [
    ("        let mut candidates = self.children.entries().filter(|entry| entry.owner.slot == slot);\n", "        let mut candidates = self.children.keyed_entries().filter(|(key, _)| key.owner.is_empty() && key.slot == slot).map(|(_, entry)| entry);\n", 1),
    (
        "Some(self.child_content_root.with_member_read(&member.slot, &member.child_id, &member.dialect, revision, read)?),",
        "Some(self.child_content_root.with_member_read(MemberKeyRef::root(&member.slot, &member.child_id), &member.dialect, revision, read)?),",
        1,
    ),
    ("children.entries().find(|child|child.owner.slot==member.slot&&child.owner.child_id==member.child_id)", "children.get(&(member.slot.clone(),member.child_id.clone()))", 2),
]

REGISTRY_TEST_REPLACEMENTS = [
    ("ChildMemberRegistry::<usize>::hash(&key.0, &key.1)", "ChildMemberRegistry::<usize>::hash(MemberKeyRef::root(&key.0, &key.1))", 1),
]

COMPOSITION_TEST_REPLACEMENTS = [
    ("            assert_eq!(app.child_content_root.slots().len(), 2);\n", "            assert_eq!(app.child_content_root.keys().len(), 2);\n            assert_eq!(app.child_content_root.slots(), vec![(\"slot\".into(), \"child-1\".into())], \"the document itself owns the branch alone\");\n", 1),
    (
        'app.child_content_root.typed_read::<RecursiveBranchSnapshot>("nested", "grandchild-1")',
        'app.child_content_root.typed_read_at::<RecursiveBranchSnapshot>(MemberKeyRef { owner: "child-1", slot: "nested", child_id: "grandchild-1" })',
        2,
    ),
    (
        'assert!(app.child_content_root.dialect("nested", "grandchild-1").is_none());',
        'assert!(app.child_content_root.dialect_at(MemberKeyRef { owner: "child-1", slot: "nested", child_id: "grandchild-1" }).is_none());',
        1,
    ),
]

STORE_REPLACEMENTS = [
    (
        "            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }\n            }\n",
        "            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }\n            }\n            async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<$crate::os_store::PairMerge, $crate::os_store::VcsError> {\n                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack).await),+ }\n            }\n",
        1,
    ),
]

FILES = [
    (RUNTIME, RUNTIME_REPLACEMENTS, RUNTIME_REGIONS, "pub struct MemberKeyRef<'a> {"),
    (TIME_TRAVEL, TIME_TRAVEL_REPLACEMENTS, [], "pub key: MemberKey,"),
    (TOOL_RUN, TOOL_RUN_REPLACEMENTS, [], "MemberKeyRef::root(&member.slot, &member.child_id)"),
    (REGISTRY_TEST, REGISTRY_TEST_REPLACEMENTS, [], "hash(MemberKeyRef::root("),
    (COMPOSITION_TEST, COMPOSITION_TEST_REPLACEMENTS, [], "typed_read_at::<RecursiveBranchSnapshot>"),
    (STORE, STORE_REPLACEMENTS, [], "SpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack)"),
]


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
        subprocess.run([sys.executable, str(P1), "restore"], check=True)
        return
    results = []
    for relative, replacements, regions, marker in FILES:
        text = (REPO / relative).read_text(encoding="utf-8")
        if marker in text:
            print(f"already landed: {relative}")
            continue
        results.append((relative, text, planned(relative, text, replacements, regions)))
        print(f"{relative}: {sum(count for _, _, count in replacements)} replacements, {len(regions)} regions; {len(text)} -> {len(results[-1][2])} bytes")
    subprocess.run([sys.executable, str(P1), "check"], check=True)
    if mode == "land":
        PRE.mkdir(parents=True, exist_ok=True)
        for relative, text, result in results:
            pre_image(relative).write_text(text, encoding="utf-8")
            (REPO / relative).write_text(result, encoding="utf-8")
            print(f"landed {relative}")
        subprocess.run([sys.executable, str(P1), "land"], check=True)


if __name__ == "__main__":
    main()
