"""🌳️ Counts the distinct compile units of semio-s-artifact-stdio-semio (and of its non-optional closure) across the hub
prebuild plan under per-package feature unification, from `cargo tree` alone (no compilation).
usage: python3 🌳️closure-census.py <plan.json> <out.json>"""
import json, os, re, subprocess, sys
plan, out = sys.argv[1], sys.argv[2]
ws = json.load(open(plan))["workspaces"][0]
env = dict(os.environ, CARGO_UNSTABLE_FEATURE_UNIFICATION="true", CARGO_RESOLVER_FEATURE_UNIFICATION="package")
TARGET = "semio-s-artifact-stdio-semio"
result = {}
for pkg in ws["packages"]:
    text = subprocess.run(["cargo", "tree", "--locked", "--offline", "--manifest-path", ws["manifest"], "-p", pkg, "--target", "wasm32-wasip2", "-e", "normal", "--prefix", "depth", "-f", "{p}|{f}"], env=env, capture_output=True, text=True).stdout
    stack, nodes, edges = [], {}, {}
    for line in text.splitlines():
        m = re.match(r"^(\d+)(.*)$", line)
        if not m: continue
        depth, body = int(m.group(1)), m.group(2)
        dedup = body.endswith(" (*)")
        body = body[:-4] if dedup else body
        name_ver, _, feats = body.partition("|")
        key = " ".join(name_ver.split()[:2])
        nodes.setdefault(key, feats)
        stack[depth:] = [key]
        if depth: edges.setdefault(stack[depth - 1], set()).add(key)
    roots = [k for k in nodes if k.split()[0] == TARGET]
    core = [c for r in roots for c in edges.get(r, ()) if not (c.split()[0].startswith("semio-s-artifact-stdio-") and c.split()[0] != "semio-s-artifact-stdio-contract")]
    if not roots: continue
    seen, todo = set(), list(roots)
    while todo:
        k = todo.pop()
        if k in seen: continue
        seen.add(k); todo.extend(edges.get(k, ()))
    cseen, todo = set(), list(core)
    while todo:
        k = todo.pop()
        if k in cseen: continue
        cseen.add(k); todo.extend(edges.get(k, ()))
    result[pkg] = {"full": {k: nodes[k] for k in sorted(seen)}, "core": {k: nodes[k] for k in sorted(cseen)}}
json.dump(result, open(out, "w"), indent=1)
print(f"{len(result)} of {len(ws['packages'])} packages reach {TARGET}")
