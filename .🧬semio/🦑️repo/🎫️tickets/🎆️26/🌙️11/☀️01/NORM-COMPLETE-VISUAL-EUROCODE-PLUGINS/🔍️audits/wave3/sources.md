# 🔎 Wave 3 Sources: Printed Clause and Table Text in the Repository

Read-only audit by W3-SOURCES. No code, registry or ticket state was changed. Scope: the 30 most important blocked items from `🔍️audits/wave2/printed-standards-checklist.md` (P1 items 1 to 16 and P2 items 17 to 19, split into 30 clause/table rows).

## 1. Result in one paragraph

No printed EN or DIN standard text, table image or table transcription exists in the repository. The norm research pointer `✏️s/🔌️plugins/mit-bestand/recherche/` (from `✏️s/🔌️plugins/📕️norm/AGENTS.md:10`) does not exist, and there is no `recherche` folder anywhere under `✏️s/🔌️plugins/`. A search of all 380 PDFs found only unrelated files (a thesis, a steel-component marketplace PDF, PDF/A conformance fixtures, a DWG spec). No CSV, TSV or XLSX holds standard tables. The only repo material that names clauses and values is:

- the repo's own spec targets `📐️specs/*-target.md`, which paraphrase the standards and tag values `[V]` or `[T]` (recalled or to verify), not print copies;
- the NDP registry `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`, whose `evidence` field names secondary web pages and recall notes, never a quoted print;
- two Python packages in `.venv` (`structuralcodes` 0.7.3, `groundhog` 0.15.0) that implement EN 1992 and EN 1997 formulas. They are code, not standard text, and are cited by the registry as partial cross-checks.

So every blocked item stays blocked. The only exception is partial: some items have a clause paraphrase in a spec target that gives the value as the repo's own recollection, which is useful for locating the clause but does not clear the `unverified` status.

## 2. Path checks

| Expected source | Result |
|---|---|
| `✏️s/🔌️plugins/mit-bestand/recherche/` (norm AGENTS.md pointer) | Missing. The `mit-bestand` folder is at `♻️mit-bestand/` (repo root) and has no norm research. |
| EN/DIN PDFs or scans anywhere | None found (380 PDF hits, all unrelated or fixtures). |
| Tables as CSV/TSV/XLSX | Only the ticket's `🧰️scripts/catalogue-bridge-*-inputs.tsv` and `feature-rows/*.tsv` (generator inputs, not standard data). |
| `🔍️audits/ndp-public-verification.md` (cited by registry evidence) | Exists but is a secondary web/library check, not print. Not re-audited here. |

## 3. Item-by-item mapping

Legend: FOUND = repo text with the clause or table content. PARTIAL = clause located or value stated only as the repo's recollection. NOT FOUND = nothing usable. Spec line numbers are from `📐️specs/` inside `TICKET/`. Registry locators are the `source.locator` field of `🔣️.json`.

