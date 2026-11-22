# components/stair-flight — rework report

## Pass 1 (second round)

### State found

- No earlier report file. Scripts of an earlier author exist (`rework-18-stair-flight-kit.py`, `-volumes.py`, `-roots.py`): the kit states the real geometry, `-volumes.py` writes the source, `-roots.py` writes rig, anchors, poses, clips, thermal, structure, 2D drawings and the symbol.
- Source `volumes/components/stair-flight.json` differed from the catalog at all four scales: the source carried a balustrade on both open sides (posts, flanges, handrail, bottom rail, balusters; thermal region `balustrade-steel`, a second mass), the catalog had the bare flight (17 body solids at 1:50) and no `balustrade-steel` region, height 2.90 m. The author was cut off before `-roots.py --apply`.
- Judged by rendering (`r3-sheet` 1:50, source against catalog): the source is the better state (a flight of 2.90 m without any guard is not a buildable stair, MBO section 38). The candidate of `-roots.py` had 4 findings: symbol height 2.90 against 3.989 m (twice) and the 6 mm post flanges below the 7.5 mm feature (twice).

### Reference table (real subject: straight steel centre-stringer flight with timber treads, residential or office storey)

| quantity | reference | model |
|---|---|---|
| storey height | 2.75 to 3.00 m | 2.90 m |
| rise r | DIN 18065: 140 to 190 mm (buildings in general) | 181.25 mm (16 risers) |
| going g | at least 260 mm | 280 mm |
| step rule 2 r + g | 590 to 650 mm | 642.5 mm |
| pitch | 30 to 37 degrees | 32.92 degrees |
| nosing lap of open stairs | at least 30 mm | 30 mm |
| usable width | at least 1.00 m | 1.20 m treads, 1.058 m clear between handrails |
| risers per flight | at most 18 | 16 |
| tread board | hardwood or C24, 40 to 60 mm, nosing radius 3 to 10 mm | 310 x 1200 x 40 mm, radius 8 mm |
| open riser gap | at most 120 mm where children are expected | 141 mm (see open points) |
| stringer | HEA 160: h 152, b 160, t_w 6, t_f 9, r 15 mm, 30.4 kg/m, 0.906 m2/m | h 152, b 160, t_w 6, t_f 9, r 15 (four root fillets) |
| base plate | 15 to 20 mm, four M12 anchors, edge distance at least 25 mm | 300 x 260 x 15, four M12, nuts AF 19 x 10 |
| head plate | 15 mm, four M12, slotted holes | 280 x 260 x 15 |
| tread bracket | fin 8 to 12 mm, carrier plate 6 to 10 mm | fin 10 mm, carrier 170 x 360 x 8 |
| handrail | CHS 42.4 (grip 30 to 45 mm), top 0.90 to 1.10 m above the nosing line | CHS 42.4, top 0.95 m |
| posts | CHS 42.4 or flat 40 x 10, spacing 0.8 to 1.2 m | CHS 42.4 on treads 1, 4, 7, 10, 13, 16 (0.84 m in plan), 50 mm inside the tread ends |
| post base | round flange 80 to 100 mm, 8 to 10 mm | 80 x 8 mm |
| infill | clear at most 120 mm | balusters of 12 mm at 105 mm centres (93 mm clear), bottom rail CHS 21.3 0.10 m above the nosing line |
| mass | HEA 153 kg, treads 100 kg, plates and brackets about 95 kg, two balustrades about 123 kg | 471 kg |

### Defects found (before this pass)

1. Catalog without the balustrade and with the height 2.90 m; source and catalog disagreed (all scales, all lists).
2. 1:50 body: HEA web 7.5 mm instead of the real 6 mm and the 15 mm root radii missing (two features at 1:50).
3. 1:50 body: post flange 6 mm (below the feature and thinner than a real 8 mm base flange); flange edge exactly on the tread end (no edge distance for the screws).
4. 1:500 body, thermal, structure: 70 balusters of 75 mm at 105 mm centres (30 mm gaps), a wall of rods that prints black and inflates the volume and the steel area.
5. Symbol: no balustrade, height 2.90 m against the declared 3.989 m.

