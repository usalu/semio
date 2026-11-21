# Round 3 — objects/playground-swing

State: done (2026-11-20). The roots, the 2D of every scale and view, and the volumes of every scale were rebuilt as a real public single-bay steel A-frame swing. The catalog is written and spliced. `r3-validate` reports VALID both for the catalog alone and with the source. `volumes-check.ts --product` is green on every rule at every scale except `volumes-thermal-coverage`. That rule fails on `chain-steel` (1:50) and on `seat-rubber` (1:50 and 1:100), because C16 does not let me correct those two declared areas by more than a factor of 1.5 (see "Still imperfect").

Scripts (all in T):
- `r3-playground-swing-kit.py`: hand-stated geometry, bone frames, masses, sections and helpers.
- `-roots.py`: summary, provenance, dimensions, materials, rig, anchors, loads, thermal titles and areas, structural assumptions.
- `-draw.py`: 2D for 4 scales x 3 views, plus the representation decisions.
- `-s50.py`, `-s100.py`, `-s200.py`, `-s500.py`, `-coarse.py`: the volumes.
- `-areas.json`: the area corrections.
- `-build.py`: builds the candidate; `--source` writes the volumes source, `--apply` writes the catalog members.
- `-sheet.ts`: a candidate-reading copy of `r3-sheet.ts`.
- `-suit.ts`: print-profile suitability.
- `-crop.sh`: crops and enlarges render details.

Generated output is in `🗑️generated/r3-playground-swing/`. The pre-round-3 catalog and source are kept there as `catalog-before-r3.json` and `source-before-r3.json`.

## Reference (real subject)

Class: public-playground single-bay steel A-frame swing with one flat rubber seat on two chains (EN 1176-1/-2), surface-mounted on base plates.

| quantity | value | source |
|---|---|---|
| frame height | 2.5 m class (beam top); chains 2.00 m for a 2.50 m suspension height | KOMPAN KSW921 "1-seat steel frame H 2.5 m", L x W x H 2.32 x 1.84 x 2.55 m (product data as quoted by the search index; the PDF itself could not be parsed here); Huck article 4712 |
| leg spread / height | KSW921 1.84 / 2.55 = 0.72; KSW932 (2 seats) 2.4 / 2.7 = 0.89; garden A-frame 1.63 / 1.80 = 0.9. Chosen: feet 2.0 m apart, apex 2.45 m (0.82), legs 22.2 deg from vertical | kompan.com KSW921 / KSW932; retail A-frame stand 87 x 64 x 71 in |
| seat | flat rubber seat with aluminium reinforcement, 48 x 17 cm overall, seat about 38 x 17 cm, ring screws with finger-trap protection. Chosen: 430 x 175 x 35 mm, eye bolts at +-200 mm | Huck article 4712 |
| seat clearance | free space from seat edge to frame >= 20 % L + 0.20 m = 0.58 m for L = 1.9 m; flat seat >= 0.35 m above the surface | maker's installation instructions quoting EN 1176-2 |
| pendulum | L = 1.9 m (pivot to seat hitch), T = 2.766 s, amplitude 30 deg | asset (kept), Halliday et al. |
| tubes | beam o101.6 x 3.6 (8.70 kg/m), apex sleeve o114.3 x 4.0, sockets o76.1 x 3.2, legs o60.3 x 2.9 (4.11 kg/m), ties o42.4 x 2.6 (2.55 kg/m); hot-dip galvanised | EN 10219-2 / EN 10255 section tables |
| chain | DIN 766 short link 8 x 24 (wire 8, inner length 24, inner width about 11, outer width 27.2, outer length 40 mm), about 1.3-1.4 kg/m | DIN 766; the asset's own thermal layer "link bar 8 mm" |
| swing hanger | clamp band round the beam, forged fork, bearing pin about 85 mm below the beam axis | class construction |

## Audit of the catalog state before round 3

- **Rig and dimensions.**
  - Beam axis at 2.30 m (H 2.35). The chain pivot sat on the beam axis, but the real bearing pin hangs below the beam.
  - Seat hitch at 0.40 m. The seat underside was at 0.302 m, below the 0.35 m minimum for a flat seat.
  - Legs at 60.5 deg, 2.64 m long, feet 2.6 m apart (spread/height 1.13, wider than the class 0.72-0.9).
  - A-frames at z = +-0.55 m with a 1.2 m beam. The seat edge was 0.27 m from the frame against the required 0.58 m: the frame was far too narrow for a public swing.
  - Seat bone 0.42 m long, pointing past the seat.
- **Seat.** 420 mm deep in the swing direction x 520 mm wide x 58 mm: a square slab. A real flat seat is 170-175 mm deep and about 430-480 mm wide.
- **Masses.** 11.7 kg per leg bone, which carries two legs: two o60 x 3 legs of 2.64 m alone weigh 22 kg. massKg was 54.3.
- **Body.**
  - Chains were 11 mm rods with no links.
  - The hanger was a flat tab.
  - There was no apex construction: legs just ran into the beam.
  - Brace clamps were boxes.
  - No anchor bolts and no gussets.
