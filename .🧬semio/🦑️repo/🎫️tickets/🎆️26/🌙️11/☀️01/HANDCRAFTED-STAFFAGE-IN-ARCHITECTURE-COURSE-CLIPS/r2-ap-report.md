# r2 ap: OI-34, 48, 113, 114, 134 (+ coordinator follow-ups)

Scripts: `🧰️ap/` (`oi34_radiator.py` one-shot JSON rewrite, `heal.py`, `digests.ts`, `diffdig.py`, `repin.py`, `check.sh`, `raster.py`). The radiator rewrite used a script (byte-exact JSON writer of agent pa); the other assets were edited with Edit.

## OI-34 panel-radiator (wall removed)
- Rig: `wall` bone removed; `radiator` is the fixed root (x 0.035, y 0.11, angle 90 = same world pose as before), `valve-head` re-parented with converted local position. Wall-stub, wall-break, floor-on-wall geometry gone; brackets, supply pipe and floor line rebound to `radiator` (coordinates converted); the 1:200 drawing gained the two bracket lines and the supply pipe so all three scales share one extent.
- Region `wall-stub`, its pose entries, `masonry` material, `cut-masonry` hatches, the wall assumption: removed. Support is now anchor `bracket-top` (fixed). Texts en/de rewritten ("wall of the scene").
- Dimensions 0.71 x 0.135 m (was 0.85 x 0.285), `massKg` 26.18 (unchanged; the wall bone had 0 kg). Manifest entry and summary updated. 0.71 m is the radiator's own front height (body 0.11 to 0.71 m, supply pipe from the floor), so the front and side casts are now both 0.69 to 0.73 m high.
- Scenes: the radiator asset owns no wall, so new V helper `radiator_wall(anchor, unit, mirror)` (`V/🐍️.py`, after `section_material`) draws the cut strip; added to the radiator group in system-losses, regulations, demand-power and recap (radiators standing away from a drawn wall). Not needed: ventilation natural (against the exterior wall), heating calculation (under the window walls), physics fundamentals 2205 (wall already cropped out), front casts.
- Ventilation `test_closeup_radiator_scales_and_clears_floor`: `🔣️.cases.json` `height_m` 0.85 to 0.71 (the radiator itself, not the wall bounds). Passes; Beat2_StossOderKipp rendered, radiator sits under the sill against the wall.
- front-projection fixture: side cast width 0.3 to 0.45 m (0.135 m asset plus ground overhang; measured 0.385), height 0.69 to 0.73 m; feature text updated.
- export-performance re-pin (engine-independent sha256, all frames of two casts changed):
  - `radiator-transient/0000.svg` b2d15aef... to 907f8c2c...; 0001 7608d295 to 6ed44915; 0002 3ce79099 to c16f9aff; 0003 9227f152 to 86a5343e; 0004 8724524c to 038104bb; 0005 e1af8bbb to f0d497a3; 0006 a4fdf276 to 9af18813; 0007 5866cd50 to 6073ea43; 0008 f341e612 to 0514fc15; index json d50882b2 to f73b500b.
  - `radiator-rated/0000.svg` 12a9bb93 to f73eb1f0; index json d37e0e0a to bdbc59da.
  - Full old/new hashes: `🧰️ap/repin.py` input was `🗑️generated/ap/digests.json` (deleted); rerun `digests.ts` to regenerate.

## OI-48 evaporative-humidifier
Folder moved `🔌️appliances` to `🏗️machinery`, `asset.category` and the root manifest entry set to `machinery`. No other references (casts carry the asset id only). Catalog check passes.

## OI-113 toaster
Casing 180 W to 95 W on 0.24 m2 (about +40 K, about 60 degC in a 21 degC room); slot air 545 W to 630 W (flow 0.0044 to 0.00507 m3/s at +103 K), total stays 900 W. Texts en/de, cool-touch assumption added. Thermal raster shows the casing at the top of the colour range, consistent with about 60 degC.

## OI-114 oil-pumpjack
New fixed bone `skid` (850 kg, 7.2 m, x -1.6 y 0.3) owns the four `skid` shapes; motor 1100 kg to 250 kg; total 18130 kg unchanged; structure assumption en/de.

## OI-134 decentralised-hrv-fan
Assumption en/de: both emitters (-21 K exhaust, -5 K supply) are relative to the 21 degC room air, not the outdoor air; the emitters were already consistent (exhaust leaves at about 0 degC, supply enters at about 16 degC), so no values changed.

## Verification
- Catalog check, full run: 148 of 148 assets pass. The manifest line fails for a foreign reason: `solar-thermal-collector` has no manifest entry and machinery plans 15 but has 16 assets (not my asset).
- Library vitest: first run 4 failures (export-performance digest, front-projection, 2 transport-field jet tests unrelated); after the fixes front-projection, export-performance and cast-export pass (96 tests). The two transport-field failures remain, foreign.
- React vitest: 117 of 118 pass; `catalog-loading` fails only because the catalog has 148 assets against 147 manifest entries (the solar-thermal-collector entry, foreign).
- Python: visuals `🐍️`, scenes, sections, `staffage_only` OK (50 tests, 321 s); ventilation architecture closeup test OK.
- Renders (ql, 15 fps): ventilation natural Beat2, demand system-losses Beat2; thermal 1:50 PyMuPDF rasters of radiator, toaster, pumpjack, humidifier, HRV viewed/produced without error.

## Open
- regulations, demand-power and recap scenes were not re-rendered (only a unit-level syntax pass via the scene tests); the `radiator_wall` strip is the same footprint as the old stub.
- `solar-thermal-collector` manifest entry (foreign) blocks the manifest check and React catalog-loading.
