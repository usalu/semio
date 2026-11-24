# 🧊️ Three-Views Guard, Part 2: Eleven Assets

Fixer report of 2026-11-23. Scope: the last seven of the fifteen older red assets (red-fox, runner-with-blade-prosthesis, seated-office-worker, skateboarder, sleeping-person, walking-commuter, worker-carrying-box) plus reclaimed-lattice-girder, used-viola, kneeling-gardener and parent-with-stroller.

## Result

| Asset | Sweep before | Sweep after | Validator (schema / validateAsset / checkVolumes) | Check CLI | Asset changed |
|---|---|---|---|---|---|
| kneeling-gardener | 0 (already green after the shared union fix) | 0 | not re-run (no change) | `✓` | no |
| runner-with-blade-prosthesis | 0 (already green after the shared union fix) | 0 | not re-run (no change) | `✓` | no |
| parent-with-stroller | 15 | 0 | 0 / 0 / 8 (the 8 `volumes-connected` findings are in the file as I found it: same count on the untouched copy) | `✓` | yes |
| red-fox | 1 | 0 | 0 / 0 / 0 | `✓` | yes |
| skateboarder | 1 | 0 | 0 / 0 / 0 | `✓` | yes |
| seated-office-worker | 2 | 0 | 0 / 0 / 0 | `✓` | yes |
| worker-carrying-box | 15 | 0 | 0 / 0 / 0 | `✓` | yes |
| reclaimed-lattice-girder | 6 | 0 | 0 / 0 / 0 | `✓` | yes (generator and document) |
| used-viola | 7 | 0 | 0 / 0 / 0 | `✓` | yes (generator and document) |
| sleeping-person | 5 | **4** (one finding fixed; four are the shared `unionPolygons` defect, request written) | 0 / 0 / 0 | `✓` | yes |
| walking-commuter | 8 | **8** (not fixed, asset unchanged) | not re-run (no change) | `✓` | no |

Nine of eleven pass the sweep; default-pose consistency is 0 for all eleven. All numbers are from the last run of `🧰️three-views/sweep-2.ts` over the eleven catalog files and of `🧰️three-views/prove-2.sh <id>` per changed asset (sweep, `r3-validate.ts --file … --volumes`, check CLI `--asset`, silhouette measurement).

`🧊️catalog-silhouette`: measured before and after with `🧰️three-views/silhouette-2.ts` (the bounding box, IoU and Hausdorff rules of the test without its sharp oracle). For every changed asset the list of failing scale and view pairs is the same before and after (girder 3, viola 1, stroller 10, fox 1, skateboarder 5, office worker 7, sleeper 2, worker 3); nothing that passed fails now. I did not run the vitest suite itself.

No `widthM`, `heightM` or `depthM` changed. One number of `dimensions` changed: reclaimed-lattice-girder `massKg` 779.56 → 779.54 (see there); the manifest row of the girder carries no mass that I found, the coordinator may want to check.

## Per asset

### reclaimed-lattice-girder
- Before: 6 findings at 1:500 (`props-cleared`, `lifted`, clip `lift` 3.2 to 7.99 s): side elevation up to 3.008 / 3.508 m, front 1.200 / 1.700 m.
- Cause: at 1:500 the two round slings were drawn in the side elevation only (a stroke), `author-girder.py` left them out of front and plan (`if scale < 500`).
- Fix in the generator `RECLAIMED-…/assets/reclaimed-lattice-girder/author-girder.py`: `sling_solid_shape` takes the scale; at 1:500 front and plan draw the sling as the stroke of its axis (pen 0.15 mm, as the side elevation) carrying the same lateral sweep, so it follows the swing. A first try with the filled 30 mm profile was refused by the print rule (0.19 mm at 1:500).
- Regenerated with the hash-locked geometry environment of the tests (`🧰️three-views/py-geometry.sh author-girder.py`), candidate copied to the catalog; candidate = catalog (cmp).
- Stated plainly: the generator did not reproduce the published document byte for byte before my change (60 differing paths with either shapely I tried: redundant collinear vertices in 1:50 to 1:200 outlines, reversed point order of open strokes, one gusset vertex 4 mm off a straight edge, girder mass 746.44 → 746.43 kg). The regenerated document carries the generator's current output for these; hence `massKg` 779.54. Full proof re-run on it: VALID 0/0/0, check `✓`.
- Sheet opened (1:500 `lifted`, `props-cleared`): the front shows the sling as a vertical stroke from the top chord to 3.5 m, the plan a dark stroke along the top chord between the lifting points; dimension lines read 3.51 / 3.51 m and 3.01 / 3.01 m.

