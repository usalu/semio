# EN Physical Fundamentals: Visual Layout Audit

Video: `.🧬semio/🎓️teaching/⚡️cache/ea/final/en/spectator/rendered/Full_Physical_Fundamentals_spectator_en_1080p30.mp4`

Method: frames extracted every 15 s (15 s to 1170 s), plus 1158 s, 1162 s and 1166 s for the end. No frame could be extracted at 1174 s or later, so the video ends between 1170 s and 1174 s. Frames were checked by eye at 1280x720.

## Persistent defects

| Time (s) | Defect type | Exact text involved | Where on screen |
|---|---|---|---|
| 75 to 90 | German word left in English version | red button "An" (German for "on"), next to "Kettle" and "Shower (instantaneous heater)" | right-hand toggle button in the appliance row box, approx. x 1115 to 1190, y 185 |
| 1170 (video ends before 1174) | German text left in English version, and a different screen layout at the end | "Zusammenfassung — was wir gelernt haben"; "Willems, Schild, Stricker (2024), 8. Aufl."; "Formeln und Tabellen Bauphysik"; "Wärmeschutz – Feuchteschutz – Klima – Akustik – Brandschutz"; "Strahlung · Thermische Masse · Feuchte"; "Bohne, Bohne (2026), 13. Aufl."; "Gebäudetechnik"; "Technischer Ausbau und nachhaltige Gebäudesysteme"; "Wärmepumpe · Lüftung · Anlagentechnik"; "Normen und Richtlinien"; "Jahresarbeitszahl · g-Wert · Sonnenschutz · Wärmespeicherung" | centred summary card inside an app-window capture (window title "Architecture · Building Systems", tab "Physical Fundamentals", bottom bar "Energy" / "Architecture and Technology") |

Notes on the 1170 s defect:
- At 1166 s the same card is correct English: "Summary — what we learned", "Standards", "Heat pump · Ventilation · Building services", "Seasonal performance · g-value · Sun protection · Heat storage".
- The German card with the app-window capture first appears at 1170 s and is still showing on the last extractable frame. The video's last frame is between 1170 s and 1174 s, so the German card is on screen for at most about 4 s.
- Text on this German card does not overlap and is not clipped; the defect is the language.

## Checked and not reported

- The known "Venetian blind" / "shortwave" overlap at about 808 s (and the 810 s frame showing it) was excluded as instructed.
- Mid-animation frames (e.g. the power-of-ten zeros fading at 345 s, the "DIN EN 410:2011-04 · g⊥" line at 750 s, the "Condenser" label fading at 570 s) were treated as transitions. None persisted in later frames.
- No glyph boxes, raw LaTeX, text running off the frame edge, or other text-on-text overlaps were seen in the other frames.

## Totals

- Frames checked: 81 (78 at 15 s intervals from 15 s to 1170 s, plus 1158 s, 1162 s and 1166 s).
- Persistent defects: 2 (one German button label at 75 to 90 s, one German end card at 1170 s).
