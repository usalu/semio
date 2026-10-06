#!/usr/bin/env python3
"""🎚️ S5-AGNOSTIC law v3 (coordinator decision 11:2x, report S5.6 variant C), on top of law v2 in
`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`: after the representative leaf passed, `history_edits_end_to_end` sweeps EVERY
editable leaf of the aggregate over its committed cases (at most 3 per leaf):
- HARD: every input-control kind (number, boolean, option, vector, free text) that the draft editor of some committed operation offers
  a change for must have ONE case that passes the whole scenario drafting an input of that kind; a kind nobody proves fails the law,
  naming the leaves that offer it and why their cases were skipped.
- HARD: a case on which the generic mechanism breaks (`AcceptanceVerdict::Fail`: a verb refused, a head that is no fresh fold, a preview
  with downstream applied, a withdrawal that leaves a trace) fails the law — every such case is listed, not only the first.
- CENSUS, never failing: `N of M editable leaves exercised` (M = the aggregate's leaves not declared `"editable": false`), with the
  leaves that have no committed editable case and the kinds proven, printed as `[history-edit-census]`.
A scenario can be restricted to one control kind (`kind`), the editor's offer is read by a probe (`acceptance_offered`: seed → begin →
read → exit). Anchored on the exact post-v2 text, count-asserted, docstring emojis checked unique, one write; idempotent.
Usage: [--apply] [--preview <file>]"""
import collections, sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "async fn acceptance_census"

