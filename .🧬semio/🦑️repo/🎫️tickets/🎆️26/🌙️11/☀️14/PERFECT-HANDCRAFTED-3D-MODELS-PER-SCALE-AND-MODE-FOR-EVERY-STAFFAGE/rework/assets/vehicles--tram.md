# vehicles/tram — rework report

## Pass 1 (second round), 2026-11-22

State found: no earlier report for this asset; `rework/09.md` lists it but has no section on it. Source `volumes/vehicles/tram.json` and catalog `asset.volumes` were identical (238/214/201, 143/115/112, 104/95/107, 55/65/83 solids for body/thermal/structure at 1:50/100/200/500). The asset is a *section* of a 100 % low-floor articulated tram (title "Low-Floor Tram Section"): cab end module on a powered and a trailing bogie, bellows, suspended middle module cut at its far end. The cut end is intended, so the missing third bogie is not a defect.

### Reference table (own knowledge: BOStrab / VDV 154, EN 13803, EN 50163, manufacturer-typical values for 100 % low-floor cars of the Combino / Flexity / Citadis class)

| Quantity | Reality | Model | Verdict |
| --- | --- | --- | --- |
| Car body width | 2.30 to 2.65 m (BOStrab max. 2.65) | 2.40 m | right |
| Roof height over rail (without equipment) | 3.3 to 3.6 m | 3.40 m, units to 3.65 m | right |
| Contact wire, pantograph working range | 4.2 to 6.0 m, nominal about 5.0 to 5.5 m | head at 5.00 m, arms 1.9 m each | right |
| Floor height low-floor area | 300 to 350 mm | 280 to 350 mm floor plate, entrance 300 mm | right |
| Wheel diameter (new) | 560 to 660 mm | 600 mm with flange 22 mm | right |
| Bogie wheelbase | 1.6 to 1.9 m | 1.80 m | right |
| Track gauge / wheel back-to-back | 1435 mm / about 1380 mm | contact points at z = 0.715 m (1430 mm), wheels 0.69 to 0.825 m | right within a tread width |
| Wheels | independent wheels on stub axles or portal axles | stub axles, cranked cross members under the aisle | right |
| Drive | longitudinal motor per side driving both wheels through bevel gearboxes (Combino type) | exactly that on the powered bogie, axle boxes on the trailing bogie | right |
| End module length | 7 to 10 m with cab | 9.0 m (0.45 to 9.4) | plausible |
| Suspended module length | 3 to 7 m | 5.55 m | plausible |
| Bellows length | 0.6 to 1.0 m | 0.9 m | right |
| Door clear width (double leaf) | 1.3 m | 1.30 m, leaf 0.65 x 2.15 m | right |
| Door type | plug-sliding: leaf moves 40 to 60 mm outwards, then slides along the wall | slides 0.55 m in the plane of the wall, no plug stroke | **defect, open** |
| Windscreen rake | 10 to 25 degrees | 17 degrees | right |
| Primary suspension | rubber spring between axle box / gearbox and side beam, in contact with both | powered bogie: touching; trailing bogie: 40 mm air gap between axle box and spring | **defect, fixed** |
| Secondary suspension | springs between bogie frame and car body bolster | present at 1:50 only | **defect at 1:100, 1:200, 1:500 (structure), fixed** |
| Pantograph base | base frame on four insulators over reinforced roof members; shaft in two bearings; raising drive on the frame; coupling rods pinned at both ends | frame and insulators there, but drive cylinder and lower coupling rod floated free; structure frame floated above the roof at every scale | **defect, fixed** |
| Articulation | lower slewing bearing under the gangway carrying the suspended module, upper link at roof level | lower arm 10 mm under both floors without contact; upper link 50 mm from both end rings | **defect, fixed** |

### Defects found (measured, default pose)

Listed with `rework-concrete-mixer-truck-gaps.ts` (groups of touching solids as the product rule `volumes-connected` forms them) and `r3-validate.ts --volumes` (11 findings at the start):

