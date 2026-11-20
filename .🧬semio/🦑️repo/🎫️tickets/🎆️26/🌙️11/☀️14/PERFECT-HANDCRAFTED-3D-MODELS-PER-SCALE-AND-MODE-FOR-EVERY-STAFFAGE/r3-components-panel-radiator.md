# Round 3: components/panel-radiator

Catalog asset: `S/🗂️catalog/🧱️components/panel-radiator/🔣️.json`. Scripts in `T`: `r3-panel-radiator-kit.py` (dimensions and section builders), `r3-panel-radiator-build.py` (volumes of 1:50, 1:100, 1:200), `r3-panel-radiator-roots.py` (every root value outside `volumes`; candidate document, `--apply` writes roots and volumes in one write), `r3-panel-radiator-2d-before.json` (the 2D shapes before this round, input of the roots script so it can be re-run), `r3-panel-radiator-print.ts` (print suitability of a candidate document). The asset was started fresh (no earlier r3 state existed).

## 1. Reference

| item | real value | source |
|---|---|---|
| type 22 construction | two pressed water panels (two 1.25 mm sheets each), vertical water channels at about 33 mm pitch between top and bottom headers, a trapezoidal convector fin sheet welded to the inner face of each panel, clip-on side covers, slotted top grille | EN 442-1 (type designation), manufacturer catalogues (`radiator-catalogues`) |
| height / length / depth | 600 / 1000 / 100 to 102 mm; hub distance = height - 50 mm | Purmo Ventil Compact datasheet (`purmo-ventil-compact`, drawing read from the PDF: type 22 depth 102, wall distance 25, B - 50) |
| valve radiator (Ventilkompakt) | built-in thermostatic insert with M30 x 1.5 head connection at the top of one end, two G 1/2 bottom connections at 50 mm centres in the middle of the depth (76 mm from the wall), four G 1/2 side tappings (plugs, vent) | `purmo-ventil-compact` |
| thermostat head | white plastic, about 50 to 55 mm across, 80 to 95 mm long, turning dial | `thermostat-heads` (Heimeier K, Danfoss RAW) |
| connection H-block | nickel-plated brass, two isolating spindles under caps, union nuts for 15 mm pipes | `thermostat-heads` (Heimeier Vekolux, Oventrop Multiflex) |
| output 75/65/20 | about 1.7 kW for 600 x 1000 type 22 (Purmo C22: 1709 W) | `purmo-c22-600-1000` |
| water content / mass | 6.6 l; about 29 to 31 kg empty (Kermi 400 x 600 type 22: 12.52 kg, 2.7 l, scaled by area) | `purmo-c22-600-1000`, `kermi-profil-k-22` |
| wall brackets | two upper hook brackets (radiator hangs on welded lugs of the rear panel) and two lower holders, screwed to the wall | catalogues |
| convector fin pitch | about 25 mm (authored estimate, the 2D symbol uses 50 mm) | catalogues |

## 2. Audit (before this round)

- Rig: `valve-head` joint at (u -0.025, v -0.055), i.e. a thermostat head hanging 25 mm under the radiator with a valve body standing under each end at 140 mm from the ends, connected to nothing (a type 22 compact radiator has no bottom tappings there); lockshield cap under the other end. Physically impossible arrangement.
- Anchors: `front-top-edge` y +0.1 and `grille-centre` y +0.05 lay on the wall side (v positive = towards the wall), 65 mm inside the wall and in the bracket gap; `bracket-top` (fixed) sat at lateral 0 where no bracket is (brackets at ±0.4 m).
- Mass 26 kg "with water" against about 35.6 kg (29 kg steel + 6.6 l water).
- 2D: front showed a 35 mm grille strip with convector fin tips (invisible from the front) while the section drew a 15 mm grille (views disagree); bottom cap drawn across the open air inlet; plan without head; valve parts moved inside the length "to keep 1.0 m".
- Solids (QA B/B/C): box panels in three horizontal slabs with deep seams that do not exist, no channels, convector zig-zag only, solid grille cap, end caps invented, structure one solid slab with a rail-shaped support, thermal front bands as full-depth slabs.

## 3. Root fixes (catalog, `r3-panel-radiator-roots.py`)