| # | Item (checklist) | Status | Repo location(s) with line | Notes |
|---|---|---|---|---|
| 1 | EN 1992-1-1 3.1.6, alpha_cc / alpha_ct, DE NA 0.85 (P1.1) | PARTIAL | `specs/en1992-target.md:11` (partial factors, no clause), `:25` (M-01, 2.4.2, 3.1.6), `:31` (M-02 Table 3.1) | Registry `ndp.en1992.m-01.alpha-cc-persistent` (locator 3.1.6(1)); `evidence` cites secondary sites (frilo, dlubal, Brandenburg bautechnik tipp), not the NA. |
| 2 | EN 1992-1-1 2.4.2.4 gamma_c, gamma_s, fatigue, accidental (P1.1) | PARTIAL | `specs/en1992-target.md:11` (gamma_c 1.5, gamma_s 1.15, accidental 1.2 / 1.0, fatigue 1.5 / 1.15) | Recalled values in spec; no NA text. |
| 3 | EN 1992-1-1 6.2.2 shear: C_Rd,c, k1, v_min (P1.2) | PARTIAL | `specs/en1992-target.md:182` (V-01), `:184` (V-02 6.2.2) | Registry `crd-c-shear`, `k1-shear`, `vmin-shear` (locator 6.2.2(1)); value text not in spec excerpt. |
| 4 | EN 1992-1-1 6.2.3 cot theta max, nu1, DE a/b form (P1.2) | PARTIAL | `specs/en1992-target.md:192` (V-03 6.2.3 variable strut) | Registry `cot-theta-max` (6.2.3(2)); no NA text. |
| 5 | EN 1992-1-1 6.4.4 punching C_Rd,c, k1, u0 rules (P1.3) | PARTIAL | `specs/en1992-target.md:238-240` (P-03, NDP line: `CRd,c 0.18/gamma_c` EC and DE, `k1` EC 0.1 / DE 0.12, tagged `[V]`) | Registry `crd-c-punch`, `punch-small-u0.*`; spec conflicts with checklist E92-39 (DE C_Rd,c). |
| 6 | EN 1992-1-1 6.4.5 punching reinforcement, 1.5(d/s_r) (P1.3) | PARTIAL | `specs/en1992-target.md:245` (P-04 6.4.5) | Registry `p-04.1`, formula conflict E92-43 open. |
| 7 | EN 1993-1-1 6.1 gamma_M0 / M1 / M2 (P1.4) | NOT FOUND | none located in this pass (spec `en1993-target.md` not searched for 6.1) | Registry `gamma-m1` (locator 6.1(1)); evidence "DIN EN 1993-1-1/NA 1.10 per code", secondary. |
| 8 | EN 1993-1-10 Table 2.1 t_max, 560 empty cells (P1.5) | NOT FOUND | none. Registry rows `ndp.en1993.1-10.t21.*` (locator Table 2.1) | Registry: 560 cells "Cell not transcribed"; 49 written from memory (`🔍️audits/wave2/en1993-v18.md`). |
| 9 | EN 1993-1-1 Table 3.1 fy / fu (P1.5) | PARTIAL | `specs/en1993-target.md:40-41` (V-010 3.2.1 Table 3.1: grades and inputs listed, values not in excerpt) | Registry `ndp.en1993.v-010.1` is explicitly "written from memory ... NOT double-keyed from the print edition". |
| 10 | EN 338 Tables 1 / 3, EN 14080 Table 5 timber strength (P1.7) | NOT FOUND | `specs/en1995-en1996-target.md:905` (states table transcription is still pending) | Registry strength rows (locator Table 5) downgraded to unverified; 46 verified rows conflict with their notes (checklist P1.7). |
| 11 | EN 1995-1-1 Table 3.1 kmod, Table 3.2 kdef (P2.18) | PARTIAL | `specs/en1995-en1996-target.md:32-35` (T-02, kmod EC values in-line), `:46` (T-04 kdef, Table 3.2) | Values in spec are the repo author's recollection, not print. NA-DE load-duration assignment cites "NA Table NA.1" (`:37`) without text. |
| 12 | EN 1990 Table A1.2(A/B/C), A1.1 / A2.x, psi (P1.6) | PARTIAL | `specs/en1990-en1991-target.md:83` (Table A1.2(A/B) ndp line), `:118` (psi NA-DE "match EN recommended", `[V]`) | Registry `gQ.str.A1` (Table A1.2(B)), `psi.office.0` (Table A1.1), `gM.*`; `a1.override.*` marked "POLICY PLACEHOLDER (unverified)". |
| 13 | EN 1991-1-4 Tables 7.3a / 7.4a / 7.5 / 7.6, pressure coefficients (P1.8) | PARTIAL (zone descriptions only) | `specs/en1990-en1991-target.md:275` (zones F/G/H/I, Table 7.4a duopitch), `:277` (Table 7.6/7.7 canopies) | No coefficient values in print form. Registry pitched-roof ids: `ndp.en1991.1-4.cpe.pitched0.0` not present under that exact id, so the id mapping needs checking. |
| 14 | EN 1991-1-4 Table 8.1 bridge wind depth (P1.8 / P1.9) | PARTIAL | `specs/en1990-en1991-target.md:283` (Table 8.1 "simplified method" reference) | Oracle fixture `🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party-v3-extended.json` (lines 13, 79, 145) cites "Table 8.1 ... registry values recalled from ...". Not print. |
| 15 | EN 1991-1-7 / EN 1991-2 LM1, HSLM, braking (P1.9) | NOT FOUND | none located. Registry id `ndp.en1991.2.lm1-q1` not present; EN 1991-2 values recalled per `en1991-v9.md`, `en1990-v4.md` (checklist section 3) | Only recall reports under `🔍️audits/wave2/`. |
| 16 | EN 1992-4 cone factor k, pull-out k2, Table / clause 7.2.1.4-7.2.1.5 (P1.10) | PARTIAL | `specs/en1992-target.md:420` (FA-04 7.2.1.4 pull-out, k2 7.5 cracked / 10.5 uncracked, "precisely" hedged) | Registry `cone-k-cast-cracked` (7.2.1.4) and `pullout-k2-cracked` (7.2.1.5) rely on secondary evidence; `fa-03.1` has no value. |
| 17 | EN 1994-1-1 7.4.3 (0.4) vs EN 1994-2 6.8.5.4 (0.2) tension stiffening (P1.11) | PARTIAL | `specs/en1994-en1999-target.md:254` (C-43 cl. 7.4.3, prints `Δσs = 0.4 fctm/(αst ρs)`) | No text of EN 1994-2 6.8.5.4 in repo. Registry adjudicator note 2026-11-03 says the 0.2 rests on recollection. |
| 18 | EN 1996-1-1 Tables 8.1 / 8.2 chases and recesses (P1.12) | NOT FOUND | `specs/en1995-en1996-target.md:674` (chase rule text, "NA table ... `depth <= 30 mm`" with no table values) | Registry `ndp.en1996.chase.band.*` locator "8.6.2 Tables 8.1, 8.2", evidence: "the table values are not printed in the repo". Checklist cites `en1996-v12.md`, `en1996-v5.md`. |
| 19 | EN 1996-1-1 3.6 fk / fxk tables, K, Table 3.3, NA tables NA.5-NA.12 (P1.12) | PARTIAL | `specs/en1995-en1996-target.md:503` (formula 3.2, `α`, `β`, K from Table 3.3 by unit type), `:506` (NA-DE tables NA.5 to NA.12 referenced), `:518` (Table 3.6 fxk1 / fxk2 values in-line, recalled) | Formula text is repo paraphrase; NA table values not present. |
| 20 | EN 1998-1 Table 3.2 / 3.3 spectral S, TB, TC, TD (P1.13) | NOT FOUND | none with values. `specs/en1997-en1998-target.md:351` (SEI-01 ground types, Table 3.1 reference only) | Registry `ndp.en1998.t32.*` (locator Table 3.2) unverified; checklist says "from memory". |
| 21 | EN 1998-1 gamma_I importance (4.2.5) (P1.13) | PARTIAL | `specs/en1997-en1998-target.md:340` (gamma_I I 0.8, II 1.0, III 1.2, IV 1.4, "NA-DE same values") | Registry `ndp.en1998.en1.gamma-i.*` (locator 4.2.5 / NA). NA-DE equality is the repo's claim, not print. |
| 22 | EN 1998-1 4.3.1(6) cracked stiffness 0.5 (P1.13) | NOT FOUND | none located in this pass (spec `en1998-*` only searched for SEI/AS) | Registry `en1.cracked-stiffness-factor` (locator 4.3.1(6)); checklist: "quoted from memory". |
| 23 | EN 1998-5 Annex E permeability limit 5e-4 m/s (P1.14) | NOT FOUND | none. Registry evidence: "5e-4 m/s is not in the repository spec". | Source named in checklist only (`en1998-part5-v2.md`, ticket report). |
| 24 | EN 1998-5 Annex B csr factor 0.65, F_L 1.25, pile contrast 6 (P1.14) | NOT FOUND | none. Registry `csr-factor` evidence: "τe/σ'v0 = 0.65 (a_g S/g)(σv0/σ'v0), no depth reduction r_d" (formula note only). | Checklist cites `en1998-v2` "unverified". |
| 25 | EN 1997-1 / DIN 1054 Table A.4 M2 phi, c (P1.15) | PARTIAL (cross-check only) | `specs/en1997-en1998-target.md:192` (Table A.10 ξ values, EN, recalled) | Registry `ndp.en1997.m2.phi` evidence: EN confirmed and DE "still open" by `groundhog` 0.15.0. See section 4 for the library. |
| 26 | EN 1997-2 Annex D cpt N_kt = 15 (P1.15) | NOT FOUND | none | Checklist cites recall only. |
| 27 | EN 1993-1-8 hollow joints Tables 7.8 / 7.11, v-440 (P1.16) | NOT FOUND | none (spec `en1993-target.md` not searched for 7.8 / 7.11) | Registry `ndp.en1993.v-440.1` (locator 7.4.2) "written from memory, NOT double-keyed from the print". |
| 28 | EN 1999-1-3 Table 2.1 gamma_Mf, alloy Table 3.2 (P2.21) | PARTIAL | `specs/en1994-en1999-target.md:284-285` (A-01 alloy table 3.2a-d description), `:653` (E2-01, Table 3.2 row 6082-T6, fo 260, fu 310, HAZ factors) | Registry `ndp.en1999.gamma-mf` evidence: private list flagged `confirmed=false`. Table 3.2 row in spec is a worked example, not the print. |
| 29 | EN 1992-1-1 Tables 7.2N / 7.3N crack control, Table 6.3N fatigue (P2.17 / P1.3 family) | PARTIAL | `specs/en1992-target.md:311` (SL-05, Tables 7.2N / 7.3N described), `:333` (F-03, Table 6.3N values in-line: k1 5, k2 9, 162.5 MPa, 58.5, 35) | Values are the repo spec's own statement; no print. |
| 30 | EN 1992-1-2 Table 3.2a steel ks,theta and Table 3.1 kc,theta (P2.17) | PARTIAL (concrete only) | `specs/en1992-target.md:385` (R-07, kc(θ) siliceous table 3.1, values in-line: 20 °C 1.00 ... 900 °C 0.08) | Steel ks,θ (Table 3.2a) not in repo; registry `ndp.en1994.1-2.ks.*` (locator Table 3.2a) and checklist say "recalled". |

