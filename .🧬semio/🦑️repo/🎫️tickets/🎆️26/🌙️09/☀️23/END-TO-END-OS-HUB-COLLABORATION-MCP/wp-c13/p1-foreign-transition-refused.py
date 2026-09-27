#!/usr/bin/env python3
"""⏪️ C13 prepared set P1 (row 3.11, guest-linked → window 3): an undo belongs to its author.

The history fold (`📡️replication/🔗️causal/🔀️transition`) withdrew/restored whatever operations a `Revert`/`Reinstate`
named, whoever sent it: another actor's crafted undo silently removed a peer's edit on every replica. After this set a
transition naming any operation another actor authored changes nothing and is listed in `HistoryFold.refused`.

Touches: the fold (Rust), its unit laws, the language-agnostic durable-collaborative-redo schema + fixture (every expect
gains `refused`; new foreign-revert/mixed/foreign-reinstate steps), the TS reference twin, and the store's exhaustive
retire of `HistoryFold`. Idempotent: every hunk is either applied or already applied.

usage: p1-foreign-transition-refused.py [--root <repo or overlay>] (--dry-run | --apply)"""
import json
import pathlib
import sys

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1]) if "--root" in args else pathlib.Path("/Users/ueli/Documents/semio")
apply = "--apply" in args
if apply == ("--dry-run" in args):
    sys.exit("usage: p1-foreign-transition-refused.py [--root <dir>] (--dry-run | --apply)")

REPLICATION = "🧰️framework/🔨️modules/📡️replication"
FOLD = f"{REPLICATION}/🔗️causal/🔀️transition/🦀️.rs"
FOLD_TESTS = f"{REPLICATION}/🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = f"{REPLICATION}/🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1/🔣️.json"
SCHEMA = f"{REPLICATION}/🔗️causal/🧬️schema/🗄️durable-collaborative-redo-v1/🔣️.json"
TWIN = f"{REPLICATION}/🧪️tests/🗄️durable-collaborative-redo/🟦️.ts"
RETIRE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs"

