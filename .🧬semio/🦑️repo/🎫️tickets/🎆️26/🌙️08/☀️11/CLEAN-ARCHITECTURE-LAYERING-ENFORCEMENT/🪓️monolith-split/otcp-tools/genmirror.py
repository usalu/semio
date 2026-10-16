"""Companion generator: finds the #[cfg(test)] test mounts of a production crate (from the PRE-EDIT snapshot), computes the production-side
mount removals and renders the companion lib root with inline mirror modules.
Library: used by the apply_<crate>.py drivers."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import modtree, mirror
from defs import scan
from splitter import tokenize
from stubgen import mask

ROOT = "/Users/ueli/Documents/semio"


def test_mounts(lib):
    """[(host_mp tuple, name, host_file, decl, test_file)] for every out-of-line mod carrying a cfg(test) attribute (outermost only)."""
    out = []
    nodes = modtree.tree(lib)
    test_prefixes = []
    for mp, f, d, parent in nodes:
        if any(mp[:len(p)] == p for p in test_prefixes):
            continue
        if d["inline"]:
            continue
        if any("cfg(test)" in a.replace(" ", "") for a in d["attrs"]):
            test_prefixes.append(mp)
            out.append((mp[:-1], mp[-1], parent, d, f))
    return out


def subtree_files(lib, mp):
    files = []
    for m2, f, d, parent in modtree.tree(lib):
        if m2[:len(mp)] == mp and f and f != "MISSING-PATH" and os.path.exists(f):
            files.append(f)
    return files


def decl_slice(text, d):
    """exact source slice [a, b) of a mod declaration with its attributes (decl dict from modtree.mounts)."""
    m = mask(text)
    # the declaration is on `line`; walk back over attributes, forward to ';'
    lines = text.split("\n")
    offs = [0]
    for ln in lines:
        offs.append(offs[-1] + len(ln) + 1)
    mod_off = offs[min(d["line"] - 1, len(offs) - 1)]
    rx = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+" + re.escape(d["name"]) + r"\s*;")
    mm = rx.search(m, mod_off) or rx.search(m)
    assert mm, ("mod decl not found", d["name"])
    start = mm.start()
    # attributes before: walk back
    spans = [(x.start(), x.end()) for x in modtree.ATTR.finditer(m)]
    cur = start
    while True:
        prev = [sp for sp in spans if sp[1] <= cur and not text[sp[1]:cur].strip()]
        if not prev:
            break
        cur = max(prev, key=lambda x: x[1])[0]
    return cur, mm.end()


def removal(text, d):
    a, b = decl_slice(text, d)
    end = b
    if text[end:end + 1] == "\n":
        end += 1
    start = a
    while start > 0 and text[start - 1] in " \t":
        start -= 1
    pre = text[:start]
    post = text[end:]
    m = re.search(r"(//#region ([^\n]*)\n)$", pre)
    if m and post.startswith("//#endregion " + m.group(2).strip() + "\n"):
        pre = pre[:m.start()]
        post = post[len("//#endregion " + m.group(2).strip() + "\n"):]
    elif m and post.startswith("//#endregion " + m.group(2).strip()) and post[len("//#endregion " + m.group(2).strip()):].strip() == "":
        pre = pre[:m.start()]
        post = ""
    if pre.endswith("\n\n") and post.startswith("\n"):
        post = post[1:]
    if post == "":
        pre = pre.rstrip("\n") + "\n"
    return pre + post


def used_tokens(files):
    allt = set()
    for f in files:
        first, a = tokenize(open(f, encoding="utf-8").read())
        allt |= a
    return allt


def translate_use(full, name, host_mp, pkg, real, child_mods=frozenset()):
    """map a host `use` path to a path valid from the companion; real(mp)->tuple of segments of the production crate's reachable path."""
    head = full[0]
    if head in child_mods and head not in ("crate", "self", "super"):
        return real(host_mp + (head,) + tuple(full[1:-1])) + (full[-1],)
    if head == "crate":
        return real(full[1:-1]) + (full[-1],) if len(full) > 1 else real(()) 
    if head == "self":
        return real(host_mp + tuple(full[1:-1])) + (full[-1],)
    if head == "super":
        k = 0
        while k < len(full) and full[k] == "super":
            k += 1
        base = host_mp[:max(0, len(host_mp) - k)]
        return real(base + tuple(full[k:-1])) + (full[-1],)
    return full


