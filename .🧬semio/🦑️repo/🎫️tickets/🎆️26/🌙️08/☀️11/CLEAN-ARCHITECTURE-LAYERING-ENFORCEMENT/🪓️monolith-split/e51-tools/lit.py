"""🧱 E51 E0451 sweep scanner: rustc reports E0451 (private field in a struct literal or struct pattern) only in the privacy pass, which never runs while any other error exists, so the union oracle is blind to it.
This scanner finds brace-struct literals and patterns whose struct is defined in the loaded crates and names a field that is not visible at the use site: not `pub` for another crate, private or `pub(super)` outside the defining module
(module tree from `mod x;`, `#[path]` and `include!`; inline modules honoured; descendants of the defining file count as inside). `..base` needs every field visible. `Self { .. }` is resolved through the enclosing `impl` head.
Fields behind `cfg(any(test, feature = "testkit"))` count as `pub`. Candidates for a use are the structs of its own crate and of every crate in its (transitive) dependency closure.
Integration targets (`[[test]]`, `[[bin]]`, `[[example]]`, `[[bench]]`, `tests/*.rs`, `src/bin/*.rs`, `examples/*.rs`, `benches/*.rs`, `src/main.rs`) are scanned as separate roots that see their package only through `pub`.
Read-only (writes only `GEN/e51-<tag>-*`).
usage: lit.py selftest | lit.py scan [--families a,b,c|all] [--tag T] [--no-targets]   (POS_PREFIX defaults to e51)"""
import json, os, re, sys, collections, glob

os.environ.setdefault("POS_PREFIX", "e51")
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../pos-tools")
import posunion as U
from poslib import *
from posstub import mask

FAMDIR = HERE + "/../pos-tools/families"
SKIP_FAMILIES = {"groups2", "pi2", "rii"}
TARGET_DIRS = (("tests", "test"), ("src/bin", "bin"), ("examples", "example"), ("benches", "bench"))


def balanced(t, i, o="{", c="}"):
    d = 0
    n = len(t)
    while i < n:
        ch = t[i]
        if ch == o:
            d += 1
        elif ch == c:
            d -= 1
            if d == 0:
                return i
        i += 1
    return -1


def split_top(body):
    parts, cur, d, i = [], [], 0, 0
    n = len(body)
    while i < n:
        ch = body[i]
        if ch == "-" and i + 1 < n and body[i + 1] == ">":
            cur.append("->")
            i += 2
            continue
        if ch in "([{<":
            d += 1
        elif ch in ")]}>":
            d -= 1
        if ch == "," and d == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
        i += 1
    parts.append("".join(cur))
    return [p.strip() for p in parts if p.strip()]


def family_specs(names=None):
    out = {}
    for p in sorted(glob.glob(FAMDIR + "/*.json")):
        nm = os.path.basename(p)[:-5]
        if nm in SKIP_FAMILIES or (names and nm not in names):
            continue
        out[nm] = json.load(open(p, encoding="utf-8"))["crates"]
    return out


