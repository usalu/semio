#!/usr/bin/env python3
"""🧾️ The per-asset section of the report of batch MH-5 from the logs of the session: `python3 rows.py <id>…` prints markdown per asset: the findings before (official scan of the poses, the real gate where it ran before, the model gate), what the search kept and left open (`auto/<id>.log`, `ops/<id>.json`), and the proof lines after (`proof/`, `vt/`, the matrix cells). A missing log prints as `not run`."""
import collections, glob, json, os, re, sys

U = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
W, GEN = U + "/🧰️matrix/MH-5", U + "/🗑️generated/matrix/MH-5"


def text(path):
    return open(path).read() if os.path.exists(path) else None


def last(path, cut=220):
    t = text(path)
    return "not run" if not t or not t.strip() else t.strip().splitlines()[-1][:cut]


def scan_row(report, asset):
    t = text(report)
    if not t:
        return "not run"
    row = next((r for r in json.loads(t)["rows"] if r["id"] == asset), None)
    if row is None:
        return "not run"
    hard = {code: n for code, n in row["summary"]["codes"].items() if not code.startswith("group-drift")}
    return f'{row["summary"]["hard"]} hard ({", ".join(f"`{code}` {n}" for code, n in sorted(hard.items())) or "none"}), {row["summary"]["reported"]} reported'


def gate_codes(path):
    t = text(path)
    if not t:
        return "not run"
    codes = collections.Counter()
    for m in re.finditer(r" HARD (\S+) ×(\d+) worst (\d+) mm", t):
        codes[m.group(1)] += int(m.group(2))
    total = re.search(r"hard findings: (\d+)", t)
    return f'{total.group(1) if total else "?"} hard' + (f' ({", ".join(f"`{code}` {n}" for code, n in sorted(codes.items()))})' if codes else "")


def vt(asset, name):
    t = text(f"{GEN}/vt/{asset}-{name}.log")
    if not t:
        return "not run"
    m = re.search(r"Tests\s+(.*)", t)
    return re.sub(r"\s+", " ", m.group(1)).strip()[:60] if m else "no result line"


for asset in sys.argv[1:]:
    print(f"### `{asset}`\n")
    print("| | Before | After |\n|---|---|---|")
    print(f"| Official scan (`T/🔎️scan.ts`, poses) | {scan_row(GEN + '/scan/before.json', asset)} | {last(f'{GEN}/proof/{asset}-scan.log', 120)} |")
    print(f"| Gate of MR (poses, clip instants, transitions) | {gate_codes(f'{GEN}/gate/{asset}.before.log')} | {gate_codes(f'{GEN}/proof/{asset}-gate.log')} |")
    model = text(f"{GEN}/mgate/{asset}.before.log")
    print(f"| Model gate (`mgate.ts`), hard pairs | {len(re.findall(' HARD ', model)) if model else 'not run'} | {last(f'{GEN}/mgate/{asset}.after.log', 120)} |")
    measure = text(f"{GEN}/proof/{asset}-measure.log") or ""
    lines = [line.split(asset + " ", 1)[1] for line in measure.splitlines() if re.search(r"silhouette:|validate:|sweep:", line)]
    print(f"| Silhouette guard, `validateAsset`, three-views sweep | (backup) {'; '.join(l.split(asset + ' ', 1)[1] for l in (text(f'{GEN}/land/{asset}.measure.log') or '').splitlines()[:9] if re.search(r'silhouette:|validate:|sweep:', l)) or 'see check'} | {'; '.join(lines) or 'not run'} |")
    check = text(f"{GEN}/proof/{asset}-check.log") or ""
    mark = next((line[:60] for line in check.splitlines() if line[:1] in "✓✗"), "not run")
    print(f"| Check CLI | | `{mark}` |")
    for name in ("silhouette", "three-views", "physics", "breathing", "validation", "power"):
        print(f"| `{name}` test (name filter) | | {vt(asset, name)} |")
    matrix = f"{GEN}/matrix-{asset}/matrix.json"
    if os.path.exists(matrix):
        cells = json.load(open(matrix))["cells"]
        status = collections.Counter(c["status"] for c in cells)
        fails = collections.Counter(check for c in cells for check in (c.get("failures") or {}))
        print(f"| Matrix (`T/📊️matrix.ts --assets`) | | {len(cells)} cells: **{status.get('fail', 0)} fail**, " + ", ".join(f"{n} {s}" for s, n in sorted(status.items()) if s != "fail") + (f"; failing checks {dict(fails)}" if fails else "") + f"; {last(f'{GEN}/proof/{asset}-matrix.log', 90)} |")
    else:
        print("| Matrix | | not run |")
    print()
    log = text(f"{GEN}/auto/{asset}.log") or ""
    final = log.rsplit("## " + asset + ": ", 1)
    kept = re.findall(r"^KEPT (.*)$", log, re.M)
    opened = re.findall(r"^OPEN (1:\d+ \S+)", log, re.M)
    ops = json.load(open(f"{W}/ops/{asset}.json")) if os.path.exists(f"{W}/ops/{asset}.json") else []
    print(f"Ops in `ops/{asset}.json`: {len(ops)} ({', '.join(f'{n} `{op}`' for op, n in sorted(collections.Counter(op['op'] for op in ops).items()))}).")
    shapes = collections.OrderedDict()
    for op in ops:
        if "scale" in op:
            shapes.setdefault((op["scale"], op.get("bone") or "", op["id"]), []).append(op["op"] + (":" + op["view"] if "view" in op else ""))
    for line in dict.fromkeys(kept):
        print(f"- kept {line}")
    for line in dict.fromkeys(opened):
        print(f"- searched without a passing candidate: {line}")
    print()
