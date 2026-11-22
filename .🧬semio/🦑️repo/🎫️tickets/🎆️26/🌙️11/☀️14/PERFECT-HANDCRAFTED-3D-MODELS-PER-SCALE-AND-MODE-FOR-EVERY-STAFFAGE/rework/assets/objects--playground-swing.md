# objects/playground-swing

## Pass 1 (second round) — 2026-11-22

Earlier state: batch 15 (`rework/15.md`, section 2) corrected only the seat hitch in poses and clip (rigid pendulum) and left body, thermal, structure, 2D at all scales as they were. Source `volumes/objects/playground-swing.json` and catalog `volumes` were identical at the start of this pass.

### Reference (public single-bay steel A-frame swing, EN 1176-1/-2)

| quantity | real | model before | verdict |
|---|---|---|---|
| beam axis height | 2.4 to 2.6 m | 2.45 m, beam o101.6, overall 2.51 m | ok |
| leg tube, spread | o60.3 x 2.9, spread / height 0.72 to 0.9 | o60.3, 2.0 / 2.45 = 0.82 (22.2 degrees from vertical) | ok |
| tie | o42.4 tube in tee clamps, 0.5 to 1.0 m up | o42.4 at 0.6 m | ok |
| frame clear width | seat edge to frame >= 20 % of chain length + 0.2 m = 0.58 m | frames at z +-0.875, seat half 0.215: 0.60 m | ok |
| chain | DIN 766 8 x 24 short link, L = 1.9 m, pivot pin 85 mm under the beam axis | 76 links as round-wire lofts, forged eyes | ok |
| pendulum period | 2 pi sqrt(1.9 / 9.81) = 2.765 s | clip 2.7656 s | ok |
| seat | flat rubber seat 430 to 480 x 175 x 35 mm, ground clearance >= 0.35 m | 430 x 175 x 35, underside 0.43 m | ok |
| seat fixings | eye bolts about 0.40 m apart | z +-0.20 | ok |
| suspension spacing (lateral stability) | G >= S + 0.05 L = 0.40 + 0.095 = 0.495 m: the chains converge towards the seat | hangers at z +-0.20, chains parallel (G = 0.40 m) | WRONG |
| load path (structure) | seat insert, eye bolts, chains, hanger fork, beam, apex bracket with leg sockets, legs, base plates, anchors | members ended short of their joints: seat core 20 to 25 mm under the chain ends (all scales), chain 14 to 22 mm under the beam (1:50, 1:100), legs 3.3 mm short of the apex sleeve (1:50), supports 15 / 35 mm from the legs (1:100, 1:500) | WRONG (`volumes-connected`, 7 findings on the unchanged structure lists) |
| base plates, gussets, anchor nuts, tee clamps, apex sleeve, sockets, bolts | as built | present at 1:50, simplified per scale | ok |

Judged from the sheets (1:50 body, thermal, structure in six views, close-ups of apex, hanger, chain, seat, feet; 1:100 and 1:500 body; pose `pendulum-forward`): the frame, hardware, chain and seat read as the real object; nothing floats or pokes through; the pose keeps seat and chains in one line. The two defects above were the ones a modeller and an engineer would object to.

### Rebuilt

- **Body, all four scales** (`rework-15b-playground-swing-spread.py`): hanger pins moved to z +-0.25 (0.50 m apart); every chain solid leans 1.508 degrees about the pin (asin(0.05 / 1.9)) so its last link still hangs in the seat eye at z +-0.20. 1:50: band, two fork cheeks, clamp bolt and pin per hanger moved; top eye, shank, lower eye and both link lofts (with their repeat step) leaned. 1:100, 1:200: hanger block moved, chain capsule leaned. 1:500: chain capsule leaned. Rendered: 1:50 front at full size (chains run from the forks to the eye bolts without a kink or gap), 1:100 `pendulum-forward` in six views.
- **Thermal**: follows the body by reference; the four own chain-link extrudes of 1:100 lean with the same transform. Rendered 1:50 before the change only (see open list).
- **Structure, all four scales** (`rework-15b-playground-swing-loadpath.py`): chain members run exactly pin to seat eye; 1:50 adds M10 eye-bolt members and, as joints, hanger band and fork cheeks, the four welded leg sockets and both gussets; 1:100 adds eye bolts (15 mm), hanger blocks and the four base plates as sharp-edged joints; 1:200 / 1:500 add hitch nodes (r 35 / 40 mm) and the foot blocks as joints. Rendered: 1:50 three-quarter at full size and 1:500 in six views: one connected path from seat to the four supports.
- **2D**: front elevation of the chains redrawn as leaning quads from pin to seat eye (all scales), hangers moved 50 mm outward in front and plan (1:50, 1:100, 1:200); plan chains as tip-following capsules from pin to seat eye (`rework-15b-playground-swing-plan.py`) because a rigid plan shape under the beam cannot follow the swing. Rendered and read: 1:50 rest (all three views, line and solid) and 1:50 `pendulum-forward` (plan chains converge onto the swung seat, front chains end on the raised seat).
- **Rig, poses, clips**: unchanged. The rig is right for the subject (fixed A-frames, tie and beam; both chains revolute about the one bearing axis across the asset 85 mm under the beam axis, limits +-60 degrees; seat hitch). The lean of 1.5 degrees is carried by the solids and the front/plan shapes because the rig has no lateral rest angle (`rig-lateral-rest.md` does not exist yet).
- **Decisions** (`rework-15b-playground-swing-decisions.py`): body and structure texts of every scale state the spread suspension and the closed load path (en, de).

### Validation

`VALID objects/playground-swing (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts objects/playground-swing --catalog-only --volumes`, after the last write). Catalog and source agree (published with `volumes-insert.py`).

### Open (not signed yet)

The image reader of this session stopped answering after the last publish, so these sheets were rendered after the last change but NOT read:

- 1:50 body: pose `pendulum-back`; clips `pendulum-swing` t 0.7 and 1.38 s, `occupant-push` t 1.0 and 3.0 s.
- 1:100, 1:200 structure; 1:200, 1:500 body; 1:100, 1:200, 1:500 thermal; 1:50 thermal cut.
- 2D of 1:100, 1:200, 1:500 (front chains, hangers, plan capsules) and 1:50 rest after the plan chains became capsules (at rest a 50 mm capsule under the beam; check that it reads and is not hidden or oversized at 1:500, radius 9 mm).

Known imperfections, stated plainly:

- 1:50: chains keep their length of 1.9 m while leaning, so the last link sits 0.66 mm above its old seat-eye position (inside the play of link in eye); tee clamps without grub screws; link wire without weld seam.
- 1:500: seat is a 75 mm bar, supports of the structure list are drawn below the ground line (symbols).
- `thermal.regions[].areaM2` of `chain-steel` and `seat-rubber` were not re-measured (geometry areas unchanged by a 1.5 degree lean).
- `provenance` and `structure.assumptions` texts do not mention the spread suspension yet.

### Fundamental issues

- `T/validator.md` has no section "Rule changes authors must know"; `T/rig-lateral-rest.md` does not exist: a bone cannot rest out of the sagittal plane, so a part that leans laterally at rest must be leaned solid by solid and drawn by hand in front and plan.
- A rigid plan shape of a bone that hangs vertically at rest collapses to a blob in a swung pose (every vertex has one forward position, so `carry` cannot give it a depth); only `solid` or a `followTip` capsule follows. Nothing in `validateAsset` reports this.
