# objects/stepladder — running author report

## Pass 1 (second round), 2026-11-22

State at start: no report for this asset; batch report `rework/15.md` section 6 (fold mechanism: rear-frame hinge, two spreader link bones, platform bone, poses `folding-1..7`, `folded`, clip `fold-up`) and the round-3 note `r3-objects-stepladder.md`. Source `volumes/objects/stepladder.json` and catalog `volumes` were identical. The full validation of that state was **INVALID** (2 findings, see below).

### Reference table (aluminium household platform stepladder, EN 131, 4 treads + platform; Hailo L60 / Krause Corda class, from my own knowledge)

| quantity | real | model (measured from rig, solids, sheets) | verdict |
|---|---|---|---|
| overall height open / platform height | 1.65-1.70 m / 1.03-1.06 m | 1.68 m / 1.060 m (deck top, local v -17 mm on the axle at 1.043 m) | ok |
| stile length / climbing angle | 1.80-1.85 m / 60-70 degrees | bone 1.818 m / 67.5 degrees | ok |
| rear frame | 72-76 degrees, hinged on the stiles above the platform, shorter than the stile below the hinge | 74 degrees, hinge at u 1.440 (1.33 m high, 0.27 m above the deck), rail 1.384 m | ok |
| footprint depth x width | 0.95-1.05 m x 0.47-0.50 m | 0.93 m x 0.47 m (front feet z +-0.216, rear +-0.198) | ok |
| stile / rear rail section | box 55-60 x 20-22 / 40-45 x 20 | 60 x 22 x 1.5 / 45 x 20 x 1.2, rounded corners | ok |
| treads | 80 mm deep, grooved, pitch 0.21-0.25 m | 82 mm, grooved, pitch 0.212 m, two rivets per end | ok |
| platform | 250-300 mm wide and deep, 25-30 mm rim, on an axle, rests hooked on a cross tube of the rear frame | 290 x 250 mm, 26 mm, axle + rest tube + rear rim | ok; latch detail missing |
| hand rail above platform | >= 0.6 m | stile tops 0.62 m, grab tube axis 0.56 m | ok |
| spreaders | two-link steel straps with a knuckle (trade ladders); household platform ladders often use the platform or webbing instead | two 280 mm links per side, 20 x 8 U-channel, pivots and knuckle bolts | plausible hybrid, kept |
| shoes | moulded rubber, sole level in use, about 90 x 40 mm | 88 x 38 and 76 x 32 mm, soles level when open | ok |
| mass | 4.6-5.5 kg (household), 7-8 kg (trade) | 7.1 kg | at the upper end; kept (sum of the declared sections) |

### Defects found (looked at: all four scales x body/thermal/structure in the default pose, `folded`, `folding-4`, close-ups front / right / rear three-quarter, 2D of 1:50)

1. **Structure, 1:50 and 1:100 (validator finding, real):** the contact pads lay 6.3 / 3.3 / 8.6 mm under the stile and rail members with nothing between: the shoes, which carry the frames, were not in the load path (`volumes-connected`, 2 findings: the asset was INVALID).
2. **Poses `folded`, `folding-1..7`:** the root lean was solved for the bone tips, not for the soles. The rubber shoes are fixed to the frames, so once the frames turn the soles tilt: in `folded` the rear soles (31.5 degrees off level) were sunk 21 mm into the ground, the front soles up to 5 mm in the middle keys.
3. **Platform during folding:** lifted by a linear law, so at the first keys it still lay almost level while the rest tube had already left it (it hung in the air instead of being lifted off first).
4. **1:200 platform:** the 56 mm print slab was thickened upwards in the elevation and in 3D (standing surface at 1.087 m instead of 1.060 m; the platform bone's v points down) while the front view drew it downwards (1.006-1.062): elevation/3D and front disagreed by 25 mm. With the corrected poses the print check also reported it (0.192 mm at 1:350).
5. Body proportions, sections, rig topology and joints at all four scales: measured against the table, right. Not rebuilt.

### What I changed (`rework-stepladder-pass1.py`, hand-stated numbers in its docstring)

- **Structure, all four scales:** the four shoes are members of their frames (`st-shoe-*` by reference): stile or rail end -> shoe -> contact pad -> ground without a gap. Decision text added (en, de).
- **Poses:** every folding key and `folded` re-solved by bisection so that the lowest sole edge of the front pair and of the rear pair touch the ground together (root `angleDeg` and `y`; lean -0.21 ... -7.33 degrees, lift 0.2 ... 5.7 mm, `folded` -5.31 degrees / 4.2 mm). Descriptions (en, de) now say the ladder stands on its sole edges.
- **Platform law:** lift = 62 degrees x (1 - (1 - closing/31.5)^4): 31 degrees up when the frame has closed 5 degrees, so the hooked rim is clear of the rest tube before the tube moves under it. Clip description updated.
- **1:200 platform (3D solid, elevation and front shape):** 70 mm deep (0.2 mm at 1:350), downwards from the true standing surface at 1.060 m; the obsolete 56 mm decision replaced.

### Validation and publication

- `r3-validate.ts --file <candidate> --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.
- Published twice (`rework-stepladder-pass1.py --apply`, `volumes-insert.py`); `r3-validate.ts objects/stepladder --catalog-only --volumes`: `VALID objects/stepladder (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Renders read after the last change

1:50 thermal; 1:200 body (before the last platform fix) and `folding-4` (after it); 1:500 structure; 1:100 structure; 1:50 `folding-5`, `folding-3` and `folded` close-up (candidate), `fold-up` at 0.9 s; 2D of 1:200, 1:500 and of 1:50 `folded`. They show: shoes on the ground line in every folding state, the platform lifted early and lying along the stiles when closed, the 1:200 slab hanging below the standing surface, shoes blue (members) over green pads in the structure sheets, six thermal regions as the same object.

Rendered but NOT read after the last change: 1:100 body/thermal, 1:200 thermal/structure, 1:500 body/thermal, `folded` at 1:100/1:200/1:500, `folding-1/2/6/7`, `open-in-use`, `rated-load-test`, `fold-up` 2.4 s, `climb-and-work` and `rated-load-ramp` frames, thermal cut, 2D of 1:100 (these lists and poses were not changed in form by this pass except the shoe references and the root solve).

### What remains imperfect (plainly)

- 1:50 body: no platform latch / lifting strap (the platform is lifted "by hand" in the clip); the head hinge is a bolt through a round rail head without the hinge bracket; no tool tray; rivets, extrusion grooves as in round 3.
- The type is a hybrid: platform lock AND steel link spreaders (household ladders of this class usually have only one of the two); mass 7.1 kg is a trade weight.
- 1:500 structure: the grab tube is not a member (ghost only); the platform is a 26 mm slab (passes as declared, thin in the side view).
- Platform clearance to the rear rails near the hinge is about 0-5 mm while it turns up (true to the type, not measured per pose).
- Closed angle 7 degrees is derived, not from a manufacturer drawing.

### Fundamental issues (outside this asset)

- During this pass `r3-2d.ts` failed five times with `Cannot find module '../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts' from '.../🧍️staffage/🔨️modules/🎨️depiction/🌊️flow-view/📐️model/🟦️.ts'` (concurrent edit of the flow-view module; it worked again minutes later).
- The print check (`range-end-feature`) measures poses other than the default pose but its message names neither the pose nor the view, so the cause of a finding has to be found by trial.
