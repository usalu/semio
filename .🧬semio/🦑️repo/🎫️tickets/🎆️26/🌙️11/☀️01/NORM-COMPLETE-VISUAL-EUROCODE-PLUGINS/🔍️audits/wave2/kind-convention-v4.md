# Kind convention v4 (wave 2)

Scripts: `🧰️scripts/kind-convention-v4/{verify_fixtures.py,en1993_single_base.py,taxonomy_generate.ts,taxonomy_check.ts}`. Nothing compiled, no cargo, no git writes.

## Result
Fixture check (`python3 -I verify_fixtures.py <artifact>`) is green for all ten: en1990 144/0, en1991 198/0, en1992 154/0, en1993 577/0, en1994 122/0, en1995 187/0, en1996 145/0, en1997 180/0, en1998 273/0, en1999 222/0 (scenarios/failures). `verify build-owners sync` passed. The taxonomy check (`runMutationLeafTaxonomyCheckScript`, run with bun directly) reports "fresh: 2245 payloads".

## 1. Explicit kind metadata
- Taxonomy rows (`📇️mutation-leaf-taxonomy-v1`) gained `edit: {rule, path, selectors:[{kind:id|index|implied, key}], value?|index?|item?|id?} | null`, read by the generator (`norm/🧪️tests/🔬️rust-package-commands`, `editRules`) from the `SetFieldRule`/`InsertItemRule`/`RemoveItemRule`/`IdRemoval` entries of each artifact's `edit-rules` leaf. 2217 of 2245 rows carry it; all ten Eurocode artifacts are covered except en1992 `change-member-order` (a reorder, no edit rule). Missing rules: din16798 2, vdi3805 12, iso16757 12, din4108 1 (out of scope).
- The generator was broken on the current sources: it matched `impl protocol::MutationKind<` but all leaves are `impl<Op> protocol::MutationKind<`, so it produced 0 payloads. Fixed (regex accepts generics); one leaf is still skipped by the generator itself (`en1997/change-structure-id`: descriptor identity mismatch, the en1997 owner's).
- Python engine (`🔮️oracles/🏃️execution/🔧️verbs`): `declare_kinds(rows)`; declared insert/remove/id-removal kinds walk the path (the `[]` segments are consumed by the selectors: row id, position or implied) to the list, apply and invert (the partner kind's own rule picks index vs id) with no name heuristics. This fixes en1992 `bar-layer` (`members[].longitudinal`) and en1993 nested `*-action`/`fatigue-band` (`joints[].actions`, `coldFormedMembers[].actions`, ...). Set-field kinds still use the old heuristics (all vectors pass).

## 2. en1994 designation
`🔮️oracles/🏃️execution/📚️catalogue` reads the section catalogue straight from the Rust source (`SteelSection::heb300` and the `from_catalogue` arms of the en1994 snapshot, 11 sections; no second copy); the engine `declare_catalogue` + `catalogued`: a record whose `designation` changes to a catalogued one takes the catalogue geometry (an unknown designation keeps the stored geometry, like Rust). Limitation: the inverse restores geometry only for a catalogued previous designation.

## 3. en1993 single-base vectors
`en1993_single_base.py`: insert.before is the base; insert.after = engine(insert); remove.before = insert.after; remove.after = base (round trip asserted). Rust read: `insert_*::diff` (duplicate id refused, clamped insertion) and `remove_*::diff` (removal at the index) carry no further semantics, so nothing was hand-fixed. The diff vectors were already consistent. Six snapshots rewritten (frame-bar, frame-node, joint-row; insert after, remove before and after).

## 4. en1992 clear-member-tightness edit rule
`SetFieldRule { path: "members[].tightness", kind: "clear-member-tightness", ..., value: "" }` in the en1992 `edit-rules`: an empty `value` declares the leaf's clear action. Shared `NormEditRules::payload` (app-surface `✏️edit-rules`) picks the rule by write: a `null` write raises the empty-value rule (payload = selectors only), any other write the value rule (fallback for a `null` without clear rule is the old behaviour). New unit test `a_null_write_raises_the_clear_kind_of_a_leaf_that_declares_one`. Doc comment of `SetFieldRule` updated.

## Rebuild risks (first to look at)
`set_rule` closure in `edit-rules` (borrowed iterator, `Option<&SetFieldRule>`), the new app-surface unit test, and the regenerated taxonomy (2245 rows, others' leaves included).
