# Catalogue Coverage Audit 3 (Eurocode plugins, wave 2)

Read-only audit of the spec-trace tables `✳️any/🧭️spec-trace/🔣️.json` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/`. No source file, build or git write. Scripts and outputs: `🗑️generated/coverage-3/` (`audit.py`, `design.py`, `registry.py`, `build_report.py`, `result.json`, `design.json`, `registry.json`, `run.txt`), all run with `python3 -I`.

Spec id source: `🗑️generated/catalogue-coverage/ids.json` (extraction of `📐️specs`, 655 ids, the same as coverage 2), compared with the `specIds` list and the `items` of each trace file. Check fragments were grepped in non-test `.rs` files under each artifact (a path component containing `test` is excluded, and the generated `spec-trace/🦀️.rs` twin is excluded).

## 1. Per code: spec ids and realisation

| Code | Spec ids (specs) | `specIds` in file | Items | check | data | design-case | not-modelled | untraced | extra (not in specs) | coverage 2 (missing) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| EN 1990 | 29 | 29 | 29 | 29 | 0 | 0 | 0 | 0 | - | 0 |
| EN 1991 | 60 | 60 | 60 | 36 | 0 | 24 | 0 | 0 | - | 1 (E13.13) |
| EN 1992 | 156 | 156 | 156 | 58 | 0 | 98 | 0 | 0 | - | 0 |
| EN 1993 | 142 | 142 | 142 | 129 | 5 | 0 | 8 | 0 | - | 87 (V-ids) |
| EN 1994 | 54 | 54 | 54 | 51 | 3 | 0 | 0 | 0 | - | 3 (C-02, C-50, C-51) |
| EN 1995 | 67 | 68 | 68 | 27 | 0 | 41 | 0 | 0 | T-98 | 0 |
| EN 1996 | 37 | 37 | 37 | 0 | 0 | 37 | 0 | 0 | - | 0 (6 function-only) |
| EN 1997 | 42 | 42 | 42 | 0 | 0 | 42 | 0 | 0 | - | 0 |
| EN 1998 | 26 | 26 | 26 | 22 | 4 | 0 | 0 | 0 | - | 0 (11 function-only) |
| EN 1999 | 42 | 42 | 42 | 39 | 1 | 0 | 2 | 0 | - | 2 (A-20, A-21) |
| **Total** | **655** | | **656** | **391** | **13** | **242** | **10** | **0** | T-98 (EN 1995) | 93 |

Untraced: none. Every spec id has exactly one row, there are no duplicates, and the `specIds` list matches the spec extraction except `T-98` (EN 1995). T-98 is in the trace and in the registry but missing from the `ids.json` extraction (the same gap noted in `traceability.md`). The trace therefore has 656 rows for 655 extracted ids.

Check fragments by code (found = literal in non-test artifact source; test-only = literal only in tests; not-found = in neither):

| Code | check rows | fragments | found | test-only | not found |
|---|---:|---:|---:|---:|---:|
| EN 1990 | 29 | 52 | 45 | 3 | 4 |
| EN 1991 | 36 | 37 | 34 | 0 | 3 |
| EN 1992 | 58 | 58 | 58 | 0 | 0 |
| EN 1993 | 129 | 159 | 159 | 0 | 0 |
| EN 1994 | 51 | 70 | 70 | 0 | 0 |
| EN 1995 | 27 | 27 | 10 | 13 | 4 |
| EN 1998 | 22 | 41 | 41 | 0 | 0 |
| EN 1999 | 39 | 46 | 44 | 1 | 1 |
| **Total** | | **490** | **461** | **17** | **12** |

## 2. Suspect check rows (fragment not in evaluator source)

A fragment is suspect when no non-test artifact source contains it. Each suspect was traced to the code that builds the id. The literal grep cannot see ids built with `format!` or helper closures, so most suspects are false positives. Verdicts:

- **composed**: the id is built at run time from the fragment's parts (the literal grep cannot match it). Checked by reading the builder.
- **defect**: the evaluator does not emit the fragment.

| Code | Id | Fragment | Grep | Verdict and evidence |
|---|---|---|---|---|
| EN 1990 | E90.06 | `en1990.6.11b.` | not-found | defect: The evaluator emits Eq. 6.11b outcomes with key `e611b` (`📇️catalogue/🦀️.rs:542`, `E90.17` entry at line 586), not `6.11b`. The only `en1990.6.11b.` prefix in code is the scene matcher `of_check` (`🎬️scene/🧮️combinations/🦀️.rs:72`), which expects a prefix the evaluator does not produce. |
| EN 1990 | E90.06 | `en1990.6.12b.` | not-found | defect: The evaluator emits Eq. 6.12b outcomes with key `e612b` (`📇️catalogue/🦀️.rs:550`, `E90.18` entry at line 587), not `6.12b`. The scene matcher expects `6.12b.` (`🎬️scene/🧮️combinations/🦀️.rs:74`). |
| EN 1990 | E90.10 | `en1990.e10.material.` | test-only | composed: builder in `📐️material-geometry-analysis/🦀️.rs:11` (`format!("en1990.{key}.{}")`) |
| EN 1990 | E90.11 | `en1990.e11.geometry.` | test-only | composed: builder in `📐️material-geometry-analysis/🦀️.rs:11` (`format!("en1990.{key}.{}")`) |
| EN 1990 | E90.12 | `en1990.e12.analysis.` | test-only | composed: builder in `📐️material-geometry-analysis/🦀️.rs:11` (`format!("en1990.{key}.{}")`) |
| EN 1990 | E90.17 | `en1990.6.11b.` | not-found | defect: The evaluator emits Eq. 6.11b outcomes with key `e611b` (`📇️catalogue/🦀️.rs:542`, `E90.17` entry at line 586), not `6.11b`. The only `en1990.6.11b.` prefix in code is the scene matcher `of_check` (`🎬️scene/🧮️combinations/🦀️.rs:72`), which expects a prefix the evaluator does not produce. |
| EN 1990 | E90.18 | `en1990.6.12b.` | not-found | defect: The evaluator emits Eq. 6.12b outcomes with key `e612b` (`📇️catalogue/🦀️.rs:550`, `E90.18` entry at line 587), not `6.12b`. The scene matcher expects `6.12b.` (`🎬️scene/🧮️combinations/🦀️.rs:74`). |
| EN 1991 | E14.15 | `en1991.1-4.dyn-frequency.` | not-found | composed: builder in `🌪️wave3-checks/🦀️.rs` (closure `id` at lines 111, 149, 259; `en1991.1-4.` / `en1991.1-5.` stems) |
| EN 1991 | E15.02 | `en1991.1-5.section-gradient.` | not-found | composed: builder in `🌪️wave3-checks/🦀️.rs` (closure `id` at lines 111, 149, 259; `en1991.1-4.` / `en1991.1-5.` stems) |
| EN 1991 | E15.05 | `en1991.1-5.section-expansion.` | not-found | composed: builder in `🌪️wave3-checks/🦀️.rs` (closure `id` at lines 111, 149, 259; `en1991.1-4.` / `en1991.1-5.` stems) |
| EN 1995 | T-02 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-03 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-05 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-06 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-10 | `en1995.6.2.3.tension` | not-found | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-12 | `en1995.6.3.2.compression` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-13 | `en1995.6.1.5.c90` | not-found | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-14 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-15 | `en1995.6.1.7.shear` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-17 | `en1995.6.2.3.tension` | not-found | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-20 | `en1995.6.3.2.compression` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-22 | `en1995.6.1.6.bending` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-45 | `en1995.6.5.2.notch-na` | not-found | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-64 | `en1995.7.3.stiffness` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-71 | `en1995.8.2.2.johansen` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-72 | `en1995.8.2.2.johansen` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1995 | T-73 | `en1995.8.2.3.johansen` | test-only | composed: builder in `🏗️structure-and-members/🦀️.rs` (`head(key, clause, ...)` and the `format!("en1995.7.3.{kind}.{}")` closure, line 405) and `🌉️bridge-and-connections/🦀️.rs:176` (Johansen, clause 8.2.2 or 8.2.3) |
| EN 1999 | A-61 | `en1999.1-4.bend` | test-only | composed: builder in `🥫️thin-wall-checks/🦀️.rs:33` (`mk(key, ...)` then `format!("en1999.1-4.{suffix}.{}")`) |
| EN 1999 | A-62 | `en1999.1-4.defl` | not-found | composed: builder in `🥫️thin-wall-checks/🦀️.rs:33` (`mk(key, ...)` then `format!("en1999.1-4.{suffix}.{}")`) |

Counts: 29 suspect fragments. 25 are composed (8 not-found and 17 test-only). 4 are defects, on 3 rows (E90.06 with two fragments, E90.17, E90.18).

The row E90.06 carries `en1990.6.11b.` and `en1990.6.12b.`, E90.17 carries `en1990.6.11b.`, and E90.18 carries `en1990.6.12b.`.

Semantic finding: `A-20` and `A-21` (EN 1999) both realise `en1999.6.3.1.fb`. The rows are A-20 flexural buckling and A-21 the HAZ reduction factors (kappa, omega), so one of them is probably mis-mapped. Many-to-one family rows (for example `en1992.6.1.flexure.` for S-01, S-02, S-03, S-04, S-07, S-10) are the intended family realisation and are not flagged.

## 3. Design-case rows: registry, demo document, missing cases

`registry` = an `Entry { id: "X"` or `entry_of("X"` literal in non-test artifact source. `demo document` = a JSON fixture under `fixtures/📇️catalogue/` with a `cases` list (`entryId`). The Rust demo `catalogue_demo_cases()` / `catalogue_demo_document()` builds one case per `CaseCatalogue::entries()`, so it covers the registry by construction, but only tests call it (no non-test caller).

| Code | design rows | registry hit | rows in demo document (JSON) | rows in other catalogue fixture | rows with no case in any document | demo doc cases | registry entries without a trace row |
|---|---:|---:|---:|---:|---:|---:|---:|
| EN 1991 | 24 | 24 | 24 | 0 | 0 | 98 | 74 |
| EN 1992 | 98 | 98 | 98 | 0 | 0 | 115 | 76 |
| EN 1995 | 41 | 41 | 0 | 9 | 32 | 0 | 29 |
| EN 1996 | 37 | 37 | 37 | 0 | 0 | 40 | 3 |
| EN 1997 | 42 | 42 | 0 | 42 | 0 | 0 | 29 |

Verdict per code:

- **EN 1991**: all 24 design rows are in the registry and in the demo document. The demo document has 98 cases for 98 registry entries. OK.
- **EN 1992**: all 98 design rows are in the registry and in the demo document. The demo document has 115 cases, while the registry has 174 entries (the bridge v2 report says 172 registry and 174 parsed). **59 registry entries have no case in the demo document**, and none of them has a trace row. The Rust demo fn covers all 174 by construction, so the JSON fixture is out of date with it.
- **EN 1995**: all 41 design rows are in the registry (`Entry { id }` in `📐️cross-sections`). **No demo document and no demo fn.** The 26-case fixture (`🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json`, keyed `entry`) covers 9 of the 41 rows, so **32 rows have no case in any document**.
- **EN 1996**: all 37 design rows are in the registry and in the demo document (40 cases for 40 registry entries). OK.
- **EN 1997**: all 42 design rows are in the registry (71 entries). No demo-document JSON. The Rust `catalogue_demo_document()` covers all 71 by construction (test-wired), and the 126-case catalogue fixture (keyed `entry`) covers all 42 rows, so the rows are reachable from tests only.

EN 1995 rows with no case in any document (32):

`T-100`, `T-102`, `T-103`, `T-105`, `T-11`, `T-123`, `T-124`, `T-16`, `T-18`, `T-21`, `T-23`, `T-24`, `T-32`, `T-33`, `T-46`, `T-50`, `T-62`, `T-66`, `T-75`, `T-76`, `T-77`, `T-78`, `T-79`, `T-80`, `T-81`, `T-82`, `T-91`, `T-92`, `T-94`, `T-95`, `T-96`, `T-98`

EN 1992 registry entries with no case in the demo document (59, none with a trace row):

`A-07`, `A-09`, `A-10`, `A-11`, `A-16`, `B-01`, `B-02`, `B-03`, `B-04`, `B-09`, `D-02`, `D-03`, `D-04`, `D-30`, `DT-01`, `FA-02`, `FA-03`, `FA-04`, `FA-05`, `FA-07`, `FA-08`, `FA-09`, `FA-10`, `M-01`, `M-02`, `M-03`, `M-08`, `M-09`, `M-14`, `P-01`, `P-02`, `P-03`, `P-04`, `PR-01`, `PR-02`, `PR-03`, `PR-04`, `R-03`, `R-04`, `S-01`, `S-02`, `S-03`, `S-04`, `S-07`, `S-10`, `SL-01`, `SL-02`, `SL-06`, `SL-07`, `SL-08`, `SL-09`, `T-01`, `T-02`, `T-04`, `V-02`, `V-03`, `V-04`, `V-07`, `V-08`

EN 1992 check: the demo document lists 17 entries that are not design rows (`demo_doc_entries_not_in_trace`), so it also contains extra non-trace cases.

## 4. Not-modelled rows and reasons

| Code | Id | Clause | Entity | Reason |
|---|---|---|---|---|
| EN 1993 | V-034 | 5.5.2(11) | members | hybrid girders: one material per member, no reduced web yield strength f_y,red |
| EN 1993 | V-066 | 6.2.7(1) | members | warping torsion: the member action vector carries no bimoment, only the St. Venant check en1993.6.2.7.t exists |
| EN 1993 | V-501 | 5.1/5.2 | fatigueDetails | stress ranges are entered as counted bands; the derivation from sigma_max and sigma_min (compression reduced by 0.6) is not modelled |
| EN 1993 | V-504 | EN 1993-1-9 Annex A | fatigueDetails | rainflow counting (clauses::fatigue_fire::rainflow) is not wired: the detail carries counted bands, not a stress history |
| EN 1993 | V-505 | Table 8.1..8.10 | fatigueDetails | Tables 8.1 to 8.10 are neither in the repository nor in the spec; only the category series and the S-N curve are encoded |
| EN 1993 | V-506 | Annex C / D | fatigueDetails | informational annex: Paris law constants and the crack geometry are not carried |
| EN 1993 | V-609 | 4.3 | fireExposures | advanced calculation methods need a thermal and a mechanical finite element model; the spec itself marks them out of scope for closed forms |
| EN 1993 | V-805 | EN 1993 | platedPanels | plates under transverse pressure: clauses::misc::plate_lateral_pressure_stress is not wired and the plated panel carries no lateral pressure |
| EN 1999 | A-28 | 6.7.6 | - | transverse stiffener flexural check (6.7.6) and longitudinal stiffeners (6.7.7) are not implemented; the stiffener spacing only enters the web shear buckling check 6.7.4 |
| EN 1999 | A-81 | 2.3.5.? thermal expansion α = 23e-6; creep of aluminium negligible except at ele | - | the spec excludes it: the thermal expansion coefficient is a constant of 23e-6 and the creep of aluminium is negligible below 100 degrees Celsius |

Ten rows in total (EN 1993: 8, EN 1999: 2). Zero EN 1990 rows are not-modelled now (the three open ids of wave 2 are check rows).

Data rows (informational, paths not verified): 13 rows in EN 1993 (5), EN 1994 (3), EN 1998 (4) and EN 1999 (1). Several `path` values are prose, not code paths: EN 1993 `V-015` (`... (ndp.en1993.v-300.2 unverified)`), `V-601` (`... (ndp.en1993.v-601.1 unverified)`), `V-014` (`... (bolt classes 4.6 to 10.9)`); EN 1994 `C-11` and `C-45` (`... and ...`, with explanations). EN 1998 `SEI-42` uses `and` to list two paths.

## 5. Delta against catalogue coverage 2 (`catalogue-coverage-2.md`)

| Code | Spec ids | coverage 2 missing | now untraced | now |
|---|---:|---|---:|---|
| EN 1990 | 29 | 0 | 0 | 0 missing, 24 catalogue-only; now 29 check rows |
| EN 1991 | 60 | 1 (E13.13) | 0 | resolved, now design-case |
| EN 1992 | 156 | 0 | 0 | 153 catalogue-only; now 58 check + 98 design-case |
| EN 1993 | 142 | 87 (V-ids) | 0 | all traced: 129 check, 5 data, 8 not-modelled |
| EN 1994 | 54 | 3 (C-02, C-50, C-51) | 0 | all traced: 51 check, 3 data |
| EN 1995 | 67 | 0 | 0 | 68 rows incl. T-98: 27 check, 41 design-case |
| EN 1996 | 37 | 0 (6 function-only) | 0 | all 37 design-case |
| EN 1997 | 42 | 0 | 0 | all 42 design-case |
| EN 1998 | 26 | 0 (11 function-only) | 0 | all 26: 22 check, 4 data |
| EN 1999 | 42 | 2 (A-20, A-21) | 0 | all 42: 39 check, 1 data, 2 not-modelled |

Totals: coverage 2 had 93 missing ids (EN 1993 87, EN 1994 3, EN 1999 2, EN 1991 1). Now 0 are untraced. Coverage 2 counted 655 ids. The trace has 656 rows (T-98 added). The classes of coverage 2 (`implemented-in-document`, `catalogue-only`, `function-only`, `clause-cited`) no longer apply, because every id now has a realisation kind that the tests check with `assert_traced`.

Delta against the wave-2 `traceability.md` (written earlier today, before the final trace files):

- EN 1990: 26 check + 3 not-modelled -> 29 check, 0 not-modelled.
- EN 1992: 156 design-case -> 58 check + 98 design-case.
- EN 1994: 51 check + 1 data + 2 unlocated -> 51 check + 3 data, 0 unlocated (C-11 is data, C-44 is check).
- EN 1999: 38 check + 1 data + 1 not-modelled + 2 unlocated -> 39 check + 1 data + 2 not-modelled, 0 unlocated.
- EN 1995: 68 ids incl. T-98 (same count as before). EN 1993, EN 1996, EN 1997 and EN 1998: same counts.

## 6. Open items

1. Fix the four EN 1990 defect fragments (E90.06 x2, E90.17, E90.18): use the `e611b`/`e612b` keys the evaluator emits, and align the scene matcher `of_check`.
2. Review the A-20 / A-21 mapping (both `en1999.6.3.1.fb`).
3. Add the 32 EN 1995 design cases to a demo document, and regenerate the EN 1992 demo document (59 missing registry entries).
4. Demo documents are test-only for EN 1991, 1992, 1996 and 1997 (no non-test caller).
5. The static fragment check cannot see composed ids. Add a test that asserts each composed id, so that this audit does not depend on reading builders.

Generated 2026-11-03 by `🗑️generated/coverage-3/build_report.py`.
