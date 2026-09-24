import json, os, sys, glob
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
DOC = json.load(open(os.path.join(ROOT, "📑️document/🧬️schema/📸️snapshot/🔣️.json"), encoding="utf-8"))
CROSS = {k: f"{DOC['$id']}#/$defs/{k}" for k in ("DocBlock", "DocRun", "RunStyle")}
def refs_in(node, out):
    if isinstance(node, dict):
        r = node.get("$ref")
        if isinstance(r, str) and r.startswith("#/"): out.add(r.split("/")[-1])
        for v in node.values(): refs_in(v, out)
    elif isinstance(node, list):
        for v in node: refs_in(v, out)
    return out
def reachable(d, key):
    defs = d.get(key, {}); seen = set(); todo = list(refs_in({k: v for k, v in d.items() if k != key}, set()))
    while todo:
        n = todo.pop()
        if n in seen or n not in defs: continue
        seen.add(n); todo.extend(refs_in(defs[n], set()))
    return seen
def process(path, subset_snap):
    t = open(path, encoding="utf-8").read(); d = json.loads(t)
    sid, stitle = subset_snap["$id"], subset_snap.get("title")
    sdefkey = "$defs" if "$defs" in subset_snap else "definitions"
    sdefs = set(subset_snap.get(sdefkey, {}).keys())
    changed = []
    for key in ("$defs", "definitions"):
        defs = d.get(key, {})
        for k in list(defs):
            if "$ref" in defs[k] and len(defs[k]) == 1 and not defs[k]["$ref"].startswith("#"): continue
            if k in CROSS and not path.startswith(os.path.join(ROOT, "📑️document")): defs[k] = {"$ref": CROSS[k]}; changed.append(k)
            elif k == stitle and path != subset_snap_path[0]: defs[k] = {"$ref": sid}; changed.append(k)
            elif k in sdefs and path != subset_snap_path[0]: defs[k] = {"$ref": f"{sid}#/{sdefkey}/{k}"}; changed.append(k)
        live = reachable(d, key)
        for k in list(defs):
            if k not in live: del defs[k]; changed.append(f"-{k}")
        if key in d and not d[key]: del d[key]
    if changed:
        print(path.split("🪆️subsets/")[1], changed)
        if apply: open(path, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
subset_snap_path = [None]
for subset in sorted(os.listdir(ROOT)):
    sp = os.path.join(ROOT, subset, "🧬️schema/📸️snapshot/🔣️.json")
    if subset == "✉️base" or not os.path.exists(sp): continue
    subset_snap_path[0] = sp
    snap = json.load(open(sp, encoding="utf-8"))
    for leaf in glob.glob(os.path.join(ROOT, subset, "🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json")): process(leaf, snap)
