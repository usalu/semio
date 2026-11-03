# R2 dw report: documentation wording (meters, spans, textless, window)

Scope: `contract.md` (this ticket) and `🎓️teaching/🏛️architecture/🧍️staffage/README.md`. Only these two files were edited, with Edit. No tests were run (documentation only).

## Checked against code

| Claim | Where checked | Result |
|---|---|---|
| `gas-meter`, `electricity-meter` are device casts | manifest `V/🧍️staffage/🔣️.json:90-91` (`shared-gas-meter-run`, `shared-electricity-meter-run`); assets under `🗂️catalog/🔌️appliances/` | Confirmed |
| `hot-water-cylinder` is the building store cast | manifest `:87` (`shared-tank-charge-up`); asset `🔌️appliances/` | Confirmed |
| `solar-thermal-collector` exists | asset `🗂️catalog/🏗️machinery/solar-thermal-collector/🔣️.json`; cast `:68` | Confirmed |
| `thermometer_glyph`, `clock_glyph`, `energy_tank`, `meter` stay diagrams | `V/🐍️.py:504, 997, 1029, 1060` | Confirmed as Python helpers |
| Practice "Quelle" store is still `energy_tank` | practice `🧪️practice/🐍️.py:1142` | Confirmed (OI-04 ruled no change). The docs do not say the practice store is a cylinder. |
| Textless: no legend, labels, dimensions, scale bar, ground | exporter `🟦️.ts:132` (`annotations` all false) | Confirmed |
| `window=` crop | `V/🧍️staffage/🐍️.py:530` (`staffage`), `:654-656` (`staffage_motion`), `fitted_staffage` at `V/🐍️.py:254-255` | Confirmed. Units are cast metres. |
| One span per scene | guard `V/🧪️tests/🐍️staffage_only.py:161-177, 253-258` | Confirmed (enforced) |
| Smallest span covering the hottest cast | no code path; no check | NOT in code. Written as authoring guidance and marked as unchecked. |

## Changes

- `contract.md:14`: meters removed from the glyph list. Thermometers, clocks, abstract gauges kept as glyphs. Gas meter, electricity meter and hot-water store named as device casts.
- `contract.md:39`: textless list completed (dimension, scale bar, ground line), exporter reference added.
- `contract.md` Python API block: `window` added to `staffage` and `staffage_motion`, `fitted_staffage` signature added, with a `window` bullet.
- `contract.md` thermal span: the stale "25 K on all 85 thermal casts" replaced with the counts from the manifest (86 thermal casts: 77 at 25 K, 9 at 70 K or 78 K). The one-span-per-scene rule was added.
- `contract.md` archive: device-cast bullet with the four asset ids and the four cast ids.
- `README.md` thermal options: one-span-per-scene rule added.
- `README.md` course casts: textless, `window`, device casts, and the glyph exceptions added.

## Open

- The brief's "smallest span that covers the hottest cast" rule is not enforced. Either a check or a plain guideline needs an owner decision.
- `summary.md` still says "meters ... glyphs" (OI-74 is the coordinator's wording item; this agent did not touch it).
- OI-127 (report drift) is for the coordinator. These two documents do not cite the stale ids.
- The brief's README claim ("still describes meters as diagram glyphs") was not found in the README; it had no meter wording. Meter wording was only in `contract.md:14`.
