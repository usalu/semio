# Kind convention v3 (wave 2) - interim

Scripts: `🧰️scripts/kind-convention-v3/{cases_ids.py,id_rules.py}`. Nothing compiled. No git writes.

## 1. Design-case `cases` by id - done
- `remove-case` is id-only in en1990/91/92/95/96/97: payload `{id}`, leaf label/target, diff (`design_case::remove_case(&rows, &id)`), inverse (insert back at the row position), schema json/ts, edit rules (`IdRemoval cases` + `ID_REMOVALS` + `resolve_edit` through `resolve_with_id_removals` in every family; en1992 local `IdRemoval` struct dropped for the shared one), mutation root export `resolve_edit`, editor wiring (macro `resolve = ...::resolve_edit`, commands, case-edit tests), text codecs (en1990, en1992, en1997 DSL enum), proto/graphql (en1995, en1996), unit tests, committed vectors (index replaced by the before row id).
- `insert-case` inverse removes by `case.id`. Shared crate `design-case/📇️rows`: `remove_case(rows, id)` (`UnknownCase` refusal), `CaseRefusal::OutOfRange` removed, unit tests updated. No index selector is left for `cases` anywhere.
- Latent compile break fixed on the way: the live editors of en1991, en1992, en1996 (`norm_live_editor! resolve = ...mutations::resolve`) pointed at a function that does not exist (`resolve_edit` is the export since v2); all six editors now use `resolve_edit`.

## 2. en1994 keyed lanes - done (script `en1994_lists.py`, `en1994_lists_twins.py`)
- Diff: `beams|columns|slabs|placements` and the nested `actions` lanes are `protocol::list_delta!` lanes (`list: Vec<Row>, key: String = by En1994BeamKey`, markers `En1994{Beam,Column,Slab,Placement}Key`, `En1994{Beams,Columns,Slabs}ActionKey` by a local `row_key!`); patches by local `nested_patch!` (set / within / rows) and `scalar_patch!`; generated `En1994<L>{Rows,Removed,Inserted,Moved,Modified}`. Apply = `commit_onto`, absorb = `absorb`, inverse = `inverse`. The hand-written positional modules (2300 lines) are replaced by about 90 lines (old copies in `🗑️generated/kind-convention-v3/backup-en1994-diff`, delete when green).
- 110 kind diffs rewritten (`Rows::modification(&payload.id, Patch {..})`, `::removal(&base.x, index)`, `::insertion(index, row)`, nested the same); the four twins (json compact, ts, graphql, proto) and 115 committed diff vectors (`removed: [{id,index}]`, `modified: [{id,patch}]`, `moved: []`); diff unit tests rewritten for ids.
- Not changed: the parent insert kinds keep `index` (insert position), `remove-*` are id-only.

## 3. en1992 `members[].tightness` - done (`en1992_tightness_diff.py`, `en1992_tightness_kinds.py`)
- Local `nested_patch!` gets `optional { tightness: TightnessClass => clear_tightness }`: patch fields `tightness: Option<TightnessClass>` and `clearTightness: Option<bool>` (`Some(true)` clears; absorb: later wins; inverse restores the previous value or sets the clear marker). The shared `row_patch!` is not used by en1992; it has no clear lane yet (no other consumer).
- Kinds `change-member-tightness` / `clear-member-tightness` with all layers (leaf, 4 schema twins, manifest tags 153/154, mounts, root, ts, json, edit rule (set only), binary protocol, text codec, oracle manifest, harness py/rs/feature, taxonomy, 2 vectors). 83 member patches in committed vectors padded with `tightness`/`clearTightness: null`.

## 4. en1998 nested building rows - done (`en1998_nested.py`)
- 6 kinds `insert-/remove-` `member|storey|system` (tags 248..253) with every layer incl. proto/graphql, testkit demo cases, text codec, vectors (apply). Shared parent selector via `NormEditRules::payload` (`InsertItemRule`/`IdRemoval`, selector `buildingId`). Duplicate id, unknown parent are refused, insert clamps an index past the end with a `mutation.clamped` warning (like `insert-storey-variable`). Coordination with the en1998 v5 owner through requests.md (announced after the fact: no v5 edit to these files was in flight).

## 5. Python engine - 8 of 10 green
`verify_fixtures.py` (engine vs committed vectors): en1990 140/0, en1991 198/0, en1995 185/0, en1996 141/0, en1997 169/0, en1998 254/0, en1999 203/0 failures; remaining en1992 2 (bar-layer collection name), en1993 22, en1994 1 (catalogue designation). Engine changes (`🔮️oracles/🏃️execution`): keyed `value` upserts (`change-case-number|text`), `clear-<field>`, `change-<noun>-order` as reorder, generic `newValue` writes the field the noun names (head or tail of a key), optional text fields of `site` (absence = empty), Latin plurals, vowel+y plurals, owner-prefix stripping of field names, collection by head/tail of the noun, mirror detection requires unique type-strict reads and ignores a list the mutation itself wrote. Vector fixes: 7 en1998 mutation payloads migrated to ids, `change-site-v-s30-m-s` after/diff, en1992 `insert-anchor`/`remove-member` default `headDiameter`.

## 6. Cleanup - done
- en1998 "stale" fixture dirs: referenced by the harness (py/rs/feature) and the oracle manifest under short names; renamed to the kind names (7 dirs, 4 files updated), payloads migrated to ids. en1996 proto/graphql: 5 stale messages removed.
- Extra: 33 leaves with stale `self.index|variable_index` in `target()`/`label()` fixed; en1998 graphql types regenerated from proto.

## Checks
`verify build-owners sync` passed (added=4); `check-cargo-authority.py` 0 errors. Nothing compiled. Risk to look at first on the rebuild: the `nested_patch!` macros of en1992 (optional clause) and en1994 (within), `list_delta!` invocations of en1994, the en1998 leaves (`insertion`/`removal` signatures), `design_case::remove_case(&rows, &payload.id)` (second argument `&String` to `&str`).
