#!/usr/bin/env python3
"""bd9: split the flow brep mesh lib root (1211 lines) into three path-mounted children, keeping every item byte for byte.
usage: split_flow_mesh.py [--apply]   (cwd = repo root)"""
import os, re, sys
apply = '--apply' in sys.argv
D = '✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh'
ROOT = D + '/🦀️.rs'
text = open(ROOT, encoding='utf-8').read()
lines = text.split('\n')
if lines[-1] == '':
    lines.pop()
assert lines[0].startswith('//! '), lines[0]
use_lines = lines[1:14]
assert all(l.startswith('use ') or l == '' for l in use_lines), use_lines
macro = lines[15:27]
assert macro[0].startswith('#[macro_export]') and macro[-1] == '}', macro[:1]
def seg(a, b):
    return lines[a - 1:b]
helpers = seg(29, 197)
core = seg(198, 1047) + seg(1196, len(lines))
registration = seg(1048, 1195)
assert helpers[0].startswith('pub const LIMIT'), helpers[0]
assert registration[0].startswith('/// '), registration[0]
assert core[1].startswith('pub enum MeshRetirement') or core[0].startswith('pub enum MeshRetirement'), core[:2]
assert registration[-1] == '}' and not lines[1195 - 1].strip() or True
children = [
    ('🧰️helpers', 'helpers', '🧰️ Mesh widget argument readers, mesh text codec, job drivers and grants shared by the operators.', helpers),
    ('🧮️operator-core', 'operator_core', '🧮️ The mesh operator job, its retained state, retirement cursors and the operation implementations.', core),
    ('📇️registration', 'registration', '📇️ Registration of the mesh schema and every polygon operator in the flow registry.', registration),
]
def flatten(stmt):
    m = re.match(r'use (.*);$', stmt.strip())
    body = m.group(1)
    out = []
    def rec(prefix, s):
        s = s.strip()
        if s.startswith('{') :
            inner = s[1:-1]
            depth = 0; cur = ''; parts = []
            for ch in inner:
                if ch == '{': depth += 1
                if ch == '}': depth -= 1
                if ch == ',' and depth == 0:
                    parts.append(cur); cur = ''
                else:
                    cur += ch
            if cur.strip(): parts.append(cur)
            for p in parts: rec(prefix, p)
            return
        if '::{' in s:
            head, rest = s.split('::{', 1)
            rec(prefix + head + '::', '{' + rest)
            return
        out.append(prefix + s.strip())
    rec('', body)
    return out
leaves = []
for l in use_lines:
    if l.strip():
        leaves += flatten(l)
def used_name(leaf):
    if leaf.endswith('::*'):
        return None
    if ' as ' in leaf:
        return leaf.split(' as ')[1].strip()
    return leaf.split('::')[-1]
def idents(code):
    return set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', code))
def pubnames(code_lines):
    names = set()
    for l in code_lines:
        m = re.match(r'pub (?:fn|struct|enum|trait|const|static|type) ([A-Za-z_][A-Za-z0-9_]*)', l)
        if m: names.add(m.group(1))
    return names
defs = {c[1]: pubnames(c[3]) for c in children}
files = {}
for folder, mod, doc, body in children:
    body_text = '\n'.join(body)
    ids = idents(body_text)
    head = [f'//! {doc}', '']
    for leaf in leaves:
        n = used_name(leaf)
        if n is None or n in ids:
            head.append(f'use {leaf};')
    sibling = []
    for other, names in defs.items():
        if other == mod: continue
        used = sorted(n for n in names if n in ids and n not in defs[mod])
        if used:
            sibling.append((other, used))
    allused = sorted({n for _, u in sibling for n in u})
    if allused:
        head.append('use super::{' + ', '.join(allused) + '};')
    head.append('')
    files[folder] = '\n'.join(head + body) + '\n'
root = [lines[0], '']
root += macro
root += ['']
for folder, mod, doc, body in children:
    root += [f'#[path = "{folder}/🦀️.rs"]', f'mod {mod};', f'pub use {mod}::*;']
root_text = '\n'.join(root) + '\n'
print('root lines', len(root), {k: v.count('\n') for k, v in files.items()})
for k, v in files.items():
    print('==', k); print('\n'.join(v.split('\n')[:22])[:1500])
if apply:
    for folder, content in files.items():
        os.makedirs(D + '/' + folder, exist_ok=True)
        open(D + '/' + folder + '/🦀️.rs', 'w', encoding='utf-8').write(content)
    open(ROOT + '.bd9tmp', 'w', encoding='utf-8').write(root_text)
    os.replace(ROOT + '.bd9tmp', ROOT)
