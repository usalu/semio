# 🧭 A2 library: runs the WP2/WP3 carve pipeline for the WP5/WP6 batch with (1) the re-anchored item assignment, (2) predicted post-state paths for crates that are planned but not carved yet, (3) first-party registry entries for every planned crate.
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import mycof
mycof.install()
import ns, model, registry, assign, plan as P, rewrite as RW, carve, widen
import okcrates as C
R = ns.R
CRATES = C.build()[0]

def ident_of(key): return 'semio_framework_' + key.replace('-', '_')

_orig_member_uses = widen.member_uses
def member_uses(text):
    out = set(_orig_member_uses(text))
    toks = [t for t in carve.tokenize(text) if t[0] not in ('ws', 'comment')]
    n = len(toks)
    for i, t in enumerate(toks):
        if t[0] == 'id' and i >= 2 and toks[i - 1][1] == ':' and toks[i - 2][1] == ':':
            nx = toks[i + 1][1] if i + 1 < n else ''
            nx2 = toks[i + 2][1] if i + 2 < n else ''
            if nx == '(' or (nx == ':' and nx2 == ':') or (t[1].isupper() and len(t[1]) > 1 and t[1] == t[1].upper()):
                out.add(t[1])
    return out
widen.member_uses = member_uses

def register_all():
    registry.load()
    for k, c in CRATES.items():
        registry.add_crate(ident_of(k), R + '/' + c['node'] + '/🦀️.rs')

class PredictPlan(P.Plan):
    def __init__(self, batch, crates_model=None, relocate=()):
        super().__init__(batch, crates_model=crates_model, relocate=relocate)
        self.proot = {k: R + '/' + c['node'] + '/🦀️.rs' for k, c in CRATES.items()}

    def pchain(self, f, pc):
        if f == self.proot[pc]: return []
        m = self.w.files[f]; Pm = m.parent
        if Pm is None: return [m.name]
        if model.crate_of_file(Pm.file, self.w) == pc: return self.pchain(Pm.file, pc) + Pm.inline_chain() + [m.name]
        return [m.name]

    def location(self, mod, name=None, item=None):
        r = super().location(mod, name, item)
        if r is not None: return r
        f = mod.file
        main = model.crate_of_file(f, self.w) if f in self.w.files else None
        if item is not None and f in model.cof()['items'] and os.path.exists(f):
            fm = model.file_model(f)
            for x in fm.items:
                if x['start'] == item['start']:
                    ic = x['crate']
                    if ic and ic in CRATES and ic not in self.batch and ic != main:
                        modname = self.w.files[f].name or os.path.basename(os.path.dirname(f))
                        return (ic, ['lifted_' + modname], name)
                    break
        if main in CRATES and main not in self.batch:
            chain = self.pchain(f, main)
            inl = [] if mod.file_module() is mod else mod.inline_chain()
            return (main, chain + inl, name)
        return None

def solve(crates, auto=True):
    register_all()
    carve.P.Plan = PredictPlan
    return carve.solve(crates, auto=auto)
