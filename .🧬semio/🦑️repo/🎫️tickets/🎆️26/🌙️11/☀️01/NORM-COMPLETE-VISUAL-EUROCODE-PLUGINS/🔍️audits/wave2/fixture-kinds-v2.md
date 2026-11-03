# Fixture kinds v2 (wave 2)

## Verdict
All 25 matrix-6 failures except one were a **checker regression**, not fixture drift. `kind-convention-v2/verify_fixtures.py` (rewritten at 04:06 for flat paths) did not call `verbs.declare_kinds(...)` / `declare_catalogue(...)`, so the engine fell back to name heuristics (collection "None", missing `compression-c90` key, id re-insert). The declared-edit version lives in `kind-convention-v4/verify_fixtures.py`. The fixture-kinds v1 report ran the v4 behaviour (9/10 green); matrix 6 ran the stale v2 script.

Fix: v4 script copied over `🧰️scripts/kind-convention-v2/verify_fixtures.py` (one logic, both paths correct).

## Results (v4 checker, niced, sequential, no cargo)
en1990 148/0, en1991 251/0, en1992 154/0, en1993 578/0, en1995 208/0, en1996 147/0, en1997 205/0, en1998 298/0, en1999 223/0. en1994 (frozen, not touched) 126/1: `change-reliability-class` annex `En` vs `De` queued in requests.md (the beam-steel-designation failure was also a checker artefact and is green).

## Orphans
- en1998: 11 fixture dirs carried pre-rename names again (the earlier rename was not durable: the en1998 owner's 04:08 test/oracle edit points kinds at the old dirs). Renamed all 11 to the current kind names and updated every reference (`🐍️.py`, `🥒️.feature`, `🦀️.rs` include_str paths, `🔮️oracles/🔣️.json`) with word-bounded perl substitutions: assessment-supported-building-id, building-accidental-eccentricity-ratio, building-multiple-resisting-systems, building-claims-simple-masonry, building-storey-stiffness-basis, building-storey-stiffness-ratio, foundation-supported-building-id, member-effective-stiffness-basis, member-effective-stiffness-ratio, storey-centre-of-stiffness-x-m/-y-m. Orphans now none; 298 scenarios.
- Orphan leaves: `leaf_coverage.py` en1997/en1998 now report 0 content leaves without a kind (kinds were added by other owners since 04:10: concreteClass, effectiveStiffnessBasis, storeyStiffnessBasis). Nothing to add.

## Files
- `🧰️scripts/kind-convention-v2/verify_fixtures.py`
- en1998 `🧪️tests/🧫️fixtures/🧬️mutations/✏️change-*` (11 dirs) and `🧪️tests/🫨️mutate-en1998-1/{🐍️.py,🥒️.feature,🦀️.rs}`, `🔮️oracles/🔣️.json`
- `📌️important/requests.md`, `🗑️generated/build-en1998/latest.md`

- ready for rebuild: fixture kinds v2
