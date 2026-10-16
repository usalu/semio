"""✅ Registry checks of the WP8 crates: manifests, workspace members, owner rows, launch routes (both files), folder emoji, path lengths, dependency direction."""
import json, os, re, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, common as C
FE = '️'
R = L.R
A_PKG = R + '/' + L.S_REL + '/🧪' + FE + 'testing/📦' + FE + 'packages/🦀' + FE + 'rust'
B_PKG = R + '/' + L.S_REL + '/🧪' + FE + 'testing/🎭' + FE + 'doubles/📦' + FE + 'packages/🦀' + FE + 'rust'
crates = [('os-store-testing-fixtures', A_PKG), ('os-store-testing-doubles', B_PKG)] + [(name, C.pkg_abs(bk)) for bk, (name, node) in C.BINS.items()]
fails = []
def check(cond, msg):
    if not cond: fails.append(msg)
script = 'import {readFileSync} from "node:fs"; const out={}; for (const [k,p] of Object.entries(JSON.parse(process.argv[1]))) out[k]=Bun.TOML.parse(readFileSync(p,"utf8")); console.log(JSON.stringify(out))'
man = json.loads(subprocess.run(['bun', '-e', script, json.dumps({n: p + '/Cargo.toml' for n, p in crates})], capture_output=True, text=True).stdout)
name2pkg = {'semio-framework-' + n: p for n, p in crates}
edges = collections.defaultdict(set)
for n, pkg in crates:
    m = man[n]
    check(m['package']['name'] == 'semio-framework-' + n, n + ': name')
    check(m['package']['metadata']['semio']['id'] == n and m['package']['metadata']['semio']['role'] == 'framework', n + ': id/role')
    check(m['package']['metadata']['semio']['rust-source-boundary'] == {'maxSourceBytes': 262144, 'maxSourceFiles': 32, 'maxActiveLines': 4000}, n + ': boundary keys')
    check(os.path.realpath(os.path.join(pkg, m['package']['workspace'])) == os.path.realpath(R), n + ': workspace root')
    node = os.path.normpath(os.path.join(pkg, m['lib']['path']))[:-len('/' + '🦀' + FE + '.rs')]
    check(os.path.exists(node + '/🦀' + FE + '.rs') and os.path.dirname(os.path.dirname(pkg)) == node, n + ': lib root is the node root')
    check(m['lints'] == {'workspace': True}, n + ': lints')
    check(m['package'].get('publish') is False, n + ': publish false')
    pj = json.load(open(pkg + '/📋' + FE + 'project.json'))
    check(pj['name'] == '@semio-tech/' + n + '-rs' and pj['tags'] == ['scope:framework', 'language:rust'], n + ': project.json')
    check(len(os.path.relpath(pkg, R).encode()) + len(('/📋' + FE + 'project.json').encode()) <= 240, n + ': path length')
    tables = [m.get('dependencies', {}), m.get('dev-dependencies', {})] + [t.get('dependencies', {}) for t in m.get('target', {}).values()]
    for tb in tables:
        for dk, spec in tb.items():
            if isinstance(spec, dict) and 'path' in spec:
                d = os.path.normpath(os.path.join(pkg, spec['path']))
                check(os.path.exists(d + '/Cargo.toml'), n + ': path dependency %s missing %s' % (dk, d))
                if dk in name2pkg:
                    edges[dk].add('semio-framework-' + n)
                    check(os.path.realpath(d) == os.path.realpath(name2pkg[dk]), n + ': edge %s points elsewhere' % dk)
t = open(R + '/Cargo.toml', encoding='utf-8').read()
i = t.index('members = [\n'); j = t.index('\n]', i)
members = [l.strip().rstrip(',').strip('"') for l in t[i + len('members = [\n'):j].split('\n') if l.strip()]
bad = [(a, b) for a, b in zip(members, members[1:]) if a.encode() >= b.encode()]
check(not bad and len(set(members)) == len(members), 'members byte-sorted and unique %s' % (bad[:3],))
for n, pkg in crates: check(os.path.relpath(pkg, R) in members, n + ': workspace member')
owners = json.load(open(R + '/📋' + FE + 'build-owners.json'))['owners']
oname = {o['name']: o for o in owners}
for n, pkg in crates:
    o = oname.get('@semio-tech/' + n + '-rs')
    rel = os.path.relpath(pkg, R)
    check(o is not None and o['root'] == rel and o['manifest'] == rel + '/📋' + FE + 'project.json' and o['packages'] == [], n + ': owner row')
names = [o['name'] for o in owners]; check(names == sorted(names, key=lambda s: s.encode()), 'owners sorted')
for f in ('.vscode/launch.json', '.vscode/🧩' + FE + 'launch.seed.jsonc'):
    text = open(R + '/' + f, encoding='utf-8').read()
    j = json.loads(text) if f.endswith('.json') else json.loads(subprocess.run(['bun', '-e', 'import {readFileSync} from "node:fs"; console.log(JSON.stringify(Bun.JSONC.parse(readFileSync(process.argv[1],"utf8"))))', R + '/' + f], capture_output=True, text=True).stdout)
    confs = [c for c in j['configurations'] if isinstance(c, dict)]
    cmds = collections.Counter(c.get('command') for c in confs)
    for n, pkg in crates:
        for tgt in ('build', 'check', 'test'):
            for mode in ('prototype', 'release'):
                cmd = 'bun repo run @semio-tech/%s-rs:%s --configuration=%s --parallel=1 -- --lib --offline' % (n, tgt, mode)
                check(cmds[cmd] == 1, '%s: %s %s %s routes=%d' % (f, n, tgt, mode, cmds[cmd]))
byparent = collections.defaultdict(list)
for n, pkg in crates:
    node = os.path.dirname(os.path.dirname(pkg))
    byparent[os.path.dirname(node)].append(os.path.basename(node))
for parent, kids in byparent.items():
    emo = [k.split(FE)[0] for k in os.listdir(parent) if FE in k and os.path.isdir(parent + '/' + k)]
    for k in kids:
        e = k.split(FE)[0]
        check(emo.count(e) == 1 or k == '🧪' + FE + 'testing', 'folder %s repeats a sibling emoji under %s' % (k, parent))
        check(FE in k, 'folder %s has no FE0F' % k)
# dependency direction among the WP8 crates: doubles depend on fixtures only; bins on both; fixtures on neither
check('semio-framework-os-store-testing-doubles' not in man['os-store-testing-fixtures'].get('dependencies', {}), 'fixtures must not depend on doubles')
check('semio-framework-os-kernel' not in {d for d in (man['os-store-testing-fixtures'].get('dependencies', {}))}, 'fixtures depend on the kernel facade')
check('semio-framework-os-kernel' not in man['os-store-testing-doubles'].get('dependencies', {}), 'doubles depend on the kernel facade')
print('crates', len(crates), 'failures', len(fails))
for f_ in fails[:40]: print('  FAIL', f_)
print({k.replace('semio-framework-', ''): sorted(x.replace('semio-framework-', '') for x in v)[:3] for k, v in edges.items()})
sys.exit(1 if fails else 0)
