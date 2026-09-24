"""Projects each leaf descriptor's `binaryTag` from its vocabulary's 📡️.protocol.semio record (the scaffolder's own rule)."""
import json, os, re, subprocess, sys
root = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
protos = subprocess.run(["find", "✏️s", "🧰️framework", "-path", "*🧬️mutations/💾️binary/📡️.protocol.semio", "-not", "-path", "*/target*", "-not", "-path", "*/dist/*"], cwd=root, capture_output=True, text=True).stdout.split("\n")
changed = 0
for p in filter(None, protos):
    records = {k: int(t) for k, t in re.findall(r"(?m)^\s*record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)", open(root + p, encoding="utf-8").read())}
    if not records: continue
    vocab = os.path.dirname(os.path.dirname(root + p))
    for dp, dn, fn in os.walk(vocab):
        if dp.count("/") - vocab.count("/") > 2 or "🧪️tests" in dp or "🧫️fixtures" in dp: continue
        if "🔣️.json" not in fn or dp == vocab: continue
        f = dp + "/🔣️.json"
        text = open(f, encoding="utf-8").read()
        try: j = json.loads(text)
        except Exception: continue
        if not (isinstance(j, dict) and j.get("schemaVersion") == 1 and "binaryTag" in j and "semanticKind" in j): continue
        want = records.get(j["semanticKind"])
        if want is None or j["binaryTag"] == want: continue
        changed += 1
        print(j["binaryTag"], "->", want, os.path.relpath(f, root)[:150])
        if apply:
            new = re.sub(r'("binaryTag":\s*)(null|\d+)', lambda m: m.group(1) + str(want), text, count=1)
            open(f, "w", encoding="utf-8").write(new)
print("changed:", changed)
