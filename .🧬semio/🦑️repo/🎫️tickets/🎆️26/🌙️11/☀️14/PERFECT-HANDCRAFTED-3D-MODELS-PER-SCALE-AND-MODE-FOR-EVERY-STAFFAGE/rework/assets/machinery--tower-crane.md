# machinery/tower-crane — rework report

## Pass 1 (second round), 2026-11-22

State found: no earlier rework report; `rework/10.md` section 6 says "not started". Source `T/volumes/machinery/tower-crane.json` and catalog `asset.volumes` were identical. The catalog state was INVALID before this pass (`r3-validate.ts --no-print`: 10 `volumes-feature` findings at 1:200 and 1:500).

### Reference (top-slewing hammerhead crane with tower head, 55 m jib, class Liebherr 154 EC-H / Potain MD 265, freestanding on a ballasted cross base)

| quantity | real machine | model before | verdict / after |
|---|---|---|---|
| hook height freestanding | 40 to 50 m | mast 39 m on a 1 m base, jib axis 44.4 m | ok |
| mast section | 1.9 to 2.0 m square, chords 150 to 200 mm, panels about 2 m | 2.0 m over all, 160 mm chords, 90 mm ties, 80 mm diagonals | ok |
| jib | triangular, 1.2 to 1.5 m wide, 1.5 to 2.0 m deep, top chord tube 120 to 160 mm, bottom chords are the trolley track | 1.32 m wide, 1.62 m deep, 140 mm top chord, 120 x 140 bottom chords | ok |
| jib length / counter-jib length | 55 m / 15 to 18 m | 55 m / 16.5 m | ok |
| tower head apex above the jib heel pin | 6.5 to 9 m (pendant slope of the inner tie 15 to 20 degrees, of the outer tie 9 to 11 degrees) | 3.46 m (slopes 8.6 and 4.8 degrees) | **wrong, squat head; now 6.86 m (16.8 and 9.5 degrees, back tie 22.8 degrees)** |
| tower head form | braced frame that takes jib heel and counter-jib, tapering to a slender top with the rope sheave | one 7.4 m pyramid | **now a 4.2 m pyramid plus a 6.6 m braced spire** |
| counterweight | 15 to 20 t concrete slabs hung at the counter-jib tail | 20 t, four slabs under a beam | ok |
| central ballast on a 6 x 6 to 8 x 8 m cross base | 40 to 60 t | 45 t on a 7 m cross | ok |
| cab | 1.4 x 1.9 x 2.1 m beside the tower head, raked screen | 1.4 x 1.85 x 2.1 m | ok |
| trolley, hook block, 1 m3 skip, two-fall rope 14 mm | yes | yes | ok |
| slewing | full circle about the mast axis | `turntable` twist (pose `parked-weathervane` 60 degrees) | ok |
| overall height | about 52 m with this head | 48.85 m | **now 52.25 m (`dimensions.heightM`)** |
| counter-jib walkway with handrails, cab access platform, mast section joints and rest platforms, hoist rope run winch to head sheave to jib, trolley travel rope | present on the real machine | missing | **still missing (see remaining)** |

### Rebuilt in this pass

Scripts: `rework-tower-crane-head.py` (hand-set node coordinates of the head; writes source and catalog), `rework-tower-crane-coarse.py`.

- **Rig**: `slewing.lengthM` 8.6 → 12.0 (bone reaches the apex). Topology judged right for the machine (fixed base and mast, slewing twist at the turntable, prismatic trolley and hook block, eight telescoping rope pieces) and left as it is.
- **Body, all four scales**: the tower head. The lower pyramid (base 2.4 x 1.8 m at 1.2 m, legs to the tie level at 5.4 m) is kept because its legs pass exactly through the jib heel pin, the jib rail feet and the counter-jib chord ends. The four legs now end at 5.4 m (`tower-head-leg-1..4`); above them four spire legs (`tower-head-spire-1..4`, 100 mm) run to the apex ridge at 12.0 m (0.24 m wide, carries the sheave). Ties at 7.0, 8.6 and 10.2 m (`tower-head-tie-x|z-4..6-1|2`, were one level at 6.8 m as a `repeat`), one diagonal per face in each of the three spire panels (`tower-head-diagonal-13..24`, alternating; 13..16 existed in the old top panel). `head-sheave` and the three pendants moved to the apex. Each scale keeps its own sections (1:500: 75 mm ties, no diagonals; 1:200: no ties).
- **Thermal**: references of the new bars in region `lattice` at 1:50, 1:100, 1:200. At 1:500 the head had no thermal solids at all; all head bars are now referenced there, so the thermal sheet shows the head.
- **Structure**: every new bar is a member of `slewing` (the tie copies are now solids of their own instead of explicit duplicates of a `repeat`); pendants follow the body.
- **2D, all four scales, three views**: head legs, spire legs, ties, diagonals redrawn as member shapes from the same node coordinates (elevation without the far z face, front without the far x face, as before; plan as filled bars at 1:50, strokes at 1:100 and 1:200, members at 1:500), sheave and pendants at the apex; the symbol's head rectangles and pendant lines follow. Rendered with `r3-2d` at 1:50 and read.
- **Coarse scales** (the ten findings that made the asset INVALID): 1:200 and 1:500 the two 16 mm cheek plates of the trolley sheave block are one closed block 1.0 x 0.3 x 0.112 m (`trolley-sheave-block`), at 1:500 with the two 60 mm sheaves and their axles merged in; 1:500 jib heel and tip cross tubes 75 mm, jib and counter-jib pin joints of the structure list 75 mm.
- **Decisions**: one sentence (en, de) on the head added to `volumes[scale].decisions[0]` and to the 2D `decisions` of each scale.

