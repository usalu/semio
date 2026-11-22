# appliances/electric-fan-heater — rework report

Source `volumes/appliances/electric-fan-heater.json`, chisel `rework-16-electric-fan-heater-author.py` (hand-chosen numbers; `--write` also writes rig, poses, clips and volumes into the catalog file). Earlier batch report: `rework/16.md`, section 12 (thermal coils and motor stack added at all scales, nothing else).

## Pass 1 (second round)

### State found

- Source and catalog `volumes` were identical (no half-finished edit to reconcile).
- `r3-validate.ts --volumes` was INVALID: validateAsset 5, checkVolumes 8 (`volumes-feature` on dial, dial grip and intake panel at 1:100 and 1:200; `volumes-connected` on the structure of 1:100, 1:200 and 1:500: `member-cassette` and `member-intake` 15 to 19 mm away from the shell).
- 1:50 body: a good model already (hollow moulded shell and cover, end frames, steel cassette with eight slats, slotted intake, loft handle, bosses and pads, shaded-pole motor on a bridge, shroud ring, five pitched blades, two coil stages in a sheet box).

### Reference table (compact 2 kW fan heater, IEC 60335-2-30 class, typical catalogue values)

| Feature | Reality | Model before | Model now |
|---|---|---|---|
| Housing W x D x H | 200–260 x 130–200 x 230–270 mm | 228 x 196 x 256 (+16 feet) | unchanged |
| Height with handle | 280–320 mm | 310 | unchanged |
| Wall, edge radius | 2–3 mm ABS, 15–25 mm | 2.5, 22 | unchanged |
| Handle opening | 90–110 x 25–30 mm, bar 15–18 x 25 mm | 100 x 26, bar 16 x 26 | unchanged |
| Axial fan wheel | 100–130 mm, 4–6 blades, pitch 25–35°, 1300–2500 rpm | 130 mm, 5 blades, 28°, **rigid on the body bone** | own bone `fan`, joint axis = motor shaft |
| Motor shaft | 6–8 mm steel through the stack, rear bearing in the bridge | **missing** (hub floated in front of the stack) | 8 mm x 46 mm from bridge to hub |
| Outlet grille | steel cassette, 8–12 slats 1–1.5 mm, spacer ribs | 8 slats, **no ribs** | 8 slats, 2 vertical ribs |
| Heating coils | 2 stages, 3 runs each, 6–8 mm coil | 2 x 3 runs of 7 mm | unchanged |
| Thermostat dial | 40–45 mm, 12–15 mm proud, 270° travel | 44 mm, 14.5 mm, 0–270° | unchanged |
| Feet | 4 pads 25–30 mm on bosses | 4 x 30 mm pads on 37 mm bosses | unchanged |
| Mass | 1.2–1.8 kg | 1.5 kg | 1.5 kg (1.41 body + 0.03 knob + 0.06 fan wheel) |
| Mains cord | 1.5–1.8 m, grommet low at the back | grommet only | grommet only (cord left to the scene, stated in `decisions`) |

### Defects found

1. Rig: no fan bone, the wheel could not turn in any pose or clip.
2. 1:50: hub without a shaft; grille without spacer ribs.
3. 1:100, 1:200, 1:500: the lists were the 1:50 parts list with the housing simplified. 1.5 mm slats and blades, 7 mm coils, 8 mm bridge, 26 mm handle bar, 30 mm feet and the 10 mm dial stood in models whose smallest feature is 15, 30 and 75 mm; 1:200 and 1:500 were the same list.
4. Structure 1:100 to 1:500: grille cassette and intake panel were declared members and hung free of the shell.
5. Thermal: at the coarse scales no check against the rebuilt lists existed (see below).

### Rebuilt