- **2D.**
  - The chain was drawn on top of the near A-frame, although it hangs behind it.
  - The seat was 0.42 m deep in the elevation and 0.42 x 0.52 m in plan.
  - The front view had no apex parts.
  - 1:200 and 1:500 drew the legs as thin lines 2.64 m long at the old angle.
- **Structure and thermal.** The tie had no sections. Areas: frame-galv 2.966, chain-steel 0.13, seat-rubber 0.489 (the latter from the oversized slab).

## Root fixes (catalog, old -> new)

- **`dimensions`:** H 2.35 -> 2.50, W 2.78 -> 2.18, D 1.20 -> 1.90 m, mass 54.3 -> 99.1 kg (from EN 10219-2 tube masses, plate volumes and DIN 766 link volumes, summed per bone).
- **`rig.bones`:**
  - leg-left: angle 60.5 -> 67.797 deg, length 2.64 -> 2.6462 m.
  - leg-right: origin x 2.6 -> 2.0 m, angle 119.5 -> 112.203 deg.
  - brace: (0.34, 0.6) / 1.92 m -> (0.2449, 0.6) / 1.5102 m.
  - beam: (1.3, 2.3) -> (1.0, 2.45).
  - chains: pivot (0, 0) -> (0, -0.085) on the beam (the bearing pin).
  - seat: length 0.42 -> 0.0875 m.
  - Every bone gets a mass (legs 25.25, brace 12.1, beam 28.11, chains 2.34 each, seat 3.73 kg, with centres) and an EN 10219-2 / DIN 766 / seat-core `section`.
- **`anchors`:** supports z +-0.55 -> +-0.875 m; seat-centre y -0.04 -> -0.03 m. The pose loads on the seat move to the same point.
- **`materials`:** new `aluminium` (seat core, used by the structure).
- **`provenance`:** five references added (en-1176-2, kompan-ksw921, huck-4712, en-10219-2, din-766); notes rewritten in en and de.
- **`summary`:** rewritten.
- **`thermal`:**
  - Region titles now name the hangers and eye bolts.
  - Third assumption: the stale "0.1 m2" removed.
  - Areas (C16): frame-galv 2.966 -> 3.23 (measured 3.231); chain-steel 0.13 -> 0.195 (limit; honest 0.339); seat-rubber 0.489 -> 0.326 (limit; honest 0.173). All three are listed in `area-review.md`.
- **`structure.assumptions`:** positions updated; tension-only chains.
- **`representations`:** every scale and view redrawn by hand from the kit geometry (ruling E1), with the same bones, roles and region bindings.
  - 1:50 elevation: the chain is one union outline of 38 face-on links joined by edge-on links, drawn behind the A-frame.
  - Chains and seat keep their extruded `solid` hints.
  - All new decisions are bilingual.
  - Print suitability: all four scales authored with no violations.
- **Unchanged:** `clips` and `presentation`. The pendulum period stays 2.7656 s, because L = 1.9 m is kept.

## Volumes per scale

| scale | body (expanded) | thermal | structure | what was modelled |
|---|---|---|---|---|
| 1:50 | 96 (392) | 96 refs | 38 | **Frame:** legs as lofts with the horizontal mitre on the plate; plates 180 x 100 x 12 with two 8 mm gussets and two anchor nuts with studs per foot; o42.4 ties in tee clamps (collar plus stub); beam with domed ends; per A-frame a welded apex bracket (o114.3 sleeve, two o76.1 sockets with chamfer, M12 through-bolts and nuts, 10 mm gusset, vertical sleeve bolt and nut). **Hangers:** band, two 8 mm cheeks, clamp bolt, bearing pin. **Chains:** forged top eye, shank, lower eye, then 76 DIN 766 links per chain as round-wire half-link lofts that interlock exactly (4 repeats per chain). **Seat:** eye bolts with collar and acorn nut; seat as a 9-section superellipse loft. **Structure:** hollow members, joints by reference (plates, clamps, sleeves, pins, eye bolts), wedge and roller supports, 10 masses. |
| 1:100 | 26 | 26 refs | 34 | Legs into the beam (sockets and sleeves merged); 15 mm plates and gussets; ties; domed beam; 15 mm gussets; one hanger block each; chains as 18 mm slender rods reaching the seat; 5-section seat loft. Structure: hollow members, sphere joints, supports, masses. |
| 1:200 | 18 | 18 refs | 26 | 40 mm foot blocks; leg lofts; ties; plain beam; 30 mm gussets; hanger blocks; slender chains; rounded 35 mm seat slab. Structure: solid rods, joints at apexes and pins. |
| 1:500 | 14 | 14 refs | 24 | Slender legs and ties at true size (C24), 75 mm foot blocks, beam, slender chains, 75 mm seat bar. Structure: slender rods, apex and pin joints, masses. |

Final check line (`$BUN volumes-check.ts objects/playground-swing --product`, 120 s): `FAIL: 3 measurement(s) over 2 rule line(s)`. The three failures are `volumes-thermal-coverage`: 1:50 chain-steel +74 % and seat-rubber -47 %, and 1:100 seat-rubber -47 %.

