"""🧰 Shared helpers of the pack carve generator: staging tree, item cutting, crate registry."""
import os, re, sys, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import items, tokenize

SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"
BASE = SP + "/base/"
STAGE = SP + "/stage/"
REPO = "/Users/ueli/Documents/semio/"
FW = "🧰️framework/🔨️modules/"
P = FW + "🎒️pack/"

def read_base(rel):
    return open(BASE + rel, encoding="utf8").read()

def read_stage_or_base(rel):
    p = STAGE + rel
    return open(p if os.path.exists(p) else BASE + rel, encoding="utf8").read()

def write(rel, text):
    p = STAGE + rel
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf8").write(text)

def delete(rel):
    """Marks a base file as removed in the stage (an empty `.deleted` marker)."""
    p = STAGE + rel + ".deleted"
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w").write("")

def ids(text):
    return set(t[1] for t in tokenize(text) if t[0] == "id")

def bare_ids(text):
    """Identifiers that can denote an imported name: not path continuations (`a::NAME`) and not member accesses (`.name`)."""
    toks = [t for t in tokenize(text) if t[0] not in ("ws", "comment")]
    out = set()
    for k, (kd, tx, a, b) in enumerate(toks):
        if kd != "id": continue
        if k >= 1 and toks[k - 1][1] == ".": continue
        if k + 1 < len(toks) and toks[k + 1][1] == ":" and not (k + 2 < len(toks) and toks[k + 2][1] == ":") and not (k >= 1 and toks[k - 1][1] == ":"):
            continue
        if k >= 2 and toks[k - 1][1] == ":" and toks[k - 2][1] == ":":
            before = toks[k - 3] if k >= 3 else ("punct", "", 0, 0)
            if before[0] == "id" or before[1] in (">", ")", "]"): continue
        out.add(tx)
    return out

def top_items(text):
    return items(text)

def cut_items(text, names, kinds=("fn", "struct", "enum", "const", "static", "type", "trait", "macro_rules", "union")):
    """Removes top-level items by name; returns (remaining_text, [removed item texts in file order]). Item text includes its leading docs."""
    its = items(text)
    byname = {}
    for it in its:
        if it["kind"] in kinds and it["name"] in names:
            assert it["name"] not in byname, ("duplicate", it["name"])
            byname[it["name"]] = it
    missing = set(names) - set(byname)
    assert not missing, ("missing items", missing)
    spans = []
    for it in byname.values():
        a, b = it["start"], it["end"]
        while text[a:b].lstrip().startswith("//#"):
            a = text.index("\n", text.index("//#", a)) + 1
        spans.append((a, b, it["name"]))
    spans.sort()
    out = []; last = 0; removed = []
    for a, b, n in spans:
        t = text[a:b]
        assert "//#" not in t, ("region marker inside cut item", n)
        out.append(text[last:a]); last = b; removed.append(t)
    out.append(text[last:])
    rest = "".join(out)
    rest = re.sub(r"\n{3,}", "\n\n", rest)
    return rest, removed

def lines_between(text, a_marker, b_marker):
    """Returns (before, middle, after) where middle is from the line containing a_marker through the line containing b_marker (inclusive)."""
    L = text.split("\n")
    ia = [i for i, l in enumerate(L) if l.strip() == a_marker]
    ib = [i for i, l in enumerate(L) if l.strip() == b_marker]
    assert len(ia) == 1 and len(ib) == 1, (a_marker, b_marker, ia, ib)
    return "\n".join(L[:ia[0]]), "\n".join(L[ia[0]:ib[0] + 1]), "\n".join(L[ib[0] + 1:])

import usetree

def split_header(text):
    """Splits a file into (docs_prefix, use_items, rest). `use_items` are (start,end,tree_text,is_pub,attrs) of top-level `use` items found before the first non-use/non-doc item that carries code; all top-level `use` items are returned wherever they are, rest has them removed only if remove=True."""
    raise NotImplementedError

def top_uses(text):
    out=[]
    for it in items(text):
        if it["kind"]=="use":
            out.append(it)
    return out