- **Rig** (catalog): bone `fan` on `body` at (-0.040, 0.145, 0), `abductionDeg: 90` (lateral rest: u left, v up, joint axis w along the motor shaft; checked with `rig-rest.ts`), revolute without limits, 0.06 kg; body mass 1.47 → 1.41 kg. Poses `off` / `heating-low` / `heating` set the wheel to 0 / 24 / 48°. New clip `fan-spin` (6 s, 15 fps): run-up with the first stage, second stage at 3 s, wheel 0 → 990°, dial 0 → 120 → 240°.
- **1:50**: `fan-hub`, `fan-blades` moved to the fan bone (same form), new `fan-shaft`, new `grille-ribs`; structure gains `member-fan-shaft` and bearing `joint-fan`. 28 body / 21 thermal / 20 structure solids.
- **1:100 (15 mm)**, 21 / 19 / 16: tube housing with caps kept; cassette with 15 mm rails and four 15 mm slats for the eight; intake with four 21 mm slots; blades 15 mm thick on the fan bone; shroud, bridge, heater frame, brackets and coil runs at 15 mm; dial a 44 x 15 mm disc with a flat as pointer; switch 15 mm proud; grip bar and grommet dropped. Structure: end rings instead of cassette and intake, brackets, fan shaft and bearing added.
- **1:200 (30 mm)**, 8 / 10 / 7: closed block with 22 mm radii, grille and intake as dark panels 4 and 6 mm proud, handle as a lug 146 x 30 mm, feet as two 30 mm rails, dial as a 44 mm stub, fan wheel as its swept disc inside.
- **1:500 (75 mm)**, 4 / 10 / 7: block from the floor with 30 mm chamfers, two dark panels, handle lug 146 x 75 mm; nothing else in the body.
- **Thermal, coarse scales**: casing and cover with section faces at their joint, grille and intake as thin proud plates, dial and the four 10 mm pads at true size. Measured outer areas (m², `volumes-check.ts`): 1:50 casing 0.197, cover 0.049, grille 0.044, intake 0.037, control 0.0058, feet 0.0085; total deviation 1:200 5.8 %, 1:500 9.4 %; 1:100 every region inside 30 %.
- **decisions**: rewritten en/de for all four scales.

### Validation

`r3-validate.ts appliances/electric-fan-heater --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0` (after `--write`).

### Looked at after the last change (rendered from the published catalog and read)

- 1:50 body cut sheet (all six views, pose `heating`), 1:50 structure cut sheet, pose `off` cut three-quarter, clip `fan-spin` at 5 s cut three-quarter: wheel turns on its axle inside the shroud ring, shaft runs through the stack into the bridge, nothing pokes through.
- 1:100 body sheet (six views), thermal cut three-quarter, structure three-quarter.
- 1:200 body sheet (six views), thermal cut three-quarter, 2D drawings (`r3-2d`, light-solid and light-line).
- 1:500 body sheet (six views), structure three-quarter.
- Rendered but **not read** (budget): pose `heating-low`, `fan-spin` at 2 s, `warm-up` at 2 s, `step-down-and-off` at 700 s, 1:50 thermal cut after the change, 1:200 structure, 1:500 thermal.

### What remains imperfect

- 2D drawings were not redrawn. They pass `volumes-silhouette` at every scale, but: the fan wheel has no 2D shape on the `fan` bone (it is hidden behind the grille in all three views; a section drawing would show it); the 1:200 handle is still an arch stroke while the 3D is a lug; the 1:100 front still draws the old slat count; 1:500 has no 2D (`unavailable`, unchanged).
- 1:100 dial: the pointer is only a flat on the disc, the 270° travel is barely readable in the side view.
- 1:200 / 1:500 handle lug reads as a block on the cover, not as a grip; an honest consequence of the feature size, but a modeller might prefer an arch declared `slender`.
- 1:200 body carries the swept fan disc inside a closed block (needed so that the `fan` bone has geometry and a structure member); at 1:500 the `fan` and `knob` bones have no body solid, only their structure members.
- No mains cord, no pilot lamp, no screws at 1:50.
- Clips `warm-up` and `step-down-and-off` hold the wheel at the pose angle (1 fps over 15 and 30 minutes cannot show rotation); only `fan-spin` turns it.

### Fundamental issues (outside this asset)

- `volumes-thermal-coverage`: the same grille geometry (5 mm cassette frame, eight 1.5 mm slats, six 7 mm coil runs) measures 0.044 m² at 1:50 but 0.089 m² at 1:100; the 1.5 mm slats evidently do not count at 1:50 and count fully at 1:100. The region's `areaM2` (0.05) therefore only fits the 1:50 reading; at 1:100 the thermal slats had to be four sheets 12 mm deep to stay inside 30 %. Module `S/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures` / `🌊️flood`.
- `r3-validate.ts --json` carries no measurements; `volumes-check.ts` (the only tool with the numbers) cannot read a candidate document (`--file`), so a rig change has to be written into the catalog before thermal areas can be measured.
- `r3-sheet.ts` took 10 to 30 s per sheet under host load (documented 1 to 3 s); python on this host has no PIL, so sheets cannot be tiled for reading.
