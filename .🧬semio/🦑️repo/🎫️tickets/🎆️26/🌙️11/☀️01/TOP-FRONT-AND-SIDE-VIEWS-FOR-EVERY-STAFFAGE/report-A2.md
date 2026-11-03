# Report A2: Animals (mallard-duck, pig, pigeon, red-fox, red-squirrel, roe-deer, sheep)

## State
- Front and plan views exist at every authored scale. Unavailable scales: duck 1:500, pigeon 1:500, fox 1:500, squirrel 1:200 and 1:500.
- `presentation.viewpoints` is removed; `decisions.silhouette` (en, de) is extended.
- Scripts in `T/🧰️A2/`: `lib.py` converts default-pose world metres to bone rest frames, plus `gen_*.py`, `go.py`, `verify.py`, `view.py`, `eval.ts` and `all.ts`.

## Verification
- **Check** (`office-laser`, 6 samples): ok for every authored scale except fox 1:200. There, `cannon-f-n`/`cannon-f-f` measure 0.194 mm against the 0.2 mm minimum. This is probably the elevation shape, which A2 did not touch.
- **Three-views rules and consistency:** ok for every authored scale of all seven assets.
- **Fixed:** the squirrel 1:100 ear gap.
- **Previews:** main poses and clips were inspected for all seven assets, mostly at 1:50, with some at 1:100, 1:200 and 1:500.
- **Not run:** the full suite.

## Shape counts (front/plan)
| Asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| mallard-duck | 19/19 | 10/10 | 7/7 | unavailable |
| pig | 23/23 | 19/19 | 18/18 | 12/12 |
| pigeon | 25/27 | 25/25 | 14/14 | unavailable |
| red-fox | 21/22 | 20/18 | 18/17 | unavailable |
| red-squirrel | 28/28 | 8/8 | unavailable | unavailable |
| roe-deer | 25/23 | 22/20 | 20/18 | 9/9 |
| sheep | 23/23 | 17/17 | 16/16 | 9/9 |

## Doubts (handed to QA8)
- **Torso ovals** follow only y travel, so the front body stays schematic in steep poses (duck upending, squirrel climbing).
- **Wings** cannot spread laterally (duck wing-stretch, pigeon take-off). TS's lateral spread DOF is meant to fix this.
- **Extruded necks and tails** look boxy in front.
- **Fox 1:200 cannon** minimum feature, as above.
