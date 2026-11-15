# Section 4 reference audit (APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS)

Read-only audit. Date of checks: 2026-11-13. Paths are relative to `🎓️teaching/🏛️architecture/⚡️energy/` unless stated.
Occurrence source: grep of the energy tree (`audit2/g.py`, pattern files `p-refs.txt` and `p-extra.txt`) into `audit2/all-hits.txt`.
Counts are per reference row; a line that cites two references appears in both rows.

## (a) Reference table

Distinct references: 47. Verdicts: VERIFIED 2, EDITION-ONLY 12, UNVERIFIED 17, WRONG 16.

| # | Displayed string | Occurrences (n, full list in d) | Edition/year | Locator | Report claims verified | Web result | Source URL | Verdict | Proposed replacement |
|---|---|---|---|---|---|---|---|---|---|
| 1 | DIN 4108-2:2013-02 | 9 (d1) | Y (2013-02) | N | cooling: WebSearch 'Abschn. 8, Tab. 7' of 2013-02 (claimed verified) | 2013-02 superseded: DIN lists DIN 4108-2:2026-05 as current/published (2026-05 TOC preview shows section 8) | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **WRONG** | DIN 4108-2:2026-05, Abschn. 8 (Sommerlicher Wärmeschutz) |
| 2 | DIN 4108-2 (no edition) | 11 (d2) | N | N | demand, cooling, heating (claimed, no edition) | current edition DIN 4108-2:2026-05 | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **WRONG** | DIN 4108-2:2026-05 |
| 3 | DIN 4108-2:2026-05 | 2 (d3) | Y | partial (Abschn. 8 in TOC preview; Tab. 7 not verified) | demand: 'final vs pre-published unconfirmed' (not verified by demand) | edition confirmed as published/current; Tab. 7 F_C values not read | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **EDITION-ONLY** | DIN 4108-2:2026-05, Abschn. 8 |
| 4 | DIN V 4108-6:2003-06 (and bare 'DIN V 4108-6') | 51 (d4) | Y (2003-06) / N | claimed 3.2, 5.5, 6.1-6.5, Anh. A, B.4, D.2, D.5 (not checkable: standard withdrawn) | heat-b (TOC only), demand ('Verified locators' 3.2, 5.5, 6.1-6.5, Anh. A, D.2, D.5) | WITHDRAWN (DIN listing); DIN recommends DIN V 18599 parts and DIN/TS 18599-12:2021-04; a corrigendum is also withdrawn; DIN 4108-6:2024 not found | https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447 | **WRONG** | DIN/TS 18599-12:2021-04 (successor per DIN; clause numbers must be re-checked, do not keep 3.2/5.5/6.x) |
| 5 | DIN V 18599-2 (2016-10 / 2018-09 / no part edition) | 5 (d5) | Y (both 2016-10 and 2018-09 used) | claimed 5.5.3 (demand, 2018-09) | demand (2018-09 5.5.3 'verified'), cooling (2016-10 'Interne Wärmequellen') | DIN listing marks DIN V 18599-2 as WITHDRAWN; successor for part 2 not found (2025-10 TS parts exist for -4 and -10) | no DIN URL retrieved for part 2 | **WRONG** | no verified replacement; cite the DIN/TS 18599-2 part of the 2025-10 series only after a DIN check |
| 6 | DIN V 18599-1:2018-09 | 7 (d6) | Y | N | none (not claimed by reports) | not checked individually; DIN/TS 18599-1:2025-10 is named in demand files | - | **UNVERIFIED** | check status at DIN; if superseded use DIN/TS 18599-1:2025-10 |
| 7 | DIN V 18599 / DIN V 18599:2018-09 (no part) | 19 (d7) | mixed (bare or 2018-09) | N | demand (energy balance text) | series superseded in part by DIN/TS 18599 2025-10 (confirmed for parts -4, -10) | - | **UNVERIFIED** | name the part and edition explicitly, e.g. DIN V 18599-1:2018-09 or its 2025-10 successor |
| 8 | DIN V 18599-5:2018-09 | 7 (d8) | Y | N | demand, heating (not claimed verified) | not verified; cited as a recommended successor of withdrawn DIN V 4701-10 | https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | **UNVERIFIED** | verify status at DIN before use |
| 9 | DIN V 18599-10:2018-09 (internal heat sources) | 9 (d9) | Y (2018-09) | N | heating/demand (not claimed verified) | replaced by DIN/TS 18599-10:2025-10 (DIN listing of both editions) | https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970 | **WRONG** | DIN/TS 18599-10:2025-10 (Vornorm, replaces DIN V 18599-10:2018-09) |
| 10 | DIN/TS 18599 parts 2025-10 | 4 (d10) | Y (2025-10) | N | demand (2025-10 notes) | DIN/TS 18599-4:2025-10 and -10:2025-10 confirmed; -1 and -2 mapping not confirmed | https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970 | **UNVERIFIED** | keep only parts confirmed at DIN |
| 11 | DIN EN 12831-1 (no edition) | 34 (d11) | N | claimed Abschn. 6 (demand) | demand: 'Verified locators: DIN EN 12831-1:2017-09 Abschn. 6' (only with edition) | edition DIN EN 12831-1:2017-09 current; national annex DIN/TS 12831-1:2020-04 (secondary source) | https://www.dinmedia.de/de/norm/din-en-12831-1/261292587 | **WRONG** | DIN EN 12831-1:2017-09 |
| 12 | DIN EN 12831-1:2017-09 | 7 (d12) | Y | claimed Abschn. 6 (not verified) | demand (Abschn. 6 claimed verified) | edition confirmed (DIN); Abschn. 6 not read | https://www.dinmedia.de/de/norm/din-en-12831-1/261292587 | **EDITION-ONLY** | DIN EN 12831-1:2017-09 |
| 13 | DIN/TS 12831-1:2020-04 (national annex) | 2 (d13) | Y (2020-04) | N | demand (named) | existence via Haustechnikdialog snippet; DIN page not opened | https://www.haustechnikdialog.de/SHKwissen/3109/DIN-TS-12831-1 | **EDITION-ONLY** | DIN/TS 12831-1:2020-04 (national annex to DIN EN 12831-1:2017-09) |
| 14 | DIN EN ISO 13789:2017-10 | 2 (d14) | Y (2017-10 claimed) | N | conduction file (not verified by heat-b) | no 2017-10 edition found; DIN lists DIN EN ISO 13789:2018-04 (German version of ISO 13789:2017, replaces 2008-04) | https://www.dinmedia.de/en/standard/en-iso-13789/277780736 | **WRONG** | DIN EN ISO 13789:2018-04 |
| 15 | DIN EN ISO 6946:2018-03 (and bare) | 16 (d15) | Y (2018-03) | claimed Tab. 7 (secondary sources only; not verified) | heat-a (edition and Tab. 7 'confirmed via secondary sources only') | 2018-03 current, Berichtigung 1:2023-04; Tab. 7 not verified | https://www.dinmedia.de/de/norm/din-en-iso-6946/255315365 | **EDITION-ONLY** | DIN EN ISO 6946:2018-03 (drop 'Tab. 7' unless checked) |
| 16 | DIN 4108 Beiblatt 2:2019-06 (and bare) | 4 (d16) | Y (2019-06) | N | heat-a (edition 'confirmed via secondary sources') | 2019-06 current; Berichtigung 1:2025-02 exists (CD-ROM replacement) | https://www.dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077 | **EDITION-ONLY** | DIN 4108 Beiblatt 2:2019-06 (Berichtigung 1:2025-02 beachten) |
| 17 | DIN EN 16798-1:2022-03 (and bare) | 6 (d17) | Y (2022-03) | claimed Anh. B.2 / Tab. B.2 (cooling); Tab. B.9 not cited (vent-a) | cooling (Anh. B.2 'verified') | 2022-03 current (replaces 2021-04); Anhang B is informative criteria; table numbers not read | https://www.dinmedia.de/de/norm/din-en-16798-1/349622591 | **EDITION-ONLY** | DIN EN 16798-1:2022-03, Anh. B (drop 'Tab. B.2' unless checked) |
| 18 | DIN EN 16798-3 (no edition) | 2 (d18) | N | N | cooling, ventilation (no edition) | DIN EN 16798-3:2017-11 withdrawn; successor DIN EN 16798-3:2026-10 per DIN timeline (some old pages still show 2017-11 as current) | https://www.dinmedia.de/en/standard/din-en-16798-3/390958579 | **WRONG** | DIN EN 16798-3:2026-10 (check clause numbers) |
| 19 | DIN EN 16798-7:2017-11 | 3 (d19) | Y (2017-11) | N (procedure 1/2 exists; formulas not read) | ventilation natural (2017-11 'verified'; C_d = 0,6 not from a clause) | current; replaces DIN EN 15242:2007-09 | https://www.dinmedia.de/en/standard/din-en-16798-7/258797864 | **EDITION-ONLY** | DIN EN 16798-7:2017-11 (C_d = 0,6 is an assumption, cite as such) |
| 20 | DIN EN 410 (no edition) | 6 (d20) | N | N | physics, concepts | current edition DIN EN 410:2011-04 | https://www.dinmedia.de/de/standard/din-en-410/138424295 | **WRONG** | DIN EN 410:2011-04 |
| 21 | DIN EN 14825 (no edition) | 5 (d21) | N | N (SCOP keyword) | physics, concepts | current DIN EN 14825:2023-10 (German version of EN 14825:2022); A1 draft only | https://www.dinmedia.de/de/norm/din-en-14825/343036621 | **WRONG** | DIN EN 14825:2023-10 (SCOP) |
| 22 | DIN EN ISO 13786 (no edition) | 4 (d22) | N | N | physics | current DIN EN ISO 13786:2018-04 (replaces 2008-04) | https://www.dinmedia.de/de/norm/din-en-iso-13786/255912584 | **WRONG** | DIN EN ISO 13786:2018-04 |
| 23 | DIN EN 14511 | 2 (d23) | N | N | concepts (not verified) | not checked | - | **UNVERIFIED** | check edition or drop |
| 24 | DIN EN 308 | 1 (d24) | N | N | ventilation mechanical (not verified) | not checked | - | **UNVERIFIED** | check edition or drop |
| 25 | VDI 4650 Blatt 1:2024-02 (and bare 'VDI 4650') | 10 (d25) | Y (2024-02) / N (bare) | Y (JAZ term in title) | physics (2024-02 verified, Jahresarbeitszahl) | VDI 4650 Blatt 1:2024-02 is the current edition; title 'Berechnung der Jahresarbeitszahl von Wärmepumpenanlagen' | https://vdi.de/en/home/vdi-standards/details/vdi-4650-blatt-1-calculation-of-the-seasonal-coefficient-of-performance-of-heat-pumps-electric-heat-pumps-for-space-heating-and-domestic-hot-water | **VERIFIED** | VDI 4650 Blatt 1:2024-02 (unify bare variants) |
| 26 | VDI 2078:2015-06 (and bare) | 30 (d26) | Y (2015-06) / N (bare) | partial (keywords; 'Kühllast' in title; 'innere Wärmequellen' not verified) | cooling/heating (2015-06 'verified', keyword locators only) | 2015-06 current; no successor or 2027 revision found (the README claim of a 2027 revision is UNVERIFIED) | https://vdi.de/en/home/vdi-standards/details/vdi-2078-calculation-of-thermal-loads-and-room-temperatures-design-cooling-load-and-annual-simulation | **EDITION-ONLY** | VDI 2078:2015-06 (remove 'revision announced for 2027' unless verified) |
| 27 | VDI 4645:2023-04 | 2 (d27) | Y (2023-04) | N | demand (2023-04 'verified') | 2023-04 issued; a draft VDI 4645:2026-03 was seen; current status not confirmed | https://www.dinmedia.de/de/technische-regel-entwurf/vdi-4645/399378368 | **UNVERIFIED** | check status on vdi.de before citing |
| 28 | VDI 6022 Blatt 1 (no edition) | 1 (d28) | N | N | cooling (no edition) | current VDI 6022 Blatt 1:2018-01 (Berichtigung 2023-04) | https://vdi.de/richtlinien/details/vdi-6022-blatt-1-ventilation-and-indoor-air-quality-hygiene-requirements-for-ventilation-and-air-conditioning-systems-and-units-vdi-ventilation-code-of-practice-1 | **WRONG** | VDI 6022 Blatt 1:2018-01 |
| 29 | VDI 3807 Blatt 1 (no edition) | 4 (d29) | N | N (G20/15 not verified) | heating (G20/15 'secondary sources only', VDI 3807 not read) | current VDI 3807 Blatt 1:2013-06 (replaces 2007-03) | https://vdi.de/en/home/vdi-standards/details/vdi-3807-blatt-1-verbrauchskennwerte-fuer-gebaeude-grundlagen | **WRONG** | VDI 3807 Blatt 1:2013-06 (G20/15 locator still to be checked) |
| 30 | VDI 2052 | 2 (d30) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 31 | VDI 2087 | 1 (d31) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 32 | DIN 1946-4:2018-09 (and bare) | 9 (d32) | Y (2018-09) / N (bare) | N (values 60 m³/(h·m²), 20 1/h not verified) | cooling (2018-09 'verified') | 2018-09 current (title 'Raumlufttechnische Anlagen in Gebäuden und Räumen des Gesundheitswesens'); amendment A1:2025-11 exists | https://6sku3uaup9f9.din.de/en/standard/din-1946-4/294438653 | **EDITION-ONLY** | DIN 1946-4:2018-09 (check A1:2025-11) |
| 33 | DIN 1946-6 (no edition) | 15 (d33) | N | N | ventilation (many, no edition); vent-a claims Pettenkofer link | edition not confirmed; 2009-05 was the prior edition, a 2017 draft was announced; 2019-12 claim not confirmed | - | **UNVERIFIED** | confirm current edition at Beuth before adding it (e.g. DIN 1946-6:YYYY-MM) |
| 34 | DIN 1946-7 | 2 (d34) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 35 | DIN 18032-1 | 2 (d35) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 36 | DIN 18380:2019-09 | 1 (d36) | Y (2019-09) | N | none | not checked | - | **UNVERIFIED** | check edition |
| 37 | DIN 51603-1 | 4 (d37) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 38 | DIN V 4701-10:2003-08 | 3 (d38) | Y (2003-08) | N | demand (heat-related text, not verified) | WITHDRAWN (DIN listing); DIN recommends DIN V 18599-5/-6/-8:2018-09 (not a formal replacement) | https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | **WRONG** | drop, or cite DIN V 18599-5:2018-09 / -6 / -8 as the recommended successors |
| 39 | GEG / GModG (law) | 85 (d39) | Y (GEG 2024 used) | N (paragraph numbers not verified) | demand (GEG §15/§20 'verified' on gesetze-im-internet.de; the GModG rename was flagged for checking) | GModG promulgated 28.07.2026 (BGBl. I Nr. 226) and in force 29.07.2026, renaming the GEG (see section c) | https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284 | **WRONG** | Gebäudemodernisierungsgesetz (GModG, vormals GEG) - paragraph numbers to be checked against the new text |
| 40 | Bohne, Fachbuch Gebäudetechnik, 13. Aufl. 2026, ISBN 978-3-658-51570-6 | 1 (d40) | Y (13. Aufl. 2026) | N | physics (13th ed. 2026, ISBN 978-3-658-51570-6 'verified' via baunetzwissen) | ISBN 978-3-658-51570-6 belongs to Dirk Bohne / Marius Bohne, Springer Vieweg 2026, 13th ed.; the on-screen title must match this book's title | https://www.baunetzwissen.de/gebaeudetechnik/tipps/buecher/technischer-ausbau-von-gebaeuden-4347729 | **EDITION-ONLY** | Bohne, D.; Bohne, M.: Fachbuch Gebäudetechnik, 13. Aufl., Springer Vieweg 2026, ISBN 978-3-658-51570-6 |
| 41 | Willems / Schild / Stricker, 8. Aufl. 2024, ISBN 978-3-658-44973-5 | 1 (d41) | Y (8. Aufl. 2024 claimed) | N | physics (ISBN 978-3-658-44973-5 'verified') | 8. Aufl. 2024 of 'Praxisbeispiele Bauphysik' exists with ISBN 978-3-658-44971-1 (9783658449711); the printed ISBN 978-3-658-44973-5 was not confirmed | https://www.jpc.de/jpcng/books/detail/-/art/wolfgang-m-willems-praxisbeispiele-bauphysik/hnum/11834057 | **WRONG** | Willems, W. M.; Schild, K.; Stricker, D.: Praxisbeispiele Bauphysik, 8. Aufl., Springer 2024, ISBN 978-3-658-44971-1 (ISBN to confirm at Springer) |
| 42 | Maas 1995 | 4 (d42) | Y (1995, Diss. Univ. Gesamthochschule Kassel) | Y (Kap. 8.2 'Luftwechsel', S. 74; values 0,56-1,38 1/h at 58,5 m³ room) | vent-b (ch. 8, 0,56-1,38 1/h, pages ~74-75 estimated) | dissertation 'Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung', eingereicht 27.06.1995; Kap. 8 'Luftwechsel und Lüftungswärmeverluste in der Heizzeit' S. 73-81; S. 74 gives 0,56 bis 1,38 1/h for a permanently tilted window (Heizgrenztemperatur 15 °C) | https://www.uni-kassel.de/fb6/bpy/de/forschung/abgeschlprojekte/pdfs/maas_diss.pdf | **VERIFIED** | Maas, A. (1995): Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung, Diss. Univ. Gesamthochschule Kassel, Kap. 8.2, S. 74 |
| 43 | Pettenkofer 1858 (Ueber den Luftwechsel in Wohngebäuden) | 3 (d43) | Y (1858) | N | vent-a (1858 cited; no locator) | title page: M. v. Pettenkofer, Ueber den Luftwechsel in Wohngebäuden, München, Cotta, 1858 (one bibliography says Brunswick; title page says Munich) | https://lessing.ub.uni-duesseldorf.de/download/pdf/2385693.pdf | **EDITION-ONLY** | M. v. Pettenkofer: Ueber den Luftwechsel in Wohngebäuden, München: Cotta, 1858 |
| 44 | Bundesgesundheitsbl. 51 (2008) 1358-1369 (Ad-hoc-AG Innenraumrichtwerte) | 3 (d44) | Y (2008, vol. 51) | N (article title confirmed; 1000 ppm value in the text not read) | vent-a (confirmed by web search) | 'Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft', DOI 10.1007/s00103-008-0707-2, published 29.11.2008; pages 1358-1369 | https://link.springer.com/content/pdf/10.1007/s00103-008-0707-2.pdf | **EDITION-ONLY** | Ad-hoc-AG Innenraumrichtwerte: Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft, Bundesgesundheitsbl. 51 (2008) 1358-1369, DOI 10.1007/s00103-008-0707-2 |
| 45 | IWU Deutsche Wohngebäudetypologie 2015 | 62 (d45) | Y (2015 claimed) | N | demand (no verification recorded) | not checked in this audit | - | **UNVERIFIED** | check the edition and cite the table/typology code |
| 46 | IEST-RP-CC012 | 4 (d46) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 47 | ISO 14644-4 | 2 (d47) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |

