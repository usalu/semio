# Round 2 en/de number mismatch check (agent np)

## Method
- Script: `🧰️np/np_number_check.py` (stdlib only). It walks every `🔣️.json` under `S/🗂️catalog` (153 files with the new `tower-server` and `waste-bin`) and the V manifest, and collects 17,786 bilingual `{en, de}` pairs.
- Numbers are extracted with their adjacent unit. German decimal commas are read as decimals, and German thousands dots only where the group is not leading with 0 or a sub-1000 value. Units are canonicalised (`m2`/`m²`, `degC`/`°C`/`Grad`/`degrees` to `deg`, `Watt` to `W`, and so on). Year-like numbers present in both languages are ignored.
- Pass 1 compares the value multisets. Pass 2 checks one-to-one matches for unit conflicts (for example 0.6 m vs 0.6 mm): none found.
- Each of the 49 value mismatches was read in context and judged by hand. 26 assets were edited with Edit-tool changes. The first run had 49 value mismatches, the final run 14. The 14 left are word-form equivalents or notation (see below).

## Toaster (the trigger)
- No `545` remains in `toaster/🔣️.json`. The English and German notes both use 630 W for the hot slot air.
- Sum check: 95 + 25 + 150 + 630 = 900 W, matching the 900 W rating in both languages. The air figure is consistent with 0.00507 m3/s at a 103 K rise (about 630 W).

## Fixed mismatches (verdict, fix)
Numeric fixes made with Edit. Where en was the physically consistent side, de was changed. Judgement calls are marked.

