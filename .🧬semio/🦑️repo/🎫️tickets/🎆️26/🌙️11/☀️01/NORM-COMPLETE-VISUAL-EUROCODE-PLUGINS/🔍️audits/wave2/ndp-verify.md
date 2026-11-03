# NDP Verify (wave 2)

Script: `🧰️ndp/verify.py` (run `/home/deck/semio/.venv/bin/python -I 🧰️ndp/verify.py REGISTRY.json OUTDIR [--apply]`). Registry rewritten in place (indent 1, as before), tables regenerated with `🧰️ndp/codegen.py` (all crates under the 200000 byte cap). `🧪️tests/🔖️ndp-registry/🐍️.py` passes (2920 entries, 2557 unverified). No cargo, no git write. "ready for rebuild: ndp verify" appended to `integrate/latest.md`.

## Policy used
- Independent source = third-party library embedding the Eurocode table (venv: blue-prints 0.8.0, structuralcodes 0.7.3, groundhog 0.15.0; concreteproperties 0.8.0 has only AS/NZS codes, sectionproperties/anastruct hold no code tables).
- The target specs are not independent: spec section 0.1 says the numbers were "reproduced without the printed standards at hand" (tags [N]/[V]). A spec match only shows the transcription is consistent, so it never verifies a row. Spec consistency check: of rows citing `spec.md:line` with a number, 45 line-consistent, 6 not (EN 1992 1, EN 1995 4, EN 1998 1), the rest cite no number. Not written to the registry.
- `verified` needs every numeric value of the row confirmed. Library scope "product" (EN 338 tables, E, nu, alpha_T: fixed by the code body or product standard, identical in the NA) verifies EN and NA-DE columns. Library scope "en" (recommended values only, e.g. structuralcodes gamma_c, groundhog EC7 factors) cannot confirm the NA-DE column, so those 86 rows stay `unverified` and get an `EN confirmed, NA-DE value still open` note in `evidence`.
- Citation is stored in the existing `evidence` field (library+version, module, class/symbol, attribute) and `source.edition` ("EN 338:2016" for the strength rows). No schema change was needed: `evidence` and `source` already exist, and the Rust rows do not carry `evidence`.
- Hand review: whole rows of C14, C18, C24 (all 12 properties each, 36 rows, about 21 % of the 174 new plus 36 old) compared against the raw `_strength_data` tuples (E stored in GPa, converted); C27 and D40 compared the same way to confirm the conflicts; the unit and attribute mapping (e005 = E_m_0_k etc.) checked against the library comment. Conflicts (33) were confirmed only for C27, D30 (raw tuple) and D40 by hand; the rest rely on the same code path. The 3 steel constants read from source. Previously verified C16/C24/C30 rows (36) all re-confirmed.

## Counts per code (rows: verified / unverified)
| Code | Rows | Before | After | New verified | EN-confirmed, NA open | Conflicts |
|---|---:|---|---|---:|---:|---:|
| EN 1990 | 260 | 0 / 260 | 0 / 260 | 0 | 0 | 0 |
| EN 1991 | 543 | 0 / 543 | 0 / 543 | 0 | 0 | 0 |
| EN 1992 | 194 | 0 / 194 | 0 / 194 | 0 | 11 | 0 |
| EN 1993 | 332 | 0 / 332 | 3 / 329 | 3 (E, nu, alpha_T) | 1 (gamma_M0) | 0 |
| EN 1994 | 70 | 0 / 70 | 0 / 70 | 0 | 0 | 0 |
| EN 1995 | 735 | 189 / 546 | 360 / 375 | 171 | 0 | 33 |
| EN 1996 | 289 | 0 / 289 | 0 / 289 | 0 | 0 | 0 |
| EN 1997 | 126 | 0 / 126 | 0 / 126 | 0 | 40 | 1 candidate |
| EN 1998 | 241 | 0 / 241 | 0 / 241 | 0 | 34 (ids en1997.xi.*, mislabelled code) | 0 |
| EN 1999 | 130 | 0 / 130 | 0 / 130 | 0 | 0 | 0 |
| Total | 2920 | 189 / 2731 | 363 / 2557 | 174 | 86 | 34 |

(The registry grew from 2903 to 2920 rows during the run by parallel work; "before" is the file as it stood immediately before `--apply`.)

