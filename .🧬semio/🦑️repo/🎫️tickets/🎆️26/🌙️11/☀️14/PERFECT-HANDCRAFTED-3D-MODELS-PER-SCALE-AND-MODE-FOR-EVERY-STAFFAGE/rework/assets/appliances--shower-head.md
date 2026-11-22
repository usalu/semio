# appliances/shower-head — rework report

Chisel: `rework-shower-head-pass1.py` (hand-chosen numbers; reads the pass-0 volumes kept as `rework-shower-head-pass1-base.json`, `candidate` writes a document for `r3-validate --file`, `publish` writes the volumes source and the root fields of the freshly re-read catalog file; `volumes-insert.py` afterwards restores the catalog formatting of `volumes`).

## Pass 1 (2026-11-22)

Status: **partial** (see "Remaining").

### Reference table (hand shower set on a riser rail, hansgrohe Croma Select S / Unica class)

| Feature | Reality | Model before | Model after |
|---|---|---|---|
| Rail | 22 mm tube, 650–900 mm set, pipe 669 mm, bracket axes about 0.69 m, axis 55–65 mm from the wall | 22 mm, tube 0.67 m, bracket axes 0.686 m, axis 60 mm | unchanged |
| Brackets | wall rosette 40–45 mm, arm 20–22 mm, end socket 28–30 mm | 44 / 22–20 / 30 mm | unchanged |
| Slider | sleeve 32–36 mm x 45–55 mm, clamp button, fork with tilting conical socket about 45 mm ahead of the rail | 34 x 50 mm, button, fork, two pivot pins, slotted cone | unchanged |
| Hand shower | disc 110 mm x 25–30 mm, handle 25–29 mm, overall 240–260 mm, disc cranked 15–20 degrees | 110 x 26 mm, 24.5–29 mm, 259 mm, 15 degrees | unchanged |
| Spray face | silicone nozzle field with a centre cap and a ring between the spray zones | **one flat disc** | centre cap 25 mm, zone ring 54–61 mm (texture relief) |
| Spray selector | push button 15–20 mm on the handle front under the disc | **missing** | 16 mm button, 2.5 mm proud |
| Hose | 1.25 m, 13–14 mm, crimp ferrule 16–17 mm and conical nut at the shower, nut at the elbow; swivels in the conical nut | 13 mm, 1.21 m as one rigid loft on the elbow bone, no ferrules; **in the default pose (shower tilted 8 degrees) the hose left the nut 4.7 mm off its axis and poked through the nut wall** | two lofts (falling leg + bight on `elbow`, rising leg on the new bone `hose`), ferrules 17 mm at both ends, centred in the nut in every pose |
| Wall elbow | escutcheon 60 mm, body 34 mm, G 1/2 outlet down, quarter-turn lever 50 mm | 60 / 34 mm, outlet with nut, boss, hub, lever | unchanged |
| Load path | rail on two wall brackets; the elbow is screwed into its own wall fitting; the hose hangs between elbow and hand shower | supports only at the two brackets; elbow and hose groups reached no support (the validator accepted it because the hose came within 1.5 mm of the rail) | third support `wall-elbow`; hose legs connected through `member-outlet` (extended to the nut) and `member-nut` |

### Defects found

1. Rig: the hose was rigid on the elbow bone, so tilting the hand shower (poses `running` -8, `eco` -4 degrees) tore the hose out of the nut axis.
2. 1:50 body: no selector button, no ferrules, flat spray face.
3. 1:200 body: the hose fattened to 30 mm ran 5 mm inside the 30 mm pole over its whole length and pinched at the bend under the handle.
4. 1:100, 1:200, 1:500: the hose loft twisted under the handle (sections without `rotation` flip their frame, see "Fundamental issues").
5. Structure: no support under the wall elbow; hose member not touching the elbow outlet (14 mm short) nor the hand shower (26 mm short); lever, holder and hose without member or mass solid at 1:50.

### Rebuilt