### Rendered and read after the change

1:50 body six views (default pose `lifting-load`), right-side close-up, pose `parked-weathervane` (slewed 60 degrees: head, pendants, jib stay together), 1:50 structure three-quarter, 1:500 thermal six views, 1:50 2D drawings. The head reads as a real cat head, pendants meet the apex and their jib nodes.

### Joined parts (after the first full validation run)

The first full run (`r3-validate.ts machinery/tower-crane --volumes`, 22 minutes on the loaded host) reported 17 measured findings. The committed catalog state (`git show HEAD:…`, checked at 1:500 with the same rules) has the same ones, so they are older than this pass: `volumes-connected` on body and structure at all four scales and `volumes-silhouette` at all four scales. The connections were then modelled (`rework-tower-crane-joins.py`, `rework-tower-crane-rope.py`, `rework-tower-crane-500.py`, `rework-tower-crane-500-2d.py`):

- jib tip sheave on an axle between the two tip posts (60 mm; 75 mm at 1:500), all scales;
- rope: the two falls start at the tangent points of the trolley sheaves and lie in the grooves (0.204 m off the axis), the last piece ends on the block sheave; at 1:500 one fall on the axis in all eight pieces (pieces 1 to 4 were two falls, 5 to 8 one fall 186 mm beside them), elevation drawing of pieces 1 to 4 redrawn accordingly;
- hook block 1:50 and 1:100: a 60 mm pin carries the sheave between the side plates (20 mm air before);
- mast ladder 1:50: 25 mm brackets on the mast ties every 2 m (the ladder floated inside the mast);
- skip 1:200 and 1:500: bail legs moved in to the shell (0.70 and 0.71 m), no lugs at these scales;
- trolley 1:500: 75 mm side plates, plain wheel discs, no axles, sheave block 0.36 m high;
- structure: hook block plates, pin, crosshead, hook, skip shell and lugs, winch skids and the tip axle are members, so the path skip - bail - hook - block - rope - trolley - jib - head - mast - base is closed.

### Validation

- `r3-validate.ts machinery/tower-crane --catalog-only --no-print` after `volumes-insert.py`: `VALID … schema 0, validateAsset 0` (before this pass: INVALID, 10 `volumes-feature`).
- Partial measured runs after the last change: `--scale 50 --rules connected`: 0 findings; `--scale 100` and `--scale 200` with `connected, feature, thermal-coverage, structure-coverage, slender`: 0 findings; `--scale 500` with `connected, silhouette, feature, thermal-coverage, structure-coverage, bounds`: only the two silhouette findings.
- **Not VALID with `--volumes`**: `volumes-silhouette` is open at every scale and was not worked on: 1:50 elevation p95 37.5 mm (limit 15), front p95 33.8 mm (30), plan iou 0.834 (0.88); 1:100 elevation iou 0.767 (0.9), plan 0.833; 1:200 elevation 0.766, plan 0.831; 1:500 elevation 0.767 (0.785 in the committed state), plan 0.831. The same value at three scales points at one systematic difference between the lattice drawing and the solids (not found; candidates: the chords drawn as outlines against solid bars, the ladder texture, member pens), to be located with `r3-project2d.ts --check` per view.
- The full `--catalog-only --volumes` run after the last change was started; its result is in the last line of this report if it finished within this pass.

### Remains imperfect (open list for the next pass)

