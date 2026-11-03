# Fixture kinds (wave 2)

## Results (niced, sequential, no cargo)
- `verify_fixtures.py` (scenarios/failures): en1990 148/0, en1991 246/0, en1992 154/0, en1993 578/0, en1995 208/0, en1996 147/0, en1997 204/0, en1998 296/0, en1999 223/0. en1994 126/1 (open, see below).
- Taxonomy: regenerated with bun (`🧰️scripts/kind-convention-v4/taxonomy_generate.ts`), check says "fresh: 2365 payloads" (was stale: 2245).
- en1993 feature `@id-check-outcome-pass-06/07`: the Examples table in `🚦️check-outcome-en1993-1/🥒️.feature` has unique ids now (pass-01..14, fail-08..13; 36 rows, no duplicates); nothing to renumber. The feature dry run was not run (no known command; the grep for duplicate ids is clean).

## Root causes fixed
1. `verify_fixtures.py` was stale: fixtures were flattened to `<case>/🔣️.{before,after,mutation,outcome,diff}.json`; the script read the old nested tree and silently reported 0 scenarios. Paths updated.
2. en1998: 9 fixture dirs carried pre-rename kind names (no schema kind, orphans); renamed by hand to the current kinds (mutation tags inside already matched): assessment-supported-building-id, building-accidental-eccentricity-ratio, building-multiple-resisting-systems, building-claims-simple-masonry, building-storey-stiffness-ratio, foundation-supported-building-id, member-effective-stiffness-ratio, storey-centre-of-stiffness-x-m / -y-m.
3. Python engine (`🔮️oracles/🏃️execution/🔧️verbs/🐍️.py`): set-field kinds now also use their declared `edit` (path, selectors, value; `SETS`, `declared_set`, `declared_get`, inverse writes the old value, absent old value removes the key). Needed for en1995 `change-deck-compression-c90` (field `decks[].compressionPerpendicularPa`, which the name heuristic could not find). Falls back to the heuristics when the path does not resolve.
4. en1999 `change-reliability-class/✅apply/🔣️.after.json`: `annex` was `En` (copied from change-annex); the mutation and diff leave it `De`. Fixed by hand.
- KINDS vs schema vs fixtures vs oracle manifest: no fixture dir without schema kind in any of the ten; the manifest names beyond kinds are scenario names (`-dupe`, `-clamp`, `-noop`, `-gone`); `change-k-fi-declared` occurs nowhere.

## Open (frozen artifact)
- en1994 `🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json` has the same `annex` `En` -> `De` defect (file rewritten 03:44 by the owner). `freeze.txt` still names `build-en1994` (since 03:24, 40 min at 04:04), so not edited. Fix: `sed -i 's/"annex": "En"/"annex": "De"/'` on that file, then rerun `verify_fixtures.py en1994`.
- ready for rebuild: fixture kinds (en1998 fixture dir renames, en1999 after-snapshot; Python-only otherwise)
