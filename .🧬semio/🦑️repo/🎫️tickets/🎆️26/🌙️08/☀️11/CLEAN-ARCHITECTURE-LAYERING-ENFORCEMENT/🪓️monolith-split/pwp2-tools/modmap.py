#!/usr/bin/env python3
# 🧭️ Module -> crate map of the monolith: every `mod NAME` mounted by the snapshot hubs and the snapshot root, with its target file and the plan_tools crate of that file.
import json, re, sys, os
sys.argv_saved = list(sys.argv); sys.argv = ['x']
import plan_tools as pt
from common import *
from rs import blank

def snap_or_live(rel):
    p = SNAP + '/' + rel
    return p if os.path.exists(p) else PLUG + '/' + rel

def mounts_of(text, base_dir_rel):
    out = []
    clean = blank(text)
    for m in re.finditer(r'((?:#\[[^\]]*\]\s*)*)((?:pub(?:\([^)]*\))?\s+)?)mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;', clean):
        attrs = text[m.start(1):m.end(1)]
        pm = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', attrs)
        cfg = ' '.join(re.findall(r'#\[(cfg[^\]]*)\]', attrs))
        if not pm:
            continue
        rel = os.path.normpath(os.path.join(base_dir_rel, pm.group(1)))
        out.append(dict(name=m.group(3), vis=m.group(2).strip(), file=rel, cfg=cfg))
    return out

def build():
    mods = []
    root = read(snap_or_live('🦀️.rs'))
    mods += mounts_of(root, '')
    for hub in sorted(os.listdir(SNAP + '/🪝️mounts')):
        t = read(SNAP + '/🪝️mounts/' + hub + '/🦀️.rs')
        mods += mounts_of(t, '🪝️mounts/' + hub)
    return mods

if __name__ == '__main__':
    mods = build()
    for m in mods:
        print(f"{m['name']:34s} {pt.file_crate(m['file']):22s} {m['cfg'][:30]:30s} {m['file']}")