STD_TRAITS = {"Read", "Write", "BufRead", "Seek", "FromStr", "Hash", "Hasher", "Display", "Debug", "Deref", "DerefMut", "Borrow", "BorrowMut", "AsRef", "AsMut", "From", "Into", "TryFrom", "TryInto", "Iterator", "IntoIterator", "Extend", "FromIterator", "ToString", "Future", "Stream", "Error", "Ord", "PartialOrd", "PartialEq", "Eq", "Add", "Sub", "Mul", "Div", "Index", "IndexMut", "ToOwned", "Any", "Send", "Sync", "Fn", "FnMut", "FnOnce", "Drop", "Wake"}


def host_use_lines(host_file, host_mp, pkg, real, used, drop=(), ctx="", traits=frozenset(), keep_ext=True):
    child_mods = {d["name"] for d in modtree.mounts(host_file) if d["parent_inline"] == ([x for x in ctx.split("::") if x] if ctx else [])}
    """pruned, translated copy of a host module's private imports. Kept: names the tests mention, trait names (methods need the trait in scope without naming it),
    and every external-crate import (they are always resolvable from the companion)."""
    t = open(host_file, encoding="utf-8").read()
    s = scan(t)
    keep = []
    globs = []
    for imp in s["imports"]:
        if imp["vis"] or imp["ctx"] != ctx:
            continue
        for full, nm in mirror.flatten(imp["text"]):
            internal = full[0] in ("crate", "self", "super") or full[0] in child_mods
            if full and full[-1] == "*":
                tr = translate_use(full, nm, host_mp, pkg, real, child_mods)
                globs.append(tr)
                continue
            wanted = nm in used or nm in traits or nm in STD_TRAITS or (keep_ext and not internal and full[0] not in ("std", "core", "alloc"))
            if not wanted or nm in drop:
                continue
            tr = translate_use(full, nm, host_mp, pkg, real, child_mods)
            keep.append((tr, nm))
    lines = []
    for g in globs:
        lines.append("use " + "::".join(g[:-1]) + "::*;")
    seen = set()
    items = []
    for tr, nm in keep:
        k = (tr, nm)
        if k not in seen:
            seen.add(k)
            items.append((tr, nm))
    lines += mirror.group(items)
    return lines


class Node:
    def __init__(self, name):
        self.name = name
        self.children = collections.OrderedDict()
        self.uses = []
        self.mounts = []
        self.dir = None
        self.glob = None
        self.pre = []


def render(node, parent_dir, indent, croot_dir, unique_dir):
    pad = "    " * indent
    out = []
    single = len(node.mounts) == 1 and not node.children and node.mounts[0][1].endswith("/🦀️.rs") and node.mounts[0][0] == "tests"
    if node.dir is not None:
        mdir = node.dir
    elif single:
        mdir = os.path.dirname(node.mounts[0][1])
    else:
        mdir = parent_dir
    rel = os.path.relpath(mdir, parent_dir) if mdir != parent_dir else "."
    if indent == 0 or True:
        out.append(f'{pad}#[path = "{rel}"]')
    out.append(f"{pad}mod {node.name} {{")
    for l in node.pre:
        out.append(f"{pad}    {l}")
    for u in node.uses:
        out.append(f"{pad}    {u}")
    blank = bool(node.pre or node.uses)
    for nm, ch in node.children.items():
        if blank:
            out.append("")
        out += render(ch, mdir, indent + 1, croot_dir, unique_dir)
        blank = True
    for (modname, tfile) in node.mounts:
        if blank:
            out.append("")
        if single and node.dir is None:
            out.append(f'{pad}    #[path = "{os.path.basename(tfile)}"]')
        else:
            out.append(f'{pad}    #[path = "{os.path.relpath(tfile, mdir)}"]')
        out.append(f"{pad}    mod {modname};")
        blank = True
    out.append(f"{pad}}}")
    return out
