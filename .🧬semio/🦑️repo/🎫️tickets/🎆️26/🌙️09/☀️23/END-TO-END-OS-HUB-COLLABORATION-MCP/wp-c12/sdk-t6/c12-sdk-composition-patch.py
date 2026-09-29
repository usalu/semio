"""🧩️ C12 T6 set (SDK `🔌️plugin/🦀️.rs` guest-linked + writer's embedded `dag.jack` example) — the product faults behind
LW1's `c12-nextest-1` writer reds:

1. **Member lanes of content-addressed children** (law `two_authors_typing_at_once_keep_both_runs_and_undo_only_their_own`,
   and — same path — the live "browser actor child: invocation rejected" on resumed hub frames, probe `c12short-8010`):
   `tick_backbone` folded member lanes BEFORE the derivable children followed the freshly folded parent, so a parent op that
   re-points a slot at a new content-addressed child (writer mints one per text edit) arrived together with that child's lane
   and faulted "names no live composed member"; and a lane for a child the parent already superseded (a peer's attach
   announcement of its initial child after this replica typed) faulted the same way. Now the derivable children follow the
   parent first; a lane for a child the CURRENT parent declares but no member holds is still the fault (the composition
   disagreement the rule exists for); a lane for a child the parent no longer declares is a superseded content address —
   derivable from the parent, so nothing is lost — and is skipped.
2. **Loaded documents carry the app's dialect** (laws `set_active_example_falls_back_to_empty_document`,
   `format_artifact_reformats_jack_query`, `commit_rename_renames_all_spans_at_the_config_selection`): `load_document_text`
   / `load_document_pack` reset the store with a pack that names no dialect, so the first child restore faulted "child
   restore requires the parent's exact declared dialect"; every other admission path (`try_adopt_completed`, the
   constructor) stamps `A::DIALECT` — the two load paths now do the same.
3. **Child retirements: pressure + bounded backpressure, never a saturation fault** (law
   `a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step`, red since T14's `s14b-s14c-h5-OWNERS-1X` 09-28
   20:23): an app that re-points a slot at a new content-addressed child per edit admits one child-root AND one child-member
   retirement per keystroke, but `maintenance_under_pressure` only counted the stores' displaced-owner queues, so the 64
   fixed slots drained at one step per 26-stage rotation and the 65th keystroke faulted non-retryably with
   `interactive-job.child-root-retirement-saturated` (a fast typist or a paste burst kills the live document the same way).
   (a) Both registries report pressure at a quarter occupancy and a pressured `maintenance_step` spends its step on their
   stages first (falling through to the rotation when both are blocked, since what unblocks them lives there).
   (b) Coordinator P1: a full registry makes the next admission WAIT. `reclaim_child_retirements(roots, members)` drives the
   two retirement stages synchronously in bounded single-item rounds until the next generations' slots are vacant; every
   admission (`admit_child_content_publication[_span]`, the member-retirement admissions of the follow pass and of
   `open_child`) goes through it. Where retirements are all still borrowed, the follow pass defers before it changes
   anything (the parent already holds every key in order; `child_follow_awaits_retirement` keeps the instance runnable and
   counts as pending work until a later pass followed the parent; member lanes of a child it has not followed yet are
   derivable and skipped) and the mounted child publication stays queued in its publishing ladder (bounded by the existing
   stall ceiling). The residual saturation answer — only for a ring of retirements that are every one still borrowed, at
   sites that cannot defer — is retryable.
   Laws: SDK `child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting` (10 000 publications without a
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
   (d) The history half: reading the history of a long coalesced run cost O(run) per key (the rebuilt view copied every
   printed op line of the growing row, the panel joined them all, every dirty `HistoryEntry` export cloned them). A row now
   previews its edit's LAST `HISTORY_ROW_OPERATION_PREVIEW` (8) operations and records the operation count it printed at
   (`CommandView::op_count`): a sealed row's preview is reused while its edit still has that many operations, the growing
   row is re-printed in O(preview). SDK law `a_long_coalesced_gesture_previews_its_newest_operations_only`.
   (e) The digest rule is language-neutral: `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` (single-operation,
   grown-to-two, grown-to-ten-thousand → expected digests from a third implementation, Python hashlib), schema def
   `EditDigestChains`, replayed by the TS oracle (node:crypto) and the Rust store law. Only the Rust store computes this
   digest (the single-operation sealer path is unchanged); the hub, the TS worker and replication carry revisions opaquely.
   (f) The exported `kernel::HistoryEntry` carries `op_count` beside the bounded `op_lines` preview (Rust + TS twin), and the
   React history panel prefixes an omitted head with "…"; the SDK law checks the exported row too.
4. **`dag.jack` example asset** (law `every_demo_asset_is_the_printers_own_content_addressed_output`): the content-address
   change to the specified SHA-256 prefix (`store::content_id`, auto-commit 27d829d8a5) regenerated every demo asset except
   writer's `🧪️dag-example`, whose document child still carries the old `DefaultHasher` id; `set-active-example dag.jack`
   embeds it in the guest (`include_str!`), so the printer's own id is restored there.
Idempotent; usage: python3 c12-sdk-composition-patch.py [--apply]   (default dry run; C12_REPO overrides the tree)"""
import os
import sys

REPO = os.environ.get("C12_REPO", "/Users/ueli/Documents/semio")
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
DAG = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio"
SDK_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
DURABLE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs"
STORE_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
PANEL_KIT_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs"
CANONICAL_FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json"
CANONICAL_SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json"
CANONICAL_TS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts"
CANONICAL_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs"
KERNEL = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs"
KERNEL_TS = "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts"
SHELL_HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
PREVIEW_EVAL_LAWS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs"
WRITER_LAWS = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs"
APPLY = "--apply" in sys.argv

