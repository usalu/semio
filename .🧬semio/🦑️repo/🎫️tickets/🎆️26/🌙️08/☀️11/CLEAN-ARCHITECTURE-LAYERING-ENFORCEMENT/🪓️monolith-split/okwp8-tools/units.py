"""📦 Test units: top-level mounted test trees of the kernel test closure with lines/bytes/files incl. embedded includes."""
import os, re, json, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mounts
R = '/Users/ueli/Documents/semio/'
HERE = os.path.dirname(os.path.abspath(__file__))
INC = re.compile(r'include_(?:str|bytes)!\(\s*(?:concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*)?"([^"]+)"')
KDIR = R + '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
def lines_active(s): return len(s.split('\n'))
def embedded(f):
    s = open(R + f, encoding='utf8').read()
    base = os.path.dirname(R + f)
    out = []
    for m in INC.finditer(s):
        p = m.group(1)
        if p.endswith('.rs'): continue
        out.append((os.path.normpath(KDIR + p) if p.startswith('/') else os.path.normpath(os.path.join(base, p)))[len(R):])
    return out
def tree(f, ps, seen=None):
    seen = seen if seen is not None else set()
    if f in seen: return []
    seen.add(f)
    out = [f]
    for name, tgt, cfg in mounts.children(f):
        if tgt in ps: out += tree(tgt, ps, seen)
    return out
def metrics(files):
    lines = 0; byt = 0; emb = set()
    for f in files:
        s = open(R + f, encoding='utf8').read()
        lines += lines_active(s); byt += len(s.encode())
        for e in embedded(f):
            emb.add(e)
    ebytes = sum(os.path.getsize(R + e) for e in emb if os.path.exists(R + e))
    return dict(files=len(files) + len(emb), lines=lines, bytes=byt + ebytes, emb=sorted(emb))
