"""🏷️ Says whether catalog documents still are the documents their proofs measured, apart from `asset.tags` (rewritten for the whole catalog by another lane on 2026-11-25 19:09, strings to `{ id, title }`): `python3 same-but-tags.py <catalog humans dir> <proven dir> <id>…` prints per asset `same but tags`, `identical` or the first differing paths."""
import json, sys
catalog, proven, ids = sys.argv[1], sys.argv[2], sys.argv[3:]
for a in ids:
    now = json.load(open(f"{catalog}/{a}/🔣️.json")); old = json.load(open(f"{proven}/{a}.json"))
    if now == old:
        print(a, "identical"); continue
    now["asset"].pop("tags", None); old["asset"].pop("tags", None)
    if now == old:
        print(a, "same but tags"); continue
    keys = [k for k in sorted(set(now["asset"]) | set(old["asset"])) if now["asset"].get(k) != old["asset"].get(k)]
    print(a, "DIFFERENT in", keys)
