# 📋️ Adds the downward first-party edges the S0 layout evidences to the manifests of the WP4 crates (rank-ordered among the plugin edges).
import json, os, re, sys
from common import *
CR = json.load(open(GEN + '/pwp0-crates.json'))
ADD = {'reactor-lifecycle': ['reactor-tasks'], 'runtime-instances': ['reactor-tasks'], 'reactor-surfaces': ['reactor-tasks'], 'reactor-core': ['reactor-patches'], 'reactor-wit-bridge': ['reactor-core']}
bykey = {v['name']: k for k, v in CR.items()}
for crate, adds in ADD.items():
    c = CR[crate]; mp = ROOT + c['pkg'] + '/Cargo.toml' if not c['pkg'].startswith('/') else c['pkg'] + '/Cargo.toml'
    mp = ROOT + '/' + c['pkg'] + '/Cargo.toml'
    t = open(mp, encoding='utf-8').read()
    lines = t.split('\n')
    for a in adds:
        d = CR[a]; name = d['name']
        if re.search(r'^' + re.escape(name) + r'\s*=', t, re.M): print(crate, 'already has', a); continue
        rel = os.path.relpath(ROOT + '/' + d['pkg'], ROOT + '/' + c['pkg'])
        line = f'{name} = {{ path = "{rel}" }}'
        sec = lines.index('[dependencies]')
        idx = None; last_plugin = None
        for i in range(sec + 1, len(lines)):
            if lines[i].startswith('['): break
            m = re.match(r'^(semio-framework-plugin-[\w-]+)\s*=', lines[i])
            if m and m.group(1) in bykey:
                last_plugin = i
                if idx is None and CR[bykey[m.group(1)]]['rank'] > d['rank']: idx = i
        if idx is None: idx = (last_plugin + 1) if last_plugin is not None else sec + 1
        lines.insert(idx, line); t = '\n'.join(lines)
        print(crate, '+', a, '->', rel)
    open(mp, 'w', encoding='utf-8').write('\n'.join(lines))
