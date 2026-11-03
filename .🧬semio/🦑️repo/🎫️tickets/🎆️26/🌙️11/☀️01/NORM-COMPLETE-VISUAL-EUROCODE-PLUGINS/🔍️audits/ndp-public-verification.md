# NDP Public Verification (EN 1990 to EN 1998, NA-DE)

Scope: web research (2026-11-02) on the unverified NDP values in `📌️important/ndp-verification-worklist.md`, conflicts first. No source, spec or registry file was edited. Registry status stays `unverified`: public secondary sources never replace the printed standard, they only rank which value is probably right and which candidate is wrong.

Status legend: `verified-public` = at least two independent public sources agree; `single-source` = one source; `unchanged` = nothing found. Sources by the same author or vendor count once. Where a search-result summary was the only access to a document, it is marked (snippet).

Limits: DIN/Beuth full texts are paywalled; the NA-DE text was never seen directly. Vendor manuals (Dlubal, Frilo, Infograph, Hilti, Halfen) and Hochschule sheets reproduce NA values; typos exist (see E92-41). `ndp.*` ids not in the worklist are named `(no id)`.

## Source legend

| Key | URL |
|---|---|
| L1 | https://www.fh-bielefeld.de/multimedia/Fachbereiche/Campus+Minden/%C3%9Cber+uns/Labore_+Lehrgebiete/Labore/Labor+konstruktiver+Ingenieurbau/Leporello/Leporello+1+_+Bemessung+von+Bauteilen+nach+EC5+_+2020+05+04-p-134198.pdf?download=1 |
| L3 | https://www.fh-bielefeld.de/multimedia/Fachbereiche/Campus+Minden/%C3%9Cber+uns/Labore_+Lehrgebiete/Labore/Labor+konstruktiver+Ingenieurbau/Leporello/Leporello+3+_+Bemessungshilfen+f%C3%BCr+Holztafeln+nach+DIN+EN+1995_1_1+_+2020+05+25-p-134202.pdf?download=1 |
| MAD | https://www.studiomadera.it/news/660-legno-conifera (UNI EN 338:2016 Tab. 15.1) |
| MADGL | https://studiomadera.it/news/658-classi-resistenza-legno-lamellare (EN 1194 era values) |
| HASS | https://www.hasslacher.com/data/_dateimanager/broschuere/HNT-Brettschichtholz-EN.pdf |
| CIV24 / CIV32 | https://civilaxis.com/timber-catalogue/reference/glulam/gl24h and .../gl32h |
| INFO | https://webhelp.infograph.de/hid_en1992_schub.html |
| HILTI | https://www.hilti.ch/content/dam/documents/e3/e3_hub/pdf-webinare/WEBINAR%20Nachweis%20der%20Querkrafttragf%C3%A4higkeit%20bei%20nachtr%C3%A4glichen%20Bewehrungs-anschl%C3%BCssen%20mit%20PROFIS%20Rebar.pdf |
| DLK1 / DLK2 | https://www.dlubal.com/de/support-und-schulungen/support/knowledge-base/002047 and .../001389 (punching) |
| HALF | https://www.halfen.com/cdn/Unternehmen%20I%C2%A0Company/Publikationen%20I%20Publications/Durchstanzen_von_Flachdecken_05-2012.pdf |
| KIT | https://www.fs-bau.kit.edu/zusammenfassungen//BSc_Stahlbetonbau%202.pdf |
| FRILO | https://www.frilo.eu/wp-content/uploads/DE/Manuals/nachweise_am_stahlbeton-querschnitt.pdf |
| LBV | https://lbv.brandenburg.de/download/Bautechnik/bautechnik_Tipp_15-06.pdf (Bautechnisches Pruefamt Brandenburg) |
| DLC | https://www.dlubal.com/de/downloads-und-infos/dokumente/online-handbuecher/rf-concrete-members/004608 |
| IDEA / EA | https://www.ideastatica.com/it/support-center/4-3-ultimate-limit-state-analysis and https://cdn.eurocodeapplied.com/design/en1992/anchorage-and-lap-length-table (snippet) |
| EURO | https://www.eurotec.team/application/files/2617/2475/9644/Konstruktionsleitfaden_Holzbau_DE.pdf |
| DL694 / BIB | https://www.dlubal.com/de/support-und-schulungen/support/knowledge-base/000694 and https://www.hochschule-biberach.de/sites/default/files/medien/dokumente/PraxisRegelnBau.pdf |
| EN6-33 / EN6-34 | https://dnaop.com/html/60058_5.html and https://dnaop.com/html/60058_6.html (reproduction of EN 1996-1-1 Tables 3.3, 3.4) |
| ZAPF | https://www.zapf-daigfuss.de/sites/default/files/downloads/2014/file_leporello_t100_neu_09-12_de.pdf (KS masonry per DIN EN 1996-1-1/NA) |
| DLW1 / DLW2 | https://www.dlubal.com/de/support-und-schulungen/support/knowledge-base/001554 and .../001564 (wind) |
| RPW | https://rechner-portal.de/bau-handwerk/holz-statik/windlast-zaun-carport-rechner |
| RPS / FOR | https://rechner-portal.de/bau-handwerk/dach-dachkonstruktion/schneelast-rechner and https://forum-verlag.com/fachwissen/bau-und-gebaeudemanagement/schneelast-berechnen/ |
| EQ25 | https://www.mbaec.de/fileadmin/documents/mb-news/2025/mb-news_01-25_Erdbeben.pdf |
| DINM | https://www.dinmedia.de/de/norm/din-1054/335418422 (DIN 1054 edition listing, snippet) |
| B90 | https://www.matec-conferences.org/articles/matecconf/pdf/2019/11/matecconf_krynica2018_06009.pdf and https://www.regeringen.ax/sites/default/files/attachments/page/eurokoder-nationella-val-15-10-2007.pdf (snippet) |
| INFOST | https://infosteel.be/images/Steel-Buildings-in-Europe/SSB03/15 (EN 1990 Table A1.2, snippet) |

