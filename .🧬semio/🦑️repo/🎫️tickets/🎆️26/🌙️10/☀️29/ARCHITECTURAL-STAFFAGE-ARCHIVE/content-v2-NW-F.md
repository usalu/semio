# Content report NW-F (new furniture)

Assets (all `completion: authored`, v2, new): `S/🗂️catalog/🛋️furniture/{three-seat-sofa,dining-table,bookshelf,bar-stool,picnic-table,folding-deck-chair,single-bed}/🔣️.json`.
Scripts (idempotent, one builder per asset, shared `nwf.py` + `lib.py`, `check-all.sh`): `T/🧰️scripts/NW-F/`. Evidence images: `T/🗑️generated/NW-F/*.png` (own SVG preview via rsvg-convert, print size x8 zoom). Manifest NOT edited (coordinator: seven new entries needed, poses/clips/symbols below).

| asset | W x H x D (m) | mass kg | bones | named+key poses | clips | regions | shapes 500/200/100/50 |
|---|---|---|---|---|---|---|---|
| three-seat-sofa | 2.0 x 0.85 x 0.9 | 51.8 | 10 | 5+2 | 3 | 5 | 4/9/16/23 |
| dining-table | 1.6 x 0.75 x 0.9 | 60.5 | 5 | 4+1 | 3 | 5 | 5/5/8/8 |
| bookshelf | 0.8 x 1.8 x 0.3 | 34.9 | 17 | 3+1 | 3 | 4 | 12/12/16/26 |
| bar-stool | 0.42 x 0.87 x 0.42 | 6.8 | 5 | 4+2 | 3 | 5 | 4/5/8/11 |
| picnic-table | 1.52 x 0.75 x 1.8 | 61.6 | 5 | 4+1 | 3 | 4 | 7/8/9/9 |
| folding-deck-chair | 0.78 x 0.93 x 0.58 | 5.95 | 7 | 4+1 | 3 | 3 | 4/5/6/8 |
| single-bed | 2.12 x 0.95 x 0.9 | 59.2 | 8 | 4+1 | 3 | 3 | 6/7/13/13 |

## Verification status
- Official check (`🩺️check --asset <id>`): see "Official check results" at the end (runs were queued behind the shared capacity floor).
- Own validator `nwf.validate` (mirrors the check: ids, references, mass sum = dimensions, bindings of every region/emitter in every pose, joint limits per pose, loop closing, track targets, per-scale region coverage, extents within max(2 %, 15 mm)) passes for all seven.
- Own suitability estimate `nwf.suitability` (same ink-gap/feature definitions as `🖨️representation/📐️measure`, in pure Python) reports 0 problems at all four scales for the poses named in each builder (sofa: unoccupied/one-seated/proof-load; table: closed/extended/pulled-apart; shelf: empty/loaded/shelf-adjusted; stool: unoccupied/occupied/turned-90/proof; picnic: deployed/stowed/occupied; chair: unfolded/folded/reclined; bed: flat/head-raised/asleep/reading). The sofa passed the official check with min stroke 0.13, gap 0.24, feature 0.23 mm at 1:500.
- Preview images read (print size x8): sofa 50/100/200/500, table 50 extended / 100 closed / 500 extended, shelf 50 and 100 loaded, stool 50 occupied and 500, picnic 50 deployed and 100 stowed, chair 50 unfolded / folded / 200, bed 50 reading and 100 flat. Not read: the other six views, clip instants, thermal overlays (not run by me).

## Design rules found by the measure (important for all later workers)
Every pair of shapes is measured, occluded or not, in every named pose: two shapes that do not touch or cross must be at least (0.15 mm + half the sum of their pens) apart, otherwise the ink gap is 0 and the check fails. So: members that meet are overlapped on purpose (cushion seams share an edge, the skirt overlaps the arms, mattress parts overlap at hinges), anything that stays near but unjoined is removed at the small scales (decision text says so), and details are inset >= (0.15 + pens/2)/k from neighbours. Thin parts (< 0.2 mm) are open strokes or merged slabs.

## Per asset

