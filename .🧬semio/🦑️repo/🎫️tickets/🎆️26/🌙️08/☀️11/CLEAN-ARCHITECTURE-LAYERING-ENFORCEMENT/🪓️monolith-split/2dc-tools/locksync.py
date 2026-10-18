"""🔒️ Hand-patches consumer Cargo.lock files to the carved 2d family: facade deps, compute package blocks and the transitive first-party closure, copied verbatim from the up to date reference lock; prunes the orphaned facade family. Dry-run unless --apply."""
import json, re, subprocess, sys, os
ROOT = "/Users/ueli/Documents/semio"; os.chdir(ROOT)
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
REF = "✏️s/Cargo.lock"
apply = "--apply" in sys.argv
def parse(text):
    parts = text.split("[[package]]\n")
    header, blocks = parts[0], [b.rstrip("\n") for b in parts[1:]]
    out = []
    for b in blocks:
        name = re.search(r'^name = "([^"]+)"', b, re.M).group(1)
        ver = re.search(r'^version = "([^"]+)"', b, re.M).group(1)
        src = re.search(r'^source = "([^"]+)"', b, re.M)
        deps = re.findall(r'^ "([^"]+)",?$', b, re.M)
        out.append({"name": name, "ver": ver, "src": src.group(1) if src else None, "deps": deps, "text": b})
    return header, out
def render(header, blocks):
    return header + "".join("[[package]]\n" + b["text"] + "\n\n" for b in blocks).rstrip("\n") + "\n"
def key(b):
    v = b["ver"]; core = v.split("+")[0].split("-")[0]
    return (b["name"].encode(), tuple(int(x) if x.isdigit() else 0 for x in core.split(".")), 1 if "-" not in v.split("+")[0] else 0, v)
rh, ref = parse(open(REF, encoding="utf8").read())
byname = {}
for b in ref: byname.setdefault(b["name"], []).append(b)
def depname(d): return d.split(" ")[0]
def closure(roots):
    seen, todo = {}, list(roots)
    while todo:
        n = todo.pop()
        if n in seen: continue
        bs = byname.get(n)
        if not bs or len(bs) != 1: raise Exception(("ref ambiguity", n))
        seen[n] = bs[0]
        todo += [depname(d) for d in bs[0]["deps"]]
    return seen
clos = closure(["semio-framework-2d", "semio-framework-2d-compute"])
fam = {n for n in clos if n.startswith("semio-framework-2d")}
locks = [l for l in open("/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/locks.txt", encoding="utf8").read().split("\n") if l]
report = {"changed": [], "flags": [], "pruned": []}
for lp in locks:
    text = open(lp, encoding="utf8").read()
    h, bl = parse(text)
    names = {}
    for b in bl: names.setdefault(b["name"], []).append(b)
    has_fac = "semio-framework-2d" in names
    deps_all = lambda: {depname(d) for b in bl for d in b["deps"]}
    users_of_compute = any(depname(d) in ("semio-framework-2d-compute",) for b in bl for d in b["deps"])
    if not has_fac and not users_of_compute and "semio-framework-2d-compute" not in names: continue
    changed = False
    # (a) which family roots are needed: facade if anybody still depends on it, compute if anybody depends on it
    need_roots = []
    dep_names = deps_all()
    if "semio-framework-2d" in dep_names: need_roots.append("semio-framework-2d")
    if "semio-framework-2d-compute" in dep_names or "semio-framework-2d-compute" in names: need_roots.append("semio-framework-2d-compute")
    if has_fac and "semio-framework-2d" not in dep_names and "semio-framework-2d" not in [b["name"] for b in bl if False]: pass
    needed = closure(need_roots) if need_roots else {}
    for n, rb in needed.items():
        cur = names.get(n)
        if not cur:
            if n.startswith("semio-framework") or rb["src"] is None:
                pass
            bl.append(dict(rb)); names[n] = [rb]; changed = True
        elif n in fam:
            if cur[0]["text"] != rb["text"]:
                cur[0].update(dict(rb)); changed = True
        elif len(cur) > 1:
            report["flags"].append((lp, n, "multiple versions"))
        elif cur[0]["deps"] != rb["deps"] and rb["src"] is None:
            report["flags"].append((lp, n, "first-party deps differ"))
    # (b) prune orphaned 2d family blocks (no dependents) iteratively; members are never in fam
    while True:
        dn = deps_all(); gone = [b for b in bl if b["name"] in fam and b["name"] not in dn and b["name"] != "semio-framework-2d-compute" or (b["name"] in fam and b["name"] not in dn and b["name"] == "semio-framework-2d-compute")]
        if not gone: break
        for b in gone: bl.remove(b); report["pruned"].append((lp, b["name"])); changed = True
    if changed:
        bl.sort(key=key)
        new = render(h, bl)
        report["changed"].append(lp)
        if apply: open(lp, "w", encoding="utf8").write(new)
print("locks changed", len(report["changed"]), "flags", len(report["flags"]), "pruned", len(report["pruned"]))
for f in report["flags"][:20]: print("FLAG", f)
json.dump(report, open(F + "/2dc-tools/locksync-report.json", "w"), ensure_ascii=False, indent=0)