## Conflicts (routed to `📌️important/requests.md`, values untouched)
33 EN 1995 rows against EN 338:2016: C20 rho_mean 390 vs 400; C27/C35/C40/C45/C50 `f_c,90,k` (2.6/2.8/2.9/3.1/3.2 vs 2.5/2.7/2.8/2.9/3.0); C45 E_0,05 10000 vs 10100; C50 f_t,0,k 33 vs 33.5; D18 to D70 `f_c,90,k` (e.g. D30 8.0 vs 5.3), `f_v,k`, `f_c,0,k`, E_90,mean, G_mean, D70 rho. The pattern looks like EN 338:2009 values (registry) against the 2016 edition (library); row-level edition needs fixing by a human with the print. Candidate (mapping inferred): `ndp.en1997.r3.a` 1.1 vs groundhog R3 prestressed anchorage 1.0; the row names no foundation type, so this is not counted as a hard conflict. The other 39 EC7 R-set rows match the driven-pile and retaining-structure columns under the same inferred mapping and are EN-confirmed only.

## EN-confirmed rows, NA open (library holds EN value only)
EN 1992 (structuralcodes): gamma_c 1.5, gamma_s 1.15, alpha_cc 1.0, alpha_ct 1.0, C_Rd,c numerator 0.18, k1 0.15, v_min 0.035, kt 0.6/0.4, k3 3.4, k4 0.425. EN 1993 gamma_M0 1.0. EN 1997 (groundhog): A1/A2, M1/M2, R1 to R4 (inferred pile types), xi1 to xi6 tables (34 rows with `code: EN 1998` but ids `en1997.xi.*`). The NA-DE values of these rows need the DIN NA print.

## Top remaining unverified groups and the source needed
| Rows | Group | Source needed |
|---:|---|---|
| 258 | en1991.1-4 c_pe, q_p profile, wind NA | EN 1991-1-4 and NA-DE print (no library holds wind tables) |
| 189 | en1995.strength GL, LVL, CLT, plywood, OSB, remaining C/D conflicts | EN 14080, EN 14374, EN 12369 prints, EN 338 print for the edition conflict |
| 102 | en1990.psi (Table A1.1) | EN 1990 and DIN EN 1990/NA print (spec says [V]; no library) |
| 78 | en1993.1-2 fire reduction factors | EN 1993-1-2 print |
| 75 + 21 | en1995 k_mod, k_def | EN 1995-1-1 Table 3.1/3.2 and NA print (no library holds them) |
| 74 + 20 + 20 + 24 + 21 | en1998 (4.4.2.5, T3.2, T3.3, NA.4, EN 1998-5) | EN 1998-1 and NA print |
| 69 + 16 | en1990 a2 bridges, gQ/gG | EN 1990 A2, A1.2 print |
| 49 | en1993.1-10 brittle fracture t_max | EN 1993-1-10 Table 2.1 print |
| 40 + 26 | en1993.1-4 stainless, 1-9 fatigue | EN 1993-1-4 and 1-9 print |
| 48 + 36 + 30 + 28 + 24 + 16 | en1996 K, Annex E, delta, f_vk0, fire, f_xk | EN 1996-1-1 and NA print |
| 24 + 24 | en1997 DE DIN 1054, uplift | DIN 1054:2021-04 print |
| 34 | en1997 xi NA-DE column | DIN EN 1997-1/NA print |
| 799 | rows with no numeric value (spec-line rows) | not verifiable as numbers; need a value first |

## Findings besides conflicts
- `🧪️tests/🔬️unit/🦀️.rs:324` assert `unverified_ndp_entries().count() == NDP_TABLE.len()` is wrong whenever a row is verified (already wrong at 189).
- 36 rows have an id family that does not match the `code` field (en1997 ids with code EN 1998).
- Evidence is appended with the date 2026-11-03 prefix `ndp-verify`; re-running is idempotent.

## Codegen reproducibility and the en1995 diff (coordinator question)
- The JSON is the single source. `codegen.py` is deterministic: it sorts by id, wipes `*.rs` and stale folders, and writes only from the JSON. After my `--apply` it was run once for all codes (2920 entries), so the CORRECTNESS-2 rows (en1991 `e4-*`, `fire-load-fractile.*`, en1997 `gk1.presumptive.*`) are in `🗃️table/en1991` and `🗃️table/en1997` (checked by grep). A second run produced byte-identical output to disk (`diff -r` clean): reproducible, no hand edit survives in generated files.
- Cause of the en1995 rewrite: in the generated en1995 crate, the lines that change are the per-row `.ver()` and `.ed("EN 338:2016")` suffixes of the 171 rows newly verified by this task, plus any rows the en1995 owner added to the JSON (wave6 diaphragm/bracing rows) that were not yet generated. I could not separate the second part: the pre-run generated file was overwritten before I diffed, and the working tree now equals HEAD. No evidence of a hand edit to generated en1995 output; the registry rows are the cause. Rule going forward: edit only the JSON, run `codegen.py` after every JSON change.
