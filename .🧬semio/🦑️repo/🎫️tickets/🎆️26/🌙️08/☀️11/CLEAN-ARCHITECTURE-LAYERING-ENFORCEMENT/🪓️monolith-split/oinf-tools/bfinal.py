"""Assemble the complete apply tree for the board carve: crates, manifests, facade, rewritten outside files."""
import sys, os, re, json, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bbuild, bgen, bcrates, bman, bapi, bfacade, bplan2, bxfile, bxinfer, btgen, oinf_rs
from bcrates import *
SC = bbuild.SNAP.rsplit('/snapshot-board', 1)[0]
SNAP = bbuild.SNAP
R = "/Users/ueli/Documents/semio/"
INF = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"

def sibling_idents(texts):
    out = set()
    for t in texts:
        for m in re.finditer(r'semio_framework_os_infinite_board_([a-z_]+)', t): out.add(m.group(1).replace('_', '-'))
    return out

def main(out):
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(out)
    P, crates, upward = bbuild.main(out + '/INF')
    # manifests
    libtexts = {}; testtexts = collections.defaultdict(list); targets = collections.defaultdict(list)
    for c in crates:
        extra = ''
        libtexts[c.id] = c.text
    for cid, path, n in P['testfiles']:
        text = open(path, encoding='utf-8').read()
        testtexts[cid].append(text)
        d = os.path.basename(os.path.dirname(path))
        targets[cid].append((f"board_{cid.replace('-', '_')}_{bman.slug(d)}", f"../../🧪️tests/{d}/🦀️.rs"))
    idx = {c[0]: i for i, c in enumerate(CRATES)}
    siblings = {}
    for c in crates:
        sib = sibling_idents([libtexts[c.id]] + testtexts[c.id])
        siblings[c.idx] = {idx[s] for s in sib if s != c.id}
    bman.write_manifests(crates, out + '/INF', siblings, libtexts, testtexts, targets)
    # facade
    mods = bapi.build_tree(SNAP + '/📦️packages/🦀️rust/🦀️.rs', SNAP + '/')
    ex = bapi.exports(mods)
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    P['mods_old'] = mods
    return P, crates, libtexts, testtexts, targets, siblings

def main2(out):
    P, crates, libtexts, testtexts, targets, siblings = main(out)
    ftext, missing = facade_text(crates, P['defcfg'], P)
    assert not missing, missing
    ext, used, unres = externals()
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    # externally used free functions are part of the facade surface
    by = collections.defaultdict(list)
    for n in sorted(used): by[wdef[n]].append(n)
    extra = ''.join(f"pub use {crate_ident(CRATES[ci][0])}::{{{', '.join(ns)}}};\n" if len(ns) > 1 else f"pub use {crate_ident(CRATES[ci][0])}::{ns[0]};\n" for ci, ns in sorted(by.items()))
    ftext = ftext.replace('//#endregion 🔖️BoardFacade', extra + '//#endregion 🔖️BoardFacade')
    os.makedirs(out + '/INF/📦️packages/🦀️rust', exist_ok=True)
    open(out + '/INF/📦️packages/🦀️rust/🦀️.rs', 'w', encoding='utf-8').write(ftext)
    for f, nt in ext.items():
        os.makedirs(os.path.dirname(f"{out}/EXT/{f}"), exist_ok=True)
        open(f"{out}/EXT/{f}", 'w', encoding='utf-8').write(nt)
    return P, crates, libtexts, testtexts, targets, siblings, ext, used


def facade_text(crates, defcfg, P):
    mods = P['mods_old']
    ex = bapi.exports(mods)
    lines, missing = bfacade.render(mods, ex, crates, defcfg)
    live = open(R + INF + '/📦️packages/🦀️rust/🦀️.rs', encoding='utf-8').read()
    head = live[:live.index('#[path = "."]\npub mod board {')]
    kept = set(re.findall(r'^pub use [A-Za-z0-9_:]+::(\w+);', head, re.M)) | {'world'}
    out = []
    for l in lines:
        m = re.match(r'^pub use [A-Za-z0-9_:]+::(\w+);$', l)
        if m and m.group(1) in kept: continue
        out.append(l)
    body = '\n'.join(out)
    return head + '//#region 🔖️BoardFacade\n/// ♾️ Explicit re-exports of the board family: every name here lives in a producer crate `semio-framework-os-infinite-board-*`.\n' + body + '\n//#endregion 🔖️BoardFacade\n', missing

def externals():
    meta = bxfile.load_meta(); specs = bxfile.specs()
    files = [f for f in open(SC + '/../board-users.txt', encoding='utf-8', errors='replace').read().split('\n') if f and '♾️infinite' not in f and '🎫️tickets' not in f]
    out = {}; used = {}; unresolved = []
    for f in files:
        t = open(R + f, encoding='utf-8', errors='replace').read()
        if 'BoardHost' not in t and 'DagHost' not in t: continue
        nt, plan, haz, flags, bad = bxinfer.rewrite_file(t, f, meta, specs)
        if nt == t: continue
        nf = bxinfer.new_free_names(t, nt, meta)
        byh = {}
        for n, h in nf.items(): byh.setdefault(h, set()).add(n)
        nt2 = bxinfer.add_imports(nt, byh)
        if 'UNRESOLVED-ALIAS' in nt2: unresolved.append(f)
        out[f] = nt2
        for n in nf: used[n] = nf[n]
        if haz or bad: print('HAZARD/BAD', f, haz[:3], bad[:3])
    return out, used, unresolved

if __name__ == '__main__':
    P, crates, libtexts, testtexts, targets, siblings, ext, used = main2(sys.argv[1])
    print('external files', len(ext), 'free names', sorted(used))
    for c in crates: print(c.id, 'deps', sorted(CRATES[k][0] for k in siblings[c.idx]), 'tests', len(targets[c.id]))
    print(open(sys.argv[1] + '/INF/' + CRATES[7][2] + '/📦️packages/🦀️rust/Cargo.toml', encoding='utf-8').read())