### three-seat-sofa (2.00 x 0.85 m, plan symbol `sofa-plan`)
- Rig: frame, two arms, three seat cushions (prismatic y, 0 to -90 mm) and three back cushions (0 to -40 mm). Cushions are rigid slabs that sink behind the skirt rail; sink 60 mm for 75 kg from s = 0.4 (sigma/sigma40) t, sigma = 0.8 x 735 N / 0.13 m2 = 4.5 kPa, CLD40 6 kPa (ISO 3386-1), t = 0.20 m. Poses: unoccupied, one-seated, two-seated, lounging (distributed load), seat-proof-load (1600 N, EN 12520/1728 order, tagged authored-estimate), keys impact (72 mm) and recovering. Clips: sit-down-and-settle, stand-up-and-recover, lie-down-on-sofa.
- Thermal: seat/back/arm upholstery (fabric 1 mm over HR foam 0.20 / 0.15 m, R about 4.4 m2K/W, C about 3.3 kJ/m2K), skirt (fabric on 18 mm plywood), beech feet; window sun patch 350 W/m2 (authored scenario); occupied poses bind 30 degC contact (ISO 7730) and shade.
- 1:500 one outline + seat line + two foot blocks (feet thickened to 0.14 m, stated); 1:200 arm+foot merged outlines, back band with two seams, three seat cushions, skirt; 1:100 three back and three seat cushions, arm welts, seat piping, tapered feet; 1:50 adds piping around every cushion, arm top piping, foot rings. No scale unavailable. Skirt welt removed (cushions sink within 0.03 m of it).

### dining-table (1.60 -> 2.20 m, plan symbol `table-plan`)
- Rig: two table halves with their legs (prismatic x +-0.30 m), a leaf (prismatic y 0.03 m: stored under the top, hidden behind the aprons, lifting flush), fixed slide rails. Poses: closed, extended, set-for-dinner (6 settings 0.5 kN/m2 distributed, authored-estimate; forearm contacts), edge-test-load (750 N, EN 1730/EN 12521 order, authored-estimate); key pulled-apart. Clips: extend-table, retract-table, lay-the-table.
- Thermal: oiled oak 30 mm top (17 kJ/m2K), leaf exposed 0 when stored, aprons, tapered legs, steel slide rails.
- 1:500/1:200 slab per half (top+apron merged) + leaf-with-rail bar (0.09 m thick, stated) + leg strokes (500) / polygons (200); 1:100 all members separate; 1:50 plus grain hatching (0.6/0.5 mm).

### bookshelf (0.80 x 1.80 m, plan symbol `shelf-plan`)
- Rig: carcass, five adjustable shelves each = prismatic pin-height bone (+-0.2 m, System 32 pitch 32 mm) with two half-span revolute bones (+-3 deg) that sag. Sag from beam theory: w = 454 N/m (books 6.0 kN/m3 x 0.28 m x 0.27 m, EN 1991-1-1 Annex A, rounded, authored-estimate), L = 0.764 m, E = 2.55 GPa and I = 1.36e-7 m4 (EN 312 P5) give 5.8 mm = L/132 (exceeds L/200: a full 35 kg shelf is deliberately shown overloaded), bending stress 2.2 MPa of 15 MPa; theta = 0.87 deg per half. Poses: empty, loaded, shelf-adjusted (shelf 3 up four holes = 128 mm); key lower-two-loaded. Clips: load-the-shelves (bottom up, 10 load tracks), re-pin-shelf, unload-shelves.
- Books are loads (contact fraction 0.85 at 21 degC on shelf tops), not drawn: empty and loaded differ only physically.
- Thermal: melamine chipboard 18 mm, hardboard back, plinth. 1:500 block + five shelf strokes; 1:200 block + shelf strokes; 1:100 panels thickened to 0.022 m, ten shelf halves overlapping at mid-span; 1:50 real 18 mm panels, shelf pins that move with the board when re-pinned.

### bar-stool (0.42 x 0.87 m, plan symbol `stool-plan`)
- Rig: chromed frame (legs, footrest ring), swivel seat, foam pad (prismatic y, 0 to -40 mm), backrest (prismatic x +-0.15 m). Swivel is invisible for a round seat in elevation, therefore it is shown through the backrest projection x = 0.15 m sin(azimuth) (foreshortening of the pad neglected, stated in structure assumptions). Poses: unoccupied, occupied (85/15 % seat/footrest, pad -18 mm), turned-90, seat-proof-load (1600 N, -30 mm); keys turned-270, occupied-turned. Clips: swivel-turn (looping 360 deg, 6 s), sit-and-turn, stand-up.
- Thermal: chrome tube (emissivity 0.1, alpha 0.4, 1.5 mm wall), PU leatherette over HR foam, plywood underside (hidden), contact 30 degC seat / 28 degC shoes on the ring.
- 1:500 seat block including the backrest (its 0.3 mm travel is not resolvable; front and rear legs omitted: closer than the 0.17 m minimum gap) with two leg strokes and ring; 1:200 moving T backrest, two leg strokes (0.18 mm); 1:100/1:50 capsules, plate/pad/back parts, ferrules at 1:50.

