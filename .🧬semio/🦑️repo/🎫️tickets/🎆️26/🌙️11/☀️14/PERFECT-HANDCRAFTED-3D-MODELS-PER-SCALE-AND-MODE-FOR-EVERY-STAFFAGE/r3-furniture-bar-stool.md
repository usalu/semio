# Round 3 — furniture/bar-stool

State: finished and spliced (2026-11-20). No earlier round-3 author had started this asset; I started from the committed state.

Scripts (ticket folder): `r3-bar-stool-kit.py` (the construction, every dimension stated once, world to bone-local bookkeeping), `r3-bar-stool-model.py` (body, thermal, structure, volume decisions of every scale), `r3-bar-stool-draw.py` (2D of every scale and view, ruling E1), `r3-bar-stool-build.py` (root fixes, representation decisions, symbol; writes the candidate `🗑️generated/r3-bar-stool/🔣️.json`), `r3-bar-stool-apply.py` (writes the validated candidate into the catalog, member by member in the formatting each had; writes `volumes/furniture/bar-stool.json`), `r3-bar-stool-run.sh` (build, validate, apply only when valid, validate again), `r3-bar-stool-print.ts` (print suitability per scale).

## 1. Reference

The subject is a classic four-leg swivel bar stool with a footrest ring, an upholstered round seat and a low backrest (the type sold for bar counters, 0.75 m seat height). No web search was needed; the sources are below.

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Seat height for a 1.05 m bar counter | 0.74 to 0.78 m | trade practice, counter minus 0.28 to 0.30 m knee clearance | 0.752 m (cushion top) |
| Footrest height | 0.25 to 0.30 m above the floor, about popliteal height (0.43 to 0.47 m with shoes) below the seat | anthropometric popliteal height (adult 50th percentile about 0.44 m plus shoe) | ring axis 0.27 m, top 0.2825 m: 0.47 m below the seat |
| Footrest ring | 25 to 28 mm tube welded to the outside of the legs, 0.38 to 0.45 m across | market products | 25 mm tube bent to 0.40 m (centre line), outer 0.425 m |
| Seat | 0.33 to 0.40 m round; 12 to 18 mm plywood board, 40 to 60 mm foam, cover wrapped round the board edge and stapled underneath | upholstery practice; ISO 3386-1 for foam (asset reference) | 0.36 m; 12 mm birch plywood, 40 mm HR foam, 1.5 mm leatherette with piping at the top seam |
| Swivel | ball-bearing plate (lazy-susan type) 150 to 180 mm, about 8 to 12 mm high, screwed between seat board and frame top | hardware catalogues | 170 mm, 12 mm, raised ball race |
| Frame top | legs welded into a pressed steel seat pan or a top ring | market products | pressed 2 mm pan 0.32 m with 20 mm turned-down rim, legs welded inside the rim |
| Legs | 25 x 1.5 mm steel tube (0.87 kg/m), splay 4 to 8 degrees, rubber or plastic glides | EN 1993-1-1 density (asset reference) | 25 x 1.5 mm, splay 4.7 degrees, from radius 0.144 m at the pan to the 0.1975 m foot circle, 30 x 22 mm domed rubber glides |
| Footprint | about the seat diameter plus 0.05 to 0.10 m (stability) | EN 12520 / EN 1728 tip tests (asset references) | 0.395 m foot circle, 0.425 m over glides and ring |
| Low backrest | top 0.10 to 0.20 m above the seat, 0.30 to 0.38 m wide, carried by a bent flat bar or tube from under the seat | market products | top 0.87 m (0.118 m above the seat), 0.30 m wide, 90 mm high, 26 mm thick, bent on 0.30 m; 40 x 8 mm bent flat bar |
| Mass | 6 to 10 kg for a chromed swivel stool with back | market products; tube 0.87 kg/m, sheet 7850 kg/m3 | 7.8 kg (frame 5.0, seat 1.4, cushion 0.5, backrest 0.9) |

