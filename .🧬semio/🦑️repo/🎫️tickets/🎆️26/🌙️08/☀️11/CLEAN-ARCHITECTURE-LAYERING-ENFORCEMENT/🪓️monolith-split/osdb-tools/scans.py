"""🕵️ Static scans over the staged crates: orphan/inherent impls, trait-in-scope, losslessness, restricted-visibility residue.

scans.py <stage db dir> <crates.json>
"""
import collections, json, os, re, sys
from lib import *
from model import parse_items
from rs import strip, idents
from widen import Crate

def type_defs(x):
    out = {}
    for f in x.files:
        for it in x.items(f):
            if it.kind in ('struct', 'enum', 'trait', 'union', 'type') and it.name:
                out.setdefault(it.name, set()).add(x.lib)
    return out

def scan(stage, crates_json):
    meta = json.load(open(crates_json))
    crates = {c['lib']: Crate(stage, c['lib'], c['root'], c['files']) for c in meta}
    defs = collections.defaultdict(set)
    traits = collections.defaultdict(set)
    for x in crates.values():
        for f in x.files:
            for it in x.items(f):
                if it.kind in ('struct', 'enum', 'union', 'type') and it.name:
                    defs[it.name].add(x.lib)
                if it.kind == 'trait' and it.name:
                    traits[it.name].add(x.lib)
    out = collections.defaultdict(list)
    for x in crates.values():
        for f in x.files:
            for it in x.items(f):
                if it.kind != 'impl':
                    continue
                st = it.self_type
                tr = it.trait
                st_local = x.lib in defs.get(st, ())
                st_other = bool(defs.get(st)) and not st_local
                if not tr:
                    if st_other or (not defs.get(st) and st not in ('',)):
                        out['inherent-impl-foreign-type'].append(f'{x.lib}:{f}:{it.header + 1} impl {st}')
                else:
                    tr_local = x.lib in traits.get(tr, ())
                    tr_other = bool(traits.get(tr)) and not tr_local
                    if not tr_local and not st_local and (tr_other or st_other):
                        out['orphan'].append(f'{x.lib}:{f}:{it.header + 1} impl {tr} for {st} (trait_in={sorted(traits.get(tr, ()))}, type_in={sorted(defs.get(st, ()))})')
    return out

if __name__ == '__main__':
    r = scan(sys.argv[1], sys.argv[2])
    for k, v in r.items():
        print(k, len(v))
        for l in v[:40]:
            print('  ', l[:260])
    if not r:
        print('no orphan or inherent-impl violations')