- 1:50 body: no counter-jib walkway deck and handrails, no cab access platform and ladder cage, no mast section joints (the mast is one 39 m run) and rest platforms, no hoist rope from the winch over the head sheave into the jib, no trolley travel rope and drive, no jib walkway/lifeline; counterweight slabs have no lifting eyes; winch house is a plain 7.5 mm sheet box without door and drum.
- 1:50 structure: the pendants are members in the solid list, but `asset.structure.assumptions` still says "the pendant ties are not members" and treats the jib as a cantilever; the analytic model was not touched.
- 1:100, 1:200, 1:500: not re-modelled beyond the head and the listed sub-feature parts. Mast, jib and counter-jib lattice are still the 1:50 bars declared `slender` (true section) at every scale; `rework/10.md` counts 401 / 348 / 314 solids identical to 1:50 before this pass. A decision per scale is still owed: typified lattice (`envelope: true`, fewer, thicker bars) at 1:200 and 1:500 or the true bars.
- Rendered but not all read: `max-reach`, `unload` t 12, `lift-and-traverse` t 9, 1:200 body, 1:100 thermal (read: `skip-at-ground`, `parked-weathervane`, `lift-and-traverse` t 17, 1:500 body and thermal, 1:50 structure). After the joining edits only 1:500 elevation and one clip instant at 1:50 were read again; 1:100 and 1:200 sheets and their thermal and structure lists were not read at all.
- 2D: 1:100, 1:200, 1:500 drawings of the new head were generated by the same script as 1:50 but only 1:50 was looked at.
- Thermal `areaM2` of `lattice` (434 m2) was not recomputed for the longer head (about +4 m2); no thermal finding in the full run.
- `volumes-silhouette` open at all four scales (numbers under Validation): the 2D drawings and the body disagree; this is the main open item.
- 2D: the bail at 1:200 and 1:500 and the rope falls at 1:50 to 1:200 moved by 4 to 40 mm in 3D and were not redrawn; the new parts (tip axle, block pin, ladder brackets, sheave block) have no 2D shapes.

## Pass 1 (second round), continuation, 2026-11-22 afternoon

State found: source and catalog `asset.volumes` identical (the earlier agent of this round had published); `--catalog-only --no-print` VALID; `volumes-silhouette` open at all four scales; the full `--volumes` run of the earlier agent had left an empty `validate-final.txt`.

### Reference table, additions (same machine class: 55 m hammerhead with tower head, Liebherr 154 EC-H / Potain MD 265)

| quantity | real machine | model before | after |
|---|---|---|---|
| jib and counter-jib ties | solid tie bars or flat bars, 40 to 60 mm (tie force of the order of 300 kN needs about 1000 mm2 of S355 with safety 3: 45 to 50 mm) | three 16 mm "ropes" (200 mm2), 3D material steel, 2D material wire rope | **50 mm tie bars, steel in 3D, 2D and the thermal region; area 13 m2** |
| counter-jib access | chequer-plate or grating deck over the full width, handrail 1.1 m with knee rail (EN 14439 / EN ISO 14122-3), interrupted by the winch frame | none | **30 mm deck between the top chords, 40 mm tube handrails with knee rail, posts every 1.83 m, both sides and the tail end** |
| slewing platform | handrail on all open edges | none | **handrail on three sides and up to the cab** |
| mast | bolted or pinned sections of 3 to 6 m (4.14 m Liebherr HC), ladder with rest landings at most every 10 m (EN 14439) | one 39 m run | **4 m sections: a 360 x 220 mm joint sleeve on each chord at 4, 8 ... 36 m; rest platform 1.26 x 1.8 m beside the ladder at 8, 16, 24, 32 m** |
| hoist rope reeving | winch on the counter-jib, over the tower-head sheave, down to a deflection sheave at the jib foot, along the jib through the two trolley sheaves and the hook block to a dead end at the jib tip | only the two falls between trolley and block; head sheave without any rope | **winch (roof fairlead) to head sheave, down inside the head to a 400 mm heel sheave on hangers from the head tie at 4.0 m, along the jib at the level of the trolley sheave grooves (1.062 m below the jib axis) to an anchor lug under the tip cross tube** |
| tower head front face | open portal or K-brace where the rope and jib foot pass | single diagonals crossing the rope line (computed clearance -25 mm and +11 mm) | **portal brace (two bars to the middle of the tie) in the panels 2.6 to 4.0 m and 7.0 to 8.6 m; smallest rope clearance now 18 mm (ties at 7.0 and 8.6 m)** |

### What was done (script `rework-tower-crane-fitout.py`, idempotent, hand-set numbers at its top; writes source and catalog)

