# Migration M2 report: heating conduction, convection, solar-gains, calculation

## Changes

- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py`: toaster, person and radiator now use `staffage_motion` (thermal clips). The toaster keeps its watt-fill overlay. The comparison beat uses `FadeTransform` (the section-to-device `ReplacementTransform` broke on mismatched point counts). Tealight stays a flame glyph because the archive has no candle asset.
- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py`: person (animated thermal), pendant lamp (thermal pose `on`), laptop (thermal `open-idle`), desk (`height-adjustable-desk`) and radiators (animated thermal, the second one mirrored) now come from the cast API. The local helpers `_cast` and `_pendant` replace `lamp_glyph`, `person_glyph`, `radiator` and `object_silhouette`.
- `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`: the hand-drawn Raffstore (rail, bracket, slats) is replaced by `external-venetian-blind`, pose `lowered-cut-off-45`.
- `🔥️heating/🎬️clip/🌬️convection` and the series file `🔥️heating/🎬️clip/🐍️.py`: no staffage, no change.
- Manifest `🧍️staffage/🔣️.json`: 10 casts with the `heating-` prefix, all with `theme: dark`.
- `🎨️visuals/🧪️tests/🐍️scenes.py`: the plant scale test uses tolerances, because thermal frames carry a halo around the figure (person about 6 % and radiator about 12 % taller than nominal).

## Verification

Previews rendered with `manim -ql --fps 15` and inspected as contact sheets and crops: UltimateEnergyBalance, AnlagenVerluste, Beat4_WattVergleich, Beat5_Gebaeudehuelle and Beat5_Verschattung. All rendered and show the thermal staffage. The cast export ran through the bridge.

The staffage radiator is a vertical section (0.285 m wide), not the old 1.3 m wide front panel, so radiators look narrower.

## Open items

- **Solar-gains Raffstore:** the blind asset is edge-on in this section, so the slats are not visibly tilted. The reflected rays still start at the old 48 degree slat positions, so the blind and its reflections no longer line up visually. Consider the `tilt-*` poses or a plan view, and adjust the angle to 45 degrees.
- **Python test run** (`scenes` and `section_consumers`) did not finish. The host was under critical memory pressure (the cast export queue waited more than 600 s) and the run ended with an error in the `E` line. Re-run when the machine is quiet.
- `ffmpeg` is not installed here, so frames were extracted with PyAV from the venv.
