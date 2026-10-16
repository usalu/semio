import sys, os, re, json, bisect, shutil, collections
sys.path.insert(0, ".")
from oracle2 import *

REAL = "/Users/ueli/Documents/semio/"
REPOR = REAL
LIBRARY = REPOR + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"

def facade_manifest(em, ci):
    lay = em.lay
    orig = open(MCP + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf-8").read()
    head = orig[:orig.index("[dependencies]")]
    if "[package.metadata.semio.rust-source-boundary]" not in head:
        head = head.replace('[lints]\nworkspace = true', '[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true', 1)
    deps = {}; tdeps = {}; ddeps = {}
    for r in sorted(em.ext_roots[ci]):
        k = "semio-framework-os-kernel" if r == "store" else ident_to_key(r)
        if k is None: continue
        if k in TARGET_DEPS: tdeps[k] = TARGET_DEPS[k]
        elif k in DEPS: deps[k] = DEPS[k]
    for r in sorted(em.ext_roots_test[ci]):
        k = "semio-framework-os-kernel" if r == "store" else ident_to_key(r)
        if k is None or k in deps or k in tdeps: continue
        if k in TARGET_DEPS: tdeps[k] = TARGET_DEPS[k]
        elif k in DEPS: ddeps[k] = DEPS[k]
        elif k in DEV_DEPS: ddeps[k] = DEV_DEPS[k]
    inter = set(em.crate_deps[ci])
    for key, names in lay.plan.FACADE.items():
        for n in names:
            inter.add(em.out_of_atom(find_pub_atom(em, n, key.split("::")[0])).crate)
    inter_dev = sorted(em.crate_deps_dev[ci] - inter)
    inter = sorted(inter)
    out = head.rstrip("\n") + "\n\n[dependencies]\n"
    for k, v in sorted(deps.items()):
        if k == "semio-framework-os-kernel": out += f'{k} = {{ path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["sync", "ureq"] }}\n'
        else: out += f"{k} = {v}\n"
    for c in inter:
        out += f'{crate_pkg(lay, c)} = {{ path = "../../{lay.plan.NODES[lay.names[c]][0]}/📦️packages/🦀️rust" }}\n'
    if tdeps:
        out += "\n[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]\n"
        for k, v in sorted(tdeps.items()): out += f"{k} = {v}\n"
    dev = [f"{k} = {v}" for k, v in sorted(ddeps.items())]
    for c in sorted(set(inter) | set(inter_dev)):
        tk = ci in em.testkit_users.get(c, set())
        feat = ', features = ["testkit"]' if tk else ""
        if c in inter_dev or tk:
            dev.append(f'{crate_pkg(lay, c)} = {{ path = "../../{lay.plan.NODES[lay.names[c]][0]}/📦️packages/🦀️rust"{feat} }}')
    if "tokio-tungstenite" not in "".join(dev) and "tokio-tungstenite" in orig: pass
    if dev: out += "\n[dev-dependencies]\n" + "\n".join(dev) + "\n"
    return out

import tree
SRCROOT = tree.REPO

def collect(planmod):
    lay, em, files = generate(planmod, False)
    writes = dict(files)
    for ci, n in enumerate(lay.names):
        if n == "runtime":
            writes[MCP + "/📦️packages/🦀️rust/Cargo.toml"] = facade_manifest(em, ci)
            continue
        pd = pkg_dir(lay, ci)
        writes[pd + "/Cargo.toml"] = manifest(em, ci)
        writes[pd + "/📋️project.json"] = project_json(em, ci)
    return lay, em, writes

# ---------- registrations
def members_edit(text, new):
    m = re.search(r'\[workspace\]\n(?:[^\[]*?)members = \[\n(.*?)\n\]', text, re.S)
    body = m.group(1)
    rows = body.split("\n")
    vals = [re.match(r'\s*"(.*)",?\s*$', r).group(1) for r in rows]
    add = sorted(set(new) - set(vals))
    for a in add:
        pos = len(vals)
        for i, v in enumerate(vals):
            if v.encode() > a.encode(): pos = i; break
        vals.insert(pos, a); rows.insert(pos, f'    "{a}",')
    rows = [r if r.endswith(",") else r + "," for r in rows]
    return text[:m.start(1)] + "\n".join(rows) + text[m.end(1):]

def owners_edit(text, rows_new):
    d = json.loads(text)
    owners = d["owners"]
    names = {o["name"] for o in owners}
    for r in rows_new:
        if r["name"] in names: continue
        pos = len(owners)
        for i, o in enumerate(owners):
            if o["name"] > r["name"]: pos = i; break
        owners.insert(pos, r)
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"

def taxonomy_edit(text, folders):
    d = json.loads(text)
    kinds = d["semanticDirectoryMemberKinds"]
    for (depth, name) in folders:
        key = "members-of-" * depth + "modules"
        if key not in kinds:
            kinds[key] = {"ownerKindIds": ["members-of-" * (depth - 1) + "modules"], "memberNames": [], "source": "registry"}
        if name not in kinds[key]["memberNames"]: kinds[key]["memberNames"].append(name)
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"

def launch_blocks(crates, orders, indent="    "):
    blocks = []
    for (pkg, proj) in crates:
        for (verb, label, grp) in (("build", "🏗️build", "4_build"), ("check", "🔎️check", "4_gate"), ("test", "🧪️test", "4_test")):
            for (cfg, clabel) in (("prototype", "🛠️prototype"), ("release", "🚀️release")):
                o = orders[grp].pop(0)
                name = f"🦀️ {pkg} {label} {clabel}"
                cmd = f"bun repo run {proj}:{verb} --configuration={cfg} --parallel=1 -- --lib --offline"
                b = indent + "{\n" + f'{indent}  "name": {json.dumps(name, ensure_ascii=False)},\n{indent}  "type": "node-terminal",\n{indent}  "request": "launch",\n{indent}  "command": {json.dumps(cmd, ensure_ascii=False)},\n{indent}  "cwd": "${{workspaceFolder}}",\n{indent}  "presentation": {{\n{indent}    "group": "{grp}",\n{indent}    "order": {orders["fmt"](o)}\n{indent}  }}\n{indent}}}'
                blocks.append(b)
    return blocks

def launch_edit(text, crates, anchor_name):
    names_present = set(re.findall(r'"name": "(🦀️ semio-framework-os-mcp-[^"]*)"', text))
    crates = [c for c in crates if not any(n.startswith(f"🦀️ {c[0]} ") for n in names_present)]
    if not crates: return text
    used = {"4_build": [], "4_gate": [], "4_test": []}
    for m in re.finditer(r'"group": "(4_build|4_gate|4_test)",\s*"order": ([0-9.]+)', text):
        used[m.group(1)].append(float(m.group(2)))
    band = {"4_build": 900, "4_gate": 922, "4_test": 903}
    orders = {}
    for g, b in band.items():
        inband = [u for u in used[g] if b <= u < b + 1]
        start = max(inband)
        orders[g] = [round(start + 0.00001 * (k + 1), 5) for k in range(len(crates) * 2)]
    orders["fmt"] = lambda o: repr(o)
    blocks = launch_blocks(crates, orders)
    i = text.index(f'"name": "{anchor_name}"')
    start = text.rfind("{", 0, i)
    depth = 0; j = start
    while True:
        c = text[j]
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    assert text[j + 1] == ",", "anchor must not be last"
    return text[:j + 2] + "\n" + ",\n".join(blocks) + "," + text[j + 2:]

def registry_edits(lay, em, writes):
    crates = [(crate_pkg(lay, ci), f"@semio-tech/framework-os-mcp-{n}-rs") for ci, n in enumerate(lay.names) if n != "runtime"]
    members = [os.path.relpath(pkg_dir(lay, ci), SRCROOT) for ci, n in enumerate(lay.names) if n != "runtime"]
    owners = [dict(name=p, root=os.path.relpath(pkg_dir(lay, ci), SRCROOT), manifest=os.path.relpath(pkg_dir(lay, ci) + "/📋️project.json", SRCROOT), packages=[]) for ci, (pkg, p) in zip([c for c, n in enumerate(lay.names) if n != "runtime"], crates)]
    folders = []
    for ci, n in enumerate(lay.names):
        if n == "runtime": continue
        rel = os.path.relpath(node_dir(lay, ci), MCP)
        folders.append((1 + 1, rel.split("/")[0]))
    for o in lay.outs:
        if o.kind == "prod" and not getattr(o, "primary", True):
            d = os.path.dirname(o.path)
            rel = os.path.relpath(d, MCP).split("/")
            folders.append((len(rel) + 1, rel[-1]))
    res = {}
    res[REPOR + "Cargo.toml"] = members_edit(open(REPOR + "Cargo.toml", encoding="utf-8").read(), members)
    res[REPOR + "📋️build-owners.json"] = owners_edit(open(REPOR + "📋️build-owners.json", encoding="utf-8").read(), owners)
    res[LIBRARY + "🔣️taxonomy.json"] = taxonomy_edit(open(LIBRARY + "🔣️taxonomy.json", encoding="utf-8").read(), folders)
    anchor = "🦀️ semio-framework-pets-traversal 🧪️test 🚀️release"
    for f in (".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"):
        res[REPOR + f] = launch_edit(open(REPOR + f, encoding="utf-8").read(), crates, anchor)
    return res

if __name__ == "__main__":
    mode = sys.argv[2] if len(sys.argv) > 2 else "dry"
    lay, em, writes = collect(sys.argv[1])
    regs = registry_edits(lay, em, writes)
    allw = {}
    for p, t in writes.items():
        allw[(REAL + p[len(SRCROOT) + 1:]) if p.startswith(SRCROOT + "/") else p] = t
    allw.update(regs)
    stage = SCR + "applied"
    shutil.rmtree(stage, ignore_errors=True)
    for p, t in allw.items():
        dst = stage + "/" + os.path.relpath(p, REPOR)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        open(dst, "w", encoding="utf-8").write(t)
    print("staged", len(allw), "files into", stage)
    if mode == "apply":
        for p, t in allw.items():
            os.makedirs(os.path.dirname(p), exist_ok=True)
            open(p, "w", encoding="utf-8").write(t)
        print("APPLIED", len(allw))
        import hashlib
        hs = {p: hashlib.sha256(t.encode()).hexdigest() for p, t in allw.items() if "/🌉️mcp/" in p}
        json.dump(hs, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "applied-hashes.json"), "w"), ensure_ascii=False, indent=0)