| member | old | new |
|---|---|---|
| summary | compact type 22 with thermostatic valve | type 22 valve-compact: channels, fins, covers, grille, four brackets, head on the built-in valve, H-block with two pipes |
| provenance | 8 references | + `purmo-ventil-compact`, `purmo-c22-600-1000`, `kermi-profil-k-22`, `thermostat-heads`; `radiator-catalogues` and notes rewritten (output 1.7 kW stated, 1.2 kW kept as calibrated value) |
| dimensions.depthM | 1.0 | 1.1025 (head 95 mm beyond the left end, plugs 7.5 mm beyond the right end) |
| dimensions.massKg | 26.18 | 35.78 |
| materials | painted-steel, brass | + `abs-plastic` (thermostat head, EN ISO 10456) |
| rig radiator mass | 26 kg | 35.6 kg |
| rig valve-head x, y | -0.025, -0.055 | 0.55, -0.05 (top of the left end, axis along the length) |
| anchors | `bracket-top` fixed at z 0 | `hook-left`/`hook-right` pins (u 0.465, wall, z ±0.4), `rest-left`/`rest-right` rollers normalDeg 0 (u 0.05, wall, z ±0.4) |
| anchors front-top-edge / grille-centre y | +0.1 / +0.05 | -0.1 / -0.05 |
| structure.supports | bracket-top | the four anchors above; assumption text rewritten (hanging on hooks, mass split) |
| thermal valve | "Thermostatic valve and supply pipe", 0.078 m2 | "Thermostat head, connection block and pipes", 0.052 m2 (factor 1.5) |
| thermal bracket / top-grille | 0.018 / 0.22 m2 | 0.024 / 0.27 m2 (four folded brackets, slotted grille with rim and lips) |
| thermal assumption 3 | valve and lockshield | supply through the built-in valve at the top, bands as before |
| 2D 1:50 | see audit | section: grille cap with lips, L brackets with wall flange and hook nose, head end-on (r 26 mm), H-block with cap, unions, pipe; zig-zag fitted to the fin span; bottom cap removed. Front: bands in thirds of the 585 mm panel, 25 mm grille lip, 29 channel seams and both header lines as texture, plugs at both ends, head with collar at the top of the left end, H-block with two caps, four unions and two pipes; fin tips, end seams, lockshield, second valve body removed. Plan: grille rim 103 x 1003 mm, slot rows as dashes (8 slots each), plugs, head beyond the left end |
| 2D 1:100 | idem | bands thirds of 0.57 m, 30 mm grille strip, head poly, H-block, two 15 mm pipes; head in plan |
| 2D 1:200 | idem | bands thirds of 0.6 m, grille strip removed (lip below the 30 mm feature), head, H-block, one pipe column; head in plan |
| decisions of 50/100/200 | old texts | silhouette, retained, merged, omitted (and thinElements at 100, overlays at 200) rewritten in en/de |

Side of the valve: the product's section A-A (`retained: near`) keeps the -w half, so the valve sits at -w, the left end seen from the room (Ventilkompakt with left-hand connection, a catalogue variant). Print suitability of all three drawings is met (`r3-panel-radiator-print.ts`: min gap 0.18 mm, min feature 0.21 mm at 1:50).

## 4. Model per scale

**1:50** (body 26 solids, thermal 31, structure 14; expanded body 40)
- body: each panel = channelled extrusion (30 channels, 29 seams 4 mm deep on both faces, 470-point section) between two rounded 40 mm headers; two trapezoidal fin sheets 0.6 mm, 25 mm pitch (texture, as the section draws them); folded U side covers; grille = 1.5 mm folded rim with 25 mm lips plus the slotted top (4 x 8 slots 8 x 100 mm, sheet generalised to the 7.5 mm feature); four folded L brackets (12 mm arm, 3 mm wall flange, hook nose on the upper ones); thermostat head (plastic, 52 x 82 mm, rounded cap) on its own bone, brass union collar; H-block with hexagon unions (AF 30 above, AF 27 below), nipples, two isolating caps, two 15 mm pipes to the floor; eight side-tapping plugs.
- thermal: rear panel + headers + side covers + rear plugs; front bands on the same channelled section (lower with bottom header, upper with top header) and their plugs; fins (exchange); grille rim and plate; brackets with the wall face as `sectionFaces +v` (C25 b); valve parts; emitter patches 7.5 mm proud on the grille (0.0284 m2) and the mid front.
- structure: both panels as hollow 1.25 mm sheet members, four connection nipples (joints), two hook joints, brackets (refs, they are the members), valve spindle joint, two wall screws (pins) and two wall flanges (rollers), masses of radiator and head.

