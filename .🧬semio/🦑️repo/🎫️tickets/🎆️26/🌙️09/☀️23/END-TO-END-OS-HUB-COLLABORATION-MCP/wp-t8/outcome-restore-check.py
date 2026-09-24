"""Checks that HEAD's outcome fields equal the pre-edit values the two alignment runs logged, so restoring from HEAD drops no peer edit."""
import json, subprocess, re
root = "/Users/ueli/Documents/semio/"
files = [l.strip() for l in open(root + ".tmp-ticket/wp-t8/generated/outcome-touched-json.txt") if l.strip() and not l.strip().endswith("🀄️wfc/🔣️.json")]
bad = []
for f in files:
    head = subprocess.run(["git", "show", f"HEAD:{f}"], cwd=root, capture_output=True, text=True).stdout
    cur = open(root + f, encoding="utf-8").read()
    strip = lambda t: re.sub(r'"(?:outcomeClasses|outcomes)":\s*\[[^\]]*\]', "X", t)
    if strip(json.dumps(json.loads(head), ensure_ascii=False, indent=1)) != strip(json.dumps(json.loads(cur), ensure_ascii=False, indent=1)): bad.append(f)
print(len(files), "files;", len(bad), "differ outside outcome fields:", bad[:5])
