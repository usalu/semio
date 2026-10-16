"""🧭 Rewrites `crate::` paths of the pre-split crate onto the carved crates (token level; comments and strings are never touched)."""
import os, re, collections
from rlib import *
import rcrates as C
import usetree

KEEP_ALIASES = {"test_allocation"}

class Model:
    def __init__(self, moved):
        self.moved = moved
        self.old = World(FACADE_ROOT, staged=False)
        self.worlds = {k: World(C.libroot(k)) for k in C.KEYS}
        self.file_crate = {}
        for k, w in self.worlds.items():
            for node in w.nodes.values():
                if node.key == node.file:
                    assert self.file_crate.get(node.file, k) == k, ("file owned twice", node.file, self.file_crate.get(node.file), k)
                    self.file_crate[node.file] = k
        self.needs_pub = collections.OrderedDict()
        self.log = []

    def refresh(self):
        self.worlds = {k: World(C.libroot(k)) for k in C.KEYS}

    def owner_of(self, node, name):
        o = self.moved.get((node.file, name))
        if o: return o
        return self.file_crate.get(node.file)

    def find_def(self, key, name):
        w = self.worlds[key]
        hits = [n for n in w.nodes.values() if name in n.defs and "🧪️tests" not in n.file and not self.in_test(n)]
        if len(hits) == 1: return hits[0]
        pubs = [n for n in hits if n.defs[name]["vis"] == "pub"]
        assert len(pubs) == 1, ("definition not unique in crate", key, name, [h.key for h in hits])
        return pubs[0]

    def in_test(self, n):
        while n is not None:
            if n.name in ("tests",) or (n.name or "").endswith("_tests"): return True
            n = n.parent
        return False

    def chain(self, n):
        out = []
        while n.parent is not None:
            out.append(n.name); n = n.parent
        return list(reversed(out))

    def pub_path(self, w, nd, name):
        r = w.lookup(w.root, name, mode=2)
        if r and r[0] == "def" and r[1] is nd: return [name]
        names_all = sorted({m for node in w.nodes.values() for m in node.mods} | {usetree.local_name(lf) for node in w.nodes.values() for pub, leaves, it in node.uses if it["vis"] == "pub" for lf in leaves if not lf[2] and usetree.local_name(lf)})
        frontier = [(w.root, [])]
        seen = {id(w.root)}
        for depth in range(5):
            nxt = []
            for node, path in frontier:
                for mname in names_all:
                    rm = w.lookup(node, mname, mode=2)
                    if not rm or rm[0] != "mod" or id(rm[1]) in seen: continue
                    seen.add(id(rm[1]))
                    p2 = path + [mname]
                    r = w.lookup(rm[1], name, mode=2)
                    if r and r[0] == "def" and r[1] is nd: return p2 + [name]
                    nxt.append((rm[1], p2))
            frontier = nxt
        return None

    def def_ref(self, old_node, name, here_key):
        d = self.owner_of(old_node, name)
        assert d, ("no owner", old_node.file, name)
        return self.def_ref_in(d, name, here_key)

    def def_ref_in(self, d, name, here_key):
        nd = self.find_def(d, name)
        w = self.worlds[d]
        if d == here_key:
            r = w.lookup(w.root, name)
            if r and r[0] == "def" and r[1] is nd: return "crate::" + name
            return "crate::" + "::".join(self.chain(nd) + [name])
        p = self.pub_path(w, nd, name)
        if p is None:
            self.needs_pub[(d, name)] = nd
            p = self.chain(nd) + [name]
        return C.ident(d) + "::" + "::".join(p)

    def mod_ref(self, old_node, here_key):
        if old_node.key == old_node.file:
            d = self.file_crate[old_node.file]
            w = self.worlds[d]
            nn = w.nodes[old_node.key]
            ch = self.chain(nn)
            if d == here_key: return "crate" + ("::" + "::".join(ch) if ch else "")
            return C.ident(d) + ("::" + "::".join(ch) if ch else "")
        nm = old_node.name
        if nm == "codec":
            return "crate" if here_key == "codec" else C.ident("codec")
        raise KeyError(("module without a new home", old_node.key))

    def map_path(self, segs, here_key, as_leaf=False, glob=False):
        """segs after `crate`; returns (replacement text, consumed segment count) or None to keep."""
        if segs[0] in KEEP_ALIASES: return None
        res, consumed = self.old.walk(self.old.root, ["crate"] + list(segs))
        assert res is not None, ("unresolved path", segs)
        c = consumed - 1
        if res[0] == "ext":
            return "::".join(res[1]), c
        if res[0] == "def":
            return self.def_ref(res[1], res[2], here_key), c
        if res[0] == "mod":
            return self.mod_ref(res[1], here_key), c
        raise KeyError(res)

    def map_leaf(self, leaf, here_key):
        path, alias, glob = leaf
        if path[0] != "crate" or len(path) == 1: return [leaf]
        segs = path[1:]
        r = self.map_path(segs, here_key)
        if r is None: return [leaf]
        text, c = r
        newp = text.split("::") + list(segs[c:])
        if alias is None and not glob and c == len(segs):
            res, _ = self.old.walk(self.old.root, ["crate"] + list(segs))
            if res[0] == "mod" or (res[0] == "ext" and newp[-1] != segs[-1]):
                alias = segs[-1]
        return [(newp, alias, glob)]