### used-viola
- Before: 7 findings at 1:200 (`played`, clip `bowing`): side 1.201 m, front 1.234 m (tolerance 0.024).
- Cause: fingerboard and neck taper along the neck; the front view drew each as ONE prism (hull of all sections swept along the neck). Tilted 12 degrees into the playing position, the full-height section travels to the nut and stands 33 mm too high.
- Fix in the generator `USED-MUSICAL-…/assets/used-viola/author-viola.ts` (`violaFront`): fingerboard and neck are drawn as slices along the neck (fingerboard 6 at 1:50, 3 at 1:100 and 1:200; neck by its sections), each the hull of its two end sections. Shape ids `fingerboard-1…`, `viola-neck-1…` replace `fingerboard`, `viola-neck` in the front view. The generator reproduced the published document byte for byte before the change; regenerated, copied to the catalog, candidate = catalog.
- Sheets opened (1:50 `played`, `stored`): played reads 1.21 m side and 1.22 m front; the stored front view is the case section as before.

### parent-with-stroller
- Before: 15 findings at 1:500 (every pose and instant with a twisted bone): front from −0.101 m, side from 0.
- Cause: the four shoe shapes of 1:500 front and plan were flat outlines of the walking default pose (the right shoe 0.28 m tall for the pitched foot), carried as cards; the finer scales place solids.
- Fix (`🧰️three-views/fix-parent-with-stroller.py`): the eight shapes become the side profile of the shoe swept laterally (z ±0.054 … ±0.138).
- Sheets opened (1:500 `pausing-with-stroller` before and after): front height 1.85 m before, 1.75 m after against 1.74 m side.
- Open, not mine: `checkVolumes` 8 (`volumes-connected` at 1:200 and 1:500), identical on the untouched copy.

### red-fox
- Before: 1 finding, 1:200 `curled-sleeping`: front x to 0.380 m, plan y to 0.420 m (tolerance 0.033).
- Cause: the fore paws of the front view were profiles revolved about the paw's own normal, a 40 mm disc whichever way the paw points; curled, the fore legs lie across.
- Fix (`fix-red-fox.py`): at 1:50, 1:100, 1:200 the two front fore paws become the side profile swept over the paw width at z ±0.032 (1:100 passed by 2 mm only, same defect).
- Sheets opened (1:50 `curled-sleeping`, `standing-alert`): curled, the fore paws lie flat to the side on the ground; standing is unchanged in outline size.
- Stays imperfect: the trunk of the front view is also a revolved profile and stays 28 mm narrower than the plan in the curled pose (inside the tolerance); the front view of the curled fox is a rough picture.

### skateboarder
- Before: 1 finding, 1:500 `pop`: side from −0.059 m, front from 0.013 m.
- Cause: the typified board of the 1:500 front view was a card that does not tilt with the board.
- Fix (`fix-skateboarder.py`): it becomes the side outline swept over the deck width (±0.1015).
- Sheet opened (1:500 `pop` before and after): front 1.74 m before, 1.81 m after against 1.83 m side.
- Stays imperfect: in `pop` the thick typified board of 1:500 reaches 59 mm under the ground in side and front (0.12 mm on paper); the root height of the pose is shared with the finer scales. At 1:50 to 1:200 the front deck is still a level card between two trucks at different heights (passes the guard through the wheels).

### seated-office-worker
- Before: 2 findings, clip `settle-back` at 4.2 s, 1:100 and 1:500: left hand 1.558 / 1.544 m side, 1.602 / 1.610 m front.
- Cause: front and plan hand are the same solid (palm outline swept 0.8 mm), but the front outline was 30 to 90 mm longer than plan and side.
- Fix (`fix-seated-office-worker.py`): the eight front hands take the plan outline of their scale. A first variant (swept side profile) made the hand cover a brow at 1:50 in `leaning-back` and tripped the print rule; taken back.
- Sheets opened (1:50 and 1:500 at 4.2 s): 1.58 / 1.57 m and 1.54 / 1.54 m.

