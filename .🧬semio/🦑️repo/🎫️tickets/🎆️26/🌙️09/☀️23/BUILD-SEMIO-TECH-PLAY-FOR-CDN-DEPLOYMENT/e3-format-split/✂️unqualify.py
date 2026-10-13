#!/usr/bin/env python3
"""✂️ Drops the qualification of paths whose exact item a file already imports at its top level.

For every top-level `use <path>::{A, B};` / `use <path>::A;` of a file, occurrences of `<path>::A` in that
file (outside the `use` lines themselves) become `A` — the `unused_qualifications` lint fix, applied
textually. A name imported from two different paths is left alone.

usage: ✂️unqualify.py <file.rs>...
"""
import re, sys, collections

def imports(text):
    out = collections.defaultdict(set)
    for m in re.finditer(r'(?m)^use\s+((?:crate|[a-z_][a-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*)::(\{[^;]*\}|[A-Za-z_][A-Za-z0-9_]*)\s*;', text):
        path, items = m.group(1), m.group(2)
        for item in re.split(r'\s*,\s*', items.strip('{}')):
            item = item.strip()
            if re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', item): out[item].add(path)
    return out

for f in sys.argv[1:]:
    text = open(f, encoding='utf-8').read()
    imp = imports(text); n = 0
    lines = text.split('\n')
    for i, line in enumerate(lines):
        if re.match(r'\s*(pub\s+)?use\s', line): continue
        for name, paths in imp.items():
            if len(paths) != 1: continue
            path = next(iter(paths))
            pat = re.compile(r'(?<![A-Za-z0-9_:])' + re.escape(path) + r'::' + name + r'\b')
            new = pat.sub(name, line)
            if new != line: n += 1; line = new
        lines[i] = line
    if n: open(f, 'w', encoding='utf-8').write('\n'.join(lines))
    print(f'{n:4} {f}')
