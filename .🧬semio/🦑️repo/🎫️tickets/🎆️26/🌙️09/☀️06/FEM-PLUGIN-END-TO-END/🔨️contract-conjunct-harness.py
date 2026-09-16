#!/usr/bin/env python3
"""🧪 Generates a bun harness that evaluates one `toolJobFem…Exact` contract conjunct by conjunct.
Usage: python3 🔨️contract-conjunct-harness.py <functionName> <verifyBlockStartLine> <verifyBlockEndLine> > out.ts
The verify block lines are the `const x = policyReadFileSafe(this.root, "…")` reads feeding the call."""
import re, sys
root = "/Users/ueli/Documents/semio"
fn, a, b = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
src = open(f"{root}/📜️script.ts", encoding="utf-8").read()
lines = src.split("\n")
start = src.index(f"function {fn}(")
i = src.index("{", start)
j = src.index("\n}\n", i)
body = src[i+1:j]
head = src[start:i+1]
params = re.search(r"\((.*?)\)\s*:\s*boolean", head, re.S).group(1)
param_names = [p.strip().split(":")[0].strip() for p in params.split(",") if p.strip()]
sub = sys.argv[4] if len(sys.argv) > 4 else None
if sub:
    k = body.index(f"  const {sub} = (")
    e = body.index("\n  );", k)
    prefix = body[:k]
    retexpr = body[k + len(f"  const {sub} = ("):e]
else:
    ret = body.rfind("\n  return (")
    if ret == -1: ret = body.rfind("\n  return ")
    prefix = body[:ret]
    retexpr = body[ret:].strip()
    retexpr = retexpr[len("return"):].strip().rstrip(";").strip()
    if retexpr.startswith("(") and retexpr.endswith(")"): retexpr = retexpr[1:-1]
# split top-level &&
parts=[]; cur=""; depth=0; k=0; instr=None
while k < len(retexpr):
    c = retexpr[k]
    if instr:
        cur += c
        if c == "\\": cur += retexpr[k+1]; k += 2; continue
        if c == instr: instr = None
        k += 1; continue
    if c in "\"'`": instr = c; cur += c; k += 1; continue
    if c in "([{": depth += 1
    elif c in ")]}": depth -= 1
    if depth == 0 and retexpr.startswith("&&", k):
        parts.append(cur.strip()); cur = ""; k += 2; continue
    cur += c; k += 1
parts.append(cur.strip())
reads = []
for ln in lines[a-1:b]:
    m = re.match(r'\s*const (\w+) = policyReadFileSafe\(this\.root, "([^"]+)"\);', ln)
    if m: reads.append((m.group(1), m.group(2)))
out = ['import { readFileSync, existsSync } from "node:fs";', f'import {{ toolJobRustBlock }} from "{root}/📜️script.ts";', 'const read = (p: string) => existsSync(p) ? readFileSync(p, "utf8") : "";']
for name, path in reads:
    out.append(f'const {name} = read("{root}/{path}");')
out.append(f"function harness({params}) {{")
out.append(prefix)
out.append("  const checks: Array<[string, () => boolean]> = [")
for p in parts:
    out.append("    [" + repr(p)[1:-1].replace('"','\\"').join(['"','"']) + ", () => (" + p + ")],")
out.append("  ];")
out.append('  let failed = 0; for (const [label, check] of checks) { let ok = false; try { ok = Boolean(check()); } catch (error) { console.log("THROW", label.slice(0, 160), String(error).slice(0, 120)); failed += 1; continue; } if (!ok) { console.log("FAIL", label.slice(0, 220)); failed += 1; } }')
out.append('  console.log(`[DEBUG] ${checks.length} conjuncts, ${failed} failing`); return failed === 0;')
out.append("}")
out.append(f"harness({', '.join(param_names)});")
print("\n".join(out))