LOAD_OLD_TEXT = """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
"""
LOAD_NEW_TEXT = """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
"""
LOAD_OLD_PACK = """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
"""
LOAD_NEW_PACK = """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
"""
HELPER_ANCHOR = "    impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {\n"
HELPER = """    /// 🪪️ A document loaded from its text or pack belongs to the app that loads it: a pack that names no dialect is stamped
    /// with the app's own, exactly as every admission path (`try_adopt_completed`, the constructor) stamps it — without it the
    /// first child restore faulted "child restore requires the parent's exact declared dialect".
    fn loaded_with_app_dialect<A: ArtifactApp>(mut envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>) -> ArtifactEnvelope<A::Snapshot, A::Mutation> {
        if envelope.dialect.is_none() {
            envelope.dialect = Some(A::DIALECT.into());
        }
        envelope
    }

"""
TICK_OLD = """            let folded_member_lanes = self.fold_member_inbound().await?;
            self.follow_derivable_children().await?;
"""
TICK_NEW = """            self.follow_derivable_children().await?;
            let folded_member_lanes = self.fold_member_inbound().await?;
"""
FOLD_OLD = """        /// 📥️ Routes every member-addressed backbone message the parent store pumped into the
        /// live member that owns that exact lane, and answers how many lanes folded something. A lane
        /// naming a member this replica does not hold is a fault, never a silent drop: the two
        /// replicas would otherwise disagree about the document's own composition.
        async fn fold_member_inbound(&mut self) -> Result<usize, Fault> {
            let inbound = self.store.take_member_inbound();
            if inbound.is_empty() {
                return Ok(0);
            }
            let mut folded = 0usize;
            for (slot, child_id, envelopes) in inbound {
                let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else {
                    return Err(plugin_sdk_fault(format!("backbone member lane {slot}/{child_id} names no live composed member of this replica")));
                };
"""
FOLD_NEW = """        /// 📥️ Routes every member-addressed backbone message the parent store pumped into the
        /// live member that owns that exact lane, and answers how many lanes folded something. Runs after the
        /// derivable children followed the freshly folded parent, so a lane for the child a parent op just named
        /// finds its member. A lane naming a member the CURRENT parent declares but this replica does not hold is
        /// a fault, never a silent drop: the two replicas would otherwise disagree about the document's own
        /// composition. A lane naming a child the parent no longer declares is a superseded content address (a
        /// peer's lane for the child its own later edit — or ours — replaced), and a lane naming a child the
        /// follow pass has not opened yet because it waits on retirements is the child that pass derives: both
        /// carry nothing the composition does not already hold and are skipped.
        async fn fold_member_inbound(&mut self) -> Result<usize, Fault> {
            let inbound = self.store.take_member_inbound();
            if inbound.is_empty() {
                return Ok(0);
            }
            let mut folded = 0usize;
            for (slot, child_id, envelopes) in inbound {
                let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else {
                    let projection = store::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                    if !self.derivable_follow_awaits_retirement() && (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared.to_string() == slot && fields.child_id.to_string() == child_id) {
                        return Err(plugin_sdk_fault(format!("backbone member lane {slot}/{child_id} names no live composed member of this replica")));
                    }
                    continue;
                };
"""
STAGES_OLD = """    /// 🧹️ The rotation stage that drains the config-lane stores' displaced-owner queues.
    pub const MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE: u8 = 25;
"""
STAGES_NEW = """    /// 🧹️ The rotation stage that drains the config-lane stores' displaced-owner queues.
    pub const MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE: u8 = 25;
    /// 🍂️ The rotation stage that disposes retired child-content roots, also run out of turn under child pressure.
    pub(crate) const MAINTENANCE_CHILD_ROOT_STAGE: u8 = 4;
    /// 🥀️ The rotation stage that closes retired child members, also run out of turn under child pressure.
    pub(crate) const MAINTENANCE_CHILD_MEMBER_STAGE: u8 = 20;
    /// 🌡️ Occupancy of the child-root or child-member retirement registry at which maintenance spends out-of-turn steps on
    /// it — a quarter of the fixed [`ARTIFACT_LIVE_OUTPUT_SLOTS`], as the stores' displaced-owner pressure is of theirs.
    pub(crate) const MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY: usize = ARTIFACT_LIVE_OUTPUT_SLOTS / 4;
    /// ⏳️ Rounds one child publication spends reclaiming the retirements ahead of it before it waits instead.
    pub(crate) const CHILD_RETIREMENT_RECLAIM_STEPS: usize = ARTIFACT_LIVE_OUTPUT_SLOTS * 16;
"""
ROOT_STAGE_OLD = """                4 => {
                    let Some((index, generation)) = self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor) else {
                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    };
                    let step = {
                        let retirements = &mut self.child_content_retirements;
                        let children = &mut self.children;
                        let retiring = &mut self.child_member_retirements;
                        let current = &*self.child_content_root;
                        let owners = retirements.sibling_content_owners(generation);
                        retirements
                            .get_mut(generation)
                            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-authority"), "live child root retirement authority changed during one fixed step"))?
                            .close_step(children, Some(retiring), current, &owners, maximum_items, maximum_bytes)?
                    };
                    if step != PluginCloseStep::Complete {
                        self.maintenance_child_root_cursor = index;
                        return Ok(step);
                    }
                    if !self.child_content_retirements.get(generation).is_some_and(ChildContentRetirement::terminal_is_empty) {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-terminal-not-empty"), "live child root retirement reported Complete without its exact terminal-empty witness"));
                    }
                    let retirement = self
                        .child_content_retirements
                        .remove(generation)
                        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-authority"), "terminal live child root retirement changed before exact removal"))?;
                    drop(retirement);
                    self.maintenance_child_root_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                    Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                }
"""
ROOT_STAGE_NEW = """                MAINTENANCE_CHILD_ROOT_STAGE => self.child_root_retirement_step(maximum_items, maximum_bytes),
"""
CHILD_ROOT_ANCHOR = """        fn child_member_retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
"""
CHILD_ROOT_FN = """        /// 🍂️ One bounded unit of child-root retirement — the body of stage [`MAINTENANCE_CHILD_ROOT_STAGE`], also run out of
        /// turn under child pressure and by [`Self::reclaim_child_retirements`] (which needs it without the maintenance impl's
        /// `M: Send`).
        fn child_root_retirement_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            let Some((index, generation)) = self.child_content_retirements.next_id_from(self.maintenance_child_root_cursor) else {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            };
            let step = {
                let retirements = &mut self.child_content_retirements;
                let children = &mut self.children;
                let retiring = &mut self.child_member_retirements;
                let current = &*self.child_content_root;
                let owners = retirements.sibling_content_owners(generation);
                retirements
                    .get_mut(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-authority"), "live child root retirement authority changed during one fixed step"))?
                    .close_step(children, Some(retiring), current, &owners, maximum_items, maximum_bytes)?
            };
            if step != PluginCloseStep::Complete {
                self.maintenance_child_root_cursor = index;
                return Ok(step);
            }
            if !self.child_content_retirements.get(generation).is_some_and(ChildContentRetirement::terminal_is_empty) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-terminal-not-empty"), "live child root retirement reported Complete without its exact terminal-empty witness"));
            }
            let retirement = self
                .child_content_retirements
                .remove(generation)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.maintenance-child-root-authority"), "terminal live child root retirement changed before exact removal"))?;
            drop(retirement);
            self.maintenance_child_root_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
        }

"""
MEMBER_STAGE_OLD = "                20 => self.child_member_retirement_step(maximum_items.min(1), maximum_bytes),\n"
MEMBER_STAGE_NEW = "                MAINTENANCE_CHILD_MEMBER_STAGE => self.child_member_retirement_step(maximum_items.min(1), maximum_bytes),\n"
STEP_OLD = """                return self.maintenance_config_lane_displaced_step(maximum_items, maximum_bytes);
            }
            let mut unproductive = PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
"""
STEP_NEW = """                return self.maintenance_config_lane_displaced_step(maximum_items, maximum_bytes);
            }
            if self.child_retirements_under_pressure() {
                for stage in [MAINTENANCE_CHILD_ROOT_STAGE, MAINTENANCE_CHILD_MEMBER_STAGE] {
                    LAST_MAINTENANCE_STAGE.store(u64::from(stage), std::sync::atomic::Ordering::Relaxed);
                    let step = self.maintenance_stage_step(stage, maximum_items, maximum_bytes)?;
                    if matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0) {
                        return Ok(step);
                    }
                }
            }
            let mut unproductive = PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
"""
PRESSURE_OLD = """        fn maintenance_under_pressure(&self) -> bool {
            self.store.maintenance_retirements_under_pressure()
                || self.config_store.maintenance_retirements_under_pressure()
                || self.draft_store.maintenance_retirements_under_pressure()
                || self.window_config_store.maintenance_retirements_under_pressure()
        }
"""
PRESSURE_NEW = """        fn maintenance_under_pressure(&self) -> bool {
            self.store.maintenance_retirements_under_pressure()
                || self.config_store.maintenance_retirements_under_pressure()
                || self.draft_store.maintenance_retirements_under_pressure()
                || self.window_config_store.maintenance_retirements_under_pressure()
                || self.child_retirements_under_pressure()
        }
"""
CHILD_PRESSURE_ANCHOR = """        #[inline(never)]
        /// 🧹️ One displaced-owner step of the document store — the body of stage
"""
CHILD_PRESSURE = """        /// 🌡️ True while the child-root or child-member retirement registry holds at least
        /// [`MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`] retirements. An app that re-points a slot at a new
        /// content-addressed child per edit (writer mints one per keystroke) admits one of each per commit, which the single
        /// fair step per turn drains once per rotation: a typing run saturated the fixed slots with
        /// `interactive-job.child-root-retirement-saturated`.
        fn child_retirements_under_pressure(&self) -> bool {
            self.child_content_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY || self.child_member_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY
        }

"""
LEN_OLD = """        pub(crate) fn is_empty(&self) -> bool {
            self.occupied == 0
        }
"""
LEN_NEW = """        pub(crate) fn is_empty(&self) -> bool {
            self.occupied == 0
        }

        /// 🔢️ Admitted entries — the occupied mask's population.
        pub(crate) fn len(&self) -> usize {
            self.occupied.count_ones() as usize
        }
"""
TRAIT_DOC_OLD = """        /// 🌡️ True while a displaced-owner queue of the document or a config-lane store sits at or
        /// above [`store::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`]. The runtime then spends a
        /// burst of maintenance steps on this instance in the same turn instead of one — the single
        /// fair step per turn drains one owner per 26-stage rotation while a live commit cadence
        /// displaces three or more owners per commit.
"""
TRAIT_DOC_NEW = """        /// 🌡️ True while a displaced-owner queue of the document or a config-lane store sits at or
        /// above [`store::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`], or a child-root or child-member
        /// retirement registry at or above [`MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`]. The runtime
        /// then spends a burst of maintenance steps on this instance in the same turn instead of one — the
        /// single fair step per turn drains one owner per 26-stage rotation while a live commit cadence
        /// displaces three or more owners (and, for a content-addressed child, one child root and member) per commit.
"""
DRAIN_DOC_OLD = """        /// 🌡️ Spends the maintenance the plugin runtime spends on a pressured instance: while a displaced-owner queue of the
        /// document or a config-lane store sits at or above `store::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`, the runtime
        /// drains it in one-item steps of `plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP` within the same turn. A law that
        /// commits faster than the fair rotation retires (a typing run: every keystroke amends one edit and displaces its
        /// envelope, snapshot and dag) calls this after each settled command, as the live host does.
"""
DRAIN_DOC_NEW = """        /// 🌡️ Spends the maintenance the plugin runtime spends on a pressured instance: while a displaced-owner queue of the
        /// document or a config-lane store sits at or above `store::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`, or a child
        /// retirement registry at or above `MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`, the runtime drains it in
        /// one-item steps of `plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP` within the same turn. A law that commits faster
        /// than the fair rotation retires (a typing run: every keystroke amends one edit and displaces its envelope, snapshot
        /// and dag — and a content-addressed child's root and member) calls this after each settled command, as the live host does.
"""
ADMIT_OLD = """        pub(crate) fn admit_child_content_publication(&self) -> Result<u64, Fault> {
            let generation = self.child_content_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"))?;
            if !self.child_content_retirements.can_insert(generation) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication is saturated pending an exact child snapshot disposer"));
            }
            Ok(generation)
        }

        fn admit_child_content_publication_span(&self, count: usize) -> Result<(), Fault> {
            if count > ARTIFACT_LIVE_OUTPUT_SLOTS {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-span"), "one child root publication span exceeds the fixed retirement authority"));
            }
            for offset in 1..=count {
                let generation = self.child_content_generation.checked_add(offset as u64).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"))?;
                if !self.child_content_retirements.can_insert(generation) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication span is saturated pending an exact child snapshot disposer"));
                }
            }
            Ok(())
        }
"""
ADMIT_NEW = """        /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
        /// work until maintenance returned them (see `child_follow_awaits_retirement`).
        fn derivable_follow_awaits_retirement(&self) -> bool {
            self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
        }

        /// ♻️ Whether the next `roots` child-content and `members` child-member retirement generations each find a vacant
        /// slot of their fixed registry.
        fn child_retirements_admit(&self, roots: usize, members: usize) -> bool {
            (1..=roots as u64).all(|offset| self.child_content_generation.checked_add(offset).is_some_and(|generation| self.child_content_retirements.can_insert(generation)))
                && (1..=members as u64).all(|offset| self.child_member_retirement_generation.checked_add(offset).is_some_and(|generation| self.child_member_retirements.can_insert(generation)))
        }

        /// ⏳️ Bounded backpressure of a child publication: while the next `roots` child-content and `members` child-member
        /// retirement generations find no vacant slot, drives the child-root and child-member retirement stages one item at a
        /// time — at most [`CHILD_RETIREMENT_RECLAIM_STEPS`] rounds, ending at the first round that releases nothing — and
        /// answers whether the publication is admissible now. A commit cadence faster than the maintenance rotation (a typing
        /// burst over a content-addressed child) therefore waits on maintenance instead of faulting; `false` means every
        /// retirement ahead of it is still borrowed, and the follow pass and the mounted child publication then wait, in
        /// order, for the borrower to return it.
        fn reclaim_child_retirements(&mut self, roots: usize, members: usize) -> Result<bool, Fault> {
            if roots > ARTIFACT_LIVE_OUTPUT_SLOTS || members > ARTIFACT_LIVE_OUTPUT_SLOTS {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-span"), "one child publication span exceeds the fixed retirement authority"));
            }
            for _ in 0..CHILD_RETIREMENT_RECLAIM_STEPS {
                if self.child_retirements_admit(roots, members) {
                    return Ok(true);
                }
                let released = |step: PluginCloseStep| matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0);
                let root = released(self.child_root_retirement_step(1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?);
                let member = released(self.child_member_retirement_step(1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?);
                if !root && !member {
                    break;
                }
            }
            Ok(self.child_retirements_admit(roots, members))
        }

        /// 🎟️ Admits the next immutable child-content publication once its retirement slot is vacant — reclaimed first when
        /// an older retirement still holds it ([`Self::reclaim_child_retirements`]).
        pub(crate) fn admit_child_content_publication(&mut self) -> Result<u64, Fault> {
            self.admit_child_content_publication_span(1)?;
            Ok(self.child_content_generation + 1)
        }

        /// 🎟️ Admits `count` consecutive child-content publications (see [`Self::admit_child_content_publication`]). Only a
        /// ring of retirements that are every one still borrowed answers the retryable
        /// `interactive-job.child-root-retirement-saturated`.
        fn admit_child_content_publication_span(&mut self, count: usize) -> Result<(), Fault> {
            if self.child_content_generation.checked_add(count as u64).is_none() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"));
            }
            if !self.reclaim_child_retirements(count, 0)? {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication waits on retirements that are all still borrowed").with_retryable(true));
            }
            Ok(())
        }
"""
RETIRE_CHECK_OLD = """            if !self.child_member_retirements.can_insert(retirement_generation) {
                return Err(plugin_sdk_fault("child member retirement authority is saturated"));
            }
"""
RETIRE_CHECK_NEW = """            if !self.reclaim_child_retirements(0, 1)? {
                return Err(plugin_sdk_fault("child member retirement waits on retirements that are all still borrowed").with_retryable(true));
            }
"""
ABORT_CHECK_OLD = """            if !self.child_member_retirements.can_insert(abort_generation) {
                return Err(plugin_sdk_fault("failed child retirement authority is saturated"));
            }
"""
ABORT_CHECK_NEW = """            if !self.reclaim_child_retirements(0, 1)? {
                return Err(plugin_sdk_fault("failed child retirement waits on retirements that are all still borrowed").with_retryable(true));
            }
"""
PERMIT_ANCHOR = """            let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
"""
PERMIT_WAIT = """            match self.reclaim_child_retirements(1, 0) {
                Ok(true) => {}
                Ok(false) => {
                    mounted.pending_child_publication = Some(pending);
                    return Ok(());
                }
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            }
"""
FOLLOW_DOC_OLD = """        /// deferred while a child admission is in flight or while the named child is still retiring (an undo back
        /// to it) — the pass then repeats until that retirement drained.
        async fn follow_derivable_children(&mut self) -> Result<(), Fault> {
"""
FOLLOW_DOC_NEW = """        /// deferred while a child admission is in flight or while the named child is still retiring (an undo back
        /// to it) — the pass then repeats until that retirement drained. It also waits, before it changes anything,
        /// while the retirements its own publications need are not yet admissible ([`Self::reclaim_child_retirements`]):
        /// the parent already holds every edit in order, the child it names follows once maintenance returned them, and
        /// the instance stays runnable meanwhile (`child_follow_awaits_retirement`).
        async fn follow_derivable_children(&mut self) -> Result<(), Fault> {
"""
RETIRE_LOOP_OLD = """            for (slot, child_id) in retire {
                self.retire_followed_child(&slot, &child_id).await?;
            }
"""
RETIRE_LOOP_NEW = """            let publications = retire.len() + genesis.len();
            if !self.reclaim_child_retirements(publications, publications)? {
                self.child_follow_awaits_retirement = true;
                return Ok(());
            }
            for (slot, child_id) in retire {
                self.retire_followed_child(&slot, &child_id).await?;
            }
"""
FOLLOW_END_OLD = """            if !deferred {
                self.followed_parent_generation = generation;
            }
            Ok(())
        }
"""
FOLLOW_END_NEW = """            if !deferred {
                self.followed_parent_generation = generation;
            }
            self.child_follow_awaits_retirement = deferred;
            Ok(())
        }
"""
FIELD_OLD = "        followed_parent_generation: u64,\n"
FIELD_NEW = """        followed_parent_generation: u64,
        /// ⏳️ The last follow pass waited on child retirements — its own publications were not yet admissible, or the child
        /// the parent names was still retiring — so the instance stays runnable until a later pass followed the parent.
        child_follow_awaits_retirement: bool,
"""
INIT_OLD = "                followed_parent_generation: 0,\n"
INIT_NEW = "                followed_parent_generation: 0,\n                child_follow_awaits_retirement: false,\n"
PENDING_OLD = """        fn has_pending_typed_operations(&self) -> bool {
            self.operation_progress_retired
"""
PENDING_NEW = """        fn has_pending_typed_operations(&self) -> bool {
            self.operation_progress_retired
                || self.derivable_follow_awaits_retirement()
"""
RUNNABLE_OLD = """        fn has_runnable_typed_operations(&self) -> bool {
            self.operation_progress_retired
"""
RUNNABLE_NEW = """        fn has_runnable_typed_operations(&self) -> bool {
            self.operation_progress_retired
                || self.derivable_follow_awaits_retirement()
"""
SDK_LAW_ANCHOR = """        let generation = app.admit_child_content_publication().expect("later publication can reuse the fixed retirement registry");
        app.publish_child_content_member(generation, "slot", "child-a").await.expect("later publication after bounded reclaim");
        drain_and_close_composed_fixture(&mut app);
    }
"""
SDK_LAW = """
    /// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): child-content publications at the maximum rate — 10 000 with no
    /// maintenance turn between them — never fault. Every admission waits on the retirement ahead of it (bounded synchronous
    /// reclaim) instead of answering `interactive-job.child-root-retirement-saturated`, every publication is admitted in
    /// order (exactly the next generation), the registry never holds more than its fixed slots, and the saturated registry
    /// reports maintenance pressure until maintenance returned it below its quarter-occupancy bound.
    #[semio_framework_async_macros::async_test]
    async fn child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting() {
        let mut app = contract_composed_app_raw().await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a")).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let start = app.child_content_generation;
        for index in 1..=10_000u64 {
            let generation = app.admit_child_content_publication().unwrap_or_else(|fault| panic!("publication {index} faulted instead of waiting: {fault:?}"));
            assert_eq!(generation, start + index, "publication {index} is admitted in order");
            app.publish_child_content_member(generation, "slot", "child-a").await.unwrap_or_else(|fault| panic!("publication {index} did not land: {fault:?}"));
            assert!(app.child_content_retirements.len() <= ARTIFACT_LIVE_OUTPUT_SLOTS, "the child-root retirement registry stays within its fixed slots");
        }
        assert!(PluginApp::maintenance_under_pressure(&app), "a saturated child-root retirement registry reports maintenance pressure");
        crate::app::artifact_app_laws::drain_maintenance_pressure(&mut app);
        assert!(app.child_content_retirements.len() < MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY, "pressure maintenance returns the child roots below their quarter-occupancy bound");
        drain_and_close_composed_fixture(&mut app);
    }
"""
WRITER_LAW_ANCHOR = """    crate::editor::writer::unit_tests::context::history_verb(&mut app, "redo").await;
    assert_eq!(writer_text(&app.snapshot().expect("projection")), run.expected, "one redo restores the whole run");
}
"""
WRITER_LAW = """
/// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): ONE uninterrupted 10 000-keystroke typing run at the maximum rate — one
/// full-text `text-edit` per key with corrections, no pressure drain between keys — applies every key in order, never faults,
/// and costs the same per key at its end as at its start. Every key re-points the document slot at a new content-addressed
/// child and so admits child-root and child-member retirements faster than the maintenance rotation returns them;
/// publication and the follow pass wait on those retirements (the 65th key answered
/// `interactive-job.child-root-retirement-saturated` before). Every key also amends the run's one coalesced edit, which
/// re-encoded the whole run per key before (the median key of the last 1 000 must stay within 3 × the first 1 000).
/// Afterwards maintenance returns the retirements below their pressure bound.
#[semio_framework_async_macros::async_test]
async fn a_ten_thousand_keystroke_burst_applies_every_key_in_order() {
    const KEYS: [char; 8] = ['a', 'q', 'ß', 'ü', '€', '𝄞', ' ', '\\n'];
    let mut app = new_app().await;
    let mut model = String::new();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut key_nanos = Vec::with_capacity(10_000);
    for index in 0..10_000usize {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        if index % 7 == 6 {
            model.pop();
        } else {
            if model.chars().count() == 48 {
                model.remove(0);
            }
            model.push(KEYS[(seed >> 33) as usize % KEYS.len()]);
        }
        let started = std::time::Instant::now();
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
    assert!(last <= first.saturating_mul(3), "per-key cost stays flat over one uninterrupted run: the median key of the last 1 000 took {last} ns against {first} ns for the first 1 000");
    semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *app);
    assert!(!PluginApp::maintenance_under_pressure(&*app), "maintenance returns the burst's retirements below their pressure bound");
}
"""

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
    let files = print_document_pack(store.envelope()).await.expect("owned document encode");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("owned document decode").envelope).await;
    assert_eq!(reloaded.content_revision_now(), store.content_revision_now(), "the incrementally extended revision equals the from-scratch one");
}
"""

PREVIEW_CONST_OLD = """        /// 📜️ This entry's edit's forward operations, printed via `OpText::print_op` — empty for
        /// cursor-motion entries and for a dangling `edit_id` (document replaced mid-session).
        pub op_lines: Vec<String>,
