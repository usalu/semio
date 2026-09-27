#!/usr/bin/env python3
"""♻️ S17 prepared set (2b-1): the tool-run ledger retires every snapshot alias it lets go instead of dropping it.

Measured live 2026-09-27 (serve 6690, `live/gen3d-trap-1.txt`, symbolized wasm stack): procedural generation3d trapped in
`toolRunStart` with `ordered-map root must be explicitly retired before drop` — `ToolRunLedger<EditorApp<Generation3dPlayApp>>`
dropped the last `Arc<Generation3dSnapshot>` (its `FlowHostSnapshot.layout` is an `OrderedMap`) → `Arc::drop_slow` → plain
drop → panic → the instance is dead. The ledger holds `base`/`overlay`/refold snapshots and replaces or drops them in ten
places (a replacing `toolRunStart` retires the previous entry, a rebase swaps `base`, a refold swaps `overlay` and every
intermediate fold result); whenever the store has already retired its own alias, the ledger's is the last one.

Fix (framework, domain-neutral): `ArtifactStore::retire_snapshot_alias` hands one alias to the exact owned-value
retirement the store's own returned reads use (`ReturnedSnapshotReadRetirement`: the last alias retires its snapshot, any
other only drops its count); the ledger queues every alias it lets go (`ToolRunEntry::displaced` → `retired_snapshots`)
and drives one bounded retirement per `retire_step`, like its retired jobs and publications. Law: generation3d editor unit
`a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias` (start → edit → settle the store's own
retirements → start again), red before (the native panic), green after.

Anchored, all-or-nothing. usage: python3 s17-tool-run-snapshot-retirement.py [--write]
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
RUN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs"
OPS = []


def rep(path, old, new, count=1):
    OPS.append((path, old, new, count))


rep(STORE, """    pub fn take_returned_snapshot_read_retirement(&mut self) -> Result<Option<Box<dyn ErasedSnapshotRetirement>>, VcsError>
    where
        P: Sync,
    {
        if !self.snapshot_read_leases.has_returned() {""", """    /// ♻️ Retires one snapshot alias a client held beside the store (a tool run's base or overlay, a folded intermediate):
    /// the last alias hands its snapshot to the exact owned-value retirement the store's own returned reads use, any other
    /// alias only drops its count. A client never drops an alias plainly — a snapshot whose roots must be retired (an
    /// `OrderedMap`) panics on a plain drop of its last owner.
    pub fn retire_snapshot_alias(&self, alias: Arc<P>) -> Result<Box<dyn ErasedSnapshotRetirement>, VcsError>
    where
        P: Sync,
    {
        let Some(factory) = (*self.initial_snapshot_retirement_factory).clone() else {
            return Err(VcsError::ValidationFailed("a snapshot alias retirement requires its exact owned-snapshot retirement factory".into()));
        };
        Ok(Box::new(ReturnedSnapshotReadRetirement::new(alias, factory)))
    }

    pub fn take_returned_snapshot_read_retirement(&mut self) -> Result<Option<Box<dyn ErasedSnapshotRetirement>>, VcsError>
    where
        P: Sync,
    {
        if !self.snapshot_read_leases.has_returned() {""")

rep(RUN, """    finalize: Option<ToolRunFinalize<A>>,
    announced: Option<(ToolRunState, u64, String)>,
    port: ToolRunJobPort,
}

impl<A: ArtifactApp> ToolRunEntry<A> {""", """    finalize: Option<ToolRunFinalize<A>>,
    announced: Option<(ToolRunState, u64, String)>,
    port: ToolRunJobPort,
    /// ♻️ Snapshot aliases this run let go (a replaced base or overlay, a superseded fold result), owed to the store's
    /// alias retirement ([`ToolRunLedger::retire_step`]); never dropped plainly.
    displaced: Vec<Arc<A::Snapshot>>,
}

impl<A: ArtifactApp> ToolRunEntry<A> {""")
rep(RUN, """            || self.refold_is_work()
            || match self.slot.state {""", """            || self.refold_is_work()
            || !self.displaced.is_empty()
            || match self.slot.state {""")
rep(RUN, """    fn begin_refold(&mut self, boundary: bool) {
        self.refold = Some(ToolRunRefold { running: None, cursor: 0, boundary });
    }""", """    fn begin_refold(&mut self, boundary: bool) {
        if let Some(running) = self.refold.replace(ToolRunRefold { running: None, cursor: 0, boundary }).and_then(|refold| refold.running) {
            self.displaced.push(Arc::new(running));
        }
    }

    /// ♻️ Replaces the overlay, owing the previous one to the store's alias retirement.
    fn replace_overlay(&mut self, overlay: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.overlay, overlay);
        self.displaced.push(previous);
    }

    /// ♻️ Replaces the base, owing the previous one to the store's alias retirement.
    fn replace_base(&mut self, base: Arc<A::Snapshot>) {
        let previous = std::mem::replace(&mut self.base, base);
        self.displaced.push(previous);
    }""")
rep(RUN, """            match Self::fold_one(source, &self.provisional[refold.cursor]) {
                Some(next) => refold.running = Some(next),
                None => self.conflicts = self.conflicts.saturating_add(1),
            }""", """            match Self::fold_one(source, &self.provisional[refold.cursor]) {
                Some(next) => {
                    if let Some(previous) = refold.running.replace(next) {
                        self.displaced.push(Arc::new(previous));
                    }
                }
                None => self.conflicts = self.conflicts.saturating_add(1),
            }""")
rep(RUN, """        self.overlay = refold.running.map_or_else(|| Arc::clone(&self.base), Arc::new);
        ToolRunRefoldTurn::Swapped""", """        let overlay = refold.running.map_or_else(|| Arc::clone(&self.base), Arc::new);
        self.replace_overlay(overlay);
        ToolRunRefoldTurn::Swapped""")
rep(RUN, """            for op in &appended {
                match Self::fold_one(running.as_ref().unwrap_or(&self.overlay), op) {
                    Some(next) => running = Some(next),
                    None => self.conflicts = self.conflicts.saturating_add(1),
                }
            }
            if let Some(running) = running {
                self.overlay = Arc::new(running);
            }""", """            for op in &appended {
                match Self::fold_one(running.as_ref().unwrap_or(&self.overlay), op) {
                    Some(next) => {
                        if let Some(previous) = running.replace(next) {
                            self.displaced.push(Arc::new(previous));
                        }
                    }
                    None => self.conflicts = self.conflicts.saturating_add(1),
                }
            }
            if let Some(running) = running {
                self.replace_overlay(Arc::new(running));
            }""")
rep(RUN, """    retired_jobs: Vec<ToolRunJobSlot<A::Config>>,
    retired_publications: Vec<store::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    discarded: Vec<A::Mutation>,
    closing: bool,""", """    retired_jobs: Vec<ToolRunJobSlot<A::Config>>,
    retired_publications: Vec<store::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    /// ♻️ Snapshot aliases owed to the store's alias retirement, and the one being retired now.
    retired_snapshots: Vec<Arc<A::Snapshot>>,
    snapshot_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    discarded: Vec<A::Mutation>,
    closing: bool,""")
rep(RUN, """        Self { next_run: 1, entries: Vec::new(), selected: None, driver: ToolRunDriver::default(), retired_jobs: Vec::new(), retired_publications: Vec::new(), discarded: Vec::new(), closing: false, trace_windows: BTreeMap::new(), ui_dirty: false, document_dirty: false }""", """        Self {
            next_run: 1,
            entries: Vec::new(),
            selected: None,
            driver: ToolRunDriver::default(),
            retired_jobs: Vec::new(),
            retired_publications: Vec::new(),
            retired_snapshots: Vec::new(),
            snapshot_retirement: None,
            discarded: Vec::new(),
            closing: false,
            trace_windows: BTreeMap::new(),
            ui_dirty: false,
            document_dirty: false,
        }""")
rep(RUN, """        !self.retired_jobs.is_empty() || !self.retired_publications.is_empty() || !self.discarded.is_empty() || self.entries.iter().any(|entry| entry.has_pending_work())""", """        !self.retired_jobs.is_empty() || !self.retired_publications.is_empty() || !self.retired_snapshots.is_empty() || self.snapshot_retirement.is_some() || !self.discarded.is_empty() || self.entries.iter().any(|entry| entry.has_pending_work())""")
rep(RUN, """    fn retire_entry(&mut self, entry: ToolRunEntry<A>) {
        let ToolRunEntry { job, provisional, finalize, .. } = entry;
        self.retire_owners(job, provisional, finalize);
    }""", """    fn retire_entry(&mut self, entry: ToolRunEntry<A>) {
        let ToolRunEntry { job, provisional, finalize, base, overlay, refold, mut displaced, .. } = entry;
        self.retired_snapshots.push(base);
        self.retired_snapshots.push(overlay);
        if let Some(running) = refold.and_then(|refold| refold.running) {
            self.retired_snapshots.push(Arc::new(running));
        }
        self.retired_snapshots.append(&mut displaced);
        self.retire_owners(job, provisional, finalize);
    }""")
rep(RUN, """        self.discarded.append(&mut entry.provisional);
        entry.entity_marks.clear();
        entry.entities = Arc::new(BTreeSet::new());
        entry.refold = None;
        entry.overlay = Arc::clone(&entry.base);""", """        self.discarded.append(&mut entry.provisional);
        entry.entity_marks.clear();
        entry.entities = Arc::new(BTreeSet::new());
        if let Some(running) = entry.refold.take().and_then(|refold| refold.running) {
            entry.displaced.push(Arc::new(running));
        }
        let overlay = Arc::clone(&entry.base);
        entry.replace_overlay(overlay);""")
rep(RUN, """        if !self.discarded.is_empty() {
            let count = self.discarded.len().min(maximum_items.max(1)).min(TOOL_RUN_DISCARD_OPS_PER_TURN);""", """        if let Some(retirement) = self.snapshot_retirement.as_mut() {
            return match retirement.close_step(maximum_items.max(1), maximum_bytes).map_err(plugin_sdk_fault)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    self.snapshot_retirement = None;
                    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.snapshot-close"), "tool run snapshot retirement closed without its terminal-empty witness")),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Some(PluginCloseStep::Pending { released_items, released_bytes })),
                store::SnapshotRetirementStep::Blocked => Ok(Some(PluginCloseStep::Blocked { reason: "tool run snapshot retirement is blocked" })),
            };
        }
        for entry in &mut self.entries {
            self.retired_snapshots.append(&mut entry.displaced);
        }
        if let Some(alias) = self.retired_snapshots.pop() {
            self.snapshot_retirement = Some(store.retire_snapshot_alias(alias).map_err(|error| error.into_fault())?);
            return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if !self.discarded.is_empty() {
            let count = self.discarded.len().min(maximum_items.max(1)).min(TOOL_RUN_DISCARD_OPS_PER_TURN);""")
rep(RUN, """        self.entries.is_empty() && self.retired_jobs.is_empty() && self.retired_publications.is_empty() && self.discarded.is_empty()
    }
}
//#endregion 🔖️Ledger""", """        self.entries.is_empty() && self.retired_jobs.is_empty() && self.retired_publications.is_empty() && self.retired_snapshots.is_empty() && self.snapshot_retirement.is_none() && self.discarded.is_empty()
    }
}
//#endregion 🔖️Ledger""")
rep(RUN, """            finalize: None,
            announced: None,
            port: ToolRunJobPort::default(),
        }));
        Ok(ToolRunActionOutcome::Applied(transition.effect))""", """            finalize: None,
            announced: None,
            port: ToolRunJobPort::default(),
            displaced: Vec::new(),
        }));
        Ok(ToolRunActionOutcome::Applied(transition.effect))""")
rep(RUN, """            let entry = selected_entry_mut!(self.tool_runs).expect("refold keeps the slot");
            entry.base = head;
            entry.base_generation = store_generation;
            entry.identity.base_revision = self.store.content_revision();
            entry.trace.rebind(entry.identity);
            if restart {
                entry.overlay = Arc::clone(&entry.base);
            } else {""", """            let entry = selected_entry_mut!(self.tool_runs).expect("refold keeps the slot");
            entry.replace_base(head);
            entry.base_generation = store_generation;
            entry.identity.base_revision = self.store.content_revision();
            entry.trace.rebind(entry.identity);
            if restart {
                let overlay = Arc::clone(&entry.base);
                entry.replace_overlay(overlay);
            } else {""")
rep(RUN, """            entry.base = self.store.snapshot_owner();
            entry.base_generation = store_generation;
            entry.begin_refold(true);""", """            entry.replace_base(self.store.snapshot_owner());
            entry.base_generation = store_generation;
            entry.begin_refold(true);""")
rep(RUN, """            let entry = selected_entry_mut!(self.tool_runs).expect("finalized slot");
            entry.base = head;
            entry.overlay = Arc::clone(&entry.base);
            entry.base_generation = store_generation;""", """            let entry = selected_entry_mut!(self.tool_runs).expect("finalized slot");
            entry.replace_base(head);
            let overlay = Arc::clone(&entry.base);
            entry.replace_overlay(overlay);
            entry.base_generation = store_generation;""")


GEN3D_TESTS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
rep(GEN3D_TESTS, """#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_flow_graph_edits() {""", """/// ♻️ A replacing `toolRunStart` retires the replaced run's snapshots through the store, never by a plain drop. Measured live
/// 2026-09-27 (`s` serve, V1 matrix en, `s13-s17-logs/live/gen3d-trap-1.txt`): after an `addWidget` moved the head and the
/// store retired its own alias, the replaced previewEval run held the last `Arc<Generation3dSnapshot>` and its plain drop
/// aborted the guest with `ordered-map root must be explicitly retired before drop` (`FlowHostSnapshot.layout`). Several
/// edit → re-run rounds give the store's own retirement time to finish before each replacing start.
#[semio_framework_async_macros::async_test]
async fn a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    context::dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() })).await;
    let (_, preview_view) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    for round in 0..4 {
        let effects = app.pending_effects(Some(&preview_view)).await;
        let receipt = context::drive_preview_run(&mut app, &preview_view, &effects).await;
        assert_eq!(receipt.state.as_deref(), Some("finalized"), "round {round}: the read-only run settles and finalizes itself: {receipt:?}");
        context::dispatch(&mut app, Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None })).await;
        context::settle(&mut app).await;
    }
    let effects = app.pending_effects(Some(&preview_view)).await;
    let receipt = context::drive_preview_run(&mut app, &preview_view, &effects).await;
    assert_eq!(receipt.state.as_deref(), Some("finalized"), "the last replacing run finalizes with every earlier run's snapshots retired: {receipt:?}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_flow_graph_edits() {""")


def apply(write):
    texts, problems = {}, []
    for path, old, new, count in OPS:
        if path not in texts:
            texts[path] = (ROOT / path).read_text()
        found = texts[path].count(old)
        if found != count:
            problems.append(f"{path}: expected {count}x got {found}x for {old[:100]!r}")
            continue
        texts[path] = texts[path].replace(old, new)
    print(f"ops={len(OPS)} files={len(texts)} problems={len(problems)}")
    for problem in problems:
        print("PROBLEM", problem)
    if problems:
        return 1
    if write:
        for path, text in texts.items():
            (ROOT / path).write_text(text)
        print("WRITTEN")
    return 0


if __name__ == "__main__":
    sys.exit(apply("--write" in sys.argv))
