# 🔪️ S0 cut engine: moves chunks of a source module file into piece files (children of the old parent), re-exports them, rebases path literals, balances regions.
import os, re, sys, json, hashlib, collections
from seg import *

FE0F = '️'
def snake(s): return re.sub(r'[^a-z0-9]+', '_', s.lower()).strip('_')

class Piece:
    def __init__(self, pid, crate, dir, folder, doc, src, cfg=None, mod=None, dedent=0, existing=None, mounted_elsewhere=False, glob_vis='pub', header=True, unwrap=None, reexport=None):
        self.pid, self.crate, self.dir, self.folder, self.doc, self.src = pid, crate, dir, folder, doc, src
        self.cfg = cfg; self.mod = mod or snake(pid); self.dedent = dedent
        self.existing = existing            # existing destination file (lift / macro stub): content is appended
        self.mounted_elsewhere = mounted_elsewhere
        self.glob_vis = glob_vis; self.header = header; self.unwrap = unwrap; self.reexport = reexport
    @property
    def path(self): return self.existing or f'{self.dir}/{self.folder}/🦀️.rs'

PATH_ATTR = re.compile(r'(#\[\s*path\s*=\s*)"([^"]*)"')
INCL = re.compile(r'(\binclude(?:_str|_bytes)?!\s*\(\s*)"([^"]*)"')
def rebase(text, old_file, new_file):
    od, nd = os.path.dirname(old_file), os.path.dirname(new_file)
    def fix(m):
        lit = m.group(2)
        tgt = os.path.normpath(os.path.join(od, lit))
        return m.group(1) + '"' + os.path.relpath(tgt, nd) + '"'
    return INCL.sub(fix, PATH_ATTR.sub(fix, text))

class FileCut:
    def __init__(self, src):
        self.src = src; self.abs = PLUG + '/' + src
        self.raw = open(self.abs, encoding='utf-8').read()
        self.chunks, self.R = segment(raw=self.raw)
        self.sha = hashlib.sha256(self.raw.encode()).hexdigest()

def assign(fc, rules, follow=True, unwrapped=()):
    ch = fc.chunks; n = len(ch)
    dest = ['stay'] * n
    for i, c in enumerate(ch):
        if c.kind in ('moddoc', 'externcrate', 'use', 'filler') or c.kind.startswith('region'): continue
        for pid, pred in rules:
            if pred(c): dest[i] = pid; break
    if follow:
        defs_dest = {c.name: dest[i] for i, c in enumerate(ch) if c.kind in ('struct', 'enum', 'union', 'type', 'trait') and c.name and dest[i] != 'stay'}
        for i, c in enumerate(ch):
            if c.kind == 'impl' and c.target and c.target in defs_dest and dest[i] == 'stay': dest[i] = defs_dest[c.target]
        modmap = {c.name: dest[i] for i, c in enumerate(ch) if c.kind == 'mod' and c.name and dest[i] != 'stay' and c.name not in unwrapped}
        for i, c in enumerate(ch):
            if c.kind == 'use':
                m = re.match(r'^(?:pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_]\w*)::', c.head or '')
                if m and m.group(1) in modmap and m.group(1) not in ('crate', 'super', 'self'): dest[i] = modmap[m.group(1)]
    for i, c in enumerate(ch):
        if c.kind == 'filler' and any(l.strip() for l in c.lines):
            j = i + 1
            while j < len(ch) and (ch[j].kind == 'filler' or ch[j].kind.startswith('region')): j += 1
            if j < len(ch): dest[i] = dest[j]
    return dest

def region_pairs(ch):
    pairs = []; st = []
    for i, c in enumerate(ch):
        if c.kind == 'region-region': st.append(i)
        elif c.kind == 'region-endregion' and st: pairs.append((st.pop(), i))
    return sorted(pairs)

def build(fc, pieces, dest):
    ch = fc.chunks; n = len(ch)
    pc = collections.OrderedDict((p.pid, p) for p in pieces)
    pairs = region_pairs(ch)
    owner = {}
    for a, b in pairs:
        ins = {dest[k] for k in range(a + 1, b) if not ch[k].kind.startswith('region') and not (ch[k].kind == 'filler' and not any(l.strip() for l in ch[k].lines))}
        if len(ins) == 1 and 'stay' not in ins:
            owner[a] = owner[b] = next(iter(ins))
    for a, b in pairs:
        if a in owner: dest[a] = dest[b] = owner[a]
    res = []; mounted = set()
    def mount_lines(p):
        out = []
        if p.mounted_elsewhere: return out
        if p.cfg: out.append(p.cfg)
        rel = os.path.relpath(PLUG + '/' + p.path, os.path.dirname(fc.abs))
        out.append(f'#[path = "{rel}"]'); out.append(f'mod {p.mod};')
        if p.reexport != '': out.append(p.reexport if p.reexport else f'{p.glob_vis} use {p.mod}::*;')
        return out
    used = [p for p in pieces if any(dest[i] == p.pid for i in range(n))]
    block_done = False
    for i, c in enumerate(ch):
        d = dest[i]
        if d == 'stay': res.extend(c.lines)
        elif not block_done:
            block_done = True
            for p in used: res.extend(mount_lines(p))
    texts = {}
    for p in pieces:
        idxs = [i for i in range(n) if dest[i] == p.pid]
        real = [i for i in idxs if not ch[i].kind.startswith('region') and ch[i].kind != 'filler']
        if not real: continue
        lines = []
        open_stack = []   # list of (pair index)
        def enclosing(i): return [pi for pi, (a, b) in enumerate(pairs) if a < i < b]
        def close_all(target_len=0):
            while len(open_stack) > target_len:
                pi = open_stack.pop(); a, b = pairs[pi]
                lines.append(('//#endregion ' + ch[a].name).rstrip())
        for i in idxs:
            c = ch[i]
            if c.kind.startswith('region'): 
                lines.extend(c.lines); continue
            encl = [pi for pi in enclosing(i) if owner.get(pairs[pi][0]) != p.pid]
            k = 0
            while k < len(open_stack) and k < len(encl) and open_stack[k] == encl[k]: k += 1
            close_all(k)
            for pi in encl[k:]:
                lines.append(('//#region ' + ch[pairs[pi][0]].name).rstrip()); open_stack.append(pi)
            lines.extend(c.lines)
        close_all(0)
        body = '\n'.join(lines).rstrip('\n')
        if p.unwrap:
            bl = body.split('\n')
            head = next(k for k, l in enumerate(bl) if re.match(r'\s*(pub(\([^)]*\))?\s+)?mod\s+' + p.unwrap + r'\s*\{', l))
            last = max(k for k, l in enumerate(bl) if l.strip() == '}')
            body = '\n'.join(bl[head + 1:last])
            body = '\n'.join((l[4:] if l.startswith('    ') else l) for l in body.split('\n'))
            body = re.sub(r'^\s*use super::\*;\n', '', body, count=1)
        if p.dedent:
            body = '\n'.join((l[p.dedent:] if l.startswith(' ' * p.dedent) else l) for l in body.split('\n'))
        body = rebase(body, fc.abs, PLUG + '/' + p.path)
        if p.existing:
            old = open(PLUG + '/' + p.existing, encoding='utf-8').read().rstrip('\n')
            head = [old, '']
        else:
            head = [p.doc, '']
        if p.header: head += ['use super::*;', '']
        texts[p.pid] = '\n'.join(head) + body + '\n'
    resid = '\n'.join(res).rstrip('\n') + '\n'
    return resid, texts