def use_tree_of(it):
    t=it["text"]
    toks=tokenize(t)
    for k,(kd,tx,a,b) in enumerate(toks):
        if kd=="id" and tx=="use":
            body=t[b:].rstrip()
            if body.endswith(";"): body=body[:-1]
            return re.sub(r"//[^\n]*","",body)
    raise ValueError("no use keyword")

def leaf_names(tree):
    return [usetree.local_name(l) for l in usetree.parse(tree)]

def build_uses(body, leaves, pub=False):
    """Returns `use` lines for the leaves whose local name occurs as an identifier in body (globs always kept)."""
    used=bare_ids(body)
    def keepit(l):
        n = usetree.local_name(l)
        if n is None or n in used: return True
        return trait_method_used(n, body)
    keep=[l for l in leaves if keepit(l)]
    # drop exact duplicates
    seen=set();res=[]
    for l in keep:
        k=(tuple(l[0]),l[1],l[2])
        if k in seen: continue
        seen.add(k);res.append(l)
    return usetree.emit(res)

def split_doc_uses_body(text):
    """Splits a file into (doc, uses, body): leading `//!` doc block, the contiguous top-level `use` items after it (with their text), and the remaining text."""
    L = text.split("\n")
    i = 0
    while i < len(L) and (L[i].startswith("//!") or L[i].strip() == ""): i += 1
    doc = "\n".join(L[:i]).rstrip("\n")
    rest = "\n".join(L[i:])
    its = items(rest)
    uses = []; end = 0
    for it in its:
        if it["kind"] == "use" and not it["vis"] and "#[" not in it["text"].split("use ")[0]:
            uses.append(it); end = it["end"]
        else:
            break
    body = rest[end:].lstrip("\n")
    return doc, uses, body

def leaves_of_uses(uses):
    out = []
    for it in uses:
        out += usetree.parse(use_tree_of(it))
    return out

def map_leaves(leaves, rule):
    out = []
    for path, alias, glob in leaves:
        if path and path[0] == "crate" and len(path) > 1:
            r = rule(path[1:])
            if r is not None:
                consumed, repl = r
                newp = repl.split("::") + path[1 + consumed:]
                out.append((newp, alias, glob)); continue
            raise KeyError(("unmapped use leaf", path))
        out.append((path, alias, glob))
    return out

def provided_names(root_text):
    """Names a module root makes visible to `use super::*` children: its own definitions and its use-imports."""
    out = set()
    for it in items(root_text):
        if it["kind"] == "use":
            for l in usetree.parse(use_tree_of(it)):
                n = usetree.local_name(l)
                if n: out.add(n)
        elif it["kind"] in ("fn","struct","enum","const","static","type","trait","mod","macro_rules","union") and it["name"]:
            out.add(it["name"])
    return out

def augment_uses(test_text, available, provided):
    """`available` = {name: path list} of names the old root made visible. Adds explicit `use` lines to a `use super::*` test for names it uses that the new root no longer provides."""
    I = ids(test_text)
    own = set()
    for it in items(test_text):
        if it["kind"] == "use":
            for l in usetree.parse(use_tree_of(it)):
                n = usetree.local_name(l)
                if n: own.add(n)
    own |= set(it["name"] for it in items(test_text) if it["name"])
    def wanted(n):
        if n in I: return True
        return trait_method_used(n, test_text)
    need = sorted(n for n in available if wanted(n) and n not in provided and n not in own)
    if not need:
        return test_text, []
    leaves = [available[n] if isinstance(available[n], tuple) else (available[n], None, False) for n in need]
    lines = usetree.emit(leaves)
    m = re.search(r"^use super::\*;[ \t]*$", test_text, re.M)
    assert m, "test has no `use super::*;` line"
    new = test_text[:m.end()] + "\n" + "\n".join(lines) + test_text[m.end():]
    return new, need


PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')

INCLUDE_RX = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')

def exists_stage_or_base(rel):
    return os.path.exists(STAGE + rel) or os.path.exists(BASE + rel)