HUNKS: dict[str, list[tuple[str, str]]] = {
    FOLD: [
        (
            """/// @emoji 🧮️ Everything a document's history projects to: the active edits in HLC order, the redo
/// stack, the current checkpoint and alternative, and every change/checkpoint/alternative fact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryFold {
    pub applied: Vec<String>,
    pub redo: Vec<String>,
    pub checkpoint: Option<String>,""",
            """/// @emoji 🧮️ Everything a document's history projects to: the active edits in HLC order, the redo
/// stack, the undo/redo transitions refused because they name an operation another actor authored
/// (`refused`, transition ids in fold order), the current checkpoint and alternative, and every
/// change/checkpoint/alternative fact.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryFold {
    pub applied: Vec<String>,
    pub redo: Vec<String>,
    pub refused: Vec<String>,
    pub checkpoint: Option<String>,""",
        ),
        (
            """enum FoldEvent<'a> {
    Edit(&'a FoldEdit),
    Transition(HistoryTransition),
}""",
            """enum FoldEvent<'a> {
    Edit(&'a FoldEdit),
    Transition { id: &'a str, actor: &'a str, transition: HistoryTransition },
}""",
        ),
        (
            """/// events derive the same projection whatever order they received them in. `excluded` names edits
/// withheld by a merge policy (quarantined); they never become active.
""",
            """/// events derive the same projection whatever order they received them in. `excluded` names edits
/// withheld by a merge policy (quarantined); they never become active. An undo belongs to its author: a
/// `Revert` or `Reinstate` naming any operation another actor authored withdraws or restores nothing
/// and is listed in `refused` (an edit whose author is unknown — a local-only one — belongs to anyone).
""",
        ),
        (
            """events.push((envelope.timestamp.cmp_key(), envelope.mutation_id.0.as_str(), FoldEvent::Transition(transition)));""",
            """events.push((envelope.timestamp.cmp_key(), envelope.mutation_id.0.as_str(), FoldEvent::Transition { id: envelope.mutation_id.0.as_str(), actor: envelope.actor.0.as_str(), transition }));""",
        ),
        (
            """        Ok(edit_ids)
    };
    let mut fold = HistoryFold::default();""",
            """        Ok(edit_ids)
    };
    let foreign = |edit_ids: &[String], actor: &str| edit_ids.iter().any(|edit_id| authors.get(edit_id.as_str()).copied().flatten().is_some_and(|author| author != actor));
    let mut fold = HistoryFold::default();""",
        ),
        (
            """            FoldEvent::Transition(HistoryTransition::Revert { mutation_ids }) => {
                for edit_id in owned(&mutation_ids)? {
                    if active.remove(&edit_id) {""",
            """            FoldEvent::Transition { id, actor, transition: HistoryTransition::Revert { mutation_ids } } => {
                let edit_ids = owned(&mutation_ids)?;
                if foreign(&edit_ids, actor) {
                    fold.refused.push(id.to_string());
                    continue;
                }
                for edit_id in edit_ids {
                    if active.remove(&edit_id) {""",
        ),
        (
            """            FoldEvent::Transition(HistoryTransition::Reinstate { mutation_ids }) => {
                for edit_id in owned(&mutation_ids)? {
                    if let Some(position)""",
            """            FoldEvent::Transition { id, actor, transition: HistoryTransition::Reinstate { mutation_ids } } => {
                let edit_ids = owned(&mutation_ids)?;
                if foreign(&edit_ids, actor) {
                    fold.refused.push(id.to_string());
                    continue;
                }
                for edit_id in edit_ids {
                    if let Some(position)""",
        ),
        ("FoldEvent::Transition(HistoryTransition::Commit(checkpoint)) => {", "FoldEvent::Transition { transition: HistoryTransition::Commit(checkpoint), .. } => {"),
        ("FoldEvent::Transition(HistoryTransition::Branch { alternative_id, name, checkpoint_id }) => {", "FoldEvent::Transition { transition: HistoryTransition::Branch { alternative_id, name, checkpoint_id }, .. } => {"),
        ("FoldEvent::Transition(HistoryTransition::Checkout { checkpoint_id, alternative_id }) => {", "FoldEvent::Transition { transition: HistoryTransition::Checkout { checkpoint_id, alternative_id }, .. } => {"),
        ("FoldEvent::Transition(HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }) => {", "FoldEvent::Transition { transition: HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }, .. } => {"),
    ],
    FOLD_TESTS: [
        (
            """/// 🚩️ Commit materializes change/checkpoint facts; checkout restores exactly the checkpoint's edits.""",
            """/// 🛂️ An undo belongs to its author: a revert or reinstate naming another actor's operation — alone or beside the
/// actor's own — withdraws or restores nothing and is listed as refused, on every replica.
#[test]
fn a_transition_naming_another_actors_operation_is_refused_whole() {
    let edits = [edit("a", "x", 1), edit("b", "y", 2)];
    let foreign = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "y", 3);
    let mixed = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into()), MutationId("op-b".into())] }, "y", 4);
    let fold = fold_history(&edits, &[foreign.clone(), mixed.clone()], &none()).expect("fold");
    assert_eq!(fold.applied, vec!["a", "b"]);
    assert!(fold.redo.is_empty());
    assert_eq!(fold.refused, vec![foreign.mutation_id.0.clone(), mixed.mutation_id.0.clone()]);
    let own = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "x", 5);
    let stolen = at(HistoryTransition::Reinstate { mutation_ids: vec![MutationId("op-a".into())] }, "y", 6);
    let fold = fold_history(&edits, &[own, stolen.clone()], &none()).expect("fold");
    assert_eq!(fold.applied, vec!["b"]);
    assert_eq!(fold.redo, vec!["a"]);
    assert_eq!(fold.refused, vec![stolen.mutation_id.0.clone()]);
}

/// 🚩️ Commit materializes change/checkpoint facts; checkout restores exactly the checkpoint's edits.""",
        ),
        (
            """                assert_eq!(fold.redo, redo, "{}", step["label"].as_str().unwrap_or("redo"));
""",
            """                assert_eq!(fold.redo, redo, "{}", step["label"].as_str().unwrap_or("redo"));
                let refused: Vec<String> = step["expect"]["refused"].as_array().expect("refused").iter().map(|id| id.as_str().expect("id").into()).collect();
                assert_eq!(fold.refused, refused, "{}", step["label"].as_str().unwrap_or("refused"));
""",
        ),
        (
            """    assert!(observations.contains(&"survives-hub-restart"));
}""",
            """    assert!(observations.contains(&"survives-hub-restart"));
    assert!(observations.contains(&"foreign-transition-refused"));
}""",
        ),
        (
            """/// ♻️ Language-agnostic durable collaborative redo: two authors interleave, each undoes/redoes only
/// their own mutations, and reload/`hub-restart` steps re-fold the same event set to the same
/// applied/redo projection (pure durability proof shared with the TypeScript runner).""",
            """/// ♻️ Language-agnostic durable collaborative redo: two authors interleave, each undoes/redoes only
/// their own mutations, another actor's revert/reinstate of them is refused, and reload/`hub-restart`
/// steps re-fold the same event set to the same applied/redo/refused projection (pure durability
/// proof shared with the TypeScript runner).""",
        ),
    ],
    TWIN: [
        (
            """type Expect = Readonly<{ applied: readonly string[]; redo: readonly string[] }>;""",
            """type Expect = Readonly<{ applied: readonly string[]; redo: readonly string[]; refused: readonly string[] }>;""",
        ),
        (
            """/** ♻️ Pure fold twin of Rust `fold_history` for revert/reinstate durability scenarios. */
function foldHistory(edits: readonly Edit[], transitions: readonly TransitionStep[]): { applied: string[]; redo: string[] } {""",
            """/** ♻️ Pure fold twin of Rust `fold_history` for revert/reinstate durability scenarios: an undo belongs to its author, so a
 * transition naming another actor's operation changes nothing and is listed in `refused`. */
function foldHistory(edits: readonly Edit[], transitions: readonly TransitionStep[]): { applied: string[]; redo: string[]; refused: string[] } {""",
        ),
        (
            """  const active = new Set<string>();
  const redo: string[] = [];""",
            """  const active = new Set<string>();
  const redo: string[] = [];
  const refused: string[] = [];""",
        ),
        (
            """    } else if (event.transition.kind === "revert") {
      for (const editId of owned(event.transition.mutationIds)) {""",
            """    } else if (owned(event.transition.mutationIds).some((editId) => authors.get(editId) !== event.transition.actor)) {
      refused.push(event.transition.id);
    } else if (event.transition.kind === "revert") {
      for (const editId of owned(event.transition.mutationIds)) {""",
        ),
        (
            """  return { applied, redo: [...redo] };""",
            """  return { applied, redo: [...redo], refused };""",
        ),
        (
            """          expect(fold.redo, step.label).toEqual([...step.expect.redo]);""",
            """          expect(fold.redo, step.label).toEqual([...step.expect.redo]);
          expect(fold.refused, step.label).toEqual([...step.expect.refused]);""",
        ),
        (
            """      expect(fixture.observations).toContain("survives-hub-restart");""",
            """      expect(fixture.observations).toContain("survives-hub-restart");
      expect(fixture.observations).toContain("foreign-transition-refused");""",
        ),
    ],
    RETIRE: [
        (
            """crate::artifact_retire_struct!(crate::os_spr::HistoryFold { applied, redo, checkpoint, alternative, changes, checkpoints, alternatives });""",
            """crate::artifact_retire_struct!(crate::os_spr::HistoryFold { applied, redo, refused, checkpoint, alternative, changes, checkpoints, alternatives });""",
        ),
    ],
}


