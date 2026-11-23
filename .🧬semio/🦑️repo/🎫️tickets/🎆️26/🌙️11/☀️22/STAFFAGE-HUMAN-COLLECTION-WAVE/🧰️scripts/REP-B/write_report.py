"""📝️ Assembles the repair report of the people of HW-10 … HW-18 (`U/humans-b-repair.md`) from the records of this folder: the tables of report.py, `record/suites.txt`, `record/rows.txt`, `record/sheets.txt`, `record/red.md` (what stays red, written by hand from the final measurements) and `record/files.md` (the file lists).
python3 write_report.py"""
import os
import subprocess
import sys

K = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
U = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
HERE = f"{K}/🧰️scripts/REP-B"


def part(name, missing="(not recorded)"):
    path = f"{HERE}/record/{name}"
    return open(path, encoding="utf8").read().strip() if os.path.exists(path) else missing


table = subprocess.run([sys.executable, f"{HERE}/report.py"], capture_output=True, text=True).stdout.strip()
text = f"""# 🩹️ Humans B Repair: the Catalog Gates of the People of HW-10 … HW-18

Repair author's report of 2026-11-22 22:15 to 2026-11-23 for the umbrella ticket. Scope: the 24 people of the workers HW-10 … HW-18 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (K) in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans`. No person, script or row of HW-01 … HW-09 and no established human was touched; no test, rule, tolerance, engine or schema file was edited; the manifest was not edited.

## Result in short

{part("summary.md")}

## How it was measured

- The real test: `🧪️tests/🧊️catalog-silhouette/🟦️.ts`, run as the `test` command of the package runs one file (`bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🧊️catalog-silhouette/🟦️.ts -t "(<slug>|…)( |$)"` in `📦️packages/🟦️typescript`, pinned bun, `K/🧰️scripts/REP-B/vt.sh`). It stops at the first failing expectation of a person, so the tables below also carry every failing check: bounding box, IoU and largest boundary distance of every scale and view by `measureAsset` and every rule of `checkVolumes` (`K/🧰️scripts/HW-03/measure.ts` through `REP-B/gate.sh`, condensed by `REP-B/summarize.py` into `REP-B/record/before.json` and `after.json`).
- The established base figures were measured once, read only, with the same test: `walking-commuter`, `standing-adult`, `jogger`, `standing-phone-user`, `seated-guitarist`, `sleeping-person` and `tour-guide` all fail it today (for instance `standing-adult 1:500 elevation: expected 0.906 to be greater than or equal to 0.92`, `walking-commuter 1:100 elevation Hausdorff: 0.0302 against 0.03`). The people of this scope therefore could not inherit a passing outline; every repair below was made on the person itself.

## How it was repaired

Every change is authored data of the person and is the last step of its generator: `K/🧰️scripts/HW-1x/repair/<slug>.json` holds the repair ops (idempotent), `K/🧰️scripts/REP-B/apply.ts <slug>` applies them to the file the generator wrote, and `K/🧰️scripts/REP-B/chain.sh <slug>` runs the whole chain (worker's generator, finisher's named patch, repair ops). The ops were produced by these tools of `K/🧰️scripts/REP-B`:

- `areas.ts` (`volumes-thermal-coverage`): a region that the rule reports is declared at the outer surface its volume has at 1:50 (the definition of the README: the faces the air reaches), or at the value that keeps 1:50 and 1:100 inside their limits when the two scales differ; the `exposedFraction` of every pose shrinks by the same factor, so the exchanging area the author stated per pose is unchanged (not for the sunbather, whose regions all carried a placeholder of 0.1 m²).
- `bridge.ts` (`volumes-connected`): parts of the structure that the rule finds apart get a joint capsule between their nearest points (a held broom, cup, belt bag, harness handle and the brush on the canvas are joined to the hand, arm or body that carries them); a loose body solid is moved by the measured gap onto what carries it. Joints are as thick as the smallest feature of their scale (`volumes-feature`).
- `dims.ts` (`volumes-bounds`): an axis that the body misses by more than a smallest feature is declared at the extent of the body of 1:50.
- `nudge.ts` (`volumes-structure-coverage`): a reported structure solid is moved or thinned by a few millimetres until it lies in the body (never a mass marker away from its centre of mass: the easel got the spreader stay it lacked instead, `K/🧰️scripts/HW-12/easel-stay.py`).
- `auto.ts` (the silhouette guard), after `humansb-D-auto.ts` of the 2026-11-17 guard ticket, with four ways to bring a drawing onto its body, each kept only when the view measurably improves and `validateAsset` reports nothing new: (1) a flat shape is replaced by the traced outline of the solids of its bone where the validator allows a flat shape; (2) a drawn shape that has a body solid of the same id is redrawn from that solid as a body of revolution about its bone axis (round limbs: the radial envelope of the solid) or as the tightest prism of the solid, placed by the 3D frame of the bone, so it follows the bone in every pose; (3) a bone whose drawing cannot be matched shape by shape (hands with single fingers, carried objects) is drawn solid by solid as prisms; (4) position, radius or offset of a shape next to a mismatch is fitted by the measured distance. Traces that the guard turned out not to need are pruned again.

Decisions taken (not asked back): the volume is taken as the reference where drawing and volume disagree, because it is the one 3D model all three views must agree with; where the volume itself was wrong (thermal placeholder areas, parts apart, a missing easel stay, structure outside the body) the volume was repaired first. No tolerance was loosened, no scale was copied from another, no shape was deleted without a redrawn replacement of the same part.

## Per person

{table}

Legend: a silhouette entry is a scale and view with IoU below 0.92 or a largest boundary distance above two smallest features (15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500); `box` is a bounding-box axis more than one smallest feature off the declared dimension; the rule counts are `checkVolumes` findings without `volumes-silhouette` (the guard measures the silhouettes).

## What stays red

{part("red.md")}

## The three defects named in `humans-b.md`

{part("defects.md")}

## Other catalog suites, scoped to the 24 people

{part("suites.txt")}

## Sheets opened

{part("sheets.txt")}

## Row files

{part("rows.txt")}

## Files

{part("files.md")}
"""
with open(f"{U}/humans-b-repair.md", "w", encoding="utf8") as f:
    f.write(text)
print("written", f"{U}/humans-b-repair.md", len(text), "characters")
