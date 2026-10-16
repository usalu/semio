import sys, os, re, collections, importlib, fnmatch
sys.path.insert(0, ".")
from scan import *
from chunks import chunk_of, region_map

TOPREG = re.compile(r'^//#(end)?region[ \t]+(\S.*?)\s*$', re.M)
TOPREG_LINE = re.compile(r'^//#(end)?region\b.*$')

def snake(s):
    s = re.sub(r'[^0-9A-Za-z]+', '_', s).strip('_')
    return s.lower()

def slug_from_dir(path):
    return os.path.basename(os.path.dirname(path))

class _FirstItem:
    def __init__(self, f): self.line0 = 1

def tatoms_ids(tatoms, f):
    if not hasattr(f, "_tids"): f._tids = set(id(a) for a in tatoms[f.path])
    return f._tids

GLUE = LIBROOT
def skip_file(f):
    return f.path == GLUE or not f.path.startswith(MCP + "/")

class Unit:
    def __init__(self, it, atom, crate, fileidx):
        self.it = it; self.atom = atom; self.crate = crate; self.fileidx = fileidx
        self.lead = ""; self.regions = (); self.main = (it is atom.item) if atom is not None else True
        self.out = None
    @property
    def src(self): return self.it.file.src[self.it.start:self.it.end]
    @property
    def cfg(self): return self.it.cfg

class OutFile:
    def __init__(self, crate, path, modname, kind, srcfile=None):
        self.crate = crate; self.path = path; self.modname = modname; self.kind = kind
        self.srcfile = srcfile; self.units = []; self.header = ""; self.mount_cfg = ""
        self.mount_pub = "pub"
        self.test_parent = None
    def __repr__(self): return f"<Out {self.kind} {self.modname} crate={self.crate} {os.path.relpath(self.path, MCP)}>"

def header_doc(src):
    lines = src.split("\n")
    out = []; i = 0
    while i < len(lines):
        ln = lines[i]
        if ln.startswith("//!"): out.append(ln); i += 1
        elif ln.strip() == "" and out: i += 1
        else: break
    return "\n".join(out), "\n".join(lines[i:]) if False else None

def clean_lead(lead, first=False):
    out = []
    for ln in lead.split("\n"):
        if TOPREG_LINE.match(ln): continue
        if first and ln.startswith("//!"): continue
        out.append(ln)
    s = "\n".join(out)
    s = re.sub(r'\n{3,}', '\n\n', s).strip("\n")
    return s