### picnic-table (1.52 m end view x 0.75 m, length 1.80 m, plan symbol `picnic-plan`)
- Rig: A-frame body, top, two benches on revolute hinges (+-95 deg) that fold up against the A-frames. Poses: deployed, stowed, occupied (4 x 75 kg + 0.3 kN/m2 x 1.8 m settings, contacts), bench-proof-load (1600 N, EN 581-2 order, authored-estimate); key half-folded. Clips: fold-benches, deploy-benches, sit-down-together.
- Honest limit: the table is a fold-down-bench variant of the classic A-frame picnic table; the classic fixed table is the same drawing with fixed benches.
- Thermal: weathered pine (alpha 0.7 authored), steel hinges, full sun on the top.
- 1:500 slab + leg and bench strokes + hinges (beam omitted: gap rule); 1:200 slab with beam merged, leg polygons, benches 0.05 m thick (stated); 1:100 real members; 1:50 plus grain hatching.

### folding-deck-chair (0.78 x 0.93 m side elevation, width across 0.58 m, symbol `chair-plan`)
- Rig: scissor mechanism (rear frame turns, front foot slides x = 1.452 m cos(alpha) so the front tip stays on the rear frame, seat frame on the scissor pivot) and a two-bone canvas (0.60 + 0.45 m inextensible) whose joint is the lower circle intersection, solved per pose by `deckchair.py` and stored as bone offsets. Poses: unfolded (62 deg), folded (84 deg), reclined-seated (second notch 56 deg, load 735.5 N), sling-proof-load (1600 N); key half-folded. Clips: fold-chair, unfold-chair, sit-and-recline.
- Known limits: the folded canvas bulges 0.3 m to the rear (rigid-length canvas cannot crumple); the 0.003 m foot end cap lift while folding is below print resolution.
- Thermal: beech 25 x 45 mm, cotton canvas 0.7 mm (0.6 kJ/m2K: follows sun in seconds), 30 degC contact in the seated pose.
- 1:500 three frame strokes and one sling stroke (the seat canvas run J-S omitted: it lies closer than the minimum gap to the rear frame); 1:200 five strokes; 1:100 capsules + canvas strokes + pivot; 1:50 plus top/front rail dowel sections.

### single-bed (2.12 x 0.95 m, plan symbol `bed-plan`)
- Rig: frame, slatted base in three sections (head section revolute 0 to -65 deg about x = 0.78 m; the foot section is fixed because a raised foot section would pass within centimetres of the footboard and fail the gap rule), mattress in three parts (prismatic along the section normal, 0 to -60 mm). Poses: flat, head-raised (50 deg), asleep (hips -28 mm, 40/35/25 % load split, contact 35 % at 30 degC), reading-in-bed (55/30/15 %); key half-raised. Clips: raise-head-section, get-into-bed, sit-up-to-read.
- Thermal: quilted cover 10 mm over cold foam 0.18 m, birch slats (region hidden in elevation, declared), pine frame; EN 1725 cited for scope only (no test magnitude modelled).
- 1:500/1:200 posts, rail, mattress parts with slats merged; 1:100 slat strips and piping; 1:50 plus grain hatching on posts, rail, leg.

## Parameter sources
Timber/board/steel/foam properties: ISO 10456 (timber, plywood, chipboard, steel), Wood Handbook FPL-GTR-282 (beech, oak, pine stiffness), EN 312 (P5 chipboard), EN 1993-1-1 (steel), ISO 3386-1 order for foam (stress-strain), Incropera (chrome emissivity); upholstery fabric, canvas, batting, leatherette effective properties and foam conductivity are `authored-estimate`. Heat capacities = rho cp min(t, sqrt(alpha 3600 s)). No respiration or body physiology (furniture); occupants are loads and 30 degC (ISO 7730) contact surfaces. Test-load magnitudes (1600 N seat, 750 N table edge) are recalled values of the order of EN 1728/EN 12520/EN 581-2/EN 1730 and tagged authored-estimate: verify against the standards.

## Remaining doubts
- Official check/preview of the six later assets and the other six views have not been inspected by me (see results below if filled in).
- Manifest entries (poses, clips, symbols, `analytics`, distinction texts) for the seven assets are needed from the coordinator.
- Plan symbols are authored as plan-only `true-extent` symbols; no representation has a `plan` geometry and `viewpoints` stay `["elevation"]`.
- 1:50 table, picnic table and chair have few shapes (8-9): the real objects are simple; more joinery lines would break the gap rule.