def sig_tokens(toks):
    return [i for i, t in enumerate(toks) if t[0] not in ("ws", "comment")]

def rewrite(text, here_key, model):
    toks = tokenize(text)
    sig = sig_tokens(toks)
    pos = {ti: si for si, ti in enumerate(sig)}
    edits = []
    use_spans = []
    n = len(sig)
    si = 0
    while si < n:
        ti = sig[si]; kd, tx, a, b = toks[ti]
        if kd == "id" and tx == "use" and not (si > 0 and toks[sig[si - 1]][1] in (".", "$")):
            depth = 0; sj = si + 1
            while sj < n:
                t2 = toks[sig[sj]]
                if t2[1] in "{(": depth += 1
                elif t2[1] in "})": depth -= 1
                elif t2[1] == ";" and depth == 0: break
                sj += 1
            tree_a = toks[sig[si]][3]; tree_b = toks[sig[sj]][2]
            tree = re.sub(r"//[^\n]*", "", text[tree_a:tree_b])
            has_crate = any(toks[sig[k]][0] == "id" and toks[sig[k]][1] == "crate" for k in range(si + 1, sj))
            start_si = si
            k = si - 1
            vis_start = None
            if k >= 0 and toks[sig[k]][1] == ")":
                kk = k; d2 = 0
                while kk >= 0:
                    if toks[sig[kk]][1] == ")": d2 += 1
                    elif toks[sig[kk]][1] == "(":
                        d2 -= 1
                        if d2 == 0: break
                    kk -= 1
                if kk - 1 >= 0 and toks[sig[kk - 1]][1] == "pub": start_si = kk - 1
            elif k >= 0 and toks[sig[k]][1] == "pub": start_si = k
            if has_crate:
                leaves = usetree.parse(tree)
                mapped = []
                for lf in leaves: mapped += model.map_leaf(lf, here_key)
                lines = usetree.emit(mapped)
                a0 = toks[sig[start_si]][2]; b0 = toks[sig[sj]][3]
                prefix = text[a0:toks[sig[si]][2]]
                ls = text.rfind("\n", 0, a0) + 1
                indent = re.match(r"[ \t]*", text[ls:a0]).group(0) if text[ls:a0].strip() == "" else ""
                edits.append((a0, b0, ("\n" + indent).join(prefix + l for l in lines)))
                use_spans.append((a0, b0))
            si = sj + 1
            continue
        si += 1
    def in_use(p):
        return any(a <= p < b for a, b in use_spans)
    si = 0
    while si < n:
        ti = sig[si]; kd, tx, a, b = toks[ti]
        if kd == "id" and tx == "crate" and not in_use(a) and si + 2 < n and toks[sig[si + 1]][1] == ":" and toks[sig[si + 2]][1] == ":":
            if si > 0 and toks[sig[si - 1]][1] == "$":
                si += 1; continue
            segs = []; ends = []
            k = si + 3
            while k < n and toks[sig[k]][0] == "id":
                segs.append(toks[sig[k]][1]); ends.append(sig[k])
                if k + 3 < n and toks[sig[k + 1]][1] == ":" and toks[sig[k + 2]][1] == ":" and toks[sig[k + 3]][0] == "id":
                    k += 3
                else:
                    break
            if segs:
                r = model.map_path(segs, here_key)
                if r is not None:
                    repl, c = r
                    edits.append((a, toks[ends[c - 1]][3], repl))
                    si = k + 1
                    continue
        si += 1
    edits.sort()
    for i in range(1, len(edits)):
        assert edits[i - 1][1] <= edits[i][0], ("overlapping edits", edits[i - 1], edits[i])
    out = []; last = 0
    for a, b, r in edits:
        out.append(text[last:a]); out.append(r); last = b
    out.append(text[last:])
    return "".join(out)