## (b) Conflicting editions across files

- DIN 4108-2: 2013-02 in cooling (quiz, README, solar, heating-comparison, summer-strategy), heating conduction and ventilation natural; 2026-05 in demand (README, regulations). The 2013-02 edition is superseded.
- DIN V 18599-2: 2016-10 (cooling heating-comparison, README) vs 2018-09 (demand regulations, recap). Both are withdrawn per DIN; part 2 successor not confirmed.
- DIN V 18599-10: 2018-09 in heating/demand; DIN/TS 18599-10:2025-10 replaces it (confirmed).
- DIN EN 12831-1: bare (about 20 files) vs DIN EN 12831-1:2017-09 (demand) vs DIN/TS 12831-1:2020-04 (national annex, named in demand).
- DIN EN ISO 13789: 2017-10 in conduction vs 2018-04 current.
- DIN EN 16798-3: bare (cooling, ventilation) vs withdrawn 2017-11 vs 2026-10 successor.
- DIN 4108 Beiblatt 2: bare in heating vs 2019-06 in concepts/conduction.
- VDI 4650 Blatt 1: bare 'VDI 4650' and '-1:2024-02' variants vs 'Blatt 1:2024-02'.
- DIN V 4108-6:2003-06 (withdrawn) vs bare in demand/heating; the 'F_C' / 'Annex B.4' citation depends on it.
- GEG 2024 (cooling quiz, demand README) vs GModG 2026 (demand chips, GEG renamed on 29.07.2026): name in use is mixed.
- Willems: printed ISBN 978-3-658-44973-5 vs found 978-3-658-44971-1 (same 8th ed. 2024 'Praxisbeispiele Bauphysik').
- Bohne: ISBN 978-3-658-51570-6 is verified for the 13th ed. 2026 (Dirk and Marius Bohne, Springer Vieweg); the card title must match.

