"""📝️ Assembles `multiview-MC.md` from `report-body.md`, the per-asset rows `report-rows.md` (asset, before, after, change), the decisions `report-decisions.md` and the matrix lines of `matrix.sh` (`matrix.txt` beside this file, or the log given): `python3 report.py <IN WORK|FINAL> [<matrix log>]`. The first line of the report is the status; FINAL is refused while an asset of the batch has no matrix line or a fail cell."""
import ast, os, re, sys, time
here = os.path.dirname(os.path.abspath(__file__))
status = sys.argv[1]
log = sys.argv[2] if len(sys.argv) > 2 else f"{here}/matrix.txt"
ids = [l.split("\t")[0] for l in open(f"{here}/../batches/MC.tsv")]
runs = {}
for line in open(log):
    parts = [p.strip() for p in line.split(" | ", 4)]
    if len(parts) == 5:
        m = re.match(r"status (\{.*?\})\s*(.*)", parts[4])
        runs[parts[0]] = (parts[1], parts[2], int(parts[3].split()[1]), ast.literal_eval(m.group(1)) if m else {}, (m.group(2) if m else parts[4]).strip())
rows, clean = [], 0
for row in open(f"{here}/report-rows.md"):
    cells = [c.strip() for c in row.strip().strip("|").split(" | ", 3)]
    a = cells[0].strip("`")
    if a in runs:
        code, secs, peak, cellsby, checks = runs[a]
        ok = code == "exit 0" and cellsby.get("fail", 0) == 0 and sum(cellsby.values()) > 0
        clean += ok
        matrix = f"{', '.join(f'{n} {k}' for k, n in sorted(cellsby.items()))}; {secs}, {peak} MB" + ("" if ok else f"; {code}; {checks[:240]}")
    else:
        matrix = "not run"
    rows.append(f"| {cells[0]} | {cells[1]} | {cells[2]} | {matrix} | {cells[3]} |")
ran = [a for a in ids if a in runs]
if status == "FINAL" and (len(ran) < len(ids) or clean < len(ids)):
    sys.exit(f"not FINAL: {len(ran)} of {len(ids)} run, {clean} without a fail cell")
peaks = [runs[a][2] for a in ran] or [0]
body = open(f"{here}/report-body.md").read()
for key, value in {
    "@STATUS@": status,
    "@END@": time.strftime("%H:%M"),
    "@CLEAN20@": str(clean),
    "@MATRIX_LINE@": f"{clean} of {len(ran)} run without a `fail` cell" + ("" if len(ran) == len(ids) else f", {len(ids) - len(ran)} not yet run"),
    "@ROWS@": "\n".join(rows),
    "@DECISIONS@": open(f"{here}/report-decisions.md").read().strip(),
    "@PEAKS@": f"{min(peaks)} to {max(peaks)} MB",
    "@REMOVED@": "`🗑️generated/matrix/MC` (backups, trial copies, boxes, logs, sheets, matrix folders)." if status == "FINAL" else "`🗑️generated/matrix/MC` stays until the report is FINAL.",
}.items():
    body = body.replace(key, value)
open(f"{here}/../../multiview-MC.md", "w").write(body)
print(f"{status}: {clean} of {len(ran)} run without a fail cell")
