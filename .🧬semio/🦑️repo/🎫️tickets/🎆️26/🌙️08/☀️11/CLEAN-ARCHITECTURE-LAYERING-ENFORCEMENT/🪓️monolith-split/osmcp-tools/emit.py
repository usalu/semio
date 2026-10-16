import sys, os, re, collections, json
sys.path.insert(0, ".")
from layout import *

REPO = "/Users/ueli/Documents/semio"

def crate_pkg(lay, ci):
    n = lay.names[ci]
    return "semio-framework-os-mcp" if n == "runtime" else "semio-framework-os-mcp-" + n

def crate_ident(lay, ci):
    return crate_pkg(lay, ci).replace("-", "_")

def vis_rank(v):
    v = v.replace(" ", "")
    if v in ("", "pub(self)"): return 0
    if v == "pub(super)" or v.startswith("pub(in"): return 1
    if v == "pub(crate)": return 2
    return 3

def rank_vis(r): return ["", "pub(super)", "pub(crate)", "pub"][r]

HEADVIS = re.compile(r'(?P<pre>(?:\s*#\[(?:[^\[\]]|\[[^\]]*\])*\])*\s*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?P<kw>(?:(?:default|unsafe|async|const|extern(?:\s+"[^"]*")?)\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\b)', re.S)

MEMBER = re.compile(r'(?P<attrs>(?:#\[(?:[^\[\]]|\[[^\]]*\])*\]\s*)*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?:(?:(?:const|async|unsafe|extern(?:\s+"[^"]*")?)\s+)*fn\s+(?P<fn>[A-Za-z_]\w*)|const\s+(?P<const>[A-Z_0-9]+)\s*:|(?P<field>[A-Za-z_]\w*)\s*:(?!:))')

def parse_members(code, is_impl, is_trait_impl):
    """{name: dict(kind, vis, decl, vspan)} positions relative to code."""
    out = {}
    if is_trait_impl: return out
    n = len(code)
    m = re.match(r'\s*(?:#\[(?:[^\[\]]|\[[^\]]*\])*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+\w+\s*(?:<[^{(;]*>)?\s*\(', code)
    if m and not is_impl:
        j = m.end(); d = 1; start = j; idx = 0
        while j < n and d:
            c = code[j]
            if c in "([{<": d += 1
            elif c in ")]}>":
                d -= 1
                if d == 0:
                    seg = (start, j)
                    mm = re.compile(r'\s*(?:#\[[^\]]*\]\s*)*').match(code, seg[0])
                    ds = mm.end() if mm else seg[0]
                    vm = re.compile(r'pub(?:\s*\([^)]*\))?\s*').match(code, ds)
                    if code[seg[0]:seg[1]].strip():
                        out[str(idx)] = dict(kind="tuple", vis=(vm.group(0).strip() if vm else ""), decl=ds, vspan=((vm.start(), vm.end()) if vm else None))
                    break
            elif c == "," and d == 1:
                seg = (start, j)
                mm = re.compile(r'\s*(?:#\[[^\]]*\]\s*)*').match(code, seg[0])
                ds = mm.end() if mm else seg[0]
                vm = re.compile(r'pub(?:\s*\([^)]*\))?\s*').match(code, ds)
                if code[seg[0]:seg[1]].strip():
                    out[str(idx)] = dict(kind="tuple", vis=(vm.group(0).strip() if vm else ""), decl=ds, vspan=((vm.start(), vm.end()) if vm else None))
                idx += 1; start = j + 1
            j += 1
        return out
    depth = par = br = 0
    pos = 0
    while pos < n:
        c = code[pos]
        if c == "{": depth += 1
        elif c == "}": depth -= 1
        elif c == "(": par += 1
        elif c == ")": par -= 1
        elif c == "[": br += 1
        elif c == "]": br -= 1
        if depth == 1 and par == 0 and br == 0 and code[pos-1] in "{;,}\n\t " and pos > 0:
            mm = MEMBER.match(code, pos)
            if mm and (mm.group("fn") or mm.group("const") or (mm.group("field") and not is_impl)):
                name = mm.group("fn") or mm.group("const") or mm.group("field")
                kind = "fn" if mm.group("fn") else ("const" if mm.group("const") else "field")
                vs = (mm.start("vis"), mm.end("vis")) if mm.group("vis") else None
                out[name] = dict(kind=kind, vis=(mm.group("vis") or "").strip(), decl=mm.start("attrs") + len(mm.group("attrs")), vspan=vs)
                pos = mm.end(); continue
        pos += 1
    return out