"""
PREVIEW_CONST_NEW = """        /// 📜️ The LAST [`HISTORY_ROW_OPERATION_PREVIEW`] forward operations of this entry's edit, printed via
        /// `OpText::print_op`, newest last — a bounded preview, so reading the history costs the same per key at the end of a
        /// long coalesced typing run as at its start; empty for cursor-motion entries and for a dangling `edit_id` (document
        /// replaced mid-session).
        pub op_lines: Vec<String>,
        /// 🔢️ Forward operations of this entry's edit when `op_lines` was printed — a sealed row's preview is reused only while
        /// its edit still has exactly that many.
        pub op_count: usize,
"""
PREVIEW_DECL_ANCHOR = """    /// 📜️ Checkpoint/alternative history summary exposed to apps — the swimlane columns, the
    /// undo/redo availability, the current checkout position, and the merged command+operation timeline.
"""
PREVIEW_DECL = """    /// 📜️ Operations a history row previews: its edit's newest ones (see `CommandView::op_lines`).
    pub const HISTORY_ROW_OPERATION_PREVIEW: usize = 8;

"""
PRINTED_OLD = """            let printed: HashMap<&str, &[String]> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().filter_map(|row| row.edit_id.as_deref().map(|edit_id| (edit_id, row.op_lines.as_slice()))).collect());"""
PRINTED_NEW = """            let printed: HashMap<&str, (usize, &[String])> = previous.map_or_else(HashMap::new, |previous| previous.commands.iter().filter_map(|row| row.edit_id.as_deref().map(|edit_id| (edit_id, (row.op_count, row.op_lines.as_slice())))).collect());"""
ROW_OLD = """                let mut op_lines = Vec::new();
                if let Some(edit) = edit {
                    let retained = entry.edit_id.as_deref().and_then(|edit_id| printed.get(edit_id).copied()).filter(|lines| lines.len() <= edit.forwards.len()).unwrap_or_default();
                    op_lines.extend_from_slice(retained);
                    for op in edit.forwards.iter().skip(retained.len()) {
                        op_lines.push(op.print_op());
                    }
                }"""
ROW_NEW = """                let mut op_lines = Vec::new();
                let mut op_count = 0;
                if let Some(edit) = edit {
                    op_count = edit.forwards.len();
                    match entry.edit_id.as_deref().and_then(|edit_id| printed.get(edit_id).copied()).filter(|(count, _)| *count == op_count) {
                        Some((_, lines)) => op_lines.extend_from_slice(lines),
                        None => op_lines.extend(edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op())),
                    }
                }"""
ROW_FIELD_OLD = """                    op_lines,
                    applied,
                    revertible,"""
ROW_FIELD_NEW = """                    op_lines,
                    op_count,
                    applied,
                    revertible,"""
ARM_OLD = """                                if command.op_lines.len() > edit.forwards.len() {
                                    return false;
                                }
                                for operation in edit.forwards.iter().skip(command.op_lines.len()) {
                                    command.op_lines.push(operation.print_op());
                                }
                                true"""
ARM_NEW = """                                if command.op_count > edit.forwards.len() {
                                    return false;
                                }
                                if command.op_count != edit.forwards.len() {
                                    command.op_lines = edit.forwards[edit.forwards.len().saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|operation| operation.print_op()).collect();
                                    command.op_count = edit.forwards.len();
                                }
                                true"""
HISTORY_LAW_ANCHOR = """        assert_eq!(set_label_entries.len(), 1, "a coalesced gesture must grow one entry's op_lines, not append new entries");
    }
