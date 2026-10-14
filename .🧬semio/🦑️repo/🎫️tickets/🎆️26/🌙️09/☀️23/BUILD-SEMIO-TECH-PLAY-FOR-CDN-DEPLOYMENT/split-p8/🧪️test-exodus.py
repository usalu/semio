#!/usr/bin/env python3
"""🧪️ Plans (and with `apply` runs) the move of a monolith's model-level test modules into integration targets of the
monolith package, before its crate cut: every `cfg(test)` module file whose path matches one of the regexes is
grouped by category (sqlite, mutations, io, model) and chunked into targets of at most 31 files (test sources plus the
fixtures they embed); each target root lives at `<artifact>/🧪️tests/🔬️<category>-laws[-n]/🦀️.rs`.

usage: 🧪️test-exodus.py <artifact dir (repo-rel)> <label> plan|apply <regex>...
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
artifact, label, mode = sys.argv[1:4]
patterns = [re.compile(p) for p in sys.argv[4:]]
ns = {}
exec(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], ns)
root = os.path.join(REPO, artifact, "🦀️.rs")
crate = ns["Crate"](root)
chosen = sorted(f for f in crate.test_files if isinstance(f, str) and f not in crate.included and any(p.search(os.path.relpath(f, REPO)) for p in patterns))
def category(f):
    rel = os.path.relpath(f, os.path.join(REPO, artifact))
    return "sqlite" if "🪶️sqlite" in rel else "mutation" if "🧬️mutations" in rel else "io" if "🚪️io" in rel else "model"
def weight(f):
    text = open(f, encoding="utf-8").read()
    return 1 + len(set(re.findall(r'include_(?:str|bytes)\s*!\s*\(\s*"([^"]+)"', text)))
groups = {}
for f in chosen:
    groups.setdefault(category(f), []).append(f)
targets = []
names = {"sqlite": "SQLite snapshot", "mutation": "mutation", "io": "codec and io", "model": "model"}
for cat, files in groups.items():
    chunks, cur, n = [], [], 0
    for f in files:
        w = weight(f)
        if cur and n + w > 30:
            chunks.append(cur); cur, n = [], 0
        cur.append(f); n += w
    if cur: chunks.append(cur)
    for i, chunk in enumerate(chunks):
        suffix = "" if len(chunks) == 1 else f"-{i + 1}"
        pre = os.environ.get("PREFIX", "")
        targets.append({"name": f"{pre + '_' if pre else ''}{cat}_laws{suffix.replace('-', '_')}", "root": f"{artifact}/🧪️tests/🔬️{pre + '-' if pre else ''}{cat}-laws{suffix}/🦀️.rs",
                        "doc": f"//! 🧪️ The {label} {names[cat]} laws{f' (part {i + 1})' if suffix else ''}, as an integration target over the artifact's public API.",
                        "tests": [re.escape(os.path.relpath(f, REPO)) + "$" for f in chunk]})
        print(targets[-1]["name"], len(chunk), "files", sum(weight(f) for f in chunk))
spec = {"manifest": f"{artifact}/📦️packages/🦀️rust/Cargo.toml", "targets": targets}
path = os.path.join(HERE, "🗑️generated", re.sub(r"\W", "_", label) + "-exodus.json")
json.dump(spec, open(path, "w"), ensure_ascii=False, indent=1)
if mode == "apply":
    print(subprocess.run([sys.executable, os.path.join(HERE, "🧪️integration-tests.py"), path, "apply"], capture_output=True, text=True).stdout[-400:])
