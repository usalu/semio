# Silhouette guard: humans A (15 assets)

Scope: `walking-commuter seated-reader child-on-kick-scooter running-child pregnant-person kneeling-gardener skateboarder sleeping-person standing-adult standing-phone-user runner-with-blade-prosthesis leaning-person worker-carrying-box seated-guitarist elderly-with-cane` (`🗂️catalog/👤️humans`). The guard (`catalog-silhouette`) was not loosened; every change is authored data (shape, volume solid or declared dimension), applied in place as span edits (`sil-span.py`, `humans-ops.py`, `humansa-*`).

## Result

| | Before | After |
|---|---:|---:|
| failing guard checks of the 15 (`silhouette-triage.ts current`) | 99 (14 assets; walking-commuter was already at 0) | **0** |
| `validateAsset` issues of the 15 | 0 | 0 |
| three-views sweep findings of the 15 (`humansa-threeviews.ts`) | 0 | 0 |
| `validate-all.ts` | 0 failing for humans | no humans asset failing (the failing assets of the run are vegetation, machinery and vehicles of other agents) |

The three standing constraints that interact with the guard and caught my first attempts, so every final change satisfies all four at once (guard, `validateAsset`, three-views sweep, drawn extents):

1. `validateAsset` compares the drawn extent of every scale with `dimensions` (2 % or 15 mm), rejects flat shapes on bones that leave the plane (`lateral-ambiguous`: author them as `solid` extruded or revolved) and rejects solids thinner than the thinnest drawing of the bone (`volumes-feature`).
2. The three-views test sweeps all poses and clips: a static world-metre shape (for instance on `bed-surface`) breaks the elevation/front/plan consistency of a curled pose.
3. `validateAsset` memoises per asset object: an in-memory trial must validate a `structuredClone`.

## Declared dimensions that were wrong (volume and drawing agree)

| Asset | Dimension | Before | After | Reason |
|---|---|---:|---:|---|
| elderly-with-cane | heightM | 1.65 | 1.63 | stooped: head top of drawing and volume at 1.628 to 1.637 m |
| kneeling-gardener | heightM | 1.145 | 1.132 | head top 1.128 m, knee 8 mm below ground at 1:50 |
| pregnant-person | depthM | 0.50 | 0.508 | belly: drawing and volume 0.508 m |
| running-child | heightM | 1.135 | 1.13 | volume 1.127 m (1:50), within one feature |
| skateboarder | widthM, depthM | 1.084, 1.03 | 1.1, 1.04 | hands reach 1.098 x 1.038 m in volume and drawing |
| sleeping-person | widthM | 1.9 | 1.88 | body ends at 1.948 m, starts at 0.069 m |
| worker-carrying-box | heightM | 1.81 | 1.815 | volume 1.822 to 1.828 m, 1:500 front drawing 1.783 m |
| runner-with-blade-prosthesis | heightM | 1.79 | 1.81 | the posed figure spans from the blade toe at -0.025 m to the head at 1.786 m (the blade dips 25 mm below the datum in the default pose) |

## Causes found once, applied across assets

- **Shapes with a `solid` hint in the elevation are not placed by the plain affine of their bone**: the outline is the union of the profile at both lateral ends and the sweeps, so a probe without `solid` dilated hands by 4 to 5 mm. `humansa-lib.ts` probes with `solid` and erodes the target by the sweep (hands of seated-reader, elderly, standing-adult).
- **Elevation shapes are the depth profile of the plain front and plan shapes of the same id and bone** (`twins`): an edit of an elevation hand or head changes the front and plan of the same figure. Always re-run the three views.
- **Front crotch gap** (pregnant-person, runner, standing-adult, guitarist): the drawn thigh extrusions meet at the centre line, the volume leaves a slit from the real crotch downwards. Repair: inner edges of the thigh extrusions 6 mm apart plus a small plain `crotch` shape from the crotch height to the shorts hem (runner), or the existing crotch box lowered to the volume apex (pregnant-person).
- **Volume nose floats in front of the face** (worker-carrying-box 4.5 cm, seated-guitarist 4 cm): the `nose` ellipsoid translation moved back onto the face (0.036/-0.086 to -0.046 and 0.1249/-0.113 to -0.073).
- **Toe lofts of sleeping-person sit on the centre line and twice as wide as the foot** (z 0, rx 0.07 against the foot at 0.094 and 0.042): z set to +-0.094, rx times 0.6 at all scales.
- **Finger capsules of the plan leave open slits** (standing-phone-user): capsule radii times 1.25 close the slits between the four capsules at 1:50 (the drawn mitten is a closed outline).
- **Rectangular duvet extrusions against a rounded volume** (sleeping-person front): the superellipse exponent of the four duvet lofts is 16 instead of 3, so the volume has the flat top and vertical sides of the extruded drawing.
- **Pen strokes outside the body**: closed or open detail strokes (`front` hem line of child-on-kick-scooter, `mouth-line`, `shoe-sole`, `bench-back-slat`, `sole-toe`, `torso-hem`) shortened to the body (`humansa-strokeclip.ts`, `humansa-lib.ts clipStroke`).
- **Contour redraw**: where the drawn outline and the volume projection differ by more than a feature, the shape is replaced by the outline of the projected solids in bone coordinates (`humansa-redraw.ts`, `humansa-redrawiter.ts` for tilted bones where the placement is not affine: fixed-point fit of the polygon in bone coordinates). Local `clip` and `grow` patches (`humansa-lib.ts patchShape`, `humansa-applypatch.ts`) for notches and fills.
- **1:500 legibility thickening against the volume**: the drawing is redrawn from the volume only where `validateAsset` stays clean (the auto tool validates a clone of the asset after every accepted change), because a redraw from the 1:500 volume copies its ground penetration and its enforced minimum thickness into the drawing.

