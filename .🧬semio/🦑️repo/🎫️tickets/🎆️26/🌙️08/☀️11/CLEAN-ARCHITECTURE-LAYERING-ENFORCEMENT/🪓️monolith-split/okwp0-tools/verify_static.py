# ✅ Static checks of the OS kernel partition registries: manifests, DAG, workspace members, owner rows, launch routes (both files), folder emoji, path lengths.
import json, os, re, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import okcrates as C
crates, model, cargo = C.build()
R = C.R
fails = []
def check(cond, msg):
    if not cond: fails.append(msg)
tomls = {k: C.pkg_abs(k, model) + '/Cargo.toml' for k in crates}
script = 'import {readFileSync} from "node:fs"; const out={}; for (const [k,p] of Object.entries(JSON.parse(process.argv[1]))) out[k]=Bun.TOML.parse(readFileSync(p,"utf8")); console.log(JSON.stringify(out))'
man = json.loads(subprocess.run(['bun', '-e', script, json.dumps(tomls)], capture_output=True, text=True).stdout)
check(len(man) == len(crates), 'manifest parse count')
name2key = {c['name']: k for k, c in crates.items()}
rank = {k: i for i, k in enumerate(C.ALL_KEYS)}
edges = 0
for k, c in crates.items():
    m = man[k]; pkg = C.pkg_abs(k, model)
    check(m['package']['name'] == c['name'], f'{k}: name')
    check(m['package']['metadata']['semio']['id'] == c['id'] and m['package']['metadata']['semio']['role'] == 'framework', f'{k}: id/role')
    b = m['package']['metadata']['semio']['rust-source-boundary']
    check(b == {'maxSourceBytes': 262144, 'maxSourceFiles': 32, 'maxActiveLines': 4000}, f'{k}: boundary keys')
    check(os.path.realpath(os.path.join(pkg, m['package']['workspace'])) == os.path.realpath(R), f'{k}: workspace resolves to the repository root')
    root = os.path.normpath(os.path.join(pkg, m['lib']['path']))
    check(root == C.node_abs(k, model) + '/🦀️.rs' and os.path.exists(root), f'{k}: lib path')
    check(m['lints'] == {'workspace': True}, f'{k}: lints')
    tables = [m.get('dependencies', {})] + [t.get('dependencies', {}) for t in m.get('target', {}).values()]
    for tb in tables:
        for dk, spec in tb.items():
            if isinstance(spec, dict) and 'path' in spec:
                d = os.path.normpath(os.path.join(pkg, spec['path']))
                check(os.path.exists(d + '/Cargo.toml'), f'{k}: path dependency {dk} missing {d}')
                if dk in name2key:
                    edges += 1
                    check(rank[name2key[dk]] < rank[k], f'{k}: upward edge to {dk}')
                    check(os.path.realpath(d) == os.path.realpath(C.pkg_abs(name2key[dk], model)), f'{k}: edge {dk} points elsewhere')
    for sec in ('dependencies', 'dev-dependencies'):
        for dk in m.get(sec, {}):
            pass
    pj = json.load(open(pkg + '/📋️project.json'))
    check(pj['name'] == c['project'] and pj['tags'] == ['scope:framework', 'language:rust'], f'{k}: project.json')
    check(len(os.path.relpath(pkg, R).encode()) + len('/📋️project.json'.encode()) <= 240, f'{k}: path length')
print('manifests', len(man), 'first-party edges', edges)
# DAG acyclic by construction (strictly decreasing rank); members
t = open(R + '/Cargo.toml', encoding='utf-8').read()
i = t.index('members = [\n'); j = t.index('\n]', i)
members = [l.strip().rstrip(',').strip('"') for l in t[i + len('members = [\n'):j].split('\n') if l.strip()]
check(members == sorted(members, key=lambda s: s.encode()) and len(set(members)) == len(members), 'members byte-sorted and unique')
for k, c in crates.items(): check(c['pkg'] in members, f'{k}: workspace member')
owners = json.load(open(R + '/📋️build-owners.json'))['owners']
oname = {o['name']: o for o in owners}
for k, c in crates.items():
    o = oname.get(c['project'])
    check(o is not None and o['root'] == c['pkg'] and o['manifest'] == c['pkg'] + '/📋️project.json' and o['packages'] == [], f'{k}: owner row')
names = [o['name'] for o in owners]; check(names == sorted(names, key=lambda s: s.encode()), 'owners sorted')
# launch routes
for f in ('.vscode/launch.json', '.vscode/🧩️launch.seed.jsonc'):
    text = open(R + '/' + f, encoding='utf-8').read()
    j = json.loads(text) if f.endswith('.json') else json.loads(subprocess.run(['bun', '-e', 'import {readFileSync} from "node:fs"; console.log(JSON.stringify(Bun.JSONC ? Bun.JSONC.parse(readFileSync(process.argv[1],"utf8")) : JSON.parse(readFileSync(process.argv[1],"utf8"))))', R + '/' + f], capture_output=True, text=True).stdout)
    confs = [c for c in j['configurations'] if isinstance(c, dict)]
    cmds = collections.Counter(c.get('command') for c in confs)
    orders = collections.Counter((c['presentation']['group'], c['presentation']['order']) for c in confs if 'presentation' in c)
    for k, c in crates.items():
        for tgt in ('build', 'check', 'test'):
            for mode in ('prototype', 'release'):
                cmd = f'bun repo run {c["project"]}:{tgt} --configuration={mode} --parallel=1 -- --lib --offline'
                check(cmds[cmd] == 1, f'{f}: {k} {tgt} {mode} routes={cmds[cmd]}')
    mine = [cf for cf in confs if any(('semio-framework-' + k + ' ') in cf.get('name', '') for k in crates) and cf.get('name', '').startswith('🦀️ ' + C.PREFIX)]
    exact = [cf for cf in confs if re.match(r'🦀️ ' + re.escape(C.PREFIX) + '(' + '|'.join(re.escape(k) for k in crates) + r') (🏗️build|🔎️check|🧪️test) ', cf.get('name', ''))]
    check(len(exact) == 6 * len(crates), f'{f}: route count {len(exact)}')
    dup = [o for o, n in orders.items() if n > 1 and any(cf['presentation'] == {'group': o[0], 'order': o[1]} for cf in exact)]
    check(not dup, f'{f}: duplicate order keys {dup[:3]}')
    print(f, 'configurations', len(confs), 'kernel partition routes', len(exact))
# new folders: emoji unique among siblings, RGI-style variation selector
fe0f = '️'
byparent = collections.defaultdict(list)
for k, c in crates.items():
    node = R + '/' + c['node']
    byparent[os.path.dirname(node)].append(os.path.basename(node))
def created(path):
    rf = path + '/🦀️.rs'
    return os.path.exists(rf) and 'Skeleton of `semio-framework-os-kernel`' in open(rf, encoding='utf-8').read(400)
for parent, kids in byparent.items():
    new = [n for n in kids if created(parent + '/' + n)]
    others = [n for n in os.listdir(parent) if n not in new and fe0f in n]
    for n in new:
        e = n.split(fe0f)[0]
        check(not any(o.split(fe0f)[0] == e for o in others), f'new folder {n} repeats a sibling emoji under {parent}')
        check(sum(1 for m in new if m.split(fe0f)[0] == e) == 1, f'new folder {n} repeats a new sibling emoji')
for k, c in crates.items():
    base = os.path.basename(c['node'])
    check(fe0f in base, f'{k}: folder name has emoji + FE0F')
print('failures', len(fails))
for f in fails[:40]: print('  FAIL', f)
sys.exit(1 if fails else 0)
