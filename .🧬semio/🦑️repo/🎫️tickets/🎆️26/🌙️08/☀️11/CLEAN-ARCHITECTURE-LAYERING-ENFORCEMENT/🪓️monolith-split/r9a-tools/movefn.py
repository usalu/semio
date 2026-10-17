#!/usr/bin/env python3
"""Moves free functions (with their doc comments and attributes) from one Rust file to the end of another and carries the imports they need.
usage: movefn.py <src.rs> <dst.rs> <fn,fn,...> [--vis=pub] [--dry]
The imports are the top-level `use` leaves of the source (or the destination's own) whose names occur in the moved text and are not yet imported in the destination."""
import re, sys

def read(p): return open(p, encoding='utf-8').read()

def expand(path):
    path = path.strip()
    m = re.match(r'^(.*?)::\{(.*)\}$', path, re.S)
    if not m:
        return [path]
    base, inner = m.group(1), m.group(2)
    parts, depth, cur = [], 0, ''
    for ch in inner:
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur); cur = ''
        else:
            cur += ch
    if cur.strip(): parts.append(cur)
    out = []
    for part in parts:
        part = part.strip()
        if not part: continue
        for sub in expand(part):
            out.append(base + '::' + sub if sub != 'self' else base)
    return out

def leaf(full):
    m = re.search(r'\bas\s+(\w+)$', full)
    if m: return m.group(1)
    return full.split('::')[-1]

def uses(text):
    res = []
    for m in re.finditer(r'^use\s+([^;]+);', text, re.M):
        for p in expand(m.group(1)):
            res.append(p)
    return res

def find_fn(text, name):
    m = re.search(r'^(?P<pre>(?:[ \t]*(?:///[^\n]*|#\[[^\n]*\])\n)*)[ \t]*(?:pub(?:\([a-z]+\))?\s+)?fn\s+' + re.escape(name) + r'\b', text, re.M)
    if not m: raise SystemExit('fn not found: ' + name)
    start = m.start()
    i = text.index('{', m.end())
    depth = 0; j = i
    in_str = False
    while j < len(text):
        c = text[j]
        if in_str:
            if c == '\\': j += 1
            elif c == '"': in_str = False
        else:
            if c == '"': in_str = True
            elif c == "'" and re.match(r"'(?:\\.|[^\\'])'", text[j:j+4]): j += len(re.match(r"'(?:\\.|[^\\'])'", text[j:j+4]).group(0)) - 1
            elif c == '{': depth += 1
            elif c == '}':
                depth -= 1
                if depth == 0: break
        j += 1
    end = j + 1
    if text[end:end+1] == '\n': end += 1
    return start, end

def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    flags = [a for a in sys.argv[1:] if a.startswith('--')]
    src, dst, names = args[0], args[1], args[2].split(',')
    vis = 'pub'
    for f in flags:
        if f.startswith('--vis='): vis = f[6:]
    st, dt = read(src), read(dst)
    moved = []
    for name in names:
        s, e = find_fn(st, name)
        body = st[s:e]
        st = st[:s] + st[e:].lstrip('\n') if st[e:e+1] == '\n' else st[:s] + st[e:]
        body = re.sub(r'^(\s*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+' + re.escape(name) + r'\b', lambda m: m.group(1) + vis + ' fn ' + name, body, count=1, flags=re.M)
        moved.append(body)
    text = '\n'.join(moved)
    idents = set(re.findall(r'[A-Za-z_]\w*', text))
    have = {leaf(p) for p in uses(dt)} | set(re.findall(r'^(?:pub(?:\([a-z]+\))?\s+)?(?:fn|const|static|struct|enum|type|trait)\s+(\w+)', dt, re.M))
    add = []
    for p in uses(st) + uses(read(src)):
        l = leaf(p)
        if l in idents and l not in have and not p.startswith('super::') and l != '*' and p not in add:
            add.append(p); have.add(l)
    globs = [p for p in uses(read(src)) if p.endswith('::*') and p not in uses(dt)]
    if '--dry' in flags:
        print('moved', names, 'imports', add, 'globs', globs); return
    header = ''.join('use ' + p + ';\n' for p in add)
    m = re.search(r'^(?:use [^;]*;\n)+', dt, re.M)
    if header:
        if m: dt = dt[:m.end()] + header + dt[m.end():]
        else: dt = header + '\n' + dt
    dt = dt.rstrip('\n') + '\n\n' + text.rstrip('\n') + '\n'
    open(dst, 'w', encoding='utf-8').write(dt)
    open(src, 'w', encoding='utf-8').write(st)
    print('moved', names, 'imports', add, 'globs-not-carried', globs)

main()
