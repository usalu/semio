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
        trait_leaf = set()
        for leaf in leaves:
            e = re.escape(leaf)
            pats = [r"\bimpl\s+(?:<[^>]*>\s*)?(?:[\w:]+::)?" + e + r"(?:<[^>]*>)?\s+for\b", r"\bdyn\s+(?:[\w:]+::)?" + e + r"\b", r"->\s*impl\s+(?:[\w:]+::)?" + e + r"\b", r"\bimpl\s+(?:[\w:]+::)?" + e + r"\b(?!\s+for)", r"(?:\bdyn|\bimpl)\s+[^;{]*?\+\s*(?:[\w:]+::)?" + e + r"\b"]
            for p in pats:
                if re.search(p, alltext):
                    trait_leaf.add(leaf)
                    break
        for m in re.finditer(r"\bwhere\b([^{;]*)[{;]", alltext):
            for b in re.finditer(r":\s*([^,:][^,]*)", m.group(1)):
                for tl in re.findall(r"(?:[\w:]+::)?([A-Z]\w*)", b.group(1)):
                    if tl in leaves:
                        trait_leaf.add(tl)
        for m in re.finditer(r"\b(?:fn\s+\w+|impl|struct\s+\w+|enum\s+\w+|trait\s+\w+|type\s+\w+)\s*<", alltext):
            pos = m.end() - 1
            depth = 0
            j = pos
            while j < len(alltext) and j < pos + 600:
                if alltext[j] == "<":
                    depth += 1
                elif alltext[j] == ">" and alltext[j - 1] != "-":
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            seg = alltext[pos + 1:j]
            for b in re.finditer(r"\b\w+\s*:\s*([^,]*(?:<[^<>]*>[^,]*)*)", seg):
                for tl in re.findall(r"(?:[\w:]+::)?([A-Z]\w*)", b.group(1)):
                    if tl in leaves:
                        trait_leaf.add(tl)

        def mark(kids):
            for name, n in kids.items():
                if name in trait_leaf and not n.kids:
                    n.kind = "trait"
                mark(n.kids)

        mark(self.root)
        for leaf in leaves:
            L = T = 0
            for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(leaf) + r"\s*(?:::\s*)?<", alltext):
                r = generic_args(alltext, m.end() - 1)
                if r:
                    L, T = max(L, r[0]), max(T, r[1])
            if L or T:
                self.arity[leaf] = (L, T)

    def add_name(self, modpath, name, kind):
        return self.node(list(modpath) + [name], kind)

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
                if n.kids:
                    if cap:
                        variants = [k for k in sorted(n.kids) if re.match(r"^[A-Z]", k) and not is_const(k) and not n.kids[k].kids]
                        others = [k for k in sorted(n.kids) if k not in variants]
                        stubv = f"__Stub(std::marker::PhantomData<{phantom}>)" if decl else "__Stub"
                        out.append(f"{ind}{DER}")
                        out.append(f"{ind}pub enum {name}{decl} {{ {', '.join(variants)}{', ' if variants else ''}{stubv} }}")
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

        for crate in sorted(self.root):
            out.append(f"    pub mod {crate} {{")
            emit(self.root[crate].kids, "        ")
            out.append("    }")
        out.append("}")
        return "\n".join(out) + "\n"


DEFINED_RX = re.compile(r"\b(?:struct|enum|union|trait|type|fn|const|static|mod)\s+(" + IDENT + r")|macro_rules!\s*(" + IDENT + r")|\bas\s+(" + IDENT + r")")