"""
HISTORY_LAW = """
    /// 📜️ LAW (ticket 26/09/23 C12): a coalesced gesture's history row previews only its edit's newest
    /// `HISTORY_ROW_OPERATION_PREVIEW` operations — the newest one last — however long the gesture runs, so reading the history
    /// costs the same per key at the end of a long typing run as at its start.
    #[semio_framework_async_macros::async_test]
    async fn a_long_coalesced_gesture_previews_its_newest_operations_only() {
        let mut app = contract_app().await;
        let mut value = String::new();
        for key in 0..64u8 {
            value.push(char::from(b'a' + key % 26));
            app.dispatch_typed(TestCommand::SetLabel { value: value.clone() }, &meta()).await.expect("setLabel");
            let _ = app.test_history().await;
        }
        let history = app.test_history().await;
        let row = history.commands.iter().find(|entry| entry.action_id == "setLabel").expect("the gesture's row");
        assert_eq!(row.op_count, 64, "the row counts every operation of its edit");
        assert_eq!(row.op_lines.len(), HISTORY_ROW_OPERATION_PREVIEW, "the row previews a bounded tail of its operations");
        assert!(row.op_lines.last().is_some_and(|line| line.contains(&value)), "the newest operation closes the preview: {:?}", row.op_lines);
        let exported = app.history_snapshot().await.expect("history snapshot");
        let exported = exported.upserts.iter().find(|entry| entry.action_id == "setLabel").expect("the gesture's exported row");
        assert_eq!((exported.op_count, exported.op_lines.len()), (64, HISTORY_ROW_OPERATION_PREVIEW), "the exported row carries the bounded preview and its edit's operation count");
    }
