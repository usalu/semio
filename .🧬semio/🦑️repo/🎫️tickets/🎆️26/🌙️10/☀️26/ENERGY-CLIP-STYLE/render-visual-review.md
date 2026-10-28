# Physical Video Render Visual Review

Sampled only finalized top-level segment MP4s, excluding partial output files. Files were at least three seconds old and unchanged before/after decoding. The lightweight sampling process used AV and Pillow in the shared native environment, without Manim or UV. All posters and metadata are generated review artifacts under this ticket. No video was modified.

| Segment | Status | Duration (s) | Sample (s) | Source |
| --- | --- | ---: | ---: | --- |
| 1 | sampled | 13.50 | 6.83 | Demo_Intro_PhysikalischeGrundlagen.mp4 |
| 2 | sampled | 176.33 | 88.17 | Beat1_EnergieImAlltag.mp4 |
| 3 | sampled | 161.50 | 80.83 | Beat2_Leistung.mp4 |
| 4 | sampled | 102.00 | 51.00 | Beat3_Energieerhaltung.mp4 |
| 5 | sampled | 139.67 | 69.83 | Beat4_Waermepumpe.mp4 |
| 6 | sampled | 97.50 | 48.83 | Beat5_Strahlung.mp4 |
| 7 | sampled | 93.83 | 47.00 | Beat6_ThermischeMasse.mp4 |
| 8 | sampled | 159.33 | 79.67 | Beat7_SensibelLatent.mp4 |
| 9 | sampled | 81.17 | 40.67 | Beat8_Zusammenfassung.mp4 |

Visual review notes follow after inspecting the extracted posters.

## Visual Findings

All nine extracted native 640×360 draft frames were inspected individually. No tofu boxes, clipped captions, cropped formulas, or concrete styling defect were found in these samples. The common Anta chapter chip, teal subtitle, semio palette, and square caption/formula chrome remain consistent. Captions remain readable at the draft resolution, with clear margins and no collisions with the scientific diagrams.

| Segment | Middle-frame review |
| --- | --- |
| 1 — Intro | Midpoint is intentionally inside the university-name Write animation, so its last letters are partially drawn. Outer square semio frame and institute illustration are intact. This transition sample is not a completed identity-card still; the settled intro was verified separately earlier. |
| 2 — Energy in Everyday Life | Chapter/subtitle and two-line German caption are readable. The power-of-ten diagram is in a zoom/fade transition; remaining numbers and labels are recognizable, with no missing-glyph boxes. |
| 3 — Power | Powers, units, superscripts, yellow null-count labels, and two-line caption render correctly. Diagram remains inside its content area. |
| 4 — Energy Conservation | Room/lamp schematic, power labels, bars, and German caption are legible. Umlauts in Glühlampe and Wärme render correctly. |
| 5 — Heat Pump | Verdampfer, Verdichter, Verflüssiger, and Expansionsventil labels render with their intended diagram positions. The two-line COP caption is contained and resolves ß/ü/ä. |
| 6 — Radiation | Incoming waves, Low-E label, g-value, dotted heat-flow symbol, multiplication separators, units, and formula panel remain legible. Formula and two-line caption have visible vertical clearance. |
| 7 — Thermal Mass | Three-curve legend, 24-hour axis, Greek temperature symbol, clock label, and caption are readable. Scientific plot and room section remain separated. |
| 8 — Sensible/Latent Heat | Saturation curve, degree symbol, humidity units, Greek φ, percentage labels, and caption render correctly. No card/plot/caption collision is visible. |
| 9 — Summary | Semantic-color topic list and mathematical equations have consistent type hierarchy and clear row spacing. Subscripts, Δ, dotted Q, fraction, and multiplication render without tofu. Caption stays within its square frame. |

## Scope and Outcome

The sampling process completed successfully using a 512 MB reservation and one CPU, and exited with no remaining process. It imported only AV/Pillow plus Python standard-library modules, not Manim or UV. No production source, input MP4, or player was modified/opened. Persistent input: review-clips.py. Generated evidence: 🗑️generated/qa-review/segment-1-middle.png through segment-9-middle.png, contact.png, and metadata.json.

This is a nine-frame midpoint sample of completed draft segment files at sampling time, not a full motion/audio review or a claim that every frame of the final joined video was inspected. The final render may replace segment files after this independent review; metadata records the reviewed source paths, byte sizes, durations, and sample positions. No source fix is justified by the inspected frames.
