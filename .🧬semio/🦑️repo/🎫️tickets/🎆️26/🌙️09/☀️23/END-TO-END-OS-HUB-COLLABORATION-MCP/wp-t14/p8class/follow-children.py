#!/usr/bin/env python3
"""🪆️ T14 content-addressed-child class fix (P8 § Routed, SDK route approved by the coordinator 2026-09-28 12:5x):
derivable composed children follow their coordinate, and the child the coordinate left is retired.

A content-addressed child (`<prefix>-<16 hex>` = `store::content_id` of its content, F9) is minted from the parent's own
content, so every parent-lane edit can re-point a declared coordinate at a child id no store holds — P8's
`composedChildOrphaned` (reasoning wires `addNode`/`addRelationship`; 11 more plugins mint such children). The plugin SDK
now closes that gap once for every plugin: after any parent-lane change (`handle_action`, every typed-operation publication
turn incl. framework reserved commits, every backbone tick) `VcsArtifactApp::follow_derivable_children` opens each declared
child no store holds that `ArtifactApp::genesis_child_pack` derives — through the same genesis restore path
`seed_genesis_children` uses at construction — and retires each held child whose slot now names another derived child:
out of the member map (backward-shift removal, every other member stays addressable), the ownership graph and the published
child-content root (copy-on-write removal; the previous root goes to bounded content retirement) into bounded member
retirement. Gated by the parent's store generation, so a pass that finds nothing moved costs one comparison. The
admission-abort retirement registry becomes the ONE child-member retirement registry (renamed; same bounded drain in
maintenance stage 20 and the close ladder).

usage: follow-children.py [--write] [--root <tree>]   (default: dry run on the live tree)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
problems = []
text = SDK.read_text(encoding="utf-8")
original = text
if "async fn follow_derivable_children(&mut self)" in text:
    print("already applied")
    sys.exit(0)


def exact(old, new, count=1):
    global text
    found = text.count(old)
    if found != count:
        problems.append(f"anchor found {found} times (expected {count}): {old.strip()[:90]!r}")
        return
    text = text.replace(old, new)


for old, new, count in (
    ("child_admission_abort_retirements", "child_member_retirements", 12),
    ("child_admission_abort_generation", "child_member_retirement_generation", 5),
    ("child_admission_abort_cursor", "child_member_retirement_cursor", 4),
    ("child_admission_abort_step", "child_member_retirement_step", 3),
    ("exact failed-child retirement remains admitted", "exact child-member retirement remains admitted", 1),
    ("failed child reports its exact terminal-empty witness", "retired child member reports its exact terminal-empty witness", 1),
):
    exact(old, new, count)

exact("""        fn cancel_admission(&mut self, admission: &ChildMemberAdmission) -> bool {
            if !self.reserved(admission.index) || self.generations[admission.index] != admission.generation {
                return false;
            }
            self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
            true
        }
""", """        fn cancel_admission(&mut self, admission: &ChildMemberAdmission) -> bool {
            if !self.reserved(admission.index) || self.generations[admission.index] != admission.generation {
                return false;
            }
            self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
            true
        }

        fn has_admission_in_flight(&self) -> bool {
            self.reserved.iter().any(|word| *word != 0)
        }

        /// 🍂️ Removes one exact member while no admission is in flight and re-seats every later entry of its probe
        /// run (backward shift), so every other member stays addressable by [`Self::locate`].
        fn remove(&mut self, slot: &str, child_id: &str) -> Result<Option<ChildMemberEntry<M>>, Fault> {
            if self.has_admission_in_flight() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-removal-admission"), "a child member cannot leave the fixed registry while an admission is in flight"));
            }
            let Ok(mut hole) = self.locate(slot, child_id)? else { return Ok(None) };
            let entry = self.take_at(hole).expect("located child member remains occupied");
            let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
            while let Some(moved) = self.entry(cursor) {
                let home = Self::hash(&moved.owner.slot, &moved.reference.artifact_id)?;
                if (cursor + CHILD_CONTENT_SLOTS - home) % CHILD_CONTENT_SLOTS >= (cursor + CHILD_CONTENT_SLOTS - hole) % CHILD_CONTENT_SLOTS {
                    self.relocate(cursor, hole);
                    hole = cursor;
                }
                cursor = (cursor + 1) % CHILD_CONTENT_SLOTS;
            }
            Ok(Some(entry))
        }

        fn relocate(&mut self, from: usize, to: usize) {
            let entry = unsafe { self.slots[from].assume_init_read() };
            self.slots[to].write(entry);
            self.occupied[from / 64] &= !(1 << (from % 64));
            self.occupied[to / 64] |= 1 << (to % 64);
            self.generations[to] = self.generations[from];
            let ordinal = self.ordinal_by_slot[from];
            self.dense_slots[ordinal] = to;
            self.ordinal_by_slot[to] = ordinal;
            self.ordinal_by_slot[from] = usize::MAX;
        }
