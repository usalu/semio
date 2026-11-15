# Visual audit: energy videos (German, 1080p speaker finals)

Videos: Full_Cooling_Demand_speaker_de_1080p30.mp4 (18:33.70, AAC stereo audio, 30 fps) and Full_EnergyBalance_speaker_de_1080p30.mp4 (43:13.43, AAC stereo audio, 30 fps). Frames: every 20 s plus 2, 6, 12 s, in the generated check-b folder; contact sheets cover all grid frames.

| Item | Check | Result | Frame file(s) | Note |
|---|---|---|---|---|
| 1 | Overview shows real rendered frames | pass | cool_0000s.png, eb_0000s.png | Cards are rendered scene frames; the Physik card is a text summary slide, not a placeholder. |
| 2 | Roadmap is first content scene | pass (note) | cool_0012s.png, eb_0020s.png, eb_0040s.png | The UI overview (about 0 to 10 s) comes before the Fahrplan; the Fahrplan is the first content scene. |
| 3 | No bare [W]; units on numbers; fractions | pass (sampled) | cool_0780s.png, cool_0940s.png, eb_1280s.png | No [unit] found. Units appear on numbers and compound units as fractions. |
| 4 | Captions at most two lines, math not raw | pass (sampled) | eb_0140s.png, eb_1880s.png | Captions are one or two lines; no raw $ or TeX visible. |
| 5a | Equation Q_h = Q_l - eta*Q_g | FAIL | eb_0140s.png | Shown as Q_h + Q_g = Q_l with no eta; the balance is wrong as drawn. |
| 5b | Symbols per DIN V 4108-6 | partial | cool_0820s.png, cool_0940s.png, eb_1580s.png | rho_a and c_p,a shown with units; cooling uses Q_V,sens. Dot notation on annual energies not verified at full resolution. |
| 5c | Primary energy values consistent | FAIL (minor) | eb_1940s.png, eb_1880s.png | Gaskessel Q_P shown as 153 kWh/(m2a) on the Grenzwert chart but 154 elsewhere and in its caption. |
| 5d | Degree days with 20 C indoor line | not seen at full res | eb_0800s (sheet only) | G_t,20/15 and the 20 C line appear on the sheet; not checked at full resolution. |
| 6a | No overlap or clipping (Vom Gesetz chapter) | FAIL | eb_0640s.png | Title overlaps the GModG header; caption box text runs off both left and right edges; Q_P label collides with the bar. |
| 6b | No overlap (Warum Rechnung chapter) | FAIL | eb_2240s.png | Title box overlaps the DIN/TS 18599-10 header text. |
| 6c | No overlap (Ein Gebaeude, zwei Anlagen) | FAIL | eb_1880s.png | Equation line runs into the header; the reference block overlaps the equation text. |
| 6d | No overlap (e_E equation chain) | FAIL | eb_1580s.png, eb_1600s (sheet only) | Symbols and values of e_E = e_ce*e_d*e_s*e_g overprint each other; the 1600 s equation shows the same clash. |
| 6e | No empty frames | partial | cool_0400s, cool_0460s (sheet only) | A few frames show only title and caption; likely fade transitions, not confirmed. |
| spec | Spectator first 20 s | pass (partial) | spec_cool_0000s.png | Same overview and layout as the speaker cut; spec_eb frames extracted but not viewed. |
