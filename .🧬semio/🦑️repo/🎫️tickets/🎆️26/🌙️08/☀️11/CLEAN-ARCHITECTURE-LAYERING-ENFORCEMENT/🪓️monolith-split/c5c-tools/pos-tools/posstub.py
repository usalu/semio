"""🧩 POS stub layer: a generated `__ext` module that stands in for every crate outside the union, so rustc reaches type-check (privacy errors E0603/E0616/E0624 only surface there).
Derived from frs-tools/stubgen.py. Differences: works on text that already had its crate heads rewritten to `crate::__ext::<crate>`, builds a typed tree (mod/struct/enum/trait/fn/const),
gives stub types the std derives, records the module paths of external glob imports and lets the runner grow those modules from unresolved-name diagnostics (names defined inside the union are never stubbed)."""
import re

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
MASK_RX = re.compile(r"//[^\n]*|/\*.*?\*/|(?<![A-Za-z0-9_])b?r(#*)\".*?\"\1|b?\"(?:\\.|[^\"\\])*\"|b?'(?:\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", re.S)


def mask(text):
    return MASK_RX.sub(lambda m: re.sub(r"[^\n]", " ", m.group(0)), text)


def expand_use(tree, prefix=""):
    tree = tree.strip()
    parts, cur, depth = [], "", 0
    for ch in tree:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    if len(parts) > 1:
        out = []
        for p in parts:
            if p.strip():
                out += expand_use(p, prefix)
        return out
    t = parts[0].strip()
    m = re.match(r"^(.*?)::\s*\{(.*)\}$", t, re.S)
    if m:
        return expand_use(m.group(2), prefix + m.group(1).strip() + "::")
    if t.startswith("{") and t.endswith("}"):
        return expand_use(t[1:-1], prefix)
    t = re.sub(r"\s+as\s+" + IDENT + "$", "", t)
    full = prefix + t
    if full.endswith("::*"):
        return [("glob", [s.strip() for s in full[:-3].split("::") if s.strip()])]
    if full.endswith("::self"):
        return [("mod", [s.strip() for s in full[:-6].split("::") if s.strip()])]
    return [("item", [s.strip() for s in full.split("::") if s.strip()])]


def use_aliases(tree, prefix=""):
    """[(alias, segs)] of every leaf of a use tree (alias = last segment unless `as`)."""
    out = []
    for kind, segs in expand_use(tree, prefix):
        pass
    return out


def split_top(s, sep=","):
    out, cur, d = [], "", 0
    for i, ch in enumerate(s):
        if ch in "<([{":
            d += 1
        elif ch in ">)]}" and not (ch == ">" and i > 0 and s[i - 1] == "-"):
            d -= 1
        if ch == sep and d == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    out.append(cur)
    return [x.strip() for x in out if x.strip()]


def split_top_spans(s, base, sep=","):
    out, start, d = [], 0, 0
    for i, ch in enumerate(s):
        if ch in "<([{":
            d += 1
        elif ch in ">)]}" and not (ch == ">" and i > 0 and s[i - 1] == "-"):
            d -= 1
        if ch == sep and d == 0:
            out.append((base + start, base + i, s[start:i]))
            start = i + 1
    out.append((base + start, base + len(s), s[start:]))
    return out


def bound_spans(text, base):
    """(start, end, path) of every trait path named in a bound list; start/end are absolute (base + offset)."""
    out = []
    for ps, pe, part in split_top_spans(text, base, "+"):
        lead = len(part) - len(part.lstrip())
        body = part.lstrip()
        q = 0
        if body.startswith("?"):
            q = 1
            body = body[1:]
        fm = re.match(r"^for\s*<[^>]*>\s*", body)
        if fm:
            q += fm.end()
            body = body[fm.end():]
        m = re.match(r"^((?:::)?[A-Za-z_]\w*(?:\s*::\s*[A-Za-z_]\w*)*)", body)
        if m and not body.startswith("'"):
            s = ps + lead + q
            e = s + m.end(1)
            out.append((s, e, re.sub(r"\s+", "", m.group(1))))
    return out


def bound_paths(text):
    return [p for _, _, p in bound_spans(text, 0)]


def impl_header_span(masked, pos):
    i = pos + 4
    d, j = 0, i
    while j < len(masked):
        ch = masked[j]
        if ch in "<([":
            d += 1
        elif ch in ">)]" and not (ch == ">" and masked[j - 1] == "-"):
            d -= 1
        elif ch in "{;" and d <= 0:
            break
        j += 1
    return i, j


