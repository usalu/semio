"""Binds each capability-less no-oracle decision to the capabilities of the features that cite it; drops uncited ones."""
import json, re, subprocess, sys
root = "/Users/ueli/Documents/semio/"
rows = json.load(open(sys.argv[1]))
paths = sorted({x["scope"] for x in rows if "capabilities must contain at least 1" in x["summary"]})
for rel in paths:
    doc = json.load(open(root + rel, encoding="utf-8"))
    kept = []
    for decision in doc.get("noOracleDecisions", []):
        if decision.get("capabilities"):
            kept.append(decision); continue
        cites = subprocess.run(["git", "grep", "-l", "--untracked", "-E", f"@no-oracle-{re.escape(decision['id'])}(\\s|$)", "--", "*.feature"], cwd=root, capture_output=True, text=True).stdout.split()
        caps = sorted({m for f in cites for m in re.findall(r"^@capability-(\S+)", open(root + f, encoding="utf-8").read(), re.M)})
        if caps:
            decision["capabilities"] = caps; kept.append(decision); print("bound", decision["id"], caps)
        else:
            print("dropped uncited", decision["id"], rel)
    doc["noOracleDecisions"] = kept
    open(root + rel, "w", encoding="utf-8").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
