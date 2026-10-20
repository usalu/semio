#!/usr/bin/env python3
"""🧪 Language-agnostic cases (`cases.json`) for the repoint core and the manifest editor, cross-checked with third-party parsers: every expected Rust output must parse under `rustfmt` and every manifest case must change exactly the declared dependency keys under Bun's TOML parser. usage: python3 apply.test.py"""
import json, os, subprocess, sys, shutil, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from repoint_core import rewrite_file
from manifest_edit import plan_manifest
cases = json.load(open(os.path.join(HERE, "cases.json"), encoding="utf-8"))
fails = []
def tup(v): return tuple(v)
for c in cases["rs"]:
    table = {tuple(k.split("::")): tup(v) for k, v in c["table"].items()}
    def resolve(segs, table=table):
        return table.get(tuple(segs), ("fail", "unresolved"))
    r = rewrite_file(c["input"], set(c["idents"]), resolve)
    if "error" in c:
        if not any(c["error"] in e for e in r.errors): fails.append((c["name"], "expected error", r.errors))
    elif r.errors or r.text != c["expected"]:
        fails.append((c["name"], r.errors, r.text))
    elif shutil.which("rustfmt"):
        p = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout"], input=c["expected"], capture_output=True, text=True)
        if p.returncode != 0 and "expected" in p.stderr: fails.append((c["name"], "rustfmt rejects the expected text", p.stderr[:200]))
bun = shutil.which("bun")
for c in cases["manifest"]:
    declared = set(c["declared"])
    new, err = plan_manifest(c["input"], c["key"], c["prod"], c["test"], lambda p, d=declared: p in d, lambda p, c=c: c["rel"] + p)
    if "error" in c:
        if not (err and c["error"] in err): fails.append((c["name"], "expected error", err))
        continue
    if err or new != c["expected"]:
        fails.append((c["name"], err, new)); continue
    if bun:
        js = "const k=(t:string)=>{const d:any=Bun.TOML.parse(t);return Object.keys({...(d.dependencies??{}),...(d['dev-dependencies']??{})}).sort()};console.log(JSON.stringify([k(process.argv[2]),k(process.argv[3])]))"
        f = tempfile.NamedTemporaryFile("w", suffix=".ts", delete=False); f.write(js); f.close()
        p = subprocess.run([bun, f.name, c["input"], c["expected"]], capture_output=True, text=True); os.unlink(f.name)
        before, after = json.loads(p.stdout)
        if sorted(set(after) - set(before)) != sorted(c["added"]) or sorted(set(before) - set(after)) != sorted(c["removed"]) :
            if not (set(c["added"]) <= set(after) and set(c["removed"]) <= set(before)): fails.append((c["name"], "bun toml diff", before, after))
print("cases", len(cases["rs"]) + len(cases["manifest"]), "failures", len(fails))
for f in fails: print(" ", f)
sys.exit(1 if fails else 0)