## (c) GEG / GModG status

- Verified: the Gebäudemodernisierungsgesetz (GModG) was promulgated in the BGBl. on 28.07.2026 (BGBl. I Nr. 226) and the Gebäudeenergiegesetz is renamed effective 29.07.2026. Sources: https://cci-dialog.de/wp-content/uploads/2026/08/Bundesgesetz-zur-Ueberfuehrung-des-GEG-ins-GModG.pdf ; https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284 ; https://www.baunetzwissen.de/drucken/gebaeudemodernisierungsgesetz-gmodg-3559433
- Conflicting vote dates in secondary sources (Bundestag 9, 10 or 14 July 2026); the promulgation date 28.07.2026 is consistent across sources. Do not cite the vote dates.
- Not verified: the paragraph numbers (§14, §15, §20, §85, Anlage 4, Anlage 10) under the new text; the draft amendment that moves the DIN 4108-2 reference to 2026-05 (not confirmed in force).
- Required change: where the files say 'GEG 2024' as the current law, write 'Gebäudemodernisierungsgesetz (GModG, vormals GEG, Stand 2026-07-29)'.

## (d) Occurrence index (every match, file:line, paths relative to the energy folder)

### d1. DIN 4108-2:2013-02 (9)

❄️cooling/❓️quiz/🔣️.json:101, ❄️cooling/❓️quiz/🔣️.json:102, ❄️cooling/❓️quiz/🔣️.json:117, ❄️cooling/❓️quiz/🔣️.json:118, ❄️cooling/🎬️clip/README.md:48, ❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:434, ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:343, ❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:173, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:388

