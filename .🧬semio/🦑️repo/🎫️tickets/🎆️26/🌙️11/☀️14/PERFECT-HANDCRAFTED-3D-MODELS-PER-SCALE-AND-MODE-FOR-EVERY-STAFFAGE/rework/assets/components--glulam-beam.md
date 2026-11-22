# components/glulam-beam — rework report

## Pass 1 (second round), 2026-11-22

State found: no earlier report, no mention in the batch reports. Source `volumes/components/glulam-beam.json` and catalog `asset.volumes` were identical in all solids; `r3-validate.ts --volumes` was `VALID` before any change.

### Reference table (written before judging)

| Feature | Reality | Model before | Verdict |
|---|---|---|---|
| Section | GL24h 160 x 600 mm (widths in 20 mm steps, h/b 3.75) | 160 x 600 | right |
| Length / span | 8.00 m, bearings 120 mm long, span 7.88 m between bearing centres | 8.00, plates at 0..0.12 and 7.88..8.00 | right |
| Lamellae | 40 mm (EN 14080: at most 45 mm), 15 pieces | 15 x 40 mm, alternating tone | right |
| Finger joints | 15 or 20 mm finger length, 3.8 or 6.2 mm pitch, zigzag visible on the lamella edge (beam side), straight line on the wide face; boards 1 to 5 m; staggered between neighbouring lamellae; none in the highly stressed zone of the tension lamella | 25 joints, boards 0.94 to 4.45 m, staggered, none in the middle third of the bottom lamella; 20 mm length, 20 mm pitch (two fingers per lamella) | length right; the pitch is typified (6.2 mm is below the 7.5 mm feature of 1:50, a flank of the 20 mm pitch is the finest that the scale carries) |
| Arris chamfer | 2 to 3 mm | absent | right for 7.5 mm feature |
| Bearing plates | steel 120 x 160 x 20 mm, bearing stress about 1.0 N/mm2 under 18.6 kN (fc,90,k 2.5) | 120 x 20 x 160 mm both ends | right; pin and roller hardware (dowel, slide layer) are hidden in reality and identical outside |
| Mass | 0.096 m2 x 8 m x 420 kg/m3 = 322.6 kg | 322.56 kg | right |
| Surfaces | sides 2 x 8 x 0.6 = 9.6 m2, top and soffit 1.28 m2 each, two end-grain faces 0.096 m2 each, plates 0.0992 m2 total of which only the edges (0.0224 m2, 23 %) meet the air | sides, top, soffit, plates; no end-grain region; plates 0.099 m2 as outer area | **defect** |
| Thermal skins | storage depth 36 mm on every exposed timber face, the beam is solid | four 36 mm skins as a tube open at both ends | **defect** |
| Structure | one bending member with the full section, pin left, roller right, travelling load plate | member box, pin wedge plate, roller cylinder, load plate at 1:50 and 1:100 | right |

### Defects found (by rendering)

1. Thermal, all four scales: the end view of the thermal list was an open rectangular tube (1:50, 1:100) — the beam read as a hollow box girder, and the end-grain faces belonged to no region. The measured outer area of the four regions contained the ring of skin ends (sides 9.682 m2 against 9.6).
2. Thermal, bearing steel: 0.099 m2 passed as outer area only because the air flooded the open tube and reached the top of the plates from inside; with the tube closed the true situation shows (top covered by the beam).
3. Body, structure, rig, 2D: no geometric defect found at 1:50 (six views, end view close-up, both clips). The body in the three poses differs only by the load, which is no body solid; the structure list shows the load plate travelling along the top face (read at t = 5.5 s).

### Rebuilt

- Catalog root (`rework-glulam-beam-ends.py`, hand-stated numbers): new thermal region `beam-end-grain` (0.192 m2, vertical plate, glulam, same storage depth as the faces, assumption text en/de); entry in the thermal block of all three poses; `bearing-steel` now `areaBasis: "total"` 0.0992 m2 with `exposedFraction` 0.23 per pose (edges only; measured 0.226).
- Thermal solids at 1:50, 1:100, 1:200: a 36 mm end-grain skin over the full section at each end; the face skins (prisms at 1:200) run between the two end skins. 1:500: a 75 mm end slab above the bearing block. The rim of an end skin is declared a section face (butt joint), so the end region measures exactly 0.192 m2; the long faces give up 0.9 % of their length. A mitre on all four edges would need a frustum, which no solid kind gives exactly (a loft with a high exponent cuts 43 mm off the corners of an 8 m face).
- Measured after the change (1:50): sides 9.596 / 9.6, top 1.277 / 1.28, soffit 1.277 / 1.28, end grain 0.192 / 0.192, plates exposure 0.226 / 0.23.
- 2D: front `beam-end` (and the fourteen front glue lines at 1:50) are bound to `beam-end-grain`; the elevation `beam` rectangle carries the region too (seen edge-on, like top and soffit). No geometry changed, so no drawing was redrawn; the 1:50 drawings were rendered and read in light-line and thermal.
- Decisions: one sentence per scale ("Thermal ends: …", en and de).
- Tried and withdrawn: a load plate in the 1:200 structure (rule `volumes-monotonic`: a coarser scale must be simpler than 1:100; the existing decision "Minimal" stands).

### Validation

`r3-validate.ts components/glulam-beam --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`. `--catalog-only --volumes`: `VALID`. Source and catalog volumes are identical. Generated PNGs and logs deleted.

### What remains imperfect

- Sheets read after the last change: 1:50 thermal (six views), 1:500 thermal end view, 1:50 structure in clip `moving-point-load` at 5.5 s, 1:50 2D (light-line, thermal). Rendered without error but NOT read after the last change: 1:100 and 1:200 in all three lists, 1:500 body and structure, 1:50 poses `self-weight` and `point-load-midspan`, clip `loading-ramp`, clip `moving-point-load` at 0.5 s. Their body and structure solids are unchanged from the state read before the change at 1:50 only; the coarser body and structure lists were judged from their numbers, not from images.
- 1:50 body: the finger joints are a typified two-finger zigzag (20 mm pitch) because the true 6.2 mm pitch is below the feature; a side close-up of a joint could not be taken (the sheet has no zoom, 8 m on 2400 px).
- 1:50 body: pin and roller bearing look alike (two plain plates). True to a plate bearing with hidden dowels, but the fixed side shows no visible restraint.
- 1:500 thermal: the soffit prism reaches the ground with the merged block, so its measured exposure is 0 (information only), and the 75 mm bearing blocks measure 0.161 m2 against 0.0992 (information only at this scale).
- End skins butt against the face skins instead of a mitre (0.9 % of the long faces' area is counted nowhere).
- No `mass` marker in the structure list: the mass is uniform along the member, which is the member itself.

### Fundamental issues (outside this asset)

- `r3-sheet.ts` has no zoom or region option; details of a long slender asset (20 mm fingers on an 8 m beam) cannot be inspected in the side views.
- `region-undrawn` demands every thermal region in the side elevation, also a face that is only seen edge-on there (end grain); the only way is to list it on the body shape of the elevation.
- `volumes-insert.py` runs `main()` at import, so its `dump` cannot be reused; the catalog file is plain two-space JSON with only `volumes` in the compact form.
- No solid kind gives an exact rectangular frustum, so storage skins cannot be mitred on all four edges.
