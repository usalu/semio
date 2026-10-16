"""🧮 Item-level losslessness: every original top-level item (normalised) must exist in the carved tree, and nothing else must appear.

lossless.py <crates.json>   (compares the pristine db tree with SP/stage)
"""
import collections, json, os, re, sys
from lib import *
from model import parse_items
from rs import strip
from scope import ALL_MODULE_IDENTS
from plan import ALL_LIBS

LIBS = sorted(set(ALL_MODULE_IDENTS) | set(ALL_LIBS) | {'db', 'db_core', 'db_io_core', 'db_io_registry', 'db_io_driver'}, key=len, reverse=True)
QUAL = re.compile(r'\b(?:' + '|'.join(re.escape(l) for l in LIBS) + r'|crate|super|self|writer|release|claimed_backend|testkit|pack|store|dsl)::')
VIS = re.compile(r'\bpub(?:\([^)]*\))?\s+')
ATTR = re.compile(r'#\[cfg(?:_attr)?\((?:[^\[\]]|\[[^\]]*\])*\)\]')
USE = re.compile(r'\buse\s[^;]*;')
TAG = re.compile(r'#\[path\s*=\s*"[^"]*"\]')

def norm(t):
    t = strip(t) if False else t
    t = re.sub(r'(?m)^\s*//.*$', '', t)
    t = re.sub(r'\bdb::(?:[a-z_]+::)?', '', t)
    t = re.sub(r'process_isolated_law\("[^"]*"', 'process_isolated_law("L"', t)
    t = ATTR.sub('', t)
    t = TAG.sub('', t)
    t = USE.sub('', t)
    t = VIS.sub('', t)
    for _ in range(3):
        t = QUAL.sub('', t)
    t = re.sub(r'include(?:_str|_bytes)?!\(\s*"[^"]*"\s*\)', 'INCLUDE', t)
    return re.sub(r'\s+', '', t)

def items_of(path):
    text = read(path)
    items, trail, lines = parse_items(text)
    out = []
    for it in items:
        if it.kind in ('use', 'extern crate'):
            continue
        if it.kind == 'mod' and strip('\n'.join(lines[it.header:it.end + 1])).rstrip().endswith(';'):
            continue
        out.append((it.kind, it.name or it.self_type, norm('\n'.join(lines[it.start:it.end + 1]))))
    return out

def walk_rs(base):
    for root, ds, fs in os.walk(base):
        ds[:] = [d for d in ds if d not in ('node_modules', 'target')]
        for f in fs:
            if f.endswith('.rs'):
                yield os.path.join(root, f)

def main():
    orig = collections.Counter(); onames = {}
    for p in walk_rs(PRISTINE):
        if p.endswith('📦️packages/🦀️rust/🦀️.rs') or p.endswith('/🛢️db/🦀️.rs') or p == PRISTINE + '/🦀️.rs':
            continue
        for kind, name, key in items_of(p):
            orig[key] += 1; onames.setdefault(key, (kind, name, os.path.relpath(p, PRISTINE)))
    new = collections.Counter(); nnames = {}
    for p in walk_rs(STAGE):
        rel = os.path.relpath(p, STAGE)
        if rel in ('🦀️.rs',) or rel.startswith('🧪️tests/🔬️unit') :
            continue
        for kind, name, key in items_of(p):
            new[key] += 1; nnames.setdefault(key, (kind, name, rel))
    missing = orig - new
    extra = new - orig
    print('original items', sum(orig.values()), 'new items', sum(new.values()))
    print('missing from the carved tree:', sum(missing.values()))
    for k, n in list(missing.items())[:40]:
        print('   ', n, *onames[k])
    print('only in the carved tree:', sum(extra.values()))
    for k, n in list(extra.items())[:40]:
        print('   ', n, *nnames[k])
main()