## 2. Audit of the committed state

- **Rig.** `backrest` joint at seat-local (0.022, 0.14) = world (-0.14, 0.72), inside the cushion: the backrest was a post growing out of the cushion top. `frame` mass 4.0 kg at y 0.35 m (the true frame with the pan is 5.0 kg at 0.43 m). `pad` (cushion) as a child of the frame instead of the seat: the cushion does not swivel (invisible for a round cushion; kept, see section 7).
- **Construction.** The legs ran straight into the 40 mm "plywood plate" of the swivelling seat: a fixed frame welded into a turning seat is impossible; no swivel, no frame top. The "plywood 12 mm" plate was drawn and modelled 40 mm thick (material, layer and area say 12 mm). The footrest ring was a capsule in elevation at R 0.19 (half inside the legs). The rubber ferrules had the material chrome-steel. The back post (30 mm round tube) rose from the cushion top at r 0.14.
- **Anchors.** Feet at x/z = +-0.21 m (the glide centres are at 0.1975 m); `ring-front` load point at x 0 (the stool axis) instead of on the ring.
- **Dimensions.** 0.42 x 0.42 m, but the drawings and the model reach 0.425 m over glides and ring; mass 6.8 kg against 7.8 kg from the construction.
- **2D.** All scales: legs and ring as before, a seat plate band that does not exist, a straight backrest block on a post. 1:200/1:500: legs as strokes ending at x 0.21 (outside the foot), plan with leg crosses through the seat. The symbol drew the seat at 0.82 to 0.87 m (backrest height) and no backrest.
- **Solids (old).** 7/6/5/4 body solids; QA (qa-visual.md): stair-stepped pad rim, masses floating, 1:200/500 legs thinner than drawn, 1:500 thermal ring offset, backrest a block. Kernel traps: three revolves with implied closure (pad 1:100, ferrule and plate 1:50).
- **Thermal.** `seat-plate` declared `hidden` but drawn and modelled as a visible body band; the frame top did not exist so it had no region; `footrest-ring` 0.084 m2 = a ring of R 0.17 inside the legs.

## 3. Root fixes (catalog file outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `dimensions` | 0.87 x 0.42 x 0.42 m, 6.8 kg | 0.87 x 0.425 x 0.425 m, 7.8 kg |
| `rig.frame` | 4.0 kg, com y 0.35; "four legs and footrest ring" | 5.0 kg, com y 0.43 (tubes 0.87 kg/m, 2 mm pan); title names legs, ring and pan |
| `rig.seat` | com x 0.01 | com x 0.004; title names swivel plate and board |
| `rig.pad` | com y 0.015 | com y 0.012; title "upholstered seat cushion" |
| `rig.backrest` | x 0.022, y 0.14, 0.15 m, com (0.10, 0) | x -0.004, y 0.14 (the bolt group of the bar under the board), 0.176 m, com (0.11, 0.045) |
| `anchors` foot-* | +-0.21 | +-0.1975 (glide centres) |
| `anchors` ring-front | (0, 0.27) | (0.2, 0.2825): top of the ring tube at the front |
| `anchors` seat-top | y 0.03 | y 0.032 (cushion top) |
| `materials` | 4 | + `rubber` (glides; ISO 10456); plywood title "seat board 12 mm" |
| `thermal.regions` seat-plate | "Seat plate underside", hidden text about the rim | "Plywood seat board (inside the upholstery)", hidden text: inside the wrapped cover (now true) |
| `thermal.regions` legs | "Chromed legs" | "Chromed legs, glides and back bar" (area 0.25 unchanged; model 0.242) |
| `thermal.regions` footrest-ring | 0.084 m2 | 0.099 m2 (ring R 0.2 outside the legs, 4 pi2 R r = 0.0987; factor 1.18, inside C16) |
| `thermal.regions` | 5 regions | + `seat-mount` (pan and swivel, 2 mm steel layer, 0.21 m2 from the model, heat capacity 7065 J/m2K) and its entry in all six poses (exposed 1, sunlit 0) |
| `thermal.assumptions[1]`, `structure.assumptions[0..1]`, `provenance.notes` | describe the plate, radius 0.14, 0.21 m feet | rewritten (en, de) for the real construction |
| `representations[*].geometry` | 11/11/13, 8/8/9, 7/7/6, 6/6/5 shapes | 13/13/14, 9/9/9, 8/8/4, 6/6/3 (elevation/front/plan at 1:50, 1:100, 1:200, 1:500) |
| `representations[*].decisions` | old | rewritten per scale (en, de) |
| `symbols[0]` | seat at 0.82-0.87 m, no back | seat 0.698-0.752 m, backrest 0.78-0.87 m, legs and ring at their places |

