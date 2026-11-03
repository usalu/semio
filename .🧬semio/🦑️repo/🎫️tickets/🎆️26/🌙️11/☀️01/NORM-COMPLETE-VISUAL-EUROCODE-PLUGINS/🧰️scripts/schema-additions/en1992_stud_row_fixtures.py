"""en1992 `studRings` -> `studRows` in every committed snapshot document (EN 1992-1-1 6.4.5): the c studs of the first ring become c rails (interior: evenly around, edge: 90 to 270 degrees, corner: 180 to 270 degrees; free edges at +x/+y) with one stud per former ring, s_0 = innermost radius, s_r = largest ring gap (0.15 m for a single ring), diameter of the innermost ring. Idempotent."""
import json, math, pathlib
ANY = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
def rows_of(rings, position):
    rings = sorted((r for r in rings), key=lambda r: r["radius"])
    n = max(rings[0]["count"], 0) if rings else 0
    span = {"Edge": (90.0, 270.0), "Corner": (180.0, 270.0)}.get(position)
    gaps = [b["radius"] - a["radius"] for a, b in zip(rings, rings[1:])]
    out = []
    for j in range(n):
        deg = span[0] + (span[1] - span[0]) * j / max(n - 1, 1) if span else 360.0 * j / n
        out.append({"id": f"r{j + 1}", "angle": math.radians(deg), "count": len(rings), "spacingFirst": rings[0]["radius"], "spacingRadial": max(gaps) if gaps else 0.15, "diameter": rings[0]["diameter"]})
    return out
def walk(node):
    changed = False
    if isinstance(node, dict):
        if isinstance(node.get("studRings"), list):
            position = (node.get("punching") or {}).get("columnPosition")
            rings = node.pop("studRings")
            node["studRows"] = rows_of(rings, position)
            changed = True
        if "studRings" in node and node["studRings"] is None:
            node["studRows"] = node.pop("studRings")
            changed = True
        for v in node.values():
            changed = walk(v) or changed
    elif isinstance(node, list):
        for v in node:
            changed = walk(v) or changed
    return changed
count = 0
import os
for path in (pathlib.Path(d) / f for d, _, fs in os.walk(ANY) for f in fs if f.endswith(".json")):
    if path.stat().st_size > 8_000_000:
        continue
    try:
        text = path.read_text(encoding="utf8")
    except Exception:
        continue
    if '"studRings"' not in text:
        continue
    data = json.loads(text)
    if walk(data):
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n" if "\n" in text.strip() else json.dumps(data, ensure_ascii=False), encoding="utf8")
        count += 1
print("files", count)
