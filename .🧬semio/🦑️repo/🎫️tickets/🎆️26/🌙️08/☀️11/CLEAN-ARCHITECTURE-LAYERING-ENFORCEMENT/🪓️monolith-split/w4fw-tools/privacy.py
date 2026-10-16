import os, re, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lex import mask
from cfg import *
# scan the produced cluster roots (and kept children / tests) in tree R for private members used from other clusters
def read(R, rel): return open(os.path.join(R, rel), encoding='utf-8').read()
def cluster_files(census, R):
    res = json.load(open(census)); owner = {}
    for r in res:
        if 'list' not in r: continue
        for p, kind, b, l in r['list']:
            if p.endswith('.rs'): owner.setdefault(p, set()).add(r['id'])
    return owner
FIELD = re.compile(r'^\s*(pub(?:\([^)]*\))?\s+)?([a-z_][A-Za-z0-9_]*)\s*:\s*', re.M)
def struct_blocks(masked):
    """yield (name, body, kind) for top-level struct definitions (named or tuple)"""
    for m in re.finditer(r'(?m)^(?:\s*pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_]\w*)\s*(<[^{;(]*>)?\s*([({;])', masked):
        name = m.group(1); op = m.group(3); i = m.end() - 1
        if op == ';': yield name, '', 'unit'; continue
        close = '}' if op == '{' else ')'
        depth = 0; j = i
        while j < len(masked):
            if masked[j] == op: depth += 1
            elif masked[j] == close:
                depth -= 1
                if depth == 0: break
            j += 1
        yield name, masked[i+1:j], 'named' if op == '{' else 'tuple'
def private_members(R, relroot, text):
    masked = mask(text)
    out = []
    for name, body, kind in struct_blocks(masked):
        if kind == 'named':
            depth = 0; cur = ''; fields = []
            for ch in body:
                if ch in '<([{': depth += 1
                if ch in '>)]}': depth -= 1
                if ch == ',' and depth == 0: fields.append(cur); cur = ''
                else: cur += ch
            fields.append(cur)
            for f in fields:
                f = re.sub(r'#\[[^\]]*\]', '', f).strip()
                m = re.match(r'(pub(?:\([^)]*\))?\s+)?([a-z_]\w*)\s*:', f)
                if m and not m.group(1): out.append(('field', name, m.group(2)))
                elif m and m.group(1) and 'crate' in m.group(1): out.append(('field', name, m.group(2)))
        elif kind == 'tuple':
            parts = [p.strip() for p in re.split(r',(?![^<]*>)', body) if p.strip()]
            for idx, p in enumerate(parts):
                p = re.sub(r'#\[[^\]]*\]', '', p).strip()
                if not p.startswith('pub ') or 'crate' in p.split()[0] and p.startswith('pub('): out.append(('tuple', name, str(idx)))
    # inherent impl private methods
    for m in re.finditer(r'(?m)^impl(?:<[^{]*?>)?\s+([A-Za-z_]\w*)(?:<[^{]*?>)?\s*(?:where[^{]*)?\{', masked):
        tname = m.group(1); i = m.end() - 1; depth = 0; j = i
        while j < len(masked):
            if masked[j] == '{': depth += 1
            elif masked[j] == '}':
                depth -= 1
                if depth == 0: break
            j += 1
        body = masked[i+1:j]
        d = 0
        for fm in re.finditer(r'(?m)^    (pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([a-z_]\w*)', body):
            vis = fm.group(1)
            if not vis or vis.strip() != 'pub': out.append(('method', tname, fm.group(2)))
    return out
