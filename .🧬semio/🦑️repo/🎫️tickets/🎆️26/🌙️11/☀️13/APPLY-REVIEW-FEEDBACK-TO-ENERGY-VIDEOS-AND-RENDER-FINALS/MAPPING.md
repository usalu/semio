# Mapping of reviewer feedback F00–F29 to energy clip sources

Read-only audit. No source file was edited. Source base:
`C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\` (abbreviated `E\` below).

Abbreviations used for the scene files (all under `E\`):

| Abbrev | File |
|---|---|
| PH | `🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py` |
| VN | `🌬️ventilation\🎬️clip\❓️need\🐍️.py` |
| VV | `🌬️ventilation\🎬️clip\📦️volume\🐍️.py` |
| VH | `🌬️ventilation\🎬️clip\🌡️heat-flow\🐍️.py` |
| VNat | `🌬️ventilation\🎬️clip\🪟️natural\🐍️.py` |
| VM | `🌬️ventilation\🎬️clip\⚙️mechanical\🐍️.py` |
| VHo | `🌬️ventilation\🎬️clip\🤝️handoff\🐍️.py` |
| HF | `🔥️heating\🎬️clip\🧲️fundamentals\🐍️.py` |
| HCv | `🔥️heating\🎬️clip\🌬️convection\🐍️.py` |
| HIG | `🔥️heating\🎬️clip\👥️internal-gains\🐍️.py` |
| HC | `🔥️heating\🎬️clip\🧱️conduction\🐍️.py` |
| HS | `🔥️heating\🎬️clip\☀️solar-gains\🐍️.py` |
| HCa | `🔥️heating\🎬️clip\🧮️calculation\🐍️.py` |

Each source line for narration is a tuple `("key", "English", "Deutsch")`. The German string is the second line after the key, and it is what the German video speaks. `tr(en, de)` is used for on-screen labels.

## 1. Timeline

### 1.1 Sources and method

- Durations come from `.🧬semio\🎓️teaching\⚡️cache\videos\en\speaker\rendered\` (ffprobe-equivalent via the bundled `ffmpeg-win-x86_64-v7.1.exe` of imageio_ffmpeg in the energy `.venv`):
  - Full videos: `Full_Physical_Fundamentals` 18:05.20, `Full_Ventilation` 31:16.60, `Full_Heating_Demand` 32:10.00. The sum of the six Ventilation chapters and the six Heating chapters matches these exactly.
  - Chapter files: `rendered\chapters\<Chapter>_speaker_en_854x480p15.mp4`. Each chapter has `.chapter.json` with `focus_frames` 54 (3.6 s intro) and `restore_frames` 44 (2.9 s outro).
  - Scene durations: `en\speaker\s\<n>\v\<SceneClass>.mp4`, one per segment (the table of segment renders is consistent with the chapter lengths to within about 5–6.5 s per chapter, which is the focus/restore time).
- **German render status:** there is no German concatenated video and no German chapter render for Physical Fundamentals, Ventilation or Heating in the cache (`.🧬semio\...\videos\de\speaker\rendered\` contains only BIM, Excel, Grasshopper and Optimization; `chapters\` contains only Excel/Grasshopper/BIM/Optimization). The German timeline the reviewer watched cannot be reproduced. The table below is the English timeline. Start times are the chapter start plus 3.6 s focus, then cumulative scene durations.
- **Reviewer timestamps vs English timeline:** the reviewer's timestamps run earlier than the English timeline, by a drift that grows through the video (about +1:00 at 17:00, +1:40 at 24:00, +2:45 at 29:00, +4:00 at 42:00, +4:34 at 44:51, about +5:30 at 55:00, +7:40 at 1:08:30). The German narration is therefore shorter than the English. The drift-corrected positions (English time = reviewer time + drift) agree with the content in every case below, which is strong support for the content matches. Per item, the drift-corrected position is given as "EN≈". These are estimates, not measured positions.
- Sub-scene positions inside a segment are not measurable from the cache (no speech audio per beat exists for English in the cache, and German audio is absent), so positions inside a scene are not exact.

### 1.2 English timeline (concatenation order physics → ventilation → heating)

Format: `scene | start (h:mm:ss) | duration (m:ss.ss)`. Chapter rows are bold in the title line.

```
## PhysicalFundamentals_FullSection  start 0:00:00.00   length 18:05.20
Beat1_Leistung                       0:00:03.60  3:11.40
Beat2_EnergieImAlltag                0:03:15.00  3:05.93
Beat3_Energieerhaltung               0:06:20.93  1:47.40
Beat4_Waermepumpe                    0:08:08.33  2:40.73
Beat5_Strahlung                      0:10:49.06  1:43.27
Beat6_ThermischeMasse                0:12:32.33  1:31.13
Beat7_SensibelLatent                 0:14:03.46  2:31.46
Beat8_Zusammenfassung                0:16:34.92  1:23.80

## Ventilation_01_WarumLueften         start 0:18:05.20   length 2:52.40
Beat1_Lueftungsbedarf (need)         0:18:08.80  1:12.60
Beat2_Lueftungsweg (need)            0:19:21.40  0:56.00
Beat3_Fahrplan (need)                0:20:17.40  0:37.27

## Ventilation_02_Luftmenge            start 0:20:57.60   length 4:25.80
Beat1_Innenvolumen (volume)          0:21:01.20  1:26.67
Beat2_Luftwechselrate (volume)       0:22:27.87  1:33.33
Beat3_Volumenstrom (volume)          0:24:01.20  1:20.67

## Ventilation_03_WaermestromLuft      start 0:25:23.40   length 5:03.87
Beat1_Luftwaerme (heat-flow)         0:25:27.00  1:08.27
Beat2_SensibleWaerme (heat-flow)     0:26:35.27  1:33.53
Beat3_LatenteLast (heat-flow)        0:28:08.80  1:23.13
Beat4_WinterUndSommer (heat-flow)    0:29:31.93  0:53.80

## Ventilation_04_NatuerlicheLueftung  start 0:30:27.27   length 10:01.93
Beat1_Fensterregeln (natural)        0:30:30.87  1:34.80
Beat2_StossOderKipp (natural)        0:32:05.67  1:59.80
Beat3_Querlueftung (natural)         0:34:05.47  1:27.60
Beat4_Auftrieb (natural)             0:35:33.07  1:50.20
Beat5_Nachtlueftung (natural)        0:37:23.27  1:38.27
Beat6_GrenzenDerFreienLueftung       0:39:01.54  1:26.13