Poses and clips are unchanged and were rendered after the rig change (`seat-proof-load`, `turned-90`, 2D `turned-90`).

## 4. What was modelled (solid counts body / thermal / structure)

**1:50 — 9 / 11 / 20.** Legs (capsule, 4 copies) from the pan rim to inside the glides; domed rubber glides (revolve); footrest ring as a true torus (24-point loop) welded outside the legs; pressed 2 mm pan with 20 mm rim, open below (revolve loop); 12 mm swivel with the raised ball race (seat bone); cushion as one revolve (crowned top, 20 mm top round, 5 mm bottom round, wrapped down to the board underside); 6 mm piping at the top seam (slender torus); bent 40 x 8 mm flat back bar (extrude of its side outline with 12 mm bend); curved back pad (loft of 15 superelliptic sections on a 0.30 m bend radius, ends rounded in plan). Thermal: references for legs, glides, bar, ring, pan, swivel, cushion and back covers; the hidden 12 mm board with arrised edges; foam shells 1.5 mm inside both covers. Structure: legs, ring, pan, glides (ground member), swivel as bearing joint, board, cushion, back bar; weld joints at the four leg tops and the four ring crossings, the foam joint, the backrest bolts; four contact pads; four masses (frame at its true centre between the legs at 0.43 m, seat, cushion, backrest).
**1:100 — 7 / 8 / 19.** Legs to the floor (glides merged), 12-point ring, pan as a 17 mm chamfered disc, 15 mm swivel, cushion with a 3-segment round, bar 15 mm, back pad 11 sections.
**1:200 — 6 / 7 / 15.** Legs and ring at their true 25 mm (C13: drawn at that width), pan and swivel merged into a 32 mm mount disc, chamfered cushion, bar and pad 30 mm, pad 10 sections.
**1:500 — 4 / 6 / 12.** Legs true 25 mm and slender (strokes), ring true 25 mm (capsule/disc in 2D), one 86 mm seat block narrowing to the pan, one 75 mm curved back panel (11 sections) round the rear of the seat.

## 5. Checks (final)