""")

exact("""        /// 🪪 Stable identity of the immutable child-resolution root retained by one job request.
""", """        fn set_entry(root: &mut ChildContentRoot, index: usize, entry: Option<std::sync::Arc<ChildContentEntry>>) {
            let page_index = index / CHILD_CONTENT_PAGE_SLOTS;
            let mut page = root.pages[page_index].as_deref().cloned().unwrap_or_default();
            page.entries[index % CHILD_CONTENT_PAGE_SLOTS] = entry;
            root.pages[page_index] = page.entries.iter().any(Option::is_some).then(|| std::sync::Arc::new(page));
        }

        /// 🍂️ The same immutable root without one exact child: a copy-on-write removal that re-seats every later
        /// entry of that child's probe run (backward shift), so every other child stays addressable.
        fn without_member(&self, slot: &str, child_id: &str) -> Result<Self, Fault> {
            let Some(current) = self.root.as_deref() else { return Ok(self.clone()) };
            let Ok(mut hole) = Self::locate(current, slot, child_id)? else { return Ok(self.clone()) };
            let mut next = current.clone();
            Self::set_entry(&mut next, hole, None);
            next.len -= 1;
            let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
            while let Some(entry) = Self::entry_at(&next, cursor).cloned() {
                let home = Self::hash(&entry.slot, &entry.child_id)?;
                if (cursor + CHILD_CONTENT_SLOTS - home) % CHILD_CONTENT_SLOTS >= (cursor + CHILD_CONTENT_SLOTS - hole) % CHILD_CONTENT_SLOTS {
                    Self::set_entry(&mut next, hole, Some(entry));
                    Self::set_entry(&mut next, cursor, None);
                    hole = cursor;
                }
                cursor = (cursor + 1) % CHILD_CONTENT_SLOTS;
            }
            Ok(Self { root: (next.len != 0).then(|| std::sync::Arc::new(next)) })
        }

        /// 🪪 Stable identity of the immutable child-resolution root retained by one job request.
""")

exact("        child_member_retirement_cursor: usize,\n", "        child_member_retirement_cursor: usize,\n        followed_parent_generation: u64,\n")
exact("                child_member_retirement_cursor: 0,\n", "                child_member_retirement_cursor: 0,\n                followed_parent_generation: 0,\n")
exact("""            this.seed_genesis_children().await.expect("ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store");
            this
""", """            this.seed_genesis_children().await.expect("ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store");
            this.followed_parent_generation = this.store.generation_now();
            this
