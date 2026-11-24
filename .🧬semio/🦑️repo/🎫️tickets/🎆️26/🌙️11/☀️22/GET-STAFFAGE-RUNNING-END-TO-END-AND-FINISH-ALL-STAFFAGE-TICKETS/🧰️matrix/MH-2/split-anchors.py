"""⚓️ Splits lumped z-0 contact anchors of an asset into one anchor per physical contact (D45): `python3 split-anchors.py <asset.json> <spec.json> [--write]`. The spec is a list of `{ "anchor": id, "into": [{ "id", "z", "title": {en, de}, ...fields }] }` and optional `{ "assumption": index, "text": {en, de} }` entries; every reference `{"anchor": id}` in clip and pose contacts, supports and volumes is repeated once per new anchor, in place. Idempotent: an anchor that no longer exists is skipped. The file keeps its indentation; it is only written when it did not change on disk since it was read."""
import json, sys
path, spec = sys.argv[1], json.load(open(sys.argv[2]))
text = open(path, encoding="utf8").read()
doc = json.loads(text)
a = doc["asset"]
changed = 0
def refs(node, old, new):
    global changed
    if isinstance(node, dict):
        for v in node.values(): refs(v, old, new)
    elif isinstance(node, list):
        i = 0
        while i < len(node):
            item = node[i]
            if isinstance(item, dict) and item.get("anchor") == old:
                node[i:i + 1] = [dict(item, anchor=n) for n in new]; changed += 1; i += len(new)
            else:
                refs(item, old, new); i += 1
for entry in spec:
    if "assumption" in entry:
        if a["structure"]["assumptions"][entry["assumption"]] != entry["text"]:
            a["structure"]["assumptions"][entry["assumption"]] = entry["text"]; changed += 1
        continue
    index = next((i for i, x in enumerate(a["anchors"]) if x["id"] == entry["anchor"]), None)
    if index is None: continue
    old = a["anchors"][index]
    a["anchors"][index:index + 1] = [dict(old, **part) for part in entry["into"]]
    changed += 1
    for key in ("clips", "poses", "structure", "volumes", "thermal", "symbols", "presentation"):
        if key in a: refs(a[key], entry["anchor"], [part["id"] for part in entry["into"]])
second = text.split("\n", 2)[1]
indent = len(second) - len(second.lstrip())
out = json.dumps(doc, indent=indent, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")
print("%s: %d changes" % (a["id"], changed))
if "--write" in sys.argv and changed:
    if open(path, encoding="utf8").read() != text: sys.exit("changed on disk since it was read: nothing written, run again")
    open(path, "w", encoding="utf8").write(out); print("written", path)
