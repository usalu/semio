"""🪄 tc2: rewrite `use super::*` / `super::x` of a test file that leaves its parent module for a companion crate.
unglob(mod, mods, parent_ident, tree_names) -> (new_text, gaps)
 * `use super::*;` becomes the parent module's `use` items (heads `crate::`/`self::`/`super::` made absolute against the parent crate) plus a glob of the parent module's public items;
 * gaps: names the file uses that the parent module defines with a visibility narrower than `pub`, or modules on the path that are not `pub` (widening list)."""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/ttc")
from ttcstub import mask

TRAITS = set(open("/Users/ueli/.cache/tc2/traits.txt", encoding="utf-8").read().split()) | {"Read", "Write", "BufRead", "Seek", "FromStr", "Display", "Debug", "Hash", "Hasher", "Iterator", "IntoIterator", "Extend", "FromIterator", "Deref", "DerefMut", "Borrow", "BorrowMut", "AsRef", "AsMut", "Serialize", "Deserialize", "DeserializeOwned", "Serializer", "Deserializer", "Future", "Stream", "Wake"} if os.path.exists("/Users/ueli/.cache/tc2/traits.txt") else set()
ITEM_RX = re.compile(r"^(?P<attrs>(?:\s*#\[[^\]]*\]\s*)*)\s*(?P<vis>pub(?:\s*\([^)]*\))?\s+)?(?P<rest>.*)$", re.S)
KIND_RX = re.compile(r"^(?:default\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+\"[^\"]*\"\s+)?(?P<kind>fn|struct|enum|union|trait|type|const|static|mod|macro_rules!)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)", re.S)


def top_level_items(raw, masked, lo, hi):
    """Yield (start, end, raw_text, masked_text) of the top-level items of region [lo, hi) (depth 0)."""
    depth = 0
    start = lo
    i = lo
    out = []
    while i < hi:
        ch = masked[i]
        if ch in "{([":
            depth += 1
        elif ch in "})]":
            depth -= 1
            if depth == 0 and ch == "}":
                nxt = re.match(r"\s*;", masked[i + 1:hi])
                if nxt and re.match(r"\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\b", masked[start:i + 1]):
                    i += 1
                    continue
                out.append((start, i + 1))
                start = i + 1
        elif ch == ";" and depth == 0:
            out.append((start, i + 1))
            start = i + 1
        i += 1
    res = []
    for a, b in out:
        seg = masked[a:b]
        off = len(seg) - len(seg.lstrip())
        res.append((a + off, b, raw[a + off:b], masked[a + off:b]))
    return res


def classify(raw_item, masked_item):
    m = ITEM_RX.match(masked_item)
    if not m:
        return None
    rest = m.group("rest").lstrip()
    vis = (m.group("vis") or "").strip()
    if rest.startswith("use ") or rest.startswith("use\t") or rest.startswith("use\n"):
        return ("use", None, vis, raw_item)
    if re.match(r"(?:unsafe\s+)?impl\b", rest) or rest.startswith("extern crate") or rest.startswith("#!["):
        return ("impl", None, vis, raw_item)
    k = KIND_RX.match(rest)
    if k:
        return (k.group("kind"), k.group("name"), vis, raw_item)
    mm = re.match(r"macro_rules!\s*([A-Za-z_]\w*)", rest)
    if mm:
        return ("macro", mm.group(1), vis, raw_item)
    return None


