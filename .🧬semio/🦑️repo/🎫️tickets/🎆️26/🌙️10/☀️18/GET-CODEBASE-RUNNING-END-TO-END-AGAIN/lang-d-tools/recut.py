import json, re, sys
A = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json"
findings = json.load(open(sys.argv[1], encoding="utf8"))
d = json.load(open(A, encoding="utf8"))
changed = []
for f in findings:
    if f["rule"] != "allowance-stale":
        continue
    m = re.fullmatch(r"(defaults )?allowed=(\d+) actual=(\d+)", f["detail"])
    table = d["defaults" if m.group(1) else "files"]
    actual = int(m.group(3))
    assert table[f["path"]] == int(m.group(2)), f
    if actual == 0:
        del table[f["path"]]
    else:
        table[f["path"]] = actual
    changed.append((f["path"][-60:], m.group(2), actual))
total = sum(d["files"].values())
if total < d["ceiling"]:
    d["ceiling"] = total
dtotal = sum(d["defaults"].values())
if dtotal < d["defaultsCeiling"]:
    d["defaultsCeiling"] = dtotal
open(A, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=1) + "\n")
for c in changed: print(c)
print("ceiling", d["ceiling"], "defaultsCeiling", d["defaultsCeiling"])
