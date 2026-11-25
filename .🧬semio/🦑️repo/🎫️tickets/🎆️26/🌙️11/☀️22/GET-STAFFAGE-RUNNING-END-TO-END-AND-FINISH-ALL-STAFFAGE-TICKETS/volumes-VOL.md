IN WORK

# 🧊️ Lane VOL: the 28 Missing Volumes in the Catalog (2026-11-25)

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, scripts `U/🧰️matrix/VOL/` and `U/🧰️linux/s4/`, output `U/🗑️generated/matrix/VOL/` (removed at the end). Every run went through `repo capacity run` (800 MB for scripts, 2500 MB for vitest).

## 1. State Found

- All 27 documents had the hash the s4 report names (`shasum -a 256`, 27 of 27 equal) before anything was written, and again right before the write (`apply.sh`: `as backed up` 27 of 27).
- `prove.ts` was rerun on the current engine for all 27 before anything was changed: 27 of 27 `ok`, 0 `checkVolumes` findings (the engine moved since the s4 proof, the result did not).

## 2. The Bodies, Looked At

Every body was rendered with `r3-sheet.ts` (six shaded views) at the next finer scale above the written one (`sheets.sh`), and all 28 pictures were opened. Seen at 1:500 unless stated:

| Asset | Seen | Judgement |
|---|---|---|
| `used-microwave-oven` 1:200 | housing, framed door with dark window, fascia, pad | good |
| `used-microwave-oven` | one blank box on the pad | poor, reworked: door frame with the window set back, fascia, housing behind |
| `used-dishwasher` | casing with the door in its notch on a pallet slab | good |
| `used-djembe-and-cajon` | goblet with head and ring in front of the box | good |
| `used-electric-cooker` | door 75 mm proud, no window, four equal plates | poor, reworked: door at its true 30 mm, window, true 199 and 163 mm plates |
| `used-electronic-organ` | no cheeks, so the side view lost half of the cabinet; bench on two slabs | poor, reworked: 75 mm cheeks of the true side outline, four bench legs |
| `used-oil-filled-radiator` | one envelope block with the control end on two feet | good (seven fins at 57 mm pitch are below the feature) |
| `used-sewing-machine` | machine with arm, case and pedal on a four-legged table | good |
| `used-tumble-dryer` | block with a plain disc | reworked: porthole ring around its glass |
| `used-bedside-table` | closed box on two slabs | poor, reworked: open shelf, four legs |
| `used-high-chair` | slanted frames, seat, tray, footplate, backrest | good |
| `used-brine-water-heat-pump` | closed box, the open machine of 1:200 gone | poor, reworked: open cabinet of declared panels with compressor, exchangers and control box |
| `used-cast-iron-sectional-boiler` | block, panel, burner door, spigot; no lower door | reworked: cleaning door |
| `used-dry-rotor-inline-pump` | stepped motor and pump with both flanges and terminal box | good |
| `used-insulated-copper-coils` | one ring in its cradle on the pallet slab | good |
| `used-panel-radiator-pallet` | stack and covers between the two end frames, valve stub | good |
| `used-towel-radiator` | the ladder was one solid plate | poor, reworked: five bars between the uprights |
| `used-wall-hung-gas-boiler` | casing, flue, pads, one typified strap | good |
| `used-accordion` | open case with both instrument halves and bellows, lid on the floor | good |
| `used-harmonium` | side panels full height in front of the top, crest a rectangle | reworked: stepped side outline, gabled crest |
| `used-moving-boxes` | three stacked cartons on the blanket | good |
| `used-spring-rocker` | horse panels on the spring, square bars as handles | reworked: round bars |
| `used-steel-string-acoustic-guitar` | closed case in two halves with its handle | good (the case is closed in the default pose) |
| `used-trumpet` | a polygon slab and a cone in a half-filled case | poor, reworked: case at its true depth, leadpipe, main tube, bell tube, valve block, flared bell |
| `used-viola` | open case, instrument outline, bow in the lid | good |
| `used-violin` | open case, instrument, bow, strap triangle | good (the neck is a 75 mm bar) |
| `used-washbasin-vanity` | closed slab, no bowl | poor, reworked: oval mouth with its floor |
| `used-wc-pan-cistern` | pointed prism for the pan, nine-point lid | poor, reworked: turned oval bowl, tapered foot, rear body, fourteen-point lid |

Reworked: 13 scripts of `🧰️linux/s4` (decisions in English and German follow the new bodies). Proven again by `prove.ts` (13 of 13 `ok`, 0 `checkVolumes` findings) and looked at again (`look.sh`, sheets `n1`, `n2`): each reads as its object at 1:500.

## 3. Written

`apply.sh` on the catalog at 16:40: 27 documents, 28 scale entries `unavailable` to `authored`. `paths.py` compares each with its backup: only `asset.volumes.500` (and `.200` of the microwave oven) differ, plus the four documents of section 4.

## 4. The 15 Findings at Other Scales

| Asset | Findings before | Repair | After |
|---|---|---|---|
| `used-cast-iron-sectional-boiler` | 4 `closed` (1:50, 1:100, 1:200) | the four middle sections are 119 mm on their 120 mm pitch, the joint of two castings (`joints.py`) | 0 |
| `used-panel-radiator-pallet` | 4 `closed` (1:50) | the radiator covers are 99 mm on their 100 mm pitch | 0 |
| `used-moving-boxes` | 6 `closed` (1:50) | the flaps are 164 mm, the seam of the carton | 0 |
| `used-trumpet` | 1 `budget` (structure 10638 at 1:100, 12158 at 1:200) | joints of 60 mm and mass markers of 36 mm at 32 mm (`trumpet.py`): 10462 at 1:200 | 0 |

Measured by `volumes.ts` at 1:50, 1:100 and 1:200: `volumeIssues`, `exportIssues`, geometry findings and `checkVolumes` findings all 0.
