import json
import os
import re
import sys

ROOT = "C:/git/semio"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
files = json.load(open(os.path.join(ROOT, TICKET, "🧰️cf3/files.json"), encoding="utf-8"))
targets = [p for k in ("created", "modified") for p in files[k] if p.endswith((".py", ".ts", ".tsx", ".md"))]
targets += [
    "🎓️teaching/🏛️architecture/🧍️staffage/README.md",
    "🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json",
]
seen = set()
pattern = re.compile(r"(@see\s+|\]\(|`)(\.{1,2}/[^\s)`\"']+|https?://[^\s)`\"']+)")
bad = []
total = 0
for rel in targets:
    if rel in seen:
        continue
    seen.add(rel)
    path = os.path.join(ROOT, rel)
    if not os.path.exists(path):
        bad.append(("MISSING-SOURCE", rel, 0, ""))
        continue
    text = open(path, encoding="utf-8", errors="replace").read()
    for n, line in enumerate(text.splitlines(), 1):
        for m in pattern.finditer(line):
            ref = m.group(2).split("#")[0].rstrip(".,;:")
            if ref.startswith("http"):
                continue
            total += 1
            full = os.path.normpath(os.path.join(os.path.dirname(path), ref))
            if not os.path.exists(full):
                bad.append(("UNRESOLVED", rel, n, ref))
print("checked", total, "refs in", len(seen), "files")
for b in bad:
    print(*b, sep="\t")
