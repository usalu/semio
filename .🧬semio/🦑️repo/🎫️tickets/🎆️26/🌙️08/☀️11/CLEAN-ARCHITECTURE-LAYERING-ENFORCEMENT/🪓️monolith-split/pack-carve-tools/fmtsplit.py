"""✂️ Partition of the retained canonical source region into admission cursors (retained) and catalog cursors (catalog)."""
import re
from lib import *

CAT_NAMES = {
    "RetainedPackChunkEntry", "RetainedPackSymbolSpan", "RetainedPackManifest", "RetainedPackCatalog",
    "RetainedPackCatalogAllocationStep", "RetainedPackCatalogAllocationError", "RetainedPackCatalogProgress", "RetainedPackCatalogEvent",
    "RetainedManifestPhase", "RetainedManifestCursor", "RetainedUtf8Cursor", "RetainedSymbolsPhase", "RetainedSymbolsCursor", "RetainedChunksPhase", "RetainedChunksCursor",
    "RETAINED_PACK_MAXIMUM_SYMBOL_SPANS", "RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS", "RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES", "RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS",
    "RetainedPackSymbolSpans", "RetainedPackSymbolScalars", "RetainedPackChunkEntries", "RetainedPackObservedChunks", "RetainedPackSymbolTable",
    "RetainedPackCatalogAllocationOwner", "RetainedPackCatalogCursor",
}

def strip_generics(s):
    d = 0; out = ""
    for ch in s:
        if ch == "<": d += 1
        elif ch == ">": d -= 1
        elif d == 0: out += ch
    return out

def impl_type(head):
    h = re.sub(r"\bwhere\b.*", "", strip_generics(head))
    ty = re.split(r"\bfor\b", h, maxsplit=1)[-1]
    names = re.findall(r"[A-Za-z_][A-Za-z_0-9]*", ty)
    return names[-1] if names else ""

def region(base):
    before, mid, after = lines_between(base, "//#region 🔖️RetainedCanonicalSource", "//#endregion 🔖️RetainedCanonicalSource")
    inner = "\n".join(mid.split("\n")[1:-1])
    inner, removed = cut_items(inner, {"retained_pack_source_laws"}, kinds=("mod",))
    return before, after, inner

def partition(inner):
    ret, cat = [], []
    for it in items(inner):
        name = it["name"]
        target = impl_type(name) if it["kind"] == "impl" else name
        (cat if target in CAT_NAMES else ret).append(it["text"])
        if it["kind"] == "impl": assert target, it["name"]
    return ret, cat

def split_laws(text, helpers_of=None):
    """Splits the retained-pack-source-laws test file into (retained part, catalog part) by component; shared helpers are copied into both."""
    its = items(text)
    head = [i for i in its if i["kind"] == "use"]
    body = [i for i in its if i["kind"] != "use"]
    catt = {n for n in CAT_NAMES if not n.isupper()} - {"retained_paged_kind"}
    defs = {i["name"]: i for i in body if i["name"] and i["kind"] in ("fn",)}
    def is_test(i): return any("test" in a for a in i["attrs"])
    tests = [i for i in body if i["kind"] == "fn" and is_test(i)]
    cat_tests = [i for i in tests if ids(i["text"]) & catt]
    ret_tests = [i for i in tests if not (ids(i["text"]) & catt)]
    def closure(ts):
        need = set(); stack = list(ts)
        while stack:
            i = stack.pop()
            for x in ids(i["text"]):
                if x in defs and x not in need and not is_test(defs[x]) and defs[x] is not i:
                    need.add(x); stack.append(defs[x])
        return need
    out = []
    for ts in (ret_tests, cat_tests):
        need = closure(ts)
        keep = [i for i in body if (i in ts) or (i["name"] in need and i["kind"] == "fn") or (i["kind"] not in ("fn",) and i["name"] and ids(i["text"]) and False)]
        out.append(keep)
    return head, out, defs
