# Verification: GLOBAL reviewer rules in the German finals "Cooling demand" and "Energy balance"

Scope: read-only check of the two final German speaker videos and their sources.
- Video A (Cooling): `.🧬semio/🎓️teaching/⚡️cache/videos/final/de/speaker/rendered/Full_Cooling_Demand_speaker_de_1080p30.mp4` (18:22.6, 1920x1080, export 11:04)
- Video B (Energy balance): `.../Full_EnergyBalance_speaker_de_1080p30.mp4` (43:14.1, export 11:04)
- Sources: `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/` and `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/`
- Reference catalogue: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📚️references/🔣️.json`
- Frames: `🗑️generated/fix-heat/verify-cool-a|b|c/`, `verify-eb-a|b|c|d/`, contact sheets `verify-cool-sheet45.png`, `verify-eb-sheet45.png`, `verify-eb-s0..s4.png` (every 45 s, tiles of 640x360). All paths relative to the ticket folder.

Method: grep of sources for the forbidden patterns, frame extraction with `frame.sh` at the requested times, and 45 s contact sheets. Mid-animation frames (partly drawn formulas, cross-fades) were ignored. Scene cross-fades were checked before being called defects.

## Verdict table

| # | Rule | Cooling demand | Energy balance |
|---|------|----------------|----------------|
| 1 | F00 no bare bracket units | APPLIED | APPLIED |
| 2 | F10/F22 units on numbers, fraction bars | APPLIED (minor typography) | APPLIED (minor typography) |
| 3 | F11 "Temperaturdifferenz Δθ", never Delta-Theta | APPLIED | APPLIED |
| 4 | F06 roadmap is the first content scene | APPLIED (see note N1) | APPLIED (see note N1) |
| 5 | F06 Q̇ consistent, index V = ventilation | PARTLY | APPLIED (see note N2) |
| 6a | F18 "Kühltechnik" where the system is meant | PARTLY | APPLIED (no technical "Kühlung" in video) |
| 6b | F28 F_C (not F_sh) in solar scenes | APPLIED | N/A (no solar shading factor shown) |
| 7 | F01/F03/F12 references findable (edition + locator) | PARTLY | PARTLY (overview card only, see row 8) |
| 8 | Overview/intro and last 10 s show real frames | APPLIED | PARTLY (explicit "Noch keine Videos" card at end) |
| 9 | 45 s samples: no held-frame layout defect | APPLIED | APPLIED |

Extra check (global, not in the rule list): ρ with condition (F08). Cooling APPLIED (`Luftdichte bei 20 °C ρ_a ≈ 1,2 kg/m³`, frame 810). EB APPLIED (`recap/🐍️.py:651-652`, 1,27 kg/m³ at 5 °C shown with ρ ≈ 1,2 at 20 °C).

## Evidence per rule

### Rule 1, F00 bracket units
- Cooling: `grep` for `[\mathrm`, `\;[`, `\,[`, `[W]`, `[kWh]`, `[K]`, `[W/` over all cooling `🐍️.py` finds nothing. Units are written as `in $\mathrm{W}$`, `in $\mathrm{kW}$` (e.g. `internal-gains/🐍️.py:267`, `system-sizing/🐍️.py:299`).
  - Frame `verify-cool-b/t_765.png`: `Q̇_S in W` next to the equation (allowed form).
  - Frame `verify-cool-b/t_810.png`: `Q̇_V,sens = ... Q̇ in kW`.
  - Frame `verify-cool-a/t_5.png` ... `t_30.png`: roadmap has no brackets.
- EB: same grep, zero hits. Frame `verify-eb-b/t_586.png`: `Q_c = Q_g - η_c · Q_l   Q in kWh`. Frame `verify-eb-s3.png` (tile 1710): `Q in kWh/(m²a)` legend.

### Rule 2, F10/F22 units on numbers and fraction bars
- Cooling: `verify-cool-b/t_765.png`: `Q̇_S = 1,92 m² · 0,70 · 0,15 · 0,60 · 800 W/m² ≈ 97 W` (units on every dimensional number). `verify-cool-b/t_855.png`: `q̇_v,R = Q̇_S / (ρ_a · c_p,a · Δθ)` with a fraction bar and `in m³/s`.
  - Minor: `verify-cool-b/t_270.png` writes `120W`, `70W` without the thin space (convention section 1).
- EB: `verify-eb-s0.png` tile 450: `η · Q_g ≈ 0,86 · 813 kWh ≈ 698 kWh`. `verify-eb-b/t_1350.png`: `Q_h,d = 30 m · 10 W/m · 3000 h = 900 kWh/a`, `Q_h,s = 1,8 kWh/d · 200 d = 360 kWh/a`. `verify-eb-s4.png` tile 2430: `Q_P = Q_E · f_P = 140 kWh/(m²a) · 1,1 = 154 kWh/(m²a)`. Fractions in display math are drawn with bars (`verify-eb-s2.png`, `verify-eb-s4.png`).
  - Minor: bar labels `154`, `55` without units (`verify-eb-s3.png` tile 1845, "Ein Gebäude, zwei Anlagen", legend says kWh/(m²a)) and `14000kWh/a` without space in the caption (`verify-eb-s2.png` tile 1350).

### Rule 3, F11 Temperaturdifferenz Δθ
- Grep `Delta-`, `Delta Theta`, `Delta x` in narration and captions of both clip trees: zero hits. Only hits are the `forbidden_source` test list (`🧪️tests/🔗️chain/🔣️.json:9`).
- Cooling: `system-sizing/🐍️.py:220` `$\Delta\theta$ ist der Temperaturhub ...`; frame `verify-cool-b/t_810.png` shows `Temperaturhub Δθ`. Term is "Temperaturhub", not "Temperaturdifferenz" as the reviewer wrote; the rule (symbol, no Delta-Theta) is met.
- EB: `regulations/🐍️.py:924` `Temperaturdifferenz $\Delta\theta = \theta_i - \theta_e$`; `recap/🐍️.py:613` `Die Temperaturdifferenz $\Delta\theta = \theta_i - \theta_e$ ...`; `recap/🐍️.py:1262` `die Temperaturdifferenz $\Delta\theta$ schrumpft`. Frame `verify-eb-c/t_600.png` and `verify-eb-s1.png` show `Δθ = 6K`, `Δθ = 32K`.

### Rule 4, F06 roadmap first
- Cooling: frame `verify-cool-a/t_5.png` is the roadmap (`Der Fahrplan des Videos`, step 1 and 2 visible, caption `Dieses Video folgt sechs Schritten.`); `t_6.png` shows all six steps; `t_12.png` step 1 highlighted; `t_20.png` step 2 with arrow; `t_30.png` step 3 with `← Lüftung: Q̇_V`.
- EB: frame `verify-eb-a/t_5.png` roadmap (`folgt sieben Schritten`); `t_6.png` all 7 steps; `t_12.png` step 1 `Bilanz`, `← Heizung: Q_h`, `← Kühlung: Q_c`; `t_20.png` step 2; `t_30.png` step 3.
- Note N1: both videos start with the course overview card for 0 to about 4 s (`verify-cool-a/t_0.png`, `verify-eb-a/t_0.png`, `t_2.png`), then the roadmap. The roadmap is the first content scene. If "first" means the literal first frame, the overview intro is in front of it.
- Roadmap source: `🗺️roadmap/🐍️.py:36` (EB) and `🗺️roadmap/🐍️.py:47-48` (cooling).

### Rule 5, F06 Q̇ consistency, index V reserved
- Cooling: loads consistently Q̇: `transmission-humidity/🐍️.py:215-216` `Der Wärmestrom $\Phi$ durch die opake Hülle, als Last am Raum $\dot{Q}_{\mathrm{T}}$, ist $U\cdot A\cdot\Delta\theta_{\mathrm{eq}}$`; `internal-gains/🐍️.py:267-268` `Der Wärmestrom $\Phi$ der Personen ...`. Index V only for ventilation: roadmap `🗺️roadmap/🐍️.py:58-61` (`Q̇_T, Q̇_V`, `Q̇_C, V̇`).
  - Exception (PARTLY): roadmap step 6 `🗺️roadmap/🐍️.py:62` `"symbols": r"Q_{\mathrm{c}}"` is not dotted. Frame `verify-cool-a/t_6.png` shows `Q_c` under "Sommerstrategie", while `t_12`/`t_20` and step 5 show `Q̇_C`.
- EB: energies are DIN notation with `Q` (kWh): `Q_h`, `Q_c`, `Q_V`, `Q_E`, `Q_P`; loads with dot `Q̇_H` (`verify-eb-s1.png` tile 1035; roadmap step 3 `verify-eb-a/t_20.png`, `Q̇_H, Q_h`). Frame `verify-eb-b/t_600.png`: narration `Heizen zählte den Jahresbedarf Q_h, Kühlen die Auslegungslast Q̇_C`, which names the distinction. Accepted per convention section 3.
  - Index V: EB frame `verify-eb-d/t_1100.png` `H_V = 75 W/K` (ventilation coefficient). Consistent.

### Rule 6a, F18 Kühltechnik
- Cooling: applied in `system-sizing` and `internal-gains/🐍️.py:783` `die Kühltechnik abführen muss`; frame `verify-cool-a/t_2.png` card title `Auslegung der Kühltechnik`.
  - NOT APPLIED for the DEC system: `summer-strategy/🐍️.py:304` `section_heading(tr("Sorption-assisted cooling (DEC)", "Sorptionsgestützte Kühlung (DEC)"))`. This names a technical system and should read "Sorptionsgestützte Kühltechnik (DEC)". Docstring line 274 has the same wording.
  - Other "Kühlung" uses are the concept "freie Kühlung" (`summer-strategy/🐍️.py:210`, frame `verify-cool-b/t_990.png` "freie Kühlung"), acceptable.
- EB: "Kühlung" only as the video reference `🗺️roadmap/🐍️.py:36` `das Kühlungsvideo` and label `🗺️roadmap/🐍️.py:61` `Kühlung: Q_c`, frame `verify-eb-a/t_12.png`. These refer to the video, not to a technical system. Treated as applied.

### Rule 6b, F28 F_C not F_sh
- Cooling: `solar-radiation/🐍️.py:99` docstring `the external blind F_C of DIN 4108-2`; frame `verify-cool-b/t_765.png`: `Q̇_S = A · F_F · F_C · g⊥ · I_s,max   Q̇_S in W` and `F_C = 0,15` (`verify-cool-b/t_720.png` shows `F_C = 0,15` with the sun-protection scale). Grep for `F_sh`, `Fsh`, `F_{sh}` in cooling and EB: no hits.
- EB: no solar shading factor shown (`Q_S = A · I_s`, `verify-eb-s0.png` tile 360). N/A.

### Rule 7, F01/F03/F12 references
- Cooling chips (frames `verify-cool-b/t_720.png`, `t_765.png`, `t_990.png`, and `verify-cool-a/t_2.png`):
  - `DIN 4108-2:2026-05 · Abschn. 8 – Sonnenschutzvorrichtung` (edition plus section; catalogue `🔣️.json:8-11`). APPLIED.
  - `DIN EN 16798-1:2022-03 · Anh. B – Kategorien der Raumluftqualität` (summer-strategy scene, frame `verify-cool-a/t_1093.png`). APPLIED.
  - `VDI 2078:2015-06 · Kühllast opake Bauteile`, `· innere Wärmequellen (Personen)`. Edition plus topic keyword; catalogue `locators` are keywords only (`🔣️.json:324-333`). PARTLY (no chapter number).
  - `VDI 6022 Blatt 1:2018-01 · Hygiene` (keyword only, `🔣️.json:347-352`). PARTLY.
  - `DIN V 4108-6:2003-06 · Solare Wärmegewinne`, frame `verify-cool-b/t_720.png`: no "(zurückgezogen)" status, although the catalogue marks it withdrawn (`🔣️.json:33-37`). The EB chip does show "(zurückgezogen)" (`verify-eb-s0.png` tile 45). Inconsistent. PARTLY.
- EB chips: `DIN/TS 18599-5:2025-10 · Übergabe · Verteilung · Speicherung` (`verify-eb-s2.png` tile 1395), `VDI 4650 Blatt 1:2024-02 · Jahresarbeitszahl (JAZ)` (frame `verify-eb-b/t_1495.png`; source `system-losses/🐍️.py:793` `_references.cite(("din-ts-18599-5", "erzeuger"), ("vdi-4650-1", "jaz"))`). The JAZ term is introduced in words (`system-losses/🐍️.py:777-779`). APPLIED.
  - `GModG (Stand 2026-07-29) § 22 Primärenergiefaktoren, Anlage 4` (frame `verify-eb-s3.png`). APPLIED.
  - `DIN EN 12831-1:2017-09 · Abschn. 6 – ausführliches Verfahren` (frame `verify-eb-d/t_1100.png`). APPLIED.
  - `Sunikka-Blank, Galvin (2012), Build. Res. Inf. 40(3)` (frame `verify-eb-s4.png` tile 2295): journal and volume, no page. Acceptable.
- The overview card references (`Willems, Schild, Stricker (2024), 8. Aufl.` and `Bohne, Bohne (2026), 13. Aufl.`, frames `verify-eb-a/t_2585.png`, `verify-cool-a/t_0.png`) are the Physik summary card. Edition correctly updated for Bohne (F01 point 1). Catalogue `bohne-2026` and `willems-2024` have `locators: {}` (`🔣️.json:430-443`), so no chapter or keyword is findable. PARTLY (F01 point 2, F03).

### Rule 8, overview/intro and last 10 s
- Cooling, 0 to 5 s: `verify-cool-a/t_0.png` course overview with real frames (Physikalische Grundlagen summary card, Lüftung house, Heizen formula, Kühlen house, Energiebilanz diagram). `verify-cool-a/t_2.png` chapter overview with real frames (Vom Heizen zum Kühlen, Interne Kühllasten, Transmission und Feuchte, Sonnenstrahlung, Auslegung der Kühltechnik, Sommerstrategie). `t_5` roadmap.
- Cooling, last 10 s (duration 1102.6 s): `verify-cool-a/t_1093.png` summer-strategy scene (real flowchart), `verify-cool-a/t_1101.png` overview cards (real frames). APPLIED.
- EB, 0 to 5 s: `verify-eb-a/t_0.png` architecture overview (real frames), `verify-eb-a/t_2.png` Energiebilanz chapter overview with seven real-frame cards (Energiebilanz, Regelwerk und Klima, Bedarf und Leistung, Nutzenergie und Endenergie, Primärenergie, Energieausweis, Theorie und Praxis). APPLIED.
- EB, last 10 s (duration 2594 s): `verify-eb-a/t_2585.png` architecture overview with real frames. `verify-eb-a/t_2593.png` Energie topic overview: BIM, Grasshopper, Excel, Optimierung cards show real frames, but the Parametrik card reads "Noch keine Videos" (an explicit empty placeholder, the box the UI report calls an "explicit localized empty card"). PARTLY, because the owner asked for no placeholder boxes in the intro.
- Cooling overview cards in the mid-video are not sampled beyond 0 to 5 s and the tail.

### Rule 9, held frames and layout (45 s samples)
- Cooling: `verify-cool-sheet45.png` (25 samples, 0 to 1080 s): no held overlap, text out of frame or clipped element seen at tile size. Only transitions at 0 s and the overview at 1080 s.
- EB: `verify-eb-s0..s4.png` (58 samples, 0 to 2565 s): no held overlap or text outside frame. Checked in full size: `verify-eb-b/t_584`, `t_586`, `t_592`, `t_600` (transient blank panel at 585 s is a scene cross-fade, then "Dieselbe Bilanz" continues), `verify-eb-d/t_1100.png` (H_T and H_V clean), `verify-eb-b/t_1350.png` (30 m Rohr label clean). Label overlap seen in tile 1485 disappeared by 1495 (mid-animation).

## Caveats
- Only frames at the requested times and 45 s samples were checked; formula frames in mid-animation are not verdict-bearing.
- Reference locators were checked against the catalogue (`🔣️.json`), not against the book or standard text. No external lookup was made.
- Videos were not re-rendered and no source file was changed.