**1:100** (body 10, thermal 14, structure 13): flat rounded panels to 0.57 m, fin-pack block, 15 mm end closures, 30 mm grille bar, 15 mm bracket arms, turned head, H-block with unions merged, two pipes; thermal bands thirds of 0.57 m; hollow panel members, nipples, hooks, supports at 15 mm, radiator mass.

**1:200** (body 6, thermal 13, structure 12): one rounded mass 0.6 x 0.1 x 1.0 m, four 30 mm bracket blocks, head, H-block, one pipe column; thermal split into rear 30 mm, convector core 40 mm, three front bands, grille top 30 mm; structure two 30 mm panel plates, nipple blocks, bracket refs, 30 mm supports, valve joint, mass.

**1:500** unavailable (representation unavailable, unchanged).

Final check line (`volumes-check.ts components/panel-radiator --product`): every rule ok at every scale (availability, silhouette, views-disagree, bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied) except `volumes-thermal-coverage` on the four water-panel regions only: panel-rear 1.451 against 0.6 m2, front bands 0.458 / 0.454 / 0.458 against 0.2 m2 (1:50), same at 1:100 and in the 1:200 total. `FAIL: 9 measurement(s) over 3 rule line(s)`. valve 0.046, bracket 0.0246, top-grille 0.302 m2 are within limits. `r3-validate.ts components/panel-radiator`: VALID (schema 0, validateAsset 0); with `--volumes` the same nine area findings.

## 5. What I saw in the final renders

- 1:50 body (`r3-sheet`, six views): front reads as a real profiled compact radiator: 30 vertical channels, top and bottom headers, plugs at the right end, white head on the top left, H-block with two caps and two pipes under the left end. Right side: panel, fin gap, panel, grille on top, head end-on in the upper corner, L brackets with flange and nose at the wall, H-block, union nuts, cap and pipe. Back: channelled rear panel, the four bracket flanges. Top: slotted grille in four rows, bracket arms in the wall gap, head beyond the end. Three-quarter: grille slots, side cover, headers, head; reads at first sight.
- 1:50 thermal cut: rear panel red, convector fins cyan between the panels, the three front bands green/purple/yellow on the channelled section, grille magenta with its slots, valve blue; plan cut shows the fin zig-zag in the horizontal section (true).
- 1:50 structure: two hollow panel members, orange nipples at the four header ends and hook joints, brackets blue with green screws and wall pads, magenta masses at the centre and in the head.
- 1:100 body and structure, 1:200 body and thermal: as described in section 4, proportions identical to 1:50, recognisable; clip `thermostat-cycle` t 5 s (right side) and pose `off` (2D) checked: the head turns about its own axis, nothing opens.
- 2D (`r3-2d`, 1:50 rated and off, 1:100 and 1:200 candidate): views agree with the bodies; plan head on the same side as in 3D.

## 6. Still imperfect (stated plainly)

- Water-panel areas (0.6 and 3 x 0.2 m2) count one face; the honest outer areas are about 1.45 and 0.43 m2 (factor 2.1 to 2.4, beyond the author's factor 1.5): listed in `area-review.md` (2026-11-20 panel-radiator). Until re-decided, `volumes-thermal-coverage` fails on these regions.
- Rated output 1.2 kW is about 30 % below the catalogue value of this size; it is a calibrated thermal datum and was not changed: request in `author-requests.md`.
- The 1:50 grille sheet (1.2 mm real) is modelled 7.5 mm thick so the plan silhouette sees it (a texture plate is left out of the silhouette); the slot flanks raise its area, hence 0.27 m2.
- Fin sheets are texture (0.6 mm); the thermostat head has no grip flutes (4 mm, below the feature); the side covers are bound to region panel-rear as the 2D binds them.
- The section `retained: near` keeps the -w half, against the description in `r3-tools.md`/C23 as I read it; I followed the product (the valve at -w is drawn and modelled), no tool request needed.
- Tests of `@teaching/staffage` were not run by me (fixtures `front-projection` radiator-outline -0.5..0.5 / 0.11..0.71 and the band regions are unchanged).