### d2. DIN 4108-2 (no edition) (11)

❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:100, 🌬️ventilation/🎬️clip/README.md:68, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1335, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:557, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:558, 📖️concepts/📝️.md:368, 📖️concepts/📝️.md:39, 📖️concepts/📝️.md:40, 📖️concepts/📝️.md:573, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:157, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3192

### d3. DIN 4108-2:2026-05 (2)

📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:648

### d4. DIN V 4108-6:2003-06 (and bare 'DIN V 4108-6') (51)

📊️demand/❓️quiz/🔣️.json:964, 📊️demand/❓️quiz/🔣️.json:965, 📊️demand/🎬️clip/README.md:27, 📊️demand/🎬️clip/README.md:61, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:1142, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:116, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:721, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:955, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:436, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:893, 📊️demand/🎬️clip/🔁️recap/🐍️.py:113, 📊️demand/🎬️clip/🔁️recap/🐍️.py:1275, 📊️demand/🎬️clip/🔁️recap/🐍️.py:470, 📊️demand/🎬️clip/🔁️recap/🐍️.py:661, 📊️demand/🎬️clip/🔁️recap/🐍️.py:958, 📊️demand/🎬️clip/🔁️recap/🐍️.py:959, 📊️demand/🎬️clip/🔁️recap/🐍️.py:991, 📊️demand/🎬️clip/🔁️recap/🧪️tests/🧮️balance/🔣️.json:4, 📖️concepts/📝️.md:333, 📖️concepts/📝️.md:368, 📖️concepts/📝️.md:375, 📖️concepts/📝️.md:401, 📖️concepts/📝️.md:403, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:1094, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:1123, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:117, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:147, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:394, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:4, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:518, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:644, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:758, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:893, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:292, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:293, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:337, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:368, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:439, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:543, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1045, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1081, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:334, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:335, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:397, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:432, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:433, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:641, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:642, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:680, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:697, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:724

