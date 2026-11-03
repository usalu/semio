# Kind convention v2 (wave 2)

Script: `🧰️scripts/kind-convention-v2/{migrate_ids,verify_fixtures,leaf_coverage,en1991_accidental_force}.py` (stage `🗑️generated/kind-convention-v2/stage/<artifact>`, optimistic sha check on apply, idempotent).

## Done
- Id selectors: en1996 (87 set, 12 insert/remove pairs), en1998 (199, 17), en1991 (23 + accidental variant, 9), en1994 (92, 4). Selectors `<parent>Id` plus last link `id`; insert keeps `index`; remove is `{parent ids, id}`. Per kind: payload, diff, inverse, schema json/ts, edit rule, fixtures (position replaced by the row id of the before snapshot). en1996/en1998/en1991 diff lanes were already id-keyed. No row type needed an id (all have one; `accidentalCases[].impact|explosion` stay `Implied`).
- Shared: `IdRemoval` + `NormEditRules::resolve_with_id_removals` (app-surface); en1991/en1999-style local copies replaced; editor commands call `resolve_edit`.
- Facets follow structs: en1998 text codec (print, parse, demo cases), proto, graphql (en1996/en1998).
- en1994 placements: remove-placement id-only, insert-placement TS imports `Placement`, harness `🦀️.rs` lists and includes the six placement kinds.
- Python engine (`🔮️oracles/🏃️execution`): id-only remove with parent ids, `identified_slot` for nested id-addressed leaves, drop inverse keeps no stray `id`. `verify_fixtures.py` (engine vs committed vectors): en1996 124 inverted/2 failures, en1998 243/3, en1991 195/2, en1994 118/1; old engine had 60/17/-/- more failures; en1990 39->6, en1997 29->11. Remaining failures: design-case `value` payloads, en1998 site kinds, en1994 catalogue designation.
- Leaf coverage (`leaf_coverage.py`): en1991 165/165 content leaves have kinds (confirmed), en1994/en1996/en1999 complete; en1992 only `members[].tightness`; en1998 `bridges[].deckLengthM|deckEiNm2` (new, en1998 owner).
- Checks: `verify build-owners sync` passed, `check-cargo-authority.py` 0 errors. Nothing compiled.

## Not done
1. en1994 diff lane still positional (hand-written rows); kinds resolve id to base position. Needs `list_delta!` + key markers, facets, diff fixtures, io codecs, unit tests.
2. en1992 `tightness` set/clear (diff macro needs a clear marker).
3. Insert/remove for en1999 nested (5), en1998 `buildings[].members|storeys|systems`, en1991 re-check (none missing in rules; 8 nested collections do not exist as lists).
4. `cases[].numbers|texts`: confirmation requested (requests.md).
5. Hand-checks: orphan fixture dirs in en1998 (7, renamed kinds), stale proto/graphql messages in en1996 (ChangeNEd*, ChangeVEd, ChangeWEd), compile risk of generated diff/inverse (templates mirror existing id kinds).