Everything else is ok:

| scale | silhouette IoU (elevation / front / plan) | structure outsideBody (elevation / front / plan) |
|---|---|---|
| 1:50 | 0.983 / 0.929 / 0.989 | 4.9 / 5.6 / 1.9 mm |
| 1:100 | 0.997 / 0.996 / 0.999 | |
| 1:200 | 0.998 / 0.999 / 1.0 | |
| 1:500 | 0.990 / 0.990 / 0.998 | |

The other rules also pass at every scale: bounds (width 0.0, height 13.5 mm at 1:50 from the sleeve bolt heads, depth 0), feature, monotonic, cross-scale, bones, slender, copied and views-disagree. frame-galv is within 0.02 % at 1:50; the 1:200 and 1:500 thermal totals are within 2 %.

## What the final renders show

- **1:50 body sheet** (`🗑️generated/r3-playground-swing/sheet/objects/playground-swing.s50.body.png`):
  - Front: the 1.9 m beam with domed ends on two A-frames edge-on, sleeves, socket bolts, tee clamps at 0.6 m, two hangers with chains of visible links, and the seat with eye bolts at 0.44 m.
  - Right side and left side: the A of two legs splayed 22 deg, the apex bracket round the beam end, the tie in its clamps, the base plates with gussets. The chain hangs behind the frame and shows the link rhythm; the seat is a narrow 175 mm plank.
  - Back: mirror of the front.
  - Top: four leg strips, plates, the beam with the sleeves, the seat under the beam.
  - Three-quarter: reads at once as a real public swing.
- **Close-ups:**
  - Apex: bracket, hex socket bolts and gusset, with the first links below.
  - Seat end: the last link in the eye bolt, the collar, the acorn nut under the rubber.
  - Foot: plate, gussets and nut.
- **1:50 pose `pendulum-forward`:** chains at 30 deg, the seat stays horizontal (counter-rotated hitch) and passes between the A-frames; the 2D side elevation shows the chain behind the near leg.
- **1:50 thermal:** frame red, chains green, seat purple; no stray solids.
- **1:50 structure** (sheet and turn 230 view): blue hollow members, orange joints at plates, clamps, sleeves, pins and eye bolts, green wedges and rollers under the plates, magenta masses on legs, ties, chains and seat.
- **1:100 / 1:200 / 1:500 body:** each is a recognisable, correctly proportioned swing at its own abstraction. The chains reach from the hanger (or, at 1:500, the beam) to the seat. The 1:500 slender parts show yellow in the renderer.
- **2D, all four scales and the forward pose** (`🗑️generated/r3-2d/objects/playground-swing.s*.png`): clean drawings that match the body. 1:500 reads as the A, the U-frame and the I-plan.

## Still imperfect (stated plainly)

- **`volumes-thermal-coverage` stays red for chain-steel (1:50) and seat-rubber (1:50, 1:100).**
  - The honest values are 0.339 and 0.173 m2. They are beyond the factor 1.5 of C16, so I applied only up to the limit (0.195 and 0.326) and listed both in `area-review.md` for the reviewers.
  - The 0.489 seat area came from the old oversized slab.
- **Simplified or merged:**
  - The tee clamps have no grub screws.
  - The hanger band has no visible split.
  - The link wire is round on a stadium path. Each link is two half lofts, so there is a seam mid side.
  - The plastic finger-trap caps of the eye bolts are an acorn nut.
- **2D generalisation:**
  - At 1:50 the chain is drawn as the union outline of the face-on and edge-on links, without the 1.6 mm slots beside the bars (not printable).
  - Front and plan draw each chain as its 27 mm link envelope through the `solid` hint.
- **Coarser scales:** 1:200 hanger blocks rise 7.7 mm above the beam top (the band merged with the fork). The 1:500 seat is a 75 mm bar (feature).
- **Tooling:**
  - `r3-sheet.ts` reads only the catalog, so I used a candidate-reading copy (`r3-playground-swing-sheet.ts`) before writing.
  - A closed-loop loft measures 0 mm long in `volumes-feature` (`sortedExtents` uses first-to-last section distance). That is why the links are split into two half lofts. Filed in `tool-requests.md` (2026-11-20 entry).

## 21 November welded links

Each of the four chains now has 38 full welded wire links modelled as continuous 8 mm hoops. The mating links alternate planes. The fine thermal model follows the steel surface. At the time of inspection, the presentation GLB had 8,236,188 bytes and all twelve scenes reloaded cleanly. The revised validator measures repeated, spatially disjoint links independently and reuses identical geometry across modes. The complete official 1:50 validation now passes at 705 MiB. Its separate thermal check passes at 367 MiB. All four current digital scales passed.

After the exporter began sharing repeated rigid geometry, the same twelve verified scenes occupied 3,114,548 bytes.

The two hanging chain outlines now retain a 0.20 mm paper stroke at 1:500. Both remain printable at the 1:1000 endpoint and the current document passed its nominal and range checks.
