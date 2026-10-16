# 🗂️ Registers the skeleton crates of the OS kernel partition: root workspace members (byte-sorted), owner rows (lexical) and six launch routes each in both launch files.
# usage: gen_registry.py dry|write [members|owners|launch]...   (idempotent; every edit re-reads its file right before writing and asserts unique anchors)
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okcrates as C

ROOT_TOML = C.R + '/Cargo.toml'
OWNERS = C.R + '/📋️build-owners.json'
LAUNCH = [C.R + '/.vscode/launch.json', C.R + '/.vscode/🧩️launch.seed.jsonc']
ANCHOR = '"name": "🦀️ semio-framework-os-renderer-wgpu-shell-tests-docs-3 🧪️test 🚀️release"'

def b(s): return s.encode('utf-8')

def reg_members(crates, write, keys):
    t = open(ROOT_TOML, encoding='utf-8').read()
    i = t.index('members = [\n'); j = t.index('\n]', i)
    head = t[:i + len('members = [\n')]; body = t[i + len('members = [\n'):j]; tail = t[j:]
    old = [l.strip().rstrip(',').strip('"') for l in body.split('\n') if l.strip()]
    assert [b(x) for x in old] == sorted(b(x) for x in old), 'members not byte-sorted before edit'
    new = [crates[k]['pkg'] for k in keys]
    todo = [p for p in new if p not in old]
    if not todo: return 'members: all present'
    merged = sorted(old + todo, key=b)
    out = head + '\n'.join('    "' + p + '",' for p in merged) + tail
    if write: open(ROOT_TOML, 'w', encoding='utf-8').write(out)
    return f'members: +{len(todo)} ({len(old)} -> {len(merged)})'

def owner_row(c):
    return f'''    {{
      "name": "{c['project']}",
      "root": "{c['pkg']}",
      "manifest": "{c['pkg']}/📋️project.json",
      "packages": []
    }},
'''

def reg_owners(crates, write, keys):
    t = open(OWNERS, encoding='utf-8').read()
    j = json.loads(t)
    names = [r['name'] for r in j['owners']]
    assert names == sorted(names, key=b), 'owners not sorted before edit'
    todo = sorted([crates[k] for k in keys if crates[k]['project'] not in names], key=lambda c: b(c['project']))
    if not todo: return 'owners: all present'
    for c in todo:
        after = [n for n in names if b(n) > b(c['project'])]
        anchor = '    {\n      "name": "' + after[0] + '",'
        assert t.count(anchor) == 1, ('owner anchor', after[0])
        t = t.replace(anchor, owner_row(c) + anchor)
        names.append(c['project']); names.sort(key=b)
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

def reg_launch(crates, write, keys):
    texts = {f: open(f, encoding='utf-8').read() for f in LAUNCH}
    todo = [crates[k] for k in keys if not all(f'"name": "🦀️ {crates[k]["name"]} 🏗️build 🛠️prototype"' in t for t in texts.values())]
    if not todo: return 'launch: all present'
    used = used_orders(texts.values())
    n = 2 * len(keys)
    base = {'build': window(used, 900.2000, n), 'check': window(used, 922.1000, n), 'test': window(used, 903.8000, n)}
    position = {k: i for i, k in enumerate(keys)}
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
        if f.endswith('.json'): json.loads(t2)
        if write: open(f, 'w', encoding='utf-8').write(t2)
    return f'launch: +{len(todo)} crates x 6 routes x 2 files; bands build {base["build"]/1e5} check {base["check"]/1e5} test {base["test"]/1e5} ({n} keys each)'

def main():
    mode = sys.argv[1]
    which = set(sys.argv[2:]) or {'members', 'owners', 'launch'}
    crates, model, cargo = C.build()
    keys = C.ALL_KEYS
    write = mode == 'write'
    for name, fn in (('members', reg_members), ('owners', reg_owners), ('launch', reg_launch)):
        if name in which: print(fn(crates, write, keys))

if __name__ == '__main__': main()