## Per asset (what was wrong, what was edited)

| Asset | Checks before | Decision |
|---|---:|---|
| walking-commuter | 0 (fixed earlier) | only the one-line numeric arrays of its volumes block restored (`humansa-compact.py`) after an earlier in-place dump expanded them |
| seated-reader | 4 | cowl, neck, head and shoe contours redrawn from the solids (drawing wrong); hand and thumb elevation from the solid frame; sole and slat strokes shortened; `shank` front taper (`zTip`); 1:500 notebook outline |
| child-on-kick-scooter | 6 | hem stroke shortened (1:50 elevation); fork blades and head tube of the plan redrawn from the volume; front handlebar and grips widened to 0.45 m to keep the declared depth; thigh/hand 1:50 and 1:500 contours |
| running-child | 6 | dimension 1.13; contour redraws at 1:50 and 1:500 |
| pregnant-person | 6 | crotch box between the thighs lowered to the volume (apex 0.7233), torso top raised by 16 mm (followTip shortens it), right forearm width, dimension depth 0.508 |
| kneeling-gardener | 7 | dimension height 1.132, hat brim front/plan, shoulder/thigh patches |
| skateboarder | 7 | dimensions 1.1 x 1.04, hands redrawn |
| sleeping-person | 7 | toe lofts, duvet exponents, one extruded plan shank shape on bone `shank-r`, dimension width 1.88 |
| standing-adult | 7 | hand contours (elevation, solid aware), 1:500 and 1:200 area patches |
| standing-phone-user | 7 | front hair outline by fixed-point fit, plan hands and ponytail from the volume, finger radii 1.25, 1:500 area |
| runner-with-blade-prosthesis | 8 | dimension 1.81, thigh slit and crotch shape, plan thigh clip |
| leaning-person | 8 | contour redraws, front hood width at 1:100 |
| worker-carrying-box | 8 | nose, toe lofts (first section 6 mm earlier), plan forearm radii, dimension 1.815 |
| seated-guitarist | 9 | nose, guitar body, thighs, shoes, upper arms, stool legs and seat, shoulder cap radius 0.04, upper-arm extrusion width |
| elderly-with-cane | 9 | dimension 1.63, cane crook follows the volume centre line at 1:100, finger lines are texture at 1:50, hand contours |

## Tools (all in this folder, prefix `humansa-`)

`humansa-lib.ts` (clusters, redraw, patch, clip, fixed-point fit), `humansa-auto.ts` (per cluster trials, keeps only changes that improve the view and keep `validateAsset` clean; `humansa-auto-run*.sh`), `humansa-clusters.ts`, `humansa-sweep.ts`, `humansa-png.ts` (drawn-only, volume-only and boundary pixels as an image), `humansa-who.ts`, `humansa-shapes-at.ts`, `humansa-pairs.ts`, `humansa-redraw.ts`, `humansa-redrawiter.ts`, `humansa-world.ts`, `humansa-addworld.ts`, `humansa-addsolid.ts`, `humansa-addextruded.ts`, `humansa-strokeclip.ts`, `humansa-applypatch.ts`, `humansa-extent.ts`, `humansa-extents2.ts`, `humansa-validate.ts`, `humansa-validate-head.ts`, `humansa-threeviews.ts`; editors by span: `humansa-apply.py` (dedupes ops, writes poly back as compact path), `humansa-dims.py`, `humansa-setval.py`, `humansa-toez.py`, `humansa-fingers.py`, `humansa-exponent.py`, `humansa-restore.py`, `humansa-restore-solids.py`, `humansa-remove-shape.py`, `humansa-compact.py`, `humansa-shift.py`.

## Files changed

`🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<slug>/🔣️.json` for the 15 slugs above (walking-commuter: formatting only). No test, guard or schema file was touched.
