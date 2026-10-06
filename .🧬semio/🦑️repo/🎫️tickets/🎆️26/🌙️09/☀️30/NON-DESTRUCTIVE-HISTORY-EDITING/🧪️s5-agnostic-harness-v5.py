#!/usr/bin/env python3
"""🧭️ S5-AGNOSTIC harness v5 (found by the stdio family, 18:27: pdf `0 of 60 editable leaves exercised`, gltf/puzzle idle leaves with
"no reason recorded"). Most stdio leaves commit wire witnesses WITHOUT a document, so the census had no case for them although the
representative search already falls back to the shipped documents. Now a leaf without a committed case is swept on the documents the
app ships (`acceptance_example_cases`: its initial document and its examples), the census says how many leaves were reached that way,
and a leaf whose editor offers no number, boolean, option, vector or text input (references, lists, opaque values only) says so
instead of "no reason recorded". Anchored on the exact post-v4 text, count-asserted, one write; idempotent.
Usage: [--apply] [--preview <file>]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "only on a shipped document"

EDITS = [
    ("""acceptance_census::<A, M>(plugin, manifest, fixtures, &withdraw_only).await),""",
     """acceptance_census::<A, M>(plugin, manifest, fixtures, examples, &withdraw_only).await),"""),
    ("""/// aggregate (the kinds of `withdraw_only` aside) and its committed cases under `fixtures`, at most [`ACCEPTANCE_CASES_PER_LEAF`] per
/// leaf.""", """/// aggregate (the kinds of `withdraw_only` aside) and its committed cases under `fixtures` — for a leaf that commits no case with a
/// document, its committed operations on the documents the app ships ([`acceptance_example_cases`]) — at most
/// [`ACCEPTANCE_CASES_PER_LEAF`] per leaf."""),
    ("""async fn acceptance_census<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, withdraw_only: &BTreeSet<String>) -> String""",
     """async fn acceptance_census<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path, examples: &[ExampleSource], withdraw_only: &BTreeSet<String>) -> String"""),
    ("""    let cases = acceptance_cases::<A>(fixtures);
    let leaf_of = |case: &AcceptanceCase<A::Mutation>| protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
    let (mut exercised, mut uncased, mut attempted) = (BTreeSet::new(), Vec::new(), BTreeSet::new());""",
     """    let mut cases = acceptance_cases::<A>(fixtures);
    let leaf_of = |case: &AcceptanceCase<A::Mutation>| protocol::SemanticMutation::<A::Snapshot>::semantics(&case.op).kind;
    let committed: BTreeSet<&str> = cases.iter().map(leaf_of).collect();
    for case in acceptance_example_cases::<A>(fixtures, examples).await {
        match committed.contains(leaf_of(&case)) {
            true => ::protocol::Mutation::<A::Snapshot>::retire_cold(case.op),
            false => cases.push(case),
        }
    }
    let shipped = leaves.iter().filter(|leaf| !committed.contains(**leaf) && cases.iter().any(|case| leaf_of(case) == **leaf)).count();
    let (mut exercised, mut uncased, mut attempted) = (BTreeSet::new(), Vec::new(), BTreeSet::new());"""),
    ("""            for kind in (0..5).filter(|kind| offered[*kind]) {
                offering[kind].insert(leaf);
            }
            let mut wanted: Vec<usize> = (0..5).filter(|kind| offered[*kind] && proven[*kind].is_none()).collect();""",
     """            for kind in (0..5).filter(|kind| offered[*kind]) {
                offering[kind].insert(leaf);
            }
            if !offered.contains(&true) {
                skipped.push(format!("{leaf} ({}): its editor offers no number, boolean, option, vector or text input to change (references, lists or opaque values only)", case.directory));
                continue;
            }
            let mut wanted: Vec<usize> = (0..5).filter(|kind| offered[*kind] && proven[*kind].is_none()).collect();"""),
    ("""        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} without a committed editable case{}{}, {} with cases but not exercised{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        uncased.len(),""",
     """        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} swept only on a shipped document for want of a committed one, {} without any editable case{}{}, {} with cases but not exercised{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        shipped,
        uncased.len(),"""),
]


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if "swept only on a shipped document" in before:
        print("SKIP: already carries harness v5")
        return
    if "left the operation unchanged" not in before:
        sys.exit("ORDER: apply harness v4 first")
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in EDITS:
        after = after.replace(old, new)
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply harness v5 ({len(EDITS)} hunks, +{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
