"""📝️ Writes the report of batch R3-V (`silhouette-R3-V.md` in the ticket folder) from the measurements before and after (🗑️generated/silhouette/R3-V/before.txt, after.txt), the proof lines of prove.sh (proof-<id>.txt), the ops files and the texts per asset of notes.json beside this file: python3 report.py"""
import json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
U = os.path.abspath(f"{HERE}/../..")
GEN = f"{U}/🗑️generated/silhouette/R3-V"
ORDER = ["city-car", "city-bicycle", "city-bus", "motor-scooter", "tram", "cargo-bike", "vintage-car-1929"]
notes = json.load(open(f"{HERE}/notes.json"))


def measured(path):
    """📐️ Per asset the lines of measure.ts and the failing checks in short."""
    out = {}
    for line in open(path):
        m = re.match(r"(\S+) 1:(\d+) f=\S+ bbox (.*?)  (elevation.*)", line)
        if not m:
            continue
        fails = [f"1:{m[2]} axis {k} {b.rstrip('!')}" for k, b in enumerate(m[3].split()) if b.endswith("!")]
        for view in re.findall(r"(\w+)(?:\(s\))? ([\d.]+)(!?)/([\d.]+)mm(!?)", m[4]):
            if view[2]:
                fails.append(f"1:{m[2]} {view[0]} IoU {view[1]}")
            if view[4]:
                fails.append(f"1:{m[2]} {view[0]} {view[3]} mm")
        entry = out.setdefault(m[1], {"lines": [], "fails": []})
        entry["lines"].append(f"1:{m[2]} " + re.sub(r"\s+", " ", m[4]))
        entry["fails"] += fails
    return out


before, after = measured(f"{GEN}/before.txt"), measured(f"{GEN}/after.txt")
text = [notes["head"], ""]
total_before = sum(len(before[a]["fails"]) for a in ORDER)
total_after = sum(len(after[a]["fails"]) for a in ORDER)
text += ["## Result", "", notes["result"].replace("{before}", str(total_before)).replace("{after}", str(total_after)), "", "## Method", "", notes["method"], "", "## Assets", ""]
for a in ORDER:
    ops = json.load(open(f"{HERE}/ops/{a}.json")) if os.path.exists(f"{HERE}/ops/{a}.json") else []
    proof = [l.strip().split(" | ", 1)[1] for l in open(f"{GEN}/proof-{a}.txt") if l.startswith(f"{a} | ")] if os.path.exists(f"{GEN}/proof-{a}.txt") else []
    text += [f"### `{a}`", "", f"- Failing checks before ({len(before[a]['fails'])}): {'; '.join(before[a]['fails']) or 'none'}.", f"- Failing checks after ({len(after[a]['fails'])}): {'; '.join(after[a]['fails']) or 'none'}.", "- After, every authored scale (IoU / largest boundary distance): " + "; ".join(after[a]["lines"]) + ".", "- Proof (`prove.sh`, each run to the end): " + "; ".join(f"`{p}`" for p in proof) + ".", f"- Changed ({len(ops)} ops, `🧰️silhouette/R3-V/ops/{a}.json`, written by `make/{a}.*`, applied by `apply.ts {a}` after the ops of batch SAV): {notes['assets'][a]}", ""]
text += ["## Files", "", notes["files"], "", "## What Stays Red", "", notes["red"], ""]
open(f"{U}/silhouette-R3-V.md", "w").write("\n".join(text))
print(f"silhouette-R3-V.md: {total_before} failing checks before, {total_after} after")