EDITS = [
    ("""const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;
""", """const ACCEPTANCE_DOWNSTREAM_PER_LEAF: usize = 2;
/// 🧰️ The input-control kinds a change is derived for, in the order of [`acceptance_change_buckets`].
const ACCEPTANCE_CONTROL_KINDS: [&str; 5] = ["number", "boolean", "option", "vector", "text"];
/// 🗃️ Committed cases the census tries per leaf before the leaf counts as not exercised.
const ACCEPTANCE_CASES_PER_LEAF: usize = 3;
/// 🪙️ Further cases tried for one control kind nobody proved in the leaf sweep.
const ACCEPTANCE_KIND_ATTEMPTS: usize = 12;
"""),
    ("""/// 🎛️ The schema-valid changes the law tries for one draft, most robust first: every number input moved by its step (then to
/// its bounds and their midpoint), every boolean flipped, every option switched, every vector's first axis moved, every free text
/// extended — each a JSON pointer and the value to draft there. Reference, array and opaque inputs are never changed.
pub fn acceptance_changes(inputs: &[ActionArgDef], value: &DslValue) -> Vec<(String, DslValue)> {""",
     """/// 🎛️ The schema-valid changes the law tries for one draft, most robust first ([`acceptance_change_buckets`] flattened): each a
/// JSON pointer and the value to draft there.
pub fn acceptance_changes(inputs: &[ActionArgDef], value: &DslValue) -> Vec<(String, DslValue)> {
    acceptance_change_buckets(inputs, value).into_iter().flatten().take(ACCEPTANCE_CHANGES_PER_LEAF).collect()
}

/// 🪣️ The schema-valid changes of one draft per input-control kind ([`ACCEPTANCE_CONTROL_KINDS`]): every number input moved by its
/// step (then to its bounds and their midpoint), every boolean flipped, every option switched, every vector's first axis moved,
/// every free text extended — each a JSON pointer and the value to draft there. Reference, array and opaque inputs are never changed.
pub fn acceptance_change_buckets(inputs: &[ActionArgDef], value: &DslValue) -> [Vec<(String, DslValue)>; 5] {"""),
    ("""    buckets.into_iter().flatten().take(ACCEPTANCE_CHANGES_PER_LEAF).collect()
}
""", """    buckets
}
"""),
    # drive
    ("""/// downstream mutation. Answers the change drafted.
async fn acceptance_drive<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    target: &str,
    store: Option<&str>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: Option<&(DslValue, String)>,
) -> Result<(String, DslValue), AcceptanceVerdict>""",
     """/// downstream mutation. With `kind` (and no `change`) only changes of that control kind are drafted. Answers the change drafted.
#[allow(clippy::too_many_arguments)]
async fn acceptance_drive<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    target: &str,
    store: Option<&str>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: Option<&(DslValue, String)>,
    kind: Option<usize>,
) -> Result<(String, DslValue), AcceptanceVerdict>"""),
    ("""    let changes = change.map_or_else(|| acceptance_changes(&inputs, &original), |change| vec![change.clone()]);""",
     """    let changes = match (change, kind) {
        (Some(change), _) => vec![change.clone()],
        (None, Some(kind)) => acceptance_change_buckets(&inputs, &original).into_iter().nth(kind).unwrap_or_default().into_iter().take(ACCEPTANCE_CHANGES_PER_LEAF).collect(),
        (None, None) => acceptance_changes(&inputs, &original),
    };"""),
    ("""        return Err(AcceptanceVerdict::Skip(format!("no schema-valid change of its {} input(s) is accepted", inputs.len())));""",
     """        return Err(AcceptanceVerdict::Skip(format!("no schema-valid{} change of its {} input(s) is accepted", kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])), inputs.len())));"""),
    # session
    ("""async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>, as_of: &(DslValue, String)) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>""",
     """async fn acceptance_session<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: &(DslValue, String),
    kind: Option<usize>,
) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>"""),
    ("""    let drafted = acceptance_drive(app, &target, None, change, alternative, Some(as_of)).await?;""",
     """    let drafted = acceptance_drive(app, &target, None, change, alternative, Some(as_of), kind).await?;"""),
    # scenario
    ("""/// sessions run on the seeded document after it was saved and loaded into a fresh instance ([`acceptance_reloaded`]).
async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>, reload: bool) -> AcceptanceVerdict""",
     """/// sessions run on the seeded document after it was saved and loaded into a fresh instance ([`acceptance_reloaded`]). With `kind`
/// the overwrite drafts an input of that control kind only ([`ACCEPTANCE_CONTROL_KINDS`]).
async fn acceptance_scenario<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>, downstream: Option<&A::Mutation>, reload: bool, kind: Option<usize>) -> AcceptanceVerdict"""),
    ("""acceptance_session(&mut overwrite, None, None, &as_of).await),""", """acceptance_session(&mut overwrite, None, None, &as_of, kind).await),"""),
    ("""    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE), as_of).await {""",
     """    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE), as_of, None).await {"""),
    ("""            match acceptance_scenario::<A, M>(manifest, case, next, reload).await {""",
     """            match acceptance_scenario::<A, M>(manifest, case, next, reload, None).await {"""),
    # child lane calls
    ("""        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None, None).await {""",
     """        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None, None, None).await {"""),
    ("""        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE), None).await {""",
     """        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE), None, None).await {"""),
    # the law
    ("""/// case's summary; an app whose aggregate has no leaf at all says so, and so does one whose every leaf is declared withdraw-only
/// (`"editable": false`, design §22.20); panics naming `plugin`, the leaf and the case otherwise.""",
     """/// case's summary; an app whose aggregate has no leaf at all says so, and so does one whose every leaf is declared withdraw-only
/// (`"editable": false`, design §22.20); panics naming `plugin`, the leaf and the case otherwise. After the representative, every
/// editable leaf is swept ([`acceptance_census`]): one passing case per input-control kind and no broken case are law, the number
/// of leaves exercised is a census line appended to the summary."""),
    ("""    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => summary,
        AcceptanceSearch::Failed(failure) => panic!("{failure}"),
        AcceptanceSearch::Exhausted(report) => panic!("{report}"),
    }
}
""", """    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => format!("{summary}\\n[history-edit-census] {}", acceptance_census::<A, M>(plugin, manifest, fixtures, &withdraw_only).await),
        AcceptanceSearch::Failed(failure) => panic!("{failure}"),
        AcceptanceSearch::Exhausted(report) => panic!("{report}"),
    }
}

/// 🔭️ Which input-control kinds ([`ACCEPTANCE_CONTROL_KINDS`]) the draft editor of `case`'s operation offers a change for: the case
/// seeded alone, `historyEditBegin`, the editor's inputs read, `historyEditExit`. `Err` is the verdict that ends the case (its
/// document or operation does not seed: a skip; the editor of an editable operation does not open or close: a broken mechanism).
async fn acceptance_offered<A, M>(manifest: fn() -> App, case: &AcceptanceCase<A::Mutation>) -> Result<[bool; 5], AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let mut app = match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true).await {
        Ok(app) => app,
        Err(AcceptanceSeedFault::Base(reason)) => return Err(AcceptanceVerdict::SkipBase(reason)),
        Err(AcceptanceSeedFault::Operation(_, reason)) => return Err(AcceptanceVerdict::SkipCase(reason)),
    };
    let offered = async {
        let target = app
            .store
            .mutation_ops()
            .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
            .first()
            .map(|op| op.mutation_id.0.clone())
            .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
        acceptance_verb(&mut app, HISTORY_EDIT_BEGIN_ACTION_ID, vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target))]).await.map_err(AcceptanceVerdict::Fail)?;
        let offered = match app.time_travel.editor() {
            None => return Err(AcceptanceVerdict::Fail("historyEditBegin opened no editor".into())),
            Some(editor) if editor.inputs_refused.is_some() => return Err(AcceptanceVerdict::Fail(format!("the editor cannot read the leaf's input schema: {:?}", editor.inputs_refused))),
            Some(editor) => acceptance_change_buckets(&editor.inputs, &editor.value).map(|bucket| !bucket.is_empty()),
        };
        acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
        Ok::<_, AcceptanceVerdict>(offered)
    }
    .await;
    acceptance_close(&mut app).await;
    offered
}

/// 📋️ LAW (design §16.3, session-5 decision): the sweep behind [`assert_history_edits_end_to_end`] over every editable leaf of the
/// aggregate (the kinds of `withdraw_only` aside) and its committed cases under `fixtures`, at most [`ACCEPTANCE_CASES_PER_LEAF`] per
/// leaf. Law: every input-control kind some case's editor offers ([`acceptance_offered`]) has one case that passes the whole
/// scenario drafting an input of that kind, and no case breaks the generic mechanism — it panics naming `plugin`, the kind or every
/// broken case otherwise. Census, never failing: the leaves exercised of the editable ones, those without a committed editable
/// case, and the leaf that proved each kind — the line it answers.
async fn acceptance_census<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, withdraw_only: &BTreeSet<String>) -> String
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let leaves: Vec<&str> = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().map(|descriptor| descriptor.semantic_kind).filter(|kind| !withdraw_only.contains(*kind)).collect();
    let cases = acceptance_cases::<A>(fixtures);
    let leaf_of = |case: &AcceptanceCase<A::Mutation>| protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
    let (mut exercised, mut uncased, mut attempted) = (BTreeSet::new(), Vec::new(), BTreeSet::new());
    let mut offering: [BTreeSet<&str>; 5] = Default::default();
    let mut proven: [Option<&str>; 5] = [None; 5];
    let (mut broken, mut skipped) = (Vec::new(), Vec::new());
    for leaf in leaves.iter().copied() {
        let own: Vec<usize> = cases.iter().enumerate().filter(|(_, case)| leaf_of(case) == leaf).map(|(index, _)| index).take(ACCEPTANCE_CASES_PER_LEAF).collect();
        if own.is_empty() {
            uncased.push(leaf);
        }
        for index in own {
            let case = &cases[index];
            let offered = match acceptance_offered::<A, M>(manifest, case).await {
                Ok(offered) => offered,
                Err(AcceptanceVerdict::Fail(reason)) => {
                    broken.push(format!("{leaf} ({}): {reason}", case.directory));
                    continue;
                }
                Err(verdict) => {
                    skipped.push(format!("{leaf} ({}): {}", case.directory, verdict.text()));
                    continue;
                }
            };
            for kind in (0..5).filter(|kind| offered[*kind]) {
                offering[kind].insert(leaf);
            }
            let mut wanted: Vec<usize> = (0..5).filter(|kind| offered[*kind] && proven[*kind].is_none()).collect();
            if wanted.is_empty() && !exercised.contains(leaf) {
                wanted.extend((0..5).find(|kind| offered[*kind]));
            }
            for kind in wanted {
                attempted.insert((index, kind));
                match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind)).await {
                    AcceptanceVerdict::Pass(_) => {
                        exercised.insert(leaf);
                        proven[kind].get_or_insert(leaf);
                    }
                    AcceptanceVerdict::Fail(reason) => broken.push(format!("{leaf} [{}] ({}): {reason}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory)),
                    verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
                }
            }
            if exercised.contains(leaf) {
                break;
            }
        }
    }
    for kind in 0..5 {
        let pending: Vec<usize> = cases.iter().enumerate().filter(|(index, case)| offering[kind].contains(leaf_of(case)) && !attempted.contains(&(*index, kind))).map(|(index, _)| index).take(ACCEPTANCE_KIND_ATTEMPTS).collect();
        for index in pending {
            if proven[kind].is_some() {
                break;
            }
            let (case, leaf) = (&cases[index], leaf_of(&cases[index]));
            match acceptance_scenario::<A, M>(manifest, case, None, false, Some(kind)).await {
                AcceptanceVerdict::Pass(_) => {
                    exercised.insert(leaf);
                    proven[kind] = Some(leaf);
                }
                AcceptanceVerdict::Fail(reason) => broken.push(format!("{leaf} [{}] ({}): {reason}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory)),
                verdict => skipped.push(format!("{leaf} [{}] ({}): {}", ACCEPTANCE_CONTROL_KINDS[kind], case.directory, verdict.text())),
            }
        }
    }
    let gaps: Vec<String> = (0..5).filter(|kind| proven[*kind].is_none() && !offering[*kind].is_empty()).map(|kind| format!("{} (offered by {})", ACCEPTANCE_CONTROL_KINDS[kind], offering[kind].iter().copied().collect::<Vec<_>>().join(", "))).collect();
    let kinds: Vec<String> = (0..5)
        .map(|kind| match proven[kind] {
            Some(leaf) => format!("{} proven by {leaf}", ACCEPTANCE_CONTROL_KINDS[kind]),
            None if offering[kind].is_empty() => format!("{} not offered", ACCEPTANCE_CONTROL_KINDS[kind]),
            None => format!("{} UNPROVEN", ACCEPTANCE_CONTROL_KINDS[kind]),
        })
        .collect();
    let census = format!(
        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} without a committed editable case{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        uncased.len(),
        if uncased.is_empty() { "" } else { ": " },
        uncased.iter().take(12).copied().collect::<Vec<_>>().join(", "),
        kinds.join(", ")
    );
    for case in cases {
        ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op);
    }
    let shown = skipped.len().min(12);
    assert!(broken.is_empty(), "{plugin}: {} leaf case(s) break the generic mechanism (first {}):\\n{}\\n[history-edit-census] {census}", broken.len(), broken.len().min(12), broken[..broken.len().min(12)].join("\\n"));
    assert!(gaps.is_empty(), "{plugin}: no committed case exercises an input of control kind {} end to end ({} skip(s), first {shown}):\\n{}\\n[history-edit-census] {census}", gaps.join("; "), skipped.len(), skipped[..shown].join("\\n"));
    census
}
"""),
]


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
    if MARKER in before:
        print("SKIP: already carries law v3")
        return
    if "async fn acceptance_withdraw_restore" not in before:
        sys.exit("ORDER: apply law v2 first")
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in EDITS:
        after = after.replace(old, new)
    repeated = {token: count for token, count in collections.Counter(emojis(after)).items() if count > 1}
    if repeated:
        sys.exit(f"EMOJI: repeated docstring emojis {repeated}")
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply law v3 ({len(EDITS)} hunks, +{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
