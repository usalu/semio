# Round 3 — furniture/park-bench

State: finished and spliced (2026-11-20). No earlier round-3 work existed for this asset (no `r3-park-bench-*` scripts, no report). The bench was rebuilt from its roots on a real construction. Everything below describes the catalog file as it is now.

Scripts (ticket folder, run with the repo's pinned geometry python `~/Library/Caches/semio/staffage-oracles/geometry/bin/python3`, which has shapely):

| Script | What it does |
| --- | --- |
| `r3-park-bench-kit.py` | Measures, seat and post geometry, the cast frame profile built from hand-stated member strips (united and filleted), the I-section split, slat sections, extrude helpers |
| `-body.py` | Every body solid of every scale |
| `-lists.py` | Masses, thermal, structure, volume decisions |
| `-roots.py` | Rig, anchors, total mass, thermal layers and areas, references, notes, structural assumptions |
| `-draw.py` | 2D drawings and representation decisions of every scale (ruling E1) |
| `-build.py` | Writes the candidate document |
| `-apply.py` | Writes the root members into the catalog (`volumes` then goes in through `volumes-insert.py`) |
| `-sheet.ts` | Copy of `r3-sheet.ts` that reads the candidate with `--file`, used before the rig change reached the catalog |
| `-print.ts` | Product `checkAssetDetailed`, including print suitability, run on a candidate |

Re-running build and apply changes nothing ("members: none"). Scratch output under `🗑️generated/r3-park-bench/` was deleted. Renders stay in the tools' folders `🗑️generated/r3-sheet/furniture/` and `🗑️generated/r3-2d/furniture/`.

## 1. Reference

Subject: a 1.80 m public park bench with armrests. It has two one-piece cast end frames and hardwood slats bolted on with carriage bolts.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Length, height, depth | 1800 mm benches; 0.80-0.90 m high; 0.57-0.70 m deep | CIS Street Furniture Type 512 Eastgate (1800 x 570, seat 490); Kronemag bench 700 (1800, seat 440); NBS-listed cast seat 1800 x 850 x 630 | 1.80 x 0.85 x 0.60 m (asset dimensions kept) |
| Seat height | about 450 mm (UK guidance 470-480; catalogue benches 440-490) | BS 8300-1:2018; Inclusive Mobility 2021 §13; Broxap guidance | 0.45 m at the front edge, 0.41 m at the rear |
| Seat depth | 380-470 mm; at most 420-450 mm where there is a backrest | HSE HSG57 via Wheels for Wellbeing / Inclusive Mobility | 0.43 m (4 x 100 mm slats, 10 mm gaps) |
| Seat rake | 2-10° | US bench guidance (search summary) | 5° |
| Seat-to-back angle | 95-105° | same | 103° (post raked 8° from vertical) |
| Backrest | top at least 300 mm above the seat | BS 8300-1:2018 | slats to 0.85 m, 0.40 m above the seat |
| Armrests | about 200 mm above the seat, grip 32-50 mm, reaching at least 80 % of the way to the seat front | BS 8300 / Inclusive Mobility summaries | top 0.63 m (0.18 m above the seat front), 50 x 35 mm, reaching 58 mm beyond the seat nose |
| Slats | 40-50 mm hardwood, 80-120 mm wide, drainage gaps 8-20 mm, overhang 100-200 mm beyond the frames | catalogue practice | seat 100 x 45, back 80 x 35, gaps 10 / 30 mm (85 mm at the armrest), 6 mm eased edges, frames 0.12 m in from the slat ends |
| Fixings | M8 / M10 carriage bolts through each slat into the frame flange, nut underneath | carpentry practice | one M8 per slat and frame (14 bolts) |
| End frame | one-piece casting, flanged (I or T) section, front leg running up to the arm, raked seat rail, back post, rear leg kicked back, lower brace, foot pads | catalogue practice | I-section: 40 mm flange rims around the outline and both openings, 10 mm web; pads 70 x 70 x 16 mm |
| Materials | oak 700 kg/m³, steel 7850 kg/m³ | ISO 10456, Wood Handbook, EN 1993 (asset references) | masses taken from the modelled volumes |

Sources:
- CIS, NBS and Kronemag listings found through a web search: https://www.externalworksindex.co.uk/entry/172230/CIS-Street-Furniture-Limited/Type-512-Eastgate-Seat-cast-iron-and-timber-outdoor-bench/ , https://www.archiexpo.com/prod/kronemag-millenium-srl/product-90760-883088.html . Their pages refuse automated fetches (403), so the figures come from the search excerpts.
- Seating guidance: https://wheelsforwellbeing.org.uk/benches-and-seating-in-public-spaces/ , https://www.broxap.com/seating.html?p=21 .

Three new reference ids were added to `provenance.references`: `bs-8300-1-2018`, `inclusive-mobility-2021`, `cast-bench-catalogues`.

## 2. Audit of the state before round 3

**Rig**
- `ground` had a length of 0.1 m, which made it a member. The moments view drew a "Ground frame 1.53 kN·m" bar under the floor.
- `end-left` and `end-right` were horizontal bars 0.24 m long lying on the floor (x 0..0.24 at y 0). They should be what an end frame is: a column from the foot to the top of the back post.
- `seat` sat at y 0.405, under the slats. `backrest` sat at y 0.62, below the middle of the slats.
- The masses were nominal estimates: frames 12.6 kg at comY 0.3, seat 20.4, back 15.3, total 60.9 kg.

**Construction (1:50 solids, which the 2D matched)**
- Each end frame was a set of separate boxes and capsules: a 0.55 m flat foot plate, two lofted "legs", a box brace and a box bearer. There was no raked seat rail.
- The back post was a capsule standing on the bearer at z 0.19. It did not continue the rear leg.
- The arm post was a 32 mm box ending under a flat 90 x 34 mm box armrest, with a ball finial on the post.
- The seat was four flat boxes (100 x 45 mm, 30 mm gaps, depth 0.49 m) with no rake. Seat height was 0.45 m all the way back.
- The back slats were 80 x 33 mm extrusions. Their bolt "revolves" were open lines with no volume.
- Nothing tied the back slats to the armrest. The armrest ran into the slat zone.
- Seat-to-back angle: about 102°, with the seat flat.

**Structure**
- Members were `of` references to the body.
- A stretcher box sat on `ground`.
- Joints were the open bolt revolves.
- Masses were flat lenses in the open space between the legs, and the qa-visual grades criticised them. The 1:100 and 1:200 supports floated, because the legs stopped at the stretcher.

**2D**
- The front view drew the frame as a rectangle of separate legs on a full-depth foot plate.
- The 1:500 front drew a "0.16 m stem" that is wider than any real member.
- The 1:200 elevation had no seat gap.

**Thermal**
- The back slat layer was 40 mm against 33 mm slats.
- `steel-frame` was 1.07 m², an estimate.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `rig.ground.lengthM` | 0.1 | 0 (a reference, not a member) |
| `rig.end-left` / `end-right` | x 0 / 1.56, y 0, 0°, length 0.24; 12.6 kg at (0.12, 0.30) | x 0.12 / 1.68, y 0, 90° (vertical column from the foot), length 0.84 (top of the back post); 28.0 kg, comX 0.415 (height of the centre), comY 0 |
| `rig.seat` | y 0.405; 20.4 kg, comY 0.02 | y 0.409 (mean centre line of the four raked slats); 22.8 kg incl. 8 bolts, comY 0 |
| `rig.backrest` | y 0.62; 15.3 kg | y 0.677 (mean centre line of the back slats); 10.7 kg incl. 6 bolts, comY 0 |
| `anchors` contact-left/right | bone-local (0.12, 0), footprint 0.12 x 0.30 | (0, 0) at the same floor point in the new frame; footprint 0.07 (pad width along the bench) x 0.583 (front to rear pad) |
| `dimensions.massKg` | 60.9 | 89.5 (sum of the modelled bones); height, width and depth unchanged |
| `thermal.regions[seat-timber]` | areaM2 2.4, layer 0.04 m / R 0.222, Lc 0.42 | 2.05 (4 slats: rounded perimeter 0.278 x 1.8 + ends; factor 0.85, within C16), layer 0.045 m / R 0.25, Lc 0.43 |
| `thermal.regions[back-timber]` | 1.3, layer 0.04 / 0.222, Lc 0.30 | 1.20 (3 slats 80 x 35, perimeter 0.218 x 1.8 + ends), layer 0.035 / 0.194444, Lc 0.36 |
| `thermal.regions[steel-frame]` | 1.07, Lc 0.05 | 1.22 (outer area of the 1:50 castings, armrests, pads and bolt heads measured by `volumes-check.ts --product`: 1.216; factor 1.14), Lc 0.04; layer 0.01 m unchanged (the 10 mm web and walls) |
| `thermal.assumptions[1]`, `[3]` | 40 mm slats, 10 mm steel; "four foot plates 0.1 %" | 45 / 35 mm slats (24 mm active), 10 mm cast wall; four 70 x 70 mm pads (0.4 %) (en, de) |
| `structure.assumptions[0]`, `[2]` | rigid body, frames fixed; chamfers | each frame on a front and a rear pad, carrying the seat on its raked rail and the back on its post; gaps, eased edges and bolt heads are drawing detail (en, de) |
| `provenance` | 6 references, v2 notes | + 3 references; notes describe the construction and the dimensions (en, de) |
| `representations[*].geometry` + `decisions` | 53/33/21, 26/14/8, 6/11/6, 3/8/1 shapes | 21/12/15, 15/11/15, 7/4/7, 4/3/4 (elevation/front/plan at 1:50, 1:100, 1:200, 1:500), all redrawn (E1); decisions rewritten in 8 keys per scale (en, de) |

Unchanged: summary, title, tags, materials, poses (no bone motion; thermal fractions and loads kept), clips, contacts, structure supports, presentation, exports. The catalog index entry carries no mass. No other asset references the bench.

Areas: the area reviewers had decided seat 2.4 for "4 slats 1.8 x 0.11875" and back 1.3 for "0.08 x 0.05 slats". The honest construction (100 x 45 and 80 x 35 slats from the reference) gives 2.05 and 1.20. Both are within a factor 1.5 and were applied at the root, as C1/C16 allow; `area-review.md` needs no new row. The occupied-pose contact fractions (0.24 / 0.47 of the seat) were left to the area reviewers. They are now a slightly larger share of the smaller area.

## 4. What was modelled (solids listed / expanded with repeats)

### 1:50 (7.5 mm) — body 21 / 35, thermal 21 references, structure 23 / 34

**Body, per end frame:**
- web: an extrude of the 9-mm-shrunk profile with both grown openings as holes, 10 mm thick;
- three flange rims, 40 mm thick: the outer ring, plus one ring around the opening under the seat and one around the opening between rail, arm post, arm band and back post;
- an armrest: a superelliptic loft, 50 x 35 mm, with a four-station knuckle at the front, running 12 mm into the back post;
- two foot pads.

The frame members are:
- front leg 50 mm at the foot, 40 mm above it, rising straight into the arm post;
- seat rail 45 mm deep, raked 5° under the slats;
- back post 36 mm, raked 8°;
- rear leg 45 mm, kicked back to z 0.245;
- lower brace 30 mm at 0.15 m;
- arm band 35 mm.

Cast fillets are 8 mm.

**Body, slats and bolts:**
- 4 seat slats, 100 x 45 mm, as one repeated extrude along the rake;
- 3 back slats, 80 x 35 mm, on the post front faces, with a 30 mm gap above and an 85 mm gap below the middle slat. The armrest meets the post in that 85 mm gap, so it never touches a slat.
- The domed carriage-bolt heads (19 x 5.3 mm, `texture`) sit over each frame: 8 on the seat, 6 on the back.

**Thermal:** one reference per body solid with its layer (`oak-slat`, `cast-style-steel`). No hidden regions, no emitters.

**Structure:**
- Members: frame web and rims per end frame (`of`; the casting is the member, D3), seat slats, back slats.
- Joints: 14 carriage bolts as own revolves. Each has a head on the slat face, an M8 shank through the slat and the 9 mm flange, and a nut under the flange.
- Supports: the two pads of each frame, one solid per anchor, 7.5 mm.
- Masses: one sphere per bone at its modelled centre. The frame centre lies among the seat slats, so that marker is 20 mm and shows through the slat gap.

### 1:100 (15 mm) — body 10 / 15, thermal 10, structure 17 / 28

- Each frame is one 40 mm plate of the same outline and openings; the 15 mm web recess is merged.
- Straight armrest bar, pads kept.
- 4 square seat slats with their gaps, 3 square back slats; bolt heads omitted.
- Structure: plates and slats; Ø 15 mm bolts with heads and nuts; 15 mm pads; masses.

### 1:200 (30 mm) — body 5 / 5, thermal 5, structure 15 / 19

- Each frame is a 40 mm plate with members of at least 30 mm and legs to the floor with their feet; the pads are merged.
- The armrest is merged into the arm band.
- The seat is one raked 45 mm board.
- The back is the lumbar slat plus one board for the two upper slats; the 85 mm armrest gap stays open.
- Structure: one 30 mm bolt per board and frame, 30 mm pads, masses.

### 1:500 (75 mm) — body 4 / 4, thermal 4, structure 13 / 16

- Each frame is a 75 mm plate of 75 mm members: front column up to the armrest, seat rail, rear leg rising into the back, arm band. The opening under the seat and the gap under the armrest stay.
- The seat is a flat 75 mm slab (the 38 mm rake is below the feature).
- The back is one 75 mm slab, posts and slats together (36 + 35 mm), raked 8°.
- Structure: one 75 mm joint where the seat and where the back cross each frame, 75 mm pads, 55 mm masses.
- `decisions` declare the model minimal.

`decisions`: two bilingual sentences per scale (body; thermal and structure).

## 5. Checks (final, catalog asset)

- `volumes-check.ts furniture/park-bench --product`: `PASS (13.4 s)`, all 39 rule lines ok.
- Silhouette IoU elevation / front / plan, with p95 0 everywhere except 15 mm in the 1:200 plan (limit 120):

| Scale | Elevation | Front | Plan |
| --- | --- | --- | --- |
| 1:50 | 0.999 | 0.999 | 1.000 |
| 1:100 | 0.997 | 1.000 | 1.000 |
| 1:200 | 0.999 | 0.999 | 0.994 |
| 1:500 | 1.000 | 1.000 | 1.000 |

- Bounds deviation at most 3.4 mm.
- Thermal outer area against `areaM2`:
  - 1:50: seat 2.046 (+0.2 %), back 1.201 (+0.1 %), steel 1.216 (−0.4 %).
  - 1:100: seat +3.6 %, back +4.9 %, steel −21.8 % (the web recess merged into a plate; limit 30 %).
  - Totals: 1:200 −12.7 %, 1:500 +7.6 %.
- `r3-validate.ts furniture/park-bench --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Product print suitability (`r3-park-bench-print.ts`): every scale suitable. Minimum gap at 1:50 is 0.22 mm, after giving the slat outlines the 0.35 mm pen and widening the upper back gap from 30 to 32 mm (as first drawn, the 1:50 elevation closed it to 0.04 mm).
- The catalog `volumes` equal `volumes/furniture/park-bench.json`.

## 6. What the final renders show (opened with the Read tool)

Opened:
- `r3-sheet` body sheets (six directions plus rig) at 1:50, 1:100, 1:200, 1:500;
- a 1:50 close three-quarter (turn 30, tilt 20) and a view from behind (turn 210, tilt −25);
- 1:50 structure (twice, before and after the joint and mass fix) and 1:50 thermal cut;
- the 1:500 front alone;
- `r3-2d` candidate sheets at 1:50, 1:200, 1:500;
- `r3-2d` 1:50 forces and moments in `occupied-two`.

Rendered but not opened: the 1:100 and 1:200 thermal and structure sheets, 1:500 structure and thermal, the final catalog `r3-2d` 1:100/1:200 sheets, and the clip frame `occupant-sit-down` t 1.5. The bones do not move in any pose, so that frame equals the default.

**1:50, front view (end-on, from +x).** It reads as a cast bench end:
- the front leg rising straight into the arm post, under a rounded armrest with its knuckle ahead of the seat nose;
- the raked seat rail carrying the four slat ends;
- the back post raked behind them, with the lumbar slat and the two upper slats;
- the armrest ending in the gap between them;
- the rear leg kicked back to its pad, and the brace low between the legs;
- the flange rim reading as a raised border around both openings.

**1:50, elevation (from the front of the bench):** the slat bands stepping down with the rake, the three back slats, the arm posts and armrests in front of the slats at both frames, and the pads.

**1:50, plan:** four seat slats with drainage gaps, back slats stepping back, armrests over the frames, the post tops behind the back.

**1:50, three-quarter and from behind:** a credible cast-iron-and-oak park bench. It has eased slat edges and bolt heads over each frame. The back slats sit on the post faces, the I-section shows on legs, rail and brace, and the pads are under every foot. Nothing floats or pierces another part. The armrest stays clear of the slats.

**1:50 structure:**
- blue frames and slats;
- orange bolt heads on the seat and on the back slats over the frames;
- green pads under all four feet;
- magenta masses on the seat (through the gap), on the back, and in each frame.

**1:50 thermal cut:** red seat, green back, violet steel.

**Coarser scales:**
- 1:100 is the same bench in plain plates and square slats.
- 1:200 has the seat as one board and the back as two boards with the armrest gap.
- 1:500 reads at once as a bench with armrests: a clean frame of 75 mm bands, seat slab and raked back slab.

**2D (product drawings):**
- 1:50 front: the casting outline with the recessed web drawn as a second tone, the armrest, the pads and the rounded slat sections.
- 1:50 elevation: the slat bands, the frames edge-on, the bolt circles on the back slats.
- 1:500: a 3.6 mm sign.

**Moments (occupied-two):**
- The floor-reference bar is gone.
- The end frames are vertical columns.
- The seat bar still shows 1.53 kN·m at its joint (see section 7).

## 7. Still imperfect

- **Seat moment in the analysis.** The product's tree solver cantilevers every bone from its joint to its parent. The seat slats therefore show 1.53 kN·m at x 0 under two occupants. The real slats span 1.56 m between the frames, and the real moment at the load points is about 0.28 kN·m. A tree rig cannot express a beam on two frames, so this was not changed. It belongs to the structural solver, not to this asset.
- **Mass markers.** The end-frame centre of mass lies in the frame plane among the seat slats (y 0.415 m). Its 20 mm marker is mostly hidden by slat 3 and shows only through the slat gap. A larger marker breaks `volumes-structure-coverage` (more than one feature outside the body). The 1:50 seat and back markers are 22 mm for the same reason.
- **1:100 steel area −22 %.** The 15 mm web recess is below the 1:100 feature, so the plate hides the recess walls. This is within the 30 % limit.
- **Bolt heads** exist at 1:50 only, as `texture`. Nuts and washers exist only as part of the structural joints.
- **Armrest gap.** The lumbar slat sits 85 mm below the middle slat, so that the armrest meets the post without touching a slat. This is constructively right, but the back reads with one larger gap.
- **Frame mass** (28 kg each, 89.5 kg total) follows from the modelled solid steel armrest and 40 mm flanges. That is at the heavy end of real cast benches, which run 60-95 kg. A hollow armrest would save about 4 kg per frame.
- **References.** The catalogue figures came from search excerpts, because the manufacturer pages refused automated fetches (403).
- **Not run.** The repo test suites were not run, only the checks of the brief.