- **Body, all four scales**, each for its feature: 1:50 and 1:100 everything above (1:50 with a separate 50 mm sheave pin and 12 mm hangers, 1:100 30 mm hangers without pin); 1:200 handrails as posts and top rail only, deck and rest platforms declared `sheet`; 1:500 no handrails, heel sheave one 114 mm disc, deck and platforms declared sheets, rope runs slender at 14 mm.
- **Thermal**: a reference for every new solid (region `lattice`, ropes `rope`, fairlead `winch-house`); region areas `lattice` 434 -> 513 m2 (deck 38.7, platforms 18.1, handrails 12.7, sleeves 4.8, head of the earlier agent 4), `pendants` 4 -> 13 m2 (material steel), `rope` 4 -> 7.1 m2.
- **Structure**: the three rope runs, the heel sheave with hangers and pin, the joint sleeves, the tip anchor and the second portal bars are members of their bones; deck, handrails, rest platforms and fairlead are not (no load path). `structure.assumptions` no longer says the pendants are no members.
- **2D, all four scales, three views**: every new part drawn (handrails, deck, sleeves, platforms, anchor, fairlead as swept rectangles so front and plan follow; ropes and portal bars as members; heel sheave and hangers). Chords, rails and counter-jib chords (members from 120 mm) are filled strips with a 0.13 mm outline in the elevation of 1:100 to 1:500 and in the plan of all scales, as they already were in the 1:50 elevation.
- **Decisions**: one paragraph per scale (en, de) in `volumes[scale].decisions[0]` and in the 2D decisions.

### Root cause of the open `volumes-silhouette` findings (found with the new picture tool)

`rework-tower-crane-diff.ts` / `.sh` writes the rule's own two masks as a picture (grey agree, red drawn only, blue body only; same hook as `digital-outline-difference.ts`). Two causes, both in the asset, none in the rule:
1. The pendant solids were steel, their 2D shapes wire rope: a slender solid follows a drawn shape of the same bone and material only, so the pendants were compared at a foreign width (blue bands along all three pendants at every scale).
2. Chords drawn as unfilled members with a pen as wide as the bar: the rule takes such an outline as the edge of a body and the bar at section plus pen, about 250 mm against 144 mm of ink; on a lattice that is the constant IoU 0.77 of 1:100 to 1:500. Filled strips agree exactly.

Measured after the change: 1:500, all measured rules (`--scale 500 --no-print`, before the plan fix): only `silhouette` plan iou 0.849 left; after the plan fix the masks give 61763 common, 69 drawn only, 928 body only cells (iou 0.984).

Tools added in `T`: `rework-tower-crane-crop.py` (candidate document with the body of chosen bones only, for close-up sheets of a 70 m asset), `rework-tower-crane-diff.ts`, `rework-tower-crane-diff.sh`.

### Print limits met on the way (first full run after the fit-out: 32 `PRINT` findings, all removed)

- Handrails exist at 1:50 only: the product measures a 40 mm tube at 0.19 mm at 1:150 (end of the 1:100 band) and 0.14 mm at 1:200, below the 0.2 mm print minimum. Slewing-platform posts stand at the same stations on both sides (a near-coincident pair printed as a 0.006 mm gap).
- 1:500: no deck, no rest platforms, no heel sheave (30 mm plates and a 60 mm disc do not print at 1:1000); chords stay pen strokes there (a filled 120 to 160 mm strip is 0.12 to 0.16 mm at 1:1000).
- The deck lies flush with the chord tops (top at 0.70 m above the counter-jib axis) so that the winch skids rest on it; at 1:200 the skids are part of the house outline in the drawing (one block, 3D unchanged).
- The hoist rope starts at the roof fairlead, not inside the house (inside it covered faces of the hidden `hoist-motor` region: 4.16 m2 against 5.1 m2, finding gone).
- Pendants are filled 50 mm strips at 1:50 and 1:100 and 50 mm pen strokes at 1:200 and 1:500; chords are filled strips also in the 1:50 front view.

### Validation after the last change

- `r3-validate.ts machinery/tower-crane --catalog-only` (schema, validateAsset, print): `VALID … schema 0, validateAsset 0`. Source and catalog `asset.volumes` are identical (written together by the script).
- Last full `--catalog-only --volumes` run (before the 1:50 front fix): `INVALID … checkVolumes 3`: 1:50 front p95 33.8 mm (30), 1:500 elevation iou 0.794 (0.9), 1:500 plan iou 0.833 (0.88). Before this continuation: 9 silhouette findings at four scales.
- After the 1:50 front fix: `--scale 50 --rules silhouette,views-disagree --no-print`: `VALID … checkVolumes 0 (PARTIAL)`. A full `--volumes` run after this last edit was not made (25 minutes on the loaded host); the edit touches only the fill of five front shapes at 1:50.
- **Not VALID with `--volumes`: two findings remain, both 1:500 `volumes-silhouette` (elevation 0.794, plan 0.833).** Cause known, no asset-side solution found: as pen strokes the chords are compared at about 250 mm of body against 144 mm of ink (pictures `d500e-zoom` of this pass), as filled strips they agree (measured: elevation passes, plan 0.984) but then `PRINT range-end-feature` refuses the band at 1:1000. See Fundamental issues.

