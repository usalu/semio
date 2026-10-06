#!/usr/bin/env python3
"""🌋️ S5-AGNOSTIC law v8 — THE CASCADE (design §23.2, goal of 2026-10-06: conflict resolution for every editor), a THIRD test of
`history_edit_acceptance_law!`: `conflict_history_edits_end_to_end`. Artifact-agnostic construction of a history with a dependent: a
committed case's operation followed by ITS OWN INVERSE on its own document (`Mutation::inverse`) — withdrawing a creation leaves its
deletion without a target, withdrawing a deletion leaves its re-creation a duplicate. On the first such history whose replay blocks:
1. `historyEditWithdraw{root}` → accept → the report classifies EVERY applied mutation, the review is Blocked, `nextProblem` names the
   report's first blocking outcome, `historyEditFinalize` is refused;
2. each blocker in turn is withdrawn (the mutation `nextProblem` names) until the review is Ready;
3. the blockers are restored in reverse → the report is the ORIGINAL one again; the root is restored → the session closes, no
   supersession, the head of before (zero trace);
4. withdrawn and resolved again → finalize overwrite → head and reload equal a fresh fold of the effective history (the mutations left);
5. second scenario, fresh instance: an INPUT of the root is edited to another valid value; when that blocks downstream, the blockers
   are resolved the same way and the finalized head and reload equal a fresh fold of the edited root plus the mutations left.
An editor none of whose histories blocks answers "no dependents" (census, not a failure); an aggregate without a parent-lane leaf says
so (child-lane conflicts are not driven). A separate test: a red here never turns the proven laws' verdict.
Anchored on the exact post-v7 text, count-asserted, docstring emojis checked unique, one write; idempotent.
Usage: [--apply] [--preview <file>]"""
import collections, sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"

CONSTS = ("""const ACCEPTANCE_KIND_ATTEMPTS: usize = 12;
""", """const ACCEPTANCE_KIND_ATTEMPTS: usize = 12;
/// 🌋️ Seeded histories the conflict law tries before the editor counts as having no dependents.
const ACCEPTANCE_CASCADE_CASES: usize = 12;
/// 🪜️ Blockers resolved in turn before a blocked review counts as never reaching ready.
const ACCEPTANCE_CASCADE_BLOCKERS: usize = 8;
/// 🧱️ Cases whose history cannot be built that the conflict law passes over before it stops looking.
const ACCEPTANCE_CASCADE_UNBUILT: usize = 60;
""")

