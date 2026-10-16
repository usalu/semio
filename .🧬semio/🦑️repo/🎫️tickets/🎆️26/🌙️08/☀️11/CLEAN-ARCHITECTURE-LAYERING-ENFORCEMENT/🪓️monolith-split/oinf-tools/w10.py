import sys, shutil; sys.path.insert(0,'.')
from wrewrite import *
SCRO = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
SNAP = SCRO + "/snapshot-infinite"
import wengine
wengine.INF = SNAP
def run_phase1(outdir):
    from wtests import load_tests, resolve_ambiguous_tests
    allfns, files = load_world_from(SNAP)
    defs, unres = resolve_calls(allfns)
    apply_hints(allfns, files)
    fixpoint(allfns, defs)
    assign_modes(allfns)
    tfns, tfiles = load_tests(SNAP)
    for f in tfns: f.mode = 'host' if f.has_param else None
    resolve_calls(allfns + tfns)
    miss = resolve_ambiguous_tests(tfns, tfiles)
    assert not miss, miss
    for f in tfns:
        for c in f.calls:
            if not c['targets']:
                assert c['name'] in ('let', 'for', 'offered'), c['name']
    files.update(tfiles)
    both = allfns + tfns
    edits, stats = make_edits_both(allfns, tfns, files)
    root = SNAP + '/🌍️world/🦀️.rs'
    text, toks = files[root]
    items, _ = chunk_file(text)
    st = [it for it in items if it.kind == 'struct' and it.name == 'World3dState'][0]
    # State impl `new`
    body_start = None
    impl = [it for it in items if it.kind == 'impl' and it.impl_ty == 'World3dState' and it.impl_trait is None][0]
    bs = text.index('{', impl.start)
    inner = text[bs+1:impl.end-1]
    sub, _ = chunk_file(inner)
    newfn = [s for s in sub if s.kind == 'fn' and s.name == 'new'][0]
    class NF: pass
    nf = NF(); nf.start = newfn.start + bs + 1; nf.end = newfn.end + bs + 1
    host, comps, impls, newtxt = build_state_defs(text, st, nf)
    edits[root].append((st.start, st.end, host + '\n\n' + comps + '\n\n' + impls))
    edits[root].append((nf.start, nf.end, newtxt.lstrip()))
    out = {}
    for path, (t, tk) in files.items():
        e = edits.get(path, [])
        nt = apply_edits(t, e) if e else t
        if path.endswith('🎛️frame/🦀️.rs'): nt = fix_manual(nt, 'world3d_tool_run_trace_cursors')
        out[path] = nt
    # write into outdir copy
    shutil.rmtree(outdir, ignore_errors=True)
    shutil.copytree(SNAP, outdir, symlinks=True)
    for path, nt in out.items():
        rel = path[len(SNAP):]
        open(outdir + rel, 'w', encoding='utf-8').write(nt)
    return allfns, files, stats

def load_world_from(snap):
    import wengine as W
    W.INF = snap
    allfns = []; files = {}
    for dp, dn, fnn in os.walk(snap + '/🌍️world'):
        if '🧪️tests' in dp or 'node_modules' in dp: continue
        for f in sorted(fnn):
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                text = open(p, encoding='utf-8').read()
                fns, toks = collect_fns(p, text)
                files[p] = (text, toks)
                for fn, i, pc in fns:
                    analyze_fn(fn, toks, pc, text)
                    allfns.append(fn)
    return allfns, files

if __name__ == '__main__':
    allfns, files, stats = run_phase1(SCRO + '/stage-w1')
    print(stats)