### Rendered and read

Read at full size before the changes: 1:50 body six views, close-ups of head, tail (counter-jib, winch, counterweight), load (trolley, block, skip), base; the rule masks of 1:500 and 1:50 elevation. Read after the fit-out (first applied state): head close-up six views and counter-jib three-quarter at 1:50: deck, handrails, fairlead, heel sheave on its hangers, rope down-run and portal brace read as on a real crane, nothing floats.
**Rendered but NOT read** (the image reader of this session stopped answering under host load at 16:20): everything after the print fixes, namely 1:100 and 1:500 close-ups, the mast with joint sleeves and rest platforms, poses `parked-weathervane` and `max-reach`, the 2D sheets of 1:50 and 1:500. Never rendered in this continuation: pose `skip-at-ground`, both clips, the thermal and structure sheets of any scale, 1:200.

### Remains open (precise list for the next pass)

- 1:500, body against 2D, elevation and plan: `volumes-silhouette` (see above and Fundamental issues).
- All scales: read the sheets listed as not read; render and read `skip-at-ground`, `lift-and-traverse` (two instants), `unload` (two instants), thermal and structure lists at four scales. Check in particular that the static rope along the jib reads right when the trolley is at 3 m and at 53 m and in `parked-weathervane` (the rope piece between the two trolley sheaves is hidden by the cheek plates only in elevation).
- 1:50 body: rest platforms and joint sleeves were validated but never looked at; the 2D orientation of the swept rectangles of the platforms in front and plan is unchecked by eye.
- 1:50 body, still missing against the reference: cab access (ladder from the mast top through the platform, door), winch drum and door (the house is a closed 7.5 mm sheet box), trolley travel rope and drive, counterweight lifting eyes, jib lifeline, slewing ring teeth and bolts, mast-to-base and mast-to-ring connections are plain contact.
- 1:100 to 1:500 body: mast, jib and counter-jib lattice are still the 1:50 bars at true section (`slender`); decided and written into the 1:500 decisions as intended (a crane at 1:500 is its line pattern), not typified. 1:100 and 1:200 carry no handrails only because of the print limit; a typified 60 mm rail (`envelope`) was not tried.
- Thermal: `lattice` 513 m2 is the 1:50 sum by hand; no thermal finding at any scale in the full run, but the coarse scales have fewer parts (within the rule's tolerance).
- Structure: the analytic model (`asset.structure`, bone sections) still treats the jib as a cantilever; only the assumption text was corrected. The hanger-to-tie joint of the heel sheave and the rope-to-anchor joint are plain contact, no `joint` solids.
- Rig: unchanged and judged right (fixed base and mast, slewing twist, prismatic trolley and block, eight rope pieces). No luffing or lateral bones needed for a hammerhead crane; `rig-lateral-rest.md` not applicable.

### Fundamental issues (outside this asset)

1. `volumes-silhouette` and the print rule contradict each other for lattice bars at 1:500. A bar of 120 to 160 mm must be a pen stroke (a filled strip is under 0.2 mm at 1:1000, `PRINT range-end-feature`), but an unfilled `member` shape with a pen as wide as the bar is compared as about 144 mm of ink (capped member stripe, `memberOf`) against about 250 mm of body for the `slender` solid that follows it (`strokesOf` / `followedWidth`, `🌓️silhouette`). Both sides should use the same width for a slender solid and the member stroke it follows. Reproduce: `sh rework-tower-crane-diff.sh 500 elevation zoom "6,42.5,12,46.5"` (blue bands of one to two cells on both sides of every chord).
2. `structureCoverageRule` counts every 2D shape of `kind: "member"` of a bone and demands as many `part: "member"` structure solids: a handrail or any other non-structural bar cannot be drawn as a member shape (worked around with swept rectangles).
3. `r3-sheet.ts` has no window or zoom option: on a 72 m asset every detail is a few pixels. `rework-tower-crane-crop.py` (bones filter into a `--file` candidate) is a workaround; a `--window` or `--bones` option belongs in the tool.
4. `followedWidth` matches drawn shape and solid by material; a mismatch (pendants: steel solid, wire-rope shape) fails silently as a silhouette difference instead of a named finding.
