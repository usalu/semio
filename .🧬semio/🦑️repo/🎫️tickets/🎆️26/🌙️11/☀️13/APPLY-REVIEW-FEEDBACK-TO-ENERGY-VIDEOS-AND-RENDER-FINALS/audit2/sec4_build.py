# 📚 Builds sec4-references.md from the grep hits (audit section 4); read-only on the energy tree
import os
import re
import collections

HERE = os.path.dirname(os.path.abspath(__file__))
HITS = os.path.join(HERE, "all-hits.txt")
OUT = os.path.join(HERE, "sec4-references.md")

ROWS = [
    dict(key="1", label="DIN 4108-2:2013-02", rx=r"DIN 4108-2:2013-02",
         ed="Y (2013-02)", loc="N", report="cooling: WebSearch 'Abschn. 8, Tab. 7' of 2013-02 (claimed verified)",
         web="2013-02 superseded: DIN lists DIN 4108-2:2026-05 as current/published (2026-05 TOC preview shows section 8)",
         url="https://www.dinmedia.de/en/standard/din-4108-2/401017482", verdict="WRONG",
         repl="DIN 4108-2:2026-05, Abschn. 8 (Sommerlicher Wärmeschutz)"),
    dict(key="2", label="DIN 4108-2 (no edition)", rx=r"DIN 4108-2(?![:\d])",
         ed="N", loc="N", report="demand, cooling, heating (claimed, no edition)",
         web="current edition DIN 4108-2:2026-05", url="https://www.dinmedia.de/en/standard/din-4108-2/401017482",
         verdict="WRONG", repl="DIN 4108-2:2026-05"),
    dict(key="3", label="DIN 4108-2:2026-05", rx=r"DIN 4108-2:2026-05",
         ed="Y", loc="partial (Abschn. 8 in TOC preview; Tab. 7 not verified)", report="demand: 'final vs pre-published unconfirmed' (not verified by demand)",
         web="edition confirmed as published/current; Tab. 7 F_C values not read", url="https://www.dinmedia.de/en/standard/din-4108-2/401017482",
         verdict="EDITION-ONLY", repl="DIN 4108-2:2026-05, Abschn. 8"),
    dict(key="4", label="DIN V 4108-6:2003-06 (and bare 'DIN V 4108-6')", rx=r"DIN V 4108-6",
         ed="Y (2003-06) / N", loc="claimed 3.2, 5.5, 6.1-6.5, Anh. A, B.4, D.2, D.5 (not checkable: standard withdrawn)",
         report="heat-b (TOC only), demand ('Verified locators' 3.2, 5.5, 6.1-6.5, Anh. A, D.2, D.5)",
         web="WITHDRAWN (DIN listing); DIN recommends DIN V 18599 parts and DIN/TS 18599-12:2021-04; a corrigendum is also withdrawn; DIN 4108-6:2024 not found",
         url="https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447", verdict="WRONG",
         repl="DIN/TS 18599-12:2021-04 (successor per DIN; clause numbers must be re-checked, do not keep 3.2/5.5/6.x)"),
    dict(key="5", label="DIN V 18599-2 (2016-10 / 2018-09 / no part edition)", rx=r"DIN V 18599-2",
         ed="Y (both 2016-10 and 2018-09 used)", loc="claimed 5.5.3 (demand, 2018-09)",
         report="demand (2018-09 5.5.3 'verified'), cooling (2016-10 'Interne Wärmequellen')",
         web="DIN listing marks DIN V 18599-2 as WITHDRAWN; successor for part 2 not found (2025-10 TS parts exist for -4 and -10)",
         url="no DIN URL retrieved for part 2", verdict="WRONG",
         repl="no verified replacement; cite the DIN/TS 18599-2 part of the 2025-10 series only after a DIN check"),
    dict(key="6", label="DIN V 18599-1:2018-09", rx=r"DIN V 18599-1(?![0-9])",
         ed="Y", loc="N", report="none (not claimed by reports)",
         web="not checked individually; DIN/TS 18599-1:2025-10 is named in demand files",
         url="", verdict="UNVERIFIED", repl="check status at DIN; if superseded use DIN/TS 18599-1:2025-10"),
    dict(key="7", label="DIN V 18599 / DIN V 18599:2018-09 (no part)", rx=r"DIN V 18599(?![-0-9])",
         ed="mixed (bare or 2018-09)", loc="N", report="demand (energy balance text)",
         web="series superseded in part by DIN/TS 18599 2025-10 (confirmed for parts -4, -10)",
         url="", verdict="UNVERIFIED", repl="name the part and edition explicitly, e.g. DIN V 18599-1:2018-09 or its 2025-10 successor"),
    dict(key="8", label="DIN V 18599-5:2018-09", rx=r"DIN V 18599-5",
         ed="Y", loc="N", report="demand, heating (not claimed verified)",
         web="not verified; cited as a recommended successor of withdrawn DIN V 4701-10",
         url="https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135", verdict="UNVERIFIED",
         repl="verify status at DIN before use"),
    dict(key="9", label="DIN V 18599-10:2018-09 (internal heat sources)", rx=r"DIN V 18599-10",
         ed="Y (2018-09)", loc="N", report="heating/demand (not claimed verified)",
         web="replaced by DIN/TS 18599-10:2025-10 (DIN listing of both editions)",
         url="https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970", verdict="WRONG",
         repl="DIN/TS 18599-10:2025-10 (Vornorm, replaces DIN V 18599-10:2018-09)"),
    dict(key="10", label="DIN/TS 18599 parts 2025-10", rx=r"DIN/TS 18599",
         ed="Y (2025-10)", loc="N", report="demand (2025-10 notes)",
         web="DIN/TS 18599-4:2025-10 and -10:2025-10 confirmed; -1 and -2 mapping not confirmed",
         url="https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970", verdict="UNVERIFIED",
         repl="keep only parts confirmed at DIN"),
    dict(key="11", label="DIN EN 12831-1 (no edition)", rx=r"DIN EN 12831(?!-1:2017-09)(?![0-9])",
         ed="N", loc="claimed Abschn. 6 (demand)", report="demand: 'Verified locators: DIN EN 12831-1:2017-09 Abschn. 6' (only with edition)",
         web="edition DIN EN 12831-1:2017-09 current; national annex DIN/TS 12831-1:2020-04 (secondary source)",
         url="https://www.dinmedia.de/de/norm/din-en-12831-1/261292587", verdict="WRONG",
         repl="DIN EN 12831-1:2017-09"),
    dict(key="12", label="DIN EN 12831-1:2017-09", rx=r"DIN EN 12831-1:2017-09",
         ed="Y", loc="claimed Abschn. 6 (not verified)", report="demand (Abschn. 6 claimed verified)",
         web="edition confirmed (DIN); Abschn. 6 not read", url="https://www.dinmedia.de/de/norm/din-en-12831-1/261292587",
         verdict="EDITION-ONLY", repl="DIN EN 12831-1:2017-09"),
    dict(key="13", label="DIN/TS 12831-1:2020-04 (national annex)", rx=r"DIN/TS 12831",
         ed="Y (2020-04)", loc="N", report="demand (named)",
         web="existence via Haustechnikdialog snippet; DIN page not opened",
         url="https://www.haustechnikdialog.de/SHKwissen/3109/DIN-TS-12831-1", verdict="EDITION-ONLY",
         repl="DIN/TS 12831-1:2020-04 (national annex to DIN EN 12831-1:2017-09)"),
    dict(key="14", label="DIN EN ISO 13789:2017-10", rx=r"DIN EN ISO 13789",
         ed="Y (2017-10 claimed)", loc="N", report="conduction file (not verified by heat-b)",
         web="no 2017-10 edition found; DIN lists DIN EN ISO 13789:2018-04 (German version of ISO 13789:2017, replaces 2008-04)",
         url="https://www.dinmedia.de/en/standard/en-iso-13789/277780736", verdict="WRONG",
         repl="DIN EN ISO 13789:2018-04"),
    dict(key="15", label="DIN EN ISO 6946:2018-03 (and bare)", rx=r"DIN EN ISO 6946",
         ed="Y (2018-03)", loc="claimed Tab. 7 (secondary sources only; not verified)", report="heat-a (edition and Tab. 7 'confirmed via secondary sources only')",
         web="2018-03 current, Berichtigung 1:2023-04; Tab. 7 not verified",
         url="https://www.dinmedia.de/de/norm/din-en-iso-6946/255315365", verdict="EDITION-ONLY",
         repl="DIN EN ISO 6946:2018-03 (drop 'Tab. 7' unless checked)"),
    dict(key="16", label="DIN 4108 Beiblatt 2:2019-06 (and bare)", rx=r"DIN 4108 (?:Beiblatt 2|Bbl\. 2|supplement 2)",
         ed="Y (2019-06)", loc="N", report="heat-a (edition 'confirmed via secondary sources')",
         web="2019-06 current; Berichtigung 1:2025-02 exists (CD-ROM replacement)",
         url="https://www.dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077", verdict="EDITION-ONLY",
         repl="DIN 4108 Beiblatt 2:2019-06 (Berichtigung 1:2025-02 beachten)"),
    dict(key="17", label="DIN EN 16798-1:2022-03 (and bare)", rx=r"DIN EN 16798-1",
         ed="Y (2022-03)", loc="claimed Anh. B.2 / Tab. B.2 (cooling); Tab. B.9 not cited (vent-a)", report="cooling (Anh. B.2 'verified')",
         web="2022-03 current (replaces 2021-04); Anhang B is informative criteria; table numbers not read",
         url="https://www.dinmedia.de/de/norm/din-en-16798-1/349622591", verdict="EDITION-ONLY",
         repl="DIN EN 16798-1:2022-03, Anh. B (drop 'Tab. B.2' unless checked)"),
    dict(key="18", label="DIN EN 16798-3 (no edition)", rx=r"DIN EN 16798-3",
         ed="N", loc="N", report="cooling, ventilation (no edition)",
         web="DIN EN 16798-3:2017-11 withdrawn; successor DIN EN 16798-3:2026-10 per DIN timeline (some old pages still show 2017-11 as current)",
         url="https://www.dinmedia.de/en/standard/din-en-16798-3/390958579", verdict="WRONG",
         repl="DIN EN 16798-3:2026-10 (check clause numbers)"),
    dict(key="19", label="DIN EN 16798-7:2017-11", rx=r"DIN EN 16798-7",
         ed="Y (2017-11)", loc="N (procedure 1/2 exists; formulas not read)", report="ventilation natural (2017-11 'verified'; C_d = 0,6 not from a clause)",
         web="current; replaces DIN EN 15242:2007-09",
         url="https://www.dinmedia.de/en/standard/din-en-16798-7/258797864", verdict="EDITION-ONLY",
         repl="DIN EN 16798-7:2017-11 (C_d = 0,6 is an assumption, cite as such)"),
    dict(key="20", label="DIN EN 410 (no edition)", rx=r"DIN EN 410(?![:0-9])",
         ed="N", loc="N", report="physics, concepts", web="current edition DIN EN 410:2011-04",
         url="https://www.dinmedia.de/de/standard/din-en-410/138424295", verdict="WRONG", repl="DIN EN 410:2011-04"),
    dict(key="21", label="DIN EN 14825 (no edition)", rx=r"DIN EN 14825(?![:\d])",
         ed="N", loc="N (SCOP keyword)", report="physics, concepts",
         web="current DIN EN 14825:2023-10 (German version of EN 14825:2022); A1 draft only",
         url="https://www.dinmedia.de/de/norm/din-en-14825/343036621", verdict="WRONG", repl="DIN EN 14825:2023-10 (SCOP)"),
    dict(key="22", label="DIN EN ISO 13786 (no edition)", rx=r"DIN EN ISO 13786(?![:\d])",
         ed="N", loc="N", report="physics", web="current DIN EN ISO 13786:2018-04 (replaces 2008-04)",
         url="https://www.dinmedia.de/de/norm/din-en-iso-13786/255912584", verdict="WRONG", repl="DIN EN ISO 13786:2018-04"),
    dict(key="23", label="DIN EN 14511", rx=r"DIN EN 14511", ed="N", loc="N", report="concepts (not verified)",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="24", label="DIN EN 308", rx=r"DIN EN 308", ed="N", loc="N", report="ventilation mechanical (not verified)",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="25", label="VDI 4650 Blatt 1:2024-02 (and bare 'VDI 4650')", rx=r"VDI 4650",
         ed="Y (2024-02) / N (bare)", loc="Y (JAZ term in title)", report="physics (2024-02 verified, Jahresarbeitszahl)",
         web="VDI 4650 Blatt 1:2024-02 is the current edition; title 'Berechnung der Jahresarbeitszahl von Wärmepumpenanlagen'",
         url="https://vdi.de/en/home/vdi-standards/details/vdi-4650-blatt-1-calculation-of-the-seasonal-coefficient-of-performance-of-heat-pumps-electric-heat-pumps-for-space-heating-and-domestic-hot-water",
         verdict="VERIFIED", repl="VDI 4650 Blatt 1:2024-02 (unify bare variants)"),
    dict(key="26", label="VDI 2078:2015-06 (and bare)", rx=r"VDI ?2078",
         ed="Y (2015-06) / N (bare)", loc="partial (keywords; 'Kühllast' in title; 'innere Wärmequellen' not verified)",
         report="cooling/heating (2015-06 'verified', keyword locators only)",
         web="2015-06 current; no successor or 2027 revision found (the README claim of a 2027 revision is UNVERIFIED)",
         url="https://vdi.de/en/home/vdi-standards/details/vdi-2078-calculation-of-thermal-loads-and-room-temperatures-design-cooling-load-and-annual-simulation",
         verdict="EDITION-ONLY", repl="VDI 2078:2015-06 (remove 'revision announced for 2027' unless verified)"),
    dict(key="27", label="VDI 4645:2023-04", rx=r"VDI 4645", ed="Y (2023-04)", loc="N",
         report="demand (2023-04 'verified')",
         web="2023-04 issued; a draft VDI 4645:2026-03 was seen; current status not confirmed",
         url="https://www.dinmedia.de/de/technische-regel-entwurf/vdi-4645/399378368", verdict="UNVERIFIED",
         repl="check status on vdi.de before citing"),
    dict(key="28", label="VDI 6022 Blatt 1 (no edition)", rx=r"VDI 6022", ed="N", loc="N",
         report="cooling (no edition)", web="current VDI 6022 Blatt 1:2018-01 (Berichtigung 2023-04)",
         url="https://vdi.de/richtlinien/details/vdi-6022-blatt-1-ventilation-and-indoor-air-quality-hygiene-requirements-for-ventilation-and-air-conditioning-systems-and-units-vdi-ventilation-code-of-practice-1",
         verdict="WRONG", repl="VDI 6022 Blatt 1:2018-01"),
    dict(key="29", label="VDI 3807 Blatt 1 (no edition)", rx=r"VDI 3807", ed="N", loc="N (G20/15 not verified)",
         report="heating (G20/15 'secondary sources only', VDI 3807 not read)",
         web="current VDI 3807 Blatt 1:2013-06 (replaces 2007-03)",
         url="https://vdi.de/en/home/vdi-standards/details/vdi-3807-blatt-1-verbrauchskennwerte-fuer-gebaeude-grundlagen",
         verdict="WRONG", repl="VDI 3807 Blatt 1:2013-06 (G20/15 locator still to be checked)"),
    dict(key="30", label="VDI 2052", rx=r"VDI 2052", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="31", label="VDI 2087", rx=r"VDI 2087", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="32", label="DIN 1946-4:2018-09 (and bare)", rx=r"DIN 1946-4",
         ed="Y (2018-09) / N (bare)", loc="N (values 60 m³/(h·m²), 20 1/h not verified)",
         report="cooling (2018-09 'verified')",
         web="2018-09 current (title 'Raumlufttechnische Anlagen in Gebäuden und Räumen des Gesundheitswesens'); amendment A1:2025-11 exists",
         url="https://6sku3uaup9f9.din.de/en/standard/din-1946-4/294438653", verdict="EDITION-ONLY",
         repl="DIN 1946-4:2018-09 (check A1:2025-11)"),
    dict(key="33", label="DIN 1946-6 (no edition)", rx=r"DIN 1946-6", ed="N", loc="N",
         report="ventilation (many, no edition); vent-a claims Pettenkofer link",
         web="edition not confirmed; 2009-05 was the prior edition, a 2017 draft was announced; 2019-12 claim not confirmed",
         url="", verdict="UNVERIFIED", repl="confirm current edition at Beuth before adding it (e.g. DIN 1946-6:YYYY-MM)"),
    dict(key="34", label="DIN 1946-7", rx=r"DIN 1946-7", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="35", label="DIN 18032-1", rx=r"DIN 18032-1", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="36", label="DIN 18380:2019-09", rx=r"DIN 18380", ed="Y (2019-09)", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition"),
    dict(key="37", label="DIN 51603-1", rx=r"DIN 51603", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="38", label="DIN V 4701-10:2003-08", rx=r"DIN V 4701-10", ed="Y (2003-08)", loc="N",
         report="demand (heat-related text, not verified)",
         web="WITHDRAWN (DIN listing); DIN recommends DIN V 18599-5/-6/-8:2018-09 (not a formal replacement)",
         url="https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135", verdict="WRONG",
         repl="drop, or cite DIN V 18599-5:2018-09 / -6 / -8 as the recommended successors"),
    dict(key="39", label="GEG / GModG (law)", rx=r"\bGEG\b|GModG|Gebäudemodernisierungsgesetz",
         ed="Y (GEG 2024 used)", loc="N (paragraph numbers not verified)",
         report="demand (GEG §15/§20 'verified' on gesetze-im-internet.de; the GModG rename was flagged for checking)",
         web="GModG promulgated 28.07.2026 (BGBl. I Nr. 226) and in force 29.07.2026, renaming the GEG (see section c)",
         url="https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284",
         verdict="WRONG", repl="Gebäudemodernisierungsgesetz (GModG, vormals GEG) - paragraph numbers to be checked against the new text"),
    dict(key="40", label="Bohne, Fachbuch Gebäudetechnik, 13. Aufl. 2026, ISBN 978-3-658-51570-6", rx=r"Bohne",
         ed="Y (13. Aufl. 2026)", loc="N", report="physics (13th ed. 2026, ISBN 978-3-658-51570-6 'verified' via baunetzwissen)",
         web="ISBN 978-3-658-51570-6 belongs to Dirk Bohne / Marius Bohne, Springer Vieweg 2026, 13th ed.; the on-screen title must match this book's title",
         url="https://www.baunetzwissen.de/gebaeudetechnik/tipps/buecher/technischer-ausbau-von-gebaeuden-4347729",
         verdict="EDITION-ONLY", repl="Bohne, D.; Bohne, M.: Fachbuch Gebäudetechnik, 13. Aufl., Springer Vieweg 2026, ISBN 978-3-658-51570-6"),
    dict(key="41", label="Willems / Schild / Stricker, 8. Aufl. 2024, ISBN 978-3-658-44973-5", rx=r"Willems|Schild|Stricker",
         ed="Y (8. Aufl. 2024 claimed)", loc="N", report="physics (ISBN 978-3-658-44973-5 'verified')",
         web="8. Aufl. 2024 of 'Praxisbeispiele Bauphysik' exists with ISBN 978-3-658-44971-1 (9783658449711); the printed ISBN 978-3-658-44973-5 was not confirmed",
         url="https://www.jpc.de/jpcng/books/detail/-/art/wolfgang-m-willems-praxisbeispiele-bauphysik/hnum/11834057",
         verdict="WRONG", repl="Willems, W. M.; Schild, K.; Stricker, D.: Praxisbeispiele Bauphysik, 8. Aufl., Springer 2024, ISBN 978-3-658-44971-1 (ISBN to confirm at Springer)"),
    dict(key="42", label="Maas 1995", rx=r"Maas", ed="Y (1995, Diss. Univ. Gesamthochschule Kassel)",
         loc="Y (Kap. 8.2 'Luftwechsel', S. 74; values 0,56-1,38 1/h at 58,5 m³ room)", report="vent-b (ch. 8, 0,56-1,38 1/h, pages ~74-75 estimated)",
         web="dissertation 'Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung', eingereicht 27.06.1995; Kap. 8 'Luftwechsel und Lüftungswärmeverluste in der Heizzeit' S. 73-81; S. 74 gives 0,56 bis 1,38 1/h for a permanently tilted window (Heizgrenztemperatur 15 °C)",
         url="https://www.uni-kassel.de/fb6/bpy/de/forschung/abgeschlprojekte/pdfs/maas_diss.pdf", verdict="VERIFIED",
         repl="Maas, A. (1995): Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung, Diss. Univ. Gesamthochschule Kassel, Kap. 8.2, S. 74"),
    dict(key="43", label="Pettenkofer 1858 (Ueber den Luftwechsel in Wohngebäuden)", rx=r"Ueber den Luftwechsel|Pettenkofer 1858",
         ed="Y (1858)", loc="N", report="vent-a (1858 cited; no locator)",
         web="title page: M. v. Pettenkofer, Ueber den Luftwechsel in Wohngebäuden, München, Cotta, 1858 (one bibliography says Brunswick; title page says Munich)",
         url="https://lessing.ub.uni-duesseldorf.de/download/pdf/2385693.pdf", verdict="EDITION-ONLY",
         repl="M. v. Pettenkofer: Ueber den Luftwechsel in Wohngebäuden, München: Cotta, 1858"),
    dict(key="44", label="Bundesgesundheitsbl. 51 (2008) 1358-1369 (Ad-hoc-AG Innenraumrichtwerte)", rx=r"Bundesgesundheitsbl|1358-1369",
         ed="Y (2008, vol. 51)", loc="N (article title confirmed; 1000 ppm value in the text not read)",
         report="vent-a (confirmed by web search)",
         web="'Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft', DOI 10.1007/s00103-008-0707-2, published 29.11.2008; pages 1358-1369",
         url="https://link.springer.com/content/pdf/10.1007/s00103-008-0707-2.pdf", verdict="EDITION-ONLY",
         repl="Ad-hoc-AG Innenraumrichtwerte: Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft, Bundesgesundheitsbl. 51 (2008) 1358-1369, DOI 10.1007/s00103-008-0707-2"),
    dict(key="45", label="IWU Deutsche Wohngebäudetypologie 2015", rx=r"IWU|Wohngebäudetypologie", ed="Y (2015 claimed)", loc="N",
         report="demand (no verification recorded)", web="not checked in this audit", url="", verdict="UNVERIFIED",
         repl="check the edition and cite the table/typology code"),
    dict(key="46", label="IEST-RP-CC012", rx=r"IEST-RP", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
    dict(key="47", label="ISO 14644-4", rx=r"ISO 14644", ed="N", loc="N", report="none",
         web="not checked", url="", verdict="UNVERIFIED", repl="check edition or drop"),
]


def esc(s):
    return str(s).replace("|", "/").replace("\n", " ")


def short(path):
    return path.split("energy/", 1)[-1] if "energy/" in path else path


def load_hits():
    out = []
    with open(HITS, encoding="utf-8") as fh:
        for raw in fh:
            raw = raw.rstrip("\n")
            if not raw or raw.startswith("TOTAL"):
                continue
            parts = raw.split(":", 2)
            if len(parts) < 3 or not parts[1].isdigit():
                continue
            out.append((short(parts[0]), int(parts[1]), parts[2]))
    return out


def main():
    hits = load_hits()
    seen = set()
    uniq = []
    for h in hits:
        if (h[0], h[1]) not in seen:
            seen.add((h[0], h[1]))
            uniq.append(h)
    per = collections.OrderedDict()
    for r in ROWS:
        rx = re.compile(r["rx"])
        per[r["key"]] = [(p, n) for (p, n, t) in uniq if rx.search(t)]
    counts = collections.Counter(r["verdict"] for r in ROWS)

    L = []
    L.append("# Section 4 reference audit (APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS)")
    L.append("")
    L.append("Read-only audit. Date of checks: 2026-11-13. Paths are relative to `🎓️teaching/🏛️architecture/⚡️energy/` unless stated.")
    L.append("Occurrence source: grep of the energy tree (`audit2/g.py`, pattern files `p-refs.txt` and `p-extra.txt`) into `audit2/all-hits.txt`.")
    L.append("Counts are per reference row; a line that cites two references appears in both rows.")
    L.append("")
    L.append("## (a) Reference table")
    L.append("")
    L.append(f"Distinct references: {len(ROWS)}. Verdicts: VERIFIED {counts['VERIFIED']}, EDITION-ONLY {counts['EDITION-ONLY']}, UNVERIFIED {counts['UNVERIFIED']}, WRONG {counts['WRONG']}.")
    L.append("")
    L.append("| # | Displayed string | Occurrences (n, full list in d) | Edition/year | Locator | Report claims verified | Web result | Source URL | Verdict | Proposed replacement |")
    L.append("|---|---|---|---|---|---|---|---|---|---|")
    for r in ROWS:
        L.append("| {} | {} | {} (d{}) | {} | {} | {} | {} | {} | **{}** | {} |".format(
            r["key"], esc(r["label"]), len(per[r["key"]]), r["key"], esc(r["ed"]), esc(r["loc"]),
            esc(r["report"]), esc(r["web"]), esc(r["url"]) or "-", r["verdict"], esc(r["repl"])))
    L.append("")
    L.append("## (b) Conflicting editions across files")
    L.append("")
    L.append("- DIN 4108-2: 2013-02 in cooling (quiz, README, solar, heating-comparison, summer-strategy), heating conduction and ventilation natural; 2026-05 in demand (README, regulations). The 2013-02 edition is superseded.")
    L.append("- DIN V 18599-2: 2016-10 (cooling heating-comparison, README) vs 2018-09 (demand regulations, recap). Both are withdrawn per DIN; part 2 successor not confirmed.")
    L.append("- DIN V 18599-10: 2018-09 in heating/demand; DIN/TS 18599-10:2025-10 replaces it (confirmed).")
    L.append("- DIN EN 12831-1: bare (about 20 files) vs DIN EN 12831-1:2017-09 (demand) vs DIN/TS 12831-1:2020-04 (national annex, named in demand).")
    L.append("- DIN EN ISO 13789: 2017-10 in conduction vs 2018-04 current.")
    L.append("- DIN EN 16798-3: bare (cooling, ventilation) vs withdrawn 2017-11 vs 2026-10 successor.")
    L.append("- DIN 4108 Beiblatt 2: bare in heating vs 2019-06 in concepts/conduction.")
    L.append("- VDI 4650 Blatt 1: bare 'VDI 4650' and '-1:2024-02' variants vs 'Blatt 1:2024-02'.")
    L.append("- DIN V 4108-6:2003-06 (withdrawn) vs bare in demand/heating; the 'F_C' / 'Annex B.4' citation depends on it.")
    L.append("- GEG 2024 (cooling quiz, demand README) vs GModG 2026 (demand chips, GEG renamed on 29.07.2026): name in use is mixed.")
    L.append("- Willems: printed ISBN 978-3-658-44973-5 vs found 978-3-658-44971-1 (same 8th ed. 2024 'Praxisbeispiele Bauphysik').")
    L.append("- Bohne: ISBN 978-3-658-51570-6 is verified for the 13th ed. 2026 (Dirk and Marius Bohne, Springer Vieweg); the card title must match.")
    L.append("")
    L.append("## (c) GEG / GModG status")
    L.append("")
    L.append("- Verified: the Gebäudemodernisierungsgesetz (GModG) was promulgated in the BGBl. on 28.07.2026 (BGBl. I Nr. 226) and the Gebäudeenergiegesetz is renamed effective 29.07.2026. Sources: https://cci-dialog.de/wp-content/uploads/2026/08/Bundesgesetz-zur-Ueberfuehrung-des-GEG-ins-GModG.pdf ; https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284 ; https://www.baunetzwissen.de/drucken/gebaeudemodernisierungsgesetz-gmodg-3559433")
    L.append("- Conflicting vote dates in secondary sources (Bundestag 9, 10 or 14 July 2026); the promulgation date 28.07.2026 is consistent across sources. Do not cite the vote dates.")
    L.append("- Not verified: the paragraph numbers (§14, §15, §20, §85, Anlage 4, Anlage 10) under the new text; the draft amendment that moves the DIN 4108-2 reference to 2026-05 (not confirmed in force).")
    L.append("- Required change: where the files say 'GEG 2024' as the current law, write 'Gebäudemodernisierungsgesetz (GModG, vormals GEG, Stand 2026-07-29)'.")
    L.append("")
    L.append("## (d) Occurrence index (every match, file:line, paths relative to the energy folder)")
    L.append("")
    for r in ROWS:
        locs = ", ".join(f"{p}:{n}" for p, n in per[r["key"]]) or "none in grep"
        L.append(f"### d{r['key']}. {esc(r['label'])} ({len(per[r['key']])})")
        L.append("")
        L.append(locs)
        L.append("")
    L.append("## (e) Notes")
    L.append("")
    L.append("- The reports' claim that DIN 4108-2 'Tab. 7 (F_C)' is verified rests on the 2013-02 edition; the notation F_C = g_tot/g is confirmed as a definition (Baunetzwissen 'Abminderungsfaktor', ift Rosenheim EnEV note), but Table 7 of the 2026-05 edition was not read.")
    L.append("- VDI 2078 'revision announced for 2027' (cooling README) is not supported by any source found; remove it or verify.")
    L.append("- Pettenkofer / DIN 1946-6 link on the ventilation corner label: the 1000 ppm limit is from the UBA/Ad-hoc-AG article; the DIN 1946-6 attribution on the same label is not verified.")
    L.append("- Not checked in this audit: IWU 2015 typology, IEST-RP-CC012, ISO 14644-4, DIN 1946-7, DIN 18032-1, DIN 18380, DIN 51603-1, DIN EN 308, DIN EN 14511, VDI 2052, VDI 2087 (marked UNVERIFIED).")
    L.append("- Outside the table: ISO 13790 and EN ISO 52016-1 (named in heating notes, not on screen) were not checked.")
    L.append("")
    with open(OUT, "w", encoding="utf-8") as fh:
        fh.write("\n".join(L) + "\n")
    print("rows", len(ROWS), dict(counts))
    print("written", OUT)


if __name__ == "__main__":
    main()
