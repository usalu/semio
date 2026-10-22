import hashlib, json, os, re, sys

ledger = json.load(open(sys.argv[1], encoding="utf-8"))
files = sorted({f for row in ledger for f, _ in row["edits"]})
bad = 0
checked = 0
for f in files:
    base = os.path.dirname(f)
    text = open(f, encoding="utf-8").read()
    if f.endswith(".rs"):
        for m in re.finditer(r'include_str!\("([^"]+)"\)', text):
            checked += 1
            if not os.path.exists(os.path.normpath(os.path.join(base, m.group(1)))):
                bad += 1
                print("MISSING rs", f[-60:], m.group(1)[-90:])
    elif f.endswith("oracles/🔣️.json"):
        def walk(node):
            global bad, checked
            if isinstance(node, dict):
                if "path" in node and "sha256" in node:
                    target = os.path.normpath(os.path.join(os.path.dirname(os.path.dirname(f)), "🧪️tests", node["path"].replace("../", "", 1))) if False else None
                    candidates = [os.path.normpath(os.path.join(os.path.dirname(f), "..", "🧪️tests", node["path"][3:])), os.path.normpath(os.path.join(os.path.dirname(f), node["path"]))]
                    hit = next((c for c in candidates if os.path.exists(c)), None)
                    checked += 1
                    if hit is None:
                        bad += 1
                        print("MISSING oracle", f[-70:], node["path"][-90:])
                    else:
                        digest = "sha256:" + hashlib.sha256(open(hit, "rb").read()).hexdigest()
                        if digest != node["sha256"]:
                            bad += 1
                            print("SHA oracle", node["path"][-90:])
                for v in node.values():
                    walk(v)
            elif isinstance(node, list):
                for v in node:
                    walk(v)
        walk(json.load(open(f, encoding="utf-8")))
print("checked", checked, "bad", bad)
