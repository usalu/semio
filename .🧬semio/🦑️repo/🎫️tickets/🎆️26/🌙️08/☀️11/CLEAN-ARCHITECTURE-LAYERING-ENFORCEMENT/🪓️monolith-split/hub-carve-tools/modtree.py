#!/usr/bin/env python3
"""🌳️ Module tree of a Rust crate root: follows #[path]-mounted modules (any cfg), prints crate path, file, cfg attrs and raw line count."""
import re, sys, os
from pathlib import Path

PATH_RE = re.compile(r'((?:\s*#\[[^\]]*\]\s*)*)\s*(pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*(;|\{)', re.M)
ATTR_PATH = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
ATTR_CFG = re.compile(r'#\[\s*(cfg(?:_attr)?\([^\]]*\))\s*\]')

def strip_comments(src):
    out = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i)
            if j < 0: j = n
            out.append(' ' * (j - i)); i = j
        elif src.startswith('/*', i):
            depth = 1; j = i + 2
            while j < n and depth:
                if src.startswith('/*', j): depth += 1; j += 2
                elif src.startswith('*/', j): depth -= 1; j += 2
                else: j += 1
            out.append(re.sub(r'[^\n]', ' ', src[i:j])); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            out.append(src[i:j+1]); i = j + 1
        else:
            out.append(c); i += 1
    return ''.join(out)

def walk(file, modpath, cfgs, out, seen, depth=0):
    src = Path(file).read_text()
    out.append((modpath, str(file), cfgs, src.count('\n') + 1))
    code = strip_comments(src)
    for m in PATH_RE.finditer(code):
        attrs, vis, name, term = m.groups()
        if term != ';': continue
        pm = ATTR_PATH.search(attrs)
        cf = ATTR_CFG.findall(attrs)
        base = Path(file).parent
        if pm:
            child = (base / pm.group(1)).resolve()
        else:
            continue
        key = str(child)
        if key in seen:
            out.append((modpath + '::' + name, key + ' (DUP)', cf, 0)); continue
        seen.add(key)
        walk(child, modpath + '::' + name, cfgs + cf, out, seen, depth + 1)

if __name__ == '__main__':
    root = Path(sys.argv[1]).resolve()
    out = []
    walk(root, 'crate', [], out, {str(root)})
    base = Path(os.environ.get('BASE', '/Users/ueli/Documents/semio/🌎️hub'))
    for mp, f, cf, n in out:
        try: f2 = str(Path(f.split(' (DUP)')[0]).relative_to(base)) + (' (DUP)' if '(DUP)' in f else '')
        except Exception: f2 = f
        print(f"{n:6d}  {mp}  {f2}  {' '.join(cf)}")