- Body 1:50: 7 floating groups — articulation arm and bearing (10 mm under the floors), `pan-lower-rod` (34 mm from the knee, free at both ends), `pan-drive` (free between the frame cross members), the four trailing-bogie axle boxes (40 mm under their primary springs).
- Body 1:100 and 1:200: the four gearboxes 20 mm under the side beams with nothing between; 1:100 articulation arm as at 1:50; trailing bogie at 1:100 had no axle boxes at all (250 mm parts at a 15 mm feature).
- Structure 1:50: 17 loose groups — the whole middle module (only tied through the floating articulation arm), pantograph mechanism (shaft 70 mm inside the frame rails, no bearings in the list), pantograph frame (nothing under it), upper articulation link, the four door leaf frames (11 mm from their carriages), every wheel with journal and bearing (5 mm from the beam, axle box / gearbox and primary spring missing from the list).
- Structure 1:100: 24 loose groups — pantograph, head strips, cab ring and posts (260 mm above the sills, nothing below), both bogie frames (no secondary springs, no bolsters), every wheel, roof bows 2 to 4 and mid bows 2 and 3 (40 mm above the cant rails), articulation, four door leaves (no header, no hangers), both bogie pivots (290 mm from the cross members). `end-cross-bearer-2` and `-4` ran straight through the wheels of the trailing and the powered bogie.
- Structure 1:200: 13 loose groups of the same kinds; the single bogie cross member hung 220 mm below the side beams without meeting the legs.
- Structure 1:500: bogie blocks, pantograph base, cab ring, one roof bow.
- Silhouette 1:50 against the front drawing: p95 31.9 mm (limit 30).

### Rebuilt by hand (scripts in `T`, every number chosen against the dumped geometry)

`rework-tram-loadpath.py`, `rework-tram-axleboxes-100.py`, `rework-tram-trim.py`, `rework-tram-rod.py`, `rework-tram-decisions.py`; helpers `rework-tram-dump.py` (one-line dump) and `rework-tram-cand.py` (candidate document).

Body
- 1:50 and 1:100: articulation arm 85 mm deep, seated 5 mm into both floor plates.
- 1:50: trailing-bogie primary springs 90 mm high, down onto the axle boxes (0.115 to 0.205 m over the axle line).
- 1:50: lower coupling rod of the pantograph moved to 60 mm beside the lower arm and pinned to it at the base and at the knee (`pan-rod-pin-base`, `pan-rod-pin-knee`); raising drive lengthened to the rear frame cross member and tied to the main shaft by `pan-drive-rod`. This also removed the front silhouette finding (the rod stood 60 mm outside the drawn mast).
- 1:100: trailing-bogie axle boxes (r 125 mm, outside the wheels) and primary pads on all eight wheels; 1:200: primary pads over the four gearboxes.

Structure (true load path rail → wheel → journal → axle box / gearbox → primary → side beam → secondary spring → bolster → sill → posts → cant rail → roof bows)
- All scales: two reinforced roof bows under the pantograph feet (`end-pan-bow-1/2` at x = 2.35 and 3.75 m), feet through the roof skin, insulators as joints; 1:500 two bearers under the base block.
- 1:50: shaft bearings, axle boxes, gearboxes and primaries referenced into the list; upper articulation link lowered into both end rings and under the bellows crown; door hangers enlarged to reach leaf and lintel, kept inside the leaf plane.
- 1:100, 1:200, 1:500: cant rails 120 / 140 / 160 mm deep so that every roof bow lands on them; bolsters over both bogies with four secondary springs each; cab bulkhead posts (floor to waist ring), B-posts to the cant rail ends and a header ring over the A-posts.
- 1:100, 1:200: cab floor frame; door headers between the door posts and a hanger per leaf; `end-cross-bearer-2` moved to x = 3.26 m and `-4` to 8.53 m (behind the wheel wells, on the sill web and the cab floor frame).
- 1:100: journals, axle boxes, gearboxes and primaries; traction link between the two cross members under each pivot; pantograph head apex frame.
- 1:200: lower chord per side joining the two legs to the cross member.

Thermal: lists unchanged (regions already nested skin / lining / pane-gap-pane, read in the cut sheet at 1:50; coverage and area rules pass). New body solids are small steel and rubber parts under the floor and on the roof frame that belong to no thermal region.

