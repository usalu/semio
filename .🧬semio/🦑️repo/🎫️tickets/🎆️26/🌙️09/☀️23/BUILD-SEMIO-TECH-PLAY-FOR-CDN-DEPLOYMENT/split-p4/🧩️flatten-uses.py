#!/usr/bin/env python3
"""🧩️ Flattens nested `use crate::..::{a::{b, c}, d}` groups into one `use` per leading module path.

Each nested group becomes one `use <prefix>::<module>::{..};` statement per distinct module, so a path-keyed
rewrite (🧭️canonical-paths.py) can retarget every leaf. Attributes directly above the statement (`#[cfg(test)]`)
and its visibility are repeated on every produced statement; indentation is kept.

usage: 🧩️flatten-uses.py <file.rs>...
"""
import re, sys, collections

def split_top(text):
    parts, depth, cur = [], 0, ''
    for ch in text:
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    return [p.strip() for p in parts if p.strip()]

def leaves(prefix, body):
    body = body.strip()
    m = re.match(r'^((?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*)\{(.*)\}$', body, re.S)
    if m:
        pre = prefix + [x for x in re.split(r'\s*::\s*', m.group(1)) if x]
        out = []
        for part in split_top(m.group(2)): out += leaves(pre, part)
        return out
    segs = [x for x in re.split(r'\s*::\s*', body) if x]
    return [(prefix + segs[:-1], segs[-1])]

STMT = re.compile(r'(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+(?P<path>crate(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)*)\s*::\s*\{(?P<body>[^;]*)\}\s*;')

def flatten(text):
    def rep(m):
        body = m.group('body')
        if '{' not in body: return m.group(0)
        prefix = [x for x in re.split(r'\s*::\s*', m.group('path')) if x]
        groups = collections.OrderedDict()
        for mods, leaf in leaves(prefix, '{' + body + '}'):
            groups.setdefault('::'.join(mods), []).append(leaf)
        attrs, indent, vis = m.group('attrs'), m.group('indent'), m.group('vis') or ''
        stmts = []
        for path, items in groups.items():
            items = list(dict.fromkeys(items))
            tail = items[0] if len(items) == 1 and items[0] not in ('self',) and ' as ' not in items[0] or len(items) == 1 and ' as ' in items[0] else '{' + ', '.join(items) + '}'
            stmts.append(f'{attrs}{indent}{vis}use {path}::{tail};')
        return '\n'.join(stmts)
    return STMT.sub(rep, text)

for f in sys.argv[1:]:
    s = open(f, encoding='utf-8').read(); t = flatten(s)
    if t != s: open(f, 'w', encoding='utf-8').write(t); print('flattened', f)