- `$BUN volumes-check.ts furniture/bar-stool --product`: **PASS** at every scale. Silhouette IoU elevation / front / plan: 1:50 0.991 / 0.988 / 0.953; 1:100 0.992 / 0.992 / 0.951; 1:200 0.986 / 0.986 / 0.913; 1:500 0.971 / 0.962 / 0.939. Bounds within 3.5 mm. Thermal area deviation at 1:50: seat-pad 0.007, seat-plate 0.046, seat-mount 0.007, legs 0.031, footrest-ring 0.001, backrest 0.063; 1:200 total 0.026, 1:500 total 0.005.
- `$BUN r3-validate.ts furniture/bar-stool --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal the source (spliced with `volumes-insert.py`, four scales authored).
- Print suitability (`r3-bar-stool-print.ts`): suitable at all four scales.

## 6. What the final renders show

- **1:50 body** (`r3-sheet`, six views): front and back: cushion with its round over the darker swivel and the pan, three legs splaying to the glides, the ring at a third of the height, the back pad centred on the bar. Right and left side: the flat bar runs under the cushion, bends up 10 mm behind its edge and disappears into the pad; the pad sits 28 mm above the cushion. Top: cushion disc, ring visible all round, leg feet and pan corners just outside the cushion, back pad as a gentle arc. Three-quarter: reads at first sight as a chromed swivel bar stool with a padded seat and a curved low back.
- **1:50 thermal (cut):** cover and foam shells nested in cushion and pad, the green hidden board inside the cushion, the purple seat mount below, chrome legs and bar, cyan ring; no shell pokes out.
- **1:50 structure:** members follow the tubes, pan, swivel, board, cushion and bar; orange welds at the leg tops and ring; green pads under the glides; the frame mass hangs between the legs at 0.43 m (true centre of mass), the backrest mass inside the pad.
- **Poses:** `turned-90`: the backrest swings to the side round the seat circle, bar and pad stay attached, 2D turns with it. `seat-proof-load`: the cushion drops 30 mm (see section 7).
- **1:100:** same stool with plain bands; **1:200:** cushion on the mount band, thick bar and pad, true tubes; **1:500:** seat block with the curved back panel and true tubes, still unmistakably the same stool.
- **2D:** 1:50 and 1:100 true views with swivel and pan bands; plan with the ring as a 25 mm annulus and the back pad on the rear edge; 1:200/1:500 signs readable, every scale print-suitable.

## 7. Still imperfect (stated plainly)

- **Plan of the backrest** is its bounding rectangle in 2D (0.066 x 0.30 m) while the 3D pad is an arc: a swivelling part must be a `solid` shape and no `solid` kind can be bent (request in `author-requests.md`). Plan p95 at 1:50 is 26 mm against 30.
- **Cushion compression** is the rigid prismatic `pad` bone: in `seat-proof-load` (30 mm) the whole cushion, wrapped edge included, drops over the swivel and the top of the pan instead of the foam squeezing; `occupied` (18 mm) only overlaps the swivel. A deformable cushion needs weights or a scale on the bone.
- **The cushion is a child of the frame**, not of the swivelling seat (rig as committed). It is round, so nothing shows; re-parenting it would change the frame of the pose `dy` values and was not done.
- **Legs on the axes** (one leg straight ahead) as committed; a 45-degree layout is equally common but would rewrite every anchor, pose contact and drawing for no gain in truth.
- **1:500 ground members** (glides) are 75 mm blocks at the feet in the structure list, because `volumes-feature` needs the feature in every extent; **1:500 back panel** area is 2x the region (only the total is compared there, 0.5 %).
- Tool inconsistencies found on the way (stroke extents with or without pen; closed slender ring not pen-widened) are in `tool-requests.md`.

## Round 5 handcraft (2026-11-21)

Script: `r5-bar-stool-build.py` (loads the round-3 construction and restates it on the corrected rig; writes the candidate that `r3-bar-stool-apply.py` puts into the catalog). The round-3 build reproduced the catalog exactly before the change (checked member by member).

### Reference (counter practice, ISO 7250 popliteal height, EN 12520 / EN 1728, market products)

| Quantity | Reference | Model 1:50 | Deviation |
| --- | --- | --- | --- |
| Seat height for a 1.05 m counter | 0.74-0.78 m | 0.752 m | 0 % |
| Footrest above floor | 0.25-0.30 m | ring axis 0.27, top 0.2825 m | 0 % |
| Seat to footrest | popliteal 0.43-0.47 m with shoes | 0.47 m | 0 % |
| Ring | 25-28 mm tube, 0.38-0.45 m | 25 mm on 0.40 m, outer 0.425 m | 0 % |
| Seat | 0.33-0.40 m round, 50-70 mm upholstered | 0.36 m, 54 mm | 0 % |
| Swivel plate | 150-180 x 8-12 mm | 170 x 12 mm | 0 % |
| Legs | 25 x 1.5 mm, splay 4-8 degrees | 25 x 1.5, 4.7 degrees, foot circle 0.395 m | 0 % |
| Low backrest | top 0.10-0.20 m above seat, 0.30-0.38 m wide | 0.118 m, 0.30 m | 0 % |
| Mass | 6-10 kg | 7.8 kg | 0 % |

### Audit and root fix: the rig

The skeleton was wrong although every solid sat right: `frame` (the welded frame, 5.0 kg) was a fixed bone lying along the floor, 0.70 m long, from the stool axis to 0.28 m beyond the front glide; `ground` was a 0.42 m bar along the floor; `pad` ran 0.36 m ahead from the axis, 0.18 m beyond the cushion rim. In the structural 2D view both long horizontal bones are load-bearing members (`eligibleBones`: length >= 5 % of the height), so the forces view drew a horizontal frame member in the air ahead of the stool.

| Bone | Old | New |
| --- | --- | --- |
| `ground` | 0.42 m along the floor | 0 m (floor marker at the foot of the axis) |
| `frame` | angle 0, 0.70 m, com (0.43, 0) world | angle 90, 0.686 m (floor to the top of the seat pan on the swivel axis), com x 0.43 along the axis |
| `seat` | x 0, y 0.698, angle 90 (in the floor frame) | x 0.698, y 0, angle 0 (same world frame) |
| `pad` | x 0, y 0.72, angle 0, 0.36 m, prismatic axis 90 | x 0.72, y 0, angle -90, 0.18 m (axis to cushion rim), prismatic axis 0 (down the frame axis) |
| anchors on `frame` | foot-xn (-0.1975, 0), foot-xp (0.1975, 0), ring-front (0.2, 0.2825) | (0, 0.1975), (0, -0.1975), (0.2825, -0.2) in the turned frame; same world points |
| poses `occupied`, `seat-proof-load`, `occupied-turned-key` | pad dy -0.018 / -0.03 / -0.018 | pad dx (along the frame axis), same travel |

Everything bound to `frame` was restated in the turned frame: the volumes (legs, glides, ring, pan, mount, supports, weld ellipsoids with their radii turned, the four-fold repeats about the frame axis) and the elevation shapes (capsules and polygons; front and plan are rest-pose world coordinates and did not change). Seat, pad and backrest keep their world frames, so their solids and shapes are unchanged. `structure.assumptions[0]` names the slide of the cushion down the stool axis.

### Grades (opened with the Read tool)

| Scale | Body before -> after | Thermal | Structure |
| --- | --- | --- | --- |
| 1:50 | B (rig) -> A- | A- | B (frame member along the floor) -> A- |
| 1:100 | B (rig) -> A- | A- | B -> A- |
| 1:200 | B (rig) -> B+ | B+ | B -> B+ |
| 1:500 | B (rig) -> B+ (tub-like merged back panel at the 75 mm floor) | B+ | B -> B+ |

Opened: 1:50 body before and after, 1:50 structure, 1:500 body, 1:50 `seat-proof-load` (the cushion slides down the axis), the 1:50 2D after the change. Rendered, not opened: the other poses, the `sit-and-turn` frame, thermal cuts and structure of 1:100-1:500, 2D 1:100-1:500 (`🗑️generated/r5-bar-stool/after/`).

### Checks

`r3-validate.ts --file` on the candidate with `--volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0); applied (`rig, anchors, representations, poses, structure`), spliced, `--catalog-only` VALID.

`volumes-check.ts furniture/bar-stool --product`: PASS (0 fail).

### Remaining

- The cushion is still rigid on its prismatic bone: under load the wrapped edge drops over swivel and pan instead of the foam squeezing (needs a deformable cushion).
- Plan of the backrest is its bounding rectangle in 2D (`solid` shapes cannot be bent).
- 1:500 back panel 75 mm thick and 0.16 m high (bar and pad merged at the feature floor).
