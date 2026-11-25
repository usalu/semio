"""📦️ Compares the `semio_*` crates named in the test closure of each crate with its Cargo.toml dependency names, and reports names used but not declared.
usage: dependency-audit.py <crate>..."""
import json, os, re, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
jobs = json.loads(subprocess.run(["python3", here + "/real-closure-jobs.py", *sys.argv[1:]], capture_output=True, text=True, check=True).stdout)
open(here + "/🗑️generated/da-jobs.json", "w").write(json.dumps(jobs))
rows = json.loads(subprocess.run(["node", "--no-warnings", here + "/real-closure-probe.mjs", here + "/🗑️generated/da-jobs.json"], capture_output=True, text=True, env={**os.environ, "DUMP": "1"}, check=True).stdout)
manifest = {j["crate"]: j["manifestDirectory"] for j in jobs}
for r in rows:
    if r["unit"] != "lib[cfg(test)]": continue
    cargo = open(manifest[r["crate"]] + "/Cargo.toml", encoding="utf8").read()
    declared = set(n.replace("-", "_") for n in re.findall(r"^([A-Za-z0-9_-]+)\s*=", re.sub(r"\[package\].*?(?=^\[)", "", cargo, flags=re.S | re.M), re.M))
    declared |= {r["crate"].replace("-", "_")}
    used = set()
    aliases = set()
    for p, *_ in r["inputs"]:
        if not p.endswith(".rs"): continue
        t = open(p, encoding="utf8").read()
        aliases |= set(re.findall(r"extern crate \w+ as (\w+);", t))
        t = re.sub(r"//[^\n]*", "", t)
        used |= set(re.findall(r"(?<![\w])((?:semio|serde|serde_json)_?[a-z0-9_]*)::", t))
        used |= set(re.findall(r"\bextern crate ([a-z0-9_]+)", t))
    miss = sorted(u for u in used if u not in declared and u not in aliases and u not in ("semio_plugin",) or (u == "semio_plugin" and "semio_plugin" not in declared))
    print(r["crate"], "MISSING:" if miss else "ok", miss if miss else "")
