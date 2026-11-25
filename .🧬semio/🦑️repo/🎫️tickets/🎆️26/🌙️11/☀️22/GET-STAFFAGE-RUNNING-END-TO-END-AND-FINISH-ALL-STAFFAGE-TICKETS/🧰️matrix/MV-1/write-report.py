"""📝️ Writes the report of batch MV-1: `python3 write-report.py <generated dir> <out.md> <state line>` writes the state (`FINAL` or `IN WORK …`) as the first line and joins the fixed text below with the result table and the per-plant sections that `report-rows.py` builds from the proof lines, the logs of `make.sh` and `before.json`; `sheets.txt` and `open.txt` beside this file are set in as the sections on the sheets and on what stays open."""
import json, os, re, subprocess, sys

here = os.path.dirname(os.path.abspath(__file__))
gen, out, state = sys.argv[1], sys.argv[2], sys.argv[3]
before = json.load(open(os.path.join(here, "before.json")))


def read(path):
    return open(path).read() if os.path.exists(path) else ""


rows = ["| Plant | Scan before (hard) | Gate before (hard, every subject) | Gate after | Scan after | Matrix after | Other proofs |", "|---|---|---|---|---|---|---|"]
for plant in sorted(before):
    proof = read(f"{gen}/proof/{plant}.txt")
    gate = re.search(r"gate exit \d+ hard findings: (\d+)", proof)
    scan = re.search(r"(\d+) hard findings\s*$", next((line for line in proof.splitlines() if line.startswith("scan")), ""))
    matrix = re.search(r"\[matrix\] (\d+) cells: (\d+) ok, (\d+) unavailable, (\d+) fail.*?\((\d+) s[^)]*\)", proof)
    lines = [line for line in proof.splitlines() if not line.startswith(("gate", "scan", "matrix", "done"))]
    green = [line for line in lines if re.search(r"exit 0|silhouette: OK|validate: 0 issues|sweep: 0 findings", line) and not re.search(r"\d+ failed", line)]
    rows.append(f"| `{plant}` | {before[plant]['scan']} | {before[plant]['gate']} | {gate[1] if gate else 'not run'} | {scan[1] if scan else 'not run'} | {f'{matrix[1]} cells, {matrix[2]} ok, **{matrix[4]} fail** ({matrix[5]} s)' if matrix else 'not run'} | {'all green' if lines and len(green) == len(lines) else f'{len(lines) - len(green)} of {len(lines)} lines not green, see section 3'} |")
total_scan = sum(v["scan"] for v in before.values())
total_gate = sum(v["gate"] for v in before.values())
result = "\n".join(rows) + f"\n\nSum before: {total_scan} hard findings in the scan, {total_gate} in the gate over every subject."
sections = subprocess.run([sys.executable, os.path.join(here, "report-rows.py"), gen], capture_output=True, text=True).stdout
head = read(os.path.join(here, "report-head.txt")).replace("@RESULT@", result)
tail = read(os.path.join(here, "report-tail.txt"))
open(out, "w").write(state + "\n\n" + head + sections + read(os.path.join(here, "sheets.txt")) + read(os.path.join(here, "open.txt")) + tail)
print(f"written {out}")
