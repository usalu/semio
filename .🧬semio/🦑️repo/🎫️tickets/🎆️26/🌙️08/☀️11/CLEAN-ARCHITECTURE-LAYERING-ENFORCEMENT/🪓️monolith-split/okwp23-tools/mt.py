# 🌳 Module-tree reader of the OS kernel for the WP2/WP3 carve: file -> (parent file, module name, visibility, cfg chain), cfg-agnostic, token level (comments and strings excluded).
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
from okrs import tokenize
R = '/Users/ueli/Documents/semio'
K = R + '/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules'
S = M + '/🏪️store'
ATTR_RX = re.compile(r'#\s*\[\s*([^\]]*)\]')

def mounts(path):
    """Yield dicts for every `mod x;` / `mod x { ... }` / include! at any brace depth, with the attribute list directly above it."""
    src = open(path, encoding='utf8').read()
    toks = [t for t in tokenize(src)]
    sig = [(i, t) for i, t in enumerate(toks) if t[0] not in ('ws', 'comment')]
    out = []
    n = len(sig)
    j = 0
    attrs = []
    depth = 0
    inline_stack = []
    while j < n:
        _, (kd, tx, a, b) = sig[j]
        if tx == '#' and j + 1 < n and sig[j + 1][1][1] == '[':
            k = j + 2; d = 1
            while k < n and d:
                if sig[k][1][1] == '[': d += 1
                elif sig[k][1][1] == ']': d -= 1
                k += 1
            attrs.append(src[sig[j][1][2]:sig[k - 1][1][3]])
            j = k; continue
        if kd == 'id' and tx in ('mod', 'pub', 'include'):
            k = j; vis = ''
            if tx == 'pub':
                k += 1; vis = 'pub'
                if k < n and sig[k][1][1] == '(':
                    d = 1; k += 1
                    while k < n and d:
                        if sig[k][1][1] == '(': d += 1
                        elif sig[k][1][1] == ')': d -= 1
                        k += 1
                    vis = 'pub(restricted)'
            if k < n and sig[k][1][1] == 'mod' and k + 1 < n and sig[k + 1][1][0] == 'id':
                name = sig[k + 1][1][1]
                nxt = sig[k + 2][1][1] if k + 2 < n else ''
                pm = None
                for at in attrs:
                    m = re.match(r'#\s*\[\s*path\s*=\s*"([^"]*)"\s*\]', at)
                    if m: pm = m.group(1)
                cfgs = [re.sub(r'\s+', ' ', at) for at in attrs if re.match(r'#\s*\[\s*cfg', at)]
                out.append(dict(name=name, vis=vis, path=pm, cfg=cfgs, inline=(nxt == '{'), pos=sig[j][1][2] if not attrs else None))
                attrs = []
                j = k + 2; continue
            attrs = []
        if kd == 'id' and tx == 'include' and j + 3 < n and sig[j + 1][1][1] == '!' and sig[j + 2][1][1] == '(' and sig[j + 3][1][0] == 'str':
            out.append(dict(name=None, include=sig[j + 3][1][1].strip('"'), cfg=[re.sub(r'\s+', ' ', at) for at in attrs if re.match(r'#\s*\[\s*cfg', at)], inline=False))
        if tx in (';', '{', '}'): attrs = []
        j += 1
    return out

def build(root=K + '/🦀️.rs'):
    """Return dict file -> dict(parent, name, vis, cfg, mod_path) for the cfg-agnostic closure. mod_path: tuple of module names from the crate root."""
    tree = {}
    stack = [(root, None, None, '', [], ())]
    while stack:
        f, parent, name, vis, cfg, mp = stack.pop()
        if f in tree: continue
        tree[f] = dict(parent=parent, name=name, vis=vis, cfg=cfg, mod_path=mp)
        base = os.path.dirname(f)
        for m in mounts(f):
            if m.get('include'):
                p = os.path.normpath(os.path.join(base, m['include']))
                if os.path.exists(p): stack.append((p, f, None, '', cfg + m['cfg'], mp))
                continue
            if m['path']:
                p = os.path.normpath(os.path.join(base, m['path']))
                if os.path.exists(p) and p.endswith('.rs'):
                    stack.append((p, f, m['name'], m['vis'], cfg + m['cfg'], mp + (m['name'],)))
    return tree

if __name__ == '__main__':
    t = build()
    for f, d in sorted(t.items(), key=lambda kv: kv[1]['mod_path']):
        print('::'.join(d['mod_path']) or '<root>', '|', d['vis'], '|', f.replace(R + '/', '').replace('🧰️framework/🛍️products/💻️os/', ''), '|', ' '.join(d['cfg']))