LAW = '''/// 🆔️ The ids of `app`'s applied document mutations, in applied order.
fn acceptance_applied_ids<A, M>(app: &VcsArtifactApp<A, M>) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    Ok(app.store.mutation_ops().map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?.iter().map(|op| op.mutation_id.0.clone()).collect())
}

/// ⛔️ Whether a replay outcome blocks finalizing: an error or a fatal (the floor `ReplayReport::blocks_finalize` reads).
fn acceptance_blocks(worst: Option<semio_framework_diagnostic::Severity>) -> bool {
    matches!(worst, Some(semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
}

/// 🧮️ The review after a replay, as the conflict law reads it: the session reviews, its report classifies every mutation of
/// `expected` (one outcome each), the status blocks exactly when an outcome is an error or a fatal, and `nextProblem` names the
/// first such outcome. Answers the outcomes in replay order and whether the review is blocked.
fn acceptance_classified<A, M>(app: &VcsArtifactApp<A, M>, expected: &[String]) -> Result<(Vec<(String, Option<semio_framework_diagnostic::Severity>)>, bool), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("the replay closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing {
        return Err(AcceptanceVerdict::Fail(format!("the replay ends at {:?}, not a review", status.stage)));
    }
    let outcomes: Vec<(String, Option<semio_framework_diagnostic::Severity>)> = match app.time_travel.session().report.as_ref() {
        Some(report) => report.outcomes.iter().map(|outcome| (outcome.mutation_id.0.clone(), outcome.worst)).collect(),
        None => return Err(AcceptanceVerdict::Fail(format!("the review holds no replay report (review {:?}, fault {:?})", status.review, status.fault))),
    };
    if let Some(missing) = expected.iter().find(|id| !outcomes.iter().any(|(classified, _)| classified == *id)) {
        return Err(AcceptanceVerdict::Fail(format!("the replay report classifies {} mutation(s) but not the applied mutation {missing}", outcomes.len())));
    }
    let first = outcomes.iter().find(|(_, worst)| acceptance_blocks(*worst)).map(|(id, _)| id.clone());
    if (status.review == Some(HistoryTimeTravelReview::Blocked)) != first.is_some() || status.blocking != first.is_some() {
        return Err(AcceptanceVerdict::Fail(format!("the review is {:?} (blocking: {}) although the report's first blocking outcome is {first:?}", status.review, status.blocking)));
    }
    let named = status.next_problem.as_ref().map(|problem| problem.mutation_id.clone());
    if named != first {
        return Err(AcceptanceVerdict::Fail(format!("nextProblem names {named:?} although the report's first blocking outcome is {first:?}")));
    }
    Ok((outcomes, first.is_some()))
}

/// ✂️ A history row's Withdraw of the mutation `id` (it opens the session, or stacks into the open review), accepted and replayed to
/// its review. `Ok(false)`: the store's supersede law admits no withdrawal of it, nothing changed.
async fn acceptance_withdraw_accept<A, M>(app: &mut VcsArtifactApp<A, M>, id: &str) -> Result<bool, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))]).await {
        Ok(()) => {}
        Err(refusal) if refusal.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(format!("the withdraw of {id} does not open: {refusal}"))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of {id} is not accepted: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    Ok(true)
}

/// 🧗️ Resolves a blocked review blocker by blocker: the mutation `nextProblem` names is withdrawn, accepted and replayed, until the
/// review is ready. Answers the ids withdrawn, in order; a blocker that cannot be withdrawn, one named twice, or more than
/// [`ACCEPTANCE_CASCADE_BLOCKERS`] of them break the mechanism (design §22.1: a blocking mutation is always resolvable).
async fn acceptance_resolve<A, M>(app: &mut VcsArtifactApp<A, M>, expected: &[String]) -> Result<Vec<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut resolved: Vec<String> = Vec::new();
    loop {
        let (_, blocked) = acceptance_classified(app, expected)?;
        if !blocked {
            return Ok(resolved);
        }
        if resolved.len() >= ACCEPTANCE_CASCADE_BLOCKERS {
            return Err(AcceptanceVerdict::Fail(format!("withdrawing {} blockers in turn does not reach a ready review", resolved.len())));
        }
        let blocker = app.time_travel.status().and_then(|status| status.next_problem).map(|problem| problem.mutation_id).ok_or_else(|| AcceptanceVerdict::Fail("a blocked review names no next problem".into()))?;
        if resolved.contains(&blocker) {
            return Err(AcceptanceVerdict::Fail(format!("nextProblem names {blocker} again after it was withdrawn")));
        }
        if !acceptance_withdraw_accept(app, &blocker).await? {
            return Err(AcceptanceVerdict::Fail(format!("the blocking mutation {blocker} cannot be withdrawn")));
        }
        resolved.push(blocker);
    }
}

/// 🏗️ Finalizes the ready review as an overwrite and compares the result with a fresh fold of `kept` on `case`'s document: the
/// live head and the head the document reloads to must both equal it. `Err` names the departure.
async fn acceptance_overwrite_folds<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, kept: &[&A::Mutation]) -> Result<(), AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the resolved review does not finalize: {refusal}")))?;
    acceptance_verb(app, HISTORY_EDIT_COMMIT_ACTION_ID, vec![("choice", DslValue::String(HISTORY_EDIT_CHOICE_OVERWRITE.to_string()))]).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the resolved review does not commit: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    let head = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    let reloaded = acceptance_reloaded_head(app, manifest).await.map_err(|reason| AcceptanceVerdict::Fail(format!("after the conflict was resolved the finalized document does not survive save and load: {reason}")))?;
    let mut fresh = acceptance_seeded::<A, M>(manifest, &case.base, kept, false).await.map_err(|fault| AcceptanceVerdict::Fail(format!("a fresh fold of the effective history does not seed: {}", fault.reason())))?;
    let fold = acceptance_head(&fresh);
    acceptance_close(&mut fresh).await;
    let fold = fold.map_err(AcceptanceVerdict::Fail)?;
    if head != fold {
        return Err(AcceptanceVerdict::Fail(format!("after the conflict was resolved the overwrite head differs from a fresh fold of the effective history at {}", acceptance_text_difference(&head.1, &fold.1))));
    }
    if reloaded != fold {
        return Err(AcceptanceVerdict::Fail(format!("after the conflict was resolved the document reloads to a head differing from a fresh fold of the effective history at {}", acceptance_text_difference(&reloaded.1, &fold.1))));
    }
    Ok(())
}

/// 🧨️ The withdraw half of the conflict law on `app`, seeded with `seed` (the case's operation, then its inverse): the root is
/// withdrawn and the report classified; a review that blocks is resolved, restored to its original report, taken back without a
/// trace, resolved again and finalized. `Err(Skip)`: withdrawing the root blocks nothing (no dependents).
async fn acceptance_cascade_withdraw<A, M>(app: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation]) -> Result<String, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let ids = acceptance_applied_ids(app)?;
    let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
    if ids.len() != seed.len() {
        return Err(AcceptanceVerdict::SkipCase(format!("its history lists {} applied operations for {} seeded ones", ids.len(), seed.len())));
    }
    let before = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if let Err(reason) = acceptance_reloaded_head(app, manifest).await {
        return Err(AcceptanceVerdict::SkipCase(format!("its seeded history does not survive save and load before any edit ({reason})")));
    }
    if !acceptance_withdraw_accept(app, &root).await? {
        return Err(AcceptanceVerdict::SkipCase("the store's law admits no withdrawal of its operation".into()));
    }
    let restore = |id: &str| vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(id.to_string()))];
    let (original, blocked) = acceptance_classified(app, &ids)?;
    if !blocked {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of the root is not restored: {refusal}")))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
        return Err(AcceptanceVerdict::Skip("withdrawing its operation blocks nothing downstream".into()));
    }
    if acceptance_verb(app, HISTORY_EDIT_FINALIZE_ACTION_ID, Vec::new()).await.is_ok() {
        return Err(AcceptanceVerdict::Fail("a blocked review lets historyEditFinalize through".into()));
    }
    let resolved = acceptance_resolve(app, &ids).await?;
    for blocker in resolved.iter().rev() {
        acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(blocker.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawn blocker {blocker} is not restored: {refusal}")))?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    }
    let (again, _) = acceptance_classified(app, &ids)?;
    if again != original {
        return Err(AcceptanceVerdict::Fail(format!("restoring the {} resolved blocker(s) does not return to the original report: {again:?} instead of {original:?}", resolved.len())));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, restore(root.as_str())).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal of the root is not restored: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    if app.store.supersessions().iter().any(|(id, _)| ids.contains(&id.0)) {
        return Err(AcceptanceVerdict::Fail("restoring every withdrawal left a supersession in the store".into()));
    }
    let after = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if after != before {
        return Err(AcceptanceVerdict::Fail(format!("after every withdrawal was restored the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))));
    }
    if !acceptance_withdraw_accept(app, &root).await? {
        return Err(AcceptanceVerdict::Fail("the root was withdrawn once and cannot be withdrawn again".into()));
    }
    let resolved = acceptance_resolve(app, &ids).await?;
    let kept: Vec<&A::Mutation> = ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op).collect();
    acceptance_overwrite_folds(app, manifest, case, &kept).await?;
    let blocking = original.iter().filter(|(_, worst)| acceptance_blocks(*worst)).count();
    Ok(format!(
        "its withdrawal is classified over {} mutation(s) and blocks {blocking}; nextProblem named each of the {} blocker(s) resolved by withdrawal; restored to the original report and then without a trace; resolved again and finalized: head and reload equal the fresh fold of the {} mutation(s) left",
        original.len(),
        resolved.len(),
        kept.len()
    ))
}

/// 🎚️ The edit half of the conflict law on a fresh instance seeded with `seed`: an input of the root is drafted to another valid
/// value until one blocks the replay downstream; that review is resolved and finalized, and the head and the reload must equal a
/// fresh fold of the edited root followed by the mutations left. `Ok(None)`: no input change of the root conflicts downstream.
async fn acceptance_cascade_edit<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, seed: &[&A::Mutation]) -> Result<Option<String>, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = acceptance_seeded::<A, M>(manifest, &case.base, seed, true).await.map_err(|fault| AcceptanceVerdict::Fail(format!("the history that seeded once does not seed again: {}", fault.reason())))?;
    let outcome = async {
        let ids = acceptance_applied_ids(&app)?;
        let root = ids.first().cloned().ok_or_else(|| AcceptanceVerdict::Fail("the seeded history left no applied operation".into()))?;
        let begin = vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(root.clone()))];
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
        let (inputs, original) = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) => (editor.inputs.clone(), editor.value.clone()),
        };
        for candidate in acceptance_changes(&inputs, &original) {
            if app.time_travel.status().is_none() {
                acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
            }
            if acceptance_verb(&mut app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_err()
                || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
            {
                continue;
            }
            acceptance_verb(&mut app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
            acceptance_pump(&mut app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
            if app.time_travel.status().is_none() {
                continue;
            }
            let (classified, blocked) = acceptance_classified(&app, &ids)?;
            if !blocked {
                acceptance_verb(&mut app, HISTORY_EDIT_RESTORE_ACTION_ID, begin.clone()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("an accepted draft of the root is not restored: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
            let resolved = acceptance_resolve(&mut app, &ids).await?;
            let edited = match app.time_travel.session().accepted_draft(&protocol::MutationId(root.clone())).map(|draft| &draft.replacement) {
                Some(protocol::InputReplacement::Input { payload, .. }) => <A::Mutation as ::protocol::OpBinary>::decode_op(payload).map_err(|error| AcceptanceVerdict::Fail(format!("the root's edited input does not decode: {error:?}")))?,
                Some(_) => return Err(AcceptanceVerdict::Fail("the root's accepted draft is a withdrawal, not its edited input".into())),
                None => return Err(AcceptanceVerdict::Fail("the resolved review no longer holds the root's draft".into())),
            };
            let kept: Vec<&A::Mutation> = std::iter::once(&edited).chain(ids.iter().zip(seed).skip(1).filter(|(id, _)| !resolved.contains(*id)).map(|(_, op)| *op)).collect();
            let folded = acceptance_overwrite_folds(&mut app, manifest, case, &kept).await;
            let left = kept.len() - 1;
            ::protocol::Mutation::<A::Snapshot>::retire_cold(edited);
            folded?;
            return Ok(Some(format!(
                "editing its input {} = {:?} is classified over {} mutation(s) and blocks downstream; {} blocker(s) resolved by withdrawal; finalized: head and reload equal the fresh fold of the edited root and the {left} mutation(s) left",
                candidate.0,
                candidate.1,
                classified.len(),
                resolved.len()
            )));
        }
        if app.time_travel.status().is_some() {
            let _ = acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        }
        Ok::<Option<String>, AcceptanceVerdict>(None)
    }
    .await;
    acceptance_close(&mut app).await;
    outcome
}

/// 🌪️ The conflict scenario for `case`: its operation followed by its own inverse on its own document is the history; the withdraw
/// half ([`acceptance_cascade_withdraw`]) must find a blocked review, then the edit half ([`acceptance_cascade_edit`]) runs on a
/// fresh instance. `Pass` carries both summaries; a skip says why this case shows no conflict; `Fail` is a broken mechanism.
async fn acceptance_cascade<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>) -> AcceptanceVerdict
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut probe = match acceptance_seeded::<A, M>(manifest, &case.base, &[], true).await {
        Ok(app) => app,
        Err(fault) => return AcceptanceVerdict::SkipBase(fault.reason()),
    };
    let inverse = match probe.store.snapshot() {
        Ok(before) => {
            let inverse = ::protocol::Mutation::<A::Snapshot>::inverse(&case.op, &before).map_err(|error| format!("its operation states no inverse on its document: {error:?}"));
            std::mem::forget(before);
            inverse
        }
        Err(error) => Err(format!("its document does not fold: {error:?}")),
    };
    acceptance_close(&mut probe).await;
    let inverse = match inverse {
        Ok(inverse) if !inverse.is_empty() => inverse,
        Ok(_) => return AcceptanceVerdict::SkipCase("its operation has an empty inverse on its document".into()),
        Err(reason) => return AcceptanceVerdict::SkipCase(reason),
    };
    let seed: Vec<&A::Mutation> = std::iter::once(&case.op).chain(&inverse).collect();
    let verdict = match acceptance_seeded::<A, M>(manifest, &case.base, &seed, true).await {
        Err(AcceptanceSeedFault::Base(reason)) => AcceptanceVerdict::SkipBase(reason),
        Err(AcceptanceSeedFault::Operation(0, reason)) => AcceptanceVerdict::SkipCase(reason),
        Err(AcceptanceSeedFault::Operation(_, reason)) => AcceptanceVerdict::SkipCase(format!("its inverse does not seed after it: {reason}")),
        Ok(mut app) => {
            let withdrawn = acceptance_cascade_withdraw(&mut app, manifest, case, &seed).await;
            acceptance_close(&mut app).await;
            match withdrawn {
                Err(verdict) => verdict,
                Ok(summary) => match acceptance_cascade_edit::<A, M>(manifest, case, &seed).await {
                    Ok(Some(edited)) => AcceptanceVerdict::Pass(format!("{summary}; {edited}")),
                    Ok(None) => AcceptanceVerdict::Pass(format!("{summary}; no input change of the root conflicts downstream")),
                    Err(AcceptanceVerdict::Fail(reason)) => AcceptanceVerdict::Fail(format!("the edit scenario: {reason}")),
                    Err(verdict) => AcceptanceVerdict::Pass(format!("{summary}; the edit scenario was skipped: {}", verdict.text())),
                },
            }
        }
    };
    drop(seed);
    for op in inverse {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(op);
    }
    verdict
}

/// 🏛️ LAW (design §23.2, the generic conflict case): on the first history of this app — a committed operation followed by its own
/// inverse, on its own document or a shipped one — whose first mutation, withdrawn, blocks the replay downstream, the conflict is
/// resolved end to end ([`acceptance_cascade`]). Answers that case's summary; an app none of whose histories blocks answers
/// "no dependents" with its census, one without a parent-lane leaf says so; panics naming `plugin`, the leaf and the case when the
/// mechanism breaks.
pub async fn assert_history_conflicts_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource]) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    if <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.is_empty() {
        return format!("{plugin}: no parent-lane leaf (conflicts between child-lane mutations are not driven by this law)");
    }
    let mut cases = acceptance_cases::<A>(fixtures);
    cases.extend(acceptance_example_cases::<A>(fixtures, examples).await);
    let (mut tried, mut unbuilt) = (0, 0);
    let (mut found, mut broken) = (None, None);
    let mut reasons: Vec<String> = Vec::new();
    let mut unloadable = BTreeSet::new();
    for case in &cases {
        if tried >= ACCEPTANCE_CASCADE_CASES || unbuilt >= ACCEPTANCE_CASCADE_UNBUILT {
            break;
        }
        if unloadable.contains(&case.base_key) {
            continue;
        }
        let leaf = protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
        match acceptance_cascade::<A, M>(manifest, case).await {
            AcceptanceVerdict::Pass(summary) => {
                found = Some(format!("{leaf} ({}) — {summary}", case.directory));
                break;
            }
            AcceptanceVerdict::Fail(reason) => {
                broken = Some(format!("{plugin}: the conflict law on {leaf} ({}) breaks the generic mechanism: {reason}", case.directory));
                break;
            }
            AcceptanceVerdict::Skip(reason) => {
                tried += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipCase(reason) => {
                unbuilt += 1;
                reasons.push(format!("{leaf}: {reason}"));
            }
            AcceptanceVerdict::SkipBase(reason) => {
                unbuilt += 1;
                unloadable.insert(case.base_key.clone());
                reasons.push(format!("{leaf}: {reason}"));
            }
        }
    }
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    if let Some(failure) = broken {
        panic!("{failure}");
    }
    match found {
        Some(summary) => format!("{plugin}: {summary}"),
        None => format!(
            "{plugin}: no dependents — {tried} histories of a committed operation and its inverse block nothing when their first mutation is withdrawn, {unbuilt} could not be built (first reasons: {})",
            reasons.iter().take(3).map(|reason| reason.chars().take(160).collect::<String>()).collect::<Vec<_>>().join(" | ")
        ),
    }
}

'''

