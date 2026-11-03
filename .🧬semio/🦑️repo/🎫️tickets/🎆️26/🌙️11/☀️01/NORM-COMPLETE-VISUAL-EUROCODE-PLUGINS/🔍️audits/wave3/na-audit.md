# 🇩🇪 National Annex Audit, Wave 3 (W3-NA-AUDIT, read-only)

Date: 2026-11-03. Scope: the ten Eurocode plugins en1990 to en1999. Read-only: no source, registry or generated file was edited; no git write, no build, no `repo` command.

## Method and scope notes

- **Where the NDPs live.** The ticket names `✏️s/🔌️plugins/📕️norm/📇️registry`, but that folder holds only the app-surface contract (`🧬️contract/`) and no NDP rows. The NDP registry is `✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (3,832 entries), with per-code runtime rows in `🔖️ndp/🗃️table/en199x/🦀️.rs`. Row counts of the Rust tables match the JSON per code. All counts below use the JSON.
- **"With EN" / "with DE"** means the registry row has a value (number or table/formula text, not `null`/`Unset`) for the recommended EN annex / the DIN NA annex.
- **"Read via API"** means a distinct registry id cited as a string literal in non-test check code (`ndp()`, `ndp_common()`, `ndp_number()`, `ndp_cell()`, builder `.ndp()` / `.ndps()`). Scanned: `🗿️artifacts/<code>/**/*.rs`, excluding `🧪️tests`, `tests/`, `target/`.
- **Literal bypass suspects** are numeric literals (or const declarations) in the same check code whose value equals a registry EN or DE value of the same code. Pure value coincidence is common, so these are triage candidates, not confirmed bypasses. Excluded from the "specific" class: integers below 10, common round constants (0, 0.5, 1, 1.5, 2, 10, 100, 0.2, 0.3 …), and literals in spec-trace, snapshot, sqlite, io, field-meta, demo, examples and registry data paths. Full match list (all classes, flagged) is in `🗑️generated/w3-na/suspects-v2.tsv` (37,011 rows). The specific subset is in `🔍️audits/wave3/na-audit-suspects.tsv` (3,032 rows, file:line, literal, class, first registry id, line text).
- **Python oracles** (`🔮️oracles/…/🐍️.py`) were not counted as check code.
- Scans were single-process Python 3 with `nice -n 19` and `.venv/bin/python -I`. I did not route them through `heavy.sh` as the preamble asks for heavy sweeps; they are seconds-long and light, but flagging it for the coordinator.

## Per-code counts

| Code | NDP rows | with EN | with DE | with both | verified | read via API (ids) | read ids with no EN | read ids with no DE | unregistered ids cited | dynamic (format-built) reads | literal suspects (specific) | all value matches |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| EN 1990 | 271 | 246 | 246 | 246 | 0 | 58 | 10 | 10 | 0 | 10 | 87 | 2,148 |
| EN 1991 | 628 | 418 | 444 | 416 | 0 | 180 | 49 | 43 | 0 | 19 | 370 | 5,270 |
| EN 1992 | 225 | 147 | 150 | 147 | 0 | 163 | 30 | 27 | 0 | 2 | 549 | 5,454 |
| EN 1993 | 896 | 232 | 232 | 232 | 3 | 98 | 61 | 61 | 0 | 12 | 640 | 5,489 |
| EN 1994 | 104 | 41 | 38 | 38 | 0 | 31 | 13 | 16 | 0 | 2 | 176 | 2,427 |
| EN 1995 | 743 | 705 | 705 | 704 | 209 | 44 | 2 | 2 | 0 | 8 | 482 | 5,148 |
| EN 1996 | 361 | 233 | 237 | 232 | 0 | 105 | 81 | 80 | 0 | 4 | 283 | 2,946 |
| EN 1997 | 164 | 118 | 139 | 115 | 0 | 7 | 0 | 3 | 0 | 16 | 55 | 1,254 |
| EN 1998 | 285 | 237 | 222 | 197 | 0 | 118 | 1 | 0 | 0 | 21 | 293 | 4,258 |
| EN 1999 | 155 | 68 | 68 | 68 | 0 | 20 | 1 | 1 | 0 | 7 | 97 | 2,617 |
| **Total** | **3,832** | **2,445** | **2,481** | **2,395** | **212** | **824** | | | **0** | **101** | **3,032** | **37,011** |

Notes on the table:
- "Read ids with no EN/DE" counts check-read ids whose registry row has no value in that annex. A read of such an id yields NaN or a missing value. Example first hits: en1993 `ndp.en1993.v-101.1` (`…/🧱️member-vids/🦀️.rs:66`), en1996 `ndp.en1996.m10.rho2` (`…/📇️masonry-catalogue/🧱️vertical-load/🦀️.rs:25`), en1991 `ndp.en1991.e2-14.1` (`…/💡️inferences/🦀️.rs:551`), en1990 `ndp.en1990.e90-24.1` (`…/📏️serviceability-and-fatigue/🦀️.rs:53`), en1998 `ndp.en1998.en3.na.return-period` (`…/📐️formulas/🦀️.rs:1056`).
- "Unregistered ids cited" is 0 for all ten after excluding format-string prefixes (for example `ndp.en1993.psi-self` + `.{index}`, `ndp.en1990.psi.office` + `.0/.1/.2`, which resolve through `format!`). Those prefixes are counted under "dynamic".
- "Verified" is the JSON `status`. Only en1993 (3 rows) and en1995 (209 rows) have any verified row. The registry's own rule is that only verified NDPs may support a compliant verdict.
- The JSON `status` and the Rust `NdpStatus` agree (same counts).

## Per-criterion findings

**1. Value for EN and DE per NDP.** Most rows have a value for both annexes, but 1,301 of the 3,832 rows have neither. Single-annex-only rows (EN-only / DE-only): en1990 0/0, en1991 2/28, en1992 0/3, en1993 0/0, en1994 3/0, en1995 1/1, en1996 1/5, en1997 3/24, en1998 40/25, en1999 0/0. The EN-only and DE-only rows are the gap a document on the other annex hits silently.

**2. Check code reads through the ndp API, never a literal.** Yes for the registered reads: 824 distinct registry ids are read via API across the ten codes (sum of the per-code column). Every checked call goes through `ndp()` / `ndp_common()` / `ndp_number()` / `ndp_cell()` / `.ndp()` with an id string or a format-built id. No `ndp_is_verified` gate was found on the paths I sampled, so an unverified value still produces a verdict.

**3. Editor lets the user pick the annex per document.** Yes for all ten: each has an `annex` field with `En`/`De` (or `en`/`de`) choices in its editor field-meta (`…/✏️editor/🏷️field-meta/🦀️.rs`), and each has a `change-annex` mutation (`…/🧬️mutations/🌍️change-annex/`). en1997 defines the annex choice in `…/⚖️checks/🦀️.rs:88` and `:146` rather than in an editor field-meta file; wiring to the editor not verified line by line.

**4. 3D and diagrams show the active annex.** Mostly no. The shared 3D viewport and 2D diagram code (`📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧊️viewport3d`, `📈️diagrams2d`) has no annex reference. Per code:
- en1990: yes, as an action-table row "Annex EN/DE" (`…/🎬️scene/⚓️actions/🦀️.rs:224`).
- en1992: yes, as a ledger row "National annex - check ledger" (`…/🎬️scene/📊️ledger/🦀️.rs:34`).
- en1991, en1993 to en1999: no annex indicator in scene or diagram code. Only the inspector shows it (check-parameters and the "Annex parameters" section, `…/🪟️visual-windows/🧾️properties/🦀️.rs:304`), which is shared across codes and is not 3D or diagram output.

**5. Literal bypass suspects.** See the table. Top by specific count: en1993 640, en1992 549, en1995 482, en1996 283, en1998 293, en1991 370, en1994 176, en1990 87, en1997 55, en1999 97. Many are geometry or table data that happens to equal an NDP value (for example `12.0`, `500.0` as f_yk, `1.25`); triage before treating as bypass. The strongest candidates are const declarations and literals on lines that name a partial factor or ψ (the `const` class in the TSV).

**6. Annex-blind reads.** `ndp_common(id)` (`⚖️compliance/📜️annex-and-tables/🦀️.rs:69`) asserts EN == DE and does not take an annex. It is called 143 times with literal ids (121 distinct) across en1991 (32 calls), en1993 (36), en1995 (27), en1998 (27), en1994 (5), en1996 (3), en1997 (3), en1999 (6), en1992 (2), en1990 (0). Today none of the 121 ids differs between annexes (checked against the JSON, 0 panics), but any future EN/DE divergence on these ids panics the check.

## Suspect list

- Full specific list (3,032 rows, file:line): `🔍️audits/wave3/na-audit-suspects.tsv`.
- Full flagged list (all 37,011 value matches, with class and specificity): `🗑️generated/w3-na/suspects-v2.tsv` inside this ticket.
- Representative specific hits (first in list order per code):
  - en1992 `🗿️artifacts/🏛️en1992/…/🧬️schema/💡️inferences/⚖️checks/🧱️materials-and-time/🦀️.rs:48` `0.7` = `ndp.en1992.alpha-cc-pl`, in `eps_c1: (0.7 * fcm.powf(0.31)).min(2.8)`.
  - en1992 `…/🧱️materials-and-time/🦀️.rs:371` `30.0` = `ndp.en1992.t3-1.ecm.c20-25`, in `"XF4" => 30.0`.
  - en1992 `…/🔗️detailing/🦀️.rs:47` `0.7` = `ndp.en1992.alpha-cc-pl`, in the anchorage factor expression.
  - en1993 `…/🔩️special-checks/🔭️special-parts-checks/🦀️.rs:199` `1.25` = `ndp.en1993.gamma-m2`, in the `≤ 1.25` explanation string (a literal in `format!` arguments, so a real bypass candidate).
  - en1991 `…/🧱️parts/🦀️.rs:738` `1.10 / 1.50 / 1.90 / 2.00 / 2.13` in the `rows` table (value-matches to footbridge NDPs).
  - en1991 `…/📇️catalogue/🌬️wind/🦀️.rs:33` `1.25` = `ndp.en1991.1-3.snow-zone.z1a.factor`, passed as `basic_velocity_pressure_pa(1.25, …)`.
  - en1990 `…/🎬️scene/📊️ranking-and-reliability/🦀️.rs:13` `1.1` = `ndp.en1990.gG_sup.equ.A1` (display scaling, likely not a bypass).

## Top 15 gaps

1. **en1993: 61 of 98 read ids have no value in either annex.** Only 232 of 896 rows have values, and only 3 are verified. Largest coverage hole of the ten.
2. **en1996: 81 read ids have no EN value (80 no DE).** 361 rows, 232 with both; 283 specific literal suspects.
3. **en1991: 49 read ids have no EN value (43 no DE), and annex-blind reads.** 32 `ndp_common` call sites ignore the annex (see Method and 6). 28 DE-only and 2 EN-only rows.
4. **en1992: 30 read ids have no EN value (27 no DE), with the most specific literal suspects of any code (549).** 38 rows have EN ≠ DE, all read through annex-aware `ndp(annex, …)`, so no panic, but literal values are not forced through the registry.
5. **en1995: 482 specific literal suspects, while only 44 registry ids are read via API across 743 rows.** The rest are reached by table-cell keys or are unused. The 209 verified rows are the only substantive verified NDPs in the set and are not what check code reads.
6. **en1998: 40 EN-only and 25 DE-only rows; 293 specific literal suspects.** Annex divergence is not covered for these rows.
7. **en1997: 24 DE-only rows and 3 read ids without DE value; only 7 ids read via API of 164 rows, 16 dynamic reads.** The annex field is defined in `⚖️checks/🦀️.rs:146`, not in an editor field-meta file, so editor wiring needs confirmation.
8. **en1994: 13 read ids have no EN value and 16 have no DE value; 3 EN-only rows; 176 specific literal suspects.** Only 31 ids read of 104 rows.
9. **en1990: 10 read ids valueless in both annexes; 3 EN ≠ DE policy-placeholder rows (`ndp.en1990.a1.override.*`), with `POLICY PLACEHOLDER` text in `🔣️.json`.** These are placeholder values flagged by their own `evidence` field; any verdict that reads them rests on unverified content.
10. **Only 212 of 3,832 rows are verified, and check code reads mostly unverified rows.** A compliant verdict on these reads is not supported by the registry's own rule.
11. **3D viewport and 2D diagrams show no active annex.** Only en1990 (action table) and en1992 (ledger) display it; the other eight codes show it only in the inspector.
12. **101 dynamic (format-built) NDP reads cannot be verified statically**, the largest counts in en1998 (21), en1991 (19), en1997 (16), en1993 (12), en1990 (10). A typo in a built id returns NaN in the `ndp()` helpers (`…/📜️ndp-tables/🦀️.rs:17`, `…/🏛️en1992/…/🧬️core/🦀️.rs:16`) without an error.
13. **Annex-blind `ndp_common` (121 ids, 143 sites in en1991, en1993, en1995, en1998 and others).** Safe today (0 mismatches) but panics on the first EN/DE divergence.
14. **Literal bypass suspects of value types `0.7`, `1.25`, `30.0`, `500.0` in shared helpers (en1992 `…/🧱️materials-and-time/🦀️.rs`, en1993 `…/🔩️special-checks/…`, en1991 `…/🧱️parts/🦀️.rs`).** Recurring NDP-looking constants in shared helper code are the highest-risk bypass class; each should be routed through `ndp(annex, id)`.
15. **Ticket pointer is wrong.** `📇️registry` holds only the app-surface contract. The NDP registry is `⚖️compliance/📜️annex-and-tables/🔖️ndp`. Any gate that checks `📇️registry` for NDP coverage would pass vacuously.

## Files produced

- This report: `🔍️audits/wave3/na-audit.md`.
- Specific suspect list: `🔍️audits/wave3/na-audit-suspects.tsv`.
- Scan scripts and raw outputs (ticket-internal, delete when the ticket closes): `🗑️generated/w3-na/` (`scan.py`, `scan2.py`, `scan3.py`, `scan4.py`, `scan2.json`, `read_availability.json`, `ndp_common_mismatch.json`, `suspects-v2.tsv`).