class World:
    """Every scanned root: lib trees of the loaded crates plus (optionally) their integration targets."""

    def __init__(self, pkgs, targets=True, extra=None, dep_closure=False):
        self.src = Source(None)
        mans = Manifests(self.src)
        mans.build_index()
        self.mans = mans
        self.primary = set(pkgs)
        if dep_closure:
            seen, stack = set(pkgs), list(pkgs)
            while stack:
                pkg = stack.pop()
                man = mans.manifest(pkg, self.src)
                if man is None:
                    continue
                for kind, name, dpkg, feats, opt, renamed in mans.deps(man):
                    if dpkg not in seen and (dpkg in mans.by_name) and (kind != "dev-dependencies" or pkg in self.primary):
                        seen.add(dpkg)
                        stack.append(dpkg)
            pkgs = sorted(seen)
        self.crates = {c.pkg: c for c in U.prepare(self.src, mans, pkgs, False)}
        self.texts = {}
        self.fid = {}
        self.parent = {}
        self.incl_site = {}
        self.owner = {}
        self.kind = {}
        self.roots = {}
        for pkg, c in self.crates.items():
            self.add_root(pkg, pkg, c.lib_rel, c.files, c.code, "lib")
        if targets:
            for pkg, c in list(self.crates.items()):
                if pkg in self.primary:
                    self.add_targets(pkg, c)
        for owner, label, rel, text, kind in extra or []:
            data = {r: x.encode("utf-8") for r, x in (text.items() if isinstance(text, dict) else [(rel, text)])}
            self.add_root(label, owner, rel, data, set(data), kind)
        self.deps = self.closure()
        self.defs = collections.defaultdict(list)
        self.nest_cache = {}
        self.impl_cache = {}
        for key in list(self.roots):
            self.collect_defs(key)
            self.collect_variants(key)

    def add_root(self, key, owner, root, files, code, kind):
        self.roots[key] = root
        self.owner[key] = owner
        self.kind[key] = kind
        for rel in code:
            tx = files[rel].decode("utf-8", "replace")
            self.texts[(key, rel)] = (tx, mask(tx))
        self.build_tree(key, root, files, code)

    def add_targets(self, pkg, c):
        man = c.man
        d = man["dir"]
        found = {}
        mtext = (self.src.read(man["rel"]) or b"").decode("utf-8", "replace")
        for blk in re.finditer(r"^\[\[(test|bin|example|bench)\]\]\s*\n((?:(?!\[).*\n?)*)", mtext, re.M):
            pm = re.search(r"^\s*path\s*=\s*\"([^\"]+)\"", blk.group(2), re.M)
            nm = re.search(r"^\s*name\s*=\s*\"([^\"]+)\"", blk.group(2), re.M)
            if pm:
                found[norm(os.path.join(d, pm.group(1)))] = (blk.group(1), nm.group(1) if nm else os.path.basename(pm.group(1)))
        for sub, kind in TARGET_DIRS:
            base = os.path.join(ROOT, d, sub)
            if os.path.isdir(base):
                for fn in sorted(os.listdir(base)):
                    if fn.endswith(".rs"):
                        found.setdefault(norm(os.path.join(d, sub, fn)), (kind, fn[:-3]))
        main = norm(os.path.join(d, "src/main.rs"))
        if os.path.exists(os.path.join(ROOT, main)):
            found.setdefault(main, ("bin", "main"))
        lib = c.lib_rel
        for rel, (kind, nm) in found.items():
            if rel == lib:
                continue
            files, code, missing, rawdirs = U.reach(self.src, rel, d)
            if rel not in files or rel not in code:
                continue
            self.add_root(f"{pkg}#{kind}:{nm}", pkg, rel, files, code, kind)

    def build_tree(self, key, root, files, code):
        stack = [root]
        seen = set()
        self.fid[(key, root)] = root
        self.parent[(key, root)] = None
        while stack:
            rel = stack.pop()
            if rel in seen or rel not in code:
                continue
            seen.add(rel)
            tx, mt = self.texts[(key, rel)]
            d = os.path.dirname(rel)
            stem = os.path.splitext(os.path.basename(rel))[0]
            me = self.fid[(key, rel)]
            for m in U.INC_CODE_RX.finditer(tx):
                tgt = U.norm(os.path.join(d, m.group(1)))
                if tgt in code and (key, tgt) not in self.fid:
                    self.fid[(key, tgt)] = me
                    self.incl_site[(key, tgt)] = (rel, m.start())
                    stack.append(tgt)
            for kind, comps, name, pth in U.module_children(tx, mt):
                sub = os.path.join(*comps) if comps else ""
                cands = []
                if pth is not None:
                    cands.append(U.norm(os.path.join(d, sub, pth)))
                else:
                    for b_ in ([sub] if comps else [sub, os.path.join(sub, stem)]):
                        for suffix in (name + ".rs", os.path.join(name, "mod.rs")):
                            cands.append(U.norm(os.path.join(d, b_, suffix)))
                for cn in cands:
                    if cn in files and cn in code and (key, cn) not in self.fid:
                        self.fid[(key, cn)] = cn
                        self.parent[(key, cn)] = me
                        stack.append(cn)
                        break
            for m in U.PATH_RX.finditer(tx):
                tgt = U.norm(os.path.join(d, m.group(1)))
                if tgt in code and (key, tgt) not in self.fid:
                    self.fid[(key, tgt)] = tgt
                    self.parent[(key, tgt)] = me
                    stack.append(tgt)

    def closure(self):
        direct = {}
        for pkg, c in self.crates.items():
            direct[pkg] = [(k, d[2]) for d in self.mans.deps(c.man) for k in (d[0],) if d[2] in self.crates]
        norm_only = {p: {d for k, d in v if k != "dev-dependencies"} for p, v in direct.items()}
        out = {}
        for pkg in self.crates:
            seen = {pkg}
            stack = [d for k, d in direct[pkg]]
            while stack:
                x = stack.pop()
                if x in seen:
                    continue
                seen.add(x)
                stack.extend(norm_only[x])
            out[pkg] = seen
        return out

    def chain(self, key, rel):
        f = self.fid.get((key, rel), rel)
        out = [f]
        while self.parent.get((key, f)):
            f = self.parent[(key, f)]
            out.append(f)
        return out

    INLINE_RX = re.compile(r"\bmod\s+([A-Za-z_]\w*)\s*\{")

    def nest_at(self, key, rel, off):
        pre = ()
        site = self.incl_site.get((key, rel))
        if site:
            pre = self.nest_at(key, site[0], site[1])
        return pre + self.nest_local(key, rel, off)

    def nest_local(self, key, rel, off):
        k = (key, rel)
        if k not in self.nest_cache:
            mt = self.texts[k][1]
            spans = []
            for m in self.INLINE_RX.finditer(mt):
                e = balanced(mt, m.end() - 1)
                if e > 0:
                    spans.append((m.end() - 1, e, m.group(1)))
            self.nest_cache[k] = spans
        return tuple(n for s, e, n in sorted(self.nest_cache[k]) if s <= off <= e)

    IMPL_RX = re.compile(r"\bimpl\b")

    def impl_type_at(self, key, rel, off):
        k = (key, rel)
        if k not in self.impl_cache:
            mt = self.texts[k][1]
            spans = []
            n = len(mt)
            for m in self.IMPL_RX.finditer(mt):
                pre = mt[:m.start()].rstrip()
                if pre and pre[-1] not in "};]{" and not pre.endswith("unsafe") and not pre.endswith("default"):
                    continue
                j, a, p = m.end(), 0, 0
                while j < n:
                    ch = mt[j]
                    if ch == "-" and mt[j + 1:j + 2] == ">":
                        j += 2
                        continue
                    if ch == "<":
                        a += 1
                    elif ch == ">" and mt[j - 1] != "=":
                        a -= 1
                    elif ch in "([":
                        p += 1
                    elif ch in ")]":
                        p -= 1
                    elif ch == ";" and p == 0:
                        j = -1
                        break
                    elif ch == "{" and p == 0 and a <= 0:
                        break
                    j += 1
                if j < 0 or j >= n:
                    continue
                head = mt[m.end():j].strip()
                if head.startswith("<"):
                    e = balanced(head, 0, "<", ">")
                    head = head[e + 1:].strip() if e > 0 else head
                head = head.lstrip("!").strip()
                depth, cut = 0, None
                for fm in re.finditer(r"[<>]|\bfor\b(?!\s*<)", head):
                    t = fm.group(0)
                    if t == "<":
                        depth += 1
                    elif t == ">":
                        depth -= 1
                    elif depth == 0:
                        cut = fm.end()
                ty = head[cut:] if cut is not None else head
                ty = re.split(r"\bwhere\b", ty)[0]
                tm = re.match(r"^\s*(?:&\s*(?:'\w+\s+)?(?:mut\s+)?)?(?:[A-Za-z_]\w*::)*([A-Za-z_]\w*)", ty)
                e = balanced(mt, j)
                if tm and e > 0:
                    spans.append((j, e, tm.group(1)))
            self.impl_cache[k] = spans
        best = None
        for s, e, nm in self.impl_cache[k]:
            if s <= off <= e and (best is None or e - s < best[1] - best[0]):
                best = (s, e, nm)
        return best[2] if best else None

    DEF_RX = re.compile(r"\bstruct\s+([A-Za-z_]\w*)")

    def collect_defs(self, key):
        for (k2, rel), (tx, mt) in list(self.texts.items()):
            if k2 != key:
                continue
            for m in self.DEF_RX.finditer(mt):
                j, d, n, kind = m.end(), 0, len(mt), None
                while j < n:
                    ch = mt[j]
                    if ch == "-" and mt[j + 1:j + 2] == ">":
                        j += 2
                        continue
                    if ch == "<":
                        d += 1
                    elif ch == ">":
                        d -= 1
                    elif d == 0 and ch == "(" and re.search(r"\bwhere\b", mt[m.end():j]):
                        e = balanced(mt, j, "(", ")")
                        if e < 0:
                            break
                        j = e + 1
                        continue
                    elif d == 0 and ch in "{(;":
                        kind = ch
                        break
                    j += 1
                if kind != "{":
                    continue
                e = balanced(mt, j)
                if e < 0:
                    continue
                fields = {}
                for p in split_top(mt[j + 1:e]):
                    attrs_txt = re.sub(r"\s+", "", p[:600])
                    gated = 'feature="testkit"' in attrs_txt
                    neg = "cfg(not(any(test" in attrs_txt
                    p = re.sub(r"#\s*\[(?:[^\[\]]|\[[^\]]*\])*\]\s*", "", p).strip()
                    fm = re.match(r"^(pub(?:\s*\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:(?!:)", p)
                    if not fm:
                        continue
                    if neg:
                        fields.setdefault(fm.group(2), re.sub(r"\s+", "", (fm.group(1) or "").strip()) or "private")
                        continue
                    vis = re.sub(r"\s+", "", (fm.group(1) or "").strip()) or "private"
                    fields[fm.group(2)] = "testkit" if (gated and vis == "pub") else vis
                line = tx.count("\n", 0, m.start()) + 1
                sv = re.search(r"\b(pub(?:\s*\([^)]*\))?)\s*$", mt[max(0, m.start() - 40):m.start()])
                self.defs[m.group(1)].append(dict(key=key, owner=self.owner[key], rel=rel, line=line, fields=fields, nest=self.nest_at(key, rel, m.start()), svis=re.sub(r"\s+", "", sv.group(1)) if sv else "private"))

    ENUM_RX = re.compile(r"\benum\s+[A-Za-z_]\w*")

    def collect_variants(self, key):
        for (k2, rel), (tx, mt) in list(self.texts.items()):
            if k2 != key:
                continue
            for m in self.ENUM_RX.finditer(mt):
                j = mt.find("{", m.end())
                semi = mt.find(";", m.end())
                if j < 0 or (0 <= semi < j):
                    continue
                e = balanced(mt, j)
                if e < 0:
                    continue
                for p in split_top(mt[j + 1:e]):
                    p = re.sub(r"#\s*\[(?:[^\[\]]|\[[^\]]*\])*\]\s*", "", p).strip()
                    vm = re.match(r"^([A-Za-z_]\w*)\s*\{", p)
                    if not vm:
                        continue
                    ve = balanced(p, p.find("{"))
                    fields = {}
                    for q in split_top(p[p.find("{") + 1:ve if ve > 0 else len(p)]):
                        q = re.sub(r"#\s*\[(?:[^\[\]]|\[[^\]]*\])*\]\s*", "", q).strip()
                        fm = re.match(r"^([A-Za-z_]\w*)\s*:(?!:)", q)
                        if fm:
                            fields[fm.group(1)] = "pub"
                    self.defs[vm.group(1)].append(dict(key=key, owner=self.owner[key], rel=rel, line=tx.count("\n", 0, m.start()) + 1, fields=fields, nest=(), variant=True, svis="pub"))

    def visible(self, vis, d, key, rel, off):
        if vis in ("pub", "testkit"):
            return True
        if d["key"] != key:
            return False
        if vis.startswith("pub(crate)") or vis.startswith("pub(in"):
            return True
        ch = self.chain(key, rel)
        df = self.fid.get((d["key"], d["rel"]), d["rel"])
        if vis.startswith("pub(super)"):
            par = self.parent.get((d["key"], df))
            if d["nest"]:
                return df in ch
            return par is None or par in ch
        if df not in ch:
            return False
        if ch[0] == df:
            un = self.nest_at(key, rel, off)
            return un[:len(d["nest"])] == d["nest"]
        return True

    LIT_RX = re.compile(r"(?<![\w$.])((?:[A-Za-z_]\w*::)*)([A-Z]\w*)\s*(?:::\s*<[^<>{};]*>)?\s*\{")
    SKIP_PREV = re.compile(r"(?:\bstruct|\benum|\bunion|\btrait|\bimpl|\bfor|\bmod|\btype|\bdyn|\bwhere|\belse|\bunsafe|\basync|\bmove|->|=>)\s*$")

    ALIAS_RX = re.compile(r"\b([A-Za-z_]\w*)\s+as\s+([A-Z]\w*)\s*[,;}]")

    def alias_map(self):
        if not hasattr(self, "_alias"):
            self._alias = collections.defaultdict(lambda: collections.defaultdict(set))
            for (key, rel), (tx, mt) in self.texts.items():
                for m in self.ALIAS_RX.finditer(mt):
                    self._alias[self.owner[key]][m.group(2)].add(m.group(1))
        return self._alias

    def scan(self, users, cands=None):
        """Hits for use sites in the owners `users`; candidate structs come from the use-owner's dependency closure, restricted to `cands` owners when given."""
        hits = []
        self.amb = []
        self.unn = []
        for (key, rel), (tx, mt) in self.texts.items():
            own = self.owner[key]
            if own not in users:
                continue
            allowed = self.deps[own] if cands is None else (self.deps[own] & cands) | ({own} if own in cands else set())
            for m in self.LIT_RX.finditer(mt):
                name = m.group(2)
                if m.group(1) and m.group(1).rstrip(":").rsplit("::", 1)[-1][:1].isupper():
                    continue
                if name == "Self" and not m.group(1):
                    name = self.impl_type_at(key, rel, m.start())
                    if not name:
                        continue
                names = [name] + sorted(self.alias_map()[own].get(name, ()))
                if not any(n in self.defs for n in names):
                    continue
                allowed_u = allowed
                if m.group(1):
                    p0 = m.group(1).split("::")[0]
                    c0 = self.crates.get(own)
                    tg = (c0.aliases.get(p0) or c0.externs.get(p0)) if c0 else None
                    if tg and tg[0] == "ext":
                        continue
                    if tg and tg[0] == "fam" and tg[1] in self.deps:
                        allowed_u = allowed & self.deps[tg[1]]
                pre = mt[max(0, m.start() - 24):m.start()]
                if self.SKIP_PREV.search(pre):
                    continue
                s = m.end() - 1
                e = balanced(mt, s)
                if e < 0:
                    continue
                body = mt[s + 1:e]
                if not re.match(r"^\s*(\}|$|\.\.|[A-Za-z_]\w*\s*(:(?!:)|,|$))", body):
                    continue
                used, base = [], False
                for p in split_top(body):
                    if p.startswith(".."):
                        base = base or bool(p[2:].strip())
                        continue
                    fm = re.match(r"^([A-Za-z_]\w*)\s*(:(?!:)|$)", p)
                    if fm:
                        used.append(fm.group(1))
                if not used and not base:
                    continue
                results, hidden = [], []
                for d in [d for n in names for d in self.defs.get(n, ())]:
                    if d["key"] != key and (self.kind[d["key"]] != "lib" or d["owner"] not in allowed_u):
                        continue
                    if d["key"] == key and own not in allowed and cands is not None:
                        continue
                    if used and not all(f in d["fields"] for f in used):
                        continue
                    need = list(used)
                    if base:
                        need += [f for f in d["fields"] if f not in used]
                    bad = [f for f in need if not self.visible(d["fields"][f], d, key, rel, m.start())]
                    if d["key"] != key and d["svis"] != "pub":
                        if bad:
                            hidden.append((d, bad))
                        continue
                    results.append((d, bad))
                if hidden and not results:
                    line = tx.count("\n", 0, m.start()) + 1
                    self.unn.append(dict(user=own, root=key, file=rel, line=line, name=name, used=used, base=base, ctx="", defs=[dict(owner=d["owner"], key=d["key"], rel=d["rel"], line=d["line"], bad=b, vis=[d["fields"][f] for f in b], svis=d["svis"]) for d, b in hidden]))
                structs = [(d, b) for d, b in results if not d.get("variant")]
                if results and all(b for d, b in results) or (structs and all(b for d, b in structs)):
                    line = tx.count("\n", 0, m.start()) + 1
                    ls = tx.rfind("\n", 0, m.start()) + 1
                    le = tx.find("\n", m.start())
                    (hits if all(b for d, b in results) else self.amb).append(dict(user=own, root=key, file=rel, line=line, name=name, used=used, base=base, ctx=tx[ls:le if le > 0 else len(tx)].strip()[:160],
                                     defs=[dict(owner=d["owner"], key=d["key"], rel=d["rel"], line=d["line"], bad=b, vis=[d["fields"][f] for f in b]) for d, b in results]))
        return hits


