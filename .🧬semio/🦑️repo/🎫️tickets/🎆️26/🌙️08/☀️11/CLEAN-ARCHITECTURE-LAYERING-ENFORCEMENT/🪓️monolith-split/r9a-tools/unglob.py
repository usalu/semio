#!/usr/bin/env python3
"""Replaces `use super::*;` of a unit-test file by the explicit list of the host items it uses, so the planner can detach the test (its `stuck` rule refuses globs).
usage: unglob.py <test.rs> [--dry]   host = <test dir>/../../🦀️.rs (the file whose `#[path = "🧪️tests/…"] mod tests;` mounts it)"""
import re, sys, os

def read(p): return open(p, encoding='utf-8').read()

def leaves(stmt):
    stmt = stmt.strip()
    m = re.match(r'^(.*?)::\{(.*)\}$', stmt, re.S)
    if not m:
        l = stmt.split('::')[-1]
        a = re.search(r'\bas\s+(\w+)$', stmt)
        return [a.group(1) if a else l]
    parts, depth, cur = [], 0, ''
    for ch in m.group(2):
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    out = []
    for p in parts:
        p = p.strip()
        if not p: continue
        if p == 'self': out.append(m.group(1).split('::')[-1]); continue
        out += leaves(p)
    return out

def top_level_names(text):
    names = set()
    for m in re.finditer(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|type|trait|mod|union)\s+(\w+)', text, re.M):
        names.add(m.group(1))
    for m in re.finditer(r'^(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', text, re.M):
        if m.group(1).strip().endswith('*'): continue
        names.update(x for x in leaves(m.group(1)) if x != '_' and x != '*')
    return names

def main():
    test = sys.argv[1]
    dry = '--dry' in sys.argv
    host = next((a[7:] for a in sys.argv if a.startswith('--host=')), None) or os.path.normpath(os.path.join(os.path.dirname(test), '..', '..', '🦀️.rs'))
    t, h = read(test), read(host)
    if not re.search(r'^use super::\*;\s*$', t, re.M):
        print('no glob', test); return
    declared = top_level_names(t)
    imported = set()
    for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', t, re.M):
        if m.group(1).strip() == 'super::*': continue
        imported.update(leaves(m.group(1)))
    declared |= set(re.findall(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|const|static|type|trait|mod)\s+(\w+)', t, re.M))
    body = re.sub(r'^use super::\*;\s*$', '', t, flags=re.M)
    idents = set(re.findall(r'[A-Za-z_]\w*', re.sub(r'//[^\n]*', '', body)))
    host_names = top_level_names(h)
    stripped = re.sub(r'//[^\n]*', '', body)
    def used(name):
        if name[0].isupper():
            return re.search(r'(?<![.\w])(?<!::)' + name + r'\b', stripped) is not None
        return re.search(r'(?<![.\w])(?<!::)' + name + r'\s*(?:\(|::|!)', stripped) is not None
    mods = set(re.findall(r'^(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', h, re.M)) & {'tests'}
    chosen = sorted(n for n in (host_names & idents) - declared - imported - mods - {'self', 'super', 'crate'} if used(n))
    line = ('use super::{%s};' % ', '.join(chosen)) if chosen else ''
    out = re.sub(r'^use super::\*;\s*$', line, t, count=1, flags=re.M)
    print(os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(test)))), '->', line[:300])
    if not dry:
        open(test, 'w', encoding='utf-8').write(out)

main()
