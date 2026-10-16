"""📇 Registration of the board crates: workspace members, build owners and launch routes."""
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wreg
from bcrates import CRATES, crate_pkg

R = wreg.R
INFREL = wreg.INFREL

def project_name(cid): return f"@semio-tech/os-infinite-board-{cid}-rs"

class C:
    def __init__(s, i, row): s.idx = 100 + i; s.id = row[0]; s.folder = row[2]

def crates(): return [C(i, r) for i, r in enumerate(CRATES)]

def reg_members(cs):
    p = R + "/Cargo.toml"; t = open(p, encoding='utf-8').read()
    i = t.index("members = ["); j = t.index("\n]", i)
    body = t[i:j].split("\n"); head, items = body[0], body[1:]
    added = 0
    for c in cs:
        m = f"{INFREL}/{c.folder}/📦️packages/🦀️rust"
        if any(l.strip().rstrip(',').strip('"') == m for l in items): continue
        idx = len(items)
        for n, l in enumerate(items):
            v = l.strip().rstrip(',').strip('"')
            if v.encode() > m.encode(): idx = n; break
        items.insert(idx, f'    "{m}",'); added += 1
    open(p, 'w', encoding='utf-8').write(t[:i] + "\n".join([head] + items) + t[j:])
    return added

def reg_owners(cs):
    p = R + "/📋️build-owners.json"; t = open(p, encoding='utf-8').read(); added = 0
    for c in cs:
        name = project_name(c.id)
        if f'"name": "{name}"' in t: continue
        root = f"{INFREL}/{c.folder}/📦️packages/🦀️rust"
        row = f'''    {{
      "name": "{name}",
      "root": "{root}",
      "manifest": "{root}/📋️project.json",
      "packages": []
    }},
'''
        rows = [(m.start(), m.group(1)) for m in re.finditer(r'    \{\n      "name": "([^"]+)"', t)]
        pos = None
        for s, n in rows:
            if n.encode() > name.encode() and n.startswith("@semio-tech/"): pos = s; break
        if pos is None: raise SystemExit("no owner anchor")
        t = t[:pos] + row + t[pos:]; added += 1
    open(p, 'w', encoding='utf-8').write(t)
    return added

def routes(c, orders):
    name = crate_pkg(c.id); proj = project_name(c.id); out = []; n = 0
    for tgt, grp, label in (("build", "4_build", "🏗️build"), ("check", "4_gate", "🔎️check"), ("test", "4_test", "🧪️test")):
        for conf, cl in (("prototype", "🛠️prototype"), ("release", "🚀️release")):
            tail = "--offline" if tgt == "test" else "--lib --offline"
            out.append(f'''    {{
      "name": "🦀️ {name} {label} {cl}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {proj}:{tgt} --configuration={conf} --parallel=1 -- {tail}",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{grp}",
        "order": {orders[n]}
      }}
    }},
''')
            n += 1
    return "".join(out)

def reg_launch(cs):
    texts = {f: open(f, encoding='utf-8').read() for f in wreg.LAUNCH}
    used = set()
    for t in texts.values(): used |= set(float(x) for x in re.findall(r'"order": ([0-9.]+)', t))
    added = 0
    for c in cs:
        name = crate_pkg(c.id)
        if all(f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t for t in texts.values()): continue
        orders = []
        for tgt in ("build", "check", "test"):
            k = 0
            for conf in range(2):
                while True:
                    v = round(wreg.BASE[tgt] + 0.00001 * (c.idx * 10 + k), 5); k += 1
                    if v not in used: break
                used.add(v); orders.append(repr(v))
        block = routes(c, orders)
        for f in list(texts):
            t = texts[f]
            if f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t: continue
            a = t.index(wreg.ANCHOR); e = t.index("\n    },\n", a) + len("\n    },\n")
            texts[f] = t[:e] + block + t[e:]
        added += 1
    for f, t in texts.items(): open(f, 'w', encoding='utf-8').write(t)
    return added
