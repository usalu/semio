# 🗂️ Registers the 88 skeleton crates: workspace members, owner rows, six launch routes each in both launch files, and the transitional dependency block of the top crate.
# usage: gen_registry.py dry|write [members|owners|launch|top]...   (idempotent; every edit re-reads its file and asserts unique anchors)
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C

ROOT_TOML = C.R + '/Cargo.toml'
OWNERS = C.R + '/📋️build-owners.json'
LAUNCH = [C.R + '/.vscode/launch.json', C.R + '/.vscode/🧩️launch.seed.jsonc']
TOP_TOML = C.T + '/📦️packages/🦀️rust/Cargo.toml'
ANCHOR = '"name": "🦀️ semio-framework-canvas-scene 🧪️test 🚀️release"'

def b(s): return s.encode('utf-8')

def reg_members(crates, write):
    t = open(ROOT_TOML, encoding='utf-8').read()
    new = [c['pkg'] for k, c in crates.items()]
    present = [p for p in new if f'"{p}"' in t]
    todo = [p for p in new if p not in present]
    if not todo: return 'members: all present'
    prefix = C.T_REL + '/'
    lines = t.split('\n')
    idx = [i for i, l in enumerate(lines) if l.strip().startswith('"' + prefix) and l.strip().endswith('",')]
    assert idx and idx == list(range(idx[0], idx[-1] + 1)), 'wgpu member block must be contiguous'
    old = [lines[i].strip().rstrip(',').strip('"') for i in idx]
    block = sorted(old + todo, key=b)
    assert set(block[:0]) == set()
    lines[idx[0]:idx[-1] + 1] = ['    "' + p + '",' for p in block]
    if write: open(ROOT_TOML, 'w', encoding='utf-8').write('\n'.join(lines))
    return f'members: +{len(todo)} (block {len(old)} -> {len(block)})'

def owner_row(c):
    return f'''    {{
      "name": "{c['project']}",
      "root": "{c['pkg']}",
      "manifest": "{c['pkg']}/📋️project.json",
      "packages": []
    }},
'''

def reg_owners(crates, write):
    t = open(OWNERS, encoding='utf-8').read()
    todo = sorted([c for c in crates.values() if f'"name": "{c["project"]}"' not in t], key=lambda c: b(c['project']))
    if not todo: return 'owners: all present'
    anchor = '    {\n      "name": "@semio-tech/os-services-rs",'
    assert t.count(anchor) == 1, 'owner anchor'
    block = ''.join(owner_row(c) for c in todo)
    t = t.replace(anchor, block + anchor)
    json.loads(t)
    if write: open(OWNERS, 'w', encoding='utf-8').write(t)
    return f'owners: +{len(todo)}'

def used_orders(texts):
    used = set()
    for t in texts: used |= set(round(float(x), 5) for x in re.findall(r'"order": ([0-9.]+)', t))
    return used

def window(used, start, n):
    k = int(round(start * 100000))
    while not all(round((k + i) / 100000, 5) not in used for i in range(n)): k += 1
    return k

def routes(crate, orders):
    spec = [('build', '4_build', '🏗️build'), ('check', '4_gate', '🔎️check'), ('test', '4_test', '🧪️test')]
    out = []; n = 0
    for tgt, grp, label in spec:
        for conf, cl in (('prototype', '🛠️prototype'), ('release', '🚀️release')):
            out.append(f'''    {{
      "name": "🦀️ {crate['name']} {label} {cl}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {crate['project']}:{tgt} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{grp}",
        "order": {orders[n]}
      }}
    }},
''')
            n += 1
    return ''.join(out)

