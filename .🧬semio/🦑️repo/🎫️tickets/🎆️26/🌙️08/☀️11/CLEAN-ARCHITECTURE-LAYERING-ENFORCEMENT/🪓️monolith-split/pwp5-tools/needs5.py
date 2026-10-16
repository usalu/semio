"""🧭️ What a staged child file takes from its parent module through `use super::*`: the parent's `use` declarations are flattened to name -> full path and intersected with the identifiers the child uses, so the flip can replace the glob by explicit imports and the manifest of the child crate gets the external crates it really needs. Names the parent does not import (they come from the parent's own glob or items) are listed separately. usage: python3 needs5.py <parent.rs> <child.rs>... [--json out]"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import plan_tools as pt
from rsitems import split_items

def flatten(spec, prefix=''):
    spec = spec.strip()
    out = []
    i = 0; depth = 0; cur = ''; parts = []
    for ch in spec:
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    if len(parts) > 1:
        for p in parts: out += flatten(p, prefix)
        return out
    spec = parts[0].strip() if parts else ''
    if not spec: return out
    m = re.match(r'^(.*?)::\{(.*)\}$', spec, re.S)
    if m: return flatten(m.group(2), prefix + m.group(1) + '::')
    if spec.startswith('{') and spec.endswith('}'): return flatten(spec[1:-1], prefix)
    path = prefix + spec
    mm = re.match(r'^(.*?)\s+as\s+(\w+)$', path)
    if mm: out.append((mm.group(2), mm.group(1)))
    elif path.endswith('::*'): out.append(('*', path))
    elif path.endswith('::self'): out.append((path.split('::')[-2], path[:-6]))
    else: out.append((path.split('::')[-1], path))
    return out

def parent_imports(text):
    imps = {}; globs = []
    for it in split_items(text):
        if it.kind != 'use': continue
        t = re.sub(r'//[^\n]*', '', it.text)
        t = re.sub(r'#\[[^\]]*\]', '', t)
        m = re.search(r'\buse\s+(.*);\s*$', t.strip(), re.S)
        if not m: continue
        for name, path in flatten(re.sub(r'\s+', ' ', m.group(1))):
            if name == '*': globs.append(path)
            else: imps[name] = path.strip()
    return imps, globs

def used_idents(text):
    return set(re.findall(r'[A-Za-z_]\w*', pt.strip(text)))

def needs(parent, child):
    imps, globs = parent_imports(open(parent, encoding='utf8').read())
    ids = used_idents(open(child, encoding='utf8').read())
    have = {n: p for n, p in imps.items() if n in ids}
    return have, globs

if __name__ == '__main__':
    av = sys.argv[1:]
    if '--json' in av: i = av.index('--json'); del av[i:i + 2]
    args = av
    parent, kids = args[0], args[1:]
    res = {}
    for k in kids:
        have, globs = needs(parent, k)
        crates = {}
        for n, p in have.items(): crates.setdefault(p.split('::')[0].lstrip(':'), []).append(p)
        res[k] = dict(imports=have, globs=globs, crates={c: sorted(v) for c, v in crates.items()})
        print(os.path.basename(os.path.dirname(k)), '| parent imports used:', len(have), '| origin crates:', {c: len(v) for c, v in crates.items()})
    if '--json' in sys.argv: json.dump(res, open(sys.argv[sys.argv.index('--json') + 1], 'w'), ensure_ascii=False, indent=1)
