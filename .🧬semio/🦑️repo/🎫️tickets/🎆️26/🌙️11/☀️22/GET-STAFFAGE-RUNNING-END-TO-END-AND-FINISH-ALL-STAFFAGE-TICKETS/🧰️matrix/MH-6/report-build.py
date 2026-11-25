"""🏗️ Assembles `U/multiview-MH-6.md` from the written part (`report-head.md.txt`, `report-tail.md.txt`) and the logs: `python3 report-build.py <GEN> <U>` fills the result table (hard findings before and after, matrix cells) and appends the proof lines of every asset from `report-data.py`."""
import json, os, re, subprocess, sys
gen, u = sys.argv[1], sys.argv[2]
here = os.path.dirname(os.path.abspath(__file__))
ids = [line.split("\t")[1] for line in open(f"{u}/🧰️matrix/batches/MH-6.tsv") if line.strip()]
def total(path):
    if not os.path.exists(path): return None
    m = re.findall(r"^hard findings: (\d+)", open(path).read(), re.M)
    return int(m[-1]) if m else None
rows = ["| Asset | Hard findings before (gate) | After | Matrix after (252 cells) |", "|---|---|---|---|"]
for a in ids:
    before, after = total(f"{gen}/gate0/{a}.log"), total(f"{gen}/proof/{a}-gate.log")
    if after is None: after = f"{total(f'{gen}/trylog/{a}.gate.log')} (gate of the trial copy; proof not finished)"
    cells = "not finished in this session"
    out = f"{gen}/proof/{a}.out"
    if os.path.exists(out):
        text = open(out).read()
        m = re.search(r"\[matrix\] (\d+) cells: (\d+) ok, (\d+) unavailable, (\d+) fail", text)
        s = re.search(r"^status (\{.*\})", text, re.M)
        if m: cells = f"**{m.group(4)} fail**" + (", " + ", ".join(f"{v} {k}" for k, v in sorted(eval(s.group(1)).items()) if k != "fail") if s else "")
    rows.append(f"| `{a}` | {before} | **{after}** | {cells} |")
sums = [sum(v for v in values if v is not None) for values in zip(*[(total(f"{gen}/gate0/{a}.log"), total(f"{gen}/proof/{a}-gate.log") if total(f"{gen}/proof/{a}-gate.log") is not None else total(f"{gen}/trylog/{a}.gate.log")) for a in ids])]
head = open(f"{here}/report-head.md.txt").read().replace("@@TABLE@@", "\n".join(rows)).replace("@@GONE@@", f"the batch had {sums[0]:,} hard findings and keeps {sums[1]:,}")
proof = subprocess.run([sys.executable, f"{here}/report-data.py", gen, *ids], capture_output=True, text=True).stdout
tail = open(f"{here}/report-tail.md.txt").read()
open(f"{u}/multiview-MH-6.md", "w").write(head.rstrip() + "\n\n## 4. Proof per asset (each run to its end in this session, after the last change of the asset)\n\nPer asset: the hard findings of the gate by code before and after (count, largest delta, scales), then the lines of `proof.sh`: gate, official scan `T/🔎️scan.ts`, silhouette guard, `validateAsset` and three-views sweep (`MR/measure.ts`), check CLI, the real tests with a name filter, `T/📊️matrix.ts --assets <id>` and its cells by status.\n\n" + proof.rstrip() + "\n\n" + tail)
print("\n".join(rows))
