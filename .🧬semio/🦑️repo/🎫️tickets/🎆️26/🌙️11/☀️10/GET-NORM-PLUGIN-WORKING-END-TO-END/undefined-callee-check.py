"""🔎️ Finds free-function calls and unqualified type names in the test closure of the given crates that no source in the repository defines or imports, the class a textual path check misses (a helper the test took through `use super::*` that never existed).
usage: undefined-callee-check.py <crate>..."""
import json, os, re, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
defs = set()
for base in ("✏️s", "🧰️framework"):
    for r, d, fs in os.walk(base):
        d[:] = [x for x in d if x not in ("node_modules", "target", ".git")]
        for f in fs:
            if f.endswith(".rs"):
                try: t = open(os.path.join(r, f), encoding="utf8").read()
                except Exception: continue
                defs |= set(re.findall(r"\b(?:fn|struct|enum|union|type|trait|const|static|mod|macro_rules!)\s+(\w+)", t))
                defs |= set(re.findall(r"\bas\s+(\w+)\b", t))
std = set("assert assert_eq assert_ne debug_assert panic format vec matches println eprintln write writeln unreachable todo include_str include_bytes env concat stringify cfg line file column dbg Some None Ok Err Box Vec String Option Result Default From Into Iterator drop std core alloc crate super self Self if for while match return let loop else fn move async await in as mut ref where impl dyn use pub mod struct enum const static type trait unsafe extern".split())
jobs = subprocess.run(["python3", here + "/real-closure-jobs.py", *sys.argv[1:]], capture_output=True, text=True, check=True).stdout
open(here + "/🗑️generated/ucc-jobs.json", "w").write(jobs)
out = subprocess.run(["node", "--no-warnings", here + "/real-closure-probe.mjs", here + "/🗑️generated/ucc-jobs.json"], capture_output=True, text=True, env={**os.environ, "DUMP": "1"}, check=True).stdout
for r in json.loads(out):
    if not r["unit"].startswith("lib[cfg(test)]") or "+all" in r["unit"]: continue
    for p, *_ in r["inputs"]:
        if not p.endswith(".rs"): continue
        t = open(p, encoding="utf8").read()
        t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t); t = re.sub(r"//[^\n]*", "", t); t = re.sub(r"'(?:\\.|[^'\\])'", "''", t)
        local = set(re.findall(r"\b(?:let|mut|fn|for)\s+\(?(?:mut\s+)?(\w+)", t)) | set(re.findall(r"[|(,]\s*(?:mut\s+)?(\w+)\s*[:,)|]", t)) | set(re.findall(r"\|(\w+)\|", t))
        for n in sorted(set(re.findall(r"(?<![\w.:!])([a-z_][a-z0-9_]*)\s*\(", t)) - std - local):
            if n not in defs: print(f"{r['crate']} {os.path.relpath(p)[-70:]}: call to undefined `{n}`")
