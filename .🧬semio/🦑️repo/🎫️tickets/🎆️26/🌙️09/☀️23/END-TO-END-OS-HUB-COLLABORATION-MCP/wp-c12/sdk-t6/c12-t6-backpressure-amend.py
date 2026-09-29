"""⏳️ C12: one-shot, idempotent amendment of `c12-sdk-composition-patch.py` (coordinator 08:4x, P1): child retirements get BOUNDED
BACKPRESSURE on top of the pressure fix — a full retirement registry makes the next admission wait on maintenance (bounded
synchronous reclaim; the follow pass and the mounted child publication defer in order) instead of a non-retryable fault — plus
the 10 000-publication SDK law and the 10 000-keystroke writer law. Usage: python3 c12-t6-backpressure-amend.py"""
import os

PATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "c12-sdk-composition-patch.py")
text = open(PATCH, encoding="utf-8").read()
MARK = "ADMIT_OLD = "
if MARK in text:
    print("already amended")
    raise SystemExit(0)


def swap(old, new):
    global text
    assert text.count(old) == 1, (old[:80], text.count(old))
    text = text.replace(old, new)


swap('''3. **Child retirements join maintenance pressure** (law `a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step`,
   red since T14's `s14b-s14c-h5-OWNERS-1X` 09-28 20:23; trinity jack's twin law times out the same way): an app that
   re-points a slot at a new content-addressed child per edit admits one child-root AND one child-member retirement per
   keystroke, but `maintenance_under_pressure` only counted the stores' displaced-owner queues, so the 64 fixed slots drained
   at one step per 26-stage rotation and the 65th keystroke faulted `interactive-job.child-root-retirement-saturated`
   (non-retryable — a fast typist or a paste burst kills the live document the same way). Now both registries report
   pressure at a quarter occupancy and a pressured `maintenance_step` spends its step on their stages first (falling through
   to the rotation when both are blocked, since what unblocks them — returned snapshot reads — lives there).
''', '''3. **Child retirements: pressure + bounded backpressure, never a saturation fault** (law
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
   bound); writer `a_ten_thousand_keystroke_burst_applies_every_key_in_order` (10 000 keys with corrections, no pressure
   drain between keys, projection equals the model after every key, pressure drains afterwards).
''')

swap('''    pub(crate) const MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY: usize = ARTIFACT_LIVE_OUTPUT_SLOTS / 4;
"""''', '''    pub(crate) const MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY: usize = ARTIFACT_LIVE_OUTPUT_SLOTS / 4;
    /// ⏳️ Rounds one child publication spends reclaiming the retirements ahead of it before it waits instead.
    pub(crate) const CHILD_RETIREMENT_RECLAIM_STEPS: usize = ARTIFACT_LIVE_OUTPUT_SLOTS * 16;
"""''')

swap('''        /// composition. A lane naming a child the parent no longer declares is a superseded content address (a
        /// peer's lane for the child its own later edit — or ours — replaced): derivable from the parent, it
        /// carries nothing the composition still holds and is skipped.
''', '''        /// composition. A lane naming a child the parent no longer declares is a superseded content address (a
        /// peer's lane for the child its own later edit — or ours — replaced), and a lane naming a child the
        /// follow pass has not opened yet because it waits on retirements is the child that pass derives: both
        /// carry nothing the composition does not already hold and are skipped.
''')
swap('''                    if (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared.to_string() == slot && fields.child_id.to_string() == child_id) {
''', '''                    if !self.derivable_follow_awaits_retirement() && (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared.to_string() == slot && fields.child_id.to_string() == child_id) {
''')

swap('''        fn child_retirements_under_pressure(&self) -> bool {
            self.child_content_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY || self.child_member_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY
        }

''', '''        fn child_retirements_under_pressure(&self) -> bool {
            self.child_content_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY || self.child_member_retirements.len() >= MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY
        }

        /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
        /// work until maintenance returned them (see `child_follow_awaits_retirement`).
        fn derivable_follow_awaits_retirement(&self) -> bool {
            self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
        }

''')

NEW_HUNKS = r'''
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
ADMIT_NEW = """        /// ♻️ Whether the next `roots` child-content and `members` child-member retirement generations each find a vacant
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
                let mut released = false;
                for stage in [MAINTENANCE_CHILD_ROOT_STAGE, MAINTENANCE_CHILD_MEMBER_STAGE] {
                    let step = self.maintenance_stage_step(stage, 1, crate::plugin_runtime::RUNTIME_CLOSE_BYTES_PER_STEP)?;
                    released |= matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0);
                }
                if !released {
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
/// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): a 10 000-keystroke burst at the maximum rate — one full-text `text-edit` per
/// key with corrections, no pressure drain between keys — applies every key in order and never faults. Every key re-points
/// the document slot at a new content-addressed child and so admits child-root and child-member retirements faster than the
/// maintenance rotation returns them; publication and the follow pass wait on those retirements (the 65th key answered
/// `interactive-job.child-root-retirement-saturated` before). Afterwards maintenance returns them below their pressure bound.
#[semio_framework_async_macros::async_test]
async fn a_ten_thousand_keystroke_burst_applies_every_key_in_order() {
    const KEYS: [char; 8] = ['a', 'q', 'ß', 'ü', '€', '𝄞', ' ', '\\n'];
    let mut app = new_app().await;
    let mut model = String::new();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
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
        dispatch(&mut app, WriterCommand::TextEdit(super::TextEdit { text: model.clone() })).await;
        assert_eq!(writer_text(&app.snapshot().expect("projection")), model, "key {index} applies in order");
    }
    semio_framework_plugin::artifact_app_laws::drain_maintenance_pressure(&mut *app);
    assert!(!PluginApp::maintenance_under_pressure(&*app), "maintenance returns the burst's retirements below their pressure bound");
}
"""
'''
swap('DAG_OLD = ', NEW_HUNKS.lstrip("\n") + '\nDAG_OLD = ')

swap('''DAG = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio"
''', '''DAG = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio"
SDK_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
WRITER_LAWS = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs"
''')

swap('''        (DRAIN_DOC_OLD, DRAIN_DOC_NEW),
    ],
    DAG: [(DAG_OLD, DAG_NEW)],
}''', '''        (DRAIN_DOC_OLD, DRAIN_DOC_NEW),
        (ADMIT_OLD, ADMIT_NEW),
        (RETIRE_CHECK_OLD, RETIRE_CHECK_NEW),
        (ABORT_CHECK_OLD, ABORT_CHECK_NEW),
        (PERMIT_ANCHOR, PERMIT_WAIT + PERMIT_ANCHOR),
        (FOLLOW_DOC_OLD, FOLLOW_DOC_NEW),
        (RETIRE_LOOP_OLD, RETIRE_LOOP_NEW),
        (FOLLOW_END_OLD, FOLLOW_END_NEW),
        (FIELD_OLD, FIELD_NEW),
        (INIT_OLD, INIT_NEW),
        (PENDING_OLD, PENDING_NEW),
        (RUNNABLE_OLD, RUNNABLE_NEW),
    ],
    DAG: [(DAG_OLD, DAG_NEW)],
    SDK_LAWS: [(SDK_LAW_ANCHOR, SDK_LAW_ANCHOR + SDK_LAW)],
    WRITER_LAWS: [(WRITER_LAW_ANCHOR, WRITER_LAW_ANCHOR + WRITER_LAW)],
}''')

open(PATCH, "w", encoding="utf-8").write(text)
print("amended")