- **Rig** (catalog root): bone `hose` (parent `shower`, joint at the nut mouth u = -0.036, rest -163.012 degrees, length 0.786 m, revolute -12..42 degrees, 0.15 kg); `elbow` mass 0.55 -> 0.40 kg (total unchanged 2.27 kg). Poses `running` and `eco` carry the compensating hose angle (8.341 and 4.173 degrees) so the leg keeps aiming at the top of the bight; residual slip at the seam under 2 mm, covered by plugs. Anchor `wall-elbow` (pin) added to `structure.supports`, every pose and both clips; the structural assumption text rewritten (en, de).
- **Body**: all four scales: hose split at the top of the bight into `hose` and `hose-rise`, one seam, each end a 75 % plug inside the other (a first try with a coincident overlap showed z-fighting and was replaced); explicit section frames on the rising leg. 1:50: `ferrule-shower`, `ferrule-elbow`, `select-button`, `face-zone-ring`, `face-centre`. 1:200: rising leg redrawn 91 mm from the wall (clear of the pole) with a rounded start inside the handle rod. 1:500: one more section in the bend under the handle.
- **Thermal**: hose wall and water column follow the split at every scale (`th-hose-rise`, `th-water-hose-rise`); the new 1:50 solids are bound to `handle`, `valve` and `head`.
- **Structure**: every scale: `member-hose-rise` (member `hose`), `support-wall-elbow`; 1:50: `member-outlet` reaches the nut, `member-nut`, `member-lever`, `mass-holder`, `mass-hose`; 1:100: `member-outlet` added, `member-shower` reaches the nut, `mass-holder`, `mass-hose`; 1:200 and 1:500: `member-shower` reaches the hose end, `member-hose` added at 1:500, `mass-hose` at 1:200.
- **Decisions**: one sentence per list and scale appended (en, de); 1:200 body sentence on the hose rewritten.
- **2D**: not redrawn. The changes stay under 0.1 mm on paper at 1:50 (hose top moves 4.7 mm, button 2.5 mm proud); the ferrules (17 x 18 mm = 0.35 mm) are not drawn. `r3-2d` 1:50 read after the change: unchanged and consistent.

### Rendered and read after the last change

1:50 body right view in `running`, `off`, `eco` (close-ups of head, holder, nut, valve, bight), `off` six-view sheet, clip `eco-and-off` t = 100 s, view from below onto the face, front view of the bight; 1:50 thermal cut and structure sheets; 1:100 right view and structure sheet; 1:200 right view; 1:500 right view with close-up of the bend. Rendered without `NOT DRAWN` but **not read**: clips `turn-on-and-run` t = 2 / 150 s and `eco-and-off` t = 300 s (step clips, identical to the poses read), thermal and structure sheets of 1:200 and 1:500, thermal of 1:100, 2D of 1:100 and 1:200.

### Validation

`r3-validate.ts appliances/shower-head --catalog-only --volumes` -> `VALID appliances/shower-head (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (same with the source merged).

### Remaining (stated plainly)

- 1:50 body: the hose leaves the nut with a kink of up to 11 degrees in `running` (rigid leg hinged at the nut; a real hose bends smoothly over the first 60 mm).
- Rig: the prismatic `holder` joint (-0.47..+0.12 m) is used by no pose; sliding it would need a hose of another shape (rigid lofts cannot follow). No pose or clip for a lowered holder.
- 1:50 body: spray nozzles (1.5 mm) are below the feature and only present as the dark face; the rail has no visible end-cap seam; the hose has no braid relief.
- 1:200 body: a thin wedge is visible between the handle rod end and the holder block in `running`; the 30 mm hose touches the pole tangentially.
- Structure 1:100–1:500: the spray disc has no member of its own (the `shower` rod ends under the disc); no `mass-holder` at 1:200/1:500; no joint solid for the new `hose` bone at any scale.
- 2D: ferrules and selector button not drawn at 1:50; drawings of 1:100 and 1:200 not re-read against the changed 1:200 hose line (6 mm = 0.03 mm on paper).
- Sheets listed above as "not read".

### Fundamental issues (outside this asset)

- Loft kernel: a loft whose sections carry no `rotation` picks a frame per section that can flip between neighbouring sections on a gently bent, sparsely sectioned path (visible as a twisted, pinched tube; seen on the hose at 1:100, 1:200, 1:500, already in the pass-0 state). Worked around here with explicit `rotation` on every section of the rising leg; the root fix belongs in the loft evaluator of `S/🔨️modules` (parallel-transport frame along the path).
- `volumes-connected`: a structure group counts as supported when any member comes within 1.5 mm of a supported group, even where the parts do not physically join (pass-0 hose passing the lower rail socket). The rule could require contact at a joint or declared connection.
- `r3-sheet.ts` has no zoom/region option; close-ups of small hardware need a manual crop of the 2400 px sheet (about 1.25 px/mm on this asset).
