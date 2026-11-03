# Migration M6: demand and physics clips

## Changed
- `📊️demand/🎬️clip/{♨️system-losses,⚖️regulations,⚡️demand-power,🏭️primary-energy,🏷️certificate,🔁️recap}/🐍️.py` and `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`: `person_glyph`, `radiator`, `lamp_glyph`, the tree silhouette, the vacuum silhouette, the laptop silhouette and the hand-drawn desk now use `staffage` / `staffage_motion` / `staffage_height` from the visuals module.
- Casts added to `V/🧍️staffage/🔣️.json` (all with `theme: dark`): `demand-<clip>-person|radiator|lamp`, `demand-primary-energy-tree` (english-oak, dark-line), `demand-certificate-{walker,sleeper,buyer,tenant}`, `demand-recap-{desk,laptop}`, `physics-fundamentals-{person,clerk,radiator,lamp,vacuum}`.
- People, radiators, lamps, laptop, sleeper and vacuum use the animated `thermal` view (idle-breathing, breathing, run-and-park). Buyer, tenant, clerk and the desk are `dark-line` (no heat meaning).
- Local helpers per clip (`_stand`, `_heater`, `_lamp`, `_radiator`, `_person`, `_vacuum`) keep the old call signatures where layouts depend on them. `_lamp` returns `group`, `bulb` and, in fundamentals, an invisible `cable` line for the current flow.
- Removed the radiator colour updater in `⚖️regulations` (thermal view is not recolourable); heat level is shown by the existing stream.
- `🏭️primary-energy` Beat 4: `TransformFromCopy(design_in, reference_in)` crashed (animated frames differ in point count); replaced with a `FadeIn` that shifts in from the design house. Not re-rendered after the fix (render was stopped on the coordinator's request).

## Kept as glyphs
Flame, factory, sun, thermometer, clock, shower head, gas hob with pot (experimental apparatus), meters, charts. `🧪️practice` clip was not in scope and still uses the old glyphs (`person_glyph`, `radiator`, `lamp_glyph`).

## Verified
- Low-quality (l, 15 fps) renders of EnergyBalance_02 to 07: all segments complete except Beat4_Referenzgebaeude (the transform crash above, since fixed). Frames inspected: Bedarf vs Verbrauch (house with thermal person, lamp, radiator), Übergabe/Verteilung/Speicherung (room section with thermal person and radiator), Rechenbeispiel.
- Fundamentals helpers and the certificate sleeper checked in a still (`m6_sheet.py`): person, clerk, radiator, vacuum, lamp (bulb point on the bulb) and sleeper all land on the floor line at the intended scale.
- Not verified: full PhysicalFundamentals render (stopped; memory pressure), recap chapter, Beat4_Referenzgebaeude after the fix, mouth/plume offsets in fundamentals Beat 7 (derived from the old glyph proportions, may need a nudge), Python unittest suite.

## Helper scripts (inputs)
`m6_frames.py` (contact sheet from an mp4 with PyAV), `m6_crop.py` (magnified crop), `m6_sheet.py` (still of helper figures). ffmpeg through `repo tools exec` crashed in bun here, so PyAV from the uv env was used.
