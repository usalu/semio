# Research: Reference Data for Architecture Energy Quizzes

Scope: defensible reference values (typical + plausible range + source) for the four quizzes
(Physikalisches Verständnis, Heizen, Kühlen, Energiebedarf) on `quizze.architektur-und-technologie.de`.
All numeric sorting lists are internally consistent (strictly increasing, gaps noted); matching-task
values are distinguishable. German sources preferred (GEG 2024, DIN 4108/18599/16798, IWU/TABULA, PHI,
AGEB/UBA). Where I could not pull an exact table cell from a paywalled/PDF-only norm in this session,
I built an engineering-consistent representative value from adjacent authoritative data points and
flagged it in Remarks — these should be spot-checked against the primary source before publication.

---

## 0. Existing repo reference data (reusable)

Repo search (`rg -i "u-wert|uwert|18599|4108|16798|luftwechsel|passivhaus|geg 2024"`) found **no
`framework/products/quiz` folder yet** (only `presentation`, `os`, `print`, `server`, `repo` exist under
`🧰️framework/🛍️products`) — the quiz product itself is still to be built per the ticket.

It did find a **headless norm-compliance engine** at `✏️s/🔌️plugins/📕️norm/🗿️artifacts/` with artifacts
`🧱️din4108`, `⚡️din18599`, `🌬️din16798` (plus Eurocodes, ISO 16757, VDI 3805 — structural/product-data,
not relevant here). These are **compliance calculators** (U-value from layer buildup, monthly heat-balance
per DIN V 18599-2, Glaser condensation per DIN 4108-3), not a lookup table of typical/representative values
per building type — so they don't directly answer "what U-value does a 1970s window have", but they **do**
contain hand-coded norm constants worth reusing verbatim as sourced answers:

| Constant (file: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`) | Value | Matches quiz item |
|---|---|---|
| `geg_anlage2_reference_u::WALL` | 0.28 W/(m²K) | Heizen T1 "Außenwand EnEV2014/GEG" |
| `geg_anlage2_reference_u::ROOF` | 0.20 W/(m²K) | Heizen T1 "Dach nach GEG" |
| `geg_anlage2_reference_u::FLOOR` | 0.35 W/(m²K) | Heizen T1 "Kellerdecke/Bodenplatte nach GEG" |
| `geg_anlage2_reference_u::WINDOW` | 1.3 W/(m²K) | Heizen T1 "Fenster GEG-Referenzausführung" |
| `geg_anlage2_reference_u::DOOR` | 1.8 W/(m²K) | Heizen T1 "Haustür nach GEG-Referenz" |
| `geg_anlage4_primary_energy_factors::{NATURAL_GAS,HEATING_OIL,ELECTRICITY_GRID,DISTRICT_HEATING,BIOMASS}` | 1.1 / 1.1 / 1.8 / 0.7 / 0.2 | Energiebedarf T2 primary-energy conversion |
| `din_v_18599_10_outdoor_air_change::{WFH,OFFICE,SCHOOL}` | 0.5 / 1.0 / 1.5 (1/h) | Kühlen T1 "Wohnung", "Einzelbüro" |
| `din_v_18599_12_tabelle5_qp_specific::{RESIDENTIAL,OFFICE,SCHOOL}` | 66 / 100 / 85 kWh/(m²a) | Energiebedarf T2 "GEG-Referenz Tabellenverfahren" |
| `din_v_18599_10_zone_defaults::{THETA_I_HEAT_C,THETA_I_COOL_C,INTERNAL_GAINS_W_M2}` | 20 °C / 26 °C / 3.5 W/m² | general context |

I cross-checked these against independent web sources (GEG 2024 Anlage 1 PDF) and they **match exactly**
(wall 0.28, roof 0.20, window 1.3 W/m²K) — good independent confirmation of the repo's own norm constants.
Verbatim source: `geg-info.de/geg_2024/anlage_01_geg_2024_referenzgebaeude_wohnbau.pdf`.
One caution used below: DIN V 18599‑10's `SCHOOL` outdoor-air-change constant (1.5 1/h) is an **annual
whole-building energy-balance default**, not the CO₂-driven design ventilation rate for a fully occupied
classroom (which DIN EN 16798‑1 / UBA guidance puts far higher, ~3–6 1/h) — flagged explicitly in the
Kühlen table so the quiz doesn't silently contradict itself.

No other reusable numeric answer tables were found in the repo (`♻️mit-bestand/🔎️recherche` hits were all
unrelated circular-construction/material-passport research).

---

## 1. Quiz: Physikalisches Verständnis

### Task 1 — Classification: Leistung (power, W) vs. Energie/Energiebedarf (energy, Wh/kWh)

| id-slug | DE | EN | Klassifikation | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|---|
| teelicht-waermeleistung | Teelicht (Wärmeleistung) | Tea light (heat output) | Leistung | 35 | W | 30–40 | Kalorimetrie handelsüblicher Teelichter (Paraffin, ~23 g/6 h) | Rate, nicht Brenndauer-Energie |
| wasserkocher-leistung | Wasserkocher | Electric kettle | Leistung | 2000 | W | 1800–3000 | VDE-Typenschildangaben handelsüblicher Geräte | begrenzt durch 230 V/16 A-Steckdose |
| jahresstromverbrauch-2p-haushalt | Jahresstromverbrauch eines 2-Personen-Haushalts | Annual electricity use, 2-person household | Energie | 2500 | kWh/a | 2000–3500 | co2online/Stromspiegel Deutschland | ohne elektr. Warmwasser |
| handyladung | Eine Handyladung | One smartphone charge | Energie | 15 | Wh | 10–20 | typ. Akkukapazität 3000–5000 mAh @ 3.7–3.85 V | |
| akw-block-leistung | Atomkraftwerk-Block (elektrische Leistung) | Nuclear plant unit (electrical output) | Leistung | 1400 | MW | 900–1600 | KKW Isar 2 (1410 MW netto); Wikipedia „Kernkraftwerk Isar" | typ. deutscher DWR-Block |
| tankfuellung-auto | Tankfüllung eines Autos (Benzin) | Full car fuel tank (petrol) | Energie | 500 | kWh | 350–700 | Heizwert Benzin ≈ 8,9 kWh/l × 40–70 l Tankgröße | |
| led-standby-leistung | LED-Lämpchen im Standby | LED indicator, standby | Leistung | 0.3 | W | 0.1–1 | typ. Gerätestandby-Messwerte | |
| haartrockner-leistung | Haartrockner | Hair dryer | Leistung | 1800 | W | 1500–2200 | VDE-Typenschild | |
| backofennutzung-energie | Energieverbrauch einer Backofennutzung (1 h) | Energy use, one hour of baking | Energie | 1.5 | kWh | 1.0–2.5 | typ. Verbrauchsmessung Elektrobacköfen (2–3 kW, Taktbetrieb) | |
| jahresheizenergie-altbau-efh | Jahresheizenergiebedarf unsaniertes Einfamilienhaus | Annual heating energy, unrenovated SFH | Energie | 30000 | kWh/a | 20000–40000 | IWU/dena Basisdaten Gebäudebestand | ca. 150 m² × 200 kWh/m²a |
| led-panel-buero-leistung | Deckenleuchte Büro (LED-Panel) | Office LED ceiling luminaire | Leistung | 40 | W | 30–60 | typ. 62×62 cm LED-Panel, DIN V 18599-4 LPD-Grenzwert 12 W/m² | |
| pv-modul-nennleistung | Nennleistung eines PV-Moduls | Nominal power of a PV module | Leistung | 400 | Wp | 300–500 | handelsübliche Modul-Datenblätter 2024–2026 | Spitzenleistung, nicht Dauerleistung |
| pv-hausdach-jahresertrag | Jährlicher Solarertrag eines typischen Hausdachs (PV) | Annual solar yield, typical house-roof PV | Energie | 8000 | kWh/a | 5000–12000 | BSW-Solar/Fraunhofer ISE; ~950–1000 kWh/kWp·a in DE × 8–10 kWp | |
| kuehlschrank-leistung | Kühlschrank (Kompressor-Leistungsaufnahme) | Refrigerator (compressor power draw) | Leistung | 100 | W | 50–150 | typ. Kompressor-Nennleistung | nur während Laufzeit |
| kuehlschrank-jahresverbrauch | Kühlschrank Jahresstromverbrauch | Refrigerator annual electricity use | Energie | 150 | kWh/a | 100–250 | EU-Energielabel Klasse A–C, typ. Haushaltsgeräte | |
| fernwaerme-anschlussleistung-mfh | Fernwärme-Anschlussleistung Mehrfamilienhaus | District-heating connection capacity, apartment building | Leistung | 150 | kW | 80–300 | typ. Heizlastauslegung DIN EN 12831 für 20–30 WE | |

### Task 2 — Numerical sorting of **power** (tiny → huge)

Strictly increasing; log-gap noted where < factor 2 (both are real physical clustering, not an error —
flagged for the teaching discussion).

| id-slug | DE | EN | Referenzwert | Einheit (W) | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| led-standby | LED-Standby-Leuchte | LED standby indicator | 0.3 | W | 0.1–1 | Gerätemessungen | |
| smartphone-ladeleistung | Smartphone-Ladeleistung | Smartphone charging power | 10 | W | 5–20 | USB-PD/QC-Ladeprofile | |
| teelicht-waermeleistung-2 | Teelicht (Wärmeleistung) | Tea light (heat output) | 35 | W | 30–40 | s.o. | Gap zu Laptop nur ×1.4 — beide real nah beieinander |
| laptop-leistungsaufnahme | Laptop (Leistungsaufnahme im Betrieb) | Laptop (power draw in use) | 50 | W | 30–65 | typ. Notebook-Netzteilauslegung | |
| mensch-ruhe-grundumsatz | Mensch im Ruhezustand (Grundumsatz) | Resting human (basal metabolic rate) | 100 | W | 80–120 | Physiologie-Standardwert (~1 kcal/kg/h) | Lehrbuchwert; guter Vergleichspunkt zu Laptop |
| haartrockner-2 | Haartrockner | Hair dryer | 1800 | W | 1500–2200 | s.o. | |
| wasserkocher-2 | Wasserkocher | Electric kettle | 2200 | W | 2000–3000 | s.o. | Gap zu Haartrockner nur ×1.2 — beide durch 230 V/16 A-Kreis limitiert |
| pkw-motor-leistung | Pkw-Verbrennungsmotor (Nennleistung) | Car engine (rated power) | 100000 | W | 70000–150000 | typ. Mittelklasse-Pkw ~100–140 kW/136–190 PS | |
| windkraftanlage-onshore | Windkraftanlage Onshore (Nennleistung) | Onshore wind turbine (rated power) | 4000000 | W | 2000000–8000000 | BWE Marktreport; moderne Anlagen 2–6(–8) MW | |
| ice-traktionsleistung | ICE-Hochgeschwindigkeitszug (Traktionsleistung) | ICE high-speed train (traction power) | 8000000 | W | 4000000–9600000 | DB/Siemens: ICE 3 8 MW, ICE 1 9,6 MW | |
| kohlekraftwerksblock | Steinkohle-/Braunkohlekraftwerksblock (elektrisch) | Coal-fired power-plant unit (electrical) | 700000000 | W | 500000000–1100000000 | typ. moderner Großblock (z. B. Trianel Lünen ~750 MW, BoA-Blöcke ~1000 MW) | |
| akw-block-el | Atomkraftwerk-Block (elektrisch) | Nuclear plant unit (electrical) | 1400000000 | W | 900000000–1600000000 | KKW Isar 2, 1410 MW netto / 3950 MW thermisch | thermisch ≈ 4 GW; el./thermisch beide gültige Vergleichswerte |
| de-durchschnittslast | Deutschlands durchschnittliche Stromlast | Germany's average electricity grid load | 56000000000 | W | 50000000000–65000000000 | Destatis 2024: 491 TWh/a ÷ 8760 h | Bruttostromverbrauch, nicht Gesamtenergie |
| welt-primaerleistung | Weltweite mittlere Primärenergieleistung | World average primary power | 19600000000000 | W | 18000000000000–21000000000000 | IEA/Enerdata 2023: 619,6 EJ/a ÷ 31,56 Ms | ≈ 19,6 TW |
| sonneneinstrahlung-erde | Sonnenstrahlungsleistung auf die Erde | Solar power incident on Earth | 174000000000000000 | W | 170000000000000000–175000000000000000 | Solarkonstante 1361 W/m² × Erdquerschnittsfläche | ≈ 174 PW; ≈ 8880× Weltprimärleistung |

### Task 3 — Numerical sorting of **energy** (tiny → huge)

All gaps ≥ factor 4 on the log scale (mostly ≥10×).

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| handyladung-2 | Eine Handyladung | One smartphone charge | 0.015 | kWh | 0.010–0.020 | s.o. | 15 Wh |
| wasser-kochen-1l | 1 Liter Wasser kochen (20→100 °C) | Boiling 1 L of water | 0.13 | kWh | 0.10–0.15 | Physik: c=4.186 kJ/kg·K × 80 K + Verluste | |
| schokoriegel | Schokoriegel/-tafel (Energiegehalt) | Chocolate bar (energy content) | 0.6 | kWh | 0.4–0.8 | Nährwertkennzeichnung, ~530 kcal/100 g | 40–100 g Riegelgröße |
| tagesbedarf-nahrung | Täglicher Nahrungsenergiebedarf eines Menschen | Daily food energy intake of a person | 2.5 | kWh | 2.2–3.0 | DGE-Referenzwerte ~2000–2500 kcal/d | |
| heizoel-1l | 1 Liter Heizöl (Energieinhalt) | 1 litre of heating oil (energy content) | 10 | kWh | 9.8–10.5 | Brennwert Heizöl EL ≈ 10 kWh/l (Marktüblich) | |
| jahresstromverbrauch-haushalt | Jahresstromverbrauch 2-Personen-Haushalt | Annual electricity use, 2-person household | 2500 | kWh | 2000–3500 | s.o. | |
| jahresheizenergie-altbau-2 | Jahresheizenergiebedarf unsaniertes Einfamilienhaus | Annual heating energy, unrenovated SFH | 30000 | kWh | 20000–40000 | s.o. | 30 MWh |
| hiroshima-bombe | Hiroshima-Atombombe (Sprengenergie) | Hiroshima atomic bomb (energy release) | 17500000 | kWh | 12000000–20000000 | „Little Boy" ≈ 15 kt TNT ≈ 63 TJ | 17,5 GWh |
| de-jahresprimaerenergie | Deutschlands Jahresprimärenergieverbrauch | Germany's annual primary energy consumption | 3000000000000 | kWh | 2900000000000–3100000000000 | AGEB Jahresbericht 2023/2024: 2997,5 TWh (10 791 PJ) | ≈ 3000 TWh |
| welt-jahresprimaerenergie | Weltweiter Jahresprimärenergieverbrauch | World annual primary energy consumption | 172000000000000000 | kWh | 165000000000000000–180000000000000000 | IEA/Enerdata 2023: 619,63 EJ ≈ 172,1 PWh | ≈ 172 000 TWh |
| sonnenenergie-jahr-erde | Jährliche Sonnenenergie auf die Erde | Annual solar energy reaching Earth | 1520000000000000000000 | kWh | 1480000000000000000000–1560000000000000000000 | 174 PW × 8760 h | ≈ 1,52×10⁹ TWh; ≈ 8760× Weltprimärenergie (die Sonne liefert in 1 h ≈ 1 Jahr Weltenergiebedarf) |

---

## 2. Quiz: Heizen

### Task 1 — U-value matching (W/(m²K)), building component ↔ construction period/standard

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| einfachverglasung | Einfachverglasung (1 Scheibe) | Single glazing | 5.8 | W/(m²K) | 5.0–6.0 | DIN 4108-4 Tab. 3 / TABULA Baualtersklasse vor 1918 | |
| isolierverglasung-1970er | Isolierverglasung (2-fach, 1970er, luftgefüllt) | Insulating double glazing, 1970s | 3.0 | W/(m²K) | 2.7–3.4 | DIN 4108-4/TABULA BAK 1969–1978 | |
| waermeschutzverglasung-2fach | 2-fach Wärmeschutzverglasung (Low-E, Edelgas) | Low-E double glazing | 1.3 | W/(m²K) | 1.1–1.5 | DIN 4108-4/TABULA BAK 1995–2001; deckt sich mit GEG-Referenzfenster (1,3) | Ug-Wert der Verglasung |
| waermeschutzverglasung-3fach | 3-fach Wärmeschutzverglasung | Triple glazing | 0.7 | W/(m²K) | 0.5–0.8 | PHI/TABULA ab 2002 | Ug-Wert; Uw des Gesamtfensters typ. 0,8–1,0 |
| vollziegel-365 | Vollziegelmauerwerk 36,5 cm (unsaniert) | Solid brick wall, 36.5 cm (unrenovated) | 1.4 | W/(m²K) | 1.3–1.6 | DIN 4108-4/TABULA BAK vor 1918 | λ_Ziegel ≈ 0,5–0,8 W/mK |
| fachwerkwand | Fachwerkwand, ausgemauert (unsaniert) | Half-timbered wall, infilled (unrenovated) | 1.5 | W/(m²K) | 0.9–2.5 | DIN 4108-4 Bestandswerte / Denkmalpflege-Literatur | hohe Streuung je nach Gefachfüllung |
| betonwand-20cm | Betonwand 20 cm (ungedämmt) | Uninsulated 20 cm concrete wall | 3.3 | W/(m²K) | 3.0–3.6 | Berechnet: λ_Beton ≈ 2,1 W/mK, R_si/R_se n. DIN 4108-2 | |
| aussenwand-geg | Außenwand nach GEG-Referenzausführung | Exterior wall, GEG reference building | 0.28 | W/(m²K) | 0.24–0.30 | GEG 2024 Anlage 1 (§ 15 Abs. 1) — repo-verifiziert `geg_anlage2_reference_u::WALL` | |
| passivhauswand | Passivhauswand | Passive-house wall | 0.13 | W/(m²K) | 0.10–0.15 | Passivhaus Institut, Zertifizierungskriterien | |
| holzbalkendecke-unsaniert | Holzbalkendecke (unsaniert) | Timber beam ceiling (unrenovated) | 1.2 | W/(m²K) | 1.0–1.6 | DIN 4108-4/TABULA | |
| dach-ungedaemmt | Steildach (ungedämmt) | Pitched roof (uninsulated) | 1.5 | W/(m²K) | 1.0–2.0 | DIN 4108-4/TABULA vor 1978 | |
| dach-geg | Dach/oberste Geschossdecke nach GEG-Referenz | Roof / top-floor ceiling, GEG reference | 0.20 | W/(m²K) | 0.18–0.24 | GEG 2024 Anlage 1 — repo-verifiziert `geg_anlage2_reference_u::ROOF` | |
| dach-passivhaus | Dach Passivhaus | Passive-house roof | 0.12 | W/(m²K) | 0.08–0.15 | Passivhaus Institut | |
| kellerdecke-ungedaemmt | Kellerdecke (ungedämmt) | Uninsulated basement ceiling | 1.2 | W/(m²K) | 1.0–1.5 | DIN 4108-4/TABULA | |
| bodenplatte-geg | Bodenplatte/Kellerdecke nach GEG-Referenz | Floor slab, GEG reference (against ground) | 0.35 | W/(m²K) | 0.30–0.40 | GEG 2024 Anlage 1 — repo-verifiziert `geg_anlage2_reference_u::FLOOR` | |
| haustuer-alt | Haustür, alt (Vollholz) | Old solid-wood front door | 3.2 | W/(m²K) | 2.9–3.5 | DIN 4108-4 Bestandswerte | |
| haustuer-geg | Haustür nach GEG-Referenzausführung | Front door, GEG reference | 1.8 | W/(m²K) | 1.5–2.0 | GEG 2024 Anlage 1 — repo-verifiziert `geg_anlage2_reference_u::DOOR` | |
| glasbausteine | Glasbausteine (Glassteinwand) | Glass block wall | 2.8 | W/(m²K) | 2.6–3.0 | Herstellerdatenblätter (z. B. Pittsburgh Corning-Nachfolger)/DIN 4108-4-Analogwert | |
| fenster-geg | Fenster nach GEG-Referenzausführung | Window, GEG reference building | 1.3 | W/(m²K) | 1.1–1.4 | GEG 2024 Anlage 1 — repo-verifiziert `geg_anlage2_reference_u::WINDOW` | Uw-Gesamtfenster |

### Task 2 — Specific heating load (W/m²) and annual heating energy demand (kWh/(m²a)) by building + standard

Representative engineering values consistent with IWU/TABULA and dena Gebäudetypologie-Reports; exact
IWU cell values were not extractable from the PDF/Excel tables in this session (paywalled/binary) — verify
against `webtool.building-typology.eu` before final print. Passivhaus row is normative (PHI criterion).

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| heizlast-efh-1950er | Heizlast unsaniertes EFH, Baujahr 1950er | Heating load, unrenovated SFH, 1950s | 160 | W/m² | 120–220 | IWU/TABULA BAK vor 1958, Näherung | |
| heizwaermebedarf-efh-1950er | Jahresheizwärmebedarf unsaniertes EFH, 1950er | Annual heating demand, unrenovated SFH, 1950s | 220 | kWh/(m²a) | 180–280 | IWU/TABULA, Näherung | |
| heizlast-efh-wschvo77 | Heizlast EFH nach WSchVO 1977 | Heating load, SFH per WSchVO 1977 | 100 | W/m² | 80–130 | IWU/TABULA BAK 1969–1978 | |
| heizwaermebedarf-efh-wschvo77 | Jahresheizwärmebedarf EFH nach WSchVO 1977 | Annual heating demand, SFH per WSchVO 1977 | 150 | kWh/(m²a) | 120–180 | IWU/TABULA | |
| heizlast-efh-wschvo95 | Heizlast EFH nach WSchVO 1995 | Heating load, SFH per WSchVO 1995 | 70 | W/m² | 55–90 | IWU/TABULA BAK 1995–2001 | |
| heizwaermebedarf-efh-wschvo95 | Jahresheizwärmebedarf EFH nach WSchVO 1995 | Annual heating demand, SFH per WSchVO 1995 | 110 | kWh/(m²a) | 90–130 | IWU/TABULA | |
| heizlast-efh-enev2002 | Heizlast EFH nach EnEV 2002 | Heating load, SFH per EnEV 2002 | 55 | W/m² | 45–70 | IWU/TABULA BAK 2002–2009 | |
| heizwaermebedarf-efh-enev2002 | Jahresheizwärmebedarf EFH nach EnEV 2002 | Annual heating demand, SFH per EnEV 2002 | 85 | kWh/(m²a) | 65–100 | IWU/TABULA | |
| heizlast-efh-enev2014geg | Heizlast EFH nach EnEV 2014/GEG 2024 | Heating load, SFH per EnEV 2014/GEG 2024 | 40 | W/m² | 30–50 | GEG-Referenzgebäude, abgeleitet aus U-Werten Anlage 1 | |
| heizwaermebedarf-efh-enev2014geg | Jahresheizwärmebedarf EFH nach EnEV 2014/GEG 2024 | Annual heating demand, SFH per EnEV 2014/GEG | 55 | kWh/(m²a) | 40–70 | GEG-Referenzgebäude, abgeleitet | |
| heizlast-kfw55 | Heizlast KfW-Effizienzhaus 55 | Heating load, KfW Efficiency House 55 | 30 | W/m² | 22–38 | KfW-Förderrichtlinie/BEG, Näherung aus 55 % Referenz-QP | |
| heizwaermebedarf-kfw55 | Jahresheizwärmebedarf KfW-Effizienzhaus 55 | Annual heating demand, KfW Efficiency House 55 | 40 | kWh/(m²a) | 30–50 | KfW/BEG, Näherung | |
| heizlast-kfw40 | Heizlast KfW-Effizienzhaus 40 | Heating load, KfW Efficiency House 40 | 20 | W/m² | 15–28 | KfW/BEG, Näherung | |
| heizwaermebedarf-kfw40 | Jahresheizwärmebedarf KfW-Effizienzhaus 40 | Annual heating demand, KfW Efficiency House 40 | 28 | kWh/(m²a) | 20–35 | KfW/BEG, Näherung | |
| heizlast-passivhaus | Heizlast Passivhaus | Heating load, Passive House | 10 | W/m² | 8–10 | Passivhaus Institut (normatives Kriterium ≤10 W/m²) | |
| heizwaermebedarf-passivhaus | Jahresheizwärmebedarf Passivhaus | Annual heating demand, Passive House | 15 | kWh/(m²a) | 10–15 | Passivhaus Institut (normatives Kriterium ≤15 kWh/m²a) | |
| heizlast-altbau-gruenderzeit-mfh | Heizlast Altbau-MFH Gründerzeit (unsaniert) | Heating load, unrenovated Gründerzeit MFH | 120 | W/m² | 90–160 | IWU/TABULA MFH BAK vor 1918 | kompaktere A/V-Ratio als EFH → niedriger als EFH-1950er |
| heizwaermebedarf-altbau-gruenderzeit-mfh | Jahresheizwärmebedarf Altbau-MFH Gründerzeit | Annual heating demand, unrenovated Gründerzeit MFH | 180 | kWh/(m²a) | 140–220 | IWU/TABULA | |
| heizlast-plattenbau-unsaniert | Heizlast Plattenbau (unsaniert) | Heating load, unrenovated prefab slab building | 130 | W/m² | 100–160 | IWU/TABULA MFH DDR-Typenbauten | |
| heizwaermebedarf-plattenbau-unsaniert | Jahresheizwärmebedarf Plattenbau (unsaniert) | Annual heating demand, unrenovated prefab slab building | 170 | kWh/(m²a) | 140–210 | IWU/TABULA | |
| heizlast-plattenbau-saniert | Heizlast Plattenbau (saniert) | Heating load, renovated prefab slab building | 55 | W/m² | 40–70 | IWU/TABULA, typ. WDVS-Nachrüstung | |
| heizwaermebedarf-plattenbau-saniert | Jahresheizwärmebedarf Plattenbau (saniert) | Annual heating demand, renovated prefab slab building | 80 | kWh/(m²a) | 60–100 | IWU/TABULA | |
| heizlast-buerogebaeude-1970er | Heizlast Bürogebäude 1970er (unsaniert) | Heating load, 1970s office building (unrenovated) | 110 | W/m² | 80–150 | IWU Nichtwohngebäude-Typologie, Näherung | hoher Glasanteil, wenig Dämmung |
| heizwaermebedarf-buerogebaeude-1970er | Jahresheizwärmebedarf Bürogebäude 1970er | Annual heating demand, 1970s office building | 160 | kWh/(m²a) | 120–200 | IWU Nichtwohngebäude-Typologie, Näherung | |

---

## 3. Quiz: Kühlen

### Task 1 — Air-change rate (1/h) or outdoor-air rate, by building use

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| wohnung | Wohnung (Aufenthaltsraum) | Dwelling (living space) | 0.5 | 1/h | 0.4–0.6 | DIN 1946-6 (Feuchteschutzlüftung); repo-verifiziert `din_v_18599_10_outdoor_air_change::WFH = 0.5` | |
| schlafzimmer | Schlafzimmer | Bedroom | 0.6 | 1/h | 0.4–1.0 | DIN 1946-6/VDI 4300 | |
| einzelbuero | Einzelbüro | Single-occupancy office | 1.0 | 1/h | 0.8–2.0 | DIN V 18599-10 „Office"-Profil (repo-verifiziert, =1.0)/ASR A3.6 | |
| grossraumbuero | Großraumbüro | Open-plan office | 3.0 | 1/h | 2.0–5.0 | DIN EN 16798-1 Kat. II, personenbezogen hochgerechnet | |
| klassenzimmer | Klassenzimmer (Vollbelegung) | Classroom (full occupancy) | 4.0 | 1/h | 3.0–6.0 | DIN EN 16798-1/UBA-Leitfaden „Lüften in Schulen" | DIN V 18599-10 „School"-Jahresprofil (repo) = 1.5 1/h ist Ganzjahres-/Ganzgebäude-Mittelwert, nicht die besetzte Unterrichtsstunde — bewusst höherer Wert hier |
| hoersaal | Hörsaal | Lecture hall | 5.0 | 1/h | 3.0–8.0 | DIN EN 16798-1, hohe Personendichte | |
| besprechungsraum | Besprechungsraum | Meeting room | 6.0 | 1/h | 4.0–10.0 | VDI 2052/ASR A3.6, kurzzeitig hohe Belegung | |
| restaurant-gastraum | Restaurant (Gastraum) | Restaurant dining area | 8.0 | 1/h | 5.0–10.0 | VDI 2052 | |
| kueche-gewerblich | Gewerbeküche | Commercial kitchen | 20 | 1/h | 15–30 | VDI 2052 | |
| krankenhaus-op | Operationssaal (Raumklasse Ib) | Hospital operating room (room class Ib) | 20 | 1/h | 15–25 | DIN 1946-4 (≥1200 m³/h + 60 m³/h·m² Zusatzluft) | Raumklasse Ia Schutzbereich lokal >150–300-fach |
| labor | Labor (mit Abzug) | Laboratory (with fume hood) | 10 | 1/h | 8–12 | VDI 2052/TRGS 526 | |
| kino-theater | Kino-/Theatersaal | Cinema/theatre auditorium | 6.0 | 1/h | 4.0–8.0 | VDI 2052 | |
| sporthalle | Sporthalle | Sports hall | 4.0 | 1/h | 3.0–6.0 | DIN 18032-1/VDI 2052 | |
| hallenbad | Hallenbad (Schwimmhalle) | Indoor swimming pool | 3.0 | 1/h | 2.0–5.0 | VDI 2089 | Entfeuchtungsanforderung zusätzlich maßgeblich |
| tiefgarage | Tiefgarage (mechanisch belüftet) | Underground car park (mechanically ventilated) | 6.0 | 1/h | 3.0–10.0 | ASR A1.3/RLT-Richtlinien Garagen | CO-getriggerte Bedarfslüftung, variabel |
| reinraum-iso7 | Reinraum ISO-Klasse 7 | Cleanroom, ISO class 7 | 40 | 1/h | 20–60 | DIN EN ISO 14644-1/VDI 2083 | ISO-Klasse 5: 240–600 1/h |

### Task 2 — Specific cooling load (W/m²) and annual cooling demand (kWh/(m²a)) by building + standard

No single normed "typical value" table exists for cooling comparable to TABULA for heating — values below
are engineering estimates from VDI 2078 cooling-load practice, DIN EN 16798-1/DIN V 18599-2/-7, and
published benchmarking studies (dena/BINE for data centres and food retail); flag for review before print.

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| kuehllast-wohngebaeude-passivhaus | Kühllast Wohngebäude Passivhaus | Cooling load, Passive House dwelling | 5 | W/m² | 0–10 | PHI: aktiv gekühlt erst ab Bedarf >15 kWh/m²a relevant | i. d. R. keine aktive Kühlung nötig |
| kuehlbedarf-wohngebaeude-passivhaus | Jahreskühlbedarf Wohngebäude Passivhaus | Annual cooling demand, Passive House dwelling | 5 | kWh/(m²a) | 0–10 | PHI-Kriterium (Grenze 15 kWh/m²a bei aktiver Kühlung) | |
| kuehllast-buero-1970er-vollverglasung | Kühllast Büro 1970er, Vollverglasung | Cooling load, 1970s office, full glazing | 80 | W/m² | 60–120 | VDI 2078, unsaniert ohne außenl. Sonnenschutz | |
| kuehlbedarf-buero-1970er-vollverglasung | Jahreskühlbedarf Büro 1970er, Vollverglasung | Annual cooling demand, 1970s office, full glazing | 60 | kWh/(m²a) | 40–90 | DIN V 18599-2/-7, Näherung | |
| kuehllast-buero-neubau-sonnenschutz | Kühllast Büro-Neubau mit außenl. Sonnenschutz | Cooling load, new office with external shading | 35 | W/m² | 25–50 | VDI 2078 | |
| kuehlbedarf-buero-neubau-sonnenschutz | Jahreskühlbedarf Büro-Neubau mit außenl. Sonnenschutz | Annual cooling demand, new office with external shading | 20 | kWh/(m²a) | 12–30 | DIN V 18599-2/-7, Näherung | |
| kuehllast-rechenzentrum | Kühllast Rechenzentrum | Cooling load, data centre | 1200 | W/m² | 500–2000 | Branchenpraxis (Uptime Institute/BitKom-Referenzen) | serverraum-/rackflächenbezogen |
| kuehlbedarf-rechenzentrum | Jahreskühlbedarf Rechenzentrum | Annual cooling demand, data centre | 8000 | kWh/(m²a) | 4000–15000 | abgeleitet: Kühllast × ~8760 h Volllast-Näherung | 24/7-Betrieb, keine saisonale Abschaltung |
| kuehllast-krankenhaus | Kühllast Krankenhaus | Cooling load, hospital | 70 | W/m² | 50–100 | VDI 2078, Mischwert OP/Normalstation | |
| kuehlbedarf-krankenhaus | Jahreskühlbedarf Krankenhaus | Annual cooling demand, hospital | 50 | kWh/(m²a) | 30–80 | DIN V 18599-2/-7, Näherung | |
| kuehllast-supermarkt | Kühllast Supermarkt | Cooling load, supermarket | 100 | W/m² | 70–150 | VDI 2078, inkl. offene Kühlmöbel | |
| kuehlbedarf-supermarkt | Jahreskühlbedarf Supermarkt | Annual cooling demand, supermarket | 130 | kWh/(m²a) | 90–200 | dena/BINE Benchmark Lebensmitteleinzelhandel | Kühlmöbel dominieren, nicht nur Raumkühlung |
| kuehllast-hotel | Kühllast Hotel | Cooling load, hotel | 50 | W/m² | 35–70 | VDI 2078 | |
| kuehlbedarf-hotel | Jahreskühlbedarf Hotel | Annual cooling demand, hotel | 30 | kWh/(m²a) | 20–45 | DIN V 18599-2/-7, Näherung | |
| kuehllast-schule | Kühllast Schule | Cooling load, school | 30 | W/m² | 20–40 | VDI 2078, sommerlicher Wärmeschutz meist ohne aktive Kühlung | |
| kuehlbedarf-schule | Jahreskühlbedarf Schule | Annual cooling demand, school | 12 | kWh/(m²a) | 5–20 | DIN V 18599-2/-7, Näherung | meist nur Teilklimatisierung (EDV-/Medienräume) |

---

## 4. Quiz: Energiebedarf

### Task 1 — Energy-standard ↔ profile (spider/radar: Heizen, Kühlen, Lüften, Kosten)

**These are didactic composite values I assembled from the per-standard heating-demand rows in §2 Task 2,
the GEG/PHI airtightness (n50) criteria, and a representative blended gas/heat-pump-electricity price —
not lifted from one single official published table.** Use as a defensible starting point; recompute
`Kosten` if the quiz's assumed energy price differs.

| id-slug | DE | EN | Achse | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|---|
| altbau-heizen-achse | Unsanierter Altbau — Heizen | Unrenovated old building — Heating | Heizen | 220 | kWh/(m²a) | 180–280 | IWU/TABULA (s. §2 T2) | |
| altbau-kuehlen-achse | Unsanierter Altbau — Kühlen | Unrenovated old building — Cooling | Kühlen | 5 | kWh/(m²a) | 0–10 | i. d. R. keine aktive Kühlung; sommerliche Überhitzung als Risiko, nicht Bedarf | didaktischer Näherungswert |
| altbau-lueften-achse | Unsanierter Altbau — Luftdichtheit (n50) | Unrenovated old building — Airtightness (n50) | Lüften | 8 | 1/h | 6–12 | typ. unsanierte Bestandsgebäude, Blower-Door-Erfahrungswerte | niedriger n50 = besser; hier als Lüftungs-Achsenproxy |
| altbau-kosten-achse | Unsanierter Altbau — spez. Heizkosten | Unrenovated old building — specific heating cost | Kosten | 22 | €/(m²a) | 15–30 | abgeleitet: 220 kWh/m²a × ~10 ct/kWh Gasmischpreis | Preisannahme variabel |
| wschvo95-heizen-achse | WSchVO 1995 — Heizen | WSchVO 1995 — Heating | Heizen | 110 | kWh/(m²a) | 90–130 | s. §2 T2 | |
| wschvo95-kuehlen-achse | WSchVO 1995 — Kühlen | WSchVO 1995 — Cooling | Kühlen | 6 | kWh/(m²a) | 0–12 | didaktischer Näherungswert | |
| wschvo95-lueften-achse | WSchVO 1995 — Luftdichtheit (n50) | WSchVO 1995 — Airtightness (n50) | Lüften | 5 | 1/h | 4–7 | typ. Bestandserfahrungswerte 1990er | |
| wschvo95-kosten-achse | WSchVO 1995 — spez. Heizkosten | WSchVO 1995 — specific heating cost | Kosten | 11 | €/(m²a) | 9–14 | abgeleitet 110 kWh/m²a × 10 ct/kWh | |
| enev2014geg-heizen-achse | EnEV 2014/GEG 2024 — Heizen | EnEV 2014/GEG 2024 — Heating | Heizen | 60 | kWh/(m²a) | 40–70 | s. §2 T2 | |
| enev2014geg-kuehlen-achse | EnEV 2014/GEG 2024 — Kühlen | EnEV 2014/GEG 2024 — Cooling | Kühlen | 10 | kWh/(m²a) | 5–15 | größerer Fensteranteil als Altbau, didaktischer Näherungswert | |
| enev2014geg-lueften-achse | EnEV 2014/GEG 2024 — Luftdichtheit (n50) | EnEV 2014/GEG 2024 — Airtightness (n50) | Lüften | 2.5 | 1/h | 1.5–3.0 | GEG-Referenzausführung ΔU_WB/Praxiswerte | repo: `GEG_ANLAGE1_DELTA_U_WB_REF` kontextverwandt, kein direkter n50-Grenzwert im GEG |
| enev2014geg-kosten-achse | EnEV 2014/GEG 2024 — spez. Heizkosten | EnEV 2014/GEG 2024 — specific heating cost | Kosten | 6 | €/(m²a) | 4–9 | abgeleitet 60 kWh/m²a × 10 ct/kWh | |
| kfw40-heizen-achse | KfW 40 — Heizen | KfW 40 — Heating | Heizen | 28 | kWh/(m²a) | 20–35 | s. §2 T2 | |
| kfw40-kuehlen-achse | KfW 40 — Kühlen | KfW 40 — Cooling | Kühlen | 10 | kWh/(m²a) | 5–15 | didaktischer Näherungswert | |
| kfw40-lueften-achse | KfW 40 — Luftdichtheit (n50) | KfW 40 — Airtightness (n50) | Lüften | 1.0 | 1/h | 0.6–1.5 | typ. KfW-40-Nachweise mit Blower-Door-Test | |
| kfw40-kosten-achse | KfW 40 — spez. Heizkosten | KfW 40 — specific heating cost | Kosten | 3 | €/(m²a) | 2–5 | abgeleitet 28 kWh/m²a × 10 ct/kWh | |
| passivhaus-heizen-achse | Passivhaus — Heizen | Passive House — Heating | Heizen | 15 | kWh/(m²a) | 10–15 | PHI-Kriterium | |
| passivhaus-kuehlen-achse | Passivhaus — Kühlen | Passive House — Cooling | Kühlen | 6 | kWh/(m²a) | 0–10 | PHI: aktive Kühlung nur bei Bedarf >15 kWh/m²a angesetzt | |
| passivhaus-lueften-achse | Passivhaus — Luftdichtheit (n50) | Passive House — Airtightness (n50) | Lüften | 0.6 | 1/h | 0.5–0.6 | Passivhaus Institut, normatives Kriterium n50 ≤ 0,6 1/h | |
| passivhaus-kosten-achse | Passivhaus — spez. Heizkosten | Passive House — specific heating cost | Kosten | 1.8 | €/(m²a) | 1.2–2.5 | abgeleitet 15 kWh/m²a × ~12 ct/kWh (häufig Wärmepumpe/Strom) | |
| plusenergiehaus-heizen-achse | Plusenergiehaus — Heizen | Plus-energy house — Heating | Heizen | 15 | kWh/(m²a) | 10–20 | analog Passivhaus-Hülle + PV-Überschuss | |
| plusenergiehaus-kuehlen-achse | Plusenergiehaus — Kühlen | Plus-energy house — Cooling | Kühlen | 10 | kWh/(m²a) | 5–15 | oft mehr Verglasung für solare Gewinne/PV-Fläche | didaktischer Näherungswert |
| plusenergiehaus-lueften-achse | Plusenergiehaus — Luftdichtheit (n50) | Plus-energy house — Airtightness (n50) | Lüften | 0.6 | 1/h | 0.5–1.0 | analog Passivhausstandard-Hülle | |
| plusenergiehaus-kosten-achse | Plusenergiehaus — spez. Energiekosten (netto) | Plus-energy house — net specific energy cost | Kosten | -2 | €/(m²a) | -8–0 | Nettoenergiekosten nach PV-Einspeisung/Eigenverbrauchsgutschrift | negativ = Nettoertrag; stark einspeisevergütungs-/strompreisabhängig |

### Task 2 — Total specific final/primary energy demand (kWh/(m²a)) by building + standard

Three rows are **directly reused from the repo's own DIN V 18599-12 Tabelle 5 constants**
(`din_v_18599_12_tabelle5_qp_specific`) — the only place in the codebase with an authoritative,
already-cited per-usage-profile primary-energy table.

| id-slug | DE | EN | Referenzwert | Einheit | Plausibler Bereich | Quelle | Bemerkung |
|---|---|---|---|---|---|---|---|
| primaerenergie-wohngebaeude-geg-tabelle | Primärenergiebedarf Wohngebäude, GEG-Tabellenverfahren | Primary energy, residential, GEG tabular method | 66 | kWh/(m²a) | — | DIN V 18599-12:2018 Tab. 5 — repo-verifiziert `din_v_18599_12_tabelle5_qp_specific::RESIDENTIAL` | Vereinfachtes Nachweisverfahren, keine Range (Normwert) |
| primaerenergie-buerogebaeude-geg-tabelle | Primärenergiebedarf Bürogebäude, GEG-Tabellenverfahren | Primary energy, office, GEG tabular method | 100 | kWh/(m²a) | — | DIN V 18599-12:2018 Tab. 5 — repo-verifiziert `::OFFICE` | Normwert |
| primaerenergie-schule-geg-tabelle | Primärenergiebedarf Schule, GEG-Tabellenverfahren | Primary energy, school, GEG tabular method | 85 | kWh/(m²a) | — | DIN V 18599-12:2018 Tab. 5 — repo-verifiziert `::SCHOOL` | Normwert |
| endenergie-efh-1950er-unsaniert | Endenergiebedarf gesamt, unsaniertes EFH 1950er | Total final energy, unrenovated SFH 1950s | 250 | kWh/(m²a) | 200–320 | abgeleitet: Heizwärmebedarf §2 T2 + TWW + Hilfsstrom | |
| endenergie-efh-wschvo95 | Endenergiebedarf gesamt, EFH nach WSchVO 1995 | Total final energy, SFH per WSchVO 1995 | 130 | kWh/(m²a) | 110–160 | abgeleitet analog | |
| endenergie-efh-enev2014geg | Endenergiebedarf gesamt, EFH nach EnEV 2014/GEG | Total final energy, SFH per EnEV 2014/GEG | 70 | kWh/(m²a) | 55–90 | abgeleitet analog | |
| endenergie-efh-kfw55 | Endenergiebedarf gesamt, KfW-Effizienzhaus 55 | Total final energy, KfW Efficiency House 55 | 50 | kWh/(m²a) | 38–62 | abgeleitet analog | |
| endenergie-efh-kfw40 | Endenergiebedarf gesamt, KfW-Effizienzhaus 40 | Total final energy, KfW Efficiency House 40 | 35 | kWh/(m²a) | 25–45 | abgeleitet analog | |
| endenergie-efh-passivhaus | Endenergiebedarf gesamt, Passivhaus | Total final energy, Passive House | 25 | kWh/(m²a) | 20–35 | PHI-Heizkriterium 15 + TWW/Hilfsstrom | PHI-Primärenergiekriterium (PER) separat ≤ 60 kWh/m²a, anderes Bezugssystem |
| endenergie-efh-plusenergiehaus | Endenergiebedarf netto, Plusenergiehaus | Net final energy, Plus-energy house | 0 | kWh/(m²a) | -10–10 | Passivhaus-Hülle + PV-Bilanz, definitionsgemäß ≈ 0 oder negativ im Jahresmittel | |
| primaerenergie-efh-1950er-gas | Primärenergiebedarf, unsaniertes EFH 1950er (Gasheizung) | Primary energy, unrenovated SFH 1950s (gas heating) | 275 | kWh/(m²a) | 220–350 | 250 kWh/m²a × f_P Erdgas 1,1 — repo-verifiziert `geg_anlage4_primary_energy_factors::NATURAL_GAS` | |
| primaerenergie-efh-passivhaus-wp | Primärenergiebedarf, Passivhaus (Wärmepumpe/Strom) | Primary energy, Passive House (heat-pump/electric) | 45 | kWh/(m²a) | 35–60 | 25 kWh/m²a × f_P Strommix 1,8 — repo-verifiziert `geg_anlage4_primary_energy_factors::ELECTRICITY_GRID` | End- ≠ Primärenergiefaktor-System des PHI (PER); hier GEG-f_P-Systematik verwendet |

---

## Summary of key source anchors

- **GEG 2024, Anlage 1** (`geg-info.de/geg_2024/anlage_01_geg_2024_referenzgebaeude_wohnbau.pdf`): Außenwand 0.28, Dach/oberste Geschossdecke 0.20, Fenster 1.3, Außenwand/Bodenplatte gegen Erdreich 0.35 W/(m²K) — matches repo `geg_anlage2_reference_u`.
- **DIN 4108-4 / TABULA**, via elbmodsan.de richtwert table (BAnz AT 04.12.2020 B1/B2 + IWU TABULA): U-values by construction period for wall/roof/window/floor.
- **Passivhaus Institut**: Heizwärmebedarf ≤15 kWh/(m²a), Heizlast ≤10 W/m², n50 ≤0.6 1/h (passiv.de Zertifizierungskriterien PDF).
- **AGEB Jahresbericht 2023/2024**: Germany primary energy 2997.5 TWh (10,791 PJ) for 2023.
- **IEA/Enerdata 2023**: world primary energy 619.63 EJ ≈ 172.1 PWh.
- **Destatis**: Germany electricity consumption 2024 = 491 TWh → average grid load ≈ 56 GW.
- **Wikipedia „Kernkraftwerk Isar"**: Isar 2 = 1410 MW net electrical / 3950 MW thermal.
- **DIN 1946-4**: hospital OR ≥1200 m³/h + 60 m³/(h·m²) (room class Ib, ≈20-fold); room class Ia protected zone >150–300-fold.
- **DIN V 18599-10/-12** (repo `⚡️din18599` norm plugin, cross-checked): outdoor-air-change WFH/Office/School = 0.5/1.0/1.5 1/h; primary-energy qp,tab Residential/Office/School = 66/100/85 kWh/(m²a); primary-energy factors natural gas/oil 1.1, electricity 1.8, district heating 0.7, biomass 0.2.

Items marked "didaktischer Näherungswert" or "Näherung" (mainly the spider-chart cooling/cost axes and the
non-Passivhaus heating-load/demand pairs in §2 Task 2) are engineering-consistent estimates I built from
adjacent authoritative figures rather than a single directly-cited table cell — recommended to spot-check
against the IWU TABULA webtool, dena Gebäudereport, and VDI 2078 worked examples before finalizing the quiz
answer key.
