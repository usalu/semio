"""📥️ `use` statement helpers: parse a file header into (cfg attrs, path, alias) leaves, find the leaves a moved text needs, and add or drop them."""
import re
from model import strip

USE = re.compile(r'((?:[ \t]*#\[[^\n]*\]\n)*)([ \t]*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);[ \t]*\n', re.M)

def expand(tree):
    tree = tree.strip()
    m = re.match(r'^(.*?)\{(.*)\}\s*$', tree, re.S)
    if not m:
        if ' as ' in tree:
            p, a = tree.rsplit(' as ', 1); return [(p.strip(), a.strip())]
        return [(tree, tree.split('::')[-1].strip())]
    prefix = m.group(1).strip().rstrip(':'); body = m.group(2); parts = []; depth = 0; cur = ''
    for ch in body:
        if ch == '{': depth += 1
        if ch == '}': depth -= 1
        if ch == ',' and depth == 0: parts.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): parts.append(cur)
    out = []
    for p in parts:
        p = p.strip()
        if not p: continue
        for path, alias in expand(p):
            if path == 'self': out.append((prefix, prefix.split('::')[-1]))
            else: out.append((prefix + '::' + path if prefix else path, alias))
    return out

def uses(text):
    """List of dicts: attrs, vis, path, alias, span for every top-level `use` of the text (column 0 only)."""
    out = []
    for m in USE.finditer(text):
        if m.group(2): continue
        for path, alias in expand(m.group(4)):
            out.append(dict(attrs=m.group(1), vis=(m.group(3) or '').strip(), path=path, alias=alias, span=m.span()))
    return out

def needed(text_moved, src_text):
    toks = set(re.findall(r'(?<![\w.])([A-Za-z_]\w*)', strip(text_moved)))
    got = []
    for u in uses(src_text):
        a = u['alias']
        if a in toks and a != '_': got.append(u)
    return got

def render(u, rewrite=None):
    path = rewrite(u['path']) if rewrite else u['path']
    leaf = path.split('::')[-1]
    tail = '' if leaf == u['alias'] else ' as ' + u['alias']
    return '%s%suse %s%s;\n' % (u['attrs'], (u['vis'] + ' ') if u['vis'] else '', path, tail)

def add_uses(dst_text, lines):
    have = {(u['path'], u['alias']) for u in uses(dst_text)}
    aliases = {u['alias'] for u in uses(dst_text)}
    new = []
    for u in lines:
        if (u['path'], u['alias']) in have or u['alias'] in aliases: continue
        new.append(render(u))
    if not new: return dst_text
    ms = [m for m in USE.finditer(dst_text) if not m.group(2)]
    first_item = re.search(r'(?m)^(?!use |//|#\[|\s*$)', dst_text)
    ms = [m for m in ms if first_item is None or m.end() <= first_item.start()]
    cut = ms[-1].end() if ms else 0
    if not ms:
        m = re.match(r'(?:\s*//![^\n]*\n)*\n?', dst_text); cut = m.end()
    return dst_text[:cut] + ''.join(new) + dst_text[cut:]
