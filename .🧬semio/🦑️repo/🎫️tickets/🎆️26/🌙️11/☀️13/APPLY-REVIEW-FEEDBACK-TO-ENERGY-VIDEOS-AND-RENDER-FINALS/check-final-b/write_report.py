import os

TK = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(TK, "check-final-b", "REPORT.md")

TEXT = """# Read-only frame audit: two final German course videos

Scope: `Full_Heating_Demand_speaker_de_1080p30.mp4` (vb, 31:10) and `Full_EnergyBalance_speaker_de_1080p30.mp4` (vd, 43:13).
Method: one frame every 5 s (374 frames vb, 519 frames vd) via the imageio ffmpeg binary, 3x2 contact sheets at 640x360
(sheet n starts at n x 30 s; timestamp box top right). Heating sheets 0:00-24:20 were read directly; heating sheets 24:30-31:10 and
all energy sheets were read by separate reviewers and then the disputed frames were re-checked at full 1280x720.
Held-frame verdicts only: a frame counted as a defect is still in its final state 1 s later, or is a static layout clash.
Frames of counters or fades were classed as transient and are listed so the reader can see the decision.

## Heating demand (vb)

| Timestamp | What was seen | Verdict |
|---|---|---|
| 0:00 | UI overview; the cards show real rendered frames of the course scenes (Grundlagen, Lueftung, Heizen, Kuehlen, Energiebilanz) | real frames (confirmation) |
| 0:05 | "Der Fahrplan des Videos", six-step roadmap | ok (confirmation) |
| 1:00-1:10 | Scene fades in, blank wall outline | transient |
| 2:20-2:25 | "Beispiel-Wintertag / aussen 0 C" label is touched by the "Strahlung" chip below it | real (minor overlap) |
| 4:15-4:25 | Spectrum panel is translucent: "kalter Nachthimmel" and "Beispiel-Wintertag / aussen 0 C" show through it (full-res 4:25) | real (text under panel) |
| 9:15 | "U = 1,43 W/m2K" and "= 2059 W" (thumbnail had read "143") | ok, not a defect |
| 12:20-12:30 | "1m" width label of the cube sits on the top border of the formula box below | real (minor touch) |
| 13:15-14:05 | Circle multipliers "3x" (toaster) and "14x" (candle) contradict the ratio labels below them "=0,8x Toaster", "=1,2x Mensch", "=1,4x Teelicht" (14:05 crop) | real (inconsistent numbers) |
| 15:20-15:30 | Thermal-bridge scene shows "U = 0,24 W/m2K" explicitly | ok (confirmation) |
| 16:05-16:06 | Ghost formula fragments ("-0,48 W/6m", "+0,67 W/6m = +0,112 W/mK") are left behind under and through the box "psi_i - psi_e = U * 2d = 0,24 * 0,80 m = 0,19 W/mK" (full-res 16:05 and 16:06) | real (ghost formula, not tidy) |
| 16:10-16:11 | "l = 0,8 m" then "6,1 m" while the value counts up to 8,0 m at 16:15 | transient |
| 16:25 | "(4,0 + 0,0) W/K" before the sum settles | transient (counter) |
| 17:30-17:31 | "dT = 19 K, ~1430 W" then "20 K, ~1500 W" | transient (counter) |
| 20:15-20:20 | "q_i = 2,7 W/m2" then "14,5 W/m2" (290 W / 20 m2) | transient (counter) |
| 21:40-21:55 | Solar gains: g-perp = 0,60 placed at the glazing, "50 % + 10 % = 60 %" | ok (confirmation) |
| 22:45-23:20 | Shading: "F_C = 1,00", then 0,00, 0,14, 0,15, 0,16; "F_S = 0,80" | ok (counter in between) |
| 23:50 | "dT = 0,2 K ... Q = 39 kJ" (full-res); 23:53 shows 2,0 K and 480 kJ | transient (counter) |
| 23:55-23:57 | "1 m2 * 0,133 kWh/m2 = 0,2 kWh" then "13 m2 ... 1,8 kWh" then "19 m2 ... 2,7 kWh" | transient (counter) |
| 24:00 | Thermal-mass tile: "20 m2 * 0,133 kWh/m2 = 2,7 kWh" | ok (confirmation) |
| 24:25-25:00 | Solar product line counts through 749 W and 445 W to "768 W * 0,60 * 0,70 * 0,90 * 0,80 * 1,00 = 232 W" (matches caption 232 W) | transient (factors animate) |
| 26:25-26:55 | Heating-degree-day chart labelled "theta_i = 20 C Raumtemperatur", Heizgrenze 15 C (reviewer read, not re-checked by me) | ok (reviewer) |
| 30:45 | "Q_h,s = 1,8 kWh/d * 200 d = 360 kWh/a" (comma present) | ok, not a defect |

Thermometers (10:20-10:25): 40 C vs 0 C, then 31 C vs 9 C, then 20 C vs 20 C: visibly different levels (confirmation).
F_C is used in the solar equation (24:15-24:55, "F_C"); no "F_sh" symbol appears in any heating sheet (confirmation).

## Energy balance (vd)

| Timestamp | What was seen | Verdict |
|---|---|---|
| 0:00-0:05 | Overview with real frames, then the Fahrplan (reviewer read) | ok (confirmation, reviewer) |
| 7:50 | "0,86 * 813 kWh = 698 kWh" (exact product 699, rounding) | ok |
| 10:10-10:50 | Law card "Gesetz: Gebaeudemodernisierungsgesetz / GModG (Stand 2026-07-29) - vormals GEG", fully inside the frame (full-res 10:10 and 10:50) | ok (confirmation) |
| 19:30-21:35 | Q_h label reads "approx. 11700 kWh/a" (Jahresdauerlinie) | reference value |
| 21:45-21:55 | Q_h label reads "approx. 11500 kWh/a" (full-res 21:45) | real (value differs from 11700 earlier) |
| 22:25-22:50 | "1,8 kWh/d * 200 d = 248" counting to "= 360 kWh/a" (comma present) | transient (counter) |
| 28:10-29:00 | Captions say "Gebaeudeenergiegesetz" while the card says "GModG ... vormals GEG" | real (inconsistent law name, medium) |
| 30:30 | "Ein Gebaeude, zwei Anlagen": Gaskessel Q_E 140, Waermepumpe Q_E 31; no overlap | ok |
| 31:20 | "e_P = ... 1,54 gegen 0,56" (1,54, not 154) | ok, not a defect |
| 31:25-31:30 | Red equation line overlaps the chart legend row; the bottom equation box covers the x-axis labels Nutzenergie / Endenergie / Primaerenergie (one full-res read) | real (overlap; re-check if it persists) |
| 33:25 | "1 Monate" (singular "1" with plural noun) | real (minor wording) |
| 35:35 | "100 kWh/m2a * 1,40 = 140 kWh/m2a", caption e_E = 1,40 | ok |
| 41:55 | "Q_E = 133" counting | transient |
| 42:00-42:01 | "Q_E = 140", e_E = 1,40 (full-res 42:01) | ok |

Confirmations for the energy video:
- (a) overview cards show real frames and a Fahrplan follows: reviewer-confirmed at 0:00-0:05.
- (b) reference chip never collides with the title: confirmed (both reviewers, sheets 0-86).
- (c) law card text "Gesetz: Gebaeudemodernisierungsgesetz" is inside the frame: confirmed at 10:10 and 10:50 (full-res read).
- (d) no doubled e_E formula: confirmed by both reviewers.
- (e) no duplicate certificate sheet: none seen in 0:00-43:13 (the "Ausweis" A+-H scale at 41:55 is a legend, not a second sheet).

## Real defects (summary)

Heating: 16:05-16:06 ghost formula fragments under the psi formula; 13:15-14:05 "3x"/"14x" multipliers contradict the ratio labels; 4:15-4:25 scene text bleeds through the spectrum panel; 2:20-2:25 "aussen 0 C" touched by the Strahlung chip; 12:20-12:30 "1m" label touches the formula box.

Energy: 21:45-21:55 Q_h 11500 kWh/a vs 11700 kWh/a earlier; 31:25-31:30 equation over legend and x-axis labels; 28:10-29:00 law name inconsistent (GEG vs GModG); 33:25 "1 Monate" wording.

Not defects although flagged by a reviewer: 9:15 "143" (thumbnail artefact, full-res 1,43), 31:20 "154" (full-res 1,54), 41:55 "133" and 22:25 "248" (counters), 16:25/17:30/20:15/23:50/23:55 counters (checked one to two seconds later).
"""

with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(TEXT)
print("written", OUT)
