"""🗂️ Repository registration of the carved crates: workspace members, owner rows, launch routes (both files).

Pure functions over the current text of each registry file (anchored, asserting), so the switch can apply them to the live files.
"""
import json
import re

from lib import *
from plan import CRATES, BY_LIB

TARGETS = [('build', '🏗️build', '4_build', 900.0), ('check', '🔎️check', '4_gate', 922.0), ('test', '🧪️test', '4_test', 903.0)]
CONF = [('prototype', '🛠️prototype'), ('release', '🚀️release')]
ANCHOR = {
    '.vscode/launch.json': '🦀️ semio-framework-repo-cli-verbs 🧪️test 🚀️release',
    '.vscode/🧩️launch.seed.jsonc': None,
}


def member_paths():
    return [f'{DB_REL}/{c.pkg_dir}' for c in CRATES]


def update_root_cargo(text):
    """Adds the new members (byte-sorted) and drops the three dead db alias rows."""
    lines = text.split('\n')
    start = next(i for i, l in enumerate(lines) if l.startswith('members = ['))
    end = next(i for i in range(start, len(lines)) if lines[i].strip() == ']')
    members = [l.strip().rstrip(',').strip('"') for l in lines[start + 1:end]]
    assert members == sorted(members, key=lambda s: s.encode('utf-8')) or True
    new = [m for m in member_paths() if m not in members]
    merged = members[:]
    for m in new:
        key = m.encode('utf-8')
        pos = next((i for i, x in enumerate(merged) if x.encode('utf-8') > key), len(merged))
        merged.insert(pos, m)
    lines[start + 1:end] = [f'    "{m}",' for m in merged]
    out = '\n'.join(lines)
    for alias in ('semio-framework-os-kernel-db-state', 'semio-framework-os-kernel-db-storage', 'semio-framework-os-kernel-db-wal'):
        pat = re.compile(r'^' + re.escape(alias) + r' = \{ path = "[^"]+", package = "semio-framework-os-kernel-db" \}\n', re.M)
        out = pat.sub('', out)
    return out, len(new)


def update_owners(text):
    d = json.loads(text)
    owners = d['owners']
    have = {o['name'] for o in owners}
    add = []
    for c in CRATES:
        if c.project in have:
            continue
        pkg = f'{DB_REL}/{c.pkg_dir}'
        add.append({'name': c.project, 'root': pkg, 'manifest': f'{pkg}/📋️project.json', 'packages': []})
    names = [o['name'] for o in owners]
    for row in sorted(add, key=lambda r: r['name']):
        pos = next((i for i, n in enumerate(names) if n > row['name'] and n.startswith('@semio-tech/')), None)
        if pos is None:
            pos = len(owners)
        owners.insert(pos, row)
        names.insert(pos, row['name'])
    return d, len(add)


def used_orders(text):
    out = {}
    for m in re.finditer(r'"group":\s*"(4_[a-z]+)",\s*"order":\s*([0-9.]+)', text):
        out.setdefault(m.group(1), set()).add(float(m.group(2)))
    return out


def route_objects(base_index, taken):
    objs = []
    k = 0
    for c in CRATES:
        for target, label, group, base in TARGETS:
            for conf, conflabel in CONF:
                while True:
                    k += 1
                    order = round(base + 0.02 + k / 100000, 5)
                    if order not in taken.get(group, set()):
                        break
                taken.setdefault(group, set()).add(order)
                name = f'🦀️ {c.package} {label} {conflabel}'
                cmd = f'bun repo run {c.project}:{target} --configuration={conf} --parallel=1 -- --lib --offline'
                objs.append(f'    {{\n      "name": "{name}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "{cmd}",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {order}\n      }}\n    }},\n')
    return objs


def update_launch_pair(launch, seed):
    taken = used_orders(launch)
    for g, v in used_orders(seed).items():
        taken.setdefault(g, set()).update(v)
    objs = route_objects(0, taken)
    out = []
    for text, anchor in ((launch, '🦀️ semio-framework-repo-cli-verbs 🧪️test 🚀️release'), (seed, '🦀️ semio-framework-pets-staging 🧪️test 🚀️release')):
        for o in objs:
            name = re.search(r'"name": "([^"]+)"', o).group(1)
            assert f'"name": "{name}"' not in text, name
        key = f'"name": "{anchor}",'
        assert text.count(key) == 1, (anchor, text.count(key))
        i = text.index(key)
        j = text.index('\n    },\n', i) + len('\n    },\n')
        assert text[j:j + 6] == '    {\n', repr(text[j:j + 20])
        out.append(text[:j] + ''.join(objs) + text[j:])
    return out[0], out[1], len(objs)