## 1. Conflicts (section 1 of the worklist)

| Registry id | Code / clause | Value in code (repo) | Public value | Sources | Status | Action needed |
|---|---|---|---|---|---|---|
| `ndp.en1992.m-01.1` (E92-02) | EN 1992 3.1.6 `α_cc` accidental | DE accidental 0.85; EN 1.0 | DE: `α_cc = 0.85` generally; a higher value up to 1.0 only in justified cases (short-term loading). No fixed accidental NDP value found. | LBV, DLC | verified-public (0.85 default), accidental 1.0 unchanged | Keep 0.85 for DE; do not hard-code 1.0 for DE accidental; needs NA 3.1.6 text. |
| `ndp.en1992.m-01.1` (E92-05) | EN 1992 2.4.2.4 `γ_c` accidental | DE 1.3; EN 1.2 | DE accidental `γ_c = 1.3` (Table NA.2.1DE); EN 1.2; persistent 1.5 | FRILO, LBV (snippet) | verified-public | Repo is right; spec line 28 (1.2) is the EN value. Fix the spec. |
| `ndp.en1992.d-04.1` (E92-13) | EN 1992 Table 4.3N/4.3DE | `f_ck >= 35`, slab/beam only | nothing found | none | unchanged | Print check. |
| `ndp.en1992.v-02.1` (E92-24) | EN 1992 6.2.2 `k1` | 0.15 both annexes | DE `k1 = 0.12` (EN 0.15); DE `C_Rd,c = 0.15/γc` (EN 0.18/γc) | INFO, HILTI | verified-public | Make `k1` and `C_Rd,c` annex-dependent in `H92:184-185`. |
| `ndp.en1992.v-02.1` (E92-25) | EN 1992 6.2.2 `v_min` | `0.035 k^1.5 sqrt(fck)` both annexes | DE `v_min = (κ1/γc) k^1.5 sqrt(fck)`, `κ1 = 0.0525` (d <= 600), `0.0375` (d > 800), linear in between; EN 0.035 | INFO, HILTI | verified-public | Implement DE d-dependence in `H92:184,217`. Note `0.0525/1.5 = 0.035`: DE equals EN for d <= 600 only. |
| `ndp.en1992.v-03.1` (E92-26) | EN 1992 6.2.3 `cot θ` | `clamp(1.2 + 0.2 σcp/fcd, 1, 3)` | DE `1.0 <= cot θ <= (1.2 + 1.4 σcp/fcd)/(1 - V_Rd,cc/V_Ed) <= 3.0`, `V_Rd,cc = c 0.48 fck^(1/3) (1 - 1.2 σcp/fcd) bw z`, `c = 0.5` | INFO (full); DLK1 partial | single-source | Replace `0.2` by `1.4`, add the `V_Rd,cc` term (`H92:107-113`). |
| `ndp.en1992.p-03.2` (E92-39) | EN 1992 6.4.4 punching `C_Rd,c` | DE 0.15/γc (`H92:214`), 0.18 (`M92:169`) | DE `0.18/γc` flat slab; `0.18/γc (0.1 u0/d + 0.6)` interior columns with `u0/d < 4`; `0.15/γc` slab on ground / footings; `k1 = 0.1`; `ρl <= min(0.02; 0.5 fcd/fyd)` | DLK1, DLK2 (same vendor) | single-source | Fix `H92:214` (0.18 for flat slabs); `M92:169` is right. |
| `ndp.en1992.p-03.2` (E92-41) | EN 1992 6.4.4 punching `v_min` | 0.035 both | DE `(0.0525/γc) k^1.5 sqrt(fck)` d <= 600, `0.0375` d > 800. The vendor page prints `0.00525/0.00375` (dropped zero; ratio matches the shear form). | DLK1/DLK2 (typo), INFO+HILTI for the shear analogue | single-source | Apply the shear `κ1` form to punching; confirm against NA print. |
| `ndp.en1992.p-04.1` (E92-43) | EN 1992 6.4.5 | `v_Rd,c + 0.75 Asw fyd/(u1 d)` (`M92:179`) | `v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) Asw f_ywd,ef (1/(u1 d)) sin α`, `f_ywd,ef = 250 + 0.25 d <= f_ywd`; DE `κ_sw,1 = 2.5`, `κ_sw,2 = 1.4` | DLK1, HALF (eq. 9) | verified-public (formula); κ_sw single | Replace `M92:179`. |
| `ndp.en1992.b-01.1` (E92-47) | EN 1992 8.4.2 `f_bd` | `f_ctk,0.05` direct | `f_bd = 2.25 η1 η2 f_ctd`, `f_ctd = α_ct f_ctk,0.05/γc`; DE `α_ct = 0.85` | IDEA, EA (formula); DLC (α_ct) | verified-public (formula); α_ct single | Apply `α_ct/γc` in `F92:64`, `H92:373`. |
| `ndp.en1992.f-03.1` (E92-59) | fatigue `Δσ_Rsk` | `162.5e6/1.15` | nothing found | none | unchanged | Print check. |
| `ndp.en1992.fa-03.1` (E92-80) | anchors cone `k1` | 7.5 / 11.0 | nothing found | none | unchanged | Print check (EN 1992-4). |
| `ndp.en1992.e-03.1` (E92-82) | EN 1992 6.2.3 `ν1` | `0.6(1 - fck/250)` both | DE `ν1 = 0.75 ν2`, `ν2 = min(1.1 - fck/500, 1.0)`; EN `0.6(1 - fck/250)`. Punching: EN `v_Rd,max = 0.4 ν fcd`; DE `v_Rd,max = 1.4 v_Rd,c,u1` | KIT, FRILO; DLK1 (punching) | verified-public | Make `ν1` annex-dependent (`H92:200`). |
| `ndp.en1992.e-05.1` (E92-83) | worked example `C_Rd,c` | 0.12 vs 0.10 | `0.18/1.5 = 0.12` is EN; DE shear `0.15/1.5 = 0.10` | INFO, HILTI | verified-public | Example is EN; `H92:214` 0.10 is DE-shear and is right there. |
| `ndp.en1992.e-12.1` (E92-86) | EN 1992-1-2 Table 5.5 beam | ss 80/25..200/65 | not obtained | none | unchanged | Print check. Hint (own recollection, not evidence): the spec row 80/15, 120/25, 150/35, 200/45 looks like the continuous-beam table. |
| `ndp.en1995.t-01.1` (E95-01) | EN 338 C24 `f_t,0,k` | 14.0 | 14.5 (EN 338:2016) | L3, MAD | verified-public | Update `T95:42`; spec correct. Repo values look like the 2009 edition. |
| `ndp.en1995.t-01.1` (E95-02) | C24 `f_t,90,k` | 0.5 | 0.4 | L3, MAD | verified-public | Update `T95:42`, `O95:34`. |
| `ndp.en1995.t-01.1` (E95-04) | C16 `f_t,90,k` | 0.5 | 0.4; also C16 `f_t,0,k` 8.5, `f_c,90,k` 2.2, `G_mean` 500 | MAD | single-source | Update `T95:38`, `O95:30`. |
| `ndp.en1995.t-01.1` (E95-05) | C30 `f_t,90,k` | 0.6 | 0.4; C30 `f_t,0,k` 19.0, `f_c,0,k` 24, `f_c,90,k` 2.7, `G_mean` 750 | L3, MAD | verified-public | Update `T95:44`, `O95:36`. |
| `ndp.en1995.t-01.1` (E95-06) | EN 14080 GL24h | 19.2, 0.5, 2.5, `G` 720 | `f_t,0,g,k` 19.2, `f_t,90` 0.5, `f_c,0` 24, `f_c,90` 2.5, `G_g,mean` 650. Spec values (16.5, 0.4, 2.7) are EN 1194 era. | HASS, L3, CIV24 | verified-public | Repo right except `G`: 720 -> 650 (`T95:51`, `O95:43`). Fix spec line 28. |
| `ndp.en1995.t-01.1` (E95-07) | GL28h | 22.3, 0.5, 28, 2.5, `G` 780 | 22.3, 0.5, 28, 2.5, `G` 650 (spec 19.5/0.45/26.5/3.0 are EN 1194 values or the GL28c column) | HASS, L3 | verified-public | `G` 780 -> 650 (`T95:55`). |
| `ndp.en1995.t-01.1` (E95-08) | GL32h | 25.6, 32, 2.5, `G` 890 | 25.6, 32, 2.5, `G` 650 | HASS, CIV32 | verified-public | `G` 890 -> 650 (`T95:59`). Combined GL24c/28c: `f_t,0` 17.0/19.5, `f_c,0` 21.5/24.0 (HASS, L3). |
| `ndp.en1995.t-03.1` (E95-14/15/16/21) | EN 1995 `γ_M` glulam, LVL, plywood/OSB, CLT | 1.3 all | DE `γ_M = 1.3` for "Holz, Holz- und Gipswerkstoffe" and for timber connections; 1.25 / 1.2 are the EN recommended values. CLT 1.25 not confirmed. | L1, EURO (connections only) | single-source | Repo 1.3 is consistent; spec line 43 uses EN values. Print check for LVL/CLT. |
| `ndp.en1995.t-13.1` (E95-26) | `k_c,90` | geometric formula capped 1.5 | 1.0 (NVH and BSH, `l1 < 2h`, or hardwood); 1.25 NVH `l >= 2h` continuous; 1.5 BSH `l >= 2h` continuous; discrete cases listed in L1 | L1 (L3 same authors) | single-source | Replace the formula by the tabulated cases (`T95:359-365`). |
| `ndp.en1995.t-15.1` (E95-29) | `k_cr` LVL | formula 0.556 | nothing found | none | unchanged | Print check. |
| `ndp.en1995.t-63.1` (E95-44) | EN 1995 7.3.3 `f1` | 8 Hz both | EN: 8 Hz minimum. DE 6 Hz not found; L1 states the check applies to residential floors with `f1 > 8 Hz`. | DL694, BIB (EN 8 Hz); L1 | EN verified-public; DE unchanged | Keep 8 Hz for EN; DE needs NA 7.3.3 print. |
| `ndp.en1995.t-64.1` (E95-45/46) | 7.3.3 `a` | 1.7 / 1.5 | nothing found | none | unchanged | Print check. |
| `ndp.en1996.m-01.1` (E96-01) | shape factor δ | 0.81 | nothing found | none | unchanged | Print check. |
| `ndp.en1996.m-02.2` (E96-06/07) | EN 1996 Table 3.3 `K` clay G3/G4 GP | 0.40 | EN 0.35 / 0.35 (G1 0.55, G2 0.45) | EN6-33 | single-source | `F96:53` wrong for EN; DE NA uses own `f_k` tables (ZAPF), check separately. |
| `ndp.en1996.m-02.2` (E96-08/09) | `K` clay thin layer G1/G2 | 0.80 all | EN 0.75 / 0.70 (G3 0.50, G4 0.35; calcium silicate G1 0.80, G2 0.65) | EN6-33 | single-source | `F96:48` must be per material and group. |
| `ndp.en1996.m-03.1` (E96-15/16/18) | `f_vk0` | 0.20 / 0.15 / 0.10 mortar law | EN Table 3.4: clay GP M10-M20 0.30, M2.5-M9 0.20, M1-M2 0.10, thin 0.30, light 0.15; calcium silicate 0.20 / 0.15 / 0.10, thin 0.40, light 0.15. DE (KS, ZAPF): NM II 0.08, IIa 0.18, III 0.22, IIIa 0.26, thin 0.22 | EN6-34; ZAPF | single-source each | Make `F96:105-116` depend on unit material and annex; repo mortar-only law matches calcium silicate/AAC, not clay. |
| `ndp.en1996.m-03.3` (E96-21) | `f_vlt` cap DE | `0.045 f_b` | `f_vlt2 = 0.45 f_bt,cal sqrt(1 + σ/f_bt,cal)`, `f_bt,cal` = 0.020 / 0.026 / 0.032 of the normalised mean strength for hollow blocks / perforated or grip-pocket units / solid units; spec `0.026 f_b` = perforated units. | ZAPF | single-source | Replace `F_VLT_OVER_FB_DE` (`F96:120,128-129`). |
| `ndp.en1996.m-04.1` (E96-22/24/25) | `f_xk1/f_xk2` | various | nothing found | none | unchanged | Print check. |
| `ndp.en1996.m-07.1` (E96-36) | Table 2.3 `γ_M` | 1.5/1.7/2.0/2.2/2.5 | EN recommended Category I, designed mortar, classes 1-5: 1.5/1.7/2.0/2.2/2.5 (prescribed mortar Cat. I 1.7..2.7, Cat. II 2.0..3.0); DE NA values not found | JRC/BRIC training slide (search snippet) | single-source | Repo matches EN designed-mortar row; spec "1.7/2.0" is a mix. DE NA print check. |
| `ndp.en1996.m-10.1`, `m-80.1`, `m-81.1`, `m-90.1`, `x1-*`, `ndp.en1995.e3-bearing-perpendicular.1` (E96-38, 54, 57, 58, 60, 63, 67) | `ρ2`, storey height, `Φ1/Φ2`, REI table, floor `f1` | see worklist | nothing found | none | unchanged | Print check. E96-63 depends on E95-44. |

