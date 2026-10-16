# 🗺️ Post-state model of one carve batch: which crate owns which file / item, the module path of every file inside its crate, the staging files that carry items split off a multi-owner file.
import os, re, sys, collections, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, model, registry, assign
import okcrates as C
R = ns.R; M = ns.M; S = ns.S
FE0F = '️'

def ident_of(key): return 'semio_framework_' + key.replace('-', '_')

def slug_of_mod(name): return 'lifted_' + name

class Staging:
    def __init__(self, src_file, crate, items, modname, path):
        self.src_file = src_file; self.crate = crate; self.items = items; self.modname = modname; self.path = path

class Plan:
    def __init__(self, batch, crates_model=None, relocate=()):
        self.batch = list(batch)
        self.relocate = set(relocate)
        self.w = ns.world()
        self.crates_model = crates_model or C.build()[0]
        self.node = {}          # crate -> node dir (abs)
        self.root_file = {}     # crate -> lib root file
        for k in self.batch:
            c = self.crates_model[k]
            self.node[k] = R + '/' + c['node']
            self.root_file[k] = self.node[k] + '/🦀️.rs'
        self.file_crate = {}    # file -> crate (main) for every world file owned by the batch
        self.staging = {}       # (src_file, crate) -> Staging
        self.moved_items = collections.defaultdict(list)    # src_file -> [(item, crate)] items that leave the file
        self.relocated_tests = []
        self.relocated_files = set()
        self.reloc_name = {}
        self._classify()

    def crate_of(self, f):
        c = model.crate_of_file(f, self.w)
        return c if c in self.batch else None

    def _classify(self):
        roots = {rf: k for k, rf in self.root_file.items()}
        def relocated(m):
            while m is not None:
                if m.file in self.relocate and m.key == m.file: return True
                m = m.parent
            return False
        for f, mod in self.w.files.items():
            if relocated(mod): self.relocated_files.add(f); continue
            c = roots.get(f) or model.crate_of_file(f, self.w)
            if c in self.batch: self.file_crate[f] = c
        for f in list(self.file_crate) + self._extra_split_sources():
            if f not in model.cof()['items']: continue
            fm = model.file_model(f)
            main = self.file_crate.get(f) or model.crate_of_file(f, self.w) or fm.main_crate()
            for it in fm.items:
                c = it['crate']
                if it['kind'] in ('mod', 'use', 'extern') or (it['kind'] is None and not it.get('forced')): continue
                if c and c != main and c in self.batch:
                    self.moved_items[f].append((it, c))
        for f, lst in self.moved_items.items():
            by = collections.defaultdict(list)
            for it, c in lst: by[c].append(it)
            modname = self.w.files[f].name or os.path.basename(os.path.dirname(f))
            for c, its in by.items():
                slug = self._slug(f)
                path = self.node[c] + '/📥️lifted/' + slug + '/🦀️.rs'
                if any(s.path == path for s in self.staging.values()):
                    path = self.node[c] + '/📥️lifted/' + os.path.basename(os.path.dirname(os.path.dirname(f))) + '-' + slug + '/🦀️.rs'
                self.staging[(f, c)] = Staging(f, c, its, slug_of_mod(modname), path)

    def _extra_split_sources(self):
        """files whose main crate is outside the batch but that hold items of a batch crate"""
        out = []
        for f in model.cof()['items']:
            if f in self.file_crate or not os.path.exists(f): continue
            fm = model.file_model(f)
            if any(it['crate'] in self.batch for it in fm.items): out.append(f)
        return out

    def _slug(self, f):
        return os.path.basename(os.path.dirname(f))

    def mod_chain(self, f):
        """names from the crate root to the module of file f, inside f's owning crate"""
        c = self.file_crate.get(f)
        if c is None: return None
        if f == self.root_file[c]: return []
        m = self.w.files[f]
        P = m.parent
        if P is None: return [m.name]
        pf = P.file
        if self.file_crate.get(pf) == c:
            return self.mod_chain(pf) + P.inline_chain() + [m.name]
        return [m.name]

    def location(self, mod, name=None, item=None):
        """post-state location of a module (name None) or of a def: (crate, [module names], name)"""
        f = mod.file
        c = self.file_crate.get(f)
        if item is not None and f in model.cof()['items']:
            fm = model.file_model(f)
            for x in fm.items:
                if x['start'] == item['start']:
                    ic = x['crate']
                    if ic and ic != (c or fm.main_crate()) and ic in self.batch:
                        st = self.staging.get((f, ic))
                        if st: return (ic, [st.modname], name)
                    break
        if c is None: return None
        chain = self.mod_chain(f)
        inl = [] if mod.file_module() is mod else mod.inline_chain()
        return (c, chain + inl, name)

if __name__ == '__main__':
    registry.load()
    p = Plan(sys.argv[1:] or ['os-store-child', 'os-store-ledger', 'os-store-replay', 'os-store-read'])
    for f, c in sorted(p.file_crate.items(), key=lambda kv: kv[1]):
        print(c, model.short(f)[-70:], p.mod_chain(f))
    for (f, c), st in p.staging.items(): print('staging', c, model.short(f)[-50:], [i['name'] for i in st.items][:6], st.path[-60:])
