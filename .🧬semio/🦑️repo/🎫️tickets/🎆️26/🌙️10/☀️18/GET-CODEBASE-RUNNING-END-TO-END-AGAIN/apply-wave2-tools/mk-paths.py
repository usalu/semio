# Builds a scratch tsconfig `paths` map for workspace packages that exist in the tree but are not linked in node_modules yet (install pending). usage: mk-paths.py <out tsconfig>
import json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
files = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", "*package.json"], cwd=ROOT, capture_output=True, text=True).stdout.split("\0")
installed = set(os.listdir(os.path.join(ROOT, "node_modules", "@semio-tech")))
def pick(value):
    if isinstance(value, str): return value
    if isinstance(value, dict):
        for key in ("semio-source", "import", "default", "node", "require"):
            if key in value:
                found = pick(value[key])
                if found: return found
        for item in value.values():
            found = pick(item)
            if found: return found
    return None
paths = {}
for rel in files:
    if not rel or "node_modules" in rel: continue
    try: data = json.load(open(os.path.join(ROOT, rel), encoding="utf-8"))
    except Exception: continue
    name = data.get("name", "")
    if not name.startswith("@semio-tech/") or name.split("/", 1)[1] in installed: continue
    base = os.path.dirname(rel)
    exports = data.get("exports")
    if isinstance(exports, str): exports = {".": exports}
    for key, value in (exports or {}).items():
        target = pick(value)
        if not target or not key.startswith("."): continue
        spec = name + key[1:]
        paths[spec] = [os.path.normpath(os.path.join(base, target))]
json.dump({"compilerOptions": {"baseUrl": ROOT, "paths": paths}}, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False)
print(len(paths), "mapped specifiers for", len({k.split("/")[1] for k in paths}), "uninstalled packages")
