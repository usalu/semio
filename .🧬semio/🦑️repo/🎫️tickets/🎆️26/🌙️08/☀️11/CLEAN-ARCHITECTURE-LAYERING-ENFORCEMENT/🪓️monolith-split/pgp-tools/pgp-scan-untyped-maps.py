import json, os, re, subprocess, sys
root = "/Users/ueli/Documents/semio"
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "✏️s", "🌎️hub"], cwd=root, capture_output=True).stdout.decode().split("\0")
rows = []
def maps(node, path=""):
    if not isinstance(node, dict):
        return
    if node.get("type") == "object" and isinstance(node.get("additionalProperties"), dict) and not node.get("properties"):
        yield path
    for key, value in (node.get("properties") or {}).items():
        yield from maps(value, f"{path}.{key}")
    items = node.get("items")
    if isinstance(items, dict):
        yield from maps(items, path + "[]")
    for key in ("oneOf", "anyOf", "allOf"):
        for index, alt in enumerate(node.get(key) or []):
            yield from maps(alt, f"{path}|{index}")
for rel in files:
    if not rel.endswith("/🟦️.ts") or "/🧬️schema/" not in rel or not os.path.exists(os.path.join(root, rel)):
        continue
    js = os.path.join(root, os.path.dirname(rel), "🔣️.json")
    if not os.path.exists(js):
        continue
    try:
        doc = json.load(open(js, encoding="utf-8"))
    except Exception:
        continue
    src = open(os.path.join(root, rel), encoding="utf-8").read()
    defs = dict(doc.get("$defs") or {})
    if isinstance(doc.get("title"), str):
        defs.setdefault(doc["title"], doc)
    found = []
    for name, definition in defs.items():
        hits = list(maps(definition))
        if not hits:
            continue
        m = re.search(rf"^export function parse{re.escape(name)}\(.*?^\}}\n", src, re.M | re.S)
        if m is None:
            found.append((name, hits, "no-parser"))
            continue
        body = m.group(0)
        for hit in hits:
            field = hit.split(".")[-1].split("[")[0].split("|")[0] if hit else ""
            line = next((l for l in body.split("\n") if f'row["{field}"]' in l), "") if field else ""
            if field and "Object.fromEntries" not in line:
                found.append((name, hit, "untyped:" + line.strip()[:120]))
    if found:
        rows.append((rel, found))
for rel, found in rows:
    print(rel)
    for item in found:
        print("   ", item)
print("files", len(rows))