Rig, poses, clips, 2D: unchanged. Rendered and read after the last geometry change at 1:50: `running` (six views, front, right, left, three-quarter), `at-stop-doors-open`, `depot-pantograph-down`, `pantograph-cycle` t = 3 s, `stop-and-open-doors` t = 12 s; structure sheets at all four scales; thermal cut at 1:50; 2D sheet at 1:50. Rendered but not read: `braking-stop`, `run-loop` t = 0.4 and 1.0, `stop-and-open-doors` t = 5, `pantograph-cycle` t = 6.

Decisions: third text of every scale (and the bogie text of 1:100) extended in English and German.

### Validation

- `r3-validate.ts vehicles/tram --volumes` (source over catalog, with print checks, after the last geometry change): `VALID vehicles/tram (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `volumes-insert.py` twice (after the geometry, after the decisions); `r3-validate.ts vehicles/tram --catalog-only --no-print`: `VALID`.
- Final proof `r3-validate.ts vehicles/tram --catalog-only --volumes` (with print checks, after the last publish): `VALID vehicles/tram (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Catalog `asset.volumes` equals the source (241/214/229, 155/115/173, 108/95/145, 55/65/104 solids).

### What remains imperfect (stated plainly)

1. Rig, doors: the leaves are called plug-sliding but only slide in the plane of the side wall (prismatic along the car, 0.55 m); in `at-stop-doors-open` and in `stop-and-open-doors` they lie coplanar in the wall skin instead of standing 40 to 60 mm proud. Needs a lateral plug bone per leaf (lateral rest placement, `T/rig-lateral-rest.md`) with poses, clips, 2D plan shapes and hangers rebound.
2. Rig, pantograph: the lower coupling rod and the upper guide rod ride rigidly on the arm bones; a true four-bar linkage (rod pivots on the base frame and on the upper arm extension) is not rigged. The head is kept level by pose angles only.
3. Rig, wheelsets: the `ws-*` bones lie along the car (0.3 m), not along the axle; rotation is about the right axis but the bone does not show the axle. The middle module pitches about the articulation only (±3 degrees); there is no yaw of bogies or articulation, so no curve pose.
4. Body 1:50, all six directions: not rebuilt in this pass. Seen and left: the left side has no doors (single-sided car, consistent in 2D); no mirrors or cameras, no coupler, no sanding pipes, no roof cable ducts, no interior beyond floor, podests and the far wall; the cut end of the middle module is a flat wall with a dark panel instead of an open gangway ring.
5. Body 1:100, 1:200, 1:500: only the bogie and articulation joints were touched; window merging differs left / right between scales as the decisions describe, not re-judged against a feature-size budget. 1:500 and 1:200 thermal and 1:100 to 1:500 thermal cut sheets were not rendered in this pass.
6. Structure: the bogie pivot of 1:100 to 1:500 sits on the bogie only; its counterpart in the car body underframe (pivot bearer between the wheel wells) is not modelled, traction goes through the secondary springs in the list. No `support` solids: the wheels stand on the ground plane and the `contact` anchors. The cut end of the middle module has no support (it hangs on the articulation, as the rig says).
7. 2D drawings: unchanged; the changed body parts are below the floor line or inside the pantograph outline and all silhouette rules pass, but the elevation does not show the new coupling-rod pins and the 1:100 trailing axle boxes as separate strokes.
8. Poses and clips not read after the change: see above; none of them moves a part that was edited other than the pantograph and door poses that were read.

### Fundamental issues (outside this asset)

- `r3-validate.ts` and `rework-09-connected.ts` print only the four worst loose groups ("and N more"); an author cannot see the whole list. `rework-concrete-mixer-truck-gaps.ts <scale> <list> <file>` does it for any asset file despite its name and default path; it should be a named shared tool (`r3-groups.ts`).
- `BoneOffset` of a prismatic joint moves along one in-plane axis only; a plug-sliding door needs two bones per leaf. Worth a note in `S/README.md`.
- Full `r3-validate.ts --volumes` with print checks took 2 to more than 10 minutes on the shared host for this asset.