### Rebuilt in this pass (scripts `rework-18-stair-flight-kit.py`, `-volumes.py`, `-roots.py`, edited; they state every number)

- **Source and catalog agree again**: `-roots.py --apply` then `volumes-insert.py`. The catalog now has the balustrade in every list and scale, region `balustrade-steel` (5.94 m2), height 3.989 m (top of the handrail at the landing), mass 471.2 kg, the 2D drawings of all four scales with the balustrade as strokes, and the symbol with handrail, posts and the two plan handrail lines.
- **Rig**: unchanged in topology (a structural rig: `stringer` on the HEA centroid axis from foot bearing to landing bearing, 5.082 m at 32.92 degrees; `walker-carrier` prismatic along the tread line). No lateral pivot exists in a fixed flight (`rig-lateral-rest.md` does not exist and is not needed here). Mass and centre of mass of the bone include the balustrades.
- **1:50 body**: web at its true 6 mm (`sheet: true`), four root fillets of 15 mm (`hea-root-top|bottom-r|l`, concave quarter section extruded along the axis, ending square inside both end cuts, at most 23 mm short of the plate on the acute side); post flange 80 x 8 mm; balustrade axis 50 mm inside the tread ends (flange edge 10 mm inside the board).
- **1:50 thermal / structure**: fillets are steel by reference and part of the stringer member.
- **1:500 all lists**: balusters removed (handrail, six posts, bottom rail at the 75 mm feature remain); one mass for flight and balustrades at the common centre of mass (inside the saw-tooth of the steps), because the centre of the balustrades alone lies in the open field between the rails.
- **2D**: 1:500 elevation without baluster strokes; symbol redrawn with the balustrade.
- **Decisions** (en, de) of 1:50 and 1:500 rewritten for the changes.

### Validation

