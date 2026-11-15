# Fragment: one reference registry for the five energy videos (agent `consistency-refs`)

Date of all checks: 2026-11-13. ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`.

## 1. What was built

Module `ENERGY/🏗️authoring/📚️references/` (layout of `📏️running-example`):

| File | Role |
|---|---|
| `🔣️.json` | data: 55 entries (standards, guidelines, law, books, article, thesis, data sets) with id, kind, designation, edition/version/year, title de+en, status, successor, `url`, `verified`, per-use `locators` |
| `🧬️schema/🔣️.schema.json` | JSON Schema 2020-12 (kind-dependent required fields; withdrawn/superseded require a successor) |
| `🐍️.py` | pure-stdlib API: `designation(id)` ("DIN 4108-2:2026-05"), `short(id)`, `ref(id, locator, *, language)`, `chip(*parts, language)` (parts: id, `(id, locator)`, `!id` = status chip; wraps to two lines above 72 characters), `cite(*parts)` (language from `SEMIO_TEACHING_LANGUAGE`, same contract as `visuals.language`), `status_chip(id, language)`, `citation`, `bibliography(id, language)`, `card(id, language)` (lines of a bibliography card), `all_forms()` |
| `🧪️tests/🐍️.py` + `🧪️tests/🔣️.cases.json` | tests; registered in `🏗️authoring/📜️script.ts` `test` list |

Tests (10): (a) schema validation with third-party `jsonschema`; (b) every entry has edition / year / version and `url` and `verified`; unique ids and designations; (c) every withdrawn/superseded entry names its status word and a current successor in the status chip (de + en), required withdrawn/current ids listed in the cases JSON; (d) language-agnostic API cases (23 cases in the cases JSON: designation, short, ref de+en, chip, status chip de+en, bibliography, citation, card); (e) scan rules with must-flag and must-pass strings in the cases JSON; (f) the source scan `Screens.test_every_shown_designation_is_a_registry_form`: every string literal (AST, docstrings and comments excluded) of all `.py` under the five videos, `🏗️authoring/{🎨️visuals,🎬️clip,📝️narration,🗺️roadmap,📏️running-example,🔤️fonts,🎨️theme}`, every JSON string (quiz, fixtures, manifest) and every line of README/concepts md is matched with the designation regex (DIN, DIN EN, DIN EN ISO, DIN V, DIN/TS, DIN SPEC, VDI, ISO, EN, IEST-RP-CC) and fails when the text is not in `all_forms()`, when a designation has no edition, when a withdrawn/superseded edition is shown without a status word or `GModG` context, when `GEG` appears outside "vormals GEG"/"formerly GEG", when an author name (Willems, Bohne, Maas, Pettenkofer, Bundesgesundheitsbl, IWU typology) appears outside its registry citation, or when a dropped claim (`Tab. 7`, `Tab. B.2`, `Anh. B.4`, "revision 2027") survives next to a designation. `🔊️speech/**` is not scanned (not mine to edit).

Declared allow rule (cases JSON `shortAllowedIn` + `shortRuleJustification`, kept minimal): the short form without edition is allowed only inside narration clauses `(key, en, de)` (spoken and subtitled; editions are not read aloud and the beat's chip shows them), plus the registry's own status chip, which names the successor by short form. Nothing else.

## 2. Registry table

Verified = page opened or search result of the named URL on 2026-11-13. "indirect" = edition read from a DIN page that replaces/names it.

| id | designation + edition | status | URL (host) | verified how / not verified, decision |
|---|---|---|---|---|
| din-4108-2 | DIN 4108-2:2026-05 | current | dinmedia.de/en/standard/din-4108-2/401017482 | page opened: published 2026-05, replaces 2013-02. Only "Abschn. 8" kept; "Tab. 7" dropped (2026-05 text unreadable) |
| din-4108-2-2013 | DIN 4108-2:2013-02 | superseded -> 2026-05 | gesetze-im-internet.de/geg/__14.html | GModG § 14 still names "DIN 4108-2: 2013-02" (Abs. 2: Abschn. 8, Abs. 3: 8.4, Abs. 5: 8.2.2). Allowed on screen only with GModG context |
| din-4108-beiblatt-2 | DIN 4108 Beiblatt 2:2019-06 | current | dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077 | search result; Berichtigung 1:2025-02; a "2023" date on a secondary page not confirmed |
| din-v-4108-6 | DIN V 4108-6:2003-06 | withdrawn -> DIN/TS 18599 | dinmedia.de/en/pre-standard/din-v-4108-6/63939447 | page opened: "Document withdrawn", replaced by DIN/TS 18599-12:2021-04, DIN recommends the DIN V 18599 series. Kept (`shownWithoutStatus`) for the symbol notation by the reviewer's wish. **All clause numbers (3.2, 5.5, 6.1-6.5, Anh. A, B.4, D.2, D.5) dropped**, replaced by keyword locators (symbols, losses, gains, utilisation factor, climate data, heating-period method) |
| din-v-4701-10 | DIN V 4701-10:2003-08 | withdrawn -> DIN/TS 18599-5 | dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | withdrawn; replaced on screen by DIN/TS 18599-5 (+ -1 Endenergie) |
| din-ts-18599 | DIN/TS 18599:2025-10 (series) | current | dinmedia.de/en/pre-standard/din-ts-18599-1/389750965 | DIN states the 2025-10 series replaced the DIN V series |
| din-ts-18599-1 / -2 / -5 / -10 | DIN/TS 18599-1/-2/-5/-10:2025-10 | current (Vornorm) | dinmedia.de (389750965, 389744117, 389786380, 389788970) | search result + page for -10: each replaces DIN V 18599-n:2018-09. Part -6 and -8 (named in old READMEs) not on screen, not registered. 2018 clause numbers (5.5.3, 6.2 ...) dropped |
| din-v-18599, -1, -2, -10 | DIN V 18599(-n):2018-09 | superseded -> TS | gesetze-im-internet.de/geg/__20.html | GModG § 20 still names DIN V 18599:2018-09 (Abs. 1), -10 (Abs. 2), -1 (Abs. 4, 5), -2 (Abs. 6). Shown only with GModG context |
| din-en-12831-1 | DIN EN 12831-1:2017-09 | current | dinmedia.de/de/norm/din-en-12831-1/261292587 | page opened; "Abschn. 6 = detailed method" verified indirectly through the DIN/TS 12831-1 description |
| din-ts-12831-1 | DIN/TS 12831-1:2020-04 | current (national addition) | dinmedia.de/de/vornorm/din-ts-12831-1/316645651 | search result of the DIN page |
| din-en-iso-13789 | DIN EN ISO 13789:2018-04 | current | dinmedia.de/de/norm/din-en-iso-13789/259116312 | search: replaces 2008-04; "2017-10" of the old scene does not exist (corrected) |
| din-en-iso-6946 | DIN EN ISO 6946:2018-03 | current | dinmedia.de/de/norm/din-en-iso-6946/255315365 | page opened; Berichtigung 2023-04; "Tab. 7" dropped, keyword locator for R_si/R_se |
| din-en-iso-13786 | DIN EN ISO 13786:2018-04 | current | dinmedia.de/de/norm/din-en-iso-13786/255912584 | page opened |
| din-en-410 | DIN EN 410:2011-04 | current | dinmedia.de/de/standard/din-en-410/138424295 | page opened |
| din-en-673 | DIN EN 673:2025-01 | current | dinmedia.de/de/norm/din-en-673/385544906 | search result (German version of EN 673:2024, replaces 2011-04); was "EN 673" in a heating quiz |
| din-en-14825 | DIN EN 14825:2023-10 | current | dinmedia.de/de/norm/din-en-14825/343036621 | page opened |
| din-en-14511-2 | DIN EN 14511-2:2023-08 | current | dinmedia.de/de/norm/din-en-14511-2/349069586 | part 1 page opened (2023-08); part 2 edition from replacement notes. Bare "DIN EN 14511" (COP) now cites part 2 (test conditions) |
| din-en-16798-1 | DIN EN 16798-1:2022-03 | current | dinmedia.de/de/norm/din-en-16798-1/349622591 | search result; "Tab. B.2" dropped, Annex B kept |
| din-en-16798-3 | DIN EN 16798-3:2026-10 | current | dinmedia.de/en/standard/din-en-16798-3/390958579 | page opened: published 2026-10, 2017-11 withdrawn |
| din-en-16798-7 | DIN EN 16798-7:2017-11 | current | dinmedia.de/en/standard/din-en-16798-7/258797864 | page opened. C_d = 0.6 on screen is an assumption, not a clause |
| din-en-308 | DIN EN 308:2022-04 | current | din.de/en/wdc-beuth:din21:353762417 | search result (2022-04 replaces 1997); German title taken from 1997 |
| din-1946-4 | DIN 1946-4:2018-09 | current | din.de/en/wdc-beuth:din21:294438653 | audit URL; A1:2025-11 reported, not confirmable |
| din-1946-6 | DIN 1946-6:2019-12 | current | din.de (toc of DIN 1946-6 Beiblatt 1:2025-06) | indirect: Beiblatt 1:2025-06 names 2019-12 as base; forum claim "2022-09" not confirmed |
| din-1946-7 | DIN 1946-7:2022-08 | current | dinmedia.de/de/norm/din-1946-7/355461283 | search result: replaces 2009-07 |
| din-18032-1 | DIN 18032-1:2014-11 | current | dinmedia.de/de/norm/din-18032-1/222157679 | search: DIN lists 2014-11 as current (replaces 2003-09); a 2026 draft exists |
| din-18380 | DIN 18380:2019-09 | current (latest found) | dinmedia.de/de/norm/din-18380/310840113 | latest edition found; no 2023 edition; status not explicitly confirmed |
| din-51603-1 | DIN 51603-1:2024-11 | current | dinmedia.de/en/standard/din-51603-1/382208921 | search result: replaces 2020-09 |
| vdi-4650-1 | VDI 4650 Blatt 1:2024-02 | current | vdi.de (detail page) | audit-verified; JAZ is the title term, SCOP belongs to DIN EN 14825 |
| vdi-2078 | VDI 2078:2015-06 | current | vdi.de (detail page) | audit-verified; "revision announced for 2027" removed everywhere (no source) |
| vdi-4645 | VDI 4645:2023-04 | current | dinmedia.de/de/technische-regel/vdi-4645/364873293 | search: 2023-04 current (replaces 2018-03); a newer draft / Blatt 2:2026-01 (training) seen, no new main edition confirmed |
| vdi-6022-1 | VDI 6022 Blatt 1:2018-01 | current | vdi.de (detail page) | audit-verified; Berichtigung 2023-04 |
| vdi-3807-1 | VDI 3807 Blatt 1:2013-06 | current | vdi.de (detail page) | audit-verified; G20/15 locator kept as keyword |
| vdi-2052-1 | VDI 2052 Blatt 1:2017-04 | current | vdi.de (detail page) | search result (replaces 2006-04); bare "VDI 2052" now cites Blatt 1 |
| vdi-2087 | VDI 2087:2006-12 | current | vdi.de (detail page) | search result; Berichtigung 2008-04 |
| iso-14644-4 | ISO 14644-4:2022 | current | dinmedia.de/en/standard/iso-14644-4/362298406 | search result (2nd edition 2022); claim "0.2-0.5 m/s" in the quiz not verified in the text |
| iso-7730 | ISO 7730:2025 | current | iso.org/standard/85803.html | search: 4th edition 2025-09 withdraws 2005 |
| iso-8996 | ISO 8996:2021 | current | iso.org/standard/74443.html | search: 3rd edition 2021-12 |
| iso-52016-1 | ISO 52016-1:2017 | current | iso.org/standard/65696.html | search: confirmed 2022 |
| gmodg | GModG (Stand 2026-07-29) | current law | gesetze-im-internet.de/geg/BJNR172810020.html | OFFICIAL source opened: title "Gesetz zur Einsparung von Energie und zur Modernisierung der Wärmeversorgung in Gebäuden (Gebäudemodernisierungsgesetz - GModG)", changed by Art. 4 of the act of 2026-07-23, BGBl. 2026 I Nr. 226, heading changed with effect from 2026-07-29. Paragraph headings checked in the table of contents: 14, 15, 16, 20, 22, 60c, 71 (weggefallen, 71-73), 79-82, 84, 86, 87, Anlagen 1, 4, 10. §§ 14, 15, 20, 86 text opened. "Abs." numbers and "Anlage 4 Nr. 2, 8, 9, 12" **dropped** (not verified) |
| bohne-2026 | Bohne, Bohne (2026), 13. Aufl. | book | baunetzwissen.de/.../technischer-ausbau-von-gebaeuden-4347729 | page opened: title "Gebäudetechnik - Technischer Ausbau und nachhaltige Gebäudesysteme", authors Dirk and Marius Bohne, Springer Vieweg Heidelberg, ISBN 978-3-658-51570-6 (e-book -51571-3) |
| willems-2024 | Willems, Schild, Stricker (2024), 8. Aufl. | book | jpc.de/.../formeln-und-tabellen-bauphysik/hnum/11834083 | see decisions |
| maas-1995 | Maas (1995), Diss. Univ. Kassel, Kap. 8.2, S. 74 | thesis | uni-kassel.de/.../maas_diss.pdf | audit-verified (S. 74: 0.56-1.38 1/h) |
| pettenkofer-1858 | Pettenkofer (1858), München: Cotta | book | lessing.ub.uni-duesseldorf.de/.../2385693.pdf | audit-verified title page |
| adhoc-ag-2008 | Bundesgesundheitsbl. 51 (2008) 1358-1369 | article | link.springer.com/.../10.1007/s00103-008-0707-2.pdf | audit-verified, DOI |
| iwu-typologie-2015 | IWU Wohngebäudetypologie (2015), 2. Aufl. | data set | iwu.de/.../2015_IWU_LogaEtAl_Deutsche-Wohngebäudetypologie.pdf | search: 2nd extended edition 2015-02-10, ISBN 978-3-941140-47-9, no later edition found. "Tabelle 26" dropped |
| sunikka-galvin-2012 | Sunikka-Blank, Galvin (2012), Build. Res. Inf. 40(3) | article | structurae.net/... | search: 260-273, DOI 10.1080/09613218.2012.690952 |
| sdes-bilan-2023 | SDES, Bilan énergétique de la France pour 2023 | data set | statistiques.developpement-durable.gouv.fr | page found, figures of the quiz not re-read |
| fraunhofer-ise-pv-fakten | Fraunhofer ISE, Aktuelle Fakten zur Photovoltaik in Deutschland | data set | ise.fraunhofer.de | living document 2026-05-05, quiz value not re-read |
| dwd-try-2017 | DWD TRY 2017 | data set | baulinks.de/webplugin/2017/0983.php4 | DWD page not retrievable; secondary page |

**Not registered, claim dropped from screen:** IEST-RP-CC012 (current revision unverifiable; ventilation quiz items now say "typical guideline value" without a source), DIN 4108-3 (edition unverified; concepts dew point row loses the attribution and the "-3" in the physics row), the λ attribution "from DIN 4108" in the conduction scene (DIN 4108-4 would be the source, not verified), the label "(DIN 4108)" at the insulation layer, "Fraunhofer ISE heat pump field test" (no concrete source in the quiz, left as text), DIN V 18599-6/-8 (not on screen).

## 3. Decisions

- **GModG.** Confirmed on the official site: the law was renamed, the abbreviation is GModG, effective 2026-07-29 (BGBl. 2026 I Nr. 226). Every shown "GEG" is now "GModG" (quiz, README, scene labels); "GEG" survives only as "vormals GEG"/"formerly GEG" (regulations law card, primary-energy law note) and is enforced by the scan. "GModG 2024" in the quizzes (anachronism) became "GModG". The statutory text still names DIN 4108-2:2013-02 (§ 14) and DIN V 18599:2018-09 (§ 20), so the registry keeps both editions as `superseded` entries usable only with GModG context; scenes show the current editions (2026-05, DIN/TS 18599 2025-10) and the primary-energy scene says "GModG § 20 still cites DIN V 18599-1:2018-09". The content of the heating quiz about the legal requirements (e.g. 55 %/70 % H'_T values, Anlage 1 values) was renamed but **not re-verified against the new text** (open point).
- **Willems.** The audit mixed two books: ISBN 978-3-658-44973-5 (as on screen) belongs to "Formeln und Tabellen Bauphysik", 8. Aufl. 2024 (Springer Vieweg, 2024-08-31, e-book/DOI 10.1007/978-3-658-44974-2; confirmed by jpc listing and the DOI the scene already showed); 978-3-658-44971-1 is "Praxisbeispiele Bauphysik". The scene cites the former, so the ISBN stays and the registry records the correction. Original German titles are used in both languages for all German works (findability).
- **Bohne.** Title on screen is now exactly the publisher title "Gebäudetechnik - Technischer Ausbau und nachhaltige Gebäudesysteme" (card via `card("bohne-2026")`).
- **Withdrawn handling.** DIN V 4108-6:2003-06 stays for the notation; chip text "DIN V 4108-6:2003-06 (zurückgezogen) – Symbolik; aktuell: DIN/TS 18599" (en: "(withdrawn) – notation; current: DIN/TS 18599") is shown ONCE per video: heating `🧮️calculation/Scene2` (replaces the source line "Heating-period method of DIN V 4108-6 (Annex D.2)", two lines with VDI 3807 Blatt 1:2013-06); cooling `⚖️heating-comparison/Beat1_WinterGains` (second line below the DIN/TS 18599-2 chip); demand `🔁️recap/Beat1_Bilanzgrenze` (replaces the old `3.2 · 5.5` chip). Physics: no balance, none. Ventilation: no DIN V 4108-6 citation at all, none. DIN V 4701-10:2003-08 is replaced on screen by DIN/TS 18599-5:2025-10 (+ -1 Endenergie) in `♨️system-losses/Beat4`; the withdrawn marking lives in the registry and README.
- **Clause numbers.** Every unverified clause/table number was replaced by a verifiable keyword locator or dropped (see table). Verified as is: DIN 4108-2 Abschn. 8 (also named by GModG § 14), DIN EN 12831-1 Abschn. 6, Maas Kap. 8.2 S. 74, GModG paragraph headings.
- **Layout.** Chips longer than 72 characters wrap to a second line (`chip`); two-part chips such as GModG + DIN/TS fit. The visual check of every chip stays with the parent.

## 4. Changed files (file:line groups)

Chips now `_din_ref(_references.cite(...))`, `import` of `_references` added after `tr = _manim_module.tr` in each file (ENERGY-relative):
see `🗑️generated/consistency-out/refs/usage-lines.txt` for the exact lines; groups:
cooling: `☀️solar-radiation` 202,433; `⚖️heating-comparison` 274 (status chip),342,412; `🌤️summer-strategy` 172,305,502; `👥️internal-gains` 202,284,585,792,910; `💧️transmission-humidity` 233,384; `📐️system-sizing` 230,403.
ventilation: `⚙️mechanical` 280,454,736,943; `❓️need` 247 (Pettenkofer + Bundesgesundheitsbl. instead of DIN 1946-6),284 (label with "(1858)"),438; `🌡️heat-flow` 304,479,623,786; `📦️volume` 487,733,928; `🤝️handoff` 279,455; `🪟️natural` 346,591,592 (Maas, Kap. 8.2 S. 74),820,1032,1321,1558.
demand: `♨️system-losses` 586,774,1003; `⚖️regulations` 569 (law card),586,719,953,1140; `⚡️demand-power` 434,630,891,1087; `🏭️primary-energy` 416,421,422,627,955,1188; `🏷️certificate` 341,544,785; `🔁️recap` 468 (status chip),659,989,1273; `🧪️practice` 313,570,707,1069.
heating: `☀️solar-gains` 393,517,643,757,892,1122; `🌬️convection` 172; `👥️internal-gains` 336,367,438,542; `🧮️calculation` 640 (status chip); `🧱️conduction` 387,579,697,882,1044,1240 (+ narration "DIN 4108-2" at the minimum-resistance clause, "DIN 4108 Beiblatt 2", label "Insulation layer", λ clause without false attribution); `🧲️fundamentals` 634,744,921,1078,1158,1273,1502,1663.
physics `🧲️fundamentals`: 1920 (JAZ/SCOP label), 2137 (EN 410), 2266 (4108-2), 2600 (13786), 3183-3190 (bibliography cards via `card()`); narration `jaz_ref` "VDI 4650 Blatt 1" (the English clause said "part 1").
Prose strings (labels, narration, quiz, README, concepts, fixtures) rewritten by script to exact registry forms (non-narration: edition appended; narration: short form; DIN V 18599(-n) -> DIN/TS 18599(-n); DIN EN 12831 -> DIN EN 12831-1; EN 16798-1 -> DIN EN 16798-1:2022-03; VDI 4650-1 -> VDI 4650 Blatt 1; Bbl. 2 -> Beiblatt 2; clause numbers after DIN V 4108-6 removed; GEG -> GModG; IWU citation -> "IWU Wohngebäudetypologie (2015)", "Tabelle 26" removed): quiz JSON of all five videos, `📖️concepts/📝️.md`, the five `🎬️clip/README.md` (cooling, demand and ventilation README paragraphs rewritten by hand: references on screen, 2027 revision claim removed, Pettenkofer/Maas/Bundesgesundheitsbl. lines), `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` (DIN 1946-6:2019-12), `🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🔣️.json`, `📊️demand/🎬️clip/🔁️recap/🧪️tests/🧮️balance/🔣️.json`, `🌬️ventilation/🎬️clip/🧪️architecture/🔣️.numbers.json` (Maas source), narration in `📦️volume`, `👥️internal-gains` (heating), `⚖️heating-comparison`, `🧮️calculation`, `♨️system-losses`, `⚖️regulations`, `🏭️primary-energy`, `🧪️practice`.
New: `🏗️authoring/📚️references/**`; one line in `🏗️authoring/📜️script.ts` (test registration).
Scripts of this task: `TK/consistency/refs/{inventory,grep_lines,manifest_order,apply_chips,apply_text,fix_law_names}.py`, `run-tests.sh`, `family-tests.sh`.

## 5. Verification run

Narration guard clean. Passing (SEMIO_TEACHING_PROFILE=preview): references (10), visuals `scenes` (15), `labels` (5), fonts heading (3), demand recap balance / chain / primary-energy chain+scale, ventilation volume + architecture (numbers, transitions), physics fundamentals (13), heating conduction (2), cooling system-sizing (2). Visuals `integration`: 8 of 9 pass; `test_physics_scene_constructor_bounds` fails on the radiator staffage height (0.681 vs 0.738), unrelated to references (staffage geometry of another agent). No renders were run.

## 6. Open points

- The symbol agent may want to render `F_C` in the registry locator text `Abschn. 8 – Sonnenschutzvorrichtung F_C` (plain text, underscore shown literally); same for `R_si, R_se`, `e_p`.
- Legal facts quoted in the heating/cooling/demand quizzes (55 %/70 % H'_T limits, Anlage 1 values, "65-%-rule repealed", efficiency classes) were renamed to GModG but not re-read against the new text; GModG § 14/§ 20 text confirms only the DIN editions.
- Unverified claims left on screen as text: Fraunhofer ISE heat pump field test, SDES and Fraunhofer ISE quiz numbers (sources registered, figures not re-read), DIN 1946-4 values (60 m³/(h·m²), 20 1/h), DIN 18032-1 "60 m³/h per athlete", ISO 14644-4 "0.2-0.5 m/s", DIN 51603-1 calorific value; editions are verified, the numbers are not.
- DIN 1946-4:2018-09/A1:2025-11, DIN 18380 current status and VDI 4645 successor not confirmable.
- Long chips (law + standard) now wrap to two lines; visual check of overlap with the heading and the caption is for the parent.
- Not checked: `🔊️speech/🗣️lexicon/🔣️.json` mentions GEG (speech agent's file, outside the scan).