### d5. DIN V 18599-2 (2016-10 / 2018-09 / no part edition) (5)

❄️cooling/🎬️clip/README.md:48, ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:275, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:721, 📊️demand/🎬️clip/🔁️recap/🐍️.py:1275, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1241

### d6. DIN V 18599-1:2018-09 (7)

🌬️ventilation/🎬️clip/📦️volume/🐍️.py:474, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:475, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:488, 📊️demand/🎬️clip/♨️system-losses/🐍️.py:1004, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:417, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:955, 📊️demand/🎬️clip/🧪️practice/🐍️.py:1070

### d7. DIN V 18599 / DIN V 18599:2018-09 (no part) (19)

❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:267, ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:268, 🌬️ventilation/🎬️clip/README.md:67, 🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🔣️.json:29, 📊️demand/🎬️clip/README.md:60, 📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/README.md:98, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:542, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:543, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:575, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:959, 📖️concepts/📝️.md:24, 📖️concepts/📝️.md:39, 📖️concepts/📝️.md:40, 📖️concepts/📝️.md:417, 📖️concepts/📝️.md:41, 📖️concepts/📝️.md:42, 📖️concepts/📝️.md:43, 📖️concepts/📝️.md:79

### d8. DIN V 18599-5:2018-09 (7)

📊️demand/🎬️clip/♨️system-losses/🐍️.py:1073, 📊️demand/🎬️clip/♨️system-losses/🐍️.py:587, 📊️demand/🎬️clip/♨️system-losses/🐍️.py:775, 📖️concepts/📝️.md:404, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1316, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1317, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:160

### d9. DIN V 18599-10:2018-09 (internal heat sources) (9)

📊️demand/🎬️clip/🧪️practice/🐍️.py:314, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:203, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:368, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:439, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:543, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:645, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:646, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:744, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:87

### d10. DIN/TS 18599 parts 2025-10 (4)

❄️cooling/🎬️clip/README.md:48, 📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:631, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:422

### d11. DIN EN 12831-1 (no edition) (34)

❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:267, ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:268, 🌬️ventilation/🎬️clip/README.md:67, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:439, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:305, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:480, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:787, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:104, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:488, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:714, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:715, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:734, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:929, 🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:280, 🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:456, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:551, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:552, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:566, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:631, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:651, 📖️concepts/📝️.md:175, 📖️concepts/📝️.md:39, 📖️concepts/📝️.md:40, 📖️concepts/📝️.md:416, 📖️concepts/📝️.md:41, 📖️concepts/📝️.md:78, 🔥️heating/🎬️clip/🌬️convection/🐍️.py:82, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:292, 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:293, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:443, 🧲️physics/❓️quiz/🔣️.json:147, 🧲️physics/❓️quiz/🔣️.json:148, 🧲️physics/❓️quiz/🔣️.json:1557, 🧲️physics/❓️quiz/🔣️.json:1558