def trait_position_spans(t):
    """[(start, end, path)] of trait paths in trait positions (impl headers, generic/where bounds, supertraits, dyn/impl Trait) of one masked file."""
    out = []
    for m in re.finditer(r"(?:^|(?<=[;}{\]\s]))(?:unsafe\s+)?impl\b", t):
        a, b = impl_header_span(t, t.index("impl", m.start()))
        h = t[a:b]
        k = re.search(r"\bwhere\b", h)
        hw = h[:k.start()] if k else h
        wh_off = a + k.end() if k else None
        off = a
        lead = len(hw) - len(hw.lstrip())
        hs = hw.lstrip()
        off += lead
        if hs.startswith("<"):
            d = 0
            for i, ch in enumerate(hs):
                if ch == "<":
                    d += 1
                elif ch == ">" and hs[i - 1] != "-":
                    d -= 1
                    if d == 0:
                        gen = hs[1:i]
                        for gs, ge, g in split_top_spans(gen, off + 1):
                            if ":" in g and not g.strip().startswith("'"):
                                ci = g.index(":")
                                out += bound_spans(g[ci + 1:], gs + ci + 1)
                        off += i + 1
                        hs = hs[i + 1:]
                        break
        l2 = len(hs) - len(hs.lstrip())
        hs2 = hs.lstrip()
        off += l2
        if hs2.startswith("!"):
            hs2 = hs2[1:]
            off += 1
        fm = None
        for fm_ in re.finditer(r"\bfor\b(?![^<]*>)", hs2):
            fm = fm_
            break
        if fm:
            out += bound_spans(hs2[:fm.start()], off)
        if wh_off is not None:
            wtxt = h[k.end():]
            for ws, we, w in split_top_spans(wtxt, wh_off):
                if ":" in w:
                    ci = w.index(":")
                    out += bound_spans(w[ci + 1:], ws + ci + 1)
    for m in re.finditer(r"\b(?:fn\s+\w+|struct\s+\w+|enum\s+\w+|trait\s+\w+|type\s+\w+|union\s+\w+)\s*<", t):
        pos = m.end() - 1
        d, j = 0, pos
        while j < len(t) and j < pos + 2000:
            if t[j] == "<":
                d += 1
            elif t[j] == ">" and t[j - 1] != "-":
                d -= 1
                if d == 0:
                    break
            j += 1
        for gs, ge, g in split_top_spans(t[pos + 1:j], pos + 1):
            gst = g.strip()
            if ":" in g and not gst.startswith("'") and not gst.startswith("const"):
                ci = g.index(":")
                out += bound_spans(g[ci + 1:], gs + ci + 1)
    for m in re.finditer(r"\bwhere\b([^{;]*?)(?=\{|;)", t):
        for ws, we, w in split_top_spans(m.group(1), m.start(1)):
            if ":" in w:
                ci = w.index(":")
                out += bound_spans(w[ci + 1:], ws + ci + 1)
    for m in re.finditer(r"\btrait\s+\w+(?:<[^{;]*?>)?\s*:\s*([^{;]*?)(?=\{|where|;)", t):
        out += bound_spans(m.group(1), m.start(1))
    for m in re.finditer(r"\b(dyn|impl)\s+((?:for\s*<[^>]*>\s*)?[A-Za-z_:][^,;{}=)\]]*)", t):
        if m.group(1) == "impl":
            before = t[max(0, m.start() - 40):m.start()].rstrip()
            if before == "" or before[-1] in ";}{]" or before.endswith("unsafe"):
                continue
        out += bound_spans(m.group(2), m.start(2))
    seen, uniq = set(), []
    for s, e, p in out:
        if (s, e) not in seen and p:
            seen.add((s, e))
            uniq.append((s, e, p))
    return sorted(uniq)


def trait_positions(t):
    return [p for _, _, p in trait_position_spans(t)]


def generic_args(text, pos):
    depth, i, n = 0, pos, len(text)
    args, cur = [], ""
    while i < n:
        ch = text[i]
        if ch == "<":
            depth += 1
            if depth > 1:
                cur += ch
        elif ch == ">":
            if i > 0 and text[i - 1] == "-":
                cur += ch
            else:
                depth -= 1
                if depth == 0:
                    args.append(cur)
                    break
                cur += ch
        elif ch == "," and depth == 1:
            args.append(cur)
            cur = ""
        elif ch in ";{}":
            return None
        else:
            cur += ch
        i += 1
    else:
        return None
    args = [a.strip() for a in args if a.strip()]
    return sum(1 for a in args if a.startswith("'")), sum(1 for a in args if not a.startswith("'"))


class Node:
    __slots__ = ("kind", "kids")

    def __init__(self, kind=None):
        self.kind = kind
        self.kids = {}


