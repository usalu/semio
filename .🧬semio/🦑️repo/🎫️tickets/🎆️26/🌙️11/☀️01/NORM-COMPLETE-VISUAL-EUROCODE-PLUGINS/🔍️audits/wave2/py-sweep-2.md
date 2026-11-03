# 🐍 Python Sweep 2 (Wave 2, 2026-11-03)

Read-only re-run of the py-sweep suite (`py-sweep.md`) after the schema codegen, path-budget flattening, fixture-kinds, feature-row and typed-enum edits. Run sequentially with `nice -n 19 ionice -c3 .venv/bin/python -I`. No fixes applied, no git writes. Raw logs and driver: `🗑️generated/py-sweep-2/` (`run.sh`, `rc.tsv`, one `.log` per check).

## Summary

| Area | Pass | Fail |
|---|---|---|
| Eurocode oracle self-checks | 15 | 0 |
| Shared Python tests | 4 | 3 |
| Fixture checker (`verify_fixtures.py`, ten artifacts) | 9 | 1 |
| Feature dry runs (one per artifact) | 10 | 0 |

## 1. Eurocode oracle self-checks (`--self-check`)

| Artifact | Oracle | Result |
|---|---|---|
| en1990 | check-outcome | PASS |
| en1991 | check-outcome | PASS |
| en1992 | check-outcome | PASS |
| en1993 | check-outcome | PASS |
| en1994 | check-outcome | PASS |
| en1994 | check-outcome/🏛️columns | PASS |
| en1994 | 🧮️catalogue | PASS |
| en1994 | ⚖️compliance | PASS (no `--self-check` flag; self-check runs with no arguments: `oracle self-check OK`) |
| en1995 | check-outcome | PASS |
| en1996 | check-outcome | PASS |
| en1997 | check-outcome | PASS |
| en1997 | ⚖️compliance | PASS |
| en1997 | 🌍️geotechnics (with case file `🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json`) | PASS |
| en1998 | check-outcome | PASS |
| en1999 | check-outcome | PASS |

## 2. Shared Python tests

| Test | Result | Summary |
|---|---|---|
| NDP registry (`⚖️compliance/🧪️tests/🔖️ndp-registry/🐍️.py`) | PASS | default and `--check` both rc=0 |
| handle-paths (`⚖️compliance/🧪️tests/🔖️handle-paths/🐍️.py`) | FAIL | 66 uncovered handle paths |
| i18n-sinks (`⚖️compliance/🧪️tests/🌐️i18n-sinks/🐍️.py`) | FAIL | 1 violation |
| path-budget (`⚖️compliance/🧪️tests/📏️path-budget/🐍️.py`) | PASS | |
| spec-trace verifier (`⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`) | PASS | |
| schema-sync (`🧪️tests/🧬️schema-sync/🐍️check.py`) | FAIL | 2370 units, 2704 Rust types, 2 mismatches (was 199) |

### Failures

- **handle-paths**: `66` uncovered handle paths, first lines:
  ```
  uncovered handle path bridgeFatigue[].crossbeamSpacing 🔩️en1993/…/🎬️scene-members/🕹️handles/🦀️.rs
  uncovered handle path bridgeFatigue[].deckThickness …
  uncovered handle path builtUpMembers[].chainSpacing …
  uncovered handle path weldGroups[].width …
  ```
  All 66 listed are `en1993` `scene-members/🕹️handles` paths (bridgeFatigue, builtUpMembers, coldFormedMembers, weldGroups, …) with no `SetFieldRule`. Likely owner: en1993 scene-members handle edit rules (the visual-grips / handle-rules work). Was 0 in py-sweep 1.
- **i18n-sinks**: first line:
  ```
  🗿️artifacts/🌍️en1997/…/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/⚓️walls-and-anchors/🦀️.rs: gate reason without a coded rule: tendon modulus and area must be positive
  ```
  Likely owner: en1997 walls-and-anchors catalogue gate reason (needs a coded rule in the shared reason table, with a German template).
- **schema-sync**: 2 `type` mismatches, both for the En1996 design situation enum:
  ```
  type en1996:🧬️schema/🔺️diff En1996Diff [rust]: designSituation: enum members json=['Persistent','Transient','Accidental','Seismic'] rust=['Persistent','Transient','Accidental']
  type en1996:🧬️schema/📸️snapshot En1996Snapshot [rust]: designSituation: enum members json=[…same…] rust=[…]
  ```
  Likely owner: en1996 Rust twin of `designSituation` (missing `Seismic`), i.e. the schema-codegen output for en1996.

## 3. Fixture checker (`🧰️scripts/kind-convention-v4/verify_fixtures.py <artifact>`)

| Artifact | Result |
|---|---|
| en1990 | PASS |
| en1991 | PASS |
| en1992 | PASS |
| en1993 | PASS |
| en1994 | FAIL: 126 scenarios, 125 inverted, 1 failure |
| en1995 | PASS |
| en1996 | PASS |
| en1997 | PASS |
| en1998 | PASS |
| en1999 | PASS |

### Failure

```
change-reliability-class/✅apply: after-snapshot differs
orphan fixture dirs (no kind):
en1994: 126 scenarios, 125 inverted, 1 failures
```

Owner: en1994 `🧬️mutations/🎯change-reliability-class/✅apply/🔣️.after.json` has `annex: "En"` where the mutation and diff say `De`. This is the open item in `fixture-kinds.md` (frozen artifact, `freeze.txt` names `build-en1994`). Fix: change `"annex": "En"` to `"annex": "De"` in that file, then rerun `verify_fixtures.py en1994`. Not fixed here.

## 4. Feature dry runs (`🧰️scripts/wave2-features-dryrun.py`, one artifact at a time)

| Artifact | Case | Result |
|---|---|---|
| en1990 | check-outcome-en1990-1 | PASS, 69 ok, 0 failed |
| en1991 | check-outcome-en1991-1 | PASS, 58 ok, 0 failed |
| en1992 | check-outcome-en1992-1 | PASS, 63 ok, 0 failed |
| en1993 | check-outcome-en1993-1 | PASS, 36 ok, 0 failed (duplicate-id parse error from py-sweep 1 is gone) |
| en1994 | check-outcome-en1994-1 | PASS, 52 ok, 0 failed |
| en1995 | check-outcome-en1995-1 | PASS, 43 ok, 0 failed |
| en1996 | check-outcome-en1996-1 | PASS, 35 ok, 0 failed |
| en1997 | check-outcome-en1997-1 | PASS, 37 ok, 0 failed |
| en1998 | check-outcome-en1998-1 | PASS, 55 ok, 0 failed (was 31) |
| en1999 | check-outcome-en1999-1 | PASS, 114 ok, 0 failed (was 24) |

## Failure list

1. handle-paths: 66 uncovered en1993 scene-members handle paths (owner: en1993 handle edit rules).
2. i18n-sinks: en1997 walls-and-anchors gate reason without a coded rule (owner: en1997 catalogue reason table).
3. schema-sync: 2 en1996 `designSituation` enum mismatches, Rust lacks `Seismic` (owner: en1996 schema codegen).
4. fixtures en1994: `change-reliability-class` after-snapshot `annex` En/De (owner: en1994 frozen fixture, known).
