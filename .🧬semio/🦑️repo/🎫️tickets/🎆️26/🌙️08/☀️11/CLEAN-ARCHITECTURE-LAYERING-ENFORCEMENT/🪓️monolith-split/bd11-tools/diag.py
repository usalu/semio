#!/usr/bin/env python3
"""usage: diag.py <test.ts> [ctx=230] : soft-expect run; prints each failing assertion (line:col, message, preceding source text) and thrown errors"""
import re, sys, os, subprocess
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
T = os.path.abspath(sys.argv[1]); ctx = int(sys.argv[2]) if len(sys.argv) > 2 else 230
D = os.path.dirname(T); C = os.path.join(D, "bd8-diag.ts")
s = open(T).read()
m = re.search(r'import\s*\{([^}]*)\}\s*from\s*"bun:test";?', s)
names = [n.strip() for n in m.group(1).split(",") if n.strip() and n.strip() != "expect"]
shim = F + ("/bd11-tools/softexpect-long.ts" if os.environ.get("BD11_LONG") else "/bd8-tools/softexpect.ts")
s2 = s[:m.start()] + 'import { ' + ', '.join(names) + ' } from "bun:test"; import { expect } from "' + shim + '";' + s[m.end():]
open(C, "w").write(s2)
log = F + "/🗑️generated/bd11-soft.log"; open(log, "w").close()
env = dict(os.environ, BD8_SOFT_LOG=log, SEMIO_TEST_ARTIFACT_DIR=F + "/🗑️generated/bd11-artifacts", SEMIO_REPO_ROOT="/Users/ueli/Documents/semio")
try:
    r = subprocess.run(["bash", "-c", "ulimit -n 65536; cd /Users/ueli/Documents/semio && nice -n 10 perl -e 'alarm 240; exec @ARGV' -- bun test \"./" + os.path.relpath(C, "/Users/ueli/Documents/semio") + "\" 2>&1"], capture_output=True, text=True, env=env)
finally:
    os.remove(C)
out = r.stdout
src = s.split("\n")
seen = set()
for l in open(log).read().strip().split("\n"):
    if not l: continue
    mm = re.search(r'\(([^)]*bd8-diag\.ts):(\d+):(\d+)\) :: (.*)', l)
    if not mm: print("?", l[:300]); continue
    ln, col = int(mm.group(2)), int(mm.group(3))
    if (ln, col) in seen: continue
    seen.add((ln, col))
    line = src[ln - 1]
    msg = re.sub(r'\s+', ' ', mm.group(4))[:(1500 if os.environ.get('BD11_LONG') else 140)]
    print(f"L{ln}:C{col} [{msg}]\n   ...{line[max(0, col - ctx):col + 6]}")
print("failing assertions:", len(seen))
for l in out.split("\n"):
    if re.match(r'^(error:|\(fail\)| *[0-9]+ (pass|fail))', l) and 'expect(' not in l: print("RUN:", l[:200])
for mm in re.finditer(r'(ENOENT[^\n]*|Cannot find[^\n]*|TypeError[^\n]*|SyntaxError[^\n]*)', out):
    print("THROWN:", mm.group(1)[:220])
