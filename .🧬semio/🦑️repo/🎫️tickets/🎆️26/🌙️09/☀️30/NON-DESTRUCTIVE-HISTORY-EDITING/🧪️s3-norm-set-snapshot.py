#!/usr/bin/env python3
"""📤️ S3-NORM: the fifteen norm `set-snapshot` commands commit leaf-labelled field mutations against the CURRENT document.

1. Design §20.4/§20.6: the unit test of every `✏️editor/🎮️commands/📤️set-snapshot` stops asserting the hand-written
   `"setSnapshot"` description (the runtime field is being deleted; the row label comes from the leaves).
2. EN 1992, EN 1993 and EN 1999 diffed the payload against an EMPTY document instead of the current one, so a replace of a
   non-empty document appended duplicates (`mutation.duplicate-id`) and never reset a field to its default value. Their
   handlers now diff against `doc.snapshot` like the other twelve, EN 1999's then-dead `from_snapshot_replace` is deleted,
   and their tests prove both laws on a committed vector (self-replace emits nothing; before → after reaches after).

Usage: python3 🧪️s3-norm-set-snapshot.py [--check]
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
ARTIFACTS = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
COMMAND = f"{SUBSET}/✏️editor/🎮️commands/📤️set-snapshot"
CURRENT_BASE = {
    "🏛️en1992": ("En1992", "&En1992Snapshot::default()", "↔️change-member-width"),
    "🔩️en1993": ("En1993", "&En1993Snapshot::default()", "🔥️update-fire-inputs"),
    "🪶️en1999": ("En1999", "&En1999Snapshot::empty()", "➕add-member"),
}
OLD_NAME = "handle_commits_the_payload_document_under_its_action_id"
NEW_NAME = "handle_commits_the_payload_document_as_its_field_mutations"

LAWS = '''
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{vector}/✅apply/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{vector}/✅apply/📸️snapshot/➡️after/🔣️.json");

fn replace(projection: &{p}Snapshot, payload: &{p}Snapshot) -> Vec<{p}Mutation> {{
    let config = NoConfig::default();
    handle(&ReplaceSnapshot {{ snapshot: payload.clone() }}, &ArtifactView::new(projection, &HistoryView::empty()), &ConfigView {{ snapshot: &config, window: None }}).expect("handle").artifact_mutations
}}

#[test]
fn replacing_a_document_by_itself_emits_nothing() {{
    let document: {p}Snapshot = pack::json::from_json_str(BEFORE).expect("before");
    assert_eq!(replace(&document, &document), Vec::<{p}Mutation>::new());
}}

#[test]
fn replacing_a_document_reaches_the_payload() {{
    let before: {p}Snapshot = pack::json::from_json_str(BEFORE).expect("before");
    let after: {p}Snapshot = pack::json::from_json_str(AFTER).expect("after");
    let reached = replace(&before, &after).iter().fold(before.clone(), |document, mutation| {{
        let raised = <{p}Mutation as protocol::Mutation<{p}Snapshot>>::diff(mutation, &document);
        assert!(raised.messages().is_empty(), "{{mutation:?}} raised {{:?}}", raised.messages());
        <crate::{p}Diff as protocol::MutationDiff<{p}Snapshot>>::apply(raised.diff(), &document).expect("apply")
    }});
    assert_eq!(reached, after);
}}
'''


def edit(path: Path, transform, check: bool) -> int:
    text = path.read_text()
    out = transform(text)
    if out == text:
        return 0
    if not check:
        path.write_text(out)
    return 1


def main() -> int:
    check = "--check" in sys.argv
    pending = 0
    for artifact in sorted(p.name for p in ARTIFACTS.iterdir() if (p / COMMAND).is_dir()):
        test = ARTIFACTS / artifact / COMMAND / "🧪️tests/🔬️unit/🦀️.rs"

        def label(text: str) -> str:
            text = re.sub(r'\n    assert_eq!\(emit\.description\.as_deref\(\), Some\("setSnapshot"\)\);', "", text)
            return text.replace(OLD_NAME, NEW_NAME)

        pending += edit(test, label, check)
        if artifact not in CURRENT_BASE:
            continue
        prefix, empty, vector = CURRENT_BASE[artifact]

        def handler(text: str) -> str:
            text = text.replace(f"_doc: &ArtifactView<'_, {prefix}Snapshot>", f"doc: &ArtifactView<'_, {prefix}Snapshot>")
            return text.replace(f"{prefix}Mutation::from_snapshot({empty}, &payload.snapshot)", f"{prefix}Mutation::from_snapshot(doc.snapshot, &payload.snapshot)")

        pending += edit(ARTIFACTS / artifact / COMMAND / "🦀️.rs", handler, check)

        def laws(text: str) -> str:
            text = re.sub(rf"{prefix}Mutation::from_snapshot_replace\(&{prefix}Snapshot::default\(\)\)", f"{prefix}Mutation::from_snapshot(&projection, &{prefix}Snapshot::default())", text)
            text = text.replace(f"{prefix}Mutation::from_snapshot(&{prefix}Snapshot::default(), &{prefix}Snapshot::default())", f"{prefix}Mutation::from_snapshot(&projection, &{prefix}Snapshot::default())")
            return text if "replacing_a_document_by_itself_emits_nothing" in text else text.rstrip("\n") + "\n" + LAWS.format(p=prefix, vector=vector)

        pending += edit(test, laws, check)
        if artifact == "🪶️en1999":
            aggregate = ARTIFACTS / artifact / SUBSET / "🧬️schema/🧬️mutations/🦀️.rs"
            pending += edit(aggregate, lambda text: re.sub(r"    pub fn from_snapshot_replace\(target: &En1999Snapshot\) -> Vec<En1999Mutation> \{\n        Self::from_snapshot\(&En1999Snapshot::empty\(\), target\)\n    \}\n\n", "", text), check)
    print(f"{'pending' if check else 'rewritten'}={pending}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