def reg_launch(crates, write):
    texts = {f: open(f, encoding='utf-8').read() for f in LAUNCH}
    todo = [c for c in crates.values() if not all(f'"name": "🦀️ {c["name"]} 🏗️build 🛠️prototype"' in t for t in texts.values())]
    if not todo: return 'launch: all present'
    used = used_orders(texts.values())
    n = 2 * len(crates)
    base = {'build': window(used, 900.0800, n), 'check': window(used, 922.0520, n), 'test': window(used, 903.5920, n)}
    position = {k: i for i, k in enumerate(C.ALL_KEYS)}
    blocks = []
    for c in todo:
        i = position[c['key']]
        orders = []
        for tgt in ('build', 'check', 'test'):
            for conf in range(2):
                orders.append(repr(round((base[tgt] + 2 * i + conf) / 100000, 5)))
        blocks.append(routes(c, orders))
    block = ''.join(blocks)
    for f, t in texts.items():
        assert t.count(ANCHOR) == 1, 'launch anchor ' + f
        a = t.index(ANCHOR); e = t.index('\n    },\n', a) + len('\n    },\n')
        t2 = t[:e] + block + t[e:]
        json.loads(t2)
        if write: open(f, 'w', encoding='utf-8').write(t2)
    return f'launch: +{len(todo)} crates x 6 routes x 2 files; bands build {base["build"]/1e5} check {base["check"]/1e5} test {base["test"]/1e5} ({n} keys each)'

def reg_top(crates, write):
    t = open(TOP_TOML, encoding='utf-8').read()
    prod = [c for c in crates.values() if c['kind'] in ('lib', 'shell')]
    todo = [c for c in prod if c['name'] + ' = ' not in t]
    if not todo: return 'top: all present'
    def line(c):
        return c['name'] + ' = { path = "../../' + c['folder'] + '/📦️packages/🦀️rust" }'
    notwasi = [c for c in todo if c['cfg'] == 'both']
    native = [c for c in todo if c['cfg'] == 'native']
    browser = [c for c in todo if c['cfg'] == 'wasm']
    plain = [c for c in todo if c['cfg'] == 'all']
    head = '# 🧭️ Transitional carve targets of the crate partition of this crate: the work package that carves a module replaces its mount in `🧊️renderer/📥️mounts` by `extern crate <crate> as <module>;`; the phase 2 pass of WP0 prunes this block to the entrypoint dependencies.'
    # not-wasi table: before the linux comment
    a1 = "# 🐧️ winit's Linux backends are features;"
    assert t.count(a1) == 1, 'top anchor linux'
    t = t.replace(a1, head + '\n' + '\n'.join(line(c) for c in notwasi) + '\n\n' + a1)
    a2 = "[target.'cfg(all(target_arch = \"wasm32\", target_os = \"unknown\"))'.dependencies]"
    assert t.count(a2) == 1, 'top anchor browser'
    t = t.replace(a2, head + '\n' + '\n'.join(line(c) for c in native) + '\n\n' + a2)
    a3 = '\n[dev-dependencies]\nsemio-framework-job = { workspace = true, features = ["testkit"] }'
    assert t.count(a3) == 1, 'top anchor dev'
    t = t.replace(a3, '\n' + head + '\n' + '\n'.join(line(c) for c in browser) + '\n' + a3)
    a4 = '[dependencies]\nsemio-framework-job-worker = '
    assert t.count(a4) == 1, 'top anchor plain'
    t = t.replace(a4, '[dependencies]\n' + '\n'.join(line(c) for c in plain) + '\nsemio-framework-job-worker = ')
    if write: open(TOP_TOML, 'w', encoding='utf-8').write(t)
    return f'top: +{len(todo)} dependency lines ({len(notwasi)} not-wasi, {len(native)} native, {len(browser)} browser, {len(plain)} everywhere)'

def main():
    mode = sys.argv[1]
    which = set(sys.argv[2:]) or {'members', 'owners', 'launch', 'top'}
    crates = C.build()
    write = mode == 'write'
    for name, fn in (('members', reg_members), ('owners', reg_owners), ('launch', reg_launch), ('top', reg_top)):
        if name in which: print(fn(crates, write))

if __name__ == '__main__': main()
