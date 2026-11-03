"""🛠️ Apply exact de-value swaps from map.tsv: fresh read, exact token replace, JSON validity check, truncating write."""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a

maps = [l.rstrip("\n").split("\t") for l in open(Path(__file__).parent / "map.tsv", encoding="utf8") if l.strip()]
changed = 0
for f in sorted(a.CATALOG.rglob("🔣️.json")):
    text = f.read_text(encoding="utf8")
    new = text
    n = 0
    for old, repl in maps:
        key = f'"de": "{old}"'
        if key in new:
            n += new.count(key)
            new = new.replace(key, f'"de": "{repl}"')
    if n:
        json.loads(new)
        fresh = f.read_text(encoding="utf8")
        if fresh != text:
            print("SKIP (changed meanwhile)", f.parent.name)
            continue
        try:
            f.write_text(new, encoding="utf8", newline="")
        except OSError as e:
            print("WRITE FAILED", f.parent.name, e)
            continue
        changed += n
        print(f.parent.name, n)
print("replacements", changed)
