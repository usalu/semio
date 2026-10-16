"""🔍 Name-keyed privacy crossing scan: every non-`pub` item, method and field declared in the library closure (snapshot or working tree, test-gated mounts excluded) that a test file names.
usage: privscan.py <snapshot-key|cur> <pkg> <testfile-rel>...   (testfile paths are repo-relative)
"""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ttclib import *
import ttcunion as U
from ttcstub import mask

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"
VIS = r"(?P<vis>pub(?:\([^)]*\))?\s+)?"
ITEM_RX = re.compile(r"(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*" + VIS + r"(?:default\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?(?:extern\s+\"[^\"]*\"\s+)?(?P<kind>fn|struct|enum|trait|type|const|static|mod|union)\s+(?P<name>" + IDENT + ")")
FIELD_RX = re.compile(r"(?m)^[ \t]*(?:#\[[^\]]*\]\s*)*" + VIS + r"(?P<name>" + IDENT + r")\s*:\s*[^:]")
STRUCT_RX = re.compile(r"\bstruct\s+(" + IDENT + r")\s*(?:<[^{;]*>)?\s*(?:where[^{;]*)?\{")


def body_end(t, i):
    d = 1
    while i < len(t) and d:
        if t[i] == "{":
            d += 1
        elif t[i] == "}":
            d -= 1
        i += 1
    return i


def strip_cfg_test(t):
    out, i = [], 0
    rx = re.compile(r"#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+\w+\s*(?:;|\{)|use\s[^;]*;|fn\s|static\s|const\s|impl\b)")
    return t


def main():
    key, pkg, tests = sys.argv[1], sys.argv[2], sys.argv[3:]
    mans = Manifests(Source(None))
    mans.build_index()
    src = Source(None if key == "cur" else "snap:" + SCRATCH + "/ttc-snap/" + key)
    man = mans.manifest(pkg, src)
    files, code, missing = U.reach(src, mans.lib_rel(man), man["dir"])
    tfiles = {rel: mask(open(os.path.join(ROOT, rel), encoding="utf-8").read()) for rel in tests}
    ttext = "\n".join(tfiles.values())
    ttok = collections.Counter(re.findall(IDENT, ttext))
    tset = set(tests)
    decl = []
    for rel in sorted(code):
        if rel in tset or "🧪️tests" in rel:
            continue
        t = mask(files[rel].decode("utf-8", "replace"))
        for m in ITEM_RX.finditer(t):
            vis = (m.group("vis") or "").strip()
            if vis == "pub":
                continue
            decl.append((m.group("kind"), m.group("name"), vis or "private", rel))
        for sm in STRUCT_RX.finditer(t):
            e = body_end(t, sm.end())
            body = t[sm.end():e - 1]
            segs, d, st = [], 0, 0
            for k, ch in enumerate(body):
                if ch in "<([{":
                    d += 1
                elif ch in ">)]}":
                    if ch == ">" and k and body[k - 1] == "-":
                        continue
                    d -= 1
                elif ch == "," and d == 0:
                    segs.append(body[st:k])
                    st = k + 1
            segs.append(body[st:])
            for seg in segs:
                fm = re.match(r"^\s*(?:#\[[^\]]*\]\s*)*(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<name>[A-Za-z_]\w*)\s*:", seg)
                if not fm:
                    continue
                vis = (fm.group("vis") or "").strip()
                if vis == "pub":
                    continue
                decl.append(("field " + sm.group(1), fm.group("name"), vis or "private", rel))
    hits = collections.defaultdict(list)
    for kind, name, vis, rel in decl:
        n = ttok.get(name, 0)
        if n:
            hits[(kind, name, vis)].append((n, rel))
    for (kind, name, vis), lst in sorted(hits.items(), key=lambda kv: (kv[0][0], kv[0][1])):
        print(f"{kind:34} {name:42} {vis:11} uses={lst[0][0]:4} {os.path.basename(os.path.dirname(lst[0][1]))}")
    print(f"-- {len(hits)} non-pub declarations named by tests (of {len(decl)} non-pub declarations in {len(code)} closure files)")


main()