class Layout:
    def __init__(self, planmod):
        self.plan = importlib.import_module(planmod)
        self.w = World()
        w = self.w
        self.flags = {id(a): flags_for(w, a) for a in w.atoms}
        self.names = [n for n, _ in self.plan.CRATES]
        self.crate_of = {}
        self.chunk = {}
        over = getattr(self.plan, "ATOM_OVERRIDE", {})
        for a in w.atoms:
            if id(a) in w.mount_atoms: continue
            if self.flags[id(a)] and is_test_file(a.file, w.files): continue
            if skip_file(a.file): continue
            ck = chunk_of(a); self.chunk[id(a)] = ck
            key = f"{ck}::{a.name}"
            tgt = over.get(key, over.get(a.name))
            if tgt is not None: ci = self.names.index(tgt)
            else:
                ci = None
                for i, (n, pats) in enumerate(self.plan.CRATES):
                    if any(fnmatch.fnmatchcase(ck, p) for p in pats): ci = i; break
            if ci is None:
                self.unassigned = getattr(self, "unassigned", set()); self.unassigned.add(ck); continue
            self.crate_of[id(a)] = ci
        if getattr(self, "unassigned", None): raise SystemExit("unassigned chunks: %r" % sorted(self.unassigned))
        # items per file with lead
        self.fileorder = {p: i for i, p in enumerate(sorted(w.files))}
        self.units = []
        self.unit_of_item = {}
        for p, f in sorted(w.files.items()):
            if is_test_file(f, w.files) or skip_file(f): continue
            its = sorted([x for x in w.allitems if x.file is f], key=lambda x: x.start)
            spans = [s for s in region_map(f)]
            prev_end = 0
            first = True
            for it in its:
                lead = f.src[prev_end:it.start]
                prev_end = it.end
                u = None
                atom = self.atom_of(it)
                if atom is not None and id(atom) in self.crate_of:
                    u = Unit(it, atom, self.crate_of[id(atom)], self.fileorder[p])
                    u.lead = clean_lead(lead, first=first)
                    u.regions = self.region_path(f, it)
                    self.units.append(u); self.unit_of_item[id(it)] = u
                first = False
            f.tail = f.src[prev_end:]
        self.build_outputs()
        self.add_tests()
        self.compute_empty_regions()

    def add_tests(self):
        w = self.w
        tfiles = [f for f in w.files.values() if is_test_file(f, w.files) and not skip_file(f)]
        tatoms = collections.defaultdict(list)
        for a in w.atoms:
            if id(a) in w.mount_atoms: continue
            if a.file in tfiles: tatoms[a.file.path].append(a)
        # per-file need and links
        need = {}; links = collections.defaultdict(set)
        fileof = {}
        anchors = getattr(self.plan, "TEST_ANCHOR", {})
        anchored = set()
        for f in tfiles:
            rel = os.path.relpath(f.path, MCP)
            if rel in anchors:
                anchored.add(f.path)
                for a in tatoms[f.path]: self.crate_of[id(a)] = self.names.index(anchors[rel])
                continue
            for a in tatoms[f.path]: fileof[id(a)] = f.path
        for f in tfiles:
            if f.path in anchored: continue
            mx = -1
            for a in tatoms[f.path]:
                for t in w.atom_refs(a, test=True):
                    if id(t) in self.crate_of: mx = max(mx, self.crate_of[id(t)])
                    elif id(t) in fileof and fileof[id(t)] != f.path:
                        links[f.path].add(fileof[id(t)]); links[fileof[id(t)]].add(f.path)
            need[f.path] = mx
        # groups
        parent = {f.path: f.path for f in tfiles}
        def find(x):
            while parent[x] != x: parent[x] = parent[parent[x]]; x = parent[x]
            return x
        for a, bs in links.items():
            for b in bs: parent[find(a)] = find(b)
        gmax = collections.defaultdict(lambda: -1)
        for f in tfiles:
            if f.path not in anchored: gmax[find(f.path)] = max(gmax[find(f.path)], need[f.path])
        pins = getattr(self.plan, "TEST_PIN", {})
        usednames = collections.defaultdict(set)
        for o in self.outs: usednames[o.crate].add(o.modname)
        self.test_outs = []
        for f in sorted(tfiles, key=lambda f: f.path):
            rel = os.path.relpath(f.path, MCP)
            ci = gmax[find(f.path)] if f.path not in anchored else self.names.index(anchors[rel])
            if f.modpath[0] == "integration": ci = self.names.index("runtime")
            if rel in pins: ci = self.names.index(pins[rel])
            if ci < 0:
                raise SystemExit(f"test file without crate need: {rel}")
            base = snake(f.modpath[-2] + "_" + f.modpath[-1]) if len(f.modpath) > 2 else snake(f.modpath[-1])
            nm = base; k = 2
            while nm in usednames[ci]: nm = f"{base}_{k}"; k += 1
            usednames[ci].add(nm)
            o = OutFile(ci, f.path, nm, "test", srcfile=f)
            o.primary = True; o.mount_cfg = "#[cfg(test)]"
            if f.modpath[0] == "integration": o.kind = "integration"; ci = self.names.index("runtime"); o.crate = ci
            its = sorted([x for x in w.allitems if x.file is f], key=lambda x: x.start)
            prev_end = 0; first = True
            for it in its:
                lead = f.src[prev_end:it.start]; prev_end = it.end
                atom = self.atom_of(it)
                if atom is not None and id(atom) in tatoms_ids(tatoms, f):
                    self.crate_of[id(atom)] = ci
                    u = Unit(it, atom, ci, self.fileorder[f.path])
                    u.lead = clean_lead(lead, first=first); u.regions = self.region_path(f, it)
                    u.out = o; o.units.append(u); self.units.append(u)
                first = False
            f.tail = f.src[prev_end:]
            self.outs.append(o); self.test_outs.append(o)

    def compute_empty_regions(self):
        self.empty_regions = collections.defaultdict(list)
        kept = collections.defaultdict(list)
        for u in self.units: kept[u.it.file.path].append(u.it.line0)
        for p, f in self.w.files.items():
            if skip_file(f): continue
            self.region_path(f, f.__dict__.get("_dummy") or _FirstItem(f))
            for (s, e, nm, d) in f._topspans:
                if not any(s < ln <= e for ln in kept[p]):
                    self.empty_regions[p].append(nm)

    def atom_of(self, it):
        w = self.w
        if not hasattr(self, "_atom_by_item"):
            self._atom_by_item = {}
            for a in w.atoms:
                self._atom_by_item[id(a.item)] = a
                for im in a.impls: self._atom_by_item[id(im)] = a
        return self._atom_by_item.get(id(it))

    def region_path(self, f, it):
        if not hasattr(f, "_topspans"):
            stack = []; spans = []
            for m in TOPREG.finditer(f.src):
                ln = f.src.count('\n', 0, m.start()) + 1
                if m.group(1):
                    if stack:
                        nm, st = stack.pop(); spans.append((st, ln, nm, len(stack)))
                else: stack.append((m.group(2), ln))
            f._topspans = sorted(spans, key=lambda s: (s[3], s[0]))
        path = [nm for (s, e, nm, d) in f._topspans if s < it.line0 <= e]
        return tuple(path)

    def build_outputs(self):
        w = self.w
        per = collections.defaultdict(lambda: collections.defaultdict(list))
        for u in self.units:
            per[u.it.file.path][u.crate].append(u)
        self.outs = []
        self.out_of = {}
        groups = {}
        usednames = collections.defaultdict(set)
        merges = []
        def newname(c, base, path):
            nm = base
            if nm in usednames[c]: nm = nm + "_" + snake(slug_from_dir(path))
            usednames[c].add(nm); return nm
        for p in sorted(per):
            f = w.files[p]
            share = {c: sum(x.it.text.count("\n")+1 for x in us) for c, us in per[p].items()}
            primary = max(share, key=lambda c: (share[c], -c))
            for c, us in sorted(per[p].items()):
                rel = os.path.relpath(p, MCP)
                if c == primary:
                    o = OutFile(c, p, "", "prod", srcfile=f)
                    o.modname = newname(c, f.modpath[-1] if len(f.modpath) > 1 else "root_items", p)
                    o.mount_cfg = f.attrs; o.primary = True
                    o.units = list(us)
                    self.outs.append(o); self.out_of[(p, c)] = o
                    continue
                spec = self.plan.PIECES.get((rel, self.names[c]))
                if spec is None: raise SystemExit(f"no PIECES entry for ({rel}, {self.names[c]})")
                if isinstance(spec, str):
                    merges.append((p, c, MCP + "/" + spec[1:], us)); continue
                emoji, kebab = spec
                gk = (c, kebab)
                if gk in groups:
                    o = groups[gk]
                    o.units += us
                    self.out_of[(p, c)] = o
                else:
                    d = os.path.join(os.path.dirname(p), emoji + "\ufe0f" + kebab)
                    o = OutFile(c, os.path.join(d, "🦀️.rs"), "", "prod", srcfile=f)
                    o.modname = newname(c, kebab.replace("-", "_"), d)
                    o.mount_cfg = f.attrs; o.primary = False; o.piece_emoji = emoji; o.kebab = kebab
                    o.units = list(us)
                    groups[gk] = o
                    self.outs.append(o); self.out_of[(p, c)] = o
        for (p, c, target, us) in merges:
            o = self.out_of[(target, c)]
            o.units += us
            self.out_of[(p, c)] = o
        for o in self.outs:
            for u in o.units: u.out = o
        for o in self.outs:
            o.units.sort(key=self.unit_key)
        self.outs = [o for o in self.outs if o.units]

    def unit_key(self, u):
        return (u.fileidx, u.it.start)

def _unused_piece_name(lay, f, crate):
    """(emoji, folder-kebab, module name) for a non-primary piece of source file f in crate."""
    base = lay.plan.PIECES.get((os.path.relpath(f.path, MCP), lay.names[crate]))
    if base is None:
        raise SystemExit(f"no PIECES entry for ({os.path.relpath(f.path, MCP)}, {lay.names[crate]})")
    emoji, kebab = base
    return (emoji, emoji + "️" + kebab, kebab.replace("-", "_"))

if __name__ == "__main__":
    pass