## Ventilation_05_MechanischeLueftung  start 0:40:29.20   length 5:34.53
Beat1_Grundtypen (mechanical)        0:40:32.80  1:05.80
Beat2_Waermerueckgewinnung           0:41:38.60  1:40.73
Beat3_Anlagenvergleich               0:43:19.33  1:07.07
Beat4_Kanalquerschnitt               0:44:26.40  1:35.80

## Ventilation_06_Uebergabe            start 0:46:03.73   length 3:18.07
Beat1_Zusammenfassung (handoff)      0:46:07.33  1:55.33
Beat2_UebergabeHeizungKuehlung       0:48:02.66  1:16.87

## Heating_01_Introduction             start 0:49:21.80   length 10:10.40
Beat1_DreiWegeDerWaerme (HF)         0:49:25.40  1:17.47
Beat2_Waermeleitung (HF)             0:50:42.87  0:52.73
Beat3_Konvektion (HF)                0:51:35.60  0:58.33
Beat4_Strahlung (HF)                 0:52:33.93  0:48.27
Beat5_Zusammenfassung (HF)           0:53:22.20  1:15.00
Beat6_VonWegenZuZahlen (HF)          0:54:37.20  0:52.27
Beat7_Waermedurchlasswiderstand (HF) 0:55:29.47  1:16.87
Beat8_UWert (HF)                     0:56:46.34  1:15.07
Beat9_WaermestromFormel (HF)         0:58:01.41  1:27.87

## Heating_02_Conduction               start 0:59:32.20   length 7:04.27
Beat1_MakroUndMikro                  0:59:35.80  0:55.13
Beat2_RWert                          1:00:30.93  0:50.07
Beat3_UWertUndGradient               1:01:21.00  1:17.60
Beat4_WattVergleich                  1:02:38.60  1:00.40
Beat5_Gebaeudehuelle                 1:03:39.00  1:02.67
Beat6_Waermebruecken                 1:04:41.67  1:53.27

## Heating_03_Convection               start 1:06:36.47   length 1:18.27
Beat1_LueftungImWinter               1:06:40.07  1:13.13

## Heating_04_InternalGains            start 1:07:54.74   length 2:24.93
Beat1_WinterInterneGewinne           1:07:58.34  0:28.67
Beat2_PersonenPhiP                   1:08:27.01  0:25.93
Beat3_GeraetePhiE                    1:08:52.94  0:23.33
Beat4_BeleuchtungPhiL                1:09:16.27  0:26.53
Beat5_SummeUndDichte                 1:09:42.80  0:35.33

## Heating_05_Solar                    start 1:10:19.67   length 4:59.13
Beat1_VerlustZuGewinn                1:10:23.27  0:18.93
Beat2_BestrahlungUndFlaeche          1:10:42.20  0:31.53
Beat3_GWert                          1:11:13.73  0:45.20
Beat4_SaisonaleWinkel                1:11:58.93  0:40.20
Beat5_Verschattung                   1:12:39.13  0:52.40
Beat6_ThermischeMasse (store-room)   1:13:31.53  0:36.33   (clause segment, estimated)
Beat6_ThermischeMasse (winter)       1:14:07.86  0:07.73   (clause segment, estimated)
Beat8_Hauptgleichung                 1:14:15.59  1:01.67

## Heating_06_FinalCalculation         start 1:15:18.80   length 6:13.00
ReviewingHeatLosses                  1:15:22.40  0:47.47
Scene2                               1:16:09.87  0:57.20
ReviewingHeatGains                   1:17:07.07  0:52.33
Scene4                               1:17:59.40  1:20.13
UltimateEnergyBalance                1:19:19.53  0:34.73
AnlagenVerluste                      1:19:54.26  1:35.27

