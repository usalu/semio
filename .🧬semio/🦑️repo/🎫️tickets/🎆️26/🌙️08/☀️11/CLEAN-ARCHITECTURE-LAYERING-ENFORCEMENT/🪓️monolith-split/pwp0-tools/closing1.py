# 🧭️ WP0 closing pass 1: applies the frozen-table deltas of WP1/WP3/WP4/WP5 to pwp0-crates.json and to the manifests (idempotent), forwards window-kits-rich's feature in the facade, adds the job -> semio-framework-tool-run edge and retires the empty companions tests-08 / tests-09.
# usage: closing1.py deltas|retire   (deltas: table + manifests + facade; retire: registries and folders of tests-08 and tests-09)
import json, os, re, sys, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C
R = C.R
GENJ = C.GEN + '/pwp0-crates.json'
DELTAS = {
    'actor-host': (['abi'], []),
    'operation': ([], ['interaction']),
    'window-kits': ([], ['operation']),
    'reactor-lifecycle': (['reactor-tasks'], []),
    'runtime-instances': (['reactor-tasks'], []),
    'reactor-surfaces': (['reactor-tasks', 'plugin-program'], []),
    'reactor-core': (['reactor-patches'], []),
    'reactor-wit-bridge': (['reactor-core', 'decl-app'], []),
    'reactor-turn': (['decl-emission', 'app-builder', 'plugin-program'], []),
    'reactor-context': (['decl-emission', 'extension'], []),
    'runtime-ops': (['app-builder'], []),
    'instance-app': (['reactor-jobs'], []),
    'tests-05': (['composition-publication', 'publication-mounted'], []),
}
FEATURE_ADD = {'artifact-app-testing': ['window-kits-rich']}

def load(): return json.load(open(GENJ, encoding='utf-8'))

def pkgdir(c): return R + '/' + c['pkg']

def dep_line(crates, d, frm):
    rel = os.path.relpath(pkgdir(crates[d]), pkgdir(crates[frm]))
    return f'{crates[d]["name"]} = {{ path = "{rel}" }}'

def edit_manifest(crates, k, add, drop):
    p = pkgdir(crates[k]) + '/Cargo.toml'
    t = open(p, encoding='utf-8').read()
    out = []
    changed = False
    for d in drop:
        pat = re.compile(r'^' + re.escape(crates[d]['name']) + r' = \{[^\n]*\}\n', re.M)
        m = pat.search(t[:t.index('[dev-dependencies]')] if '[dev-dependencies]' in t else t)
        if m:
            t = t[:m.start()] + t[m.end():]; changed = True
    for d in add:
        if re.search(r'^' + re.escape(crates[d]['name']) + r' = ', t, re.M): continue
        sec = t.index('[dependencies]')
        end = t.find('\n[', sec + 1); end = len(t) if end < 0 else end + 1
        body = t[sec:end]
        lines = body.split('\n')
        last = max((i for i, l in enumerate(lines) if l.startswith('semio-framework-plugin-')), default=None)
        newl = dep_line(crates, d, k)
        if last is None: lines.insert(1, newl)
        else: lines.insert(last + 1, newl)
        t = t[:sec] + '\n'.join(lines) + t[end:]; changed = True
    if changed: open(p, 'w', encoding='utf-8').write(t)
    return changed

