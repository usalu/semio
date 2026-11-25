#!/usr/bin/env python3
"""📰️ Builds `U/multiview-MH-5.md` from `report-template.md`, the hand-written `report-open.md` and `report-files.md`, and the logs of the session (the generated folder is deleted at the end, so the report carries every number): `python3 report.py`. The state table has one row per asset: the hard findings of the real gate before (where it ran) and after, the matrix cells, and whether the catalog file carries the repair."""
import collections, json, os, re, subprocess

U = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
W, GEN = U + "/🧰️matrix/MH-5", U + "/🗑️generated/matrix/MH-5"
assets = [line.split("\t")[1] for line in open(U + "/🧰️matrix/batches/MH-5.tsv").read().splitlines() if line.strip()]
text = lambda path: open(path).read() if os.path.exists(path) else ""
scan = {row["id"]: row for row in json.loads(text(GEN + "/scan/before.json") or '{"rows": []}')["rows"]}


def hard(path):
    m = re.search(r"hard findings: (\d+)", text(path))
    return m.group(1) if m else "not run"


rows = ["| Asset | Scan before (poses, hard) | Gate before (all subjects, hard) | Model gate before (hard pairs) | Gate after (hard) | Scan after | Matrix after | Catalog file |", "|---|---|---|---|---|---|---|---|"]
for a in assets:
    model = len(re.findall(r"1:\d+ HARD ", text(f"{GEN}/mgate/{a}.before.log")))
    matrix = "not run"
    if os.path.exists(f"{GEN}/matrix-{a}/matrix.json"):
        cells = json.load(open(f"{GEN}/matrix-{a}/matrix.json"))["cells"]
        status = collections.Counter(c["status"] for c in cells)
        matrix = f"{len(cells)} cells: **{status.get('fail', 0)} fail**, " + ", ".join(f"{n} {s}" for s, n in sorted(status.items()) if s != "fail")
    after = re.search(r"\[scan\].*", text(f"{GEN}/proof/{a}-scan.log"))
    landed = "repaired" if os.path.exists(f"{GEN}/landed/{a}.json") else "unchanged"
    rows.append(f"| `{a}` | {scan[a]['summary']['hard'] if a in scan else 'not run'} | {hard(f'{GEN}/gate/{a}.before.log')} | {model} | {hard(f'{GEN}/proof/{a}-gate.log')} | {after.group(0)[7:60] if after else 'not run'} | {matrix} | {landed} |")
detail = subprocess.run(["python3", W + "/rows.py"] + assets, capture_output=True, text=True).stdout
out = text(W + "/report-template.md").replace("STATE-TABLE", "\n".join(rows)).replace("DETAIL-SECTIONS", "## 4. Per asset\n\n" + detail).replace("FILES", text(W + "/report-files.md").strip()).replace("OPEN", text(W + "/report-open.md").strip())
open(U + "/multiview-MH-5.md", "w").write(out)
print("written", len(out))