def short(s):
    return s.replace("semio-framework-os-renderer-wgpu", "~").replace("semio-framework-", "sf-")


def selftest():
    fams = family_specs()
    pair = None
    for nm, pkgs in fams.items():
        w = World(pkgs, targets=False)
        for a in w.crates:
            for b in w.crates:
                if a != b and a in w.deps[b] and not any(b in w.deps[a] for _ in [0]):
                    pair = (nm, a, b)
                    break
            if pair:
                break
        if pair:
            break
    assert pair, "no dependent crate pair in any family"
    nm, a, b = pair
    probe_a = (
        "\nmod zz_a { pub struct ZzProbe { f: u8, pub g: u8, pub(super) h: u8 } }\nmod zz_b { fn t() { let _ = ZzProbe { f: 1, g: 2, h: 3 }; } }\n"
        "mod zz_a2 { use super::zz_a::*; fn t() { let _ = ZzProbe { f: 1, g: 2, h: 3 }; } }\n"
        "mod zz_c { pub struct ZzOk { f: u8 } mod zz_d { fn t() { let _ = super::ZzOk { f: 1 }; } } }\n"
        "pub struct ZzPub { pub a: u8, pub b: u8 }\npub struct ZzPriv { a: u8 }\npub struct ZzBase { pub a: u8, b: u8 }\npub struct ZzOrig { pub a: u8 }\npub(crate) struct ZzAlias { a: u8 }\npub(crate) struct ZzHid { a: u8 }\n"
        "pub struct ZzGate { #[cfg(any(test, feature = \"testkit\"))] #[doc(hidden)] pub t: u8, #[cfg(not(any(test, feature = \"testkit\")))] t: u8 }\n"
        "fn zz_same() { let _ = ZzPriv { a: 1 }; let _ = ZzBase { a: 1, b: 2 }; }\n"
        "mod zz_e { pub struct ZzSelf { f: u8 } }\nmod zz_g { impl super::zz_e::ZzSelf { fn mk() -> Self { Self { f: 1 } } } }\n"
    )
    probe_b = (
        "\nuse zz::ZzOrig as ZzAlias;\nfn zz_w() { let _ = ZzAlias { a: 1 }; let _ = ZzHid { a: 1 }; }\n"
        "\nfn zz_x() { let _ = ZzPub { a: 1, b: 2 }; let _ = ZzGate { t: 1 }; }\n"
        "fn zz_y(x: u8) { let _ = ZzPriv { a: 1 }; let ZzPriv { a } = x; let _ = ZzBase { a: 1, ..Default::default() }; }\n"
        "fn zz_z(x: u8) { let ZzPriv { .. } = x; let _ = ZzEnum::ZzQ { a: 1 }; let _ = ZzVar { v: 1 }; }\npub struct ZzQ { a: u8 }\npub struct ZzVar { v: u8 }\npub enum ZzEnum { ZzQ { zz: u8 }, ZzVar { v: u8 } }\n"
    )
    w0 = World([a, b], targets=False)
    ra, rb = w0.crates[a].lib_rel, w0.crates[b].lib_rel
    w = World([a, b], targets=False, extra=[(a, "zz-a-lib", ra, w0.texts[(a, ra)][0] + probe_a, "lib"), (b, "zz-b-lib", rb, w0.texts[(b, rb)][0] + probe_b, "lib"), (b, "zz-b-test", "zz/tests/t.rs", "fn zz_t() { let _ = ZzPriv { a: 1 }; let _ = ZzPub { a: 1, b: 2 }; }", "test"), (a, "zz-inc", "zz/inc_root.rs", {"zz/inc_root.rs": "mod zz_h { pub struct ZzInc { f: u8 } include!(\"zz_inc.rs\"); }\nmod zz_i { include!(\"zz_inc2.rs\"); }\n", "zz/zz_inc.rs": "mod tests { fn t() { let _ = super::ZzInc { f: 1 }; } }\n", "zz/zz_inc2.rs": "fn t() { let _ = ZzInc { f: 1 }; }\n"}, "lib")])
    got = sorted((h["root"], h["name"]) for h in w.scan({a, b}) if h["root"].startswith("zz-"))
    want = sorted([("zz-a-lib", "ZzProbe"), ("zz-a-lib", "ZzProbe"), ("zz-a-lib", "ZzSelf"), ("zz-b-lib", "ZzPriv"), ("zz-b-lib", "ZzPriv"), ("zz-b-lib", "ZzBase"), ("zz-b-test", "ZzPriv"), ("zz-inc", "ZzInc")])
    unn = sorted((h["root"], h["name"]) for h in w.unn if h["root"].startswith("zz-"))
    ok = got == want and unn == [("zz-b-lib", "ZzHid")]
    print("  unnameable (reported apart):", unn)
    print(f"selftest pair {short(a)} <- {short(b)} (family {nm})")
    print("  want", want)
    print("  got ", got)
    print("  legal cases not reported: a `mod tests` inside an `include!`d file of the defining inline module (hosted nest), renamed import (`ZzOrig as ZzAlias` hides the private `ZzAlias`), a `pub(crate)` struct of another crate (`ZzHid`), `ZzPriv { .. }` rest pattern, enum variant paths `ZzEnum::ZzQ`, a struct whose name is also a variant (`ZzVar`: ambiguity stays quiet), ZzOk (child), ZzPub (all pub, also from the integration-target root), ZzGate (testkit field), ZzPriv/ZzBase in the defining module")
    print("SELFTEST", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "selftest":
        sys.exit(selftest())
    args = sys.argv[2:] if len(sys.argv) > 1 and sys.argv[1] == "scan" else sys.argv[1:]
    tag = "scan"
    names = None
    targets = True
    i = 0
    while i < len(args):
        a = args[i]
        if a == "--tag":
            tag = args[i + 1]
            i += 1
        elif a == "--families":
            v = args[i + 1]
            names = None if v == "all" else set(v.split(","))
            i += 1
        elif a == "--no-targets":
            targets = False
        i += 1
    fams = family_specs(names)
    allpkgs = []
    for v in fams.values():
        for p in v:
            if p not in allpkgs:
                allpkgs.append(p)
    w = World(allpkgs, targets=targets, dep_closure=True)
    nroots = collections.Counter(w.kind.values())
    rows = []
    sites = {}
    amb = {}
    for nm, pkgs in fams.items():
        fam = set(pkgs) & set(w.crates)
        hits = w.scan(fam)
        for h in w.amb:
            amb.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
        for h in hits:
            sites.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
        rows.append((nm, len(pkgs), len(fam), len(hits)))
    uni = w.scan(set(allpkgs) & set(w.crates), None)
    for h in w.amb:
        amb.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
    for h in uni:
        sites.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
    unn = {}
    for nm, pkgs in fams.items():
        w.scan(set(pkgs) & set(w.crates))
        for h in w.unn:
            unn.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
    world = w.scan(set(w.crates), None)
    for h in w.amb:
        amb.setdefault((h["root"], h["file"], h["line"], h["name"]), h)
    world_only = [h for h in world if (h["root"], h["file"], h["line"], h["name"]) not in sites]
    out = dict(tag=tag, families={nm: dict(listed=a, loaded=b, hits=c) for nm, a, b, c in rows}, union_hits=len(uni), sites=list(sites.values()), ambiguous=list(amb.values()), world_only=world_only, world_hits=len(world), unnameable=list(unn.values()))
    json.dump(out, open(f"{GEN}/{PREFIX}-{tag}-e0451.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    nd = sum(1 for v in w.defs.values() for d in v if not d.get("variant"))
    print(f"{len(w.crates)} crates loaded ({len(set(allpkgs) & set(w.crates))} in the families), roots {dict(nroots)}, {len(w.texts)} files in module trees, {len(w.defs)} struct and variant names, {nd} brace-struct definitions")
    print("family\tlisted\tloaded\thits")
    for nm, a, b, c in rows:
        print(f"{nm}\t{a}\t{b}\t{c}")
    print(f"union (all crates, dependency-closure candidates): {len(uni)} hits; unique sites over families and union: {len(sites)}")
    print(f"world scan (every loaded crate as a user, outside the families too): {len(world)} hits, {len(world_only)} outside the families")
    for k, n in collections.Counter((short(h["user"]), short(h["defs"][0]["owner"]), h["name"]) for h in world_only).most_common(100):
        print(f"  W {n:3} {k[0]} -> {k[1]} {k[2]}")
    print(f"unnameable (the struct itself is private or pub(crate) in another root, so a first error precedes E0451; review by hand): {len(unn)}")
    for k, n in collections.Counter((short(h["root"]), short(h["defs"][0]["owner"]), h["name"]) for h in unn.values()).most_common(50):
        print(f"  U {n:3} {k[0]} -> {k[1]} {k[2]}")
    print(f"ambiguous (a private-field struct and an enum variant of the same name both match; review by hand): {len(amb)}")
    by = collections.Counter((short(h["user"]), short(h["defs"][0]["owner"]), h["name"]) for h in sites.values())
    for k, n in by.most_common(200):
        print(f"  {n:3} {k[0]} -> {k[1]} {k[2]}")


if __name__ == "__main__":
    main()
