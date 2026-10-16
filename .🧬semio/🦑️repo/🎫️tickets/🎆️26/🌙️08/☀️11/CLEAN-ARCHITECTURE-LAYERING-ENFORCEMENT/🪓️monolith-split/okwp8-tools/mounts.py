"""🌲 Mount parent map of the kernel test closure: #[path] mods (and include!) -> child file."""
import os, re, json, sys
R = '/Users/ueli/Documents/semio/'
HERE = os.path.dirname(os.path.abspath(__file__))
PAT = re.compile(r'((?:#\s*\[[^\]]*\]\s*)*)\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_0-9]+)\s*;', re.S)
PATH = re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]')
INC = re.compile(r'include!\(\s*"([^"]+)"\s*\)')
def children(f):
    s = open(R + f, encoding='utf8').read() if not f.startswith('/') else open(f, encoding='utf8').read()
    base = os.path.dirname(R + f if not f.startswith('/') else f)
    out = []
    for m in PAT.finditer(s):
        attrs = m.group(1)
        p = PATH.search(attrs)
        cfg = 'cfg(test)' if re.search(r'cfg\s*\(\s*test\s*\)', attrs) else ''
        if p:
            tgt = os.path.normpath(os.path.join(base, p.group(1)))
            out.append((m.group(2), tgt[len(R):], cfg))
    for m in INC.finditer(s):
        tgt = os.path.normpath(os.path.join(base, m.group(1)))
        out.append(('include', tgt[len(R):], ''))
    return out
if __name__ == '__main__':
    d = json.load(open(HERE + '/../🗑️generated/okwp8-closure-before.json'))
    t = d['test native']
    paths = [r['path'] for r in t if r['kind'] != 'embedded']
    ps = set(paths)
    parent = {}
    for f in paths:
        for name, tgt, cfg in children(f):
            if tgt in ps: parent.setdefault(tgt, []).append((f, name))
    for f in paths:
        short = lambda x: x.split('🔨️modules/')[-1].replace('🧰️framework/🛍️products/💻️os/', 'K:')
        print(short(f), '<=', [(short(p), n) for p, n in parent.get(f, [])])
