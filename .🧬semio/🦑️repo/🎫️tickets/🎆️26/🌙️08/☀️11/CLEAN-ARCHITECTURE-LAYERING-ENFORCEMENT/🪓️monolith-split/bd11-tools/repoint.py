#!/usr/bin/env python3
"""usage: repoint.py <test.ts> [--dry] : in every test block that has a failing assertion, replace readFileSync(new URL("<non-test .rs>", import.meta.url),"utf8") by pluginFamilyText(); adds the import."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import needles
T = os.path.abspath(sys.argv[1]); dry = "--dry" in sys.argv
src, sites, out = needles.run(T)
lines = src[:]
starts = [i for i, l in enumerate(lines) if re.match(r'\s*(?:test|it)(?:\.\w+)?\(', l)]
def block(i):
    s = max([x for x in starts if x <= i], default=None)
    if s is None: return None
    e = min([x for x in starts if x > i], default=len(lines))
    return (s, e)
blocks = sorted({block(ln - 1) for ln, _, _ in sites if block(ln - 1)})
Q = "[\"']"
READ = re.compile(r'readFileSync\(\s*(?:new URL\(\s*' + Q + r'([^"\']+\.rs)' + Q + r'\s*,\s*import\.meta\.url\s*\)|(?:join|resolve)\(\s*import\.meta\.dir\s*,\s*' + Q + r'([^"\']+\.rs)' + Q + r'\s*\))\s*,\s*' + Q + r'utf8' + Q + r'\s*\)')
D = os.path.dirname(T); n = 0; used = set()
MODS = os.path.dirname(needles.PLUGIN)
def sub(m):
    global n
    p = os.path.normpath(os.path.join(D, m.group(1) or m.group(2)))
    if "/🧪️tests/" in p or "/🧷️companions/" in p or os.path.exists(p): return m.group(0)
    if p.startswith(needles.PLUGIN + "/"):
        parts = p[len(needles.PLUGIN) + 1:].split("/")
        n += 1
        if len(parts) == 1 or parts[0].endswith(".rs"): used.add("pluginFamilyText"); return "pluginFamilyText()"
        used.add("pluginSubtreeText"); return f'pluginSubtreeText("{parts[0]}")'
    if p.startswith(MODS + "/"):
        mod = p[len(MODS) + 1:].split("/")[0]; used.add("moduleFamilyText"); n += 1; return f'moduleFamilyText("{mod}")'
    return m.group(0)
for s_, e in blocks:
    for i in range(s_, e):
        lines[i] = READ.sub(sub, lines[i])
text = "\n".join(lines)
if n:
    rel = os.path.relpath(needles.PLUGIN + "/🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts", D)
    have = re.search(r'import\s*\{([^}]*)\}\s*from\s*"[^"]*🔌️plugin-family/🟦️\.ts"', text)
    if have:
        names = {x.strip() for x in have.group(1).split(",") if x.strip()} | used
        text = text[:have.start()] + "import {" + ", ".join(sorted(names)) + "}" + text[text.index("from", have.start()):]
    else:
        imp = 'import {' + ", ".join(sorted(used)) + '} from "' + rel + '";\n'
        m = list(re.finditer(r'^import .*$', text, re.M))
        pos = m[-1].end() + 1 if m else 0
        text = text[:pos] + imp + text[pos:]
print(f"{os.path.basename(os.path.dirname(os.path.dirname(T)))}: blocks={len(blocks)} reads replaced={n}")
if not dry and n: open(T, "w").write(text)