SILENT_EXT = {
    "Write": {"write_all", "write", "flush", "write_fmt", "write_str"}, "Read": {"read", "read_to_string", "read_exact", "read_to_end", "bytes"},
    "BufRead": {"lines", "read_line", "fill_buf"}, "Hasher": {"finish", "write"}, "Hash": {"hash"}, "FromStr": {"from_str"},
    "TryFrom": {"try_from"}, "TryInto": {"try_into"}, "FromIterator": {"from_iter"}, "Borrow": {"borrow"}, "AsRef": {"as_ref"},
    "SinkExt": {"send", "close", "flush"}, "StreamExt": {"next", "map", "filter", "split", "for_each", "collect", "take"},
    "OsStrExt": {"as_bytes"}, "PermissionsExt": {"mode", "set_mode", "from_mode"}, "CommandExt": {"pre_exec"}, "AsRawFd": {"as_raw_fd"}, "FromRawFd": {"from_raw_fd"},
    "FileExt": {"read_at", "write_at"}, "OpenOptionsExt": {"mode", "custom_flags"}, "MetadataExt": {"mode", "uid"},
}

class Emitter:
    def __init__(self, lay):
        self.lay = lay; self.w = lay.w
        self.byatom_out = {}
        for o in lay.outs:
            for u in o.units:
                if u.main: self.byatom_out[id(u.atom)] = o
        self.out_names = {id(o): {u.atom.name for u in o.units if u.main} for o in lay.outs}
        self.scans = {}
        self.edits = collections.defaultdict(list)
        self.need_vis = {}
        self.member_need = collections.defaultdict(dict)
        self.wanted = {}
        self.extra_imports = collections.defaultdict(list)   # out id -> [(atom)]
        self.ext_roots = collections.defaultdict(set)         # crate -> ext crate names
        self.ext_roots_test = collections.defaultdict(set)
        self._mem = {}
        self.dyn_imports = {}
        self.mod_wanted = collections.defaultdict(dict)
        self.mod_prod = collections.defaultdict(set)
        self.facade_flat = {}
        self.prod_need = collections.defaultdict(set)
        self.traits = [a for a in self.w.atoms if a.kind == "trait" and id(a) in lay.crate_of]
        self.trait_methods = {}
        for t in self.traits:
            ms = set(re.findall(r'\bfn\s+([A-Za-z_]\w*)', t.item.text))
            self.trait_methods[id(t)] = ms

    def out_of_atom(self, a): return self.byatom_out.get(id(a))

    def target_atom(self, atoms):
        cand = [a for a in atoms if id(a) in self.lay.crate_of and self.out_of_atom(a) is not None]
        return cand[0] if cand else None

    sandbox = False
    def path_of(self, o, to, name):
        return self.mod_path(o, to) + "::" + name

    def mod_path(self, o, to):
        if o.kind == "integration" and to.crate == o.crate and not self.sandbox:
            self.facade_flat[id(to)] = to
            return "semio_framework_os_mcp"
        if self.sandbox: return f"crate::{crate_ident(self.lay, to.crate)}::{to.modname}"
        if to.crate == o.crate: return f"crate::{to.modname}"
        return f"{crate_ident(self.lay, to.crate)}::{to.modname}"

    def crate_prefix(self, o, ci):
        if self.sandbox: return f"crate::{crate_ident(self.lay, ci)}"
        return crate_ident(self.lay, ci)

    def members_of(self, t):
        if id(t) in self._mem: return self._mem[id(t)]
        res = {}
        for k, it in enumerate([t.item] + t.impls):
            if it.kind in ("enum", "trait") or it.text.lstrip().startswith("dyn_enum_close"): 
                if it.kind != "enum" or True:
                    if it.kind in ("enum", "trait"): continue
            for name, d in parse_members(it.text, it.kind == "impl", bool(it.impl_trait)).items():
                res.setdefault(name, []).append((it, d))
        self._mem[id(t)] = res
        return res

    def run(self):
        lay = self.lay
        for o in lay.outs:
            for u in o.units:
                full = scan_item(self.w, u.it)
                utest = any("cfg(test)" in c.replace(" ", "") for c in u.it.cfg)
                prod = [] if utest else scan_item(self.w, u.it, prod_only=True)
                prodids = set(id(a) for r in prod for a in r["atoms"])
                for r in full: r["testonly"] = not any(id(a) in prodids for a in r["atoms"])
                self.scans[id(u)] = full
                u._prodids = prodids
        self.refnames = {}
        for o in lay.outs:
            names = set()
            for u in o.units:
                for r in self.scans[id(u)]:
                    t = self.target_atom(r["atoms"])
                    if t is not None and self.out_of_atom(t) is not o: names.add(t.name)
            self.refnames[id(o)] = names
        # dyn_enum_close special: extra refs from trait
        for o in lay.outs:
            wanted = collections.defaultdict(dict)
            for u in o.units:
                refs = list(self.scans[id(u)])
                if "dyn_enum_close" in u.it.text:
                    for r in list(refs):
                        for a in r["atoms"]:
                            if a.kind == "trait":
                                self.dyn_imports.setdefault(id(o), [])
                                tro = self.out_of_atom(a)
                                if (tro, a.name) not in self.dyn_imports[id(o)]: self.dyn_imports[id(o)].append((tro, a.name))
                                for it in [a.item]:
                                    for rr in scan_item(self.w, it):
                                        refs.append(dict(rr, start=-1, end=-1, extra=True))
                for r in refs:
                    t = self.target_atom(r["atoms"])
                    if t is None: continue
                    if self.out_of_atom(t) is o: continue
                    if self.qualified_keep(o, u, r, t, self.out_of_atom(t)):
                        if not r.get("testonly"): self.mod_prod[id(o)].add(self.out_of_atom(t).modname)
                        continue
                    wanted[t.name][id(t)] = t
                    if not r.get("testonly") and not r.get("extra"): self.prod_need[id(o)].add(id(t))
                    elif r.get("extra"): self.prod_need[id(o)].add(id(t))
                # trait method-scope imports
                meths = set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\s*\(', u.it.text))
                for tr in self.traits:
                    if meths & self.trait_methods[id(tr)]:
                        if self.out_of_atom(tr) is o: continue
                        vis = self.w.lookup(u.it.file, tr.name, frm=u.it.file)
                        if any(x.kind == "atom" and x.obj is tr for x in vis) or self.glob_sees(u.it.file, tr):
                            wanted[tr.name][id(tr)] = tr
                            self.prod_need[id(o)].add(id(tr))
                u._refs = refs
            self.wanted[id(o)] = wanted
        for o in lay.outs:
            bad = self.bad_names(o)
            o.bad = bad
            for u in o.units:
                for r in u._refs:
                    t = self.target_atom(r["atoms"])
                    if t is None: continue
                    to = self.out_of_atom(t)
                    if to is not o:
                        need = 3 if to.crate != o.crate else 2
                        self.need_vis[id(t)] = max(self.need_vis.get(id(t), 0), need)
                        self.note_members(u, t, need)
                    if r["start"] < 0: continue
                    segs = r["segs"]; ix = r["ix"]
                    foreign_bad = to is not o and t.name in bad
                    if not foreign_bad and self.qualified_keep(o, u, r, t, to):
                        self.mod_wanted[id(o)][to.modname] = to
                        continue
                    if u.atom.kind == "mod" and u.main:
                        if to is o: continue
                        foreign_bad = True
                        if any(s0 <= r["start"] < e0 for (s0, e0) in self.use_spans(u)): continue
                    if foreign_bad:
                        repl = self.path_of(o, to, t.name)
                    elif ix > 0:
                        repl = t.name
                    else:
                        continue
                    tail = "::".join(segs[ix+1:])
                    if tail: repl += "::" + tail
                    self.edits[id(u)].append((r["start"], r["end"], repl))
        self.crate_deps = collections.defaultdict(set)
        self.crate_deps_dev = collections.defaultdict(set)
        for o in lay.outs:
            for name, d in self.wanted[id(o)].items():
                for t in d.values():
                    to = self.out_of_atom(t)
                    if to.crate == o.crate: continue
                    if o.kind in ("test", "integration") or id(t) not in self.prod_need[id(o)]: self.crate_deps_dev[o.crate].add(to.crate)
                    else: self.crate_deps[o.crate].add(to.crate)
        self.compute_testkit()
        self.alloc_users = set(o.crate for o in lay.outs for u in o.units if "test_allocation" in u.it.text)
        fci = lay.names.index("foundation")
        if self.alloc_users:
            for ci in self.alloc_users:
                if ci != fci: self.testkit_users[fci].add(ci)
        self.fixture_outs = set()
        INC = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')
        for o in lay.outs:
            if o.kind not in ("test", "integration"): continue
            for u in o.units:
                src = u.src
                for m in INC.finditer(src):
                    lit = m.group(1)
                    absf = os.path.normpath(os.path.join(os.path.dirname(u.it.file.path), lit))
                    relp = os.path.relpath(absf, self.pkg_dir_of(o.crate))
                    self.edits[id(u)].append((m.start(), m.end(), f'fixture_text("{relp}")'))
                    self.fixture_outs.add(id(o))
        # nested `use` statements (fn bodies, inline modules): rewrite crate-relative paths to canonical ones
        from uses import parse_use_tree, USE_RE
        for o in lay.outs:
            for u in o.units:
                text = u.it.text
                for m in USE_RE.finditer(text):
                    paths, j = parse_use_tree(text, m.end())
                    end = text.find(";", j) + 1 if text[j:j+1] != ";" else j + 1
                    byprefix = collections.OrderedDict(); ok = True
                    for p, alias in paths:
                        if p[0] not in ("crate", "super", "self"): ok = False; break
                        if p[-1] == "*": ok = False; break
                        res = resolve_ix(self.w, u.it.file, list(p))
                        if not (res and isinstance(res[0], list)): ok = False; break
                        t = self.target_atom([x.obj for x in res[0]])
                        if t is None: ok = False; break
                        to = self.out_of_atom(t)
                        if to is not o:
                            self.need_vis[id(t)] = max(self.need_vis.get(id(t), 0), 3 if to.crate != o.crate else 2)
                        mp = self.mod_path(o, to)
                        nm = t.name + ((" as " + alias) if alias and alias != t.name else "")
                        if len(p) - 1 > res[1] + 1 : nm = None
                        byprefix.setdefault(mp, []).append(nm)
                    if not ok or not byprefix or any(n is None for v in byprefix.values() for n in v): continue
                    repl = "".join(f"use {mp}::{names[0] if len(names)==1 else '{' + ', '.join(sorted(names)) + '}'};" + ("\n    " if k < len(byprefix) - 1 else "") for k, (mp, names) in enumerate(byprefix.items()))
                    self.edits[id(u)].append((m.start(), end, repl))
                    # drop ref edits inside this statement
                    self.edits[id(u)] = [ed for ed in self.edits[id(u)] if ed == (m.start(), end, repl) or not (m.start() <= ed[0] < end)]
        # receivers whose type is never named in the using unit: widen by access name over every out that can reach the defining crate
        acc = {}
        for o in lay.outs:
            names = set()
            for u in o.units:
                t = u.it.text
                names |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\s*\(', t))
                names |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\b(?!\s*\()', t))
                names |= set(re.findall(r'\.\s*(\d+)\b', t))
                names |= set(re.findall(r'\b([A-Za-z_]\w*)\s*:(?!:)', t))
                names |= set(re.findall(r'\b([A-Za-z_]\w*)\s*[,}]', t))
            acc[id(o)] = names
        reach_cache = {}
        def reach(c):
            if c in reach_cache: return reach_cache[c]
            seen = set(); stack = [c]
            while stack:
                x = stack.pop()
                for d in set(self.crate_deps.get(x, ())) | set(self.crate_deps_dev.get(x, ())):
                    if d not in seen: seen.add(d); stack.append(d)
            reach_cache[c] = seen; return seen
        for a in lay.w.atoms:
            if id(a) not in lay.crate_of or a.kind not in ("struct", "enum", "union"): continue
            to = self.out_of_atom(a)
            if to is None or to.kind != "prod": continue
            for name, lst in self.members_of(a).items():
                cur = min(vis_rank(d["vis"]) for (_, d) in lst)
                if cur >= 3: continue
                need = 0
                for o in lay.outs:
                    if o is to or name not in acc[id(o)]: continue
                    if o.crate == to.crate: need = max(need, 2)
                    elif to.crate in reach(o.crate): need = max(need, 3)
                if need > cur: self.member_need[id(a)][name] = max(self.member_need[id(a)].get(name, 0), need)
        # member + head vis edits
        for o in lay.outs:
            for u in o.units:
                t = u.atom
                if u.main and id(t) in self.need_vis:
                    cur = self.head_vis(u.it.text)
                    if cur is not None:
                        rank = self.need_vis[id(t)]
                        if vis_rank(cur[0]) < rank:
                            self.edits[id(u)].append((cur[1], cur[2], rank_vis(rank) + " "))
                for name, need in self.member_need.get(id(t), {}).items():
                    for (it, d) in self.members_of(t).get(name, []):
                        if it is not u.it: continue
                        if vis_rank(d["vis"]) >= need: continue
                        if d["vspan"]:
                            s, e = d["vspan"]; self.edits[id(u)].append((s, e, rank_vis(need) + " "))
                        else:
                            self.edits[id(u)].append((d["decl"], d["decl"], rank_vis(need) + " "))

    def test_variants(self, t):
        to = self.out_of_atom(t)
        if to is None: return []
        return [x.atom for x in to.units if x.main and x.atom.name == t.name and any("cfg(test)" in c.replace(" ", "") and "not(test)" not in c.replace(" ", "") for c in x.it.cfg)]

    def compute_testkit(self):
        lay = self.lay
        self.testkit_atoms = {}
        self.testkit_outs = set()
        self.testkit_users = collections.defaultdict(set)
        self.testkit_forward = collections.defaultdict(set)
        def mark(t):
            vs = self.test_variants(t)
            for v in vs: self.testkit_atoms[id(v)] = v
            return bool(vs)
        for o in lay.outs:
            for u in o.units:
                for r in getattr(u, "_refs", []):
                    t = self.target_atom(r["atoms"])
                    if t is None: continue
                    to = self.out_of_atom(t)
                    if to is o or to.crate == o.crate: continue
                    if to.kind in ("test", "integration"):
                        self.testkit_outs.add(id(to)); self.testkit_users[to.crate].add(o.crate)
                    elif mark(t):
                        self.testkit_users[to.crate].add(o.crate)
        changed = True
        while changed:
            changed = False
            for o in lay.outs:
                for u in o.units:
                    tk = (id(u.atom) in self.testkit_atoms) or (id(o) in self.testkit_outs)
                    if not tk: continue
                    for r in getattr(u, "_refs", []):
                        t = self.target_atom(r["atoms"])
                        if t is None: continue
                        to = self.out_of_atom(t)
                        if to.crate == o.crate:
                            if to.kind == "prod" and self.test_variants(t) and id(t) not in self.testkit_atoms and mark(t): changed = True
                            continue
                        if to.kind in ("test", "integration"):
                            self.testkit_forward[o.crate].add(to.crate)
                            if id(to) not in self.testkit_outs: self.testkit_outs.add(id(to)); changed = True
                        elif self.test_variants(t):
                            self.testkit_forward[o.crate].add(to.crate)
                            if id(t) not in self.testkit_atoms and mark(t): changed = True
        for o in lay.outs:
            if id(o) in self.testkit_outs: o.mount_cfg = '#[cfg(any(test, feature = "testkit"))]'
        self.testkit_names = collections.defaultdict(set)
        for t in self.testkit_atoms.values(): self.testkit_names[(id(self.out_of_atom(t)), t.name)].add(1)

    def pkg_dir_of(self, ci):
        n = self.lay.names[ci]
        if n == "runtime": return os.path.join(MCP, "📦️packages", "🦀️rust")
        return os.path.join(MCP, self.lay.plan.NODES[n][0], "📦️packages", "🦀️rust")

    def use_spans(self, u):
        if not hasattr(u, "_use_spans"):
            from uses import parse_use_tree, USE_RE
            spans = []
            text = u.it.text
            for m in USE_RE.finditer(text):
                paths, j = parse_use_tree(text, m.end())
                end = text.find(";", j) + 1 if text[j:j+1] != ";" else j + 1
                spans.append((m.start(), end))
            u._use_spans = spans
        return u._use_spans

    def qualified_keep(self, o, u, r, t, to):
        segs = r["segs"]; ix = r["ix"]
        return (r["start"] >= 0 and to is not o and ix == 1 and segs[0] == to.modname and segs[0] not in ("crate", "self", "super")
                and not (u.atom.kind == "mod" and u.main)
                and to.modname not in self.out_names[id(o)] and to.modname not in self.refnames.get(id(o), ()))

    def glob_sees(self, f, tr):
        for u in self.w.uses[f.path]:
            if u["glob"]:
                for t in self.w.resolve_path(f, u["path"][:-1], frm=f):
                    if t.kind == "mod":
                        for x in self.w.lookup(t.obj, tr.name, frm=f, public_only=True):
                            if x.kind == "atom" and x.obj is tr: return True
        return False

    def note_members(self, u, t, need):
        if t.kind not in ("struct", "enum", "union", "type"): return
        txt = u.it.text
        for name, lst in self.members_of(t).items():
            if all(vis_rank(d["vis"]) >= need for (_, d) in lst): continue
            if re.search(r'(?<![\w])' + re.escape(name) + r'(?![\w])', txt) or (name.isdigit() and re.search(r'\.' + name + r'\b', txt)):
                self.member_need[id(t)][name] = max(self.member_need[id(t)].get(name, 0), need)

    def head_vis(self, text):
        m = HEADVIS.match(text)
        if not m: return None
        if m.group("vis"): return (m.group("vis").strip(), m.start("vis"), m.end("vis"))
        return ("", m.start("kw"), m.start("kw"))

    def bad_names(self, o):
        bad = set(); local = self.out_names[id(o)]
        for name, d in self.wanted[id(o)].items():
            if len(d) > 1 or name in local: bad.add(name)
        return bad

    # ---------- text assembly
    def unit_text(self, u):
        src = u.src
        eds = sorted(self.edits.get(id(u), []), key=lambda e: (e[0], e[1]))
        # drop duplicates
        uniq = []
        for e in eds:
            if uniq and uniq[-1][0] == e[0] and uniq[-1][1] == e[1] and uniq[-1][2] == e[2]: continue
            uniq.append(e)
        last = -1
        for e in uniq:
            if e[0] < last: raise SystemExit(f"overlapping edits in {u.atom.name}: {e} after {last}")
            last = e[1]
        out = []; pos = 0
        for (s, e, r) in uniq:
            out.append(src[pos:s]); out.append(r); pos = e
        out.append(src[pos:])
        txt = "".join(out)
        if not self.sandbox and "crate::test_allocation::" in txt and u.out.crate != self.lay.names.index("foundation"):
            txt = txt.replace("crate::test_allocation::", "semio_framework_os_mcp_foundation::test_allocation::")
        if id(u.atom) in getattr(self, "testkit_atoms", {}) and any(c.replace(" ", "") == "#[cfg(test)]" for c in u.it.cfg):
            txt = txt.replace("#[cfg(test)]", '#[cfg(any(test, feature = "testkit"))]', 1)
        elif (id(u.out), u.atom.name) in getattr(self, "testkit_names", {}) and any("cfg(not(test))" in c.replace(" ", "") for c in u.it.cfg):
            txt = txt.replace("#[cfg(not(test))]", '#[cfg(not(any(test, feature = "testkit")))]', 1)
        return txt

    def cfg_of(self, units):
        sets = []
        for u in units:
            sets.append(set(u.it.cfg) | set(re.findall(r'#\[cfg[^\]]*(?:\][^\]]*)?\]', u.out.mount_cfg or "")) if False else set(u.it.cfg))
        return set.intersection(*sets) if sets else set()

    def import_block(self, o):
        lay = self.lay
        wanted = self.wanted[id(o)]
        bad = o.bad
        lines_by_cfg = collections.defaultdict(lambda: collections.defaultdict(set))   # cfgkey -> modpath -> names
        for name, d in sorted(wanted.items()):
            if name in bad: continue
            (t,) = d.values()
            to = self.out_of_atom(t)
            users = [u for u in o.units if self.unit_uses(u, t)]
            cs = [set(u.it.cfg) for u in users] or [set()]
            common = set.intersection(*cs)
            variants = [x for x in to.units if x.main and x.atom.name == t.name]
            cfgs = (set(t.item.cfg) if len(variants) == 1 else set()) | common
            mc = re.findall(r'#\[cfg(?:[^\]]|\](?!\s*$))*\]', to.mount_cfg or "")
            cfgs |= set(x for x in re.findall(r'#\[cfg\(.*?\)\]', to.mount_cfg or ""))
            if o.kind == "prod" and id(t) not in self.prod_need[id(o)]: cfgs.add("#[cfg(test)]")
            key = tuple(sorted(cfgs))
            lines_by_cfg[key][self.mod_path(o, to)].add(t.name)
        out = []
        for modname, to in sorted(self.mod_wanted.get(id(o), {}).items()):
            mc = re.findall(r'#\[cfg\(.*?\)\]', to.mount_cfg or "")
            if o.kind == "prod" and modname not in self.mod_prod[id(o)]: mc = mc + ["#[cfg(test)]"]
            for c in mc: out.append(c)
            out.append(f"use {self.mod_path(o, to)};")
        for (tr_out, trname) in self.dyn_imports.get(id(o), []):
            if self.sandbox: continue
            if tr_out.crate == o.crate and tr_out is not o: out.append(f"use crate::__semio_dispatch_{trname};")
            elif tr_out.crate != o.crate: out.append(f"use {crate_ident(self.lay, tr_out.crate)}::__semio_dispatch_{trname};")
        for key in sorted(lines_by_cfg):
            for modp in sorted(lines_by_cfg[key]):
                names = sorted(lines_by_cfg[key][modp])
                body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
                for c in key: out.append(c)
                out.append(f"use {modp}::{body};")
        return out

    def unit_uses(self, u, t):
        for r in u._refs:
            if t in r["atoms"] or any(a is t for a in r["atoms"]): return True
        meths = set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\s*\(', u.it.text))
        if t.kind == "trait" and meths & self.trait_methods.get(id(t), set()): return True
        return False

KNOWN_EXT = ["schemars", "serde", "serde_json", "axum", "tokio", "futures", "tokio_tungstenite", "framework_hash", "store", "std", "core", "alloc"]

def ext_entries(em, o):
    """External `use` lines for out o, plus set of ext crate roots used."""
    w = em.w
    lines = []
    roots = set()
    seen = set()
    by_src = collections.defaultdict(list)
    for u in o.units: by_src[u.it.file.path].append(u)
    for sp, us in by_src.items():
        f = w.files[sp]
        body = "\n".join(u.it.text for u in us)
        def subtree(ff):
            yield ff
            for (_, cp, _, _) in ff.children:
                yield from subtree(w.files[cp])
        wholetext = "\n".join(x.text for sf in subtree(f) for x in w.allitems if x.file is sf and x.kind != "use")
        # (1) names used by token, resolved through the original scope (follows `super::`/globs)
        firsts = set(m.group("p").split("::")[0].strip() for m in PATHRE.finditer(body))
        from prod import blank_cfg_test
        prod_body = "\n".join("" if any("cfg(test)" in c.replace(" ", "") for c in u.it.cfg) else blank_cfg_test(u.it.text) for u in us)
        firsts_prod = set(m.group("p").split("::")[0].strip() for m in PATHRE.finditer(prod_body)) if o.kind == "prod" else firsts
        for tok in sorted(firsts):
            if tok in KEYWORDS or tok == "":
                continue
            tg = w.lookup(f, tok, frm=f)
            if not tg or any(t.kind != "ext" for t in tg): continue
            for t in tg:
                p = t.obj
                if p[-1] == tok or True:
                    alias = "" if p[-1] == tok else f" as {tok}"
                    key = (p, tok)
                    if key in seen: continue
                    seen.add(key); roots.add(p[0])
                    cfg = ""
                    for ent in w.uses[sp]:
                        if ent["name"] == tok and not ent["glob"]: cfg = ent["cfg"]; break
                    if tok not in firsts_prod and "cfg(test)" not in cfg.replace(" ", ""): cfg = (cfg + "\n" if cfg else "") + "#[cfg(test)]"
                    lines.append((cfg + "\n" if cfg else "") + f"use {'::'.join(p)}{alias};")
        # (2) silent trait imports (never named in the file text)
        for ent in w.uses[sp]:
            p = ent["path"]
            if ent["glob"]: continue
            name = ent["name"]
            tg = w.lookup(f, name, frm=f) if p[0] in ("crate", "super", "self") else w.resolve_path(f, p, frm=f)
            if not tg or any(t.kind != "ext" for t in tg): continue
            if re.search(r'(?<![\w])' + re.escape(name) + r'(?![\w])', wholetext): continue
            meths = set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\s*\(', body)) | set(re.findall(r'\b(write|writeln|format_args)!', body))
            tab = SILENT_EXT.get(name)
            used = bool(tab & meths) if tab is not None else True
            if not used: continue
            for t in tg:
                pp = t.obj
                key = (pp, name)
                if key in seen: continue
                seen.add(key); roots.add(pp[0])
                alias = "" if pp[-1] == name else f" as {name}"
                if name == "_": alias = " as _"
                lines.append((ent["cfg"] + "\n" if ent["cfg"] else "") + f"use {'::'.join(pp)}{alias};")
    return sorted(set(lines)), roots

def build_file(em, o):
    lay = em.lay
    hdr = []
    if o.primary and o.srcfile is not None:
        d, _ = header_doc(o.srcfile.src)
        if d: hdr.append(d)
    if not hdr:
        emoji = getattr(o, "piece_emoji", "🧱")
        nm = lay.plan.NODES[lay.names[o.crate]][1]
        hdr.append(f"//! {emoji}️ {getattr(o, 'kebab', o.modname).replace('-', ' ').capitalize()} of the {lay.names[o.crate]} crate: {nm}")
    ext, roots = ext_entries(em, o)
    tgt = em.ext_roots_test if o.kind in ("test", "integration") else em.ext_roots
    imps = em.import_block(o)
    tgt[o.crate] |= roots
    from scaffold_idents import DEP_IDENTS
    for u in o.units:
        code = em.unit_text(u) if False else u.it.text
        for ident in DEP_IDENTS:
            if ident in code and re.search(r'(?<![\w$])(?:::)?' + ident + r'(?:::|\b)', code):
                if re.search(r'(?<![\w$])(?:::)?' + ident + r'::', code) or re.search(r'\buse\s+' + ident + r'\b', code): tgt[o.crate].add(ident)
    body = []
    cur = ()
    for u in o.units:
        path = tuple((nm, u.it.file.path) for nm in u.regions)
        common = 0
        while common < len(cur) and common < len(path) and cur[common] == path[common]: common += 1
        for nm, _ in reversed(cur[common:]): body.append(f"//#endregion {nm}")
        for nm, _ in path[common:]: body.append(f"//#region {nm}")
        cur = path
        text = em.unit_text(u)
        piece = ""
        if u.lead.strip(): piece += u.lead.strip("\n") + "\n"
        piece += text
        body.append(piece)
    for nm, _ in reversed(cur): body.append(f"//#endregion {nm}")
    if getattr(o, "primary", False) and o.srcfile is not None:
        for nm in lay.empty_regions.get(o.srcfile.path, []):
            body.append(f"//#region {nm}\n//#endregion {nm}")
    out = "\n".join(hdr) + "\n\n"
    blocks = ext + imps
    if any(id(u.atom) in em.testkit_atoms for u in o.units):
        blocks = [b.replace("#[cfg(test)]", '#[cfg(any(test, feature = "testkit"))]') for b in blocks]
    if blocks: out += "\n".join(blocks) + "\n\n"
    if id(o) in em.fixture_outs:
        out += '/// \U0001f9ea\ufe0f Reads a committed fixture from the package directory at run time instead of embedding it in the test binary.\nfn fixture_text(relative: &str) -> &\'static str {\n    Box::leak(std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)).expect(relative).into_boxed_str())\n}\n\n\n'

    out += "\n\n".join(body) + "\n"
    return out