end of concatenation: 1:21:31.80 (4891.8 s)
```

Notes on the timeline:
- The Beat6 clause split in Heating_05 is inferred: the chapter length minus the other segments leaves 44.07 s, which equals 36.33 s plus 7.73 s from two hashed Beat6 renders. Medium confidence.
- Scene-level offsets inside a chapter can be off by up to about 1 min, because the segment renders do not perfectly sum to the chapter file lengths for some chapters (Ventilation_01 gap 6.5 s, Heating_01 gap 6.5 s, Heating_02 gap 5.1 s).

### 1.3 Drift between reviewer timestamps and English timeline

| Reviewer | Reviewer time (s) | Drift (EN minus reviewer) | EN≈ | EN scene |
|---|---|---|---|---|
| F04 | 17:08 | +1:00 | 18:08 | Beat1_Lueftungsbedarf |
| F06 | 19:24 | +1:00 | 20:24 | Beat3_Fahrplan (starts 20:17) |
| F07 | 20:02 | +1:00 | 21:02 | Beat1_Innenvolumen |
| F08 / F09 | 23:44 / 24:01 | +1:40 | 25:24 / 25:41 | Beat1_Luftwaerme |
| F12 | 29:20 | +2:45 | 32:05 | Beat2_StossOderKipp (start) |
| F13 / F14 / F15 | 32:55 / 33:29 / 34:04 | +2:50 to +3:00 | 35:45 to 37:04 | Beat4_Auftrieb, end of Beat4 and start of Beat5_Nachtlueftung |
| F16 | 36:57 | about +3:45 | about 40:40 | Beat1_Grundtypen (mechanical) |
| F17 / F18 | 42:29 / 42:44 | about +4:00 | about 46:30 | Beat1_Zusammenfassung (handoff) |
| F19 | 44:51 | +4:34 | 49:25 | Heating Beat1_DreiWegeDerWaerme |
| F20 | 47:01 | about +5:00 | about 52:00 | Heating Beat4_Strahlung |
| F21 | 48:31 | about +5:00 | about 53:30 | Heating Beat5_Zusammenfassung |
| F22 | 51:23 | about +6:00 | about 57:20 | Heating Beat8_UWert / Beat9 |
| F23 | 53:08 | about +6:00 | about 59:10 | Conduction Beat1_MakroUndMikro |
| F24 | 55:36 | about +5:30 | about 1:01:05 | Conduction Beat3_UWertUndGradient |
| F25 | 58:07 | about +6:10 | about 1:04:20 | Conduction Beat5 / Beat6_Waermebruecken |
| F26 / F27 / F28 | 1:03:24 / 1:03:47 / 1:04:50 | about +7:40 | about 1:11:05 / 1:11:30 / 1:12:30 | Solar Beat3_GWert, Beat3/4 border, Beat5_Verschattung (start 1:12:39) |
| F29 | 1:08:29 | +7:40 | 1:16:09 | Calculation Scene2 (start 1:16:09.87) |

The drift values from F19 onward are approximations. The matches are strong at F28 (F_sh in Beat5) and F29 (GTZ at Scene2 start).

## 2. Items

### F00 (global) Units "Q in W", "[Q] = W", "Q = 120 W"; "[W]" is not correct

- Video: all energy videos (Physical Fundamentals, Ventilation, Heating; the same pattern exists in Cooling and Demand).
- Chapter/scene: many, see code lines.
- Source: the unit is appended by the math panel helper as `(None, r"\;[\mathrm{W}]", COLOR)`. This is the "[W]" form the reviewer objects to.
- Hits in the three reviewed videos:
  - VH (heat-flow): 508 (Beat2 region, `dt` term), 666 (Beat3), 768 (Beat4 `Q̇_V` panel).
  - HF (heating fundamentals): 979 (Beat5 `Q̇ = ...`), 1648 (Beat9 `Q̇ = U · A · Δθ`).
  - HC (conduction): 824 (Beat3), 1076 (Beat5), 1091 (Beat5 `Φ_i [W]` label).
  - HS (solar): 574 (Beat2), 713 (Beat3 `Φ = G · A · F_f · g [W]`), 1066 (Beat5), 1139 (Beat8).
  - PH (physics): 630 and 762 (Beat1), `P\;[\mathrm{W}]`.
  - HIG (internal gains): 175 (helper that builds `Φ_p + Φ_e + … [W]`).
- Same pattern in Cooling (not reviewed): `❄️cooling\🎬️clip\☀️solar-radiation\🐍️.py:442`, `💧️transmission-humidity\🐍️.py:292, 489`, `📐️system-sizing\🐍️.py:294, 431`, `👥️internal-gains\🐍️.py:437, 627, 721, 847`.
- Reviewer concern: the "[W]" unit notation is wrong. The chosen convention should be applied in all panels consistently (either `Q in W`, `[Q] = W`, or `Q = 120 W`, with units on the result only or on every number).

### F01 "JAZ Bohne 2022": new edition this year; reference not findable under "JAZ"

- Video: physical fundamentals. Chapter: PhysicalFundamentals_FullSection. Scene class: `Beat4_Waermepumpe` (PH, class at 1674, construct at 1720). Drift-corrected position about 8:08 to 10:45 EN; the reviewer's 8:50 falls at the JAZ explanation.
- Narration tuple `("jaz", ...)`: PH lines 1700–1702. EN line 1701: "Averaged over a year this is the seasonal performance factor JAZ — for air-source units often around three." DE line 1702: "Übers Jahr gemittelt ergibt sich die Jahresarbeitszahl JAZ — bei Luft-Wärmepumpen oft rund 3."
- Mobject: PH line 1902: `tr(r"\mathrm{SPF} \approx 3 \quad \text{(Bohne 2022)}", r"\mathrm{JAZ} \approx 3 \quad \text{(Bohne 2022)}")`. The German label shows "JAZ ≈ 3 (Bohne 2022)".
- Reference list (Beat8 `refs` block, PH 3161–3170): PH 3164–3166: `("Bohne D (2022)", "Gebäudetechnik und technischer Ausbau von Gebäuden", "", tr("12th edition, Wiesbaden", "12. Auflage, Wiesbaden"), "https://doi.org/10.1007/978-3-658-37488-4", tr("Heat pump · Ventilation · Building services", "Wärmepumpe · Lüftung · Anlagentechnik"), P_CYAN)`.
- Reviewer objects to: (1) "2022" is outdated, a new edition appeared this year (the 12th edition cited is 2022). The edition year and the DOI must be updated to the new edition; I cannot verify the new edition from the repo, so the new edition's details are not checked here. (2) The label "JAZ" is not findable in the book (the reference has no keyword/page). Fix: cite a findable location (chapter or index keyword that exists in the book), and use the same wording in the label.
- Confidence: high for the location, medium for what the book actually contains (not verifiable offline).

### F02 "Hier muss zwischen Endenergie und Wärme unterschieden werden"

- Video: physical fundamentals. Scene: `Beat4_Waermepumpe` (PH 1674). EN≈ 9:46 (drift 0).
- Narration: `("final", ...)` PH 1703–1705 (EN 1704 "Final energy is what is delivered…", DE 1705 "Endenergie ist, was ans Haus geliefert und bezahlt wird…"). `("compare1", ...)` PH 1706–1708 (DE 1708: "Je 1 kWh Endenergie liefern Holzhackschnitzel rund 0,85 kWh Wärme, Heizöl rund 0,94, Erdgas rund 0,96.").
- Mobjects: PH 1925 `Text(tr("Final energy", "Endenergie"))`; PH 1957 `Text(tr("Useful heat from 1 kWh final energy", "Nutzwärme aus 1 kWh Endenergie"))`; `fe_heat_lbl` "Nutzwärme" (about PH 1945).
- Reviewer objects to: Endenergie (delivered energy, the meter) and Wärme (useful heat) are mixed in the compare line 1708 ("1 kWh Endenergie liefern … 0,85 kWh Wärme") and in the axis label. The two quantities need explicit labels, e.g., "Endenergie" vs "Nutzwärme" with the efficiency/conversion shown.

### F03 "Willems et al. 2024" unclear what is found here

- Video: physical fundamentals. Candidate scenes: `Beat5_Strahlung` (PH class 2028) and `Beat6_ThermischeMasse` (PH class 2308).
- Three occurrences of the `Willems et al. 2024` tag:
  - PH 2114 (Beat5, `norm_tag`, below the g-value label; the g-value narration `("g", ...)`).
  - PH 2243 (Beat5, `norm2`, shading/blinds scene, beams stopped).
  - PH 2577 (Beat6, `norm`, radiation exchange and thermal mass scene).
- Reference: PH 3161–3163 `("Willems WM, Schild K, Stricker D (2024)", "Formeln und Tabellen Bauphysik", "Wärmeschutz – Feuchteschutz – Klima – Akustik – Brandschutz", tr("8th edition, Wiesbaden", "8. Auflage, Wiesbaden"), "https://doi.org/10.1007/978-3-658-44974-2", tr("Radiation · Thermal mass · Moisture", ...))`.
- Drift-corrected position for 12:46 is EN≈12:46. The best time match is PH 2243 (Beat5 at about 12:08 by scene proportion), then 2114 (about 11:20), then 2577 (about 13:50). Confidence on which tag: low-medium. The three tags use the same bare text and no page or section; the reviewer's objection is that nothing specific is referenced.
- Fix: each tag should name the specific chapter or table (or the DIN/EN standard that the value comes from), and the bibliography entry should give the chapter that holds the quantity.

### F04 "Grenzwert nach Pettenkofer"

- Video: ventilation. Chapter: Ventilation_01_WarumLueften. Scene: `Beat1_Lueftungsbedarf` (VN class 213). EN≈ 18:08 (drift +1:00).
- Narration: VN 227–228. EN 227: "At about 1000 ppm the air counts as used up: the Pettenkofer value." DE 228: "Bei rund 1000 ppm gilt die Luft als verbraucht: der Pettenkoferwert."
- Mobject: VN 288–289: `tag(tr("Pettenkofer limit", "Pettenkoferwert"), r"1000\,\mathrm{ppm}", tick_pk.get_y())`. The German label is "Pettenkoferwert", EN "Pettenkofer limit".
- Docstring reference: VN 216 `https://de.wikipedia.org/wiki/Pettenkofer-Zahl` (not a book reference).
- Reviewer objects to: "Grenzwert" wording. The Pettenkofer value (1000 ppm CO2) is a hygienic guideline/Richtwert, not a legal limit. The text should say "Richtwert" or "Pettenkofer-Zahl" consistently, and the reference should be a citable source (a DIN or book reference, not a Wikipedia link).

