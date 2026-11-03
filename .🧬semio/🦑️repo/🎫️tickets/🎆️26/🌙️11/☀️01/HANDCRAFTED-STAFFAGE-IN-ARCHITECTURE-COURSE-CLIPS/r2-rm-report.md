# Round 2, agent rm: recap radiator, primary-energy meters, summer-strategy Beat3

Scripts in `T/🧰️rm/` (`rm_render.sh`, `rm_frames.py`, `rm_crop.py`). Generated output deleted.

## 1. Recap radiator (fixed)
- Cause: `_heater` (`📊️demand/🎬️clip/🔁️recap/🐍️.py:202`) used `shared-radiator-winter` (side view, thin sliver) plus `radiator_wall`.
- Decision: the scene is a house section like the primary-energy `_furnish` and practice `_heater`, which already use the front viewpoint, so the recap now uses `shared-radiator-front-winter` (no wall strip, anchored 0.8 u from the right wall, storey metre scale). `_heater` feeds Beat1 and Beat3 (`rad.get_top()` still valid).
- Looked at: Beat1_Bilanzgrenze (heating step): a red-orange finned panel with heat rings, reads as a warm radiator. Beat3_Gewinnseite: radiator fades in with the heating step, fine. Beat4 has no radiator or house.

## 2. Primary-energy meters (no defect, enlarged)
- Beat2 renders show the electricity meter on the Netzstrom row (frame at the "strom" step, right end of the supply line) and the gas meter on the Erdgas / Heizöl row. rr2 sampled instants after `unlane("strom")` fades it out (`🏭️primary-energy/🐍️.py:704`, at `strom_geg`), so it looked absent. Wood and ambient rows get no meter by design (`METER_CASTS`, line 231). There is no oil row of its own: "Erdgas / Heizöl" shares the gas-meter cast.
- Change: meter cast size 0.34 x 0.38 to 0.44 x 0.5 (line 638) so the small devices read at 480p. Beat1 uses the gas meter on the supply (rendered earlier by rr2, unchanged).

## 3. Summer-strategy Beat3 (nothing to fix)
- `Beat3_KomfortStrategie` is a decision flowchart (Sommertag, diamond, boxes) with a travelling dot; it has no staffage or devices. Wheel, exchanger, humidifier and heater live in Beat2 (agent sc). Rendered and looked at: clean, no clipping.

## Tests
- `staffage_only`: Ran 22, OK. `integration`: Ran 9, OK.

Open: BIM casts still unseen; Beat1 of primary-energy not re-rendered after the meter size change (same helper, size only).
