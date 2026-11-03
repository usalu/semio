# r2-vr: render verification of round-2 energy clip scenes

Scripts: `🧰️vr/` (`render.sh <lang> <clip dir> <Scene>`, `frames.py`, `grid.py`, `range.py`, `crops.py`). Renders: ql, 15 fps, de, one scene at a time. Frames via PyAV. Generated output deleted. No source edited.

| Scene | Verdict | Defect | Fix |
|---|---|---|---|
| cooling summer-strategy Beat2_SorptionsKuehlung (de) | OK | Wheel orange, exchanger neutral, humidifiers blue, coil pale orange on the one 70 K span; consistent. Solar collector is a glyph primitive (known open from cl). Coil "70 °C" readout and the dashed solar line pass close to the "Plattentauscher" label, no overlap. | none |
| ventilation volume Beat1_Innenvolumen (de) | OK | No staffage; "10 m" dimension clears the formula box (OI-67 confirmed). | none |
| ventilation volume Beat3_Volumenstrom (de) | OK | Four standing persons, equal size, feet on one floor, thermal, breath puffs light and inside the window (bj fix holds, no arm bundles). | none |
| ventilation natural Beat2_StossOderKipp (de) | OK | Radiators (0.71 m, wall-less rig) sit under the sill at ~0.7 m scale vs the 2.5 m room; no floating parts. | none |
| heating conduction Beat4_WattVergleich (de) | OK | Person 1.75 m, toaster and tealight scaled to the same unit, magnifier circles clear labels (OI-132 confirmed). | none |
| physics fundamentals Beat1_Leistung (de, kettle) | OK | Kettle goes blue, orange, red after the click inside the panel; thermal. Steam plume is not visible at panel scale (clipped by the icon window); not a defect of placement. | none |
| demand system-losses Beat2_UebergabeVerteilungSpeicherung (de) | OK | Person 1.66 m vs 2.5 m storey, radiator 0.6 m, thermal peach. | none |
| demand recap Beat2_Verlustseite (de) | OK | No staffage in the sampled frames. | none |
| demand demand-power Beat2_Normheizlast (de) | OK | Radiators inside the house section proportionate; wall strips fine. | none |
| demand regulations Beat4_Gradtagszahl (de) | NOT VERIFIED | Render failed, see below. | none |

## Not verified
- en renders (SorptionsKuehlung, Volumenstrom, WattVergleich) and de Beat4_Gradtagszahl failed at the cast export step, after the de renders above had passed: `VIEWPOINTS is not defined` (first), then `Spread syntax requires ...iterable not be null or undefined` (shared-person-winter, shared-toaster-winter, shared-person-pathlines). Cause is another agent's in-progress edit of the TS/Python staffage exporter; the same scenes rendered earlier in this run. Rerun `🧰️vr/render.sh en ...` once the exporter is stable.
- Not rendered: ventilation need (_room_occupants), recap Beat4, system-losses Beat4 (same radiator helper as the rendered scenes).
