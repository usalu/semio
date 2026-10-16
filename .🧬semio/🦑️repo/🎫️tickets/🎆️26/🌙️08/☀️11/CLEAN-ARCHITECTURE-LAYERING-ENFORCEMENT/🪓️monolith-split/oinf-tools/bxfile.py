"""Rewrite code outside the host crates (tests, plugins, renderer) to the decomposed host API, by receiver spec."""
import sys, os, json, collections, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bdec, bmap, dmap
from oinf_rs import *
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf/"

class M: pass

def specs():
    return {
     'BoardHost': bdec.Spec('BoardHost', 'Board', bmap.F, bmap.ORDER, bmap.COMPONENTS),
     'DagHost': bdec.Spec('DagHost', 'Dag', dmap.F, dmap.ORDER, dmap.COMPONENTS),
    }

def load_meta():
    out = {'BoardHost': ({}, collections.defaultdict(list)), 'DagHost': ({}, collections.defaultdict(list))}
    for w in ('board', 'dag'):
        for m in json.load(open(SC + f'b-meta-{w}.json', encoding='utf-8')):
            x = M()
            for k, v in m.items(): setattr(x, k, v)
            x.name = m['oldname']; x.newname = m['name']; x.var_mut = m['var_mut']
            if m['kind'] == 'method': out[m['host']][0][x.name] = x
            elif m['kind'] == 'pfn': out[m['host']][1][x.name].append(x)
    return out

class XCtx: pass

def rewrite(text, rel, host_ty, receivers, meta, strict=True):
    """receivers: list of receiver source strings (typed host_ty). Returns (new text, hazards, flags)."""
    spec = specs()[host_ty]
    methods, pfns = meta[host_ty]
    items, _ = chunk_file(text)
    toks = bdec.ctoks(text); starts = [t.a for t in toks]
    f = bdec.Fn()
    f.rel = rel; f.name = '<file>'; f.parent = None; f.trait = None; f.kind = 'xfile'; f.mode = 'owner'
    f.a = 0; f.b = len(text); f.fa = 0; f.fb = len(text)
    f.var = None; f.var_mut = True; f.has_self = False; f.var_param = None; f.life = None; f.implit = None
    f.fni = 0; f.po = f.pc = 0; f.params = []; f.bi0 = 0; f.bi1 = len(toks) - 1; f.has_body = True; f.ti = 0
    f.newname = '<file>'; f.comps = []
    f.aliases = [tuple(t.s for t in bdec.ctoks(r)) for r in receivers]
    cache = {rel: (text, toks, starts)}
    ctx = XCtx(); ctx.spec = spec; ctx.methods = methods; ctx.pfns = pfns; ctx.cache = cache; ctx.hazards = []; ctx.fns = [f]
    bdec.analyze(spec, [f], cache, methods, pfns)
    E = bdec.fn_edits(ctx, f)
    return bdec.apply_edits(text, E), ctx.hazards, f.flags
