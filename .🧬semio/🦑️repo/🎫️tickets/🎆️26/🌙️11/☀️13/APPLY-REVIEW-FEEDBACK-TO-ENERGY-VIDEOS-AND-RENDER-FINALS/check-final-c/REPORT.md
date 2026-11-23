# Frame audit: Full_Ventilation_speaker_de_1080p30.mp4 (presenter version)

Read-only audit. Video: `.🧬semio/🎓️teaching/⚡️cache/ve/final/de/speaker/rendered/Full_Ventilation_speaker_de_1080p30.mp4` (about 31 min).
Method: one frame every 5 s at 1280x720 (373 frames, `check-final-c/frame.sh`), 6 frames per contact sheet at 640x360 (63 sheets, `check-final-c/sheets.py`), each sheet viewed. Suspicious frames were re-extracted at full resolution, plus frames 1-2 s later to separate transients from defects. Frames are in `🗑️generated/check-final-c/`.

## Real defects (held frames)

| Timestamp | What was seen | Verdict |
|---|---|---|
| 7:15 | Heating-element slide ("Wärmekapazität der Luft"): `Q = 1,2 kg · 1005 J/(kg K) · 0,6 K` is shown as `= 745 J` and `= 0,207 Wh`. The product is 723,6 J, i.e. about 724 J = 0,201 Wh. Wrong numbers. | real (numeric error) |
| 18:45 and 18:50 | Winter opening sketch ("Auftrieb: Höhe erzeugt Druck"): the label "Zuluftöffnung" runs across the outline of the inlet opening, and "A = 0,1 m²" sits on the opening's border. Same at both timestamps, so it is not a transition. | real (label overlap) |
| 1:15 to 1:30 | "Grenzwert nach Pettenkofer (1858)" runs into the header band of the Feuchte/Geruch columns and its text touches the top of the Geruch frame (checked at 1:20 full-res). | real but minor (borderline overlap) |

## Transients checked and not reported

| Timestamp | What was seen | Verdict |
|---|---|---|
| 0:45 | Blank frame at the start of the Pettenkofer scene | transient |
| 2:25 | Sun tile overlaps the Winter chip (transition) | transient |
| 4:25 | Fresh-air counter reads 136 m³ at t≈0,6 h while still counting | transient (counter) |
| 6:25 | "Volumenstrom" slide: boxes "4·30 = 120 < V·n = 220" and a faded second formula mid-morph; clean at 6:27 | transient |
| 8:05 | SI-unit formula ("Gebäudebezogene Einheiten" / "SI-Einheiten") mid-morph; clean at 8:07 | transient |
| 12:00 | Thumbnail of the app window appears in the top-left (zoom-out transition) | transient |
| 14:45 | Ghost "Luft 340 Wh" bar over the Kipplüftung row; clean at 14:47 | transient |
| 18:55 | Chart frame still fading in | transient |
| 20:10 | Sommer and Winter schedule chips overlap while switching; clean at 20:12 and 20:15 | transient |
| 22:30 | Faded lines during the fan-set transition | transient |
| 23:20 | Floating "17 W" counter label | transient |
| 14:55 | "≈ 2,4×" reads correctly at full-res (830 / 340 = 2,44) | not a defect |
| 15:40 | Square-root panel: about 13 px clear of the room and about 12 px above the caption (full-res) | not a defect |

## Confirmations

1. Intro: 0:00 shows the UI overview with real video frames in the cards; the "Fahrplan" (roadmap) runs 0:05 to 0:40. CONFIRMED.
2. Stack effect (natural ventilation): square-root panel clear of room and caption (15:40, full-res); `h` label clear of the Winter box; chimney (Abluftöffnung) rather than window plus exhaust (16:50 to 17:55); Δρ(θ) and v(Δp) introduced 17:00 to 17:45; chain 0,6 · 0,1 m² · 1,15 m/s = 0,069 m³/s ≈ 250 m³/h at 17:55 and 18:00. CONFIRMED.
3. Night ventilation: sun and external shading context at 19:00 to 19:25 (sun tile, "Verschattung", storage mass). CONFIRMED.
4. Mechanical chapter: "Zuluft unten" / "Abluft oben" tags (22:10 to 22:40); latent chip "Q̇_V,lat Winter 770 W Sommer 810 W" clear of the formula box (24:00, full-res); winter latent loss is a number, 770 W (24:00 and 29:00). CONFIRMED.
5. Density and units: ρ = 1,27 kg/m³ (5 °C) at 6:55; "Gebäudebezogene Einheiten" / "SI-Einheiten" slide at 8:07; "Grenzwert nach Pettenkofer (1858)" at 1:20; purge vs tilt 340 + 300 + 190 = 830 Wh (14:45 to 14:55); "Kühltechnik" wording in the handoff with the sum shown (30:35: "ihre Summe Q̇_V = 1260 W", and 450 + 810 = 1260). CONFIRMED.

## Summary

Three real defects: 7:15 wrong product (745 J / 0,207 Wh, should be 724 J / 0,201 Wh), 18:45 and 18:50 label overlap at the inlet opening, and a minor borderline label overlap at 1:15 to 1:30. Everything else checked is either a transient or consistent.
