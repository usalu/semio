"""🧾️ The per-plant rows of the report of batch MV-1: `python3 report-rows.py <generated dir>` reads `before.json` beside this file (scan and gate before the repair), the proof lines `proof/<id>.txt`, the logs of `make.sh` (`log/<id>-make-*.log`) and the saved anchor ops, and prints one markdown section per plant: findings before and after by code, what was changed, the proof lines as printed."""
import json, os, re, sys

here = os.path.dirname(os.path.abspath(__file__))
gen = sys.argv[1]
before = json.load(open(os.path.join(here, "before.json")))
flags = {line.split("\t")[0]: line.rstrip("\n").split("\t") for line in open(os.path.join(here, "plants.tsv"))}


def read(path):
    return open(path).read() if os.path.exists(path) else ""


for plant in sorted(before):
    was = before[plant]
    proof = read(f"{gen}/proof/{plant}.txt").strip().splitlines()
    carry = read(f"{gen}/log/{plant}-make-carry.log")
    plates = re.search(r"(\d+) plates with (\d+) points", carry)
    bones = sorted({(m[1], m[2]) for m in re.finditer(r"^(\d+) (?:front|plan) (\S+): .*stations, [1-9]\d* plates", carry, re.M)})
    worst_before = max([int(m[1]) for m in re.finditer(r": (\d+) mm @ .*stations, [1-9]", carry)] or [0])
    worst_after = max([int(m[1]) for m in re.finditer(r"-> (\d+) mm @ .*stations, [1-9]", carry)] or [0])
    ops = json.loads(read(os.path.join(here, "ops", f"{plant}.json")) or "[]")
    print(f"### `{plant}`\n")
    print(f"- Before: scan {was['scan']} hard ({', '.join(f'{k} {v}' for k, v in was['scanCodes'].items())}); gate over every pose, clip instant and transition {was['gate']} hard ({'; '.join(f'{k} {v}' for k, v in was['gateCodes'].items())}).")
    gate = next((line for line in proof if line.startswith("gate")), "gate not run")
    scan = next((line for line in proof if line.startswith("scan")), "scan not run")
    print(f"- After: {gate}; {scan}.")
    changed = []
    row = flags.get(plant, [plant, "", "", ""]) + ["", "", ""]
    if row[3].startswith("rebind"):
        changed.append("`rebind.ts" + row[3][6:] + "`")
    if os.path.exists(os.path.join(here, f"fix-{plant}.ts")):
        changed.append(f"`fix-{plant}.ts`")
    if plates and int(plates[1]) > 0:
        scales = sorted({s for s, _ in bones}, key=int)
        names = sorted({b for _, b in bones})
        changed.append(f"`carry.ts{(' ' + row[1]) if row[1] else ''}`: {plates[1]} depth plates ({plates[2]} points) below the front and plan outlines of {len(names)} bones at 1:{', 1:'.join(scales)} ({', '.join(names[:8])}{', …' if len(names) > 8 else ''}); worst bone drift {worst_before} mm before, {worst_after} mm after")
    if ops:
        moved = [f"{op['id']} on {op['set']['bone']}" for op in ops]
        changed.append(f"`anchors.ts --ops ops/{plant}.json`: {len(ops)} anchors ({', '.join(moved[:6])}{', …' if len(moved) > 6 else ''})")
    print(f"- Changed (`make.sh {plant}`): {'; '.join(changed) if changed else 'nothing'}.")
    print("\n```")
    for line in proof:
        print(f"{plant} | {line}")
    print("```\n")
