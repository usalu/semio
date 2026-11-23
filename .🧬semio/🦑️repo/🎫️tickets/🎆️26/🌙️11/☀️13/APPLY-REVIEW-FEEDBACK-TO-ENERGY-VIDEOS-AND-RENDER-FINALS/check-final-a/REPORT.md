# Frame audit: two finished German course videos (presenter versions)

Method: one frame every 5 s (fps=1/5) from each video, 3x2 contact sheets (37 per video, all viewed). Suspicious frames re-extracted at full 1280x720 and at 1 s steps. Sheet frames lose decimal commas, so comma-only readings were checked at full size before counting them.

Result: no real defects in held frames. Every flagged item was either a counter or fade mid-animation, or a comma lost in the sheet downscale.

## Physics video (Full_Physical_Fundamentals_speaker_de_1080p30.mp4, 18:12)

| Timestamp | What was seen | Verdict |
|---|---|---|
| 0:00-0:02 | UI overview: cards with real video frames (Physik, Lüftung, Heizen, Kühlen, Energiebilanz) | real (ok) |
| 0:05-0:25 | Fahrplan "Der Fahrplan des Videos", six steps, step 1 highlighted | real (ok) |
| 1:50 | P = 1,0 kW readout (sheet showed 10 kW, comma lost) | real (ok) |
| 3:06 | Kraftwerksblock 1,4 GW (sheet showed 14 GW, comma lost) | real (ok) |
| 6:00-9:55 | Heating-side formulas, COP x Q_E consistent | real (ok) |
| 10:00-11:55 | Window load: A = 1,92 m² (sheet 192 m²), 576 W consistent | real (ok) |
| 13:00-13:03 | Thermal-mass tile: Δθ counting 0,0 K to 2 K, Q counting to 480 kJ (0,7 K / 160 kJ read at 787 s is 0,67 K rounded) | transient |
| 13:02-13:04 | Area readout 3 m² with 0,5 kWh while area/kWh still counting (792 s), 18 m² = 2,5 kWh at 793 s, settles to 20 m² = 2,7 kWh from 794 s | transient |
| 13:17-13:55 | Readout "20 m² · 0,133 kWh/m² = 2,7 kWh", unit as fraction, stable, no overlap | real (ok) |
| 13:20-14:10 | Sheet-only check: DIN EN ISO 13786 chip bottom right in the thermal-mass scene, no overlap with readout | real (ok) |
| 14:00-14:25 | 480 kJ = 0,13 kWh; 1 kg from 0 to 100 °C = 419 kJ | real (ok) |
| 14:45 | 2257 kJ / 419 kJ ≈ 5,4 | real (ok) |
| 15:00-15:55 | Psychrometric chart: 4 g / 15 g / 27 g per kg at 0/20/30 °C, 50 % at 20 °C = 7 g, dew point 9,3 °C | real (ok) |
| 16:00-16:55 | Sensible/latent split 70 W / 30 W; "etwa 45 g/h" (30 W latent gives about 48 g/h, rounding only, not reported) | real (minor, not a defect) |
| 17:00-18:12 | Summary cards, formula list, reference cards (DIN chip with edition) | real (ok) |

## Cooling video (Full_Cooling_Demand_speaker_de_1080p30.mp4, 18:22)

| Timestamp | What was seen | Verdict |
|---|---|---|
| 0:00-0:02 | UI overview: cards with real video frames (Vom Heizen zum Kühlen, Interne Kühllasten, Transmission, Sommerstrahlung, Auslegung, Sommerstrategie) | real (ok) |
| 0:05-0:55 | Fahrplan, six steps | real (ok) |
| 2:50-3:00 | Q_C = Q_T + Q_V + Q_S + Q_i, subscript i consistent with later slides (checked 2:55 at full size) | real (ok) |
| 6:30-6:55 | Q_i = 1200 W + 420 W + 400 W = 2020 W written as a sum in the load table; caption subscript i (full size 6:32) | real (ok) |
| 7:10-7:35 | "Q_i ist die gesamte interne Last, die die Kühltechnik abführen muss"; "bevor die Kühltechnik sie abführen muss" | real (ok) |
| 8:30-8:55 | Opaque envelope: Q_T = U · A · Δθ_eq in W, U in W/(m²K) | real (ok) |
| 9:50 | 220 m³/h · 0,34 Wh/(m³K) · 6 K ≈ 450 W | real (ok) |
| 10:10 | Q_V = 1260 W, 1260/680 ≈ 1,9 (sheet 19, comma lost), 1940 W sum | real (ok) |
| 11:10-11:15 | I_s,max = 800 W/m² peak (11:05 still growing to 537, transient) | real (ok) |
| 11:40-11:55 | F_C = 1,00 / 0,00 / 0,15 (F_C, not F_sh) | real (ok) |
| 12:35-12:50 | A = 1,92 m², 0,70 · 0,15 · 0,60 · 800 W/m² · 1,92 m² ≈ 97 W | real (ok) |
| 13:00-13:02 | Temperature-lift chip reads 26-18 = 0 K while counting (13:05 shows 8 K) | transient |
| 13:10-13:55 | Sensible ventilation formula in kW with ρ_a ≈ 1,2 kg/m³, c ≈ 1,0 kJ/(kgK), Δθ = 8 K | real (ok) |
| 14:00-14:25 | q_vR = Q_S / (ρ · c · Δθ), arrow "umstellen nach q_vR" | real (ok) |
| 15:00-15:20 | 1510 W from 30 m² · 0,70 · 0,15 · 0,60 · 800 W/m²; 1,51 kW / (1,2 · 1,0 · 8) = 0,157 m³/s (106 W/m² at 15:05 is a counter) | real (ok) |
| 15:25-15:40 | 0,157 / 0,061 ≈ 2,6; A = 0,157 / 2,5 ≈ 0,0629 m² vs 0,0244 m² | real (ok) |
| 15:45 | zoom transition to overview card | transient |
| 16:00-17:55 | Strategy and flow charts, DEC sorption chain, subtitles in sync | real (ok) |
| 18:20 | Closing overview | real (ok) |

## Confirmations

1. Both videos open with a UI overview whose cards show real video frames (physics 0:00, cooling 0:00-0:02 full size), followed by the Fahrplan (physics 0:05, cooling 0:05-0:55). Confirmed.
2. Physics thermal-mass tiles: readout reads "20 m² · 0,133 kWh/m² = 2,7 kWh" with the unit as a fraction and no overlap with the DIN EN ISO 13786 chip (the chip sits in the other scene). Confirmed.
3. Cooling: "Kühltechnik" wording (7:10, 7:35), sum 2020 W in the load table (6:50-7:00), F_C (not F_sh) in 11:40-11:55, 800 W/m² peak (11:10). Confirmed.

## Real defects

None found in held frames.
