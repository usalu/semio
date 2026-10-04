#!/usr/bin/env python3
"""🍃️ G12 harness (S4-AGNOSTIC): `assert_history_edits_end_to_end` reports "no parent-lane leaf" for an app whose mutation aggregate has no leaf
at all (a composed parent after design §20.15 — its content is edited on the child lane, covered by `composed_child_history_law!`), instead of
panicking on zero cases; any app WITH leaves keeps the strict search. Anchored, one write, idempotent. Usage: [--apply]."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
OLD = """{
    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => summary,"""
NEW = """{
    if <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.is_empty() {
        return format!("{plugin}: no parent-lane leaf (a composed parent edits its content on the child lane)");
    }
    match acceptance_search::<A, M>(plugin, manifest, fixtures, examples, false).await {
        AcceptanceSearch::Passed(summary) => summary,"""
DOC_OLD = "/// case's summary; panics naming `plugin`, the leaf and the case otherwise.\n"
DOC_NEW = "/// case's summary; an app whose aggregate has no leaf at all says so; panics naming `plugin`, the leaf and the case otherwise.\n"

with open(PATH, encoding="utf-8") as handle:
    before = handle.read()
if "DESCRIPTORS.is_empty()" in before:
    sys.exit("SKIP: already applied")
if before.count(OLD) != 1 or before.count(DOC_OLD) != 1:
    sys.exit(f"ANCHOR: {before.count(OLD)} / {before.count(DOC_OLD)}")
after = before.replace(OLD, NEW).replace(DOC_OLD, DOC_NEW)
if "--apply" not in sys.argv:
    sys.exit("WOULD apply")
with open(PATH, encoding="utf-8") as handle:
    if handle.read() != before:
        sys.exit("RACE")
with open(PATH, "w", encoding="utf-8") as handle:
    handle.write(after)
print("WROTE")
