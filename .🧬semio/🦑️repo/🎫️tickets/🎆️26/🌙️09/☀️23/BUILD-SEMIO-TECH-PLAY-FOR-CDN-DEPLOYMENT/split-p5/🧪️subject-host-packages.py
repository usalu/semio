#!/usr/bin/env python3
"""🧪️ Declares, per fem subset owner, the layer crates its language-neutral test adapters name as `subjectHostPackages`
in the owner's oracle registry (`<owner>/🔮️oracles/🔣️.json`): the generated subject host links the owner's nearest
package (the artifact aggregator) plus exactly these crates.

usage: 🧪️subject-host-packages.py plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); REPO = "/Users/ueli/Documents/semio"
crates = {}
for cfg in ("🧾️fem2d-crates.json", "🧾️fem3d-crates.json"):
    for e in json.load(open(os.path.join(HERE, cfg), encoding="utf-8"))["crates"]:
        crates[e["name"].replace("-", "_")] = (e["name"], e["dir"])
import glob
for man in glob.glob(os.path.join(REPO, "✏️s/🔨️modules/🏗️fem/⚙️engine/**/📦️packages/🦀️rust/Cargo.toml"), recursive=True):
    name = re.search(r'^name\s*=\s*"([^"]+)"', open(man, encoding="utf-8").read(), re.M).group(1)
    crates[name.replace("-", "_")] = (name, os.path.relpath(os.path.dirname(man), REPO))
ADAPTER = re.compile(r"(mutate-fem|solves-fem|round-trips)")
role = {"model": "📸️ round-trips the committed documents through the snapshot codecs the model crate owns",
        "mutations": "🧬️ drives the production JSON apply/undo bridge of the dispatch enum the mutations crate owns",
        "io": "🚪️ reads the analysis report bridges and examples the io crate owns",
        "engine": "⚙️ solves through the artifact engine bridge the engine crate owns",
        "fem-engine": "🧮️ compares against the shared fem engine results this crate owns",
        "editor_results_window": "📊️ drives the results-window playback transient the results window crate owns"}
for d in ("◻️2d", "🧊️3d"):
    subsets = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts", d, "🏅️standards/🔖️1/🪆️subsets")
    for s in sorted(os.listdir(subsets)):
        tests = os.path.join(subsets, s, "🧪️tests")
        if not os.path.isdir(tests): continue
        idents = set()
        for case in os.listdir(tests):
            if not ADAPTER.search(case): continue
            p = os.path.join(tests, case, "🦀️.rs")
            if os.path.exists(p): idents |= set(re.findall(r"\b(semio_s_artifact_fem_[23]d_[a-z_]+|semio_s_fem_[a-z0-9_]+)\s*::", open(p, encoding="utf-8").read()))
        if not idents: continue
        rows = []
        for i in sorted(idents):
            name, path = crates[i]
            layer = i.split("_", 5)[-1] if i.startswith("semio_s_artifact") else "fem-engine"
            rows.append({"implementation": "rust", "package": name, "path": path, "rationale": "The subject half " + role.get(layer, "names this layer crate").split(" ", 1)[1] + "."})
            rows[-1]["rationale"] = role.get(layer, "🧩️ names this layer crate").split(" ", 1)[0] + " The subject half " + role.get(layer, "🧩️ names this layer crate").split(" ", 1)[1] + "."
        reg = os.path.join(subsets, s, "🔮️oracles", "🔣️.json")
        print(os.path.relpath(reg, REPO), [r["package"] for r in rows])
        if sys.argv[1] == "apply":
            data = json.load(open(reg, encoding="utf-8"))
            data["subjectHostPackages"] = rows
            open(reg, "w", encoding="utf-8").write(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
