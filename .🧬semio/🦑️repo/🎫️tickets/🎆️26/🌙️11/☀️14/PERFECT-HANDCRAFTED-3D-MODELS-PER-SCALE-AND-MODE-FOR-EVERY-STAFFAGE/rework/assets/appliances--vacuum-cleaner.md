# appliances/vacuum-cleaner — rework report

## Pass 1 (second round)

Author script (hand-chosen numbers, re-runnable from the pre-pass source): `rework-vc-pass1.py`; render helper `rework-vc-render.sh`. Source `volumes/appliances/vacuum-cleaner.json` and catalog were identical at the start (no half-finished edit from the earlier pass) and are identical again after publishing.

### Reference table (canister vacuum cleaner, EN 60312 class, from my own knowledge of typical products)

| Feature | Reality | Model before | Model after |
|---|---|---|---|
| Canister L x W x H | 0.42–0.50 x 0.28–0.31 x 0.22–0.26 m | 0.50 x 0.25 (0.30 over wheels) x 0.25 m, handle to 0.28 | unchanged |
| Rear wheels | diameter 120–180 mm, tread 20–30 mm, on stub axles or bosses moulded to the shell flanks | 120 x 25 mm, **6.6 mm clear of the shell, nothing between wheel and canister** | 120 x 25 mm on two 40 mm bearing bosses of the shell |
| Front caster | one swivel caster, twin wheel 40–50 mm, 2 x 12 mm | twin wheel 50 mm in a fork, copied unchanged to all scales | 1:50 unchanged; coarser scales one roller |
| Hose | 32–45 mm outside, 1.5–1.8 m, corrugated | 36 mm, ten links of 0.15 m, three-step skinned bends | unchanged |
| Handle | bent tube with air slide, 40–45 mm | 42 mm sleeve, slide, swivel | unchanged |
| Wand | 32/35 mm steel, telescopic 0.6–1.0 m | 35 mm, 0.86 m, inner tube, collar, lock | unchanged |
| Floor nozzle | 260–300 mm wide, 80–90 mm deep, swivel neck | 285 mm, yoke, ball, two bristle strips | unchanged |
| Motor-fan unit / dust bag | 110–140 mm fan stage, 80–95 mm can; 3–4.5 l bag | present in thermal (earlier pass) | unchanged |

### Defects found (measured, baseline `r3-validate --volumes`: INVALID, validateAsset 14, checkVolumes 22)

- Body 1:50, 1:100, 1:200: both rear wheels floated 6.5 mm beside the shell; the axle existed only in the structure list.
- Body 1:100, 1:200, 1:500: the running gear was the 1:50 parts list (12 mm caster wheels, 20.5 mm rims, 26 mm fork, 25 mm tyres at a 75 mm feature).
- Structure 1:50: the nozzle housing did not meet the neck (4.3 mm; pivot was a 10 mm pin inside a 33 mm bore), the grip mass marker hung 4 mm clear inside the handle tube, nothing tied the hose chain to the tub.
- Structure 1:200: base tub ended 6.7 mm under the tub wall (open load path); 1:500: no base at all, caster and supports 41.7 mm from the wall.
- Decisions: 1:50 said the motor was not modelled while a later entry described it; 1:100 to 1:500 described 25 mm tyres and 12 mm swivel wheels that are below the feature of those scales.

### Rebuilt

- **Body 1:50 / 1:100**: `wheel-boss-l/r` (revolves on the axle line, 40 mm, from inside the shell flank to the rim) so each wheel meets the canister at its joint. 1:100: the twin caster is one 48 mm roller `caster-wheel` in its fork (as the 2D drawing of that scale shows it).
- **Body 1:200**: each rear wheel one 120 x 30 mm disc turned with its bearing boss (`rear-tyre-l/r`), rims removed; caster one 50 mm roller reaching into the shell, fork removed.
- **Body 1:500**: rear wheel pair one 120 mm roller over the 0.30 m track (`rear-wheels`, the `wheel-pair` of the 2D plan and front), caster one 50 x 75 mm roller; canister top re-lofted through eight plan sections instead of the abrupt six-section cap.
- **Thermal**: wheel bosses added to region `body-shell` (1:50, 1:100); wheels region follows the per-scale wheels and rollers at 1:100 to 1:500 (the true-size discs I first tried at 1:500 are below the feature there).
- **Structure**: 1:50 `member-socket` (hose socket sleeve on the tub), wheel bosses as members, the nozzle pivot as the real 35 mm barrel between both yoke cheeks (`member-yoke-l/r`), grip mass marker fills the bore. 1:100 socket sleeve with a 15 mm wall. 1:200 base tub drawn up into the wall, socket plug, supports on the new discs and roller. 1:500 a 75 mm base tub under the wall, supports on the wheel roller and caster roller.
- **Rig, poses, clips, 2D**: not changed. The 2D drawings were checked with `r3-2d` at 1:50 and 1:500 and agree with the model (they already drew one caster wheel from 1:100 and a wheel pair at 1:500); the silhouette rules pass.
- **Decisions**: rewritten for all four scales (en, de) to state what is modelled now.

### Looked at after the last change

1:50 body: running (six views before the thermal-only last edit), parked (six views), running-pulled (six views), clip `floor-stroke` t = 1 s, back close-up, thermal top view; 1:50 structure three-quarter; 1:100 body and thermal cut; 1:200 body and structure; 1:500 body and thermal cut; 2D 1:50 and 1:500.

### Validation

`r3-validate.ts appliances/vacuum-cleaner --catalog-only --volumes` → `VALID appliances/vacuum-cleaner (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### What remains imperfect (stated plainly)

- Rig: the caster has only its rolling axis; the swivel about the vertical stem cannot be expressed in this planar rig (no `rig-lateral-rest.md` in the ticket yet). The rear wheels share one bone (they are independent in reality).
- 1:50 body: the hose is ten 0.15 m links with three-step bends, so tight bends show knuckles (running pose near the floor, the fold at the handle is a sharp V where a real handle is a bent tube of about 80 mm radius). No mains cable and plug. No suction dial, cord-rewind detail or parking rail on the canister.
- 1:200 / 1:500 body: the canister loft shows flat facets in the side view (linear between sections); at 1:500 the wheel roller shows as a bar behind the tapering rear of the shell in back and top view.
- 1:500 thermal: the wheels region is the roller (about twice the true wheel surface); the rule accepted it, the number is information only.
- Not read in this pass although rendered: clip `floor-stroke` t = 3 s, clip `run-and-park` t = 2 s and t = 300 s, 1:50 running six-view after the last (thermal-only) edit, 1:50 thermal cut after the edit, 1:100 structure, 1:200 thermal cut, 1:500 structure, 2D 1:100 and 1:200.

### Fundamental issues (outside this asset)

- `validator.md` has no section "Rule changes authors must know" (the task refers to it); I read the whole head of the file instead.
- `volumes-connected`, reason `near`: a `mass` marker is named as the nearest solid of "the rest" ("member-hose-1 (+26) 4.0 mm from mass-grip"), although the report says mass markers float. Either the marker should not count as "the rest" or the message should name the nearest non-mass solid; the finding sent me to the wrong part first.
- The same message says the hose group "stands on its own supports" although it holds no support solid; the wording hides that the real defect was the missing tie between hose and tub.
