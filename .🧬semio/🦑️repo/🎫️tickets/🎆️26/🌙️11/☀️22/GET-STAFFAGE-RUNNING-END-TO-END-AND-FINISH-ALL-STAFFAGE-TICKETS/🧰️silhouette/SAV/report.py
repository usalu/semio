"""📝️ Assembles `silhouette-SAV.md` of the umbrella ticket from the measurements of this fixer in 🗑️generated/silhouette/SAV (the checks before, `final/measure.txt`, the suite and check lines of `final.sh`, the real test log), the repair ops of `repair/` and the notes below. Run before the generated folder is deleted.
python3 report.py"""
import collections, glob, json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
U = os.path.abspath(f"{HERE}/../..")
GEN = f"{U}/🗑️generated/silhouette/SAV"
ANIMALS = "dairy-cow domestic-cat goat hedgehog herring-gull horse labrador-dog mallard-duck pig pigeon red-fox red-squirrel roe-deer sheep".split()
VEHICLES = "cargo-bike city-bicycle city-bus city-car delivery-van electric-kick-scooter motor-scooter tram vintage-car-1929".split()
NOTES = {
    "dairy-cow": "1:50 and 1:100: trunk shapes (`chest`, `hindbody`) redrawn as the traced outline of the solids of their bones, pasterns and hooves of the front view drawn from their solids, the near hind pastern outline 1.5 mm in. 1:500: the ear stroke of front and plan is removed, because the 1:500 body carries the ears as `texture` solids (not part of the silhouette) and a `texture` shape has no pen at 1:500 (`shape-role-undrawn`); the sentence about the ear stroke is taken out of `decisions.silhouette`.",
    "domestic-cat": "Declared length only: the bodies are 0.757 to 0.760 m long.",
    "goat": "1:200 plan: tail from its solids, the near thigh gives way to a traced outline of the pelvis solids; 1:500 plan: traces of the two thigh solids added.",
    "hedgehog": "1:50 front `nose`, `dome`, plan `skirt-rear-near` and 1:100 traces of coat, trunk and head solids (greedy pass with 10 mm area bins). The owned checks pass; the real test stops in the third-party oracle (see below).",
    "herring-gull": "1:50 plan: the ninth near primary drawn as the prism of its solid; `near-mirror` on the same bone gave way to it.",
    "horse": "Declared length and depth. 1:50: the tail segments of the side elevation (`tail-1`, `tail-3`, `tail-4`) get the rear outline of their lofts by hand (they were 12 to 20 mm narrower), the plan `tail-1` is 18 mm narrower at the dock, the front `tail-3` 2 mm in; ear and tail prisms from the greedy pass. 1:100: one elevation trace.",
    "labrador-dog": "1:500: hind limbs, trunk, head, ear and tail of elevation and plan redrawn from their solids.",
    "mallard-duck": "1:50, 1:100, 1:200 plan and 1:200 front: wing and tail feathers and trunk drawn from their solids (74 ops). The plan of 1:50 now shows the single tail feathers as a zigzag edge where it had one smooth outline: rougher, kept because it is what the body has.",
    "pig": "1:50 and 1:100 elevation `gaskin-f` from its solids, 1:50 plan traces of the two scapulae added (the body has them proud of the chest).",
    "pigeon": "1:200 elevation and plan: shoulder, tail, trunk and coverts from their solids. The front changes of the greedy pass and a plan prism of the eighth primary were taken back: they broke `spread-front-x-plan-y` of the three-views sweep in the airborne poses.",
    "red-fox": "Declared length only: the bodies are 1.083 to 1.091 m long.",
    "red-squirrel": "1:50 and 1:100 front and plan: the six tail segments are bodies of revolution with `lateralScale` 1.13 to 1.42, which `evaluate` ignores (requests.md, three-views part 1); their profile radius is multiplied by that scale and the member removed (`revolved-lateral.py`), so the tail is as wide as its elliptic body. 1:50 plan `tail-3` as prism; 1:100 elevation: the fingers redrawn from their solids, which closes the opening between hands and muzzle exactly as the body has it (35.8 mm before). Claw and sole prisms of the greedy pass were taken back (`suitability-gap` of the check CLI).",
    "roe-deer": "1:50 plan: `neck` redrawn from its solids.",
    "sheep": "Declared height only: the 1:50 body is 0.838 m high (topknot), the coarser bodies 0.822 to 0.824 m.",
    "cargo-bike": "Declared depth only (1:50 body 0.648 m). The greedy pass found no accepted drawing change in three runs and its last result added a three-views finding (`turning-left` at 1:100), so no drawing op is kept.",
    "city-bicycle": "1:200 elevation: one trace added; 1:50 plan `fork-crown` and `crank-boss-n` from their solids; 1:50 front and plan: the front hub disc is shortened to the hub shell (`bicycle-hub.py`): the body is a stepped hub and the old disc stood 25 mm proud of its flange.",
    "city-bus": "1:100, 1:200, 1:500 front `body-shell` redrawn as the traced outline of the body solids, 1:100 plan `body-shell`, front tyre and door leaf from their solids.",
    "city-car": "Volume first (`city-car-ends.py`): the bodies were 3.941, 3.932, 3.900, 3.900 m long, no declared length fitted all four; front plate, intake and rear diffuser of 1:50 sit 8, 5 and 4 mm closer to the bumpers (3.929 m) and the length is declared at 3.926 m. Drawings: traced outlines of the chassis solids added below the 1:50 plan, front and elevation (`chassis-sil`), 1:50 elevation mirror, exhaust pipe, tyre and the wheel wells from their solids (the drawn wheel-arch gaps give way to the wheelhouse plates the body has), 1:100 front mirrors and `body-shell` as traced outlines.",
    "delivery-van": "Declared length (1:50 body 5.353 m). 1:50 and 1:100: spare wheel, tail pipe and mirrors drawn from their solids, `grille` added at 1:100, front `body-shell` of 1:50 as the traced outline of the body (its flat lower edge lay 24 mm below the underbody), plan and elevation traces of the greedy pass (sliding door, wheels, chassis).",
    "electric-kick-scooter": "1:200: plan deck as prisms of its two solids, front rear wheel from its solid.",
    "motor-scooter": "Declared length (bodies 1.862 to 1.881 m). 1:50: elevation `body` as the traced outline of the frame solids, `cvt` and `seat` from their solids, front rear tyre and stand bar, swing arm traces. Taken back after looking at the sheets: a redrawn rear tyre of 1:100 (a zigzag where the wheel was) and two traces of the whole frame stored under the id `grab-rail` (they covered the footboard ribs).",
    "tram": "Declared height and depth. 1:500 front `shell-end`, 1:200 bogie legs and gangway, 1:100 and 1:50 front wheel, plan end-car trace, pantograph trace of the 1:50 front. Taken back after looking at the sheets: the prisms of the 1:50 side elevation (they hid door frames and bogies), the `pan-base` traces (the outline of the whole end car under that id) and the pantograph-head prisms of the 1:100 front (`suitability-feature` 0.199 mm).",
    "vintage-car-1929": "Nothing kept. The greedy results were looked at and discarded: the 1:100 front lost radiator and headlamps behind a trace of the whole chassis stored as `fender-r`, the 1:50 side elevation lost the spare-wheel rim.",
}
HEAD = """# 🌓️ Silhouette Batch SAV: 14 Animals and 9 Vehicles

Fixer report of 2026-11-23 (16:40 to 21:21) for `📋️silhouette-brief.md`, batch `🧰️silhouette/SAV.tsv`. Every number below was measured in this session with the pinned bun, one tool process at a time. No rule, threshold, test, fixture, engine, schema or README file was touched; the manifest was not edited.

## Result in short

- **Not all green.** The real `🧊️catalog-silhouette` test passes for **15 of 23** assets (0 before): 13 animals (`dairy-cow`, `domestic-cat`, `goat`, `herring-gull`, `horse`, `labrador-dog`, `mallard-duck`, `pig`, `pigeon`, `red-fox`, `red-squirrel`, `roe-deer`, `sheep`) and 2 vehicles (`delivery-van`, `electric-kick-scooter`).
- **8 stay red**: `hedgehog` (every owned check passes; the third-party polygon-clipping oracle of the test throws on this asset, before and after, see `requests.md`), `city-car` (one check, 15.1 mm against 15 mm), `city-bicycle` (one check), `city-bus` (4), `motor-scooter` (4), `tram` (6), `cargo-bike` (7) and `vintage-car-1929` (6, unchanged).
- Failing guard checks (bounding-box axis, IoU, largest boundary distance per scale and view): **96 before, 29 after**.
- `validateAsset` 0 issues and product check CLI `✓` for all 23 at the end; `🧊️three-views` 46 passed and `🌡️catalog-physics` 60 passed for the 23, the same as before the repair.
- Declared dimensions changed for 9 assets: `🧰️silhouette/SAV/manifest-changes.json` (the asset files carry them already; the manifest states width and height only, so `depthM` entries are for the record).

## Method

- Measured with `measure.ts` (`measureAsset`, the path of the guard), located with `where.ts` (one raster) and `tiles.ts` (the 512-cell tiles of the guard, after `🧰️silhouette/SC/tiles.ts`), looked at with `png.ts` (drawing against volume) and `sheet.ts` (product drawings before and after).
- The volume is the reference. A drawn shape that disagrees is redrawn from the solids it draws: as their traced outline where the validator accepts a flat shape on that bone, as a prism or body of revolution placed by the bone frame where it does not (`auto.ts`, the greedy repair of `STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-B`, extended here by a hand-directed mode `MANUAL`, by the tiles of the guard in its measure, by `AREABIN` for small animals and by `SKIP`). Volumes were changed only for `city-car` (ends of the 1:50 body), drawings deleted only where the body has no such part (`dairy-cow` ear stroke at 1:500).
- A declared dimension that no scale could meet is set to the extent of the 1:50 body, moved into the interval that every authored scale accepts (`dims.ts`).
- Every candidate was checked before it was shipped: all guard checks, `validateAsset`, the three-views sweep (`🧰️three-views/tv1-sweep.ts --file`), the product check CLI (`--file`), and the before and after sheets. Results that made a drawing worse were taken back (listed per asset).
- Reproducible: the assets of this batch have no single generator (their history is a chain of patch scripts in the ticket of 2026-11-14), so each repair is one idempotent op list `🧰️silhouette/SAV/repair/<id>.json`, applied by `🧰️silhouette/SAV/apply.ts <id>` as the last step (a second run leaves the file unchanged). `city-car-ends.py`, `revolved-lateral.py`, `bicycle-hub.py` and `dims.ts` are the named scripts that produced the hand-made ops.
"""
TAIL = """A suite line with only skipped tests has no test that names one of the 23 assets (`🫁️breathing-catalog` covers people and trees, `✅️asset-validation` and `⚡️asset-power` fixtures and whole-catalog loops); for those `validateAsset` and the check CLI were run per asset (lines above). Power was not changed.

## Sheets opened

Before and after product drawings (`sheet.ts`: `evaluate`, `depict`, SVG export, light-line view) were opened during the repair at full size for `dairy-cow` 1:50, `herring-gull` 1:50, `pig` 1:50, `mallard-duck` 1:50 and 1:200, `hedgehog` 1:100, `red-squirrel` 1:50, `horse` 1:50, `goat` 1:500, `labrador-dog` 1:500, `delivery-van` 1:50, `city-car` 1:50, `motor-scooter` 1:50 and 1:100, `electric-kick-scooter` 1:200, `city-bus` 1:100, `tram` 1:50, 1:100 and 1:200, `vintage-car-1929` 1:50 and 1:100, and at the end for all 23 at 1:50 and at the coarsest authored scale (`review.sh`, three assets to an image at reduced width). REVIEW_LINE

## What stays red

| Asset | Real test line | Every failing check | Why |
|---|---|---|---|
| `hedgehog` | `Error: Unable to complete output ring starting at [0.1907, 0.0156]` (polygon-clipping, from `exactIou`) | none of the owned checks | The oracle throws in four of six views at every snap, on the file before this repair too (`requests.md`, "silhouette SAV"). Not an asset defect that was found. |
| `city-car` | `1:50 elevation Hausdorff: expected 0.01512 … 0.015` | 1:50 elevation 15.1 mm | One raster cell at the notch between cowl and bonnet at (0.936, 0.989) m. Was 87.9 mm; traces of cowl, scuttle and bonnet and a tighter greedy pass did not move it. |
| `city-bicycle` | `1:50 plan Hausdorff: expected 0.031875 … 0.015` | 1:50 plan 31.9 mm | Two openings between fork crown, head lamp and mudguard at (0.41, ±0.03) m: the lamp is drawn as a box, the body is a cone. A flat trace of the lamp on the steering bone brought 28.1 mm and three new `validateAsset` issues and was taken back. |
| `city-bus` | `1:100 elevation Hausdorff: expected 0.33375 … 0.03` | 1:50 elevation 335.6, front 39.4, plan 58.8 mm; 1:100 elevation 333.8 mm | The 334 mm are a boundary of the volume at the rear end at (0.35, 1.21 to 1.30) m with no drawn boundary near it; its cause was not analysed, nor were the 1:50 front and plan. 1:100 front and plan, 1:200 and 1:500 are green now. |
| `motor-scooter` | `1:500 front: expected 0.9095 … 0.92` | 1:50 plan 43.1 mm; 1:100 elevation 147.6, plan 45.0 mm; 1:500 front IoU 0.9095 | Plan: the front shock absorber of the body at (0.58, -0.14) m is not drawn, and the drawn mudguard edge at y -0.097 m has no body edge within 43 mm (steering bone: a flat trace is not valid there, a fitted solid was not accepted). 1:100 elevation: rear wheel against its hub and transmission case; the greedy redraw looked wrong and was taken back. 1:500 front not worked on. |
| `tram` | `1:200 front Hausdorff: expected 0.17557 … 0.06` | 1:50 elevation 30.8, front 39.4, plan 33.5 mm; 1:100 elevation 42.8, front 165.0 mm; 1:200 front 175.6 mm | The pantograph: its upper arm and head in the front view (1:50, 1:100, 1:200) and its base and knee in the side elevation (1:50, 1:100). The prism redraws were taken back (see above); the 1:50 plan was not analysed again after the end-car trace. 1:500, 1:200 elevation and plan, 1:100 plan are green now; the bounding box passes at every scale. |
| `cargo-bike` | `1:200 elevation Hausdorff: expected 0.07115 … 0.06` | 1:50 elevation 64.4, front 24.4, plan 48.8 mm; 1:100 elevation 67.9, front 56.2, plan 53.7 mm; 1:200 elevation 71.2 mm | The worst place of the 1:50 side elevation lies inside the front wheel at (2.01, 0.20) m; redraws of rim, tyre and wheel were measured and not accepted (fork, bar and front wheel are bones on which no flat trace is valid). Only the depth is declared now. |
| `vintage-car-1929` | `1:100 elevation Hausdorff: expected 0.06047 … 0.03` | as before: 1:50 elevation 88.1, front 53.3, plan 97.1 mm; 1:100 elevation 60.5, front 33.8, plan 72.1 mm | Worst places as measured at the start: front wheel rim and rear drum (side elevation), frame rail at the running-board bracket (front), steering arm and tie rod (plan). Not repaired; the file is unchanged. |

Limits of the method, stated plainly: traced outlines are plainer than hand-drawn shapes (the `city-car` side elevation has no wheel-arch gap any more, the `mallard-duck` plan shows single tail feathers), and a few traces lie under an id that names only part of what they outline (`chassis-sil`, `pelvis-sil`, `trunk-sil`). Seven vehicles were not brought to green in this session.

## Requests

- `requests.md`, section "silhouette SAV — `🧊️catalog-silhouette`: the polygon-clipping oracle throws on `animals/hedgehog` at every snap (stays red)".
- Met again, already filed by others: `lateralScale` of a revolved 2D shape is ignored (three-views part 1; worked around in `red-squirrel`), the tiles of the guard read another distance than one raster (silhouette SF, SC; a discarded candidate of `vintage-car-1929` read 72.1 mm in the tiles of the guard against 18.8 mm on one raster in the 1:100 plan). `auto.ts` and `tiles.ts` of this folder therefore measure the tiles too.

## Files

Created (repo-relative, under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):

- `silhouette-SAV.md`
FILES_CREATED

Updated:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (one section appended)
FILES_UPDATED

Removed:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🗑️generated/silhouette/SAV` (tool output: backups, candidates, logs, sheets)
"""
counts = json.load(open(f"{GEN}/final/counts.json"))
before, after = counts["before"], counts["after"]
real = {}
log = open(glob.glob(f"{GEN}/vt/final-*catalog-silhouette.log")[0], errors="replace").read()
for m in re.finditer(r"^AssertionError: (\S+) (.*)$", log, re.M):
    real.setdefault(m.group(1), m.group(2))
