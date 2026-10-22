import json, os, subprocess, sys, re
root = os.getcwd()
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
pk = {}
for f in subprocess.check_output(["git", "ls-files", "--", "*package.json"], text=True).split("\n"):
    if not f or "node_modules" in f or f.startswith(".🧬semio"): continue
    try: j = json.load(open(f, encoding="utf8"))
    except Exception: continue
    n = j.get("name")
    if n and n not in pk: pk[n] = (os.path.dirname(f), j)
def resolve(spec):
    parts = spec.split("/")
    name = "/".join(parts[:2]); sub = "." + spec[len(name):]
    d, j = pk[name]
    ex = j.get("exports", {})
    tgt = ex.get(sub)
    if not isinstance(tgt, str): raise SystemExit("unresolvable export %s %r" % (spec, tgt))
    return os.path.normpath(os.path.join(d, tgt))
apply = "--write" in sys.argv
for rnd in range(12):
    out = subprocess.check_output(["bun", C + "/slice-play3-tools/registry-closure.ts", "--json"], text=True)
    off = json.loads(out.strip().splitlines()[-1])
    if not off: print("round", rnd, "zero offenders"); break
    print("round", rnd, [o["specifier"] for o in off])
    if not apply: break
    for o in off:
        if o["specifier"] == "<missing>": raise SystemExit("missing files: %r" % o["paths"])
        tgt = resolve(o["specifier"])
        for path in set(o["paths"]):
            rel = os.path.relpath(tgt, os.path.dirname(path))
            if not rel.startswith("."): rel = "./" + rel
            s = open(path, encoding="utf8").read()
            q = ('"%s"' % o["specifier"], "'%s'" % o["specifier"])
            n = sum(s.count(x) for x in q)
            if n == 0: raise SystemExit("specifier not found in %s" % path)
            for x in q: s = s.replace(x, '"%s"' % rel)
            open(path, "w", encoding="utf8").write(s)
            print("  rewrote", n, o["specifier"], "->", rel[-60:], "in", path[-70:])
