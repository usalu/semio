"""📊️ The proof table of lane CL-T1 as Markdown, read from `GEN/light/<id>.txt`, `GEN/proof/<id>.txt` and `GEN/proof/<id>-modelcat.txt`: per asset the light proof, the GLB of the default pose, gate, scan, guard, check CLI, matrix cells and the findings the single-asset `🧊️model-catalog` run still names. table.py"""
import glob, os, re

GEN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🗑️generated", "matrix", "CL-T1")
print("| Asset | Light proof | GLB default pose | Gate | Scan | Guard, `validateAsset`, sweep | Check CLI | Matrix | `🧊️model-catalog` single run | Digest after |")
print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
for path in sorted(glob.glob(os.path.join(GEN, "light", "*.txt"))):
    a = os.path.basename(path)[:-4]
    light = open(path, encoding="utf-8").read()
    proof_path = os.path.join(GEN, "proof", a + ".txt")
    proof = open(proof_path, encoding="utf-8").read() if os.path.exists(proof_path) else ""
    find = lambda pattern, text, default="": (re.search(pattern, text) or [default, default])[1] if re.search(pattern, text) else default
    glb = find(r"glb of the default pose: (\d+) bytes", light)
    cells = find(r"(\d+ cells: [^;]*fail)", proof)
    status = find(r"status (\{[^}]*\})", proof)
    seconds = find(r"missing \((\d+) s\)", proof)
    cat = (re.findall(r"model-catalog exit (\d+)", proof) or [""])[-1]
    names_path = os.path.join(GEN, "proof", a + "-modelcat.txt")
    names = [l.strip().strip('",') for l in open(names_path, encoding="utf-8")] if os.path.exists(names_path) else []
    kinds = {}
    for n in names:
        k = "GLB size" if "GLB of" in n else "KHR_mesh_quantization (writer in flight)" if "KHR_mesh_quantization" in n else "triangles" if "triangles" in n else "bounds" if n.startswith("bounds") else n.split(" ")[0]
        kinds[k] = kinds.get(k, 0) + 1
    sizes = sorted(float(m) for n in names for m in re.findall(r"GLB of ([0-9.]+) MB", n))
    cat_text = "not run" if cat == "" else "green" if cat == "0" else ", ".join(f"{v} {k}" for k, v in kinds.items()) + (f" ({sizes[0]} to {sizes[-1]} MB)" if sizes else "")
    guard = "OK, 0, 0" if "silhouette: OK" in proof and "validate: 0 issues" in proof and "sweep: 0 findings" in proof else ("-" if proof == "" else "see log")
    print(f"| `{a}` | {find(r'LIGHT PROOF .*: ([A-Z]+)', light, '-')} | {'' if glb == '' else f'{int(glb) / 1048576:.2f} MiB'} | {find(r'gate exit (0 hard findings: 0)', proof) and '0 hard'} | {'clean' if '1 clean' in proof else ''} | {guard} | {'`✓`' if 'check: exit 0 ✓' in proof else ''} | {cells} {status} ({seconds} s) | {cat_text} | `{find(r'sha-256 ([0-9a-f]{16}),', proof)}` |")