checks = {}
for line in open(f"{GEN}/final/check.txt"):
    m = re.match(r"^(✓|✗) (\S+)", line)
    if m:
        checks[m.group(2)] = line.strip()
suites = [l.strip() for l in open(f"{GEN}/final/suites.txt") if l.startswith("== ")]
manifest = json.load(open(f"{HERE}/manifest-changes.json"))

def ops_of(a):
    path = f"{HERE}/repair/{a}.json"
    if not os.path.exists(path):
        return "none"
    c = collections.Counter()
    for o in json.load(open(path)):
        if o["op"] == "dims":
            c["declared dimensions"] += len(o["set"])
        elif o["op"].startswith("solid"):
            c[f"1:{o['scale']} volume solids moved"] += 1
        elif o["op"] == "set":
            c["decision text"] += 1
        else:
            c[f"1:{o['scale']} {o['view']}"] += 1
    return ", ".join(f"{k} ×{v}" for k, v in c.items())

short = lambda l: "; ".join(l) if l else "none"
out = [HEAD.rstrip(), ""]
out += ["## Per asset", "", "Failing checks are those of `measureAsset` as the guard reads it (`🧰️silhouette/SAV/measure.ts`: bounding box per axis against one smallest feature, IoU 0.92 and largest boundary distance against two smallest features per authored scale and view), measured before on the files of 2026-11-23 16:41 and after on the catalog files at the end. The real test line is the result of `🧊️catalog-silhouette` for the asset in the final run.", ""]
for a in ANIMALS + VEHICLES:
    passed = a not in real and a != "hedgehog"
    out += [f"### `{a}`", "",
            f"- Before ({len(before[a])}): {short(before[a])}",
            f"- After ({len(after[a])}): {short(after[a])}",
            f"- Real `🧊️catalog-silhouette` test: {'passes' if passed else ('`Error: Unable to complete output ring …` of the polygon-clipping oracle (see `requests.md`)' if a == 'hedgehog' else '`AssertionError: ' + a + ' ' + real[a] + '`')}",
            f"- `validateAsset`: 0 issues (before: 0); check CLI: `{checks.get(a, 'not run')}`",
            f"- Repair ops `🧰️silhouette/SAV/repair/{a}.json`: {ops_of(a)}" + (f"; manifest change: `{json.dumps(manifest[a])}`" if a in manifest else ""),
            f"- Changed: {NOTES[a]}", ""]