def fixture_after(fixture: dict) -> dict:
    """🧫️ Every expect gains `refused`; the foreign steps land after the final reload, then survive a hub restart."""
    steps = [dict(step, expect=dict(step["expect"], refused=step["expect"].get("refused", []))) if step["kind"] == "expect" else step for step in fixture["steps"]]
    if not any(step.get("id") == "t-b-undo-a1-foreign" for step in steps):
        full = ["a1", "b1", "a2", "b2"]
        steps += [
            {"kind": "revert", "id": "t-b-undo-a1-foreign", "actor": "author-b", "mutationIds": ["op-a1"], "logicalMs": 30},
            {"kind": "expect", "label": "b-cannot-undo-a", "expect": {"applied": full, "redo": [], "refused": ["t-b-undo-a1-foreign"]}},
            {"kind": "revert", "id": "t-b-undo-mixed", "actor": "author-b", "mutationIds": ["op-b1", "op-a2"], "logicalMs": 31},
            {"kind": "expect", "label": "a-mixed-revert-is-refused-whole", "expect": {"applied": full, "redo": [], "refused": ["t-b-undo-a1-foreign", "t-b-undo-mixed"]}},
            {"kind": "revert", "id": "t-a-undo-a2-late", "actor": "author-a", "mutationIds": ["op-a2"], "logicalMs": 32},
            {"kind": "reinstate", "id": "t-b-redo-a2-foreign", "actor": "author-b", "mutationIds": ["op-a2"], "logicalMs": 33},
            {"kind": "expect", "label": "b-cannot-redo-a", "expect": {"applied": ["a1", "b1", "b2"], "redo": ["a2"], "refused": ["t-b-undo-a1-foreign", "t-b-undo-mixed", "t-b-redo-a2-foreign"]}},
            {"kind": "hub-restart", "label": "refusals-survive-hub-restart"},
            {"kind": "expect", "label": "refusals-converged", "expect": {"applied": ["a1", "b1", "b2"], "redo": ["a2"], "refused": ["t-b-undo-a1-foreign", "t-b-undo-mixed", "t-b-redo-a2-foreign"]}},
        ]
    observations = fixture["observations"] + ([] if "foreign-transition-refused" in fixture["observations"] else ["foreign-transition-refused"])
    return dict(fixture, steps=steps, observations=observations)