- `r3-validate.ts --file <candidate> --volumes`: `VALID file/stair-flight (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- After publishing: `r3-validate.ts components/stair-flight --catalog-only`: `VALID ... schema 0, validateAsset 0`.

## Pass 1 (second round), second author session

### State found

- Report of the first session existed and ended at "Validation" without its announced open points. Source `volumes/components/stair-flight.json` and the catalog `asset.volumes` were identical (balustrade state published); nothing to reconcile.
- Rendered and read at full size before any change: 1:50 body (sheet, three-quarter, right side), 1:50 structure and thermal, 1:100 / 1:200 / 1:500 body, 1:500 structure. Poses and clips move no solid (a structural rig: the load cases change loads, `walker-carrier` is a massless prismatic carrier), so every pose shows the same body.

### Measured against the reference table (additions to the table above)

| quantity | reference | found | after this session |
|---|---|---|---|
| clear opening between treads (open risers) | at most 120 mm where children are expected (DIN 18065, MBO section 38) | 141 mm (181.25 - 40) | 111 mm under a 30 x 30 mm guard batten |
| guard batten ("Kinderschutzleiste") | timber 25 to 40 mm, screwed under the tread behind the nosing | missing | 30 x 30 x 1200 mm, 30 mm behind each nosing, 16 pieces, 7.3 kg |
| mass | 471 kg + battens | 471.2 kg | 478.5 kg |
| infill tone at 1:200 | real 12 mm bars at 105 mm = 11 % solid | 30 mm bars at 105 mm = 29 % | unchanged (see below) |

Everything else of the table (rise, going, pitch, HEA 160 section with root radii, plates, anchors, brackets, handrail height, post spacing, baluster spacing) was measured right in the renders and the kit numbers.

### Defects found

1. All scales but 1:500, body and 2D: open riser 141 mm, above the 120 mm limit; the stair was not buildable in a dwelling.
2. Texts (decisions of volumes and representations, summary) stated "risers open, 141 mm" as the final state.
3. 1:200 body: the infill reads heavier than reality (29 % against 11 %).

### Rebuilt (scripts `rework-18-stair-flight-kit.py`, `-volumes.py`, `-roots.py`; every number stated there)

- **Body 1:50, 1:100, 1:200**: new solid `riser-guard` (extrude, timber, repeated 16 times along the pitch): a 30 x 30 mm batten over the full 1.2 m, under each tread 30 mm behind its nosing, i.e. directly behind the back edge of the tread below; 4 features at 1:50, 2 at 1:100, 1 at 1:200. At 1:500 the risers are closed in the folded band, as before.
- **Rig**: topology unchanged; stringer mass 478.5 kg and centre of mass recomputed with the battens (flight centre 84 mm above the stringer axis, was 81). `rig-lateral-rest.md` read: a fixed flight has no lateral pivot, nothing to apply.
- **Structure**: the batten carries nothing and is not a member; the mass marker of the flight moved with the centre of mass. Load path unchanged (treads, carrier and fin brackets, HEA, pin at the base plate, roller at the head plate; handrail, posts, flanges on the treads).
- **Thermal**: unchanged lists; the batten has no walking surface and the region `tread-top` counts the walking surface only, so it is left out like the tread undersides.
- **2D** (1:50, 1:100, 1:200): elevation and front redrawn with the 16 battens (`riser-guard-01..16`, a square under each nosing in the side view, a strip under each tread in the front view); plan unchanged (hidden under the treads). 1:500 and the symbol unchanged.
- **Decisions and summary** (en, de): volumes decisions of all four scales, representation decisions (silhouette, retained, omitted), asset summary.
- **Tried and withdrawn**: every second baluster at 1:200 (15 instead of 35 per side, 14 % solid). `volumes-cross-scale` measured 90 mm against 1:100 (limit 60) and 187.5 mm against 1:500 (limit 168.8); the rule is right that neighbouring scales must agree, so all 35 bars stay at the 30 mm feature and the decision text says why.

### Validation

- Candidate: `r3-validate.ts --file 🗑️generated/rework-stair-flight/stair-flight/🔣️.json --volumes`: `VALID file/stair-flight (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published (`-roots.py --apply`, `volumes-insert.py`): `r3-validate.ts components/stair-flight --catalog-only`: `VALID ... schema 0, validateAsset 0`.
- Re-rendered after publishing, no `NOT DRAWN`: all four scales in all three lists, poses `self-weight` and `point-load-tread`, clips `loading-ramp` and `moving-point-load` at t = 1 s and 4 s, 2D of all four scales. Read at full size after the change: 1:50 body from below (battens), 1:50 2D, clip `moving-point-load` t = 4 s, 1:200 body, 1:100 structure, 1:200 thermal, 1:500 thermal, 2D 1:200 and 1:500.

### What remains imperfect, stated plainly

- 1:200, body / thermal / structure: the 35 balusters per side are 30 mm bars at 105 mm centres (29 % solid, real 11 %); the balustrade reads as a grille. Thinning is blocked by `volumes-cross-scale` unless 1:100 is thinned too, which would be wrong there.
- 1:50 body: the posts stand on 40 mm softwood treads through an 80 mm flange. That is built this way, but for 0.5 kN/m on the handrail the tread end and its four screws are at their limit; a side-mounted post on a welded lug or a 60 mm hardwood tread would be the more robust detail. Not changed (it moves the stringer and every derived value).
- 1:50 body: no weld seams, no tread screws, no slotted holes in the head plate, no handrail end caps (all below or at the 7.5 mm feature).
- Structure: the balusters are listed as members although they carry only the infill load to the two rails.
- Read after the last change as well: 1:100 body, 2D 1:100, clip `loading-ramp` t = 1 s. Not read again (rendered without error; lists not touched by the batten, or a pose that moves no solid): 1:100 thermal, 1:200 structure, 1:500 body and structure, pose sheets `self-weight` and `point-load-tread`.
- Final line: `r3-validate.ts components/stair-flight --catalog-only --volumes`: `VALID components/stair-flight (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