"""

CHAINS_FIXTURE = """{
  "schema": "semio.store.edit-digest-chains.v1",
  "cases": [
    {
      "name": "single-operation",
      "edit": {
        "id": "edit-typing",
        "actor": "actor-1",
        "forwards": [
          {
            "SetN": {
              "n": 1
            }
          }
        ],
        "inverse": [
          {
            "SetN": {
              "n": 0
            }
          }
        ],
        "mutationMeta": [
          {
            "mutation_id": "edit-typing#0",
            "dependencies": [
              "prior-1"
            ],
            "base_version": 0,
            "author_id": "actor-1",
            "timestamp": {
              "actor": 1,
              "physical_ms": 42,
              "logical": 0
            },
            "undo_policy": "ExactBaseOnly",
            "label": "Edit text",
            "group_id": "group-typing"
          }
        ],
        "description": "typing 🧵",
        "coalesceKey": "typing",
        "sequenceNumber": 7,
        "startedAt": "2026-09-29T11:00:00Z",
        "finishedAt": "2026-09-29T11:00:09Z"
      },
      "expectedDigest": "87ef04718e7724e860fe74985366c195ba2820713ce1a26f3a17f5a5ff1baa14"
    },
    {
      "name": "grown-to-two",
      "edit": {
        "id": "edit-typing",
        "actor": "actor-1",
        "forwards": [
          {
            "SetN": {
              "n": 1
            }
          },
          {
            "SetN": {
              "n": 2
            }
          }
        ],
        "inverse": [
          {
            "SetN": {
              "n": 0
            }
          },
          {
            "SetN": {
              "n": 1
            }
          }
        ],
        "mutationMeta": [
          {
            "mutation_id": "edit-typing#0",
            "dependencies": [
              "prior-1"
            ],
            "base_version": 0,
            "author_id": "actor-1",
            "timestamp": {
              "actor": 1,
              "physical_ms": 42,
              "logical": 0
            },
            "undo_policy": "ExactBaseOnly",
            "label": "Edit text",
            "group_id": "group-typing"
          },
          {
            "mutation_id": "edit-typing#1",
            "dependencies": [
              "prior-1"
            ],
            "base_version": 1,
            "author_id": "actor-1",
            "timestamp": {
              "actor": 1,
              "physical_ms": 43,
              "logical": 0
            },
            "undo_policy": "ExactBaseOnly",
            "label": "Edit text",
            "group_id": "group-typing"
          }
        ],
        "description": "typing 🧵",
        "coalesceKey": "typing",
        "sequenceNumber": 7,
        "startedAt": "2026-09-29T11:00:00Z",
        "finishedAt": "2026-09-29T11:00:09Z"
      },
      "expectedDigest": "3d2a6e092a9f185e673405f674282ec3c4c892033ce8299d464e304acba2f105"
    },
    {
      "name": "grown-to-ten-thousand",
      "header": {
        "id": "edit-typing",
        "actor": "actor-1",
        "description": "typing 🧵",
        "coalesceKey": "typing",
        "sequenceNumber": 7,
        "startedAt": "2026-09-29T11:00:00Z",
        "finishedAt": "2026-09-29T11:00:09Z"
      },
      "generatedOperations": 10000,
      "expectedDigest": "747be7889c6633277ad5da84894482d72fe237662080d81ff233a77ae83c7093"
    }
  ]
}
"""
CHAINS_SCHEMA_OLD = """        "label": { "type": "string" },
        "group_id": { "type": "string" }
      }
    }
  }
}
"""
CHAINS_SCHEMA_NEW = """        "label": { "type": "string" },
        "group_id": { "type": "string" }
      }
    },
    "EditDigestChains": {
      "type": "object",
      "additionalProperties": false,
      "required": ["schema", "cases"],
      "properties": {
        "schema": { "const": "semio.store.edit-digest-chains.v1" },
        "cases": { "type": "array", "minItems": 3, "items": { "oneOf": [{ "$ref": "#/$defs/EditDigestChainsEditCase" }, { "$ref": "#/$defs/EditDigestChainsGeneratedCase" }] } }
      }
    },
    "EditDigestChainsEditCase": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "edit", "expectedDigest"],
      "properties": {
        "name": { "type": "string", "minLength": 1 },
        "edit": { "type": "object" },
        "expectedDigest": { "$ref": "#/$defs/CanonicalEditDigest" }
      }
    },
    "EditDigestChainsGeneratedCase": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "header", "generatedOperations", "expectedDigest"],
      "properties": {
        "name": { "type": "string", "minLength": 1 },
        "header": { "type": "object" },
        "generatedOperations": { "type": "integer", "minimum": 2 },
        "expectedDigest": { "$ref": "#/$defs/CanonicalEditDigest" }
      }
    }
  }
}
"""
CHAINS_TS_OLD = """  for (const [name, hostile] of hostiles) assert.equal(exported(name)(hostile), false, `${name} hostile is refused`);

}
//#endregion 🧵️CanonicalEditOracle"""
CHAINS_TS_NEW = """  for (const [name, hostile] of hostiles) assert.equal(exported(name)(hostile), false, `${name} hostile is refused`);

  const chains = read("./🧫️fixtures/🔗️edit-digest-chains.json");
  const validateChains = exported("EditDigestChains");
  assert(validateChains(chains), JSON.stringify(validateChains.errors));
  assert.equal(validateChains({ ...chains, cases: chains.cases.map((row: Record<string, unknown>) => ({ ...row, unexpected: true })) }), false, "EditDigestChains hostile is refused");
  const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const record = (domain: string, parts: Buffer[]) => {
    const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
    for (const part of parts) hash.update(u64(part.length)).update(part);
    return hash.digest();
  };
  const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
  const text = (edit: Record<string, unknown>, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
  const editDigest = (edit: Record<string, unknown>) => {
    const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
    if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(edit))]);
    const sequence = Buffer.alloc(4);
    sequence.writeInt32BE(edit.sequenceNumber as number);
    return record("edit-chained", [
      Buffer.from(String(edit.id)),
      ...text(edit, "actor"),
      ...text(edit, "description"),
      ...text(edit, "coalesceKey"),
      sequence,
      Buffer.from(String(edit.startedAt)),
      ...text(edit, "finishedAt"),
      u64(forwards.length), chain("edit-forward", forwards),
      u64(inverse.length), chain("edit-inverse", inverse),
      u64(meta.length), chain("edit-meta", meta),
    ]);
  };
  for (const row of chains.cases) {
    const edit = row.edit ?? {
      ...row.header,
      forwards: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index + 1 } })),
      inverse: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index } })),
    };
    assert.equal(editDigest(edit).toString("hex"), row.expectedDigest, `edit digest vector ${row.name}`);
  }
}
//#endregion 🧵️CanonicalEditOracle"""
CHAINS_LAW_OLD = """            SnapshotRetirementStep::Blocked => panic!("positive retirement grant blocked"),
        }
    }
    panic!("final authority strings did not retire");
}
"""
CHAINS_LAW_NEW = CHAINS_LAW_OLD + """
/// 🔗️ LAW (ticket 26/09/23 C12): the revision digest rule — a single-operation edit hashes its canonical JSON, an edit grown
/// past one operation hashes its header fields and one running chain per operation list — matches the language-neutral
/// vectors (derived by a third implementation, replayed by the TS oracle too), and extending the chains of a grown edit by the
/// operations an amend appended equals its from-scratch digest.
#[test]
fn edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️edit-digest-chains.json")).unwrap();
    let edit_of = |case: &serde_json::Value| match case.get("edit") {
        Some(edit) => Edit::<DslValue>::from_value(edit.clone().into()).unwrap(),
        None => {
            let mut header = case["header"].clone();
            let count = case["generatedOperations"].as_u64().unwrap();
            header["forwards"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index + 1 } })).collect();
            header["inverse"] = (0..count).map(|index| serde_json::json!({ "SetN": { "n": index } })).collect();
            Edit::<DslValue>::from_value(header.into()).unwrap()
        }
    };
    for case in fixture["cases"].as_array().unwrap() {
        let digest = super::super::CursorRevisionAccumulator::edit_digest(&edit_of(case));
        assert_eq!(semio_framework_hash::hex_lower(&digest), case["expectedDigest"].as_str().unwrap(), "edit digest vector {}", case["name"]);
    }
    let mut grown = edit_of(&fixture["cases"][1]);
    let (_, chains) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, None);
    grown.forwards.push(serde_json::json!({ "SetN": { "n": 3 } }).into());
    grown.inverse.push(serde_json::json!({ "SetN": { "n": 2 } }).into());
    let (extended, _) = super::super::CursorRevisionAccumulator::edit_digest_extending(&grown, chains);
    assert_eq!(extended, super::super::CursorRevisionAccumulator::edit_digest(&grown), "an amend's incremental digest equals the from-scratch digest");
}
"""