| # | Asset / pointer | Verdict | Fix |
|---|---|---|---|
| 1 | evaporative-humidifier / representations/500/reason | de dropped "0.5 mm" drops | de: "die Tropfen von 0,5 mm" |
| 2 | oil-pumpjack / asset/provenance/notes | de said 20 kW at 95 %, contradicting the 24 kW shaft load and 1.8 kW loss (24/0.93 - 24 = 1.8, en) | de parenthetical replaced by en's clause; de also gained the dropped sentence on the 5.57 m and 6.27 m heights |
| 3 | wheel-loader / 500/decisions/retained | de dropped the overall 8.6 m | de: "die Gesamtlänge von 8,6 m" |
| 4 | goat, sheep, courier-with-hand-truck, person-cooking, pregnant-person, runner-with-blade-prosthesis (thermal/0), standing-adult, standing-phone-user, bookshelf | de wrote dot decimals (1.369 m2), which read as 1369 | de decimal commas (1,369 etc.) |
| 5 | runner-with-blade-prosthesis / thermal/1 | de used 204 m/min and 44 ml/(kg min); 3.1 m/s is about 186 m/min, en 184 gives 40 | de aligned to en: 184 m/min, 0,2 x 184 + 3,5 = 40 (judgement: en kept as reference) |
| 6 | red-squirrel / thermal/2 | de dropped the 100° reaction angle | de: "(100°)" added |
| 7 | person-with-umbrella / poses/1 | de dropped the 5° lean | de: "lehnt sich um 5 Grad" |
| 8 | person-on-crutches / structure/0 | en said 0.6 kg for two crutches; the rigid body has 0.55 kg per crutch (1.1 kg) | en: 1.1 kg (de was correct) |
| 9 | seated-office-worker / 500/decisions silhouette and thinElements | de dropped 3.6 mm, 0.9 mm per 0.45 m, and the 9 mm (0.02 mm) laptop lid | de clauses added |
| 10 | tower-server / 100/decisions/thinElements | de "25 mm und 14 mm" vs en "2.5 mm and 1.4 mm" on paper; en matches the paper-width sentence (judgement) | de: 2,5 mm und 1,4 mm |
| 11 | city-bus / 500/decisions/merged | de dropped the 0.55 m door leaf (door-leaf w = 0.55 in the geometry) | de: "der 0,55 m breite Flügel mit 0,45 m Flügelweg" |
| 12 | cafe-chair / 100 thinElements (de line 1821) | de dropped the 0.35 mm pen tolerance | de: "innerhalb der 0,35-mm-Stifttoleranz" |
| 13 | dining-table / 50/decisions/retained | de dropped the leg clause (0.56 m shoulder) | de clause added |
| 14 | dining-table / symbols/0/description | de dropped 0.13 mm | de: "(0,13 mm)" added |
| 15 | dining-table / thermal/0 | de dropped the 12 mm penetration depth | de: "(12 mm)" added |
| 16 | folding-deck-chair / 100 thinElements | de dropped 0.0018 m | de: "(0,0018 m dick nur auf Papier)" added |
| 17 | height-adjustable-desk / 50 hatching | de dropped 0.025 m | de: "(0,025 m Tischplatte, ...)" added |
| 18 | park-bench / 200 thinElements | de dropped 0.04 m post | de: "Pfosten (0,04 m)" |
| 19 | three-seat-sofa / 500 thinElements | de "0,18 mm" vs en "0.28 mm" (0.14 m block at 1:500 is 0.28 mm); de dropped the 0.08 m overshoot | de: 0,28 mm and 0,08 m |
| 20 | external-venetian-blind / 50 retained | de dropped the 0.3 mm render line | de: "Putz 0,015 m als 0,3-mm-Linie" |
| 21 | glulam-beam / 100 silhouette | de dropped 5 mm proud of the soffit | de clause added |
| 22 | cooking-pot / 100 thinElements | de dropped 0.08 mm | de: "8 mm (0,08 mm)" |
| 23 | drinking-fountain / provenance notes | de dropped the 14 K / 26 degC / 4 K clause | de clause added |
| 24 | stepladder / 100 retained | de "0,10 m" vs en "0.09 m" tread depth; the geometry does not settle it (judgement: en kept) | de: 0,09 m (low confidence) |
| 25 | gas-hob / 50 silhouette | en front 6 mm x 1.8 mm is 1:50 (0.30 m); de front 15 mm x 4,4 mm is 1:20 | de front aligned to 1:50: 6 mm and 1,8 mm |
| 26 | waste-bin / 500 silhouette (another agent's new asset) | de dropped "0.1 mm-accurate" | de clause added |
| 27 | worker-carrying-box / 200 thinElements | de dropped the 26 mm line weight for 50 mm real | de clause added |

## Equivalent, not changed (remaining 14 value hits)
- forklift clips/0 and structure/1: en "1 t" vs de "die Tonne" (same figure).
- cyclist poses/0, events 0-3 titles, clips/1: en "4 o'clock" vs de "vier Uhr".
- jogger structure/0: en "2 body weights" vs de "das Doppelte des Körpergewichts".
- tram thermal/0: de "(13,9 m/s)^2" vs en "squared" (notation).
- cantilever-balcony thermal/1: en "2D" vs de "zweidimensional".
- glass-balustrade notes: en "one-metre" vs de "1 m".
- spiral-stair clips/0 phase title: en "1 step" vs de "einem Schritt"; structure/0: en "3D torsion" vs de "räumliche Torsion".

## Open (not changed)
- gas-hob plan figures "15 mm by 26 mm" (en and de alike) are 1:20; at 1:50 the plan would be about 6 x 10.4 mm. The representation key is 50. Needs an owner decision on the scale.
- 95 de dot-decimal warnings (for example "6.9 m", "0.13 mm") where the values agree. These are format only (German decimal comma convention) and were not changed. The script prints them as `WARN`.

## Catalog check
- Command: `repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>` per changed asset. The full run is too heavy here (about 10 minutes for a single asset under host load), so only the changed assets are checked.
- Changed assets (32): evaporative-humidifier, oil-pumpjack, wheel-loader, goat, sheep, courier-with-hand-truck, person-cooking, pregnant-person, runner-with-blade-prosthesis, standing-adult, standing-phone-user, bookshelf, red-squirrel, person-with-umbrella, person-on-crutches, seated-office-worker, tower-server, city-bus, cafe-chair, dining-table, folding-deck-chair, height-adjustable-desk, park-bench, three-seat-sofa, external-venetian-blind, glulam-beam, cooking-pot, drinking-fountain, stepladder, gas-hob, worker-carrying-box, waste-bin.
- Status: NOT COMPLETED. The first changed asset (evaporative-humidifier) was run once. It waited about 1160 s in `[capacity]` on host memory pressure ("behind the host itself, nothing of ours"), then exited 1 with no `✓` or `✗` lines. The other 31 changed assets were not run, because each would wait the same way under the current host load. Re-run the 32 assets above once memory is free. The catalog check still needs to pass for the changed assets.
- Also not verified: the 32 changed assets have not been rendered or exported.
