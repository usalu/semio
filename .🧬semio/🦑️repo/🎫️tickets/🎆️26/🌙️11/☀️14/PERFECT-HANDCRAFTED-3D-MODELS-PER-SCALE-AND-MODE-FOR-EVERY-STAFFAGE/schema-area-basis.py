"""Inserts "areaBasis": "exchange" after areaM2 in every thermal region of model leaf or kind foliage/fur/feather/hair of the catalog, plus `core` of appliances/decentralised-hrv-fan (ruling C19).
Each file is re-read right before writing; only the inserted text changes (checked by JSON comparison); the write is a temp file renamed over the original, and is skipped and retried when the file changed in between."""
import glob, json, os, re, sys, time

ROOT = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
KINDS = {"foliage", "fur", "feather", "hair"}
EXTRA = {("decentralised-hrv-fan", "core"), ("shower-head", "spray")}

def wanted(asset):
    return {r["id"] for r in asset["thermal"]["regions"] if r.get("areaBasis") != "exchange" and (r.get("model") == "leaf" or r.get("kind") in KINDS or (asset["id"], r["id"]) in EXTRA)}

def insert(text, ids, asset):
    start = text.index('"regions"', text.index('"thermal"'))
    out, pos = [], 0
    for rid in sorted(ids, key=lambda i: text.index(f'"id": "{i}"', start)):
        at = text.index(f'"id": "{rid}"', start)
        m = re.compile(r'("areaM2":\s*[-0-9.eE+]+)(,)(\s*)').search(text, at)
        if m is None:
            raise ValueError(f"no areaM2 after {rid}")
        out.append(text[pos:m.end(1)])
        out.append(f',{m.group(3)}"areaBasis": "exchange"')
        pos = m.end(1)
    out.append(text[pos:])
    return "".join(out)

changed, skipped = 0, 0
for path in sorted(glob.glob(f"{ROOT}/*/*/🔣️.json")):
    for attempt in range(5):
        before_mtime = os.stat(path).st_mtime_ns
        text = open(path, encoding="utf8").read()
        doc = json.loads(text)
        asset = doc["asset"] if "asset" in doc else doc
        ids = wanted(asset)
        if not ids:
            break
        new = insert(text, ids, asset)
        after = json.loads(new)
        a2 = after["asset"] if "asset" in after else after
        for r in a2["thermal"]["regions"]:
            if r["id"] in ids:
                if r.get("areaBasis") != "exchange":
                    raise SystemExit(f"{path}: {r['id']} not marked")
                del r["areaBasis"]
        if after != doc:
            raise SystemExit(f"{path}: JSON differs beyond areaBasis")
        tmp = path + ".areabasis.tmp"
        open(tmp, "w", encoding="utf8").write(new)
        if os.stat(path).st_mtime_ns != before_mtime or open(path, encoding="utf8").read() != text:
            os.remove(tmp)
            time.sleep(0.2)
            continue
        os.replace(tmp, path)
        changed += 1
        print("marked", asset["id"], sorted(ids))
        break
    else:
        skipped += 1
        print("skipped (busy)", path)
print(f"changed {changed}, skipped {skipped}")