### d12. DIN EN 12831-1:2017-09 (7)

📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:646, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1089, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:632, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:893, 🔥️heating/🎬️clip/🌬️convection/🐍️.py:173, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1664

### d13. DIN/TS 12831-1:2020-04 (national annex) (2)

📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:632

### d14. DIN EN ISO 13789:2017-10 (2)

🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1045, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:156

### d15. DIN EN ISO 6946:2018-03 (and bare) (16)

📖️concepts/📝️.md:243, 📖️concepts/📝️.md:41, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:112, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:155, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:580, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:698, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:883, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1079, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1159, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1274, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1453, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1503, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:443, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:635, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:745, 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:922

### d16. DIN 4108 Beiblatt 2:2019-06 (and bare) (4)

📖️concepts/📝️.md:278, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1228, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1229, 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1241

### d17. DIN EN 16798-1:2022-03 (and bare) (6)

❄️cooling/🎬️clip/README.md:48, ❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:503, 🌬️ventilation/🎬️clip/README.md:68, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:107, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:929, 📖️concepts/📝️.md:40

### d18. DIN EN 16798-3 (no edition) (2)

❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:196, 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:282

### d19. DIN EN 16798-7:2017-11 (3)

🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1046, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1572, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:829

### d20. DIN EN 410 (no edition) (6)

📖️concepts/📝️.md:39, 📖️concepts/📝️.md:82, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:148, 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:644, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2138, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3191

### d21. DIN EN 14825 (no edition) (5)

📖️concepts/📝️.md:63, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1714, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1715, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1921, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3191

### d22. DIN EN ISO 13786 (no edition) (4)

📖️concepts/📝️.md:39, 📖️concepts/📝️.md:85, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2601, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3192

### d23. DIN EN 14511 (2)

📖️concepts/📝️.md:39, 📖️concepts/📝️.md:80

### d24. DIN EN 308 (1)

🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:456

### d25. VDI 4650 Blatt 1:2024-02 (and bare 'VDI 4650') (10)

📊️demand/🎬️clip/♨️system-losses/🐍️.py:775, 📊️demand/🎬️clip/♨️system-losses/🐍️.py:929, 📊️demand/🎬️clip/🧪️practice/🐍️.py:894, 📖️concepts/📝️.md:39, 📖️concepts/📝️.md:63, 📖️concepts/📝️.md:81, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1714, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1715, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1921, 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3191

### d26. VDI 2078:2015-06 (and bare) (30)

❄️cooling/❓️quiz/🔣️.json:183, ❄️cooling/❓️quiz/🔣️.json:184, ❄️cooling/❓️quiz/🔣️.json:207, ❄️cooling/❓️quiz/🔣️.json:208, ❄️cooling/❓️quiz/🔣️.json:27, ❄️cooling/❓️quiz/🔣️.json:28, ❄️cooling/🎬️clip/README.md:3, ❄️cooling/🎬️clip/README.md:48, ❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:203, ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:413, ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:203, ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:285, ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:586, ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:793, ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:911, ❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py:234, ❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py:385, ❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:231, ❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:404, 🌬️ventilation/🎬️clip/README.md:68, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:480, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:624, 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:787, 🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:280, 🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:456, 📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:554, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:555, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:647, 📖️concepts/📝️.md:43

### d27. VDI 4645:2023-04 (2)

📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1089

### d28. VDI 6022 Blatt 1 (no edition) (1)

❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:306

### d29. VDI 3807 Blatt 1 (no edition) (4)

📖️concepts/📝️.md:400, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:198, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:641, 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:642

### d30. VDI 2052 (2)

🌬️ventilation/❓️quiz/🔣️.json:967, 🌬️ventilation/❓️quiz/🔣️.json:968

### d31. VDI 2087 (1)

🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:945

### d32. DIN 1946-4:2018-09 (and bare) (9)

❄️cooling/❓️quiz/🔣️.json:230, ❄️cooling/❓️quiz/🔣️.json:231, 🌬️ventilation/❓️quiz/🔣️.json:951, 🌬️ventilation/❓️quiz/🔣️.json:952, 🌬️ventilation/🎬️clip/README.md:68, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:723, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:724, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:734, 📖️concepts/📝️.md:40

### d33. DIN 1946-6 (no edition) (15)

🌬️ventilation/❓️quiz/🔣️.json:795, 🌬️ventilation/❓️quiz/🔣️.json:796, 🌬️ventilation/🎬️clip/README.md:68, 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:738, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:107, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:248, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:104, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:354, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:599, 🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:268, 🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:269, 🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:271, 🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:272, 📖️concepts/📝️.md:39, 📖️concepts/📝️.md:40

### d34. DIN 1946-7 (2)

🌬️ventilation/❓️quiz/🔣️.json:931, 🌬️ventilation/❓️quiz/🔣️.json:932

### d35. DIN 18032-1 (2)

🌬️ventilation/❓️quiz/🔣️.json:816, 🌬️ventilation/❓️quiz/🔣️.json:817

### d36. DIN 18380:2019-09 (1)

📊️demand/🎬️clip/🧪️practice/🐍️.py:708

### d37. DIN 51603-1 (4)

🧲️physics/❓️quiz/🔣️.json:263, 🧲️physics/❓️quiz/🔣️.json:264, 🧲️physics/❓️quiz/🔣️.json:2723, 🧲️physics/❓️quiz/🔣️.json:2724

### d38. DIN V 4701-10:2003-08 (3)