### worker-carrying-box
- Before: 15 findings (`walk-midstance-l/-r`, `walk-toe-off-l/-r`, `carry-walk` 0.22 / 0.66 s, all scales): side x from −0.465 m, plan from −0.39 … −0.41 m.
- Cause: plan and front shoes were cards; the pitched swing foot is foreshortened.
- Fix (`fix-worker-carrying-box.py`): 32 shoe shapes (rear foot and toe box, both feet, plan and front, four scales) become the side profile swept over the shoe width.
- Sheet opened (1:50 `walk-midstance-l`): the trailing shoe reaches back in the plan as in the side view. Stays imperfect: shoes are rectangles in plan and front now, not the rounded outlines.

### sleeping-person
- Before: 5 findings: 1:50 `turn-over` 3.2 s (side to 0.917 m, front to 0.832 m) and 6.4 s not evaluating at four scales.
- Fixed (`fix-sleeping-person.py`): the 1:50 front view had no left arm at all (the other scales and the right arm have it); sleeve, forearm sleeve, cuff and hand added as the solids the other views place. Sheet opened (1:50 at 3.2 s): the raised left arm shows in the front view, 0.56 m side and 0.54 m front.
- Open: the four `evaluate` findings at 6.4 s. One front shape (`baffle-1-3-1`, a skinned quilt panel) arrives at `unionPolygons` as two faces with edges 0.2 to 0.6 micrometres apart; 10 of 71 instants between 6.37 and 6.51 s throw. A trial nudge of a quilt key left the failing instants in place and was taken back. Request with the input polygons: `requests.md`, section "three-views part 2 — `unionPolygons` …".

### walking-commuter
- Before and after: 8 findings (`glance-at-watch` 0.8 and 3.2 s, all scales): side x to 0.52 m, plan to 0.43 m. Asset unchanged (cmp with my copy of the start).
- Cause confirmed: no shape of the asset is a solid; front and plan limbs are cards of the standing figure. With the left forearm level the plan forearm and hand are slivers.
- Three redraws of the left forearm, wrist, cuff and hand were tried and each made the sweep 0 but broke a guard the asset passes today, so none was kept: (1) front outline swept along the bone normal for both views: print rule 1:500 (`hand-l` 0.198 mm), silhouette plan 1:100 Hausdorff 0.044 > 0.030; (2) the same in front, swept side profile in plan: same print finding, Hausdorff 0.043; (3) bodies of revolution: `extent-front-depth` at 1:50 (0.543 against 0.56 m), same print finding, Hausdorff 0.036. The right arm cards are drawn for the folded forearm of the standing pose and cannot be restated from the rest frame.
- What it needs: the left arm of front and plan drawn again by hand together with its volume (round footprints in plan, a 1:500 hand at least 0.2 mm), as the existing request "`walking-commuter`: plan feet and hand no longer follow the stride" proposes.

## Files changed

Catalog (repo-relative, under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`): `🧱️components/reclaimed-lattice-girder/🔣️.json`, `🪑️objects/used-viola/🔣️.json`, `👤️humans/parent-with-stroller/🔣️.json`, `🐕️animals/red-fox/🔣️.json`, `👤️humans/skateboarder/🔣️.json`, `👤️humans/seated-office-worker/🔣️.json`, `👤️humans/sleeping-person/🔣️.json`, `👤️humans/worker-carrying-box/🔣️.json`.

Generators and candidates (under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/`): `RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/assets/reclaimed-lattice-girder/author-girder.py` and `🔣️.json`; `USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/assets/used-viola/author-viola.ts` and `🔣️.json`.

Umbrella ticket: `requests.md` (one section appended), `three-views-2.md`, and new under `🧰️three-views/`: `sweep-2.ts`, `shapes-2.ts`, `silhouette-2.ts`, `evaluates-2.ts`, `union-2.ts`, `rest-frames.ts`, `look.sh`, `prove-2.sh`, `py-geometry.sh`, `json-diff.py`, `tv2_doc.py`, `fix-parent-with-stroller.py`, `fix-red-fox.py`, `fix-skateboarder.py`, `fix-seated-office-worker.py`, `fix-sleeping-person.py`, `fix-worker-carrying-box.py` (the fix scripts are records; each ran once and refuses or double-applies on a second run).

The older humans and the fox have no generator that can be run again (chains of one-off patch scripts in the 3D ticket), so the catalog file is their source; the edits keep every untouched byte (`tv2_doc.py`). Not touched: shared code, tests, fixtures, the manifest, any other asset. `🗑️generated/tv2/` removed.