Also relevant, not in the 30: `specs/en1993-target.md:40` (V-010), `specs/en1990-en1991-target.md:159` (Table D1 for k_n, V unknown), `specs/en1992-target.md:599` (strut `k2 = 0.85` with DE `0.75? [V]`).

## 4. Secondary code sources found in the repo (not print, use with care)

- `.venv/lib/*/site-packages/structuralcodes` version 0.7.3: implements EN 1992-1-1 (`codes/ec2_2004/shear.py` with cot-theta limits and shear formulas). It encodes EN defaults and should be treated as a formula cross-check, not as NA text. Registry evidence cites it for `crd-c-shear` / `vmin-shear` / `cot-theta-max`.
- `.venv/lib/*/site-packages/groundhog` version 0.15.0 (`standards/`): geotechnics standard code, cited for the DE M2 factor check (`ndp.en1997.m2.phi`). Not inspected beyond the directory listing here.
- Repo-level secondary web references in registry `evidence`: frilo.eu, dlubal.com handbooks, lbv.brandenburg.de bautechnik tipp (for alpha_cc DE 0.85). These are third-party summaries, not the DIN text.

## 5. Recommendation for the human

1. The printed texts are not in the repo. To clear the blocked items, the printed clauses or tables must be added to the repo, for example as a source folder under the norm plugin or as a transcribed table file with a `source.locator` and page number. Without them, every row stays `unverified`.
2. Fix the broken pointer: `✏️s/🔌️plugins/📕️norm/AGENTS.md:10` points to `../mit-bestand/recherche/`, which does not exist.
3. For a fast partial check before the prints arrive, run the two `.venv` libraries as cross-checks for EN 1992 shear, EN 1992 punching and EN 1997 M2 values. Record their output in the registry evidence field only, not as `verified`.