def schema_after(schema: dict) -> dict:
    """🧬️ `Expect` requires `refused`: the transition ids the fold refused, in fold order."""
    expect = schema["definitions"]["Expect"]
    required = expect["required"] + ([] if "refused" in expect["required"] else ["refused"])
    properties = dict(expect["properties"], refused={"type": "array", "items": {"$ref": "#/definitions/Id"}})
    return dict(schema, definitions=dict(schema["definitions"], Expect=dict(expect, required=required, properties=properties)))


problems = 0
for relative, hunks in HUNKS.items():
    path = root / relative
    text = path.read_text(encoding="utf-8")
    for old, new in hunks:
        if text.count(new) == 1 and text.count(old) == 0:
            state = "already applied"
        elif text.count(old) == 1:
            state = "applies"
            text = text.replace(old, new)
        else:
            state = f"MISSING (old {text.count(old)}, new {text.count(new)})"
            problems += 1
        print(f"{relative}: {state}: {old.strip().splitlines()[0][:90]}")
    if apply and problems == 0:
        path.write_text(text, encoding="utf-8")
for relative, transform in ((FIXTURE, fixture_after), (SCHEMA, schema_after)):
    path = root / relative
    before = json.loads(path.read_text(encoding="utf-8"))
    after = transform(before)
    print(f"{relative}: {'already applied' if after == before else 'applies'}")
    if apply and problems == 0 and after != before:
        path.write_text(json.dumps(after, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"{'APPLIED' if apply and problems == 0 else 'DRY RUN'}: {problems} problem(s)")
sys.exit(1 if problems else 0)