out += ["## Other suites, name filter over the 23 assets (final run, `final.sh`)", ""] + ["- `" + re.sub(r"\s+×.*$", "", s[3:]).strip() + "`" for s in suites] + [""]
created = "\n".join(f"- `🧰️silhouette/SAV/{os.path.relpath(f, HERE)}`" for f in sorted(glob.glob(f"{HERE}/*") + glob.glob(f"{HERE}/repair/*")) if os.path.isfile(f))
updated = "\n".join(f"- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/{'🐕️animals' if a in ANIMALS else '🚜️vehicles'}/{a}/🔣️.json`" for a in ANIMALS + VEHICLES if os.path.exists(f"{HERE}/repair/{a}.json"))
review = "In the final review every asset draws all three views at both scales and no drawing has lost a part against its state before; seen and kept: the plainer wheel arches of `city-car`, the feathered tail edge of the `mallard-duck` plan, the angular mirrors of `delivery-van` and `city-car` (the outline of their solids). At the reduced width of the review, parts smaller than a few centimetres cannot be judged."
out += [TAIL.rstrip().replace("FILES_CREATED", created).replace("FILES_UPDATED", updated).replace("REVIEW_LINE", review), ""]
open(f"{U}/silhouette-SAV.md", "w").write("\n".join(out))
print("written", sum(len(v) for v in before.values()), "->", sum(len(v) for v in after.values()))
