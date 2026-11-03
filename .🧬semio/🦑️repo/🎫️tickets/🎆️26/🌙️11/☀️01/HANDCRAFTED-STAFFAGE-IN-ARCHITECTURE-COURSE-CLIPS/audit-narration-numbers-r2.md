# R2 NN: narration device numbers vs catalog

Verdicts per claim (en and de share one verdict): justified 10, match 27.

Scope: energy clip modules (`🎓️teaching/🏛️architecture/⚡️energy/**/🎬️clip/**/🐍️.py`, speech modules included): 21937 non-docstring string constants enumerated, 37 device quantities audited in the narration and on-screen strings (en and de rows where both exist). Docstrings and comments are not narration and are not audited.
Catalog values come from the cast manifest `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` (cast to asset) and the catalog assets `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/**/🔣️.json`, computed by `T/🧰️nn/audit_narration_numbers.py`.
Device heat = sum of region `operating.heatW` at level 1 plus emitter air heat V rho c_p dT (rho c_p 1206 J/m3K). Person heat = activityMet x metWM2 (58.15) x skinAreaM2. Verdict: match within 5 %, minor deviation within 15 %, mismatch beyond.

## Claims

| # | clip file:line | claim | language | asset value (source) | verdict |
|---|---|---|---|---|---|
| C1 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:248 | seated adult about 120 W | en | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C1 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:249 | seated adult about 120 W | de | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C2a | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:254 | asleep about 70 W | en | 70.91 (sleeping-person pose 0, 0.7 MET; manifest: cooling-internal-gains-sleeper) | match (-1 %) |
| C2a | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:255 | asleep about 70 W | de | 70.91 (sleeping-person pose 0, 0.7 MET; manifest: cooling-internal-gains-sleeper) | match (-1 %) |
| C2b | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:254 | at the desk 120 W | en | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C2b | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:255 | at the desk 120 W | de | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C2c | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:254 | walking 300 W | en | 286.4 (walking-commuter pose 2, 2.6 MET; manifest: cooling-internal-gains-walker) | match (+5 %) |
| C2c | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:255 | walking 300 W | de | 286.4 (walking-commuter pose 2, 2.6 MET; manifest: cooling-internal-gains-walker) | match (+5 %) |
| C2d | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:254 | sport up to 900 W | en | 895 (jogger pose 0, 8 MET; manifest: cooling-internal-gains-jogger) | match (+1 %) |
| C2d | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:255 | sport up to 900 W | de | 895 (jogger pose 0, 8 MET; manifest: cooling-internal-gains-jogger) | match (+1 %) |
| C3 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:263 | per person at desk 120 W | en | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C3 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:264 | per person at desk 120 W | de | 123.9 (seated-office-worker pose 0, 1.1 MET; manifest: cooling-internal-gains-seated-worker) | match (-3 %) |
| C4 | - | on-screen label single person 120 W | en | no English string in source | n/a |
| C4 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:480 | on-screen label single person 120 W | de | 116.9 (standing-adult pose 0 (cast cooling-internal-gains-standing); manifest: cooling-internal-gains-standing) | match (+3 %) |
| C5 | 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:352 | load 80 W, two thirds of seated full output | en | 82.6 (2/3 of seated-office-worker pose 0, 1.1 MET; manifest: heating-internal-gains-office-worker) | match (-3 %) |
| C5 | 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:353 | load 80 W, two thirds of seated full output | de | 82.6 (2/3 of seated-office-worker pose 0, 1.1 MET; manifest: heating-internal-gains-office-worker) | match (-3 %) |
| C6a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2699 | resting person 70 + 30 = 100 W | en | 116.9 (standing-adult (shared-person-winter) pose 0, 1.2 MET; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about (70 W sensible + 30 W latent); the standing-adult asset runs 1.2 MET, the 100 W figure is the VDI 2078 resting value |
| C6a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2700 | resting person 70 + 30 = 100 W | de | 116.9 (standing-adult (shared-person-winter) pose 0, 1.2 MET; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about (70 W sensible + 30 W latent); the standing-adult asset runs 1.2 MET, the 100 W figure is the VDI 2078 resting value |
| C6b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2699 | 45 g/h from 30 W latent | en | 47.85 (derived 30 W / 2.257 MJ/kg x 3600 s; manifest: shared-person-winter) | justified (-6 %): derived from the stated 30 W latent, stated with about |
| C6b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2700 | 45 g/h from 30 W latent | de | 47.85 (derived 30 W / 2.257 MJ/kg x 3600 s; manifest: shared-person-winter) | justified (-6 %): derived from the stated 30 W latent, stated with about |
| C7 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1337 | resting person 100 W | en | 116.9 (standing-adult pose 0, 1.2 MET; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C7 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1338 | resting person 100 W | de | 116.9 (standing-adult pose 0, 1.2 MET; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C8 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1325 | lamp draws 60 W electric | en | 60 (pendant-lamp rated 60 W; manifest: none in beat (pendant-lamp asset)) | match (+0 %) |
| C8 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1326 | lamp draws 60 W electric | de | 60 (pendant-lamp rated 60 W; manifest: none in beat (pendant-lamp asset)) | match (+0 %) |
| C9 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1334 | all 60 W become room heat | en | 60 (pendant-lamp rated 60 W; manifest: none in beat (pendant-lamp asset)) | match (+0 %) |
| C9 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1335 | all 60 W become room heat | de | 60 (pendant-lamp rated 60 W; manifest: none in beat (pendant-lamp asset)) | match (+0 %) |
| C10 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:571 | vacuum cleaner 700 W | en | 699.7 (vacuum-cleaner regions 157 W + slot air 543 W; manifest: physics-fundamentals-vacuum) | match (+0 %) |
| C10 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:572 | vacuum cleaner 700 W | de | 699.7 (vacuum-cleaner regions 157 W + slot air 543 W; manifest: physics-fundamentals-vacuum) | match (+0 %) |
| C11 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:574 | kettle 2 kW | en | 1990 (electric-kettle regions; manifest: physics-fundamentals-kettle) | match (+1 %) |
| C11 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:575 | kettle 2 kW | de | 1990 (electric-kettle regions; manifest: physics-fundamentals-kettle) | match (+1 %) |
| C12 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:577 | shower heater 20 kW (1 kWh in 3 min) | en | no catalog value: no heater in the catalog; shower-head is a hand shower (12 L/min) | justified: no heater in the catalog; 1 kWh in 3 min is arithmetic and 18 to 24 kW is the real instantaneous heater range |
| C12 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:578 | shower heater 20 kW (1 kWh in 3 min) | de | no catalog value: no heater in the catalog; shower-head is a hand shower (12 L/min) | justified: no heater in the catalog; 1 kWh in 3 min is arithmetic and 18 to 24 kW is the real instantaneous heater range |
| C13a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:905 | 30 L in about 3 min | en | 2.5 (shower-head 12 L/min; manifest: none in beat (shower-head asset, cast in Beat1)) | justified (+20 %): the beat states a throttled 10 L/min (30 L in about 3 min, 1 kWh = 20 kW); the head's 12 L/min is its maximum and the beat draws no flow rate |
| C13a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:906 | 30 L in about 3 min | de | 2.5 (shower-head 12 L/min; manifest: none in beat (shower-head asset, cast in Beat1)) | justified (+20 %): the beat states a throttled 10 L/min (30 L in about 3 min, 1 kWh = 20 kW); the head's 12 L/min is its maximum and the beat draws no flow rate |
| C13b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:905 | shower water 38 °C | en | 38 (shower-head summary; manifest: none in beat (shower-head asset)) | match (+0 %) |
| C13b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:906 | shower water 38 °C | de | 38 (shower-head summary; manifest: none in beat (shower-head asset)) | match (+0 %) |
| C14 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:908 | 30 kg x 4.19 kJ/kgK x 28 K | en | no catalog value: arithmetic only: 3516 kJ, no catalog value | justified: pure arithmetic, 30 kg x 4.19 x 28 K = 3516 kJ, stated as about 3520 |
| C14 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:909 | 30 kg x 4.19 kJ/kgK x 28 K | de | no catalog value: arithmetic only: 3516 kJ, no catalog value | justified: pure arithmetic, 30 kg x 4.19 x 28 K = 3516 kJ, stated as about 3520 |
| C15 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2672 | 1 kg water 0 to 100 °C 419 kJ | en | 419 (cooking-pot water 1.0 kg, material c; manifest: physics-fundamentals-pot) | match (+0 %) |
| C15 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2673 | 1 kg water 0 to 100 °C 419 kJ | de | 419 (cooking-pot water 1.0 kg, material c; manifest: physics-fundamentals-pot) | match (+0 %) |
| C16 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2675 | boiling point 100 °C | en | 100 (cooking-pot phaseChange pointC; manifest: physics-fundamentals-pot) | match (+0 %) |
| C16 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2676 | boiling point 100 °C | de | 100 (cooking-pot phaseChange pointC; manifest: physics-fundamentals-pot) | match (+0 %) |
| C17 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2678 | latent heat 2257 kJ/kg | en | 2257 (cooking-pot latentHeatJKg; manifest: physics-fundamentals-pot) | match (+0 %) |
| C17 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2679 | latent heat 2257 kJ/kg | de | 2257 (cooking-pot latentHeatJKg; manifest: physics-fundamentals-pot) | match (+0 %) |
| C18 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:926 | tealight energy 0.15 kWh | en | 0.1564 (12.8 g paraffin x 42 to 46 MJ/kg (mid); manifest: none in beat (tealight asset)) | match (-4 %) |
| C18 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:927 | tealight energy 0.15 kWh | de | 0.1564 (12.8 g paraffin x 42 to 46 MJ/kg (mid); manifest: none in beat (tealight asset)) | match (-4 %) |
| C19a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:595 | tealight 40 W | en | 40 (tealight regions 40 W (summary 35 W); manifest: none in beat (tealight asset)) | match (+0 %) |
| C19a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:596 | tealight 40 W | de | 40 (tealight regions 40 W (summary 35 W); manifest: none in beat (tealight asset)) | match (+0 %) |
| C19b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:595 | resting person 100 W | en | 116.9 (standing-adult pose 0, 1.2 MET; manifest: none in beat (standing-adult asset)) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C19b | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:596 | resting person 100 W | de | 116.9 (standing-adult pose 0, 1.2 MET; manifest: none in beat (standing-adult asset)) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C19c | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:595 | vacuum cleaner 700 W | en | 699.7 (vacuum-cleaner 700 W; manifest: physics-fundamentals-vacuum) | match (+0 %) |
| C19c | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:596 | vacuum cleaner 700 W | de | 699.7 (vacuum-cleaner 700 W; manifest: physics-fundamentals-vacuum) | match (+0 %) |
| C20 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2306 | 2.7 kWh = almost 4 h vacuuming | en | 3.859 (vacuum-cleaner 700 W: 2.7 kWh = 3.9 h; manifest: none in beat (vacuum-cleaner asset)) | match (+4 %) |
| C20 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2307 | 2.7 kWh = almost 4 h vacuuming | de | 3.859 (vacuum-cleaner 700 W: 2.7 kWh = 3.9 h; manifest: none in beat (vacuum-cleaner asset)) | match (+4 %) |
| C21 | 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1544 | full-power electric heater 2000 W | en | 2001 (electric-fan-heater stages 1000 + 1000 W; manifest: none in beat (electric-fan-heater asset)) | match (-0 %) |
| C21 | 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1545 | full-power electric heater 2000 W | de | 2001 (electric-fan-heater stages 1000 + 1000 W; manifest: none in beat (electric-fan-heater asset)) | match (-0 %) |
| C22a | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:837 | tealight 40 W | en | 40 (tealight regions 40 W; manifest: heating-conduction-tealight) | match (+0 %) |
| C22a | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:838 | tealight 40 W | de | 40 (tealight regions 40 W; manifest: heating-conduction-tealight) | match (+0 %) |
| C22b | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:837 | resting person 100 W | en | 116.9 (standing-adult pose 0; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C22b | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:838 | resting person 100 W | de | 116.9 (standing-adult pose 0; manifest: shared-person-winter) | justified (-14 %): textbook rounding of the resting person, stated with about |
| C22c | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:837 | toaster 900 W | en | 901.6 (toaster regions 355 W + slot air 546 W; manifest: shared-toaster-winter) | match (-0 %) |
| C22c | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:838 | toaster 900 W | de | 901.6 (toaster regions 355 W + slot air 546 W; manifest: shared-toaster-winter) | match (-0 %) |
| C23 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:227 | resting person 30 W latent | en | no catalog value: no latent split stored in the person asset (total 117 W) | justified: the person asset stores the total only; 30 W latent is the textbook split of the resting person and consistent with the physics clip |
| C23 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:228 | resting person 30 W latent | de | no catalog value: no latent split stored in the person asset (total 117 W) | justified: the person asset stores the total only; 30 W latent is the textbook split of the resting person and consistent with the physics clip |
| C24 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:236 | 30 m³/h air per person | en | no catalog value: no air-demand value in the person asset | justified: planning figure of DIN 1946-6 type, not a device property |
| C24 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:237 | 30 m³/h air per person | de | no catalog value: no air-demand value in the person asset | justified: planning figure of DIN 1946-6 type, not a device property |
| C25 | 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:550 | radiator 600 W (scene constant P_RAD) | en | 618 (panel-radiator night pose, 51.5 % of the rated 1.2 kW; manifest: ventilation-natural-radiator-on (pose night)) | match (-3 %) |
| C25 | 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:551 | radiator 600 W (scene constant P_RAD) | de | 618 (panel-radiator night pose, 51.5 % of the rated 1.2 kW; manifest: ventilation-natural-radiator-on (pose night)) | match (-3 %) |
| C26 | 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1293 | storage standby loss 1.8 kWh/day | en | 1.824 (hot-water-cylinder standby 76 W; the calc clip draws no tank, mapped to the tank asset of cast shared-tank-charge-up; manifest: shared-tank-charge-up (system-losses clip)) | match (-1 %) |
| C26 | 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1294 | storage standby loss 1.8 kWh/day | de | 1.824 (hot-water-cylinder standby 76 W; the calc clip draws no tank, mapped to the tank asset of cast shared-tank-charge-up; manifest: shared-tank-charge-up (system-losses clip)) | match (-1 %) |
| C27 | 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:903 | hygiene airflow 0.061 m³/s | en | 0.061 (mvhr-unit 0.061 m³/s per stream; manifest: ventilation-mechanical-mvhr) | match (+0 %) |
| C27 | 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:904 | hygiene airflow 0.061 m³/s | de | 0.061 (mvhr-unit 0.061 m³/s per stream; manifest: ventilation-mechanical-mvhr) | match (+0 %) |
| C28 | ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:395 | coil air drop 13 K | en | 13.3 (split-ac-indoor-unit air rise 13.3 K; manifest: cooling-heating-comparison-cooler) | match (-2 %) |
| C28 | ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:396 | coil air drop 13 K | de | 13.3 (split-ac-indoor-unit air rise 13.3 K; manifest: cooling-heating-comparison-cooler) | match (-2 %) |

## En and de parity

Numbers of each sentence in order; word numbers converted.

| # | clip file:line | en numbers | de numbers | same |
|---|---|---|---|---|
| C1 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:249 | [120] | [120] | yes |
| C2a | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:255 | [70, 120, 300, 900] | [70, 120, 300, 900] | yes |
| C3 | ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:264 | [120] | [120] | yes |
| C5 | 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:353 | [80, 2] | [80, 2] | yes |
| C6a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2700 | [70, 30, 45.0] | [70.0, 30.0, 45.0] | yes |
| C7 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1338 | [100] | [100.0] | yes |
| C8 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1326 | [60] | [60.0] | yes |
| C9 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1335 | [60] | [60.0] | yes |
| C10 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:572 | [0.7, 1.4, 1] | [0.7, 1.4, 1.0] | yes |
| C11 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:575 | [2, 3] | [2.0, 3] | yes |
| C12 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:578 | [1, 3, 20, 18.0, 24.0] | [1.0, 3, 20.0, 18.0, 24.0] | yes |
| C13a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:906 | [3, 30, 10, 38] | [3, 30.0, 10.0, 38.0] | yes |
| C14 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:909 | [3520.0] | [3520.0] | yes |
| C15 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2673 | [1, 0, 100, 419.0] | [1.0, 0.0, 100.0, 419.0] | yes |
| C16 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2676 | [100] | [100.0] | yes |
| C17 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2679 | [2257.0, 5] | [2257.0, 5] | yes |
| C18 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:927 | [0.15, 3000] | [0.15, 3000.0] | yes |
| C19a | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:596 | [40, 100, 700] | [40.0, 100.0, 700.0] | yes |
| C20 | 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2307 | [20, 2.7, 4] | [20.0, 2.7, 4] | yes |
| C21 | 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1545 | [2000] | [2000] | yes |
| C22a | 🔥️heating/🎬️clip/🧱️conduction/🐍️.py:838 | [40.0, 100.0, 900.0] | [40.0, 100.0, 900.0] | yes |
| C23 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:228 | [30.0] | [30.0] | yes |
| C24 | 🌬️ventilation/🎬️clip/❓️need/🐍️.py:237 | [30.0] | [30.0] | yes |
| C25 | 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:551 | [600.0, 300.0] | [600.0, 300.0] | yes |
| C26 | 🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1294 | [1.8, 200.0, 360.0] | [1.8, 200.0, 360.0] | yes |
| C27 | 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:904 | [0.061] | [0.061] | yes |
| C28 | ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:396 | [32, 19] | [32.0, 19.0] | yes |

## Coverage: unit-bearing strings not audited

263 further non-docstring strings carry a number with a unit but are building, air or device-free (heat-loss, ventilation volume, U values, kWh of buildings, labels, code). Counts per clip file:

| clip file | strings |
|---|---|
| ❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py | 1 |
| ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py | 2 |
| ❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py | 1 |
| ❄️cooling/🎬️clip/👥️internal-gains/🐍️.py | 5 |
| ❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py | 8 |
| ❄️cooling/🎬️clip/📐️system-sizing/🐍️.py | 5 |
| 🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py | 18 |
| 🌬️ventilation/🎬️clip/❓️need/🐍️.py | 2 |
| 🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py | 19 |
| 🌬️ventilation/🎬️clip/📦️volume/🐍️.py | 16 |
| 🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py | 11 |
| 🌬️ventilation/🎬️clip/🪟️natural/🐍️.py | 27 |
| 📊️demand/🎬️clip/♨️system-losses/🐍️.py | 15 |
| 📊️demand/🎬️clip/⚖️regulations/🏗️composition/🐍️.py | 1 |
| 📊️demand/🎬️clip/⚖️regulations/🐍️.py | 5 |
| 📊️demand/🎬️clip/⚡️demand-power/🐍️.py | 6 |
| 📊️demand/🎬️clip/🏭️primary-energy/🐍️.py | 2 |
| 📊️demand/🎬️clip/🏷️certificate/🐍️.py | 1 |
| 📊️demand/🎬️clip/🔁️recap/🐍️.py | 15 |
| 📊️demand/🎬️clip/🧪️practice/🐍️.py | 3 |
| 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py | 2 |
| 🔥️heating/🎬️clip/🌬️convection/🐍️.py | 3 |
| 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py | 4 |
| 🔥️heating/🎬️clip/🧮️calculation/🐍️.py | 20 |
| 🔥️heating/🎬️clip/🧱️conduction/🏗️composition/🐍️.py | 2 |
| 🔥️heating/🎬️clip/🧱️conduction/🐍️.py | 22 |
| 🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py | 12 |
| 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py | 35 |