ANCHOR_BEFORE = "/// ⏳️ Polls the law's future to completion on the calling thread"

MACRO = ("""            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged", $plugin);
        }
    };
}
""", """            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged", $plugin);
        }

        /// ⚔️ LAW (design §23.2): a conflict a history edit of this editor causes downstream is classified, named and resolved end to end.
        #[test]
        fn conflict_history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_conflicts_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[history-edit-cascade] {summary}");
        }
    };
}
""")


def emojis(text):
    found, previous = [], False
    for line in text.splitlines():
        stripped = line.strip()
        doc = stripped.startswith("///") or stripped.startswith("//!")
        if doc and not previous:
            found.append(stripped[3:].strip().split(" ")[0])
        previous = doc
    return found


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if "fn conflict_history_edits_end_to_end" in before:
        print("SKIP: already carries law v8")
        return
    if "the seed gesture {action} does not publish" not in before:
        sys.exit("ORDER: apply harness v7 first")
    for name, old in (("consts", CONSTS[0]), ("insert", ANCHOR_BEFORE), ("macro", MACRO[0])):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR {name}: {before.count(old)} matches")
    after = before.replace(CONSTS[0], CONSTS[1]).replace(ANCHOR_BEFORE, LAW + ANCHOR_BEFORE).replace(MACRO[0], MACRO[1])
    repeated = {token: count for token, count in collections.Counter(emojis(after)).items() if count > 1}
    if repeated:
        sys.exit(f"EMOJI: repeated docstring emojis {repeated}")
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply law v8 (+{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
