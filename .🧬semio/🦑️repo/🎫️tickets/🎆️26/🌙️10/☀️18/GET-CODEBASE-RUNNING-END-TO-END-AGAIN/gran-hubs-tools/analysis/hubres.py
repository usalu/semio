"""🧭 Hub path resolver for the repoint tools: `make(hub, own_map, overrides)` -> object with `.resolve(segs)` returning ('rewrite', owner ident, new segs) | ('own',) | ('modalias', segs) | ('fail', why). `own_map` sends hub-defined names (macros, relocated items) to their new crate: {first path seg: (new crate ident, [new path prefix])}; `overrides` maps names the hub does not export to their defining crate (broken imports): {first seg: (crate ident, [path])}."""
from hubenv import *
from cratemodel import Model
FAC = {"semio_framework", "semio_framework_os_kernel", "semio_framework_plugin"}


class HubRes:
    def __init__(self, hub, own_map=None, overrides=None, world=None, allow_gated=False):
        self.allow_gated = allow_gated
        self.gate = {}
        self.hub = hub
        if world is None:
            self.world, self.I2L, self.I2D, self.LIB = make_world()
        else:
            self.world = world
            self.LIB, self.I2D = libmaps()
            self.I2L = world.i2l
        self.HD = hubdir(hub) if hub != "umbrella" else byname["semio-framework"]
        self.ID = self.LIB[self.HD]
        self.model = self.world.model_for(self.ID)
        self.own_map = own_map or {}
        self.overrides = overrides or {}
        MAN = json.load(open(OUT + "/manifests.json"))
        self.gated = set()
        hubfeat = MAN[self.HD]["features"]
        for x in MAN[self.HD]["deps"]:
            if x["kind"] in ("normal", "build") and (x["optional"] or x["target"]):
                for k in ("normal", "build"):
                    for dn, t in C[self.HD]["deps"][k]:
                        if dn == x["key"] and t in C:
                            self.gated.add(self.LIB[t])
                            feats = sorted(f for f, lst in hubfeat.items() if "dep:" + x["key"] in lst or (x["key"] in lst))
                            self.gate[self.LIB[t]] = {"pkg": x["pkg"], "target": x["target"], "optional": x["optional"], "features": feats, "dir": t}
        self.cache, self.pure = {}, {}

    def mod_pure(self, node, seen=None):
        seen = seen if seen is not None else set()
        if id(node) in seen:
            return True
        seen.add(id(node))
        if node.defs:
            return False
        for vis, p, al, it in node.uses:
            if vis.startswith("pub") and p[0] in node.children and not self.mod_pure(node.children[p[0]], seen):
                return False
        return all(self.mod_pure(c, seen) for nm, c in node.children.items() if c.vis.startswith("pub"))

    def resolve(self, segs, macro_ns=False):
        key = (tuple(segs), macro_ns)
        if key in self.cache:
            return self.cache[key]
        Model.macro_last = macro_ns
        try:
            out = self._resolve(list(segs))
        finally:
            Model.macro_last = False
        self.cache[key] = out
        return out

    def _resolve(self, segs):
        h = segs[0]
        for n in range(len(segs), 0, -1):
            k = "::".join(segs[:n])
            if k in self.own_map:
                ident, prefix = self.own_map[k]
                return ("rewrite", ident, list(prefix) + segs[n:])
        q, suffix, r = list(segs), [], None
        while q:
            r = self.world.resolve(self.model, q)
            if r:
                break
            suffix.insert(0, q.pop())
        if suffix:
            for k in (suffix[0], segs[-1]):
                if k in self.overrides:
                    ident, path = self.overrides[k]
                    return ("rewrite", ident, list(path) + suffix[1:])
        if not r or not q:
            if h in self.overrides:
                ident, path = self.overrides[h]
                return ("rewrite", ident, list(path) + segs[1:])
            return ("fail", "unresolved")
        if r["kind"] == "external":
            return ("fail", "external " + str(r.get("owner")))
        if r["owner"] == self.ID and r["kind"] in ("def", "mod", "crate"):
            if r["kind"] == "mod" and not suffix:
                nd = [x for x in self.model.nodes if x.name == r["name"] and os.path.relpath(x.file, ROOT) == r["file"]]
                return ("modalias", list(segs)) if nd and self.mod_pure(nd[0]) else ("own",)
            return ("own",)
        st = [s for s in r.get("steps", []) if s[0] not in FAC and s[0] != self.ID]
        if not st:
            return ("fail", "no non-facade owner")
        if st[0][0] in self.gated and not self.allow_gated:
            return ("fail", "gated owner " + st[0][0])
        return ("rewrite", st[0][0], list(st[0][1:]) + suffix)