def use_bindings(use_text):
    """Names bound by a `use` item (approximate): last segments / aliases of the expanded tree; globs reported as '*'."""
    from ttcstub import expand_use
    body = re.sub(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+", "", use_text.strip()).rstrip(";").strip()
    names = []
    for kind, segs in expand_use(body):
        if kind == "glob":
            names.append("*")
        elif kind == "mod":
            names.append(segs[-1])
        else:
            names.append(segs[-1])
    for m in re.finditer(r"\bas\s+([A-Za-z_]\w*)", body):
        names.append(m.group(1))
    return names


def module_scope(mod, mods, raw, masked):
    """Region (lo, hi) of the module that holds `mod`'s declaration and that module's crate path (list of names)."""
    inner = [x for x in mods if x.inline and x.parent_file == mod.parent_file and x.span[0] < mod.span[0] and mod.span[1] <= x.span[1]]
    if inner:
        x = min(inner, key=lambda y: y.span[1] - y.span[0])
        mm = re.compile(r"mod\s+" + re.escape(x.name) + r"\s*\{").search(masked, x.span[0])
        return mm.end(), x.span[1] - 1, x.chain
    file_mods = [x for x in mods if x.file == mod.parent_file and not x.inline]
    chain = file_mods[0].chain if file_mods else []
    return 0, len(raw), chain


def absolutize_use(use_text, parent_ident, chain):
    def head(m):
        h = m.group(1)
        rest_chain = list(chain)
        if h == "crate":
            base = [parent_ident]
        elif h == "self":
            base = [parent_ident] + rest_chain
        else:
            base = [parent_ident] + rest_chain[:-1]
        return " ".join([]) + "::".join(base) + "::"
    t = re.sub(r"(?<![A-Za-z0-9_:])(crate|self)::", head, use_text)
    sup = re.compile(r"(?<![A-Za-z0-9_:])((?:super::)+)")

    def sup_head(m):
        n = m.group(1).count("super::")
        base = [parent_ident] + list(chain)[:max(0, len(chain) - n)]
        return "::".join(base) + "::"
    return sup.sub(sup_head, t)


def used_ref(kind, name, fm):
    """Does the masked file text reference the item `name` of `kind` (not a field, method, local binding or path tail)?"""
    n = re.escape(name)
    if kind == "fn":
        if re.search(r"(?<![A-Za-z0-9_.:])" + n + r"\s*(?:::\s*<[^>]*>)?\s*\(", fm):
            return not re.search(r"\blet\s+(?:mut\s+)?" + n + r"\b", fm) or bool(re.search(r"(?<![A-Za-z0-9_.:])" + n + r"\s*\(", fm))
        return bool(re.search(r"[(,]\s*" + n + r"\s*[,)]", fm)) and not re.search(r"\blet\s+(?:mut\s+)?" + n + r"\b|\b" + n + r"\s*:\s|\|[^|]*\b" + n + r"\b", fm)
    if kind == "mod":
        return bool(re.search(r"(?<![A-Za-z0-9_.:])" + n + r"\s*::", fm))
    if kind in ("macro",):
        return bool(re.search(r"(?<![A-Za-z0-9_.:])" + n + r"\s*!", fm))
    if kind in ("const", "static"):
        return bool(re.search(r"(?<![A-Za-z0-9_.])" + n + r"(?![A-Za-z0-9_])", fm)) and not re.search(r"\blet\s+(?:mut\s+)?" + n + r"\b", fm)
    return bool(re.search(r"(?<![A-Za-z0-9_.])" + n + r"(?![A-Za-z0-9_])", fm))


def target_scope(chain, mods, lib):
    if not chain:
        raw = open(lib, encoding="utf-8").read()
        return lib, raw, mask(raw), 0, len(raw)
    cand = [x for x in mods if x.chain == list(chain) and not x.test_gated]
    if not cand:
        return None
    x = cand[0]
    raw = open(x.file, encoding="utf-8").read()
    masked = mask(raw)
    if x.inline:
        mm = re.compile(r"mod\s+" + re.escape(x.name) + r"\s*\{").search(masked, x.span[0])
        return x.file, raw, masked, mm.end(), x.span[1] - 1
    return x.file, raw, masked, 0, len(raw)


def unglob(test_mod, mods, parent_ident, lib, chain, extra_text=""):
    """Imports that replace a glob of the ancestor module `chain` in the moved file `test_mod`; returns (lines, gaps)."""
    tgt = target_scope(chain, mods, lib)
    if tgt is None:
        return None, [("unknown-module", test_mod.file, ".".join(chain), "mod", "", "")]
    pf, raw, masked, lo, hi = tgt
    items = []
    for a, b, rt, mt in top_level_items(raw, masked, lo, hi):
        c = classify(rt, mt)
        if c:
            items.append(c)
    fraw = open(test_mod.file, encoding="utf-8").read()
    fm = mask(fraw)
    fm_own = fm
    gaps = []
    defined_here = set()
    for a, b, rt, mt in top_level_items(fraw, fm, 0, len(fraw)):
        c = classify(rt, mt)
        if c and c[1]:
            defined_here.add(c[1])
    fm = fm + "\n" + mask(extra_text) if extra_text else fm
    used = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", fm))
    uses, pubdefs, privdefs = [], [], []
    for kind, name, vis, rt in items:
        if kind == "use":
            uses.append(rt)
        elif name:
            if name in defined_here:
                continue
            if vis == "pub":
                pubdefs.append((kind, name))
            else:
                privdefs.append((kind, name, vis))
    for kind, name, vis in privdefs:
        if name in used and kind != "impl" and kind != "mod" and used_ref(kind, name, fm):
            gaps.append(("private-item", pf, ".".join(chain), kind, name, vis or "private"))
    own = set()
    for a, b, rt, mt in top_level_items(fraw, fm, 0, len(fraw)):
        c = classify(rt, mt)
        if c and c[0] == "use":
            own |= {x for x in use_bindings(rt) if x not in ("*", "_")}
    out_lines = []
    local = {n for k, n, v, r in items if n}
    pub_bound = {n for k, n in pubdefs}
    for k, n, v, r in items:
        if k == "use" and v == "pub":
            pub_bound |= {x for x in use_bindings(r) if x != "*"}
    base = "::".join([parent_ident] + list(chain))
    for u in uses:
        if re.match(r"\s*(?:#\[[^\]]*\]\s*)*pub\s+use\b", u):
            continue
        bnames = [x for x in use_bindings(u) if x != "_"]
        if bnames and "*" not in bnames and all(x in own for x in bnames):
            continue
        head = re.search(r"use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)", u)
        allnames = use_bindings(u)
        if (head and head.group(1) not in ("std", "core", "alloc") and (head.group(1).startswith("semio_") or head.group(1) in ("crate", "self", "super") or head.group(1) in local or head.group(1) in ("protocol", "store", "dsl"))
                and "*" not in allnames and "_" not in allnames and bnames
                and not any(x in used for x in bnames) and not any(x in TRAITS for x in bnames)):
            continue
        clash = [x for x in bnames if x in own]
        if clash and "{" in u:
            for x in clash:
                u = re.sub(r"\b" + re.escape(x) + r"\s*,\s*", "", u, count=1) if re.search(r"\b" + re.escape(x) + r"\s*,", u) else re.sub(r",\s*" + re.escape(x) + r"\b", "", u, count=1)
            bnames = [x for x in use_bindings(u) if x != "_"]
        if not re.match(r"\s*(?:#\[[^\]]*\]\s*)*pub\b", u) and bnames and "*" not in bnames and all(x in pub_bound for x in bnames):
            continue
        u2 = absolutize_use(u.strip(), parent_ident, chain)
        u2 = re.sub(r"((?:#\[[^\]]*\]\s*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+)(?:::)?([A-Za-z_][A-Za-z0-9_]*)(?=::|\s*;|\s+as\b)", lambda m: m.group(1) + (base + "::" + m.group(2) if m.group(2) in local else m.group(2)), u2, count=1)
        hm = re.search(r"use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)", u.strip())
        if hm and hm.group(1) in local:
            private = [(kind, name, vis) for kind, name, vis, rt in items if name == hm.group(1) and vis != "pub" and name not in defined_here]
            if private:
                bound = [n for n in use_bindings(u2) if n != "_"]
                if "*" not in bound and not any(n in used for n in bound):
                    continue
                for kind, name, vis in private:
                    gaps.append(("private-child", pf, ".".join(chain), kind, name, vis or "private"))
        if hm and hm.group(1) in local and "*" in use_bindings(u2):
            child = [x for x in mods if x.chain == list(chain) + [hm.group(1)] and not x.test_gated]
            if child:
                for kind, name, vis, rt in module_items(child[0], mods):
                    if kind not in ("use", "impl", "mod") and name and name not in defined_here and vis != "pub" and name in used and used_ref(kind, name, fm):
                        gaps.append(("private-item", pf, ".".join(list(chain) + [hm.group(1)]), kind, name, vis or "private"))
        u2 = re.sub(r"^((?:#\[[^\]]*\]\s*)*)pub(?:\s*\([^)]*\))?\s+use\b", r"\1use", u2)
        names = use_bindings(u2)
        if any(n in defined_here for n in names if n != "*"):
            if "{" not in u2:
                continue
        out_lines.append("#[allow(unused_imports)] " + " ".join(u2.split()) if not u2.lstrip().startswith("#[") else u2.strip().replace("\n", " ").replace("#[", "#[allow(unused_imports)] #[", 1))
    for depth in range(1, len(chain) + 1):
        pm = [x for x in mods if x.chain == list(chain[:depth]) and not x.test_gated]
        if pm and pm[0].vis != "pub":
            gaps.append(("private-module", pf, ".".join(chain[:depth]), "mod", chain[depth - 1], pm[0].vis or "private"))
    path = "::".join([parent_ident] + list(chain))
    if pubdefs:
        out_lines.append(f"#[allow(unused_imports)] use {path}::*;")
    return out_lines, gaps


def module_items(mod, mods):
    """Top-level classified items of a (non-test) module, wherever its body lives."""
    raw = open(mod.file, encoding="utf-8").read()
    masked = mask(raw)
    if mod.inline:
        mm = re.compile(r"mod\s+" + re.escape(mod.name) + r"\s*\{").search(masked, mod.span[0])
        lo, hi = mm.end(), mod.span[1] - 1
    else:
        lo, hi = 0, len(raw)
    out = []
    for a, b, rt, mt in top_level_items(raw, masked, lo, hi):
        c = classify(rt, mt)
        if c:
            out.append(c)
    return out


def glob_gaps(text, mods, test_file, context=""):
    """Private items of the modules a moved file globs through `crate::a::b::*`, restricted to names the file (the statement `text` inside `context`, descendants included) uses."""
    masked = mask(text)
    ctx = context if context else text
    cmask = mask(ctx)
    used = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", cmask))
    defined_here = set()
    for a, b, rt, mt in top_level_items(ctx, cmask, 0, len(ctx)):
        c = classify(rt, mt)
        if c and c[1]:
            defined_here.add(c[1])
    gaps = []
    for m in re.finditer(r"use\s+crate::((?:[A-Za-z0-9_]+::)*)\*\s*;", masked):
        chain = [x for x in m.group(1).split("::") if x]
        target = [x for x in mods if x.chain == chain and not x.test_gated]
        if not chain:
            continue
        if not target:
            gaps.append(("unknown-module", test_file, ".".join(chain), "mod", "", ""))
            continue
        for depth in range(1, len(chain) + 1):
            pm = [x for x in mods if x.chain == chain[:depth] and not x.test_gated]
            if pm and pm[0].vis != "pub":
                gaps.append(("private-module", test_file, ".".join(chain[:depth]), "mod", chain[depth - 1], pm[0].vis or "private"))
        for kind, name, vis, rt in module_items(target[0], mods):
            if kind in ("use", "impl") or not name or name in defined_here:
                continue
            if vis != "pub" and name in used:
                gaps.append(("private-item", test_file, ".".join(chain), kind, name, vis or "private"))
    return gaps


def widen_edits(mods, lib, gaps):
    """Minimal `pub` edits for gap items: returns {file: [(offset_start, offset_end, replacement)]} (descending apply order)."""
    edits = {}
    done = set()
    for g in gaps:
        kind_g, pf, chain_s, kind, name = g[0], g[1], g[2], g[3], g[4]
        chain = [x for x in chain_s.split(".") if x] if chain_s else []
        key = (pf, chain_s, kind, name)
        if key in done:
            continue
        done.add(key)
        tgt = target_scope(chain, mods, lib)
        if tgt is None:
            continue
        f, raw, masked, lo, hi = tgt
        if kind_g == "private-module":
            parent_chain, name_ = chain[:-1], chain[-1]
            tgt = target_scope(parent_chain, mods, lib)
            f, raw, masked, lo, hi = tgt
            name, kind = name_, "mod"
        hit = None
        for a, b, rt, mt in top_level_items(raw, masked, lo, hi):
            c = classify(rt, mt)
            if c and c[0] == kind and c[1] == name:
                hit = (a, b, rt, mt)
                break
        if hit is None:
            edits.setdefault("__missing__", []).append(key)
            continue
        a, b, rt, mt = hit
        m = ITEM_RX.match(mt)
        vs, ve = (m.start("vis"), m.end("vis")) if m.group("vis") else (m.start("rest"), m.start("rest"))
        edits.setdefault(f, []).append((a + vs, a + ve, "pub "))
    return edits
