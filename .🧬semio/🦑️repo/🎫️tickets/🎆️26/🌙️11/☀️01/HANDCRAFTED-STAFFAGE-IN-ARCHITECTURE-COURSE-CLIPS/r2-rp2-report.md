# r2-rp2 report: physics fundamentals, second half

**Beats covered (second half of 8):** Beat5_Strahlung, Beat6_ThermischeMasse, Beat7_SensibelLatent, Beat8_Zusammenfassung. Beat8 has no staffage (not rendered). Beat1-4 belong to rp1.
File: `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`. Every staffage scene rendered at ql/15 fps and inspected frame by frame.

## Findings and fixes
1. Beat5 winter radiator (`construct`, region "winter and summer"): the side-section asset includes its own wall slab, which floated as a blue stub 0.55 units in front of the real wall; the panel was 0.68 m (not true scale), and its foot was lifted 0.12 units off the floor. Fix: `_staffage(..., unit=room["h"]/2.5, anchor=(x_l, y_f), window=(0, -0.06, 0.25, 1.0))`. The wall slab is dropped, the brackets meet the real wall face and the foot stands on the floor at true scale (0.85 m asset). The room window sill is raised from 0.22 to 0.34 of the room height (about 0.9 m, realistic) so the radiator sits below the sill. All beams, labels and Low-E marks derive from `win_lo` and `win_hi` and followed.
2. Beat7 hob and pot: the Q/T/m labels (edge right at the pot) touched the hob frame. `col_x = min(bl[0]-0.35, hob.get_left()[0]-0.3)` gives a clear gap. Thermal reads physically: cold blue water at 0 C, warm orange at 50 and 100 C, bubbles and steam during the latent phase, flame under the grate.
3. Beat7 person (idle-breathing, thermal): standing on the bar baseline, 1.72 units, sensible (ripples, orange dots) and latent (teal dots) cues fine, no label overlap. No change.
4. Beat6 vacuum (run-and-park clip): a pictogram beside a readout column (not in the 5 m x 4 m tile scale), pale in the thermal view as an unheated machine. Larger would collide with the label at the right edge. No change.
5. Beat5 external and internal blinds (dark-line, `BLIND_PORTION`): lamellae, headbox and rays read correctly, consistent style. No change.
6. Test `V/🧪️tests/🐍️scenes.py:28`: `physics._radiator` no longer exists (callers migrated), replaced by `_staffage` with the same unit rule. 11 scene tests OK.

## Tests
- `staffage_only` guard: 18 run, 6 fail, all Rule A "not allowlisted" (`_vent_unit`, `_meter`, `_meter_glyph`, `_thermo`, `_gas_meter`, `_thermometer`). Already listed in `r2-g6-report.md` as owned by g4. `_thermometer` in my file is only called from Beat2 (about 1107) and Beat4 (about 1876), which belong to rp1. Not touched.
- Visuals tests `🐍️`, `scenes`, `icons`, `sections`, `transforms`: 37 run, all OK after the test fix above.
- No catalog asset changed, so no catalog check was needed.

## Open
- `_thermometer` Rule A violation (rp1/g4 area).