📊️demand/🎬️clip/README.md:51, 📊️demand/🎬️clip/README.md:62, 📊️demand/🎬️clip/♨️system-losses/🐍️.py:1004

### d39. GEG / GModG (law) (85)

❄️cooling/❓️quiz/🔣️.json:101, ❄️cooling/❓️quiz/🔣️.json:102, ❄️cooling/❓️quiz/🔣️.json:117, ❄️cooling/❓️quiz/🔣️.json:118, ❄️cooling/❓️quiz/🔣️.json:124, ❄️cooling/❓️quiz/🔣️.json:125, ❄️cooling/❓️quiz/🔣️.json:167, ❄️cooling/❓️quiz/🔣️.json:168, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:474, 🌬️ventilation/🎬️clip/📦️volume/🐍️.py:475, 🏗️authoring/🔊️speech/🗣️lexicon/🔣️.json:158, 📊️demand/❓️quiz/🔣️.json:1054, 📊️demand/❓️quiz/🔣️.json:1055, 📊️demand/🎬️clip/README.md:52, 📊️demand/🎬️clip/README.md:55, 📊️demand/🎬️clip/README.md:56, 📊️demand/🎬️clip/README.md:60, 📊️demand/🎬️clip/README.md:98, 📊️demand/🎬️clip/README.md:9, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:1062, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:533, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:534, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:571, 📊️demand/🎬️clip/⚖️regulations/🐍️.py:588, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:110, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:1188, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:422, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:426, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:548, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:627, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:79, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:955, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:959, 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py:97, 📊️demand/🎬️clip/🏭️primary-energy/🔊️speech/🐍️.py:1, 📊️demand/🎬️clip/🏷️certificate/🐍️.py:166, 📊️demand/🎬️clip/🏷️certificate/🐍️.py:342, 📊️demand/🎬️clip/🏷️certificate/🐍️.py:545, 📊️demand/🎬️clip/🏷️certificate/🐍️.py:786, 📊️demand/🎬️clip/🧪️practice/🐍️.py:1070, 📊️demand/🎬️clip/🧪️practice/🐍️.py:121, 📊️demand/🎬️clip/🧪️practice/🐍️.py:708, 📊️demand/🎬️clip/🧪️tests/🔗️chain/🐍️.py:1, 🔥️heating/❓️quiz/🔣️.json:1513, 🔥️heating/❓️quiz/🔣️.json:1514, 🔥️heating/❓️quiz/🔣️.json:1536, 🔥️heating/❓️quiz/🔣️.json:1537, 🔥️heating/❓️quiz/🔣️.json:1543, 🔥️heating/❓️quiz/🔣️.json:1544, 🔥️heating/❓️quiz/🔣️.json:1547, 🔥️heating/❓️quiz/🔣️.json:1548, 🔥️heating/❓️quiz/🔣️.json:1559, 🔥️heating/❓️quiz/🔣️.json:1560, 🔥️heating/❓️quiz/🔣️.json:165, 🔥️heating/❓️quiz/🔣️.json:166, 🔥️heating/❓️quiz/🔣️.json:169, 🔥️heating/❓️quiz/🔣️.json:170, 🔥️heating/❓️quiz/🔣️.json:181, 🔥️heating/❓️quiz/🔣️.json:182, 🔥️heating/❓️quiz/🔣️.json:210, 🔥️heating/❓️quiz/🔣️.json:211, 🔥️heating/❓️quiz/🔣️.json:214, 🔥️heating/❓️quiz/🔣️.json:215, 🔥️heating/❓️quiz/🔣️.json:225, 🔥️heating/❓️quiz/🔣️.json:226, 🔥️heating/❓️quiz/🔣️.json:320, 🔥️heating/❓️quiz/🔣️.json:321, 🔥️heating/❓️quiz/🔣️.json:324, 🔥️heating/❓️quiz/🔣️.json:325, 🔥️heating/❓️quiz/🔣️.json:335, 🔥️heating/❓️quiz/🔣️.json:336, 🔥️heating/❓️quiz/🔣️.json:342, 🔥️heating/❓️quiz/🔣️.json:343, 🔥️heating/❓️quiz/🔣️.json:346, 🔥️heating/❓️quiz/🔣️.json:347, 🔥️heating/❓️quiz/🔣️.json:357, 🔥️heating/❓️quiz/🔣️.json:358, 🔥️heating/❓️quiz/🔣️.json:364, 🔥️heating/❓️quiz/🔣️.json:365, 🔥️heating/❓️quiz/🔣️.json:368, 🔥️heating/❓️quiz/🔣️.json:369, 🔥️heating/❓️quiz/🔣️.json:379, 🔥️heating/❓️quiz/🔣️.json:380, 🧲️physics/❓️quiz/🔣️.json:359, 🧲️physics/❓️quiz/🔣️.json:360

### d40. Bohne, Fachbuch Gebäudetechnik, 13. Aufl. 2026, ISBN 978-3-658-51570-6 (1)

🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3188

### d41. Willems / Schild / Stricker, 8. Aufl. 2024, ISBN 978-3-658-44973-5 (1)

🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3185

### d42. Maas 1995 (4)

🌬️ventilation/🎬️clip/README.md:111, 🌬️ventilation/🎬️clip/🧪️architecture/🔣️.numbers.json:3, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:600, 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:601

### d43. Pettenkofer 1858 (Ueber den Luftwechsel in Wohngebäuden) (3)

🌬️ventilation/🎬️clip/README.md:69, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:210, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:248

