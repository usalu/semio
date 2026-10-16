import os, re, json, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cfg import *
from cargo import pkg_dir
def add_workspace_members(path, members):
    t = open(path, encoding='utf-8').read()
    i = t.index('members = ['); j = t.index(']\n', i)
    block = t[i:j]
    lines = block.split('\n')
    existing = re.findall(r'"([^"]+)"', block)
    for m in sorted(members, key=lambda x: x.encode()):
        if m in existing: continue
        # find first existing entry greater than m
        pos = None
        for k, ln in enumerate(lines):
            mm = re.match(r'^\s*"([^"]+)",?\s*$', ln)
            if mm and mm.group(1).encode() > m.encode(): pos = k; break
        new = f'    "{m}",'
        if pos is None: lines.append(new)
        else: lines.insert(pos, new)
        existing.append(m)
    t = t[:i] + '\n'.join(lines) + t[j:]
    open(path, 'w', encoding='utf-8').write(t)
def add_owner_rows(path, rows):
    t = open(path, encoding='utf-8').read()
    d = json.loads(t)
    have = {o['name'] for o in d['owners']}
    # textual insertion before the first owner row whose name sorts after the new name
    lines = t.split('\n')
    for row in sorted(rows, key=lambda r: r['name']):
        if row['name'] in have: continue
        idx = None
        for k, ln in enumerate(lines):
            mm = re.match(r'^      "name": "([^"]+)",$', ln)
            if mm and mm.group(1) > row['name'] and lines[k-1].strip() == '{' and lines[k-2].strip() in ('[', '},') and idx is None:
                idx = k - 1
                break
        obj = ['    {', f'      "name": {json.dumps(row["name"], ensure_ascii=False)},', f'      "root": {json.dumps(row["root"], ensure_ascii=False)},', f'      "manifest": {json.dumps(row["manifest"], ensure_ascii=False)},', '      "packages": []', '    },']
        if idx is None: raise SystemExit('no anchor for owner row ' + row['name'])
        lines[idx:idx] = obj
        have.add(row['name'])
    open(path, 'w', encoding='utf-8').write('\n'.join(lines))
    json.loads('\n'.join(lines))
ROUTES = [('🏗️build', '🛠️prototype', 'build', 'prototype', '4_build', 900.0), ('🏗️build', '🚀️release', 'build', 'release', '4_build', 900.0),
          ('🔎️check', '🛠️prototype', 'check', 'prototype', '4_gate', 922.0), ('🔎️check', '🚀️release', 'check', 'release', '4_gate', 922.0),
          ('🧪️test', '🛠️prototype', 'test', 'prototype', '4_test', 903.0), ('🧪️test', '🚀️release', 'test', 'release', '4_test', 903.0)]
def used_orders(text):
    return {m.group(1) for m in re.finditer(r'"order":\s*([0-9.]+)', text)}
def route_objects(clusters, text, start_index=0):
    used = used_orders(text)
    objs = []
    n = 0
    for c in clusters:
        pkg = CLUSTERS[c]['pkg']; pid = f'{c}'
        for k, (verb, mode, target, conf, group, base) in enumerate(ROUTES):
            # unique order key in the 0.0X band: base + 0.015xx / 0.045xx / 0.585xx
            band = {'4_build': 0.01500, '4_gate': 0.04500, '4_test': 0.58500}[group]
            while True:
                n += 1
                val = round(base + band + 0.00001 * (start_index + n), 5) if group != '4_test' else round(base + 0.58500 + 0.00001 * (start_index + n), 5)
                s = repr(val)
                if s not in used and str(val) not in used: break
            used.add(str(val))
            objs.append('    {\n' + f'      "name": "🦀️ {pkg} {verb} {mode}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/{pid}-rs:{target} --configuration={conf} --parallel=1 -- --lib --offline",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {val}\n      }}\n' + '    },')
    return objs
def add_launch_routes(path, clusters, anchor_name_prefix='🦀️ semio-framework-pets-'):
    t = open(path, encoding='utf-8').read()
    if all(f'"name": "🦀️ {CLUSTERS[c]["pkg"]} 🏗️build 🛠️prototype"' in t for c in clusters): return
    lines = t.split('\n')
    last = None
    for k, ln in enumerate(lines):
        if ln.strip().startswith('"name": "' + anchor_name_prefix): last = k
    if last is None: raise SystemExit('no launch anchor')
    k = last
    while lines[k].rstrip() != '    },': k += 1
    new = [c for c in clusters if f'"name": "🦀️ {CLUSTERS[c]["pkg"]} 🏗️build 🛠️prototype"' not in t]
    objs = route_objects(new, t)
    lines[k+1:k+1] = '\n'.join(objs).split('\n')
    open(path, 'w', encoding='utf-8').write('\n'.join(lines))
