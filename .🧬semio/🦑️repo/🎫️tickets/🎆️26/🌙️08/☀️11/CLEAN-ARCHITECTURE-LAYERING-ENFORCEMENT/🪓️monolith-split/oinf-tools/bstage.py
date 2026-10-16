"""Stage one host decomposition: python3 bstage.py <board|dag> <dst>. OINF_ROOT must name the source tree."""
import sys, os, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bdec, bmap, dmap
from modtree import INFROOT
HOSTS = {
 'board': lambda: bdec.Spec('BoardHost', 'Board', bmap.F, bmap.ORDER, bmap.COMPONENTS, constructors={'new', 'new_normal'}, ext={'BoardHostRetirement': ('self', '.', 'host')}),
 'dag': lambda: bdec.Spec('DagHost', 'Dag', dmap.F, dmap.ORDER, dmap.COMPONENTS, constructors={'default_demo', 'from_host_snapshot', 'from_host_snapshot_without_layout', 'from_host_snapshot_with_layout'}, ext={'DagSession': [('self', '.', 'state', '.', 'borrow', '(', ')', '.', 'host'), ('self', '.', 'state', '.', 'borrow_mut', '(', ')', '.', 'host'), ('inner', '.', 'host'), ('g', '.', 'host')]}),
}
MANUAL = {
 'new_normal': [("let mut host = Self { port_mode: GraphPortMode::Normal, ..Self::default() };\n        host.focus.selection_options.select_handles = false;", "let mut host = Self::default();\n        host.style.port_mode = GraphPortMode::Normal;\n        host.focus.selection_options.select_handles = false;")],
}
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"
EXT_FILES = SC + '/board-users.txt'
R = "/Users/ueli/Documents/semio/"

def universe(mods, exclude_ty):
    names = set()
    for rel, (text, items) in mods.items():
        for it in items:
            if it.kind in ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'union', 'macro_rules') and it.name: names.add(it.name)
            if it.kind == 'impl' and it.impl_ty != exclude_ty:
                bs = text.index('{', it.start)
                sub, _ = bdec.chunk_file(text[bs + 1: it.end - 1])
                for s in sub:
                    if s.name: names.add(s.name)
    return names

def ext_api(spec, mods):
    import re
    fns, cache = bdec.collect(spec, mods)
    mnames = {f.name for f in fns if f.kind == 'method'}
    files = [l.strip() for l in open(EXT_FILES, encoding='utf-8', errors='replace')]
    files = [f for f in files if '♾️infinite' not in f and '🎫️tickets' not in f]
    used = set()
    for f in files:
        t = open(R + f, encoding='utf-8', errors='replace').read()
        if spec.ty not in t: continue
        for m in re.finditer(r'\.([a-z_][a-z0-9_]*)\s*\(', t):
            if m.group(1) in mnames: used.add(m.group(1))
    return used

def main(which, dst):
    bdec.MANUAL.update(MANUAL)
    spec = HOSTS[which]()
    _mods = bdec.load_mods(INFROOT)
    spec.retire = which == 'dag'
    spec.taken = universe(_mods, spec.ty)
    spec.ext_api = ext_api(spec, _mods)
    ctx, out = bdec.run_decompose(spec, INFROOT, dst)
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(INFROOT, dst, symlinks=True, ignore=shutil.ignore_patterns('node_modules'))
    for rel, t in out.items():
        open(dst + '/' + rel, 'w', encoding='utf-8').write(t)
    import json
    meta = [dict(host=spec.ty, rel=f.rel, name=f.newname, oldname=f.name, kind=f.kind, mode=f.mode, comps=f.comps, parent=f.parent, forced=f.forced, var_mut=bool(f.var_mut), var_param=f.var_param, has_self=f.has_self, nparams=len(f.params) - (1 if f.has_self else 0)) for f in ctx.fns]
    json.dump(meta, open(dst + '/../b-meta-' + which + '.json', 'w', encoding='utf-8'), ensure_ascii=False)
    return ctx
if __name__ == '__main__':
    ctx = main(sys.argv[1], sys.argv[2])
    print('fns', len(ctx.fns), 'hazards', len(ctx.hazards))
    for f, g, why in ctx.hazards[:80]: print(f.rel[-40:], f.name, '->', g.name, why)
