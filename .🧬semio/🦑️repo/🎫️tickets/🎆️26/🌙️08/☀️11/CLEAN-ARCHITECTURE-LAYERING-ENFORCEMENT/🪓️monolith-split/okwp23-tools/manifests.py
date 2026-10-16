# 📦 Cargo manifests of the carved crates: first-party edges from the rewritten sources, external specs copied from the kernel manifest, dev-dependencies from the test files, features for the hot-path census.
import os, re, sys, json, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, okcrates as C, gen_files as G
from okrs import tokenize

SKIP = {'std', 'core', 'alloc', 'crate', 'self', 'super', 'Self'}
FIRST = {'semio_framework_' + k.replace('-', '_'): k for k in C.RANK}

def heads(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = set(); n = len(toks)
    for i, t in enumerate(toks):
        if t[0] != 'id' or i + 2 >= n: continue
        if toks[i + 1][1] != ':' or toks[i + 2][1] != ':': continue
        prev = toks[i - 1][1] if i else ''
        if prev in ('.', '$', "'"): continue
        if prev == ':' and i >= 2 and toks[i - 2][1] == ':':
            pre = toks[i - 3] if i >= 3 else None
            if pre is not None and (pre[0] == 'id' or pre[1] in ('>', ')', ']')): continue
        out.add(t[1])
    # `use name::{...}` and `use name;` forms are covered by the path head; attribute paths like #[serde(...)] are not extern paths
    return out

def analyse(batch_obj):
    pl = batch_obj.plan; out = batch_obj.out
    normal = collections.defaultdict(set); test = collections.defaultdict(set)
    for f, c in pl.file_crate.items():
        h = heads(out[f])
        (test if batch_obj.is_test_file(f) else normal)[c] |= h
    for (sf, c), st in pl.staging.items(): normal[c] |= heads(out[st.path])
    return normal, test

def build_manifests(batch_obj, features=None):
    crates, model_, cargo = C.build()
    entries = C.old_entries(cargo)
    normal, test = analyse(batch_obj)
    reg = set(ns._ident_roots)
    result = {}; report = {}
    for key in batch_obj.plan.batch:
        c = crates[key]
        first, ext, missing = set(), set(), set()
        for h in normal[key]:
            if h in SKIP: continue
            if h in FIRST and FIRST[h] != key:
                first.add(FIRST[h]); continue
            if h in FIRST: continue
            if h in reg or h in C.IDENT_KEY or h.startswith('semio_framework_'):
                k = C.ident_key(h)
                if k in entries: ext.add(k)
                else: missing.add(h)
        dev_first, dev_ext = set(), set()
        for h in test[key] - normal[key]:
            if h in SKIP: continue
            if h in FIRST and FIRST[h] != key: dev_first.add(FIRST[h]); continue
            if h in FIRST: continue
            if h in reg or h in C.IDENT_KEY or h.startswith('semio_framework_'):
                k = C.ident_key(h)
                if k in entries: dev_ext.add(k)
                else: missing.add(h)
        for (sc, dc), n in batch_obj.rw.rep.cross.items():
            if sc == key and dc in C.RANK and dc != key: first.add(dc)
        c['deps'] = sorted(first, key=C.RANK.index)
        c['ext'] = sorted(ext)
        c['dev'] = sorted(dev_ext) + ['__first__' + d for d in sorted(dev_first - first)]
        c['features'] = (features or {}).get(key, [])
        c['real'] = True
        c['dep_features'] = {}
        c['cfg'] = 'all'
        report[key] = dict(first=c['deps'], ext=c['ext'], dev=c['dev'], missing=sorted(missing))
    return crates, model_, entries, report

def render(batch_obj, features=None, dev_features=None):
    crates, model_, entries, report = build_manifests(batch_obj, features)
    out = {}
    for key in batch_obj.plan.batch:
        c = crates[key]
        dev_first = [d[len('__first__'):] for d in c['dev'] if d.startswith('__first__')]
        c['dev'] = [d for d in c['dev'] if not d.startswith('__first__')]
        text, missing = G.manifest(crates, key, model_, entries)
        pkg = C.pkg_abs(key, model_)
        extra = []
        for d in dev_first:
            feats = (dev_features or {}).get(key, {}).get(d)
            extra.append(C.crate_name(d) + ' = { path = "' + os.path.relpath(C.pkg_abs(d, model_), pkg) + '"' + (', features = [' + ', '.join('"' + f + '"' for f in feats) + ']' if feats else '') + ' }')
        for d, feats in (dev_features or {}).get(key, {}).items():
            if d not in dev_first:
                extra.append(C.crate_name(d) + ' = { path = "' + os.path.relpath(C.pkg_abs(d, model_), pkg) + '", features = [' + ', '.join('"' + f + '"' for f in feats) + '] }')
        if extra:
            assert text.rstrip().endswith('[dev-dependencies]') or '[dev-dependencies]' in text
            text = text.rstrip('\n') + '\n' + '\n'.join(sorted(extra)) + '\n'
        out[key] = text
    return out, report
