# usage: register.py <key>...   idempotent: workspace member, owner row, six launch routes in launch.json and the seed
import sys, re, json
from mk import *
keys = sys.argv[1:]
LAUNCH = [R + "/.vscode/launch.json", R + "/.vscode/🧩️launch.seed.jsonc"]
ANCHOR = '"name": "🦀️ semio-framework-pets-staging 🧪️test 🚀️release"'
def project_name(key): return f"@semio-tech/framework-{CR[key][1]}-rs"
def member_path(key): return os.path.relpath(pkg(key), FW)
def reg_member(key):
    p = FW + "/Cargo.toml"; t = open(p, encoding="utf-8").read()
    m = member_path(key)
    if f'"{m}"' in t: return
    i = t.index("members = ["); j = t.index("\n]", i)
    body = t[i:j].split("\n")
    head, items = body[0], body[1:]
    idx = len(items)
    for n, l in enumerate(items):
        v = l.strip().rstrip(",").strip('"')
        if v.encode() > m.encode(): idx = n; break
    items.insert(idx, f'    "{m}",')
    open(p, "w", encoding="utf-8").write(t[:i] + "\n".join([head] + items) + t[j:])
    print("member", m, "at", idx)
def reg_owner(key):
    p = R + "/📋️build-owners.json"; t = open(p, encoding="utf-8").read()
    name = project_name(key)
    if f'"name": "{name}"' in t: return
    row = f'''    {{
      "name": "{name}",
      "root": "{os.path.relpath(pkg(key), R)}",
      "manifest": "{os.path.relpath(pkg(key), R)}/📋️project.json",
      "packages": []
    }},
'''
    rows = [(m.start(), m.group(1)) for m in re.finditer(r'    \{\n      "name": "([^"]+)"', t)]
    pos = None
    for s, n in rows:
        if n.encode() > name.encode() and n.startswith("@semio-tech/framework-"): pos = s; break
    if pos is None: raise SystemExit("no owner anchor")
    open(p, "w", encoding="utf-8").write(t[:pos] + row + t[pos:])
    print("owner", name)
def used_orders(t): return set(float(x) for x in re.findall(r'"order": ([0-9.]+)', t))
def routes(key, orders):
    name = CR[key][0]; proj = project_name(key); out = []
    spec = [("build", "4_build", "🏗️build"), ("check", "4_gate", "🔎️check"), ("test", "4_test", "🧪️test")]
    n = 0
    for tgt, grp, label in spec:
        for conf, cl in (("prototype", "🛠️prototype"), ("release", "🚀️release")):
            out.append(f'''    {{
      "name": "🦀️ {name} {label} {cl}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {proj}:{tgt} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{grp}",
        "order": {orders[n]}
      }}
    }},
''')
            n += 1
    return "".join(out)
BASE = {"build": 900.0711, "check": 922.0471, "test": 903.5871}
def reg_launch(key):
    texts = {f: open(f, encoding="utf-8").read() for f in LAUNCH}
    name = CR[key][0]
    if all(f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t for t in texts.values()): return
    used = set()
    for t in texts.values(): used |= used_orders(t)
    orders = []
    idx = list(CR).index(key)
    for tgt in ("build", "check", "test"):
        c = 0
        for conf in range(2):
            while True:
                v = round(BASE[tgt] + 0.00001 * (idx * 10 + c), 5); c += 1
                if v not in used: break
            used.add(v); orders.append(repr(v))
    block = routes(key, orders)
    for f, t in texts.items():
        if f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t: continue
        a = t.index(ANCHOR); e = t.index("\n    },\n", a) + len("\n    },\n")
        open(f, "w", encoding="utf-8").write(t[:e] + block + t[e:])
    print("launch", name)
for k in keys:
    reg_member(k); reg_owner(k); reg_launch(k)
