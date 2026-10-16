# 🗂️ Registers the skeleton crates of the plugin partition: workspace members, owner rows, six launch routes each in both launch files, and the transitional dependency block, feature forwards and boundary triple of the facade.
# usage: gen_registry.py dry|write [members|owners|launch|facade]...   (idempotent; every edit re-reads its file right before writing and asserts unique anchors)
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C

ROOT_TOML = C.R + '/Cargo.toml'
OWNERS = C.R + '/📋️build-owners.json'
LAUNCH = [C.R + '/.vscode/launch.json', C.R + '/.vscode/🧩️launch.seed.jsonc']
FACADE_TOML = C.PKG + '/Cargo.toml'
ANCHOR = '"name": "🦀️ semio-framework-os-renderer-wgpu-test-support 🧪️test 🚀️release"'
ENV = '"env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'
def b(s): return s.encode('utf-8')

def reg_members(crates, write):
    t = open(ROOT_TOML, encoding='utf-8').read()
    lines = t.split('\n')
    i = lines.index('members = [')
    j = next(k for k in range(i, len(lines)) if lines[k] == ']')
    old = [l.strip().rstrip(',').strip('"') for l in lines[i + 1:j]]
    assert old == sorted(old, key=b), 'members block must be byte-sorted'
    new = [c['pkg'] for c in crates.values()]
    todo = [p for p in new if p not in old]
    if not todo: return 'members: all present'
    block = sorted(old + todo, key=b)
    lines[i + 1:j] = ['    "' + p + '",' for p in block]
    out = '\n'.join(lines)
    if write:
        assert open(ROOT_TOML, encoding='utf-8').read() == t, 'root Cargo.toml changed; rerun'
        open(ROOT_TOML, 'w', encoding='utf-8').write(out)
    return f'members: +{len(todo)} ({len(old)} -> {len(block)})'

def reg_owners(crates, write):
    t = open(OWNERS, encoding='utf-8').read()
    o = json.loads(t)
    assert t == json.dumps(o, indent=2, ensure_ascii=False) + '\n', 'owner registry not canonical'
    names = {x['name'] for x in o['owners']}; roots = {x['root'] for x in o['owners']}
    rows = []
    for c in crates.values():
        assert c['project'] not in names or any(x['name'] == c['project'] and x['root'] == c['pkg'] for x in o['owners']), 'owner name collision ' + c['project']
        assert c['pkg'] not in roots or c['project'] in names, 'owner root collision ' + c['pkg']
        if c['project'] not in names: rows.append(dict(name=c['project'], root=c['pkg'], manifest=c['pkg'] + '/📋️project.json', packages=[]))
    if not rows: return 'owners: all present'
    for r in sorted(rows, key=lambda r: r['name']):
        at = next((k for k, x in enumerate(o['owners']) if x['name'] > r['name']), len(o['owners']))
        o['owners'].insert(at, r)
    out = json.dumps(o, indent=2, ensure_ascii=False) + '\n'
    if write:
        assert open(OWNERS, encoding='utf-8').read() == t, 'owners changed; rerun'
        open(OWNERS, 'w', encoding='utf-8').write(out)
    return f'owners: +{len(rows)}'

def used_orders(texts):
    used = set()
    for t in texts: used |= set(round(float(x), 5) for x in re.findall(r'"order": ([0-9.]+)', t))
    return used
def window(used, start, n):
    k = int(round(start * 100000))
    while not all(round((k + i) / 100000, 5) not in used for i in range(n)): k += 1
    return k

def routes(c, orders):
    spec = [('build', '4_build', '🏗️build'), ('check', '4_gate', '🔎️check'), ('test', '4_test', '🧪️test')]
    out = []; n = 0
    for tgt, grp, label in spec:
        for conf, cl in (('prototype', '🛠️prototype'), ('release', '🚀️release')):
            out.append(f'''    {{
      "name": "🦀️ {c['name']} {label} {cl}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {c['project']}:{tgt} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{grp}",
        "order": {orders[n]}
      }},
      {ENV}
    }},
''')
            n += 1
    return ''.join(out)