EXPORT_OLD = """                    op_lines: entry.op_lines.clone(),
                    applied: entry.applied,"""
EXPORT_NEW = """                    op_lines: entry.op_lines.clone(),
                    op_count: entry.op_count as u64,
                    applied: entry.applied,"""
KERNEL_ENTRY_OLD = """    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,"""
KERNEL_ENTRY_NEW = """    /// 📜️ The newest forward operations of this row's edit, newest last — a bounded preview; `op_count` says how many the
    /// edit holds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    /// 🔢️ Forward operations of this row's edit; more than `op_lines` holds means the preview omits the older ones.
    #[serde(default)]
    #[value(default)]
    pub op_count: u64,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,"""
KERNEL_TS_OLD = """  readonly opLines?: readonly string[];
  readonly applied?: boolean;"""
KERNEL_TS_NEW = """  /** 📜️ The newest forward operations of this row's edit, newest last — a bounded preview. */
  readonly opLines?: readonly string[];
  /** 🔢️ Forward operations of this row's edit; more than `opLines` holds means the preview omits the older ones. */
  readonly opCount?: number;
  readonly applied?: boolean;"""
SHELL_HOST_OLD = """                description: entry.opLines?.join(" · "),"""
SHELL_HOST_NEW = """                description: entry.opLines && (entry.opCount ?? 0) > entry.opLines.length ? ["…", ...entry.opLines].join(" · ") : entry.opLines?.join(" · "),"""