def crawl(root_rel):
    """Relative paths of every .rs file reachable from a lib root through #[path] mounts, resolved against stage-over-base."""
    seen = []; stack = [root_rel]
    while stack:
        rel = stack.pop()
        if rel in seen: continue
        seen.append(rel)
        text = read_stage_or_base(rel)
        for m in list(PATH_RX.finditer(text)) + list(INCLUDE_RX.finditer(text)):
            p = os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1)))
            if p.endswith(".rs") and exists_stage_or_base(p): stack.append(p)
    return seen

def finalize_crate(key_libroot, rewrite):
    """Rewrites every crate file not yet staged (rewrite(text)->text) and stages it when changed."""
    done = []
    for rel in crawl(key_libroot):
        if os.path.exists(STAGE + rel): continue
        t = open(BASE + rel, encoding="utf8").read()
        n = rewrite(t)
        if n != t:
            write(rel, n); done.append(rel)
    return done

def cut_uses(text, prefixes):
    """Removes top-level `use` items whose tree text starts with one of `prefixes`; returns (rest, removed texts)."""
    its = items(text)
    spans = []
    for it in its:
        if it["kind"] == "use" and any((it["name"] or "").startswith(p) for p in prefixes):
            a, b = it["start"], it["end"]
            while text[a:b].lstrip().startswith("//#"):
                a = text.index("\n", text.index("//#", a)) + 1
            spans.append((a, b))
    out = []; last = 0; removed = []
    for a, b in sorted(spans):
        out.append(text[last:a]); last = b; removed.append(text[a:b])
    out.append(text[last:])
    return re.sub(r"\n{3,}", "\n\n", "".join(out)), removed


STD_TRAITS = {"Write", "Read", "Seek", "BufRead", "Hash", "Hasher", "FromStr", "Borrow", "BorrowMut", "Deref", "DerefMut", "AsRef", "AsMut", "Display", "Debug", "Future", "Wake", "Ord", "PartialOrd", "BuildHasher", "IntoIterator", "FromIterator", "Extend", "Iterator", "ToString", "ToOwned", "Any", "Error"}
def _load_traits():
    p = os.path.join(os.path.dirname(os.path.abspath(__file__)), "traits.txt")
    return {l.strip() for l in open(p, encoding="utf8") if l.strip()} | STD_TRAITS
TRAITS = _load_traits()

_AWAIT = r"\.\s*(?:%s)\s*\((?:[^()]|\([^()]*\))*\)\s*\.await"
TRAIT_RX = {
    "PackSource": re.compile(_AWAIT % "len|is_empty|read_at|read_exact_at"),
    "PackSink": re.compile(_AWAIT % "write_all|position|flush"),
    "CompressionCodec": re.compile(r"\.\s*(?:compress|decompress)\s*\("),
}
def trait_method_used(name, body):
    rx = TRAIT_RX.get(name)
    return bool(rx and rx.search(body))


def build_uses_split(lib_scan, test_scan, leaves):
    """`use` lines the library needs, then `#[cfg(test)]` lines for the extra imports only the tests (through `use super::*`) need."""
    lib_lines = build_uses(lib_scan, leaves)
    all_lines = build_uses(lib_scan + "\n" + test_scan, leaves)
    extra = [l for l in all_lines if l not in lib_lines]
    # re-derive per-leaf so a partially needed group is split exactly
    lib_leaves = set()
    for ln in lib_lines:
        for lf in usetree.parse(ln[4:-1]): lib_leaves.add((tuple(lf[0]), lf[1], lf[2]))
    kept = []
    seen = set()
    for lf in leaves:
        n = usetree.local_name(lf)
        key = (tuple(lf[0]), lf[1], lf[2])
        if key in lib_leaves or key in seen: continue
        if n is None or n in bare_ids(test_scan) or trait_method_used(n, test_scan):
            kept.append(lf); seen.add(key)
    test_lines = ["#[cfg(test)]\n" + x for x in usetree.emit(kept)]
    return lib_lines, test_lines
