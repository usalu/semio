"""🗂️ Index of the pub fns of the carved infinite board crates: name -> [(crate, file, line, signature)]."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
INF = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
GEN = os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated"
FN = re.compile(r"^\s*pub(?:\((?:crate|super|in [^)]*)\))?\s+(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)")
def crates():
    out = {}
    for dp, dn, fn in os.walk(INF):
        dn[:] = [d for d in dn if d not in ("node_modules", "dist", "target")]
        if "Cargo.toml" in fn and "/🎲️board/" in dp + "/":
            t = open(dp + "/Cargo.toml", encoding="utf8").read()
            m = re.search(r'^name = "([^"]+)"', t, re.M)
            p = re.search(r'^path = "([^"]+)"', t, re.M)
            if m and p:
                out[m.group(1)] = os.path.normpath(os.path.join(dp, p.group(1)))
    return out
def build():
    idx = {}
    for pkg, lib in sorted(crates().items()):
        lines = open(lib, encoding="utf8").read().split("\n")
        for i, l in enumerate(lines, 1):
            m = FN.match(l)
            if m:
                sig = l.strip()
                j = i
                while "{" not in sig and ";" not in sig and j < len(lines):
                    sig += " " + lines[j].strip(); j += 1
                idx.setdefault(m.group(1), []).append((pkg, lib.replace(ROOT + "/", ""), i, sig.split("{")[0].strip()[:300], bool(re.match(r"^\s*pub\s+fn|^\s*pub\s+const", l))))
    return idx
if __name__ == "__main__":
    idx = build()
    json.dump(idx, open(GEN + "/rii-infinite-fn-index.json", "w"), ensure_ascii=False, indent=0)
    print(len(idx), "names")
    for n in sys.argv[1:]:
        for e in idx.get(n, []):
            print(n, "->", e[0].replace("semio-framework-os-infinite-", ""), e[2], e[3])
