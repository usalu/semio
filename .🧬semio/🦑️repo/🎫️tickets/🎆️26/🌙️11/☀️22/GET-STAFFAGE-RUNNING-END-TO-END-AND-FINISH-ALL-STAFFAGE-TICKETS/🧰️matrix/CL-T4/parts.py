"""🧮️ The structure triangles of a full `groups.ts` listing by part of the solid (member, joint, mass, support, glyph) per scale: parts.py <asset file.json> <groups listing.txt> [list]"""
import json, re, sys, collections
asset = json.load(open(sys.argv[1]))["asset"]
want = sys.argv[3] if len(sys.argv) > 3 else "structure"
scene = None
for line in open(sys.argv[2]):
    m = re.match(r"^s(\d+)-(\w+) (\d+) triangles", line)
    if m:
        if scene and scene[1] == want: print(scene[0], dict(total), {k: v for k, v in sorted(kinds.items(), key=lambda t: -t[1])[:12]})
        scene = (m.group(1), m.group(2)); total = collections.Counter(); kinds = collections.Counter()
        parts = {re.sub(r"\d+", "#", s["id"]): s.get("part", s.get("region", "body")) for s in asset["volumes"][scene[0]].get(scene[1], [])}
        continue
    m = re.match(r"^  (\d+) = (\d+) x \d+  (.+?):(\S+)$", line)
    if m and scene:
        part = "glyph" if "/glyph" in m.group(4) else parts.get(m.group(3), "?")
        total[part] += int(m.group(1)); total[part + " solids"] += int(m.group(2)); kinds[part + ":" + m.group(4).split("/")[0]] += int(m.group(1))
if scene and scene[1] == want: print(scene[0], dict(total), {k: v for k, v in sorted(kinds.items(), key=lambda t: -t[1])[:12]})
