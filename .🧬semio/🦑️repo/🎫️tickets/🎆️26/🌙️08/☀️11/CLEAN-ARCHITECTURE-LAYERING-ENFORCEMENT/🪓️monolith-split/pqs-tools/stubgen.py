"""Sandbox-only stub generator: rewrites paths rooted at unresolved external crates to `crate::__ext::<crate>::...`
and appends a generated stub module to the lib root so rustc proceeds to type-check (privacy errors E0616/E0624/E0451
only surface in the typeck phase, which unresolved crates otherwise cut short). Applied identically to baseline and
current sandbox trees; never to the repo."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from pqslib import *

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
NOT_BEFORE = r"(?<![A-Za-z0-9_\.])(?<!::)"


def rs_files(root):
    out = []
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                out.append(os.path.join(dp, f))
    return out


def mask(text):
    """Blank out comments and string/char literals so path scans only see code (length preserved)."""
    out = list(text)
    n = len(text)
    i = 0
    while i < n:
        c = text[i]
        if c == "/" and text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
            continue
        if c == "/" and text.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if text.startswith("/*", j): d += 1; j += 2
                elif text.startswith("*/", j): d -= 1; j += 2
                else: j += 1
            for k in range(i, j):
                if out[k] != "\n": out[k] = " "
            i = j
            continue
        m = re.match(r'b?r(#*)"', text[i:i + 40]) if c in "br" else None
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            close = '"' + m.group(1)
            j = text.find(close, i + m.end())
            j = n if j < 0 else j + len(close)
            for k in range(i, j):
                if out[k] != "\n": out[k] = " "
            i = j
            continue
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(n, j + 1)
            for k in range(i, j):
                if out[k] != "\n": out[k] = " "
            i = j
            continue
        if c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i + 16])
            if m:
                for k in range(i, i + m.end()):
                    out[k] = " "
                i += m.end()
                continue
        i += 1
    return "".join(out)


def expand_use(tree, prefix=""):
    """Expand a use tree string (without `use`/`;`) into [(kind, segs)]; globs keep kind glob."""
    tree = tree.strip()
    parts, cur, depth = [], "", 0
    for ch in tree:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    paths = []
    if len(parts) > 1:
        for p in parts:
            if p.strip():
                paths += expand_use(p, prefix)
        return paths
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


TRAIT_RX = [
    r"\bimpl\s+(?:<[^>]*>\s*)?(?:\w+::)*({I})(?:<[^>]*>)?\s+for\b",
    r"\bdyn\s+(?:\w+::)*({I})\b",
    r"->\s*impl\s+(?:\w+::)*({I})\b",
    r"(?:\bdyn|\bimpl)\s+[^;{{]*?\+\s*(?:\w+::)*({I})\b",
]


def collect(files, crates):
    """Return {crate: {segs tuple: kind}}; kind in item|trait|macro|glob|mod."""
    found = {c: {} for c in crates}
    alt = "|".join(sorted(map(re.escape, crates), key=len, reverse=True))
    use_rx = re.compile(r"(?:^|[;{}\s])use\s+(?:::)?((?:" + alt + r")\b[^;]*);", re.S)
    path_rx = re.compile(NOT_BEFORE + r"(?:::)?((?:" + alt + r")(?:\s*::\s*" + IDENT + r")+)")
    texts = {p: mask(read(p)) for p in files}
    for p, t in texts.items():
        for m in use_rx.finditer(t):
            for kind, segs in expand_use(m.group(1)):
                if segs and segs[0] in found:
                    found[segs[0]].setdefault(tuple(segs[1:]), kind)
        for m in path_rx.finditer(t):
            segs = [s.strip() for s in re.split(r"\s*::\s*", m.group(1))]
            start = m.start(1)
            if "#[" in t[max(0, start - 3):start]:
                continue
            tail = t[m.end(1):m.end(1) + 3]
            kind = "macro" if tail.lstrip().startswith("!") else "item"
            found[segs[0]].setdefault(tuple(segs[1:]), kind)
    alltext = "\n".join(texts.values())
    traitish = set()
    for crate, d in found.items():
        for segs in list(d):
            if not segs:
                continue
            leaf = segs[-1]
            if not re.match(r"^[A-Z]", leaf) or leaf in traitish:
                continue
            for rx in TRAIT_RX:
                if re.search(rx.format(I=re.escape(leaf)), alltext):
                    traitish.add(leaf)
                    break
    for crate, d in found.items():
        for segs in list(d):
            if segs and segs[-1] in traitish and d[segs] not in ("macro",):
                d[segs] = "trait"
    for crate, d in found.items():
        for segs in list(d):
            if segs and segs[-1] in HINT_TRAITS and d[segs] not in ("macro",):
                d[segs] = "trait"
    ARITY.clear()
    ARITY.update(infer_arity(alltext, found))
    infer_variants(alltext, found)
    return found


ARITY = {}
VARIANT_ARITY = {}
HINT_TRAITS = {"Deserializer", "Serializer", "DeserializeOwned", "Deserialize", "Serialize", "NativeSchemaControl"}


def balanced_arity(text, pos):
    """Number of top-level comma separated fields in the parenthesised group opening at text[pos] == '('."""
    depth, n, seen, i = 0, 0, False, pos
    while i < len(text):
        ch = text[i]
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
            if depth == 0:
                return (n + 1) if seen else 0
        elif ch == "," and depth == 1:
            n += 1
            seen = False
            i += 1
            continue
        if depth >= 1 and not ch.isspace() and not (ch == "(" and depth == 1):
            seen = True
        i += 1
    return None


def infer_variants(alltext, found):
    """Sandbox stub quality: enum variants named anywhere as `Enum::Variant` (also through an imported alias) and their
    tuple arity from the call or pattern that follows, so patterns resolve and the type-check phase runs."""
    VARIANT_ARITY.clear()
    for crate, d in found.items():
        enums = {}
        for segs in list(d):
            if len(segs) >= 2 and re.match(r"^[A-Z]", segs[-2]) and re.match(r"^[A-Z]", segs[-1]):
                enums[segs[-2]] = segs[:-1]
        for name, prefix in enums.items():
            for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"\s*::\s*([A-Z][A-Za-z0-9_]*)\b", alltext):
                variant = m.group(1)
                d.setdefault(tuple(prefix) + (variant,), "item")
                after = m.end()
                while after < len(alltext) and alltext[after] in " \t":
                    after += 1
                if after < len(alltext) and alltext[after] == "(":
                    n = balanced_arity(alltext, after)
                    if n is not None:
                        VARIANT_ARITY[(name, variant)] = max(n, VARIANT_ARITY.get((name, variant), 0))


def generic_args(text, pos):
    """Parse `<...>` starting at text[pos] == '<'; return (lifetimes, types) or None."""
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
            args.append(cur); cur = ""
        elif ch in ";{}":
            return None
        else:
            cur += ch
        i += 1
    else:
        return None
    args = [a.strip() for a in args if a.strip()]
    life = sum(1 for a in args if a.startswith("'"))
    typ = sum(1 for a in args if not a.startswith("'"))
    return life, typ


def infer_arity(alltext, found):
    leaves = set()
    for crate, d in found.items():
        for segs, kind in d.items():
            if segs and re.match(r"^[A-Z]", segs[-1]) and kind != "macro":
                leaves.add(segs[-1])
    out = {}
    for leaf in leaves:
        L = T = 0
        for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(leaf) + r"\s*(?:::\s*)?<", alltext):
            r = generic_args(alltext, m.end() - 1)
            if r:
                L, T = max(L, r[0]), max(T, r[1])
        if L or T:
            out[leaf] = (L, T)
    return out


def short_args(name):
    L, T = ARITY.get(name, (0, 0))
    ps = [f"'a{i}" for i in range(L)] + [f"T{i}" for i in range(T)]
    return "<" + ", ".join(ps) + ">" if ps else ""


def generics_decl(name, with_phantom):
    L, T = ARITY.get(name, (0, 0))
    ps = [f"'a{i}" for i in range(L)] + [f"T{i} = ()" for i in range(T)]
    if not ps:
        return "", ""
    decl = "<" + ", ".join(ps) + ">"
    phantom = "(" + ", ".join([f"&'a{i} ()" for i in range(L)] + [f"T{i}" for i in range(T)]) + ",)"
    return decl, phantom


def strip_ext_attrs(text, crates):
    """Sandbox-only: drop derive entries and attributes that only external proc-macro crates can provide."""
    alt = "|".join(sorted(map(re.escape, crates), key=len, reverse=True))
    t = text

    def fix_derive(m):
        items = [s.strip() for s in m.group(1).split(",") if s.strip()]
        keep = [s for s in items if not re.match(r"^(?:crate::__ext::|::)?(?:" + alt + r")\b", s)]
        return "derive(" + ", ".join(keep) + ")"

    t = re.sub(r"derive\(([^()]*)\)", fix_derive, t)
    t = re.sub(r"#\[\s*serde\s*\([^\]]*\)\s*\]", "", t)
    t = re.sub(r"#!?\[\s*(?:crate::__ext::)?(?:" + alt + r")(?:\s*::\s*\w+)+(?:\s*\([^\]]*\))?\s*\]", "", t)
    t = re.sub(r"#\[\s*cfg_attr\s*\(\s*test\s*,\s*\)\s*\]", "", t)
    return t


def build_stub(found):
    tree = {}

    def add(crate, segs, kind):
        node = tree.setdefault(crate, {})
        for s in segs[:-1]:
            nxt = node.get(s)
            if not isinstance(nxt, dict):
                nxt = {}
                node[s] = nxt
            node = nxt
        if segs:
            leaf = segs[-1]
            if leaf not in node:
                node[leaf] = kind

    for crate, d in found.items():
        for segs, kind in sorted(d.items(), key=lambda kv: len(kv[0])):
            add(crate, list(segs), kind)
    out = ["#[allow(dead_code, unused, non_snake_case, non_camel_case_types, missing_docs)]", "pub mod __ext {", "    pub struct __Any;"]

    def is_const(n):
        return re.match(r"^[A-Z][A-Z0-9_]+$", n) is not None

    def emit(node, ind):
        for name in sorted(node):
            v = node[name]
            if isinstance(v, dict):
                if re.match(r"^[A-Z]", name):
                    variants = [k for k in sorted(v) if re.match(r"^[A-Z]", k) and not is_const(k) and not isinstance(v[k], dict)]
                    others = [k for k in sorted(v) if k not in variants]
                    decl, phantom = generics_decl(name, True)
                    stubv = f"__Stub(std::marker::PhantomData<{phantom}>)" if decl else "__Stub"
                    vtxt = [k + ("(" + ", ".join(["crate::__ext::__Any"] * VARIANT_ARITY[(name, k)]) + ")" if VARIANT_ARITY.get((name, k), 0) > 0 else "") for k in variants]
                    if variants:
                        out.append(f"{ind}pub enum {name}{decl} {{ {', '.join(vtxt)}, {stubv} }}")
                    else:
                        out.append(f"{ind}pub struct {name}{decl}(std::marker::PhantomData<{phantom}>);" if decl else f"{ind}pub struct {name};")
                    if others:
                        out.append(f"{ind}impl{decl.replace(' = ()', '')} {name}{short_args(name)} {{")
                        for k in others:
                            out.append(f"{ind}    pub const {k}: usize = 0;" if is_const(k) else f"{ind}    pub fn {k}() {{}}")
                        out.append(f"{ind}}}")
                else:
                    out.append(f"{ind}pub mod {name} {{")
                    emit(v, ind + "    ")
                    out.append(f"{ind}}}")
            else:
                if v in ("macro", "glob"):
                    continue
                if v == "trait":
                    decl, _ = generics_decl(name, False)
                    out.append(f"{ind}pub trait {name}{decl} {{}}")
                elif is_const(name):
                    out.append(f"{ind}pub const {name}: usize = 0;")
                elif re.match(r"^[A-Z]", name):
                    decl, phantom = generics_decl(name, True)
                    out.append(f"{ind}pub struct {name}{decl}(std::marker::PhantomData<{phantom}>);" if decl else f"{ind}pub struct {name};")
                elif v == "mod":
                    out.append(f"{ind}pub mod {name} {{}}")
                else:
                    out.append(f"{ind}pub fn {name}() {{}}")

    for crate in sorted(tree):
        out.append(f"    pub mod {crate} {{")
        emit(tree[crate], "        ")
        out.append("    }")
    out.append("}")
    return "\n".join(out) + "\n"


def rewrite(text, crates):
    t = mask(text)
    alt = "|".join(sorted(map(re.escape, crates), key=len, reverse=True))
    rx = re.compile(NOT_BEFORE + r"(?:::)?(" + alt + r")\b(?=\s*::|\s+as\b)")
    pieces, last = [], 0
    for m in rx.finditer(t):
        start = m.start(1)
        if "#[" in t[max(0, start - 3):start]:
            continue
        if t.startswith("::", m.start()):
            start = m.start()
        pieces.append(text[last:start])
        pieces.append("crate::__ext::" + m.group(1))
        last = m.end(1)
    pieces.append(text[last:])
    return "".join(pieces)


def apply(roots, lib_rel, crates, extra_stub=""):
    files = []
    for r in roots:
        files += rs_files(r)
    found = collect(files, crates)
    stub = build_stub(found)
    for r in roots:
        for p in rs_files(r):
            t = read(p)
            t2 = strip_ext_attrs(rewrite(t, crates), crates)
            if t2 != t:
                write(p, t2)
        lp = os.path.join(r, lib_rel)
        write(lp, read(lp) + "\n" + stub + extra_stub)
    return stub


if __name__ == "__main__":
    name = sys.argv[1]
    t = TARGETS[name]
    root = os.path.join(W, name, "oracle-A")
    print(build_stub(collect(rs_files(root), t["ext"])))
