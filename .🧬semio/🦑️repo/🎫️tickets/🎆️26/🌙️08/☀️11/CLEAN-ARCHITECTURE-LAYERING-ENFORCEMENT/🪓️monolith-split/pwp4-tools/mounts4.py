# 🔩️ Mount equivalence: the resolved mount targets (`#[path]`, `include!`, `include_str!`, `include_bytes!`) of the two cut files before equal those of the residuals plus their pieces after, plus exactly the piece files.
import os, re, sys
from common import *
from spec4 import *
RX = re.compile(r'(?:#\[\s*path\s*=\s*"([^"]*)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)|\binclude(?:_str|_bytes)?!\s*\(\s*"([^"]*)"\s*\))')
def targets(path, text):
    out = []
    for m in RX.finditer(text):
        lit = m.group(1) or m.group(3)
        out.append((os.path.normpath(os.path.join(os.path.dirname(path), lit)), m.group(2) or '<include>'))
    return out
bad = 0
for src, pieces, rules in CUTS:
    base = SCR + '/precut/' + src
    base_text = open(base, encoding='utf-8').read()
    before = collections.Counter(t for t, _n in targets(PLUG + '/' + src, base_text)) if False else None
import collections
for src, pieces, rules in CUTS:
    base_text = open(SCR + '/precut/' + src, encoding='utf-8').read()
    before = collections.Counter(os.path.normpath(t) for t, _n in targets(os.path.normpath(PLUG + '/' + src), base_text))
    after = collections.Counter()
    for f in [src] + [p.path for p in pieces if not p.mounted_elsewhere or True]:
        full = os.path.normpath(PLUG + '/' + f)
        if not os.path.exists(full): continue
        # the lift/macro destinations: only the text WP4 added counts; their stub text has no mounts
        for t, _n in targets(full, open(full, encoding='utf-8').read()): after[os.path.normpath(t)] += 1
    piece_files = {os.path.normpath(PLUG + '/' + p.path) for p in pieces}
    lost = before - after; added = after - before
    unexpected = {k: v for k, v in added.items() if k not in piece_files}
    print(src, '| mounts before', sum(before.values()), 'after (residual + pieces)', sum(after.values()), '| lost', dict(lost) or 0, '| added non-piece', unexpected or 0)
    if lost or unexpected: bad += 1
sys.exit(1 if bad else 0)
