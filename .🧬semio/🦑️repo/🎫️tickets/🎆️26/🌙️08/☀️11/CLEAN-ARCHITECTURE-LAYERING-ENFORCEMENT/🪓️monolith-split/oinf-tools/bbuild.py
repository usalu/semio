"""Plan + assemble the board crates into a scratch output tree: python3 bbuild.py <out>. OINF_ROOT = staged tree."""
import sys, os, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bplan2, bgen, bcrates

SNAP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf/snapshot-board"

def main(out):
    import btgen, bxfile, oinf_rs
    P = bplan2.plan('none')
    uc, ic, conf = bplan2.assignment(P)
    assert not conf, conf
    meta = bxfile.load_meta()
    modnames = {x for x in oinf_rs.LOCAL_ROOTS if x not in bgen.LOCAL_SEG}
    pool0, byfile0 = btgen.load_pool(SNAP, meta, modnames)
    extra = btgen.test_used_sets(pool0)
    crates, ic2, imports, defcfg, upward = bgen.build(P, uc, ic, extra_used=extra, tree=P['tree'])
    shutil.rmtree(out, ignore_errors=True)
    bgen.write_crates(crates, out)
    ext_roots = bgen.external_roots(P['mods'])
    T = btgen.build_tests(SNAP, meta, crates, imports, defcfg, ext_roots, modnames)
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    files = []
    for tg in T['targets']:
        path, text = btgen.gen_target(T, tg, crates, imports, defcfg, wdef, out, modnames, None)
        files.append((bcrates.CRATES[tg['crate']][0], path, text.count('\n')))
    P['T'] = T; P['testfiles'] = files; P['defcfg'] = defcfg; P['imports'] = imports
    return P, crates, upward

if __name__ == '__main__':
    P, crates, upward = main(sys.argv[1])
    for c in crates:
        print(c.id, len(c.items), 'items', c.text.count('\n'), 'lines', 'deps', sorted(bcrates.CRATES[k][0] for k in c.cross))
    for f in P['testfiles']: print('test', f[0], f[1][len(sys.argv[1]):][-60:], f[2])
    print('unreached', [(P['T']['pool'][k].tag, P['T']['pool'][k].it.kind, P['T']['pool'][k].it.name) for k in P['T']['unreached']][:20])
    print('upward refs', len(upward))
    seen = collections.Counter((a, b, c) for a, b, c in upward)
    for k, v in sorted(seen.items())[:60]: print(k)