def reg_launch(crates, write, order):
    texts = {f: open(f, encoding='utf-8').read() for f in LAUNCH}
    todo = [c for c in crates.values() if not all(f'"name": "🦀️ {c["name"]} 🏗️build 🛠️prototype"' in t for t in texts.values())]
    if not todo: return 'launch: all present'
    used = used_orders(texts.values())
    n = 2 * len(crates)
    base = {'build': window(used, 900.3000, n), 'check': window(used, 922.0620, n), 'test': window(used, 903.6000, n)}
    blocks = []
    for c in todo:
        i = order.index(c['key'])
        orders = []
        for tgt in ('build', 'check', 'test'):
            for conf in range(2): orders.append(repr(round((base[tgt] + 2 * i + conf) / 100000, 5)))
        blocks.append(routes(c, orders))
    block = ''.join(blocks)
    outs = {}
    for f, t in texts.items():
        assert t.count(ANCHOR) == 1, 'launch anchor ' + f
        a = t.index(ANCHOR); e = t.index('\n    },\n', a) + len('\n    },\n')
        t2 = t[:e] + block + t[e:]
        json.loads(t2) if f.endswith('.json') else None
        outs[f] = t2
    if write:
        for f, t2 in outs.items():
            assert open(f, encoding='utf-8').read() == texts[f], 'launch file changed; rerun'
            open(f, 'w', encoding='utf-8').write(t2)
    return f'launch: +{len(todo)} crates x 6 routes x 2 files; bands build {base["build"]/1e5} check {base["check"]/1e5} test {base["test"]/1e5} ({n} keys each)'

def facade_rel(c): return os.path.relpath(C.R + '/' + c['pkg'], C.PKG)
def reg_facade(crates, write):
    t = open(FACADE_TOML, encoding='utf-8').read()
    prod = [c for c in crates.values() if c['kind'] in ('lower', 'instance')]
    todo = [c for c in prod if c['name'] + ' = ' not in t]
    new = t
    if todo:
        optional = {'testing-laws', 'testing-acceptance'}
        plain = [c for c in todo if c['key'] not in optional and c['key'] != 'reactor-wit-bridge']
        opt = [c for c in todo if c['key'] in optional]
        wit = [c for c in todo if c['key'] == 'reactor-wit-bridge']
        head = ['# 🧭️ Transitional carve targets of the crate partition of this crate (PLAN-PLUGIN-CRATES.md, section 12.5): the work package that carves a module replaces its mount in `🪝️mounts` by `extern crate <crate> as <module>;`; phase 2 of WP0 prunes this block to the facade re-exports.']
        lines = head + [c['name'] + ' = { path = "' + facade_rel(c) + '" }' for c in plain] + [c['name'] + ' = { path = "' + facade_rel(c) + '", optional = true }' for c in opt]
        a = '\n[dev-dependencies]\n'
        assert new.count(a) == 1, 'facade anchor dev-dependencies'
        tail = ['', "[target.'cfg(all(target_arch = \"wasm32\", target_env = \"p2\"))'.dependencies]"] + [c['name'] + ' = { path = "' + facade_rel(c) + '" }' for c in wit] if wit else []
        new = new.replace(a, '\n' + '\n'.join(lines) + ('\n' + '\n'.join(tail) if tail else '') + '\n' + a)
    feat_old = 'artifact-app-testing = []\nmedia-turn-testing = []\nreserved-producer-testing = []\noriginal-actor-context-testing = []\ncanvas-runtime-assets = ["dep:semio-framework-canvas-assets"]\n'
    if feat_old in new:
        def fwd(f): return ', '.join('"' + crates[k]['name'] + '/' + f + '"' for k in C.FEATURES[f])
        aat = '"dep:semio-framework-plugin-testing-laws", "dep:semio-framework-plugin-testing-acceptance", ' + fwd('artifact-app-testing')
        feat_new = (f'artifact-app-testing = [{aat}]\nmedia-turn-testing = [{fwd("media-turn-testing")}]\nreserved-producer-testing = [{fwd("reserved-producer-testing")}]\n'
                    f'original-actor-context-testing = [{fwd("original-actor-context-testing")}]\ncanvas-runtime-assets = ["dep:semio-framework-canvas-assets", "semio-framework-plugin-builder/canvas-runtime-assets"]\n')
        new = new.replace(feat_old, feat_new)
    tri_old = 'maxSourceBytes = 1048576\nmaxSourceFiles = 128\nmaxActiveLines = 12000\n'
    if tri_old in new: new = new.replace(tri_old, 'maxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n')
    if new == t: return 'facade: up to date'
    if write:
        assert open(FACADE_TOML, encoding='utf-8').read() == t, 'facade manifest changed; rerun'
        open(FACADE_TOML, 'w', encoding='utf-8').write(new)
    return f'facade: +{len(todo)} dependency lines, features forwarded, boundary triple at the ceiling'

def main():
    mode = sys.argv[1]
    which = set(sys.argv[2:]) or {'members', 'owners', 'launch', 'facade'}
    crates, order = C.build()
    write = mode == 'write'
    for name, fn in (('members', lambda: reg_members(crates, write)), ('owners', lambda: reg_owners(crates, write)), ('launch', lambda: reg_launch(crates, write, order)), ('facade', lambda: reg_facade(crates, write))):
        if name in which: print(fn())

if __name__ == '__main__': main()
