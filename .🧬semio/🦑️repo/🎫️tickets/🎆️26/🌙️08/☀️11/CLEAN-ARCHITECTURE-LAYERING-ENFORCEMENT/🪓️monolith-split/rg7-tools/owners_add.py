import json, os, re, sys, bisect
root = os.getcwd()
scan = json.load(open(sys.argv[1]))
path = os.path.join(root, "📋️build-owners.json")
text = open(path, encoding="utf8").read()
data = json.loads(text)
assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == text
names = {o["name"] for o in data["owners"]}
roots = {o["root"] for o in data["owners"]}
added = []
for m in scan["missing"]:
    if m in roots:
        continue
    proj = os.path.join(m, "📋️project.json")
    if os.path.exists(proj):
        name = json.load(open(proj, encoding="utf8"))["name"]; manifest = proj
    else:
        name = re.search(r'^name\s*=\s*"([^"]+)"', open(os.path.join(m, "Cargo.toml"), encoding="utf8").read(), re.M).group(1); manifest = os.path.join(m, "Cargo.toml")
    assert name not in names, name
    row = {"name": name, "root": m, "manifest": manifest, "packages": []}
    keys = [o["name"] for o in data["owners"]]
    i = bisect.bisect_left(keys, name)
    data["owners"].insert(i, row); names.add(name); added.append(name)
out = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
assert open(path, encoding="utf8").read() == text, "peer edited build-owners.json meanwhile"
open(path, "w", encoding="utf8").write(out)
print(len(added), "rows added:", *added, sep="\n  ")