### F05 "bedarf es immer der Lüftung neben Heizung und Kühlung"

- Video: ventilation. Scene: `Beat1_Lueftungsbedarf` (VN 213). EN≈ 18:52 (drift +1:00).
- Narration: VN 242–243. EN 242: "This need exists in winter as much as in summer, so ventilation comes before heating and cooling." DE 243: "Diesen Bedarf gibt es im Winter wie im Sommer, darum steht die Lüftung vor Heizung und Kühlung."
- Reviewer objects to: the sentence wording (the reviewer's version "bedarf es immer der Lüftung neben Heizung und Kühlung"). The source wording says the ventilation "comes before" heating and cooling, not "next to". Rewrite with the reviewer's intent: ventilation is always required in addition to heating and cooling.

### F06 Roadmap placement, consistent symbols (φ vs Q̇, index V, Δx)

- Video: ventilation (and all videos per the global note).
- Roadmap scene: `Beat3_Fahrplan` in VN (class 581, construct after). EN≈ 20:24 (drift +1:00). It is placed last in Ventilation_01 (EN 20:17).
- Roadmap symbols: VN line 620–623 `symbols = [r"V,\;n,\;\dot{V}", r"\Delta\theta,\;c_{\mathrm{Luft}},\;\Delta x", r"A_{\mathrm{eff}}", r"\eta_{\mathrm{WRG}}", r"\Phi_{\mathrm{V}},\;\dot{Q}_{\mathrm{L}}"]`.
  - Card 2 "Wärmestrom" (line 621) shows `Δx`, which is not introduced until VH Beat3 (`Latent`, line 572–575). This is the "Δx: warum hier?" objection.
  - Card 5 "Übergabe" (line 623) uses `Φ_V` and `Q̇_L`, which conflicts with the Lüftung symbol `V` (volume) in card 1.
- Heat symbol inconsistency: ventilation uses `Q̇` (VH 4–5 docstring, `Q̇_V,sens`); heating uses `Φ` (HCa 422–431 narration "Phi-V"/"Phi-T"; HCv 150 "Phi-V gleich V mal n mal c-Luft mal Delta-Theta"). The reviewer asks for `Q̇` for both, with Q′ (Q-Punkt) as the common symbol and `V` reserved for volume. Handoff uses `Φ_V` at VHo 389 ("Heating takes this flow over as the ventilation term phi V").
- Reviewer objects to: (1) roadmap must be the first thing in every video (move Beat3_Fahrplan to the start of Ventilation_01, before Beat1); (2) use one heat-flow symbol across the three videos; (3) do not introduce Δx in the roadmap before it is defined.

### F07 "die Außenluft verblasst": remove?

- Video: ventilation. Chapter: Ventilation_02_Luftmenge. Scene: `Beat1_Innenvolumen` (VV class 454). EN≈ 21:02 (drift +1:00).
- Narration: VV 466. DE: "Die Außenluft verblasst — getauscht wird nur die Luft innen. Wände, Decken und Dach sind aber keine Luft." (EN counterpart at VV 465.)
- Reviewer objects to: the phrase "verblasst" (fades) has no physical meaning. Delete the sentence, or rephrase to "Nur die Luft im Raum wird getauscht; Wände, Decken und Dach sind keine Luft."

### F08 "ρ ist mE etwas größer, 1,27 kg/m³. Prüfen"

- Video: ventilation. Scene: `Beat1_Luftwaerme` (VH class 263). EN≈ 25:24 (drift +1:40).
- Constant: VH 107 `RHO_AIR, CP_AIR = 1.2, 1005.0` (also VNat 281 and VHo 83 `RHO_AIR = 1.2`).
- Mobject: VH 310 `rho_lbl = math_label(rf"\rho = {label_number(RHO_AIR, 1)}\,\mathrm{{kg/m^{{3}}}}", ...)`. The one-decimal format prints "ρ = 1.2 kg/m³".
- Natural density function for comparison: VNat 290 `_rho_air(theta)` ideal gas, `ρ = p/(R·T)` with p = 101325 Pa (computed, not shown).
- Reviewer objects to: the density constant. ρ at about 10–15 °C is about 1.25–1.27 kg/m³; the value 1.2 is the 20 °C figure. The constant and the label precision (one decimal hides the difference) should be fixed; it also changes `c_Luft = ρ·c_p/3600` (VH 108).
- Confidence: high for location.

### F09 Folie with units: top "Heizung" → "Gebäudebezogene Einheiten", bottom "SI-Einheiten" with note on DIN variation

- Video: ventilation. Scene: `Beat1_Luftwaerme` (VH 263–418). EN≈ 25:41 (drift +1:40).
- Units panels: VH 376–383. `heiz_row` (`\dot{V}[m³/h] · c_Luft[Wh/(m³K)] = W/K`), `kuehl_row` (`q_{v,R}[m³/s] · ρ·c_p[J/(m³K)] = W/K`).
- Chips: VH 385 `chip(tr("Heating", "Heizung"), HEIZUNG)`, `chip(tr("Cooling", "Kühlung"), KUEHLUNG)`.
- Narration: VH 281–284 (DE 281: "Die Energie je Kubikmeter und Kelvin ist die volumenbezogene Wärmekapazität der Luft, c-Luft — etwa 0,34 Wattstunden.", DE 284: "Heizung und Kühlung schreiben dieselbe Größe verschieden: c-Luft in Wh/(m³·K) oder ρ mal c-p in J/(m³·K).").
- Reviewer objects to: the two rows mix Wh/(m³K) and J/(m³K) and Heizung/Kühlung labels on a per-building-unit basis. The reviewer's proposal: top header "Gebäudebezogene Einheiten" (building-related units, Wh/(m³K), m³/h) and bottom header "SI-Einheiten" (J/(m³K), m³/s), with a note that DIN usage varies.

### F10 "in der Mitte fehlen Einheiten" (also 29:35)

- Video: ventilation. Scene: `Beat1_Luftwaerme` (VH 263–418). EN≈ 25:50 (drift +1:40).
- Mid-equation without units: VH 400–402 `perk = math_readout(lambda: rf"\dot{{V}} \cdot c_{{\mathrm{{Luft}}}} = {label_number(V_DOT * k.get_value())} \cdot {label_number(C_LUFT, 2)} \approx {label_number(V_DOT * C_LUFT * k.get_value())}\,\mathrm{{W/K}}")`. The readout drops units from the factors (`V_DOT` and `C_LUFT`) and carries `W/K` only on the result.
- Also VH 405: `(None, r"\frac{\rho \cdot c_{p}}{3600\,\mathrm{s/h}}", P_WHITE)` and VH 401–402 with no units on the middle factors.
- Reviewer objects to: units missing in the middle of equations (also at 29:35 and in every video). Fix: put units on every number consistently, or only on the result.

### F11 Temperaturdifferenz "Delta-Theta" → "Δθ" everywhere

- Video: ventilation (and everywhere, global). Scene: `Beat2_SensibleWaerme` (VH 419). EN≈ 26:05 to 26:35 (drift +1:40).
- Occurrences of the spelled-out form "Delta-Theta" / "Delta-x" in narration (German strings):
  - VH 428 (Beat2): "Treiber ist die Temperaturdifferenz Delta-Theta, innen minus außen. …"
  - VH 431 (Beat2): "… Volumenstrom mal c-Luft mal Delta-Theta, in Watt."
  - VH 443 (Beat2): "Im Sommer: 26 minus 32, also Delta-Theta minus 6 Kelvin …"
  - VH 572 (Beat3): "… Delta-x beträgt 4,5 g/kg."
  - VHo 228 (handoff Beat1): "Delta-Theta ist die Triebkraft: 20 Kelvin im Winter, 6 Kelvin im Sommer."
  - VHo 234 (handoff Beat1): "Delta-x beträgt 4,5 Gramm je Kilogramm."
  - HC 672 (conduction Beat3), HC 1008 (conduction Beat5 "U-i mal A-i mal Delta-Theta-i"), HC 1204 (conduction Beat6 "Delta-U-WB"), HCa 330, 333, 425 (calculation), HCv 150.
- The on-screen math already uses `\Delta\theta`; the spelled form is only in narration.
- Reviewer objects to: the spelled "Delta-Theta" in narration. Change the German narration to "Temperaturdifferenz Δθ" (and "Delta-x" to "Δx", "Delta-U-WB" to "ΔU_WB") everywhere. Spoken form should be the symbol name.

### F12 "Referenz für die 8%"

- Video: ventilation. Scene: `Beat2_StossOderKipp` (VNat class 535). EN≈ 32:05 (drift +2:45; the 8 % is the "five" narration, early in Beat2).
- Narration: VNat 547 (DE): "Nach fünf Minuten ist beim weit offenen Fenster die ganze Luft getauscht, beim gekippten erst etwa 8 %."
- Nearest citation in the same scene: `din = _din_ref("DIN 1946-6")` at VNat 582. This does not cite the 8 % value; it is a general DIN reference for the scene.
- Reviewer objects to: the 8 % figure has no source. Fix: give the specific source (DIN standard, table, or book chapter) for the value, or label it as an example calculation with its assumptions (for example, 50 m³ room, air change rate per window opening).

### F13 "Fenster unten und Abluft oben sieht komisch aus. Für beides einfach Öffnungen, bzw. Kamin"

- Video: ventilation. Scene: `Beat4_Auftrieb` (VNat class 952). EN≈ 35:45 (drift +2:50).
- Narration: VNat 977 (DE): "Deshalb sitzen Zulüfte tief bei den Menschen und die Abluft hoch am Dach — Komfort ohne Ventilator." (the comfort step; EN 976).
- Mobjects: VNat 1002 `din = _din_ref("DIN EN 16798-7")`; inlet `_window(np.array([room_c[0] - 2.1, -0.85, 0.0]), height=0.60)` (around VNat 1009–1010, low window on the left wall); `inlet_lbl = Text(tr("Supply air", "Zuluft"))` VNat 1013; `outlet_lbl = Text(tr("Extract air", "Abluft"))` VNat 1030; shaft/outlet at roof.
- Reviewer objects to: the drawing of the low window and high exhaust is awkward. Replace with plain openings (inlet opening low, outlet opening high) or a chimney, as the reviewer suggests.
- Confidence: medium. Alternative candidate is Beat2 window drawing (`open_window(rk["window"], fraction=0.17, tilt=True)`, VNat 630), but the reviewer's time and the "Abluft oben" wording match Beat4.

### F14 "hier fehlt die Einführung Δρ(θ) und v(Δp)"

- Video: ventilation. Scene: `Beat4_Auftrieb` (VNat 952). EN≈ 36:30 (drift +3:00).
- Narration `("rho", ...)` VNat 966–969 (DE 969: "Der Dichteunterschied folgt aus der Temperaturdifferenz: 18 °C außen, 26 °C innen — je wärmer die Raumluft, desto stärker der Auftrieb."). `("taller", ...)` 970–972 ("der Strom wird schneller" = the v(Δp) hint, not formalized).
- Formula code: `_rho_air` VNat 290 (ideal gas), `_stack_pressure` VNat 299–303 (`Δp = h·g·(ρ_a − ρ_i)`).
- Mobjects: VNat 1067 `dp = meter("Δp", ...)`; 1080 `("dp", r"Δp", P_YELLOW), (None, "=", P_WHITE)`; 1083 `("rho", r"(\rho_{a} - \rho_{i})", P_BLUE)`; 1037/1042 `\rho_a`, `\rho_i` labels.
- Missing: no `Δρ(θ)` formula is shown (only words), and `v(Δp)` (speed from pressure, v = √(2Δp/ρ)) is not introduced at all.
- Reviewer objects to: no introduction of Δρ(θ) and v(Δp) before they are used. Add a short derivation/formula step before VNat 1067.

### F15 "Kontext zu Sonnenstrahlung und Verschattung visuell präsent"

- Video: ventilation. Candidate scene: `Beat5_Nachtlueftung` (VNat class 1189). EN≈ 37:05 (drift +3:00).
- Narration: VNat 1192 (EN 1192 "Summer comfort lives on night ventilation once the envelope is shaded."), 1201–1202 ("After sunset …"), 1207.
- Visual: sun glyph VNat 1305 (`sun = sun_glyph(sky_pos)`), 1342 `FadeIn(sun)`, 1316 `_badge(tr("Day"), tr("close + shade", "schließen + verschatten"))`.
- Secondary sun glyph in Beat2: VNat 751 (`sun = sun_glyph(...)` in the sum bar of the window-exchange energy row).
- Reviewer objects to: the context of solar radiation and shading is only in text/badges. Show the sun and the shading device in the image.
- Confidence: low-medium (the reviewer timestamp matches the start of Beat5 only with drift).

### F16 "Sieht nach Lüftungskurzschluss aus: Zuluft unten, Abluft oben"

- Video: ventilation. Candidate scene: `Beat1_Grundtypen` (VM class 249). EN≈ 40:40 (drift about +3:45).
- Narration: VM 255–262 (exhaust-only, supply-only, balanced supply/extract).
- Visual: balanced system group VM 295–297 (labels `Abluftanlage`, `Zuluftanlage`, `Zu-/Abluftanlage`), VM 364 `Abluft über Fugen`, VM 397 `("supply", [sup_fan], [GrowArrow(sup_in), FadeIn(leak), FadeIn(port_tags[1])]`.
- Other candidate: `Beat4_Auftrieb` inlet/outlet placement VNat 1013/1030 (Zuluft low, Abluft high; this is the correct arrangement, so the short-circuit look would have to come from the drawing).
- Reviewer objects to: the picture looks like a short-circuit (supply and extract too close), and the reviewer wants supply low and extract high. Fix the arrangement of the duct ports in the balanced-system drawing.
- Confidence: medium-low on the scene (mechanical Beat1 fits the drift; the natural Beat4 drawing is the alternative).

### F17 "3. Symbol rechts ist wohl die Enthalpie … latente Lüftungsverluste im Winter; der '-' ist nicht korrekt" (screenshot lost)

- Video: ventilation. Candidate scenes:
  - `Beat4_WinterUndSommer` (VH class 727), which has the winter/summer totals with a `+` (`\dot{Q}_V = \dot{Q}_{V,sens} + \dot{Q}_{V,lat}`, VH 765–769). Winter narration VH 735–736 ("Latent heat is not part of the heating load.") and the winter note VH 796–797 ("latent: not part of the heating load"). This is the strongest content match for "im Winter geht auch reichlich Energie über latente Verluste verloren".
  - `Beat1_Zusammenfassung` (VHo class 209): the card `("dx", r"\Delta x", "Feuchte, Sommer", …)` at VHo 204 and the latent narration VHo 236–237, 242–243.
  - `Beat2_Waermerueckgewinnung` (VM 424): the latent panel VM 686–698 (`\dot{Q}_{V,sens}\cdot(1-\eta) + \dot{Q}_{V,lat} = 90 W + 810 W`).
- Symbol "Enthalpie": not present in any file (grep of `Enthalp` returns nothing). The reviewer is right that the latent flow is not named as enthalpy.
- Minus sign: VM 626 and 673 (`−{…} W` recovery values) and VM 351 (the `−, +, =` pressure row). None of these is a winter latent term; the latent term is always `+`.
- Reviewer's time: 42:29 gives EN≈ 46:30 (drift +4:00), which matches the handoff (VHo Beat1). The heat-flow Beat4 match is at EN 29:30 (drift would be −13). Time favours the handoff, content favours heat-flow Beat4. Confidence on scene: low-medium.
- Fix: name the latent flow `Q̇_V,lat` (or `h`/enthalpy flow) explicitly in all three places, give the winter latent flow its own term, and remove the minus where the sum is shown.

### F18 "Kühlung → Kühltechnik"; "…spalte ist verwirrend. Die Summe darstellen und darauf Bezug nehmen"

- Video: ventilation. Scene: `Beat1_Zusammenfassung` (VHo class 209). EN≈ 46:45 (drift +4:00).
- "Kühlung" occurrences (rename to "Kühltechnik" as the reviewer asks):
  - VHo 177 `(tr("Cooling", "Kühlung"), COL_COOL)` (chip list, module-level before Beat1).
  - VHo 267 `head_s = body_text(tr("Summer · Cooling", "Sommer · Kühlung"), …)` (Beat1 summer header).
  - VH 385 `chip(tr("Cooling", "Kühlung"), KUEHLUNG)` (heat-flow Beat1).
  - VM 651 `s_head = head(tr("Summer · Cooling", "Sommer · Kühlung"), …)` (mechanical Beat2).
  - VHo 395–396 ("Im Sommer kehrt sich die Richtung um …").
- "…spalte": the table column is the card `("dt", r"\Delta\theta", "Winter / Sommer", lambda k: rf"{label_number(DT_WINTER * k)}\,/\,{label_number(DT_SUMMER * k)}\,\mathrm{{K}}", COL_DT)` at VHo 202–203. The "Winter / Sommer" column with two values in one cell is the confusing "…spalte". Replace with two separate rows or show the sum (for example 20 K + 6 K) and reference it.
- Sum: VHo 236–243 (latent 810 W; "Übrig bleiben 300 W im Winter und 90 W im Sommer"). The reviewer asks for the sum to be shown and referred to.

### F19 "der zweite Hauptsatz der Thermodynamik"

- Video: heating. Chapter: Heating_01_Introduction. Scene: `Beat1_DreiWegeDerWaerme` (HF class 450). EN≈ 49:25 (drift +4:34).
- Narration: HF 465 (EN) and 466 (DE): "Von allein nie umgekehrt. Das ist der zweite Hauptsatz — und der Grund, warum ein Gebäude überhaupt beheizt werden muss."
- Label: HF 477 `din = _din_ref(tr("2nd law", "2. Hauptsatz"))`.
- Reviewer objects to: "der zweite Hauptsatz" without "der Thermodynamik". Fix: "der zweite Hauptsatz der Thermodynamik" (and the same in the English label "2nd law of thermodynamics").

### F20 "Bezug zu elektromagnetischer Welle und Licht herstellen"

- Video: heating. Chapter: Heating_01_Introduction. Scene: `Beat4_Strahlung` (HF class 827). EN≈ 52:00 (drift about +5:00).
- Narration: `("label", …)` HF 829–831; `("waves", …)` HF 832–834: EN 833 "Every warm surface emits invisible infrared radiation: the radiator to the wall, the outer wall to the cold night sky." DE 834 "Jede warme Oberfläche sendet unsichtbare Infrarotstrahlung aus: der Heizkörper an die Wand, die Außenwand an den Nachthimmel."
- Other radiation narration: `("still", …)` HF 838–839 ("Die Strahlung kommt trotzdem an — sie braucht kein Medium."), HF 840–841 (sun's heat across empty space).
- No narration or mobject mentions electromagnetic waves or light. Light is shown only as infrared text.
- Reviewer objects to: the missing link to electromagnetic waves and light (visible light vs infrared, both part of the same spectrum; the sun is the example). Add one narration line and a spectrum label in this scene.

### F21 "Text in der Box ist zu kryptisch: 'Wärmeübergang Rsi und Rse fasst Strahlung und Konvektion zusammen'; U-Wert braucht eine Einführung → später"

- Video: heating. Scene: `Beat5_Zusammenfassung` (HF class 925). EN≈ 53:30 (drift about +5:00).
- Narration: HF 930–932 (EN 930 "Inside, convection and radiation bring the heat to the wall…", DE 932 "Innen bringen Konvektion und Strahlung die Wärme an die Wand…"). HF 940 (EN) and 941 (DE): "DIN EN ISO 6946 fasst beide Oberflächen zu R-si und R-se zusammen — mit Summe d durch Lambda ergibt das den U-Wert."
- The box text the reviewer quotes corresponds to the sentence at HF 940–941 (the R_si / R_se summary). The R_si/R_se box uses the ISO 6946 wording (HF 438–439 and conduction 107: `R_si + Σ d/λ + R_se`).
- Reviewer objects to: the box is cryptic, and the U-value needs an introduction, which should move later (to Beat8_UWert, HF 1382, where U = 1/R_ges is introduced at HF 1434).
- Fix: move the U-value sentence out of Beat5 and use plain wording for the surface resistances.

### F22 "Wieder Einheiten in der Mitte der Gleichungen (alles prüfen); besser mit Bruchstrich" (screenshot lost)

- Video: heating. Candidate scenes: `Beat9_WaermestromFormel` (HF class 1533) and `Beat8_UWert` (HF class 1382). EN≈ 57:20 (drift about +6:00).
- Equation with units only at the end: HF 1642–1648 `math_panel([("q", r"\dot{Q}"), (None, "="), ("u", "U"), (None, r"\cdot"), ("a", "A"), (None, r"\cdot"), ("dt", r"\Delta\theta"), (None, r"\;[\mathrm{W}]", P_PRIMARY)])`. Here the units are on the result only, and the factors are joined by `\cdot`. The reviewer wants a fraction (Bruchstrich) form: `U·A·Δθ` written as a fraction with units.
- Units inside the readout: HF 1434 `u_src = lambda: rf"U = \frac{{1}}{{R_{{\mathrm{{ges}}}}}} = {label_number(u_now(), 2)}\,\mathrm{{W/(m^{{2}}K)}}"` (units only on the final number; the fraction is already used here, which is the form the reviewer likes), and HF 1616 `rf"U = {label_number(u_t.get_value(), 2)}\,\mathrm{{W/(m^{{2}}K)}}"`.
- Reviewer objects to: units in the middle of equations (general rule F00/F10). Use fractions with units on every number or only on the result.

### F23 "Temperaturen (Thermometerstände) sind visuell nicht unterschiedlich"

- Video: heating. Chapter: Heating_02_Conduction. Scene: `Beat1_MakroUndMikro` (HC class 359). EN≈ 59:10 (drift about +6:00; the scene starts at 59:35).
- Two thermometers, one per side: HC 398–404 `for x, trk, color in ((-3.2, t_left, PASTEL_RED), (3.2, t_right, PASTEL_BLUE)): th = thermometer_glyph(np.array([x, -0.7, 0.0]), height=1.75, color=color, level=trk.get_value() / 40.0)`. The levels follow `t_left` and `t_right` trackers, so the columns look the same when both trackers are close.
- Also HF 1238 and 1240 (`Text("20 °C")`, `Text("0 °C")` in Beat7) and the temperature narration HF 1160–1170 (drop across a layer). These are labels, not thermometers, and fall at EN≈ 55:30 (drift +6:00 → 1:01 would be later).
- Reviewer objects to: the two thermometer readings do not differ visually. Use different heights/colours of the columns (the drop must be visible), and make the 20 °C and 0 °C readings explicit.

### F24 "— dies ist die Temperaturdifferenz Δθ"

- Video: heating. Chapter: Heating_02_Conduction. Scene: `Beat3_UWertUndGradient` (HC class 653). EN≈ 1:01:05 (drift about +5:30).
- Narration `("profile", …)` HC 670–672: EN 671 "Through the wall, temperature drops from indoors to outdoors — that gap is delta theta." DE 672: "Durch die Wand fällt die Temperatur von innen nach außen — diese Lücke ist Delta-Theta."
- Reviewer objects to: the sentence is a dash-ended fragment in the German, and the spoken term is "Delta-Theta". Fix: "… — diese Lücke ist die Temperaturdifferenz Δθ." (spoken form "Temperaturdifferenz Δθ"). See F11.

### F25 Wärmebrücken derivation overloads students; "U = 0,24 W/m²K" explicit; clean up

- Video: heating. Chapter: Heating_02_Conduction. Scene: `Beat6_Waermebruecken` (HC class 1179). EN≈ 1:04:20 (drift about +6:10).
- The U-value tag: HC 1232 `u_tag = math_label(rf"U = {label_number(U_INSULATED, 2)}\,\mathrm{{W/(m^{{2}}K)}}", O + np.array([2.0, 1.2, 0.0]), ...)`. `U_INSULATED = _u_value(WALLS[1][1])` (HC 122) equals 0.24 for the 12 cm insulated wall. The label shows "U = 0.24 W/(m^2K)" (German format "0,24").
- Derivation steps in the scene: balcony narration HC 1198–1199, `("methods", …)` 1200–1201 (Ψ·l for corners and balcony, "rund 2,1 W/K"), `("pauschal", …)` 1202–1204 (ΔU_WB 0.10 / 0.05). Constants: HC 137–138 (`CORNER_D, CORNER_LEG, CORNER_H, PSI_E`, `BALCONY_PSI, BALCONY_L`), HC 1266 `real = hu["e"] + PSI_E * CORNER_H`, HC 1389 `chi_read = ... \Psi \cdot l ...`, HC 1424–1426 (table heads "without thermal bridges", "+ Σ Ψ_k l_k", "detailliert", "pauschal, ohne Nachweis", "pauschal, DIN 4108 Bbl. 2").
- Reviewer objects to: (1) the bridge derivation is too detailed for students (prefer the flat-rate ΔU_WB route, detailed Ψ route as an aside); (2) "U = 0,24 W/m²K" must be explicit in the text, not only as a small tag; (3) clean up the scene (overlapping labels around the corner detail, HC 1232 and 1215).

### F26 "Bitte Symbole auf DIN 4108-6 abstimmen" (screenshot lost)

- Video: heating. Best time match: `Beat3_GWert` (HS class 603) and HS Beat4/5 border, EN≈ 1:11:05 (drift +7:40). Also `Scene2` (HCa 418) for the DIN 4108-6 degree-day symbols.
- Symbols in HS: `g` (HS 701–713 g_sym and panel `("g", "g", COLOR_GVAL)`), `F_f` (HS 713 `("ff", r"F_{\mathrm{f}}")`), `F_sh` (HS 957 `F_{\mathrm{sh}} = ...`, 1066, 1117, 1139), `G` (`("g_irr", "G")`), `A` (`("a", "A")`), `Φ` (`("phi", r"\Phi")`). These are not all the DIN 4108-6 symbols; the DIN uses `g⊥` and `F_C`.
- DIN source reference: HS 883 `din = _din_ref("DIN V 18599-2 · DIN 4108-2")` (Beat5) and HCa 430–431 "after DIN 4108-6 and DIN V 18599-2" (narration), HCa 630 (label "Gradtagzahl nach DIN 4108-6 / DIN V 18599-2").
- Reviewer objects to: the symbol set differs from DIN 4108-6 (for example `g` vs `g⊥`, `F_sh` vs `F_C`, `G_t`, `H_T`, `H_V`). Use the DIN symbols, consistently in the narration, panels and labels.

### F27 "sehr detailliert, kann einfacher dargestellt werden; g fehlplatziert" (screenshot lost)

- Video: heating. Scene: `Beat3_GWert` (HS 603) and `Beat8_Hauptgleichung` (HS 1080). EN≈ 1:11:30 (drift +7:40).
- "g fehlplatziert": the `g` factor is placed as the last term before the unit in the g-value panel (HS 713 `("g", "g", COLOR_GVAL), (None, r"\;[\mathrm{W}]")`), and in the final equation it appears between `F_f` and `F_sh` (HS 1117 list `("gval", "g", COLOR_GVAL), ("fsh", r"F_{\mathrm{sh}}", COLOR_FSH)`; HS 1139 `("fsh", r"F_{\mathrm{sh}}", COLOR_FSH), (None, r"\;[\mathrm{W}]")` with `g` in the row above).
- Detail: the many intermediate panels (HS 1066 and 1139 both with `g · F_sh` and the `\textcolor` readout at HS 1169) carry the full product. The reviewer wants a simpler layout: Φ = G · A · g · F_f · F_sh, with g placed directly after A.

### F28 "Fsh muss FC nach DIN 4108-6 sein. Inkonsistenz der DIN?" (1:04:50)

- Video: heating. Scene: `Beat5_Verschattung` (HS class 853). EN≈ 1:12:30 (drift +7:40; scene starts 1:12:39).
- Symbol definitions: HS 872 `("fsh", …)` in the first panel, HS 950 `fsh = ValueTracker(1.0)`, HS 957 `F_{\mathrm{sh}}` label, HS 1066 `("fsh", r"F_{\mathrm{sh}}", COLOR_G)`, HS 1139 `("fsh", r"F_{\mathrm{sh}}", COLOR_FSH)`, HS 1184 init.
- DIN reference: HS 883 `din = _din_ref("DIN V 18599-2 · DIN 4108-2")`.
- Reviewer objects to: the symbol F_sh should be F_C as in DIN 4108-6 (Sonnenschutzfaktor / Abminderungsfaktor). The DIN itself is not inconsistent: DIN 4108-6 uses F_C for the shading factor, so the repo symbol F_sh should be renamed to F_C (and the DIN reference should name the table).

### F29 "die GTZ braucht einen (auch visuellen) Bezug zu einer Innenraumtemperatur"

- Video: heating. Chapter: Heating_06_FinalCalculation. Scene: `Scene2` (HCa class 418). EN≈ 1:16:10 (drift +7:40; scene start 1:16:09.87).
- Narration `("gradtag", …)` HCa 427–428: EN 428 "The degree-day total G t is the area under the daily indoor-outdoor difference over the heating season, about 3,500 kelvin-days per year." DE 428 "Die Gradtagzahl G-t ist die Fläche unter der täglichen Differenz innen minus außen über die Heizperiode: rund 3 500 Kelvintage pro Jahr."
- Visual: thermometer glyph HCa 574 (`thermo = thermometer_glyph(np.array([6.15, -1.45, 0.0]), height=2.0, color=PURPLE, level=0.0)`), column driven by the daily profile (HCa 575–576). The column shows only the profile, not an indoor reference temperature.
- Label: HCa 630 `unit_text = _label(tr("Annual heat loss in kWh/a — degree days per DIN 4108-6 / DIN V 18599-2", "Jahres-Wärmeverlust in kWh/a — Gradtagzahl nach DIN 4108-6 / DIN V 18599-2"))`.
- Reviewer objects to: the GTZ (degree-day number) is defined with no visible reference to the indoor temperature (base temperature, typically 15 °C in DIN 4108-6). Add the indoor reference line (for example 20 °C or the base temperature) to the chart, and name the base temperature in the narration and label.

## 3. Global consequences (F00, F01, F03, F06, F10/F22, F11, F12)

- Units: F00 "[W]" applies to every math panel in all energy videos, including Cooling and Demand (see F00 list).
- References (F01, F03, F12): the bibliography in PH 3161–3166 is the only book list in the physics video; the other scenes cite DIN labels only (`_din_ref` calls, e.g. VNat 338, 582, 802, 1002, 1231, 1442; HC 383, 1216; HF 477; HCa 630). Each must name a findable edition, chapter or keyword.
- Symbol Δθ (F11): spelled "Delta-Theta" appears in narration in VH, VHo, HC and HCa (list in F11).
- Roadmap (F06): Beat3_Fahrplan of ventilation must move to the start of the video; the same applies to the roadmap in the other energy videos (not inspected here).

## 4. Confidence summary

| Item | Scene confidence | Basis |
|---|---|---|
| F00, F04, F05, F07, F08, F09, F10, F11, F19, F20, F21, F24, F25, F28, F29 | high | exact narration or mobject text plus drift-consistent timing |
| F01, F02, F06, F12, F13, F14, F22, F23, F26, F27 | medium-high | exact text, drift-consistent timing |
| F03 | low-medium (which tag) | three equal tags, timing favours PH 2243 |
| F15, F16, F17 | low-medium | content and drift disagree or candidates overlap |
| F18 | high (scene), medium (which "…spalte") | handoff timing and "Winter / Sommer" column |

No source file was edited. Only this report was written, inside the ticket folder.
