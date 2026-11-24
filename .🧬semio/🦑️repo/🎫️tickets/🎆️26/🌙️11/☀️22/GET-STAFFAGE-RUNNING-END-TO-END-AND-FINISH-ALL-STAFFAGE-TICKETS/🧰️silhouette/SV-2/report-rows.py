"""📝️ The per-plant rows of the report of SV-2 from the logs of this session (🗑️generated/silhouette/SV-2: `before` and `after` of measure.ts, `proof/lines.txt` of prove.sh) and the repair ops: failing checks before and after, what the ops change, the proof line. Prints markdown.
python3 report-rows.py"""
import collections, json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "..", "..", "🗑️generated", "silhouette", "SV-2")
slugs = [l.split("\t")[1] for l in open(os.path.join(HERE, "..", "SV-2.tsv")) if l.strip()]
proof = {}
for l in open(os.path.join(GEN, "proof", "lines.txt")): proof[l.split(" | ")[0]] = l.strip()
def checks(folder, slug):
    path = os.path.join(GEN, folder, slug + ".log")
    if not os.path.exists(path): return None, []
    out = []
    for l in open(path):
        m = re.match(r"\S+ 1:(\d+) bbox (.*?)  \(feature", l)
        if m:
            for name, part in zip(("width", "height", "depth"), m.group(2).split("  ")):
                if "FAIL" in part: out.append(f"1:{m.group(1)} {name} {part.replace(' FAIL', '').replace('/', ' m against ')} m")
            continue
        m = re.match(r"\S+ 1:(\d+) (\w+)\s+iou ([\d.]+)( FAIL)?  H ([\d.]+)/([\d.]+)( FAIL)?", l)
        if m:
            if m.group(4): out.append(f"1:{m.group(1)} {m.group(2)} IoU {m.group(3)}")
            if m.group(7): out.append(f"1:{m.group(1)} {m.group(2)} distance {m.group(5)} m (limit {m.group(6)})")
    return len(out), out
for slug in slugs:
    nb, before = checks("before", slug)
    na, after = checks("after", slug)
    ops = json.load(open(os.path.join(HERE, "repair", slug + ".json")))
    kinds = collections.Counter()
    dims = {}
    for o in ops:
        if o["op"] == "dims": dims.update(o["set"]); continue
        if o["op"].startswith("solid"): kinds[f"1:{o['scale']} solids moved"] += 1; continue
        stroke = o.get("shape", {}).get("filled") is False
        what = {"shape": "strokes redrawn" if stroke else "filled shapes retraced", "shape-add": "strokes added" if stroke else "filled shapes added", "shape-remove": "shapes removed"}[o["op"]]
        kinds[f"1:{o['scale']} {what}"] += 1
    print(f"### `{slug}`\n")
    print(f"- Before: {nb} failing checks: {'; '.join(before)}.")
    print(f"- After: {na} failing checks{': ' + '; '.join(after) if after else ''}.")
    print(f"- Changed ({len(ops)} ops in `repair/{slug}.json`): {('declared ' + ', '.join(f'{k} {v}' for k, v in dims.items()) + '; ') if dims else ''}{', '.join(f'{v} × {k}' for k, v in sorted(kinds.items(), key=lambda x: (-int(x[0].split(' ')[0][2:]), x[0])))}.")
    print(f"- Proof: `{proof.get(slug, 'NOT RUN')}`\n")