### d44. Bundesgesundheitsbl. 51 (2008) 1358-1369 (Ad-hoc-AG Innenraumrichtwerte) (3)

🌬️ventilation/🎬️clip/README.md:70, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:211, 🌬️ventilation/🎬️clip/❓️need/🐍️.py:248

### d45. IWU Deutsche Wohngebäudetypologie 2015 (62)

📊️demand/❓️quiz/🔣️.json:1008, 📊️demand/❓️quiz/🔣️.json:1009, 📊️demand/❓️quiz/🔣️.json:1031, 📊️demand/❓️quiz/🔣️.json:1032, 📊️demand/❓️quiz/🔣️.json:1054, 📊️demand/❓️quiz/🔣️.json:1055, 📊️demand/❓️quiz/🔣️.json:205, 📊️demand/❓️quiz/🔣️.json:206, 📊️demand/❓️quiz/🔣️.json:225, 📊️demand/❓️quiz/🔣️.json:226, 📊️demand/❓️quiz/🔣️.json:257, 📊️demand/❓️quiz/🔣️.json:258, 📊️demand/❓️quiz/🔣️.json:305, 📊️demand/❓️quiz/🔣️.json:306, 📊️demand/❓️quiz/🔣️.json:325, 📊️demand/❓️quiz/🔣️.json:326, 📊️demand/❓️quiz/🔣️.json:345, 📊️demand/❓️quiz/🔣️.json:346, 📊️demand/❓️quiz/🔣️.json:365, 📊️demand/❓️quiz/🔣️.json:366, 📊️demand/❓️quiz/🔣️.json:385, 📊️demand/❓️quiz/🔣️.json:386, 📊️demand/❓️quiz/🔣️.json:405, 📊️demand/❓️quiz/🔣️.json:406, 📊️demand/❓️quiz/🔣️.json:425, 📊️demand/❓️quiz/🔣️.json:426, 📊️demand/❓️quiz/🔣️.json:445, 📊️demand/❓️quiz/🔣️.json:446, 📊️demand/❓️quiz/🔣️.json:545, 📊️demand/❓️quiz/🔣️.json:546, 📊️demand/❓️quiz/🔣️.json:565, 📊️demand/❓️quiz/🔣️.json:566, 📊️demand/❓️quiz/🔣️.json:585, 📊️demand/❓️quiz/🔣️.json:586, 📊️demand/❓️quiz/🔣️.json:605, 📊️demand/❓️quiz/🔣️.json:606, 📊️demand/❓️quiz/🔣️.json:898, 📊️demand/❓️quiz/🔣️.json:899, 🔥️heating/❓️quiz/🔣️.json:1490, 🔥️heating/❓️quiz/🔣️.json:1491, 🔥️heating/❓️quiz/🔣️.json:1513, 🔥️heating/❓️quiz/🔣️.json:1514, 🔥️heating/❓️quiz/🔣️.json:1536, 🔥️heating/❓️quiz/🔣️.json:1537, 🔥️heating/❓️quiz/🔣️.json:1582, 🔥️heating/❓️quiz/🔣️.json:1583, 🔥️heating/❓️quiz/🔣️.json:1606, 🔥️heating/❓️quiz/🔣️.json:1607, 🔥️heating/❓️quiz/🔣️.json:1629, 🔥️heating/❓️quiz/🔣️.json:1630, 🔥️heating/❓️quiz/🔣️.json:1653, 🔥️heating/❓️quiz/🔣️.json:1654, 🔥️heating/❓️quiz/🔣️.json:1676, 🔥️heating/❓️quiz/🔣️.json:1677, 🔥️heating/❓️quiz/🔣️.json:1700, 🔥️heating/❓️quiz/🔣️.json:1701, 🧲️physics/❓️quiz/🔣️.json:1557, 🧲️physics/❓️quiz/🔣️.json:1558, 🧲️physics/❓️quiz/🔣️.json:2805, 🧲️physics/❓️quiz/🔣️.json:2806, 🧲️physics/❓️quiz/🔣️.json:283, 🧲️physics/❓️quiz/🔣️.json:284

### d46. IEST-RP-CC012 (4)

🌬️ventilation/❓️quiz/🔣️.json:1003, 🌬️ventilation/❓️quiz/🔣️.json:1004, 🌬️ventilation/❓️quiz/🔣️.json:987, 🌬️ventilation/❓️quiz/🔣️.json:988

### d47. ISO 14644-4 (2)

🌬️ventilation/❓️quiz/🔣️.json:1023, 🌬️ventilation/❓️quiz/🔣️.json:1024

## (e) Notes

- The reports' claim that DIN 4108-2 'Tab. 7 (F_C)' is verified rests on the 2013-02 edition; the notation F_C = g_tot/g is confirmed as a definition (Baunetzwissen 'Abminderungsfaktor', ift Rosenheim EnEV note), but Table 7 of the 2026-05 edition was not read.
- VDI 2078 'revision announced for 2027' (cooling README) is not supported by any source found; remove it or verify.
- Pettenkofer / DIN 1946-6 link on the ventilation corner label: the 1000 ppm limit is from the UBA/Ad-hoc-AG article; the DIN 1946-6 attribution on the same label is not verified.
- Not checked in this audit: IWU 2015 typology, IEST-RP-CC012, ISO 14644-4, DIN 1946-7, DIN 18032-1, DIN 18380, DIN 51603-1, DIN EN 308, DIN EN 14511, VDI 2052, VDI 2087 (marked UNVERIFIED).
- Outside the table: ISO 13790 and EN ISO 52016-1 (named in heating notes, not on screen) were not checked.