DAG_OLD = 'document=child_id=document-76a05fef01e1c7ae target="document-76a05fef01e1c7ae!s.stdio.semio@v1/document"\n'
DAG_NEW = 'document=child_id=document-21ed2f21032d3ec2 target="document-21ed2f21032d3ec2!s.stdio.semio@v1/document"\n'

HUNKS = {
    SDK: [
        (LOAD_OLD_TEXT, LOAD_NEW_TEXT),
        (LOAD_OLD_PACK, LOAD_NEW_PACK),
        (HELPER_ANCHOR, HELPER + HELPER_ANCHOR),
        (TICK_OLD, TICK_NEW),
        (FOLD_OLD, FOLD_NEW),
        (STAGES_OLD, STAGES_NEW),
        (ROOT_STAGE_OLD, ROOT_STAGE_NEW),
        (MEMBER_STAGE_OLD, MEMBER_STAGE_NEW),
        (STEP_OLD, STEP_NEW),
        (PRESSURE_OLD, PRESSURE_NEW),
        (CHILD_PRESSURE_ANCHOR, CHILD_PRESSURE + CHILD_PRESSURE_ANCHOR),
        (LEN_OLD, LEN_NEW),
        (TRAIT_DOC_OLD, TRAIT_DOC_NEW),
        (DRAIN_DOC_OLD, DRAIN_DOC_NEW),
        (ADMIT_OLD, ADMIT_NEW),
        (CHILD_ROOT_ANCHOR, CHILD_ROOT_FN + CHILD_ROOT_ANCHOR),
        (RETIRE_CHECK_OLD, RETIRE_CHECK_NEW),
        (PREVIEW_CONST_OLD, PREVIEW_CONST_NEW),
        (PREVIEW_DECL_ANCHOR, PREVIEW_DECL + PREVIEW_DECL_ANCHOR),
        (PRINTED_OLD, PRINTED_NEW),
        (ROW_OLD, ROW_NEW),
        (ROW_FIELD_OLD, ROW_FIELD_NEW),
        (ARM_OLD, ARM_NEW),
        (ABORT_CHECK_OLD, ABORT_CHECK_NEW),
        (PERMIT_ANCHOR, PERMIT_WAIT + PERMIT_ANCHOR),
        (FOLLOW_DOC_OLD, FOLLOW_DOC_NEW),
        (RETIRE_LOOP_OLD, RETIRE_LOOP_NEW),
        (FOLLOW_END_OLD, FOLLOW_END_NEW),
        (FIELD_OLD, FIELD_NEW),
        (INIT_OLD, INIT_NEW),
        (PENDING_OLD, PENDING_NEW),
        (RUNNABLE_OLD, RUNNABLE_NEW),
        (EXPORT_OLD, EXPORT_NEW),
    ],
    KERNEL: [(KERNEL_ENTRY_OLD, KERNEL_ENTRY_NEW)],
    KERNEL_TS: [(KERNEL_TS_OLD, KERNEL_TS_NEW)],
    SHELL_HOST: [(SHELL_HOST_OLD, SHELL_HOST_NEW)],
    DAG: [(DAG_OLD, DAG_NEW)],
    SDK_LAWS: [
        (SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW),
        (HISTORY_LAW_ANCHOR, HISTORY_LAW_ANCHOR + HISTORY_LAW),
        ('\n                    op_lines: vec!["set-count value=1".into()],\n', '\n                    op_lines: vec!["set-count value=1".into()],\n                    op_count: 1,\n'),
        ("\n                    op_lines: Vec::new(),\n", "\n                    op_lines: Vec::new(),\n                    op_count: 0,\n"),
        ('\n                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],\n', '\n                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],\n                op_count: 1,\n'),
        ("\n            op_lines: Vec::new(),\n", "\n            op_lines: Vec::new(),\n            op_count: 0,\n", "every"),
    ],
    PANEL_KIT_LAWS: [("\n            op_lines: Vec::new(),\n", "\n            op_lines: Vec::new(),\n            op_count: 0,\n", "every")],
    CANONICAL_SCHEMA: [(CHAINS_SCHEMA_OLD, CHAINS_SCHEMA_NEW)],
    CANONICAL_TS: [(CHAINS_TS_OLD, CHAINS_TS_NEW)],
    CANONICAL_LAWS: [(CHAINS_LAW_OLD, CHAINS_LAW_NEW)],
    PREVIEW_EVAL_LAWS: [("\n        op_lines: Vec::new(),\n", "\n        op_lines: Vec::new(),\n        op_count: 0,\n")],
    STORE: [(DIGEST_OLD, DIGEST_NEW), (NEW_OLD, NEW_NEW), (EDIT_DIGEST_OLD, EDIT_DIGEST_NEW), (INIT_ACC_OLD, INIT_ACC_NEW), (BUILD_ACC_OLD, BUILD_ACC_NEW), (RETIRED_ACC_OLD, RETIRED_ACC_NEW)],
    DURABLE: [(DURABLE_ACC_OLD, DURABLE_ACC_NEW)],
    STORE_LAWS: [(STORE_LAW_ANCHOR, STORE_LAW_ANCHOR + STORE_LAW)],
    WRITER_LAWS: [(WRITER_LAW_ANCHOR, WRITER_LAW_ANCHOR + WRITER_LAW)],
}

