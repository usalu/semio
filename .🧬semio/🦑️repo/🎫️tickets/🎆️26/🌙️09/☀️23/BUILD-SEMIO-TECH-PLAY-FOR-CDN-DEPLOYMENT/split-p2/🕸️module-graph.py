#!/usr/bin/env python3
"""🕸️ Module dependency graph of one `#[path]`-tree crate, folded onto the L6 target-crate layers.

Walks the crate root's module tree (inline modules, `#[path]` and `#[cfg(test)]` declarations, the
"files loaded through `#[path]` are mod-rs files" rule), resolves every `crate::`/`super::`/`self::`
path (including `use crate::{..}` groups) to its owning file, assigns each file a layer from the
format's layer rules, and reports layer sizes, the layer graph and every edge the target crate graph
forbids. Test-only files (`#[cfg(test)]`-gated declarations and `🧪️tests` paths) are reported apart:
they belong to the test tree of whichever crate owns their subject.

usage: 🕸️module-graph.py <crate-root.rs> <layers.json> [--edges N] [--files]
layers.json: {"order": [layer...], "allow": {layer: [layer...]}, "rules": [[regex-on-relative-file-path, layer]...],
              "modules": [[regex-on-module-path, layer]...]}   (first match wins; modules rules take precedence)
"""
import json, os, re, sys, collections