def deltas():
    crates = load()
    order = lambda d: crates[d]['rank']
    for k, (add, drop) in DELTAS.items():
        c = crates[k]
        for d in add:
            assert crates[d]['rank'] < c['rank'], (k, d)
            if d not in c['deps']: c['deps'].append(d)
        c['deps'] = [d for d in c['deps'] if d not in drop]
        c['deps'] = sorted(set(c['deps']), key=lambda d: (d == 'test-support', crates[d]['rank']))
        if k == 'tests-05':
            c['hosts'] = ['document-media', 'publication-mounted']
            c['members'] = 'hosts: document-media, publication-mounted'
            c['desc'] = 'Companion test crate tests-05 of the plugin SDK partition (hosts: document-media, publication-mounted)'
        print('table', k, 'ok')
    for f, ks in FEATURE_ADD.items():
        for k in ks:
            if f not in crates[k]['features']: crates[k]['features'].append(f)
    json.dump(crates, open(GENJ, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    for k, (add, drop) in DELTAS.items():
        print('manifest', k, edit_manifest(crates, k, add, drop))
    jm = pkgdir(crates['job']) + '/Cargo.toml'
    t = open(jm, encoding='utf-8').read()
    if 'semio-framework-tool-run' not in t:
        anchor = 'semio-framework-plugin-ownership = '
        i = t.index(anchor); j = t.index('\n', i) + 1
        t = t[:j] + 'semio-framework-tool-run = { workspace = true }\n' + t[j:]
        open(jm, 'w', encoding='utf-8').write(t); print('job: semio-framework-tool-run added')
    fm = R + '/' + C.PKG_REL + '/Cargo.toml'
    t = open(fm, encoding='utf-8').read()
    fwd = '"semio-framework-plugin-window-kits-rich/artifact-app-testing"'
    m = re.search(r'^artifact-app-testing = \[[^\n]*\]$', t, re.M)
    assert m
    if fwd not in m.group(0):
        line = m.group(0)[:-1] + ', ' + fwd + ']'
        t = t[:m.start()] + line + t[m.end():]
        open(fm, 'w', encoding='utf-8').write(t); print('facade: artifact-app-testing forwards window-kits-rich')
    wk = pkgdir(crates['window-kits-rich']) + '/Cargo.toml'
    assert 'artifact-app-testing' in open(wk, encoding='utf-8').read()

def retire():
    crates = load()
    keys = ['tests-08', 'tests-09']
    names = [crates[k]['name'] for k in keys if k in crates]
    projects = [crates[k]['project'] for k in keys if k in crates]
    pkgs = [crates[k]['pkg'] for k in keys if k in crates]
    dirs = [crates[k]['dir'] for k in keys if k in crates]
    if not names:
        print('already retired'); return
    p = R + '/Cargo.toml'; t = open(p, encoding='utf-8').read()
    for pk in pkgs:
        row = f'    "{pk}",\n'
        assert t.count(row) == 1, pk
        t = t.replace(row, '')
    open(p, 'w', encoding='utf-8').write(t)
    p = R + '/📋️build-owners.json'; t = open(p, encoding='utf-8').read()
    for pr in projects:
        m = re.search(r'    \{\n      "name": "' + re.escape(pr) + r'",\n(?:      [^\n]*\n)*?    \},\n', t)
        assert m, pr
        t = t[:m.start()] + t[m.end():]
    json.loads(t)
    open(p, 'w', encoding='utf-8').write(t)
    for f in ('.vscode/launch.json', '.vscode/🧩️launch.seed.jsonc'):
        p = R + '/' + f; t = open(p, encoding='utf-8').read(); n = 0
        for nm in names:
            pat = re.compile(r'    \{\n      "name": "🦀️ ' + re.escape(nm) + r' [^"]*",\n(?:      [^\n]*\n|        [^\n]*\n|      \}[^\n]*\n)*?    \},\n')
            while True:
                m = pat.search(t)
                if not m: break
                t = t[:m.start()] + t[m.end():]; n += 1
        if f.endswith('.json'): json.loads(t)
        open(p, 'w', encoding='utf-8').write(t); print(f, 'routes removed', n)
    for d in dirs:
        shutil.rmtree(R + '/' + d); print('removed', d)
    for k in keys: crates.pop(k, None)
    json.dump(crates, open(GENJ, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('retired', keys)

if __name__ == '__main__':
    {'deltas': deltas, 'retire': retire}[sys.argv[1]]()
