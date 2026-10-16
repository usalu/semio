import re, sys, collections, functools
sys.path.insert(0, ".")
from igraph import *
from uses import file_uses
from prod import blank_cfg_test, is_test_file

PATHRE = re.compile(r"(?<![\w$'#])(?P<p>(?:::)?[A-Za-z_]\w*(?:\s*::\s*(?:<[^<>]*(?:<[^<>]*>[^<>]*)*>\s*::\s*)?[A-Za-z_]\w*)*)")
KEYWORDS = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn".split())

class Target:
    __slots__ = ("kind", "obj")
    def __init__(self, kind, obj): self.kind = kind; self.obj = obj
    def __repr__(self): return f"<{self.kind} {getattr(self.obj,'name',self.obj)}>"
    def __eq__(self, o): return self.kind == o.kind and self.obj is o.obj if not isinstance(self.obj, tuple) else (self.kind == o.kind and self.obj == o.obj)
    def __hash__(self): return hash((self.kind, id(self.obj) if not isinstance(self.obj, tuple) else self.obj))

class World:
    def __init__(self):
        self.files, self.allitems, self.atoms, self.defs, self.loose = build()
        self.root = self.files[LIBROOT]
        self.modfile = {tuple(f.modpath): f for f in self.files.values()}
        self.defs_in = collections.defaultdict(lambda: collections.defaultdict(list))
        self.mount_atoms = set()
        for a in self.atoms:
            if a.kind == 'mod' and a.item.text.rstrip().endswith(';'):
                self.mount_atoms.add(id(a)); continue
            self.defs_in[a.file.path][a.name].append(a)
        self.uses = {}
        for f in self.files.values():
            lst = []
            for vis, p, alias, off, pre in file_uses(f, [x for x in self.allitems if x.file is f]):
                if p[-1] == "*": name = "*"
                elif p[-1] == "self" and len(p) > 1:
                    p = p[:-1]; name = alias or p[-1]
                else: name = alias or p[-1]
                if f.modpath[0] == "integration" and p[0] == "semio_framework_os_mcp": p = ("crate",) + p[1:]
                lst.append(dict(vis=vis, path=p, name=name, cfg=pre, off=off, glob=(p[-1] == "*")))
            self.uses[f.path] = lst
        # extern crate aliases at root: `extern crate x as y;`
        self.extern_aliases = set(m.group(1) for m in re.finditer(r'extern\s+crate\s+\w+\s+as\s+(\w+)', self.root.code))
        self.child_by_name = {}
        for f in self.files.values():
            d = {}
            for (n, path, cfg, vis) in f.children: d[n] = self.files[path]
            self.child_by_name[f.path] = d
        self._memo = {}

    def descendant(self, f, anc):
        cur = f
        while cur is not None:
            if cur is anc: return True
            cur = cur.parent
        return False

    def visible(self, a_or_use_vis, owner, frm):
        return bool(a_or_use_vis) or self.descendant(frm, owner)

    def lookup(self, f, name, frm=None, seen=None, public_only=False):
        frm = frm or f
        key = (f.path, name, frm.path, public_only)
        if key in self._memo: return self._memo[key]
        seen = seen or set()
        if (f.path, name, public_only) in seen: return []
        seen = seen | {(f.path, name, public_only)}
        out = []
        for a in self.defs_in[f.path].get(name, []):
            if not public_only or self.visible(a.item.vis, f, frm):
                out.append(Target("atom", a))
        ch = self.child_by_name[f.path].get(name)
        if ch is not None and (not public_only or self.visible(
                next((v for (n, p, c, v) in f.children if n == name), ""), f, frm)):
            out.append(Target("mod", ch))
        for u in self.uses[f.path]:
            if u["glob"] or u["name"] != name: continue
            if public_only and not self.visible(u["vis"], f, frm): continue
            out += self.resolve_path(f, u["path"], frm=f, seen=seen)
        if not out:
            for u in self.uses[f.path]:
                if not u["glob"]: continue
                if public_only and not self.visible(u["vis"], f, frm): continue
                tgt = self.resolve_path(f, u["path"][:-1], frm=f, seen=seen)
                for t in tgt:
                    if t.kind == "mod":
                        out += self.lookup(t.obj, name, frm=frm, seen=seen, public_only=True)
        # dedupe
        res = []
        for t in out:
            if t not in res: res.append(t)
        if not seen or len(seen) <= 1: self._memo[key] = res
        return res

    def resolve_path(self, f, segs, frm=None, seen=None):
        frm = frm or f
        segs = list(segs)
        if segs and segs[0] == "semio_framework_os_mcp" and f.modpath[0] == "integration": segs[0] = "crate"
        if not segs: return []
        first = segs[0]
        cur = None
        if first == "crate": cur = [Target("mod", self.root)]; rest = segs[1:]
        elif first == "self": cur = [Target("mod", f)]; rest = segs[1:]
        elif first == "super":
            m = f
            rest = segs
            while rest and rest[0] == "super":
                m = m.parent if m.parent is not None else m
                rest = rest[1:]
            cur = [Target("mod", m)]
        else:
            cur = self.lookup(f, first, frm=frm, seen=seen)
            rest = segs[1:]
            if not cur:
                return [Target("ext", tuple(segs))]
        for seg in rest:
            nxt = []
            for t in cur:
                if t.kind == "mod":
                    nxt += self.lookup(t.obj, seg, frm=frm, seen=seen, public_only=True)
                elif t.kind == "atom":
                    nxt.append(t)
                elif t.kind == "ext":
                    nxt.append(Target("ext", t.obj + (seg,)))
            cur = nxt
            if not cur: return []
        return cur

    # ---- references of an atom
    def atom_refs(self, a, test=False):
        refs = {}
        parts = [a.item] + a.impls
        for it in parts:
            f = it.file
            text = it.text
            for m in PATHRE.finditer(text):
                p = m.group("p")
                if m.start() >= 1 and text[m.start()-1] == "." and not (m.start() >= 2 and text[m.start()-2] == "."): continue
                segs = [s.strip() for s in re.split(r"\s*::\s*", re.sub(r"::\s*<[^<>]*(?:<[^<>]*>[^<>]*)*>\s*(?=::)", "", p)) if s.strip()]
                if segs and segs[0] == "":
                    continue
                if not segs or segs[0] in KEYWORDS and segs[0] not in ("crate", "self", "super"): continue
                first = segs[0]
                end = m.end()
                k = end
                while k < len(text) and text[k] in " \t\n": k += 1
                follow = text[k:k+1]
                before = text[max(0, m.start()-4):m.start()].rstrip()
                if before.endswith("fn") and len(segs) == 1: continue
                tg = self.resolve_path(f, segs)
                for t in tg:
                    if t.kind == "atom" and t.obj is not a:
                        if len(segs) == 1 and t.obj.kind == "fn" and follow not in "(:<)," and follow != "": 
                            if follow not in ";}]": continue
                        if len(segs) == 1 and follow == ":" and text[k:k+2] != "::": continue
                        refs.setdefault(t.obj, set()).add(it.file.path)
            for m in re.finditer(r'\b(with|serialize_with|deserialize_with|default|skip_serializing_if)\s*=\s*"([A-Za-z_][\w:]*)"', it.file.src[it.start:it.end]):
                segs = m.group(2).split("::")
                for t in self.resolve_path(f, segs):
                    if t.kind == "atom" and t.obj is not a:
                        refs.setdefault(t.obj, set()).add(it.file.path)
        return refs

def flags_for(world, a):
    if is_test_file(a.file, world.files): return True
    return any('cfg(test)' in c.replace(' ', '') for c in a.item.cfg)

if __name__ == "__main__":
    w = World()
    print("atoms", len(w.atoms), "files", len(w.files))
    for nm in ["GatewayBackends", "ArtifactChannels", "HeadlessWorkspace", "tool_from_capability", "GatewayError"]:
        print(nm, w.lookup(w.root, nm))
