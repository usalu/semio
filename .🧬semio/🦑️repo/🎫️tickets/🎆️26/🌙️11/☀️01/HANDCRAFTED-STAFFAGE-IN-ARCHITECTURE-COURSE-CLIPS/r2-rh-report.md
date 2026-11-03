# r2-rh: heating series staffage verification

Rendered (ql, 15 fps, de, one scene at a time) and looked at every frame sheet of all 20 staffage scenes: solar-gains Beat5; internal-gains Beat1-5; calculation ReviewingHeatGains, UltimateEnergyBalance, AnlagenVerluste; conduction Beat4, Beat5; fundamentals Beat1-5. Convection has no staffage. Scripts: `T/🧰️rh/` (render.sh, sheet.py, zoom.py, svgs.py).

## Checked and fine
- Scale: person ~0.7 of storey height in all house/room sections; desk, laptop, rack (2 m in 2.6 m room), lamp consistent. Feet on floors for person, desk, rack, boiler, tank, fundamentals radiator.
- Thermal: bodies, laptop, rack, lamp, radiators warm; envelope loss arrows cold; person, rack, radiator, lamp animate (clips/transients). Server rack is cold before power-up and warm after (only lower slots populated by asset design).
- Blind (solar Beat5, re-rendered after hs/g5 changes): fits window, head box and bottom rail aligned.

## Fixed
1. `🧱️conduction/🐍️.py` `_toaster`: the fill meter was a hard brown rectangle larger than the toaster (about 0.9 x body bounds). Now sized to the visible silhouette (0.72 w, 0.5 h, offset 0.16 h).
2. `🧱️conduction/🐍️.py` Beat5: radiator pipe stubs now sit on the floor (removed the 0.048 lift that left it floating).
3. `🧮️calculation/🐍️.py` `_plant_house`: the side-view radiator asset carries its own wall plate, which floated 0.29 units off the real wall. `RAD_X` changed (-6.12, -1.88) to (-6.41, -1.59) so the plate is flush with the wall inner face (pipes, sleeves, flow paths follow `RAD_X`).
4. Same function: thermostat was floating mid-room and 3x oversized; now wall-mounted (0.11 high at (-6.42, 0.5)), label placed with `next_to`.
5. `🧲️fundamentals/🐍️.py`: `RADIATOR` box described the old 0.32 m drawing, not the 1.15 x 0.71 m asset. Corrected, so warm ripples, air-particle exclusion zone and rays start at the real radiator edge. The convection "glow" was a hard red rectangle drawn over the radiator; now a rounded halo behind it (z_index -1).

## Tests
- Heating clip tests plus `scenes`: `authoring.🎬️clip.🧪️tests` + `visuals.🧪️tests.scenes`: 42 tests OK (1 skipped).
- `visuals.🧪️tests.staffage_only`: 18 run, 5 failures, none in heating: cooling `📐️system-sizing::_vent_unit`, demand `primary-energy::_meter_glyph`, demand `practice::_thermo` and `::_gas_meter`, physics `fundamentals::_thermometer` (primitive-built objects, other agents' areas, not in the allow list).

## Open
- Conduction Beat4 comparison strip: toaster (0.74) and person (1.3) are icon-scaled, not to true scale relative to each other (deliberate comparison icons).
- Thermostat label in AnlagenVerluste sits tight against the radiator plate.
- Transient crossing text during the Q_Gewinn transform in ReviewingHeatGains (about 1 s).