## 2. Further high-impact values

| Registry id | Code / clause | Value in code (spec) | Public value | Sources | Status | Action needed |
|---|---|---|---|---|---|---|
| `ndp.en1991.e14-02.4` | EN 1991-1-4/NA Table NA.B.3 inland, WZ4 | 0.95 / 1.15 / 1.30 | WZ1..4, h <= 10 m: 0.50 / 0.65 / 0.80 / 0.95; 10-18 m: 0.65 / 0.80 / 0.95 / 1.15; 18-25 m: 0.75 / 0.90 / 1.10 / 1.30 kN/m2 | DLW1 | single-source (WZ4 inland) | Value in spec matches. |
| `ndp.en1991.e14-02.5` | coast WZ4 | 1.25 / 1.40 / 1.50 | 1.25 / 1.40 / **1.55** (Baltic coast WZ2-3 columns: 0.85/1.05 and 1.00/1.20, 1.10/1.30) | DLW1, RPW (1.55 coast >18 m) | verified-public (1.55) | Spec 1.50 is wrong. |
| `ndp.en1991.e14-02.6` | North Sea islands WZ4 | 1.40 / 1.55 / 1.65 ? | 1.40 for h <= 10 m, higher bands by detailed method 2 | DLW1 | single-source | Remove 1.55/1.65 guesses. |
| `ndp.en1991.e14-01.1` | wind basics | `c_dir = c_season = 1.0` | `v_b0` 22.5/25/27.5/30 m/s, `q_b` 0.32/0.39/0.47/0.56 kN/m2, `c_dir = c_season = 1.0`; simplified method up to 25 m | DLW1, DLW2 | verified-public | Data for the four wind zones. |
| (no id) | EN 1991-1-3/NA snow `s_k` | spec formula | Zone 1 `0.19 + 0.91((A+140)/760)^2 >= 0.65`; zone 2 `0.25 + 1.91(...) >= 0.85`; zone 3 `0.31 + 2.91(...) >= 1.10`; zones 1a/2a scaled up | RPS; FOR (zone-1 minimum 0.65, 1a Berlin 0.81 = 1.25 x 0.65) | single-source (formula); verified-public (zone 1 minimum) | Print check for 1a/2a factor. |
| `ndp.en1990.l167.1`, `ndp.en1990.e90-26.1` | EN 1990 EQU `γ_G` | 1.05 / 0.95 and 1.1 / 0.9 (inconsistent) | EN recommended A1.2(A): `γ_G,sup 1.10`, `γ_G,inf 0.90`, `γ_Q 1.50`; STR 1.35 / 1.00 / 1.50. 1.05/0.95 is not an EN value. DE NA not seen (annex under revision). | INFOST (snippet) and the Cyprus NA (snippet); L1 for STR | EN verified-public; DE unchanged | Fix `e90-26.1` text; keep 1.1/0.9 only as EN. |
| `ndp.en1990.e90-28.1` | EN 1990 Table B2/B3 | `β` 3.8 RC2, `K_FI` | `β(50 y)` = 3.3 / 3.8 / 4.3 (RC1/2/3); `K_FI` 0.9 / 1.0 / 1.1 | B90 | verified-public | Values consistent with EN. |
| `ndp.en1990.na-de.1` | ψ values | DE | snow <= 1000 m: 0.5 / 0.2 / 0; > 1000 m: 0.7 / 0.5 / 0.2; wind 0.6 / 0.2 / 0; DE `γ_G` 1.35/1.00, `γ_Q` 1.50 | L1 | single-source | Print check for imposed-load categories. |
| `ndp.en1998.sei-01.*` | EN 1998-1/NA seismic hazard | `a_gR` zones 1-3 (DIN 4149 style) | DIN EN 1998-1/NA:2023-11 uses spectral response acceleration `S_aP,R` (plateau, subsoil A-R, `T_NCR = 475 y`), digitally (`SaPR.csv`), geological classes R/T/S; no zone table | EQ25, DINM (snippet) | verified-public (edition change) | Spec built on `a_gR` zones is obsolete; re-base on NA:2023-11; no numeric value verified. |
| `ndp.en1997.l45.1`, `ndp.en1997.l183.1` | DIN 1054 Table A 2.1, A 7.1 | cite DIN 1054:2010 | DIN 1054:2010-12 is withdrawn; current DIN 1054:2021-04 (tables revised). Values not found. | DINM (snippet) | single-source (edition) | Re-base citations to 2021-04, then print check of all partial/scatter factors. |