def strip(src):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith('//', i):
            j = src.find('\n', i); j = n if j < 0 else j; out.append(' ' * (j - i)); i = j
        elif src.startswith('/*', i):
            d, j = 1, i + 2
            while j < n and d:
                if src.startswith('/*', j): d += 1; j += 2
                elif src.startswith('*/', j): d -= 1; j += 2
                else: j += 1
            out.append(re.sub(r'[^\n]', ' ', src[i:j])); i = j
        elif c == 'r' and re.match(r'r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == '_')):
            m = re.match(r'r(#*)"', src[i:]); close = '"' + m.group(1); j = src.find(close, i + len(m.group(0)))
            j = n if j < 0 else j + len(close); out.append('""' + re.sub(r'[^\n]', ' ', src[i + 2:j])[2:] if j - i > 2 else '""'); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            j += 1
            keep = re.search(r'#\[path\s*=\s*$', ''.join(out[-12:]))
            out.append(src[i:j] if keep else '"' + re.sub(r'[^\n]', ' ', src[i + 1:j - 1]) + '"'); i = j
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]) or (c == "'" and re.match(r"'\\(x..|u\{[0-9a-fA-F]+\})'", src[i:i + 12])):
            m = re.match(r"'(\\x..|\\u\{[0-9a-fA-F]+\}|\\.|[^\\'])'", src[i:]); out.append(' ' * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(c); i += 1
    return ''.join(out)

PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
MOD_RE = re.compile(r'(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])')

class Crate:
    def __init__(self, root):
        self.root = os.path.abspath(root); self.base = os.path.dirname(self.root)
        self.files = {}; self.mods = {}; self.test_files = set(); self.text = {}
        self.load(self.root, (), os.path.dirname(self.root), False)

    def load(self, path, modpath, _dir, test):
        path = os.path.normpath(path)
        if path in self.files: return
        self.files[path] = modpath; self.mods.setdefault(modpath, path)
        if test: self.test_files.add(path)
        raw = open(path, encoding='utf-8').read(); s = strip(raw); self.text[path] = s
        self.walk(s, path, modpath, os.path.dirname(path), test)

    def walk(self, s, file, modpath, dirn, test):
        stack = [(modpath, dirn, test, 0)]; depth = 0; i = 0; pending_path = None; pending_test = False
        tokens = re.finditer(r'#\[path\s*=\s*"[^"]+"\]|#\[cfg\(([^\]]*)\)\]|(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*[;{]|[{};]', s)
        for m in tokens:
            t = m.group(0)
            if t == ';':
                pending_path = None; pending_test = False; continue
            if t == '{':
                depth += 1; pending_path = None; pending_test = False; continue
            if t == '}':
                depth -= 1
                while len(stack) > 1 and stack[-1][3] > depth: stack.pop()
                continue
            if t.startswith('#[path'):
                pending_path = PATH_RE.match(t).group(1); continue
            if t.startswith('#[cfg'):
                c = m.group(1) or ''
                if re.search(r'\btest\b', c) and not re.search(r'not\s*\(\s*test', c): pending_test = True
                continue
            mm = MOD_RE.match(t); name = mm.group(1); kind = mm.group(2)
            cur_mod, cur_dir, cur_test, _ = stack[-1]
            child = cur_mod + (name,); ctest = cur_test or pending_test
            if kind == '{':
                depth += 1
                ndir = os.path.normpath(os.path.join(cur_dir, pending_path)) if pending_path else os.path.join(cur_dir, name)
                self.mods.setdefault(child, file)
                if ctest: self.test_files.add(('inline', child))
                stack.append((child, ndir, ctest, depth))
            else:
                target = os.path.join(cur_dir, pending_path) if pending_path else os.path.join(cur_dir, name + '.rs')
                if os.path.exists(target): self.load(target, child, os.path.dirname(target), ctest)
            pending_path = None; pending_test = False

    def resolve(self, modpath):
        for k in range(len(modpath), 0, -1):
            p = modpath[:k]
            if p + ('component',) in self.mods and self.mods[p + ('component',)] != self.mods.get(p):
                return self.mods[p + ('component',)]
            if p in self.mods: return self.mods[p]
        return self.root

USE_RE = re.compile(r'\b(crate|super|self)((?:\s*::\s*(?:[A-Za-z_][A-Za-z0-9_]*|super))+)(\s*::\s*\{)?')

def expand_groups(s):
    out = []
    for m in re.finditer(r'\buse\s+(crate|super|self)\s*::', s):
        j = m.end(); d = 0; k = j
        while k < len(s):
            if s[k] == '{': d += 1
            elif s[k] == '}': d -= 1
            elif s[k] == ';' and d == 0: break
            k += 1
        out.append(m.group(1) + '::' + s[j:k])
    return out

def flatten(prefix, body):
    body = body.strip()
    res = []; i = 0; segs = []
    def split_top(t):
        parts, d, cur = [], 0, ''
        for ch in t:
            if ch == '{': d += 1
            if ch == '}': d -= 1
            if ch == ',' and d == 0: parts.append(cur); cur = ''
            else: cur += ch
        if cur.strip(): parts.append(cur)
        return parts
    m = re.match(r'^((?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*)\{(.*)\}\s*$', body, re.S)
    if m:
        pre = [x for x in re.split(r'\s*::\s*', m.group(1).strip()) if x]
        for part in split_top(m.group(2)): res += flatten(prefix + pre, part)
        return res
    path = [x for x in re.split(r'\s*::\s*', re.sub(r'\s+as\s+\w+$', '', body)) if x and x != '*']
    return [prefix + path]

def test_spans(s):
    out = []
    for m in re.finditer(r'#\[cfg\(\s*test\s*\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?(?:mod\s+[A-Za-z_][A-Za-z0-9_]*\s*|fn\s+[^{;]*)\{', s):
        d = 0; j = m.end() - 1
        while j < len(s):
            if s[j] == '{': d += 1
            elif s[j] == '}':
                d -= 1
                if d == 0: break
            j += 1
        out.append((m.start(), j))
    for m in re.finditer(r'#\[cfg\(\s*test\s*\)\]\s*(?:pub(?:\s*\([^)]*\))?\s+)?use\b[^;]*;', s):
        out.append((m.start(), m.end()))
    return out

def inline_spans(s):
    spans = []
    for m in re.finditer(r'(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{', s):
        d = 0; j = m.end() - 1
        while j < len(s):
            if s[j] == '{': d += 1
            elif s[j] == '}':
                d -= 1
                if d == 0: break
            j += 1
        spans.append((m.end(), j, m.group(1)))
    return spans

def references(crate, file):
    s = crate.text[file]; modpath = crate.files[file]
    spans = inline_spans(s)
    def base_at(off):
        base = list(modpath)
        if file == crate.root: return None
        for a, b, name in spans:
            if a <= off < b: base.append(name)
        return base
    paths = []
    for m in re.finditer(r'\buse\s+(crate|super|self)\s*::', s):
        j = m.end(); d = 0; k = j
        while k < len(s):
            if s[k] == '{': d += 1
            elif s[k] == '}': d -= 1
            elif s[k] == ';' and d == 0: break
            k += 1
        g = m.group(1) + '::' + s[j:k]
        head, rest = g.split('::', 1)
        for p in flatten([head], rest): paths.append((m.start(), p))
    for m in USE_RE.finditer(s):
        if m.group(3): continue
        segs = [x for x in re.split(r'\s*::\s*', (m.group(1) + m.group(2)).strip()) if x]
        paths.append((m.start(), segs))
    out = set(); tests = test_spans(s)
    for off, segs in paths:
        if not segs or any(a <= off < b for a, b in tests): continue
        if segs[0] == 'crate': mp = tuple(segs[1:])
        else:
            base = base_at(off)
            if base is None: continue
            k = 0
            while k < len(segs) and segs[k] in ('super', 'self'):
                if segs[k] == 'super' and base: base.pop()
                k += 1
            mp = tuple(base + segs[k:])
        out.add(crate.resolve(mp))
    out.discard(file)
    return out

def main():
    root, layers_path = sys.argv[1], sys.argv[2]
    show = int(sys.argv[sys.argv.index('--edges') + 1]) if '--edges' in sys.argv else 40
    cfg = json.load(open(layers_path)); crate = Crate(root)
    rules = [(re.compile(r), l) for r, l in cfg['rules']]; mrules = [(re.compile(r), l) for r, l in cfg.get('modules', [])]
    def rel(f): return os.path.relpath(f, crate.base)
    def is_test(f): return f in crate.test_files or '🧪️tests' in rel(f)
    def layer(f):
        mp = '::'.join(crate.files[f])
        for r, l in mrules:
            if r.search(mp): return l
        for r, l in rules:
            if r.search(rel(f)): return l
        return '?'
    lines = collections.Counter(); tlines = collections.Counter(); nfiles = collections.Counter()
    for f in crate.files:
        n = sum(1 for x in open(f, encoding='utf-8'))
        (tlines if is_test(f) else lines)[layer(f)] += n; nfiles[layer(f)] += 1
    graph = collections.Counter(); bad = collections.defaultdict(list)
    allow = cfg['allow']
    for f in crate.files:
        if is_test(f): continue
        a = layer(f)
        for g in references(crate, f):
            if is_test(g): continue
            b = layer(g)
            if a == b: continue
            graph[(a, b)] += 1
            if b not in allow.get(a, []): bad[(a, b)].append((rel(f), rel(g)))
    print(f'# crate {rel(crate.root)}: {len(crate.files)} files ({len(crate.test_files)} test-gated)')
    print('| layer | files | own lines (non-test) | test lines |'); print('|---|---:|---:|---:|')
    for l in cfg['order'] + sorted(set(lines) - set(cfg['order'])):
        print(f'| {l} | {nfiles[l]} | {lines[l]} | {tlines[l]} |')
    print('\n## layer edges (file references, non-test)')
    for (a, b), n in sorted(graph.items()): print(f'{a} -> {b}: {n}{"" if b in allow.get(a, []) else "   FORBIDDEN"}')
    print('\n## forbidden edges')
    for (a, b), es in sorted(bad.items()):
        print(f'### {a} -> {b} ({len(es)})')
        for x, y in sorted(set(es))[:show]: print(f'  {x}  ->  {y}')
    if '--files' in sys.argv:
        print('\n## files')
        for f in sorted(crate.files, key=rel): print(f'{layer(f)}\t{"T" if is_test(f) else "-"}\t{"::".join(crate.files[f])}\t{rel(f)}')

main()
