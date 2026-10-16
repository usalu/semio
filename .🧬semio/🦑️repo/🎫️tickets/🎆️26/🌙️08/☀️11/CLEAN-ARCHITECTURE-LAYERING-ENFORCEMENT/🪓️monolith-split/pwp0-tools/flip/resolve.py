# 🧭️ Name resolution of the monolith namespace: where is `name` defined when seen from module `M`, and what is the final crate path of that definition.
import os, re, collections, sys
from common import *
from index import Tree, FileInfo
from modules import module_map
import model

pt = model.pt
EXTERNAL_LIBS = {
    "🏛️ownership/🧱️owner/🦀️.rs": "semio_framework_plugin_ownership",
    "🏇️mounted-owner/🦀️.rs": "semio_framework_plugin_mounted_owner",
}


class World:
    def __init__(self, plug=PLUG, files=None):
        self.plug = plug
        self.T = Tree(plug)
        self.MM = module_map(self.T)
        self.files = files if files is not None else self._all()
        self.defs = collections.defaultdict(list)
        for f in self.files:
            fi = self.T.info(f)
            for name, rows in fi.items.items():
                for r in rows:
                    self.defs[name].append((f, r))
        self.parents = collections.defaultdict(list)
        for f in self.files:
            for name, child, m in self.T.children(f):
                self.parents[child].append((f, name, m))
        self.root_ns = ["🦀️.rs"] + ["🪝️mounts/" + h + "/🦀️.rs" for h in sorted(os.listdir(plug + "/🪝️mounts"))]
        self.ext_of = {}
        for r, lib in EXTERNAL_LIBS.items():
            if self.T.exists(r):
                self._ext_tree(r, lib, [], set())
        self._modid = {}

    def _ext_tree(self, f, lib, mp, seen):
        if f in seen or not self.T.exists(f):
            return
        seen.add(f)
        self.ext_of[f] = (lib, mp)
        for name, child, m in self.T.children(f):
            self._ext_tree(child, lib, mp if name == "<include>" else mp + [name], seen)

    def _all(self):
        out = []
        for base, dirs, files in os.walk(self.plug):
            dirs[:] = [d for d in dirs if d not in model.SKIP_DIRS]
            for f in files:
                if f.endswith(".rs"):
                    out.append(os.path.relpath(os.path.join(base, f), self.plug))
        return out

    # ------------------------------------------------------------------ module identity
    def modid(self, f):
        """the module a file's items live in: a file mounted with `mod` is its own module, a file only `include!`d lives in its includer's module"""
        if f in self._modid:
            return self._modid[f]
        self._modid[f] = f
        ps = self.parents.get(f, [])
        if ps and all(name == "<include>" for _, name, _ in ps):
            self._modid[f] = self.modid(ps[0][0])
        return self._modid[f]

    def parent_module(self, f):
        """module identity of `super::` for file f"""
        ps = self.parents.get(f, [])
        if not ps:
            return None
        for p, name, m in ps:
            if name != "<include>":
                return self.modid(p)
        p, name, m = ps[0]
        from edits import find_mounts
        inline = None
        try:
            for x in find_mounts(self.T.info(p).text, p, f):
                if x["kind"] == "include" and x.get("inline"):
                    inline = x["inline"]
        except Exception:
            pass
        if inline:
            return self.modid(p)
        return self.parent_module(self.modid(p)) if self.modid(p) != f else None

    def parent_file(self, f):
        ps = self.parents.get(f, [])
        return ps[0][0] if ps else None

    # ------------------------------------------------------------------ lookup
    def lookup(self, mrel, name, seen=None, depth=0, fallback=True):
        """definition of `name` visible in module `mrel`: ('def', file, kind) | ('mod', file) | ('ext', path list) | ('inline', file, name) | None"""
        res = self._lookup(mrel, name, seen if seen is not None else set(), depth)
        if res is None and fallback:
            res = self.global_def(name)
        return res

    def global_def(self, name):
        cands = [(f, r) for f, r in self.defs.get(name, []) if r["kind"] in ("fn", "struct", "enum", "trait", "type", "const", "static", "union", "macro_rules!") and "/🧪️tests/" not in f and not f.startswith("🧪️")]
        if not cands:
            return None
        owners = {self.ext_of[f][0] if f in self.ext_of else pt.file_crate(f) for f, r in cands}
        if len(owners) == 1 and len(cands) <= 2:
            f, r = cands[0]
            return ("def", f, r["kind"])
        return None

    def _lookup(self, mrel, name, seen, depth):
        key = (mrel, name)
        if key in seen or depth > 14:
            return None
        seen.add(key)
        fi = self.T.info(mrel)
        incs = [mrel] + [c for n, c, m in self.T.children(mrel) if n == "<include>"]
        for inc in incs:
            fj = self.T.info(inc)
            if name in fj.items:
                r = fj.items[name][0]
                if r["kind"] == "mod" and name in fj.inline:
                    return ("inline", inc, name)
                if r["kind"] == "mod" and any(m["name"] == name for m in fj.mounts):
                    continue
                return ("def", inc, r["kind"])
        for name_, child, m in self.T.children(mrel):
            if name_ == name:
                return ("mod", child)
        for inc in incs:
            fj = self.T.info(inc)
            if name in fj.inline:
                return ("inline", inc, name)
        for inc in incs:
            fj = self.T.info(inc)
            for u in fj.uses:
                if u["glob"] or not u["path"]:
                    continue
                if (u["alias"] or u["path"][-1]) == name:
                    return self.follow(inc, u["path"], seen, depth + 1)
        for inc in incs:
            fj = self.T.info(inc)
            for u in fj.uses:
                if u["glob"] and u["path"]:
                    res = self.follow_glob(inc, u["path"], name, seen, depth + 1)
                    if res:
                        return res
        return None

    def crate_root_rel(self, lib):
        key = lib.replace("semio_framework_plugin_", "").replace("_", "-") if lib.startswith("semio_framework_plugin_") else None
        return CRATES[key]["root"] + "/🦀️.rs" if key in CRATES else None

    def _below(self, start, segs):
        cur = start
        for i, s in enumerate(segs):
            nxt = None
            for name_, child, mt in self.T.children(cur):
                if name_ == s:
                    nxt = child
                    break
            if nxt is None:
                res = self.lookup(cur, s, fallback=False)
                if res and res[0] == "mod":
                    nxt = res[1]
                else:
                    return ("path", cur, segs[i:])
            cur = nxt
        return ("mod", cur)

    def through_hub(self, m, rest):
        """module target below a hub unit that is an extern crate / re-export / shim"""
        if m["kind"] == "extern":
            r = self.crate_root_rel(m["lib"])
            return self._below(r, rest) if r else ("ext", [m["lib"]] + rest)
        if m["kind"] == "reexport":
            r = self.crate_root_rel(m["lib"])
            return self._below(r, m["path"] + rest) if r else ("ext", [m["lib"]] + m["path"] + rest)
        if not rest:
            return None
        name = rest[0]
        for u in m["uses"]:
            if u["glob"] or not u["path"]:
                continue
            if (u["alias"] or u["path"][-1]) == name and u["path"][0].startswith("semio_framework_plugin_"):
                r = self.crate_root_rel(u["path"][0])
                return self._below(r, u["path"][1:] + rest[1:]) if r else ("ext", u["path"] + rest[1:])
        for u in m["uses"]:
            if u["glob"] and u["path"] and u["path"][0].startswith("semio_framework_plugin_"):
                r = self.crate_root_rel(u["path"][0])
                if not r:
                    continue
                res = self._below(r, u["path"][1:] + rest)
                if res and res[0] == "mod":
                    return res
        return None

    def module_target(self, mrel, path, seen, depth):
        segs = list(path)
        cur = mrel
        if not segs:
            return ("mod", cur)
        head = segs[0]
        rest = segs[1:]
        if head == "crate":
            if not rest:
                return None
            if rest[0] == "component" and len(rest) > 1:
                rest = rest[1:]
            m = self.MM.get(rest[0])
            if m is None:
                res = None
                for r in self.root_ns:
                    res = self.lookup(r, rest[0], fallback=False)
                    if res:
                        break
                if res and res[0] == "mod":
                    cur, rest = res[1], rest[1:]
                elif res and res[0] == "inline":
                    return ("inline", res[1], res[2], rest[1:])
                else:
                    return ("rootname", rest)
            elif m["kind"] == "file":
                cur, rest = m["file"], rest[1:]
            else:
                return self.through_hub(m, rest[1:])
        elif head == "super":
            cur = self.parent_module(mrel)
            if cur is None:
                return None
            while rest and rest[0] == "super":
                cur = self.parent_module(cur)
                if cur is None:
                    return None
                rest = rest[1:]
            if cur in self.root_ns:
                return self.module_target(cur, ["crate"] + rest, seen, depth)
        elif head == "self":
            pass
        else:
            if head.startswith("semio_framework_plugin_") and self.crate_root_rel(head):
                return self._below(self.crate_root_rel(head), rest)
            if head.startswith("semio_framework") or head in ("std", "core", "alloc"):
                return ("ext", segs)
            hit = None
            for name_, child, m in self.T.children(mrel):
                if name_ == head:
                    hit = child
                    break
            if hit:
                cur, rest = hit, segs[1:]
            else:
                mm = self.MM.get(head)
                if mm and mm["kind"] == "file":
                    cur, rest = mm["file"], segs[1:]
                elif mm:
                    return self.through_hub(mm, segs[1:])
                else:
                    fi = self.T.info(mrel)
                    if head in fi.inline:
                        return ("inline", mrel, head, segs[1:])
                    return ("ext", segs)
        for i, s in enumerate(rest):
            nxt = None
            for name_, child, m in self.T.children(cur):
                if name_ == s:
                    nxt = child
                    break
            if nxt is None:
                fi = self.T.info(cur)
                if s in fi.inline:
                    return ("inline", cur, s, rest[i + 1:])
                return ("path", cur, rest[i:])
            cur = nxt
        return ("mod", cur)

    def follow(self, mrel, path, seen, depth):
        tgt = self.module_target(mrel, path[:-1], seen, depth) if len(path) > 1 else ("mod", mrel)
        if tgt is None:
            return None
        last = path[-1]
        if tgt[0] in ("ext", "rootname"):
            return ("ext", tgt[1] + [last])
        if tgt[0] == "path":
            return None
        if tgt[0] == "inline":
            return self.inline_lookup(tgt[1], tgt[2], tgt[3] + [last], seen, depth)
        return self._lookup(tgt[1], last, seen, depth)

    def inline_lookup(self, f, mname, rest, seen, depth):
        body = self.T.info(f).inline.get(mname)
        if not body or not rest:
            return None
        name = rest[0]
        for u in body["uses"]:
            if u["glob"] or not u["path"]:
                continue
            if (u["alias"] or u["path"][-1]) == name:
                return self.follow(f, u["path"], seen, depth + 1)
        if name in body["items"]:
            return ("def", f, body["items"][name][0]["kind"])
        for incp in body["includes"]:
            inc = os.path.normpath(os.path.join(os.path.dirname(f), incp))
            if self.T.exists(inc):
                r = self._lookup(inc, name, seen, depth + 1)
                if r:
                    return r
        for u in body["uses"]:
            if u["glob"] and u["path"]:
                tgt = self.module_target(f, u["path"], seen, depth + 1)
                if tgt is None or tgt[0] in ("path",):
                    continue
                if tgt[0] in ("ext", "rootname"):
                    hit = self.ext_glob_defines(tgt[1], name)
                    if hit:
                        return hit
                    continue
                if tgt[0] == "inline":
                    r = self.inline_lookup(tgt[1], tgt[2], tgt[3] + [name], seen, depth + 1)
                else:
                    r = self._lookup(tgt[1], name, seen, depth + 1)
                if r:
                    return r
        return None

    def ext_glob_defines(self, path, name):
        """does an external glob path (`semio_framework_plugin_ownership::bounded_ownership`) define `name`? -> ('def', file, kind) when we can see its source"""
        lib = path[0]
        for f, (l, mp) in self.ext_of.items():
            if l == lib and mp == path[1:]:
                fi = self.T.info(f)
                if name in fi.items:
                    return ("def", f, fi.items[name][0]["kind"])
        return None

    def follow_glob(self, mrel, path, name, seen, depth):
        tgt = self.module_target(mrel, path, seen, depth)
        if tgt is None or tgt[0] == "path":
            return None
        if tgt[0] in ("ext", "rootname"):
            return self.ext_glob_defines(tgt[1], name)
        if tgt[0] == "inline":
            return self.inline_lookup(tgt[1], tgt[2], tgt[3] + [name], seen, depth)
        return self._lookup(tgt[1], name, seen, depth)
