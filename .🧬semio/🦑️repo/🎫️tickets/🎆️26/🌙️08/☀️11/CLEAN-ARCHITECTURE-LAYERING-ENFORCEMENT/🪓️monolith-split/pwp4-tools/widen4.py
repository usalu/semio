# 🔓️ Definition scan + widening for S0 pieces: private items/fields/methods of a moved piece that are named outside the piece's module subtree become pub(super); pub(super) becomes pub(in super::super).
import os, re, collections
from seg import *
ITEM_KINDS = ('fn', 'struct', 'enum', 'union', 'const', 'static', 'type', 'trait', 'mod', 'macro_rules')
KW = r'(?:(?:async|const|unsafe|default)\s+)*(?:extern\s+"[^"]*"\s+)?'
MOUNT = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]*)"\s*\]|\binclude!\s*\(\s*"([^"]*)"\s*\))')

def mounted_children(text, fpath, read):
    out = []
    for m in MOUNT.finditer(text):
        lit = m.group(1) or m.group(2)
        p = os.path.normpath(os.path.join(os.path.dirname(fpath), lit))
        if p.endswith('.rs'): out.append(p)
    return out

def subtree(path, read, seen=None):
    path = os.path.normpath(path)
    seen = seen if seen is not None else {}
    if path in seen: return seen
    t = read(path)
    if t is None: return seen
    seen[path] = t
    for c in mounted_children(t, path, read): subtree(c, read, seen)
    return seen

def decl_names(text):
    """private declarations of a piece text -> list of (kind, name, line_index, is_pub_super)"""
    chunks, R = segment(raw=text)
    out = []
    for c in chunks:
        if c.kind in ITEM_KINDS and c.name and c.kind != 'macro_rules':
            out.append(('item', c.name, c.hl - 1, c.vis, c))
        if c.kind == 'struct':
            out += struct_fields(c)
        if c.kind == 'impl' and not c.trait:
            out += impl_members(c)
        if c.kind == 'enum': pass
    return out

def body_lines(c):
    code = PT.strip('\n'.join(c.lines)).split('\n')
    return code

def struct_fields(c):
    code = body_lines(c); out = []
    depth = 0; started = False
    for i, l in enumerate(code):
        if not started:
            if '{' in l: started = True; depth = l.count('{') - l.count('}'); 
            continue
        d0 = depth
        if d0 == 1:
            m = re.match(r'^(\s*)(?:(pub(?:\([^)]*\))?)\s+)?([A-Za-z_]\w*)\s*:(?!:)', l)
            if m: out.append(('field', m.group(3), c.s - 1 + i, m.group(2) or '', c))
        depth += l.count('{') - l.count('}')
        if depth <= 0: break
    return out

def impl_members(c):
    code = body_lines(c); out = []
    depth = 0; started = False
    for i, l in enumerate(code):
        if not started:
            if '{' in l:
                started = True; depth = l.count('{') - l.count('}')
                rest = l.split('{', 1)[1]
                if depth == 1 and rest.strip(): l2 = rest
                else: continue
            else: continue
        d0 = depth
        if d0 == 1:
            m = re.match(r'^\s*(pub(?:\([^)]*\))?)?\s*' + KW + r'fn\s+([A-Za-z_]\w*)', l)
            if m: out.append(('method', m.group(2), c.s - 1 + i, m.group(1) or '', c))
            m = re.match(r'^\s*(pub(?:\([^)]*\))?)?\s*const\s+([A-Z_][A-Z0-9_]*)\s*:', l)
            if m: out.append(('assoc-const', m.group(2), c.s - 1 + i, m.group(1) or '', c))
        depth += l.count('{') - l.count('}')
        if depth <= 0: break
    return out

_STRIPPED = {}
def stripped(path, t):
    k = (path, len(t), hash(t))
    s = _STRIPPED.get(k)
    if s is None:
        s = _STRIPPED[k] = PT.strip(t)
    return s

def used_in(name, kind, corpus, via=None):
    if via:
        rx = re.compile(r'(?<![\w$])' + re.escape(via) + r'\s*::\s*(?:\{[^}]*)?(?<![\w$])' + re.escape(name) + r'(?!\w)')
        for path, t in corpus.items():
            if rx.search(stripped(path, t)): return path
        return None
    if kind == 'item' or kind == 'assoc-const': rx = re.compile(r'(?<![\w$])' + re.escape(name) + r'(?!\w)')
    elif kind == 'field': rx = re.compile(r'(?:\.\s*' + re.escape(name) + r'(?!\w)(?!\s*\()|(?<![\w$.:])' + re.escape(name) + r'\s*:(?!:)|(?<![\w$.:])' + re.escape(name) + r'\s*[,}])')
    else: rx = re.compile(r'(?:\.\s*|::\s*)' + re.escape(name) + r'\s*(?:\(|::<)')
    for path, t in corpus.items():
        m = rx.search(stripped(path, t))
        if m: return path
    return None

def widen_piece(text, outside_corpus, report, label, via=None):
    lines = text.split('\n')
    decls = decl_names(text)
    edits = {}
    for kind, name, li, vis, c in decls:
        # skip private fields/items that already have pub(crate)/pub
        if vis in ('pub', 'pub(crate)') or (vis.startswith('pub(in ') and not vis.startswith('pub(in super')): continue
        if vis == 'pub(super)' or vis == 'pub(in super)':
            edits[(li, kind, name)] = 'in-super-super'
            continue
        if vis == '' :
            if kind == 'item' and c.kind == 'mod' and c.name == name and c.head.rstrip().endswith(';'): pass
            hit = used_in(name, kind, outside_corpus, via if kind == 'item' else None) if not (via and kind != 'item') else None
            if hit: edits[(li, kind, name)] = ('pub-super', hit)
    for (li, kind, name), act in sorted(edits.items(), reverse=True):
        l = lines[li]
        if act == 'in-super-super':
            nl = re.sub(r'pub\((?:in\s+)?super\)', 'pub(in super::super)', l, count=1)
            report.append((label, kind, name, 'pub(super)', 'pub(in super::super)', ''))
        else:
            m = re.match(r'^(\s*)(.*)$', l)
            nl = m.group(1) + 'pub(super) ' + m.group(2)
            report.append((label, kind, name, 'private', 'pub(super)', os.path.basename(os.path.dirname(act[1])) if act else ''))
        lines[li] = nl
    return '\n'.join(lines)
