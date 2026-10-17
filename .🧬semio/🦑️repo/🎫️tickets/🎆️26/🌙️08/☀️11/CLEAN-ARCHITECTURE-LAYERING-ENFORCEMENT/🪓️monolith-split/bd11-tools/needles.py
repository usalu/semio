#!/usr/bin/env python3
"""usage: needles.py <test.ts> : for each failing assertion print the expression, its literal needles, and whether each literal exists in the plugin family text (file:line) or only as near matches."""
import re, sys, os, subprocess, glob
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
R = "/Users/ueli/Documents/semio"
PLUGIN = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
SKIP = {"🧷️companions", "🧪️tests", "node_modules", "target", "🗑️generated"}
def family():
    files = []
    def walk(d):
        for e in sorted(os.scandir(d), key=lambda e: e.name):
            if e.is_dir():
                if e.name not in SKIP: walk(e.path)
            elif e.name.endswith(".rs"): files.append(e.path)
    walk(PLUGIN)
    walk2 = []
    def w2(d):
        for e in sorted(os.scandir(d), key=lambda e: e.name):
            if e.is_dir():
                if e.name not in SKIP: w2(e.path)
            elif e.name.endswith(".rs"): walk2.append(e.path)
    w2(os.path.dirname(PLUGIN) + "/🏪️store")
    return [(f, open(f, errors="replace").read()) for f in files + walk2]
def run(T):
    D = os.path.dirname(T); C = os.path.join(D, "bd8-diag.ts")
    s = open(T).read()
    m = re.search(r"""import\s*\{([^}]*)\}\s*from\s*["']bun:test["'];?""", s)
    names = [n.strip() for n in m.group(1).split(",") if n.strip() and n.strip() != "expect"]
    open(C, "w").write(s[:m.start()] + 'import { ' + ', '.join(names) + ' } from "bun:test"; import { expect } from "' + F + '/bd8-tools/softexpect.ts";' + s[m.end():])
    log = F + "/🗑️generated/bd11-soft.log"; open(log, "w").close()
    try:
        r = subprocess.run(["bash", "-c", "ulimit -n 65536; cd " + R + " && nice -n 10 perl -e 'alarm 240; exec @ARGV' -- bun test \"./" + os.path.relpath(C, R) + "\" 2>&1"], capture_output=True, text=True, env=dict(os.environ, BD8_SOFT_LOG=log))
    finally:
        os.remove(C)
    sites = []; seen = set()
    for l in open(log).read().strip().split("\n"):
        mm = re.search(r'\(([^)]*bd8-diag\.ts):(\d+):(\d+)\) :: (.*)', l)
        if not mm: continue
        k = (int(mm.group(2)), int(mm.group(3)))
        if k in seen: continue
        seen.add(k); sites.append((k[0], k[1], mm.group(4)))
    return s.split("\n"), sites, r.stdout
LIT = re.compile(r'"((?:\\.|[^"\\])*)"|`((?:\\.|[^`\\])*)`')
if __name__ == "__main__":
    T = os.path.abspath(sys.argv[1])
    src, sites, out = run(T)
    fam = family()
    for ln, col, msg in sites:
        line = src[ln - 1]; st = line.rfind("expect(", 0, col)
        expr = line[st:col + 6] if st >= 0 else line[max(0, col - 150):col]
        mc = re.search(r'Expected to (?:not )?contain: (.*?) \| Received', msg)
        if mc:
            expr = line[st:col + 6] + line[col + 6:].split(");")[0][:200]
        lits = [a or b for a, b in LIT.findall(expr)]
        if mc and not lits: lits = [mc.group(1).strip('"')]
        print(f"L{ln}:C{col} {expr[:230]}")
        for lit in lits[-2:]:
            lit = bytes(lit, "utf8").decode("unicode_escape", errors="ignore") if "\\" in lit and "\\u" not in lit else lit
            if len(lit) < 4: continue
            hits = [(os.path.relpath(f, os.path.dirname(PLUGIN)), t[:t.find(lit)].count("\n") + 1) for f, t in fam if lit in t]
            if hits:
                print(f"    FOUND {lit[:90]!r}: " + ", ".join(f"{a}:{b}" for a, b in hits[:3]) + (f" (+{len(hits)-3})" if len(hits) > 3 else ""))
            else:
                ids = sorted(set(re.findall(r'[A-Za-z_][A-Za-z0-9_]{4,}', lit)), key=len, reverse=True)[:2]
                print(f"    ABSENT {lit[:110]!r}")
                for i in ids:
                    near = [(os.path.relpath(f, os.path.dirname(PLUGIN)), n + 1, x.strip()[:150]) for f, t in fam for n, x in enumerate(t.split("\n")) if i in x][:1]
                    for a, b, x in near: print(f"        ~{i}: {a[-60:]}:{b}: {x}")
    print("sites:", len(sites))
    for mm in re.finditer(r'(ENOENT[^\n]*|Cannot find[^\n]*|TypeError[^\n]*|SyntaxError[^\n]*)', out): print("THROWN:", mm.group(1)[:220])