class Stub:
    HEAD = "crate::__ext::"

    def __init__(self):
        self.root = {}
        self.arity = {}
        self.defined = set()

    def node(self, segs, kind=None):
        if any(not re.fullmatch(IDENT, s) for s in segs):
            return None
        cur = self.root
        n = None
        for s in segs:
            n = cur.get(s)
            if n is None:
                n = Node()
                cur[s] = n
            cur = n.kids
        if n is not None and kind:
            if kind == "trait":
                n.kind = "trait"
            elif n.kind is None:
                n.kind = kind
        return n

    def scan(self, masked_texts):
        """Learn items from rewritten (masked) texts: explicit paths, use trees, trait positions, generic arity."""
        head = re.escape(self.HEAD)
        use_rx = re.compile(r"(?:^|[;{}\s])use\s+" + head + r"([^;]*);", re.S)
        path_rx = re.compile(r"(?<![A-Za-z0-9_\.])" + head + r"(" + IDENT + r"(?:\s*::\s*" + IDENT + r")*)")
        alltext = "\n".join(masked_texts)
        for t in masked_texts:
            for m in use_rx.finditer(t):
                for kind, segs in expand_use(m.group(1)):
                    if not segs:
                        continue
                    if kind == "glob":
                        self.node(segs, "mod")
                    elif kind == "mod":
                        self.node(segs, "mod")
                    else:
                        self.node(segs)
            for m in path_rx.finditer(t):
                segs = [s.strip() for s in re.split(r"\s*::\s*", m.group(1))]
                tail = t[m.end(1):m.end(1) + 3].lstrip()
                if tail.startswith("!"):
                    continue
                self.node(segs)
        leaves = set()

        def walk(kids):
            for name, n in kids.items():
                if re.match(r"^[A-Z]", name):
                    leaves.add(name)
                walk(n.kids)

        walk(self.root)
        self.trait_evidence = 0
        for t in masked_texts:
            umap = {}
            for um in use_rx.finditer(t):
                for alias, segs in self._aliases(um.group(1)):
                    umap[alias] = segs
            for pth in trait_positions(t):
                segs = [s for s in pth.split("::") if s]
                if pth.startswith("crate::__ext::") or (segs[:2] == ["crate", "__ext"]):
                    full = segs[2:]
                elif segs and segs[0] in umap:
                    full = umap[segs[0]] + segs[1:]
                else:
                    continue
                if not full:
                    continue
                n = self.node(full)
                if n is not None and not n.kids and re.match(r"^[A-Z]", full[-1]):
                    n.kind = "trait"
                    self.trait_evidence += 1
                elif n is not None and re.match(r"^[A-Z]", full[-1]):
                    n.kind = "trait"
                    self.trait_evidence += 1
        for m in re.finditer(r"(?<![A-Za-z0-9_])([A-Z]\w*)\s*(?:::\s*)?<", alltext):
            leaf = m.group(1)
            if leaf not in leaves:
                continue
            r = generic_args(alltext, m.end() - 1)
            if r:
                L, T = self.arity.get(leaf, (0, 0))
                self.arity[leaf] = (max(L, r[0]), max(T, r[1]))

    def _aliases(self, tree):
        out = []

        def rec(tr, prefix):
            parts = split_top(tr.strip())
            if len(parts) > 1:
                for p in parts:
                    rec(p, prefix)
                return
            x = parts[0].strip() if parts else ""
            m = re.match(r"^(.*?)::\s*\{(.*)\}$", x, re.S)
            if m:
                rec(m.group(2), prefix + m.group(1).strip() + "::")
                return
            if x.startswith("{") and x.endswith("}"):
                rec(x[1:-1], prefix)
                return
            am = re.match(r"^(.*?)\s+as\s+([A-Za-z_]\w*)$", x)
            alias = None
            if am:
                x, alias = am.group(1).strip(), am.group(2)
            full = prefix + x
            if full.endswith("::*"):
                return
            segs = [s.strip() for s in full.split("::") if s.strip()]
            if segs and segs[-1] == "self":
                segs = segs[:-1]
            if segs:
                out.append((alias or segs[-1], segs))

        rec(tree, "")
        return out

    def add_name(self, modpath, name, kind):
        return self.node(list(modpath) + [name], kind)

    def mark_trait(self, name):
        hit = [0]

        def walk(kids):
            for k, n in kids.items():
                if k == name and n.kind != "trait" and re.match(r"^[A-Z]", k):
                    n.kind = "trait"
                    hit[0] += 1
                walk(n.kids)

        walk(self.root)
        return hit[0]

    def mark_struct(self, name):
        hit = [0]

        def walk(kids):
            for k, n in kids.items():
                if k == name and not n.kids and n.kind == "trait":
                    n.kind = "struct"
                    hit[0] += 1
                walk(n.kids)

        walk(self.root)
        return hit[0]

    def has(self, modpath, name):
        cur = self.root
        n = None
        for s in list(modpath) + [name]:
            n = cur.get(s)
            if n is None:
                return False
            cur = n.kids
        return True

    def gen_decl(self, name):
        L, T = self.arity.get(name, (0, 0))
        ps = [f"'a{i}" for i in range(L)] + [f"T{i} = ()" for i in range(T)]
        if not ps:
            return "", "", ""
        decl = "<" + ", ".join(ps) + ">"
        args = "<" + ", ".join([f"'a{i}" for i in range(L)] + [f"T{i}" for i in range(T)]) + ">"
        phantom = "(" + ", ".join([f"&'a{i} ()" for i in range(L)] + [f"T{i}" for i in range(T)]) + ",)"
        return decl, args, phantom

    def render(self):
        out = ["#[allow(dead_code, unused, non_snake_case, non_camel_case_types, non_upper_case_globals, missing_docs)]", "pub mod __ext {"]
        DER = "#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]"

        def is_const(n):
            return re.match(r"^[A-Z][A-Z0-9_]+$", n) is not None

        def emit(kids, ind):
            for name in sorted(kids):
                n = kids[name]
                cap = re.match(r"^[A-Z]", name) is not None
                decl, args, phantom = self.gen_decl(name)
                if n.kids and n.kind == "trait":
                    d, _, _ = self.gen_decl(name)
                    out.append(f"{ind}pub trait {name}{d} {{")
                    for k in sorted(n.kids):
                        if re.match(r"^[a-z_]", k):
                            out.append(f"{ind}    fn {k}() {{}}")
                    out.append(f"{ind}}}")
                    continue
                if n.kids:
                    if cap:
                        variants = [k for k in sorted(n.kids) if re.match(r"^[A-Z]", k) and not is_const(k) and not n.kids[k].kids]
                        others = [k for k in sorted(n.kids) if k not in variants]
                        if variants:
                            stubv = f"__Stub(std::marker::PhantomData<{phantom}>)" if decl else "__Stub"
                            out.append(f"{ind}{DER}")
                            out.append(f"{ind}pub enum {name}{decl} {{ {', '.join(variants)}{', ' if variants else ''}{stubv} }}")
                        else:
                            out.append(f"{ind}{DER}")
                            out.append(f"{ind}pub struct {name}{decl}(std::marker::PhantomData<{phantom}>);" if decl else f"{ind}pub struct {name};")
                        if others:
                            out.append(f"{ind}impl{decl.replace(' = ()', '')} {name}{args} {{")
                            for k in others:
                                out.append(f"{ind}    pub const {k}: usize = 0;" if is_const(k) else f"{ind}    pub fn {k}() {{}}")
                            out.append(f"{ind}}}")
                    else:
                        out.append(f"{ind}pub mod {name} {{")
                        emit(n.kids, ind + "    ")
                        out.append(f"{ind}}}")
                    continue
                k = n.kind
                if k == "mod":
                    out.append(f"{ind}pub mod {name} {{}}")
                elif k == "trait":
                    d, _, _ = self.gen_decl(name)
                    out.append(f"{ind}pub trait {name}{d} {{}}")
                elif k == "fn" or (k is None and not cap):
                    out.append(f"{ind}pub fn {name}() {{}}")
                elif is_const(name):
                    out.append(f"{ind}pub const {name}: usize = 0;")
                elif cap:
                    out.append(f"{ind}{DER}")
                    out.append(f"{ind}pub struct {name}{decl}(std::marker::PhantomData<{phantom}>);" if decl else f"{ind}pub struct {name};")
                else:
                    out.append(f"{ind}pub fn {name}() {{}}")

        out.append("    pub mod __unresolved {")
        for i in range(1, getattr(self, "n_unresolved", 0) + 1):
            out.append(f"        pub trait Tr{i}<A0 = (), A1 = (), A2 = (), A3 = (), A4 = (), A5 = ()> {{}}")
        out.append("    }")
        for crate in sorted(self.root):
            out.append(f"    pub mod {crate} {{")
            emit(self.root[crate].kids, "        ")
            out.append("    }")
        out.append("}")
        return "\n".join(out) + "\n"


DEFINED_RX = re.compile(r"\b(?:struct|enum|union|trait|type|fn|const|static|mod)\s+(" + IDENT + r")|macro_rules!\s*(" + IDENT + r")|\bas\s+(" + IDENT + r")")