## 3. Counts

- Table rows: 45 (a row may group several worklist ids, about 75 worklist values touched).
- verified-public: 18 rows. single-source: 16 rows. unchanged: 9 rows. Mixed (EN verified-public, DE unchanged): 2 rows.
- Candidate resolutions: the spec is right for C-class timber (E95-01/02/05) and punching reinforcement (E92-43); the repo is right for `γ_c` accidental, glulam `f_t,0/f_t,90/f_c,90` and `γ_M = 1.3`; the repo is wrong for shear `k1`, `C_Rd,c`, `v_min`, `ν1`, `cot θ`, glulam `G_mean`, C-class strengths, masonry `K`/`f_vk0`/`f_vlt`.

Remaining ~470 registry entries were not reachable with public sources in this pass (EN 1991-1-2 fire, EN 1991-2 bridge, EN 1993/1994/1999 detail tables, EN 1997 numbers, EN 1996 REI and Phi).

## 4. Second pass (EN 1993, 1994, 1999, 1997, 1998)

| Registry id | Code / clause | Value in code (spec) | Public value | Sources | Status | Action needed |
|---|---|---|---|---|---|---|
| `ndp.en1993.v-001.1`, `v-880.1`, `l398.1` | EN 1993 `γ_M` (DE) | `γ_M0` 1.0, `γ_M1` 1.1, `γ_M2` 1.25 | DE NA: 1.00 / 1.10 / 1.25 (`γ_M1` DE differs from EN 1.00) | STAAD.Pro DE-NA help (https://DOCS.BENTLEY.COM/LiveContent/web/STAAD.Pro%20Help-v20/en/GUID-CE644E83-ACCA-43B4-B90D-1A4458727A15.html, snippet) | single-source | Print check; `γ_M3`, `γ_M7` not found. |
| `ndp.en1993.l384.1` | EN 1993-1-1 5.3.2 Table 5.1 `e0,d/L` | spec table | EN Table 5.1 elastic/plastic: a0 1/350, 1/300; a 1/300, 1/250; b 1/250, 1/200; c 1/200, 1/150; d 1/150, 1/100. DE Table NA.2 uses smaller imperfections for linear section interaction (c: 1/250 elastic, 1/150 plastic; snippet). | Dlubal https://www.dlubal.com/en/support-and-learning/support/knowledge-base/001486 ; mb-news 07/2016 (snippet) | single-source (EN) | Check which table the code uses; DE NA.2 values not fully extracted. |
| (no id) | EN 1993-1-1 Table 6.1 `α` | a0..d | 0.13 / 0.21 / 0.34 / 0.49 / 0.76 | Dlubal 001622 (snippet) | single-source | none |
| `ndp.en1999.*`, `ndp.en1994.*` | NDPs | spec | DE NA numbers not found. DIN EN 1999-1-1/NA editions 2013-05 and 2017-05 are marked withdrawn on DIN pages; verify the current edition. | DIN edition listing (https://www.dinmedia.de/en/standard/din-en-1999-1-1-na/171417109, snippet) | unchanged | Print check; confirm current NA edition. |
| `ndp.en1997.l45.1`, `l183.1` | DIN 1054:2021-04 Tables A 2.1, A 7.1 | 2010 values | Table values not publicly reachable (university collection and DIN pages show only that A 2.1 was revised in 2021). | https://www.geotechnik.uni-wuppertal.de/fileadmin/bauing/geotechnik/Lehre/Vorlesungsskripte_Stand_2023/2023-03-29_Tabellensammlung_Geotechnik.pdf | unchanged | Print check against the 2021-04 edition. |
| `ndp.en1998.sei-01.*` | EN 1998-1/NA:2023-11 | spectrum parameters | `S_aP,R` digital annex; no numeric table publicly extractable | EQ25 | unchanged (numeric) | Obtain `SaPR.csv` from the NA. |

Second-pass counts: 1 single-source row group (EN 1993 `γ_M`), 2 single-source (imperfection tables, `α`), 4 unchanged. Public sources for EN 1991 snow/wind were already exhausted in section 2.

## 5. Status flip not done

The request to set `verified` on `verified-public` entries was not executed, by design:
- Registry entries are line-segment granular (e.g. `ndp.en1995.t-01.1` is the whole strength-table line); most `verified-public` findings say the spec text is wrong or the code is wrong, so setting `verified` would bless wrong content and turn `UnverifiedBasis` into `Pass` for code that still contradicts the evidence.
- The registry rule (`📌️important/ndp-verification-worklist.md`) says a human records the print edition; all evidence here is secondary (vendor manuals, lecture notes, NA reproductions).
- Editing `🔣️.json` requires regenerating `🗃️table.rs` with the ticket's `codegen.py`; the Rust and Ajv tests compare both.
Candidates to flip after the code is corrected and a human has the print: `ndp.en1992.v-02.1`, `v-03.1`, `e-03.1`, `p-04.1`, `b-01.1`, `ndp.en1995.t-01.1` (after splitting C-class and GL rows into atomic entries), `ndp.en1991.e14-02.5`, `ndp.en1990.e90-28.1`.
