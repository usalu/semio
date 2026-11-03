# 📝️ haiku-inventory: Audit of the 62 Unavailable Band Decisions (relayed by the coordinator)

**Method.** Paper size in mm = metres × 1000 / denominator. The minimum printable feature is 0.2 mm on office laser, 0.4 mm on draft-300 and 0.12 mm on plot.

**Result.** 57 decisions are honest, 5 are questionable, and none is missing a reason. 13 of the honest ones are borderline, with an overall paper size of 2.4–4.4 mm.

## Coordinator rule D17

An asset whose overall paper size is **at least 3 mm** at a given scale must be drawable there. It needs a handcrafted simplified band, or a true-extent or conventional symbol, in all three views. Below 3 mm it may be `unavailable` with a truthful `cause`.

`cause` and `reason` texts must be accurate:
- Never claim "below every print profile" when the feature is at or above a profile minimum.
- Never refer to a symbol that does not exist.

## Must become drawable (D17)

| Asset | Band | Overall paper mm | Owner |
|---|---|---|---|
| parent-carrying-toddler | 200 | 8.7 | humans |
| kneeling-gardener | 200 | 9.2 | humans |
| skateboarder | 200 | 8.9 | humans |
| parent-carrying-toddler | 500 | 3.5 | humans |
| kneeling-gardener | 500 | 3.7 | humans |
| skateboarder | 500 | 3.5 | humans |
| chalkboard | 500 | 8.0 | objects |
| storage-crate-stack | 500 | 3.0 | objects |
| operating-table | 500 | 4.0 | objects (furniture) |
| external-venetian-blind | 500 | 5.5 | buildings |
| hot-water-cylinder | 500 | 3.0 | buildings |
| server-rack | 500 | 4.0 | animals (appliances B) |
| vacuum-cleaner | 500 | 3.0 | animals (appliances B); resolve the dimension conflict between the reason (1.5 × 0.78) and the manifest (H 0.76 × W 1.5) |
| solar-thermal-collector | 500 | 3.4 | machines |
| scissor-lift | 500 | 4.4 | machines (the existing true-extent symbol is acceptable if it has all three views) |
| city-bicycle | 500 | 3.7 | machines (likewise) |

Below 3 mm and staying unavailable with a truthful cause: tilt-window (2.5 mm; it also has a conventional symbol), red-fox (2.35 mm), and everything smaller.

## Reasons to correct

**Overstated against office laser:** pigeon, gas-boiler, panel-radiator, room-thermostat 200, hosta-clump, kneeling-gardener 200, parent-carrying-toddler 200, scissor-lift.

**Overstated against plot only:** herring-gull, hedgehog 200, storage-crate-stack, external-venetian-blind, parent-carrying-toddler 500.

**Refer to symbols that do not exist:**
- electricity-meter ("counter symbol")
- gas-meter ("dial symbol")
- duct-fan and decentralised-hrv-fan ("airflow symbol")
- room-thermostat ("thermostat symbol")
- ceiling-diffuser ("line break")

**Dead-end alternatives (D9):**
- hedgehog at 1:500 points to red-squirrel at 1:500, which is unavailable.
- red-squirrel at 1:500 points to domestic-cat at 1:500, which is unavailable.

## Symbol semantics

All seven symbol alternatives carry `semantics: "true-extent"`, so each is a physically scaled drawing. Three are worded "conventional" (window-plan, tilt-opening-sign, balustrade-plan), which contradicts `true-extent`. Each symbol's wording must match its semantics field.
