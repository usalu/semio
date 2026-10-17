"""📇️ Re-points the composition-laws suite registry at the six app-law targets: target, targetPath and sha256 of every app-laws suite. Reads the test-name lists of tc4-names (one per root). usage: registry.py"""
import hashlib, json, os, collections
R = "/Users/ueli/Documents/semio/"
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + "/../🗑️generated/"
C = R + "✏️s/🧑‍💻dev/🧩️composition/"
P = C + "🧪️tests/🧫️fixtures/🔣️.json"
ROOTS = {"🔭️viewer-laws": "generation3d-app-laws-viewer", "🪟️editor-mode-laws": "generation3d-app-laws-editor-modes", "🧭️editor-structure-laws": "generation3d-app-laws-editor-structure", "🎮️editor-command-laws": "generation3d-app-laws-editor-commands", "🧮️editor-unit-laws": "generation3d-app-laws-editor-unit", "🕹️editor-surface-laws": "generation3d-app-laws-editor-surface"}
law_root = {}
for d, target in ROOTS.items():
    for line in open(GEN + "tc4-names-new-" + d + ".txt", encoding="utf8"):
        line = line.strip()
        if line and not line.startswith("MACRO"):
            assert law_root.setdefault(line, d) == d, line
ORIG = {line.split("\t")[2]: line.split("\t")[0] for line in open(HERE + "/orig/index.tsv", encoding="utf8").read().splitlines()}
raw = open(HERE + "/orig/" + ORIG[P[len(R):]], encoding="utf8").read()
data = json.loads(raw)
assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == raw
UNIT = "🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/"
GROUPS = ["🕹️command-surface", "🪟️window-and-menu-laws", "🧮️engine-compute", "📏️surface-budget-and-retirement", "🔁️extension-round-trip", "📈️hot-path-and-fault-laws", "🪄️mesh-component-laws", "🚪️archive-and-brep-laws"]
split = []
for suite in data["suites"]:
    if suite["path"] == UNIT + "🦀️.rs" and suite.get("tree"):
        text = {g: open(C + UNIT + g + "/🦀️.rs", encoding="utf8").read() for g in GROUPS}
        buckets = {g: [] for g in GROUPS}
        for law in suite["laws"]:
            fn = "fn " + law.split("::")[-1] + "("
            owners = [g for g in GROUPS if fn in text[g]]
            assert len(owners) == 1, (law, owners)
            buckets[owners[0]].append(law)
        for g in GROUPS:
            if buckets[g]:
                split.append({k: v for k, v in suite.items() if k != "tree"} | {"path": UNIT + g + "/🦀️.rs", "laws": buckets[g]})
    else:
        split.append(suite)
data["suites"] = split
changed = 0
for suite in data["suites"]:
    if suite["target"] != "generation3d-app-laws":
        continue
    roots = {law_root[l] for l in suite["laws"]}
    assert len(roots) == 1, (suite["path"], roots)
    d = roots.pop()
    suite["target"] = ROOTS[d]
    suite["targetPath"] = "🧪️tests/🧊️generation3d/" + d + "/🦀️.rs"
    suite["sha256"] = hashlib.sha256(open(C + suite["path"], "rb").read()).hexdigest()
    changed += 1
open(P, "w", encoding="utf8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
print("suites updated", changed, collections.Counter(s["target"] for s in data["suites"]).most_common())
