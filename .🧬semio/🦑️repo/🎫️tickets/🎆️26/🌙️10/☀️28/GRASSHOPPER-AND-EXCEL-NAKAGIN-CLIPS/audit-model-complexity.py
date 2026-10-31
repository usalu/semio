"""📊 Record source geometry complexity independently of rendering objects."""
from collections import Counter
import hashlib
import importlib
from pathlib import Path
import sys
import time

ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/"🎓️teaching/🏛️architecture/🏢️nakagin"
M=importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
rows=[]
for layout in M.capsule_families():
    start=time.perf_counter()
    assembly=M.capsule_assembly("A","exterior",layout)
    elapsed=time.perf_counter()-start
    groups=Counter()
    for part in assembly.parts:
        groups["window-unit" if part.solid.meta.get("window_unit") else part.role]+=len(part.solid.faces)
    rows.append((layout,len(assembly.parts),sum(len(p.solid.vertices) for p in assembly.parts),sum(len(p.solid.faces) for p in assembly.parts),elapsed,dict(groups)))
report=["# Source Geometry Complexity","","This measures actual source exterior assemblies, before renderer visibility and opaque-coverage culling. It is a performance audit rather than scene acceptance.","","| Family | Parts | Vertices | Faces | Build Seconds |","|---|---:|---:|---:|---:|"]
for layout,parts,vertices,faces,elapsed,groups in rows:
    report.append(f"| {layout} | {parts} | {vertices} | {faces} | {elapsed:.4f} |")
for row in rows:
    report.extend(("",f"## {row[0]}","",str(row[-1])))
report.extend(("","## Input Fingerprints",""))
for name in ("🐍️.py","🔣️.json"):
    report.append(name+": "+hashlib.sha256((BASE/name).read_bytes()).hexdigest())
(Path(__file__).resolve().parent/"model-complexity.md").write_text("\n".join(report)+"\n",encoding="utf8")
print("[DEBUG] source exterior complexity "+str([(row[0],row[1],row[3],round(row[4],4)) for row in rows]),flush=True)
