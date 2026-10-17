"""verify.py: rustfmt parse of every touched .rs, path length, emoji docstring on new files, TOML parse of touched/new manifests, JSON parse of launch files + owners."""
import json, os, re, subprocess, sys
try:
    import tomllib
except ModuleNotFoundError:
    tomllib = None
ROOT = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
touched = [l.split("\t")[1] for l in open(HERE + "/orig/index.tsv").read().splitlines()]
new = []
for d in ["🧰️framework/🔨️modules/🔢️number/🧪️tests/🥣️companion", "🎓️teaching/🛂️proctor/🔨️modules/🧱️base", "🎓️teaching/🛂️proctor/🔨️modules/🧠️domain",
          "🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🧰️fixtures", "🎓️teaching/🛂️proctor/🔨️modules/📚️catalog/🧰️fixtures", "🎓️teaching/🛂️proctor/🔨️modules/🎭️actors/🧰️fixtures", "🎓️teaching/🛂️proctor/🔨️modules/🔭️projections/🧰️fixtures",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧪️tests/🥣️companion"]:
    for dp, dn, fn in os.walk(f"{ROOT}/{d}"):
        for f in fn: new.append(os.path.join(dp, f))
bad = 0
for p in sorted(set(touched + new)):
    r = p[len(ROOT) + 1:]
    if len(r.encode()) > 240: print("PATH>240", r); bad += 1
    if p.endswith(".rs"):
        out = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", p], capture_output=True, text=True)
        if out.returncode != 0: print("FMT FAIL", r, out.stderr[:200]); bad += 1
    if p in new and p.endswith(".rs"):
        first = open(p, encoding="utf-8").readline()
        if not (first.startswith("//!") or first.startswith("///") or first.startswith("#![")): print("NO DOCSTRING", r, first[:40]); bad += 1
    if p.endswith("Cargo.toml"):
        try: tomllib and tomllib.load(open(p, "rb"))
        except Exception as e: print("TOML FAIL", r, e); bad += 1
    if p.endswith("project.json"):
        json.load(open(p, encoding="utf-8"))
for p in ["📋️build-owners.json", ".vscode/launch.json"]:
    json.load(open(f"{ROOT}/{p}", encoding="utf-8"))
seed = open(f"{ROOT}/.vscode/🧩️launch.seed.jsonc", encoding="utf-8").read()
seed = re.sub(r"^\s*//.*$", "", seed, flags=re.M)
json.loads(seed)
print("checked", len(set(touched + new)), "files; problems", bad)