problems, writes = [], {}
CREATES = {CANONICAL_FIXTURE: CHAINS_FIXTURE}
for relative, content in CREATES.items():
    path = os.path.join(REPO, relative)
    if os.path.exists(path) and open(path, encoding="utf-8").read() != content:
        problems.append((relative.rsplit("/", 1)[-1], "exists with other content", 0))
    elif not os.path.exists(path):
        writes[path] = content
    print(relative.rsplit("/", 1)[-1], ["present" if os.path.exists(path) else "create"])
for relative, hunks in HUNKS.items():
    path = os.path.join(REPO, relative)
    text = open(path, encoding="utf-8").read()
    plan = []
    for old, new, *every in hunks:
        if new in text:
            plan.append("present")
            continue
        if text.count(old) != 1 and not (every and text.count(old) > 1):
            problems.append((relative.rsplit("/", 2)[-2], old[:70], text.count(old)))
            continue
        text = text.replace(old, new)
        plan.append("hunk")
    writes[path] = text
    print(relative.rsplit("/", 2)[-2], plan)
print(f"{len(problems)} problems {problems}", "mode=apply" if APPLY else "mode=dry-run")
if problems:
    sys.exit(1)
if APPLY:
    for path, text in writes.items():
        open(path, "w", encoding="utf-8").write(text)
