# 🐍️ Python Sweep (Wave 2)

Read-only sweep of the Python side of the test suite after the nightly edits. Run sequentially with `nice -n 19 ionice -c3 .venv/bin/python -I`. No fixes applied. Raw logs: `🗑️generated/py-sweep/`.

## 1. Eurocode oracle self-checks

| Artifact | Oracle | Result | Notes |
|---|---|---|---|
| en1990 | check-outcome | PASS | |
| en1991 | check-outcome | PASS | |
| en1992 | check-outcome | PASS | |
| en1993 | check-outcome | PASS | |
| en1994 | check-outcome | PASS | |
| en1994 | check-outcome/🏛️columns | PASS | |
| en1995 | check-outcome | PASS | |
| en1996 | check-outcome | PASS | |
| en1997 | compliance | PASS | |
| en1997 | check-outcome | PASS | |
| en1997 | geotechnics | PASS | Needs the case file argument: `…/✳️any/🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json --self-check`. Run without it, it reads stdin and fails with `JSONDecodeError`. This is a usage issue, not a regression. |
| en1998 | check-outcome | PASS | |
| en1999 | check-outcome | PASS | |

Oracles with `--self-check` found: 13 (the list above). The `en1990` and `en1998` compliance oracles and the other `🔮️oracles` without `--self-check` were not run.

## 2. Shared Python tests

| Test | Result | Summary |
|---|---|---|
| NDP registry (`⚖️compliance/🧪️tests/🔖️ndp-registry/🐍️.py`) | PASS | 322 direct lookups and 758 id literals resolve; 3057 NDP entries, 2799 unverified |
| handle-paths (`⚖️compliance/🧪️tests/🔖️handle-paths/🐍️.py`) | PASS | Every handle write path has an edit rule |
| i18n-sinks (`⚖️compliance/🧪️tests/🌐️i18n-sinks/🐍️.py`) | PASS | No English-only user-visible sink |
| spec-trace verifier (`⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`) | PASS | en1997 42 ok, en1998 32 ok, en1999 42 ok (and the other artifacts) |
| schema-sync (`🧪️tests/🧬️schema-sync/🐍️check.py`) | FAIL | rc=1, 2356 units checked, 2688 Rust types, 199 mismatches: optional 160, fields 32, bounds 3, missing-type 4 |

### schema-sync failures

```
units checked=2356 rust types=2688 mismatches=199 by category={'optional': 160, 'fields': 32, 'bounds': 3, 'missing-type': 4}
```

Likely owners by path:

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/…/🧬️schema/🔺️diff` (139 optional, 3 bounds). Bounds: `En1997PileGroupsPatch` rows and columns and `En1997PilesPatch` count, JSON maximum 4294967295 vs reader minimum 0 only.
- `…/🗿️artifacts/🌍️en1997/…/🧬️schema/🧬️mutations/*` (14 fields): `json-only=['mutation']` in the TS model, and reader field lists that omit `mutation`.
- `…/🗿️artifacts/⚖️en1990/…/🧬️schema/🧬️mutations/*` (14 fields, 8 optional): the same mutation-field mismatch.
- `…/🗿️artifacts/🫨️en1998/…/🧬️schema/🔺️diff` (13 optional).
- `…/🗿️artifacts/🪵️en1995/…/🧬️schema` (1 missing-type): `Deck` has no TS interface.
- `⚖️compliance/🧬️schema` and `⚖️compliance/🧰️design-case/🧬️schema` (4 fields, 3 missing-type): `NdpAnnexVerification`, `NdpAnnexStatus`, `CaseEntryGeometry` have no TS interface.

## 3. Feature dry runs (one case per Eurocode artifact, `🧰️scripts/wave2-features-dryrun.py`)

| Artifact | Case | Result |
|---|---|---|
| en1990 | check-outcome-en1990-1 | PASS, 69 ok |
| en1991 | check-outcome-en1991-1 | PASS, 58 ok |
| en1992 | check-outcome-en1992-1 | PASS, 63 ok |
| en1993 | check-outcome-en1993-1 | FAIL, feature parse error (see below) |
| en1994 | check-outcome-en1994-1 | PASS, 52 ok |
| en1995 | check-outcome-en1995-1 | PASS, 43 ok |
| en1996 | check-outcome-en1996-1 | PASS, 35 ok |
| en1997 | check-outcome-en1997-1 | PASS, 37 ok (about 6 minutes) |
| en1998 | check-outcome-en1998-1 | PASS, 31 ok |
| en1999 | check-outcome-en1999-1 | PASS, 24 ok |

en1993 failure, first lines of `dryrun-🔩️en1993.log`:

```
AssertionError: ['Duplicate scenario id @id-check-outcome-pass-06', 'Duplicate scenario id @id-check-outcome-pass-07']
```

Owner: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature`. Two scenarios reuse the ids `@id-check-outcome-pass-06` and `@id-check-outcome-pass-07`.

## Summary

| Area | Pass | Fail |
|---|---|---|
| Oracle self-checks | 13 | 0 (1 usage error, passes with its case file) |
| Shared tests | 4 | 1 (schema-sync, 199 mismatches) |
| Feature dry runs | 9 | 1 (en1993, duplicate scenario ids) |

### Failures

- schema-sync: 199 mismatches (en1997/en1990/en1998 `🔺️diff` and `🧬️mutations` schemas, en1995 `Deck`, shared `NdpAnnex*`/`CaseEntryGeometry`).
- en1993 dry run: duplicate scenario ids `@id-check-outcome-pass-06` and `@id-check-outcome-pass-07` in `🚦️check-outcome-en1993-1/🥒️.feature`.
