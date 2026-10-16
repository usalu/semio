"""Registration of the world crates: stage manifests/project files and repo-level registries."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wman import *
from wgen import CRATES, crate_pkg

R = "/Users/ueli/Documents/semio"
INFREL = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"

def project_name(cid): return f"@semio-tech/os-infinite-world-{cid}-rs"

def project_json(cid):
    return json.dumps({"name": project_name(cid), "tags": ["language:rust", "role:product", "family:infinite"]}, indent=2, ensure_ascii=False) + "\n"

def write_manifests(crates, outroot, siblings, libtexts, testtexts, targets):
    set_test_targets(targets)
    for c in crates:
        pkg = f"{outroot}/🌍️world/{c.folder}/📦️packages/🦀️rust"
        os.makedirs(pkg, exist_ok=True)
        open(pkg + '/Cargo.toml', 'w', encoding='utf-8').write(build_manifest(c, c.idx, libtexts[c.id], testtexts[c.id], siblings[c.idx]))
        open(pkg + '/📋️project.json', 'w', encoding='utf-8').write(project_json(c.id))

def sibling_deps(crates):
    """crate idx -> set of lower crate idx it imports from (lib + tests)."""
    sib = {c.idx: set(c.cross) for c in crates}
    return sib

def reg_members(crates):
    p = R + "/Cargo.toml"; t = open(p, encoding='utf-8').read()
    i = t.index("members = ["); j = t.index("\n]", i)
    body = t[i:j].split("\n"); head, items = body[0], body[1:]
    for c in crates:
        m = f"{INFREL}/🌍️world/{c.folder}/📦️packages/🦀️rust"
        if any(l.strip().rstrip(',').strip('"') == m for l in items): continue
        idx = len(items)
        for n, l in enumerate(items):
            v = l.strip().rstrip(',').strip('"')
            if v.encode() > m.encode(): idx = n; break
        items.insert(idx, f'    "{m}",')
    open(p, 'w', encoding='utf-8').write(t[:i] + "\n".join([head] + items) + t[j:])

def reg_owners(crates):
    p = R + "/📋️build-owners.json"; t = open(p, encoding='utf-8').read()
    for c in crates:
        name = project_name(c.id)
        if f'"name": "{name}"' in t: continue
        root = f"{INFREL}/🌍️world/{c.folder}/📦️packages/🦀️rust"
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
        t = t[:pos] + row + t[pos:]
    open(p, 'w', encoding='utf-8').write(t)

LAUNCH = [R + "/.vscode/launch.json", R + "/.vscode/🧩️launch.seed.jsonc"]
ANCHOR = '"name": "🦀️ semio-framework-pets-staging 🧪️test 🚀️release"'
BASE = {"build": 900.0811, "check": 922.0511, "test": 903.5911}

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

def reg_launch(crates):
    texts = {f: open(f, encoding='utf-8').read() for f in LAUNCH}
    used = set()
    for t in texts.values(): used |= set(float(x) for x in re.findall(r'"order": ([0-9.]+)', t))
    for c in crates:
        name = crate_pkg(c.id)
        if all(f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t for t in texts.values()): continue
        orders = []
        for tgt in ("build", "check", "test"):
            k = 0
            for conf in range(2):
                while True:
                    v = round(BASE[tgt] + 0.00001 * (c.idx * 10 + k), 5); k += 1
                    if v not in used: break
                used.add(v); orders.append(repr(v))
        block = routes(c, orders)
        for f in list(texts):
            t = texts[f]
            if f'"name": "🦀️ {name} 🏗️build 🛠️prototype"' in t: continue
            a = t.index(ANCHOR); e = t.index("\n    },\n", a) + len("\n    },\n")
            texts[f] = t[:e] + block + t[e:]
    for f, t in texts.items(): open(f, 'w', encoding='utf-8').write(t)
