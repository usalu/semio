#!/usr/bin/env python3
"""Rewrites `use super::{a, b, …};` of a unit-test file so that names the host merely IMPORTS (a private `use` of the host) are imported from their original path, and only names the host DEFINES stay under `super::`.
usage: fixsuper.py <test.rs> [--host=<host.rs>] [--dry]"""
import re, sys, os

def read(p): return open(p, encoding='utf-8').read()

def expand(path):
    path = path.strip()
    m = re.match(r'^(.*?)::\{(.*)\}$', path, re.S)
    if not m: return [path]
    base, inner = m.group(1), m.group(2)
    parts, depth, cur = [], 0, ''
    for ch in inner:
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    out = []
    for part in parts:
        part = part.strip()
        if not part: continue
        for sub in expand(part):
            out.append(base if sub == 'self' else base + '::' + sub)
    return out

def leaf(full):
    a = re.search(r'\bas\s+(\w+)$', full)
    return a.group(1) if a else full.split('::')[-1]

def host_imports(text):
    m = {}
    for s in re.finditer(r'^(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', text, re.M):
        for p in expand(s.group(1)):
            if p.endswith('::*'): continue
            m[leaf(p)] = p
    return m

def host_defs(text):
    names = set(re.findall(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|type|trait|mod|union)\s+(\w+)', text, re.M))
    names |= set(re.findall(r'^macro_rules!\s+(\w+)', text, re.M))
    return names


def modpath(host):
    parts = host.split('/')
    i = parts.index('✳️any')
    segs = []
    for part in parts[i + 1:-1]:
        slug = re.sub(r'^[^A-Za-z0-9]+', '', part).replace('-', '_')
        segs.append(slug)
    if segs and segs[0] == 'editor': return ['crate', 'editor', 'bim'] + segs[1:]
    if segs and segs[0] == 'viewer': return ['crate', 'viewer', 'bim'] + segs[1:]
    if segs and segs[0] == 'render': return ['crate', 'render', 'bim'] + segs[1:]
    return ['crate', 'standards', 'v1', 'subsets', 'any'] + segs

def absolute(path, host):
    mp = modpath(host)
    if path.startswith('self::'): return '::'.join(mp + path.split('::')[1:])
    m = re.match(r'^((?:super::)+)(.*)$', path)
    if m:
        depth = m.group(1).count('super::')
        return '::'.join(mp[:len(mp) - depth] + m.group(2).split('::'))
    return path

def main():
    test = sys.argv[1]
    host = next((a[7:] for a in sys.argv if a.startswith('--host=')), None) or os.path.normpath(os.path.join(os.path.dirname(test), '..', '..', '🦀️.rs'))
    dry = '--dry' in sys.argv
    t, h = read(test), read(host)
    imports, defs = host_imports(h), host_defs(h)
    changed = []
    def rewrite(m):
        names = [n.strip() for n in m.group(1).split(',') if n.strip()]
        keep, moved = [], []
        for n in names:
            base = n.split(' as ')[0].strip()
            if base in defs: keep.append(n)
            elif base in imports: moved.append(absolute(imports[base], host) + ('' if n == base else n[len(base):]))
            else: keep.append(n)
        if not moved: return m.group(0)
        changed.append((len(keep), len(moved)))
        out = []
        if keep: out.append('use super::{%s};' % ', '.join(keep))
        # group moved by parent
        groups = {}
        for p in moved:
            parent = '::'.join(p.split('::')[:-1]); groups.setdefault(parent, []).append(p.split('::')[-1])
        for parent, ls in groups.items():
            out.append(f'use {parent}::{{{", ".join(ls)}}};' if len(ls) > 1 else f'use {parent}::{ls[0]};')
        return '\n'.join(out)
    t2 = re.sub(r'^use super::\{([^}]*)\};', rewrite, t, count=1, flags=re.M)
    print(os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(test)))), 'moved/kept', changed)
    if not dry and t2 != t:
        open(test, 'w', encoding='utf-8').write(t2)

main()