""")

exact("""        pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
""", """        /// 🪆️ Derivable composed children follow their coordinate. A content-addressed child is minted from the
        /// parent's own content, so a parent-lane change can re-point a declared coordinate at a child id no store
        /// holds (P8 `composedChildOrphaned`). This opens every declared child no store holds that
        /// `ArtifactApp::genesis_child_pack` derives — the restore path `seed_genesis_children` uses — and retires
        /// every held child whose slot now names another derived child, so exporters, archives, hub members,
        /// agents and the next verb read the child the parent names. Gated by the parent store's generation and
        /// deferred while a child admission is in flight.
        async fn follow_derivable_children(&mut self) -> Result<(), Fault> {
            let generation = self.store.generation_now();
            if generation == self.followed_parent_generation || self.children.has_admission_in_flight() {
                return Ok(());
            }
            let mut genesis = Vec::new();
            let mut retire = Vec::new();
            {
                let snapshot = self.store.snapshot_ref();
                let projection = store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
                let mut declared = Vec::with_capacity(projection.len());
                for index in 0..projection.len() {
                    let Some((slot, fields)) = projection.get(index) else { break };
                    let key = (slot.to_string(), fields.child_id.to_string());
                    if self.children.get(&key).is_none() {
                        if let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id) {
                            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                            genesis.push((key.0.clone(), key.1.clone(), dialect, initial_pack));
                        }
                    }
                    declared.push(key);
                }
                for entry in self.children.entries() {
                    let key = (entry.owner.slot.clone(), entry.reference.artifact_id.clone());
                    if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
                        retire.push(key);
                    }
                }
            }
            for (slot, child_id) in retire {
                self.retire_followed_child(&slot, &child_id).await?;
            }
            let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
            for (slot, child_id, dialect, initial_pack) in genesis {
                let schema = genesis_member_schema::<M>(&dialect)?;
                let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
                let envelope_pack = store::genesis_member_envelope_pack(schema, &dialect, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("followed child {slot}/{child_id} envelope: {error}")))?;
                self.open_child(slot, child_id, dialect, &envelope_pack).await?;
            }
            self.followed_parent_generation = generation;
            Ok(())
        }

        /// 🍂️ Moves one held child its coordinate left out of every live authority — the member map, the ownership
        /// graph and the published child-content root (the previous root goes to bounded content retirement) —
        /// into bounded child-member retirement.
        async fn retire_followed_child(&mut self, slot: &str, child_id: &str) -> Result<(), Fault> {
            let retirement_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child member retirement generation exhausted"))?;
            if !self.child_member_retirements.can_insert(retirement_generation) {
                return Err(plugin_sdk_fault("child member retirement authority is saturated"));
            }
            let publication_generation = self.admit_child_content_publication()?;
            let next = self.child_content_root.without_member(slot, child_id)?;
            let entry = self.children.remove(slot, child_id)?.ok_or_else(|| plugin_sdk_fault("followed child left the member map before its exact retirement"))?;
            self.composition.graph_mut().await.remove_owns(child_id).await;
            let previous = std::mem::replace(&mut *self.child_content_root, next);
            if !previous.is_empty() {
                self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
            }
            self.child_content_generation = publication_generation;
            self.child_member_retirements.insert_admitted(retirement_generation, ChildMemberRetirement::new(entry));
            self.child_member_retirement_generation = retirement_generation;
            Ok(())
        }

        pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
""")

exact("""            let result = self.dispatch_action(action, args, meta).await?;
""", """            let result = self.dispatch_action(action, args, meta).await?;
            self.follow_derivable_children().await?;
""")
exact("""                return self.step_framework_reserved_commit().await;
""", """                self.step_framework_reserved_commit().await?;
                return self.follow_derivable_children().await;
""")
exact("""            self.advance_typed_operation_publication_one().await
        }
""", """            self.advance_typed_operation_publication_one().await?;
            self.follow_derivable_children().await
        }
""")
exact("""            let folded_member_lanes = self.fold_member_inbound().await?;
""", """            let folded_member_lanes = self.fold_member_inbound().await?;
            self.follow_derivable_children().await?;
""")

print(f"{SDK.relative_to(ROOT)}: {'write' if WRITE else 'dry-run'}, {len(problems)} problems")
for problem in problems:
    print("problem:", problem)
if problems:
    sys.exit(1)
if WRITE and text != original:
    SDK.write_text(text, encoding="utf-8")
