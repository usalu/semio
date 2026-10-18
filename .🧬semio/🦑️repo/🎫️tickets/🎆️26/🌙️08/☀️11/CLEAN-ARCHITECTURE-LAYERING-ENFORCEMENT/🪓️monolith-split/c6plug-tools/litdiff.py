"""🔬️ C5G anchor differential: every double-quoted (or backtick-free single-line) string literal of >= 8 chars in the reader files, with JS escapes decoded, is looked up in the snapshot of each cut root (orig) and in the new module tree of the same root; a literal found in orig but not in the tree is a regression of a source-text anchor. usage: litdiff.py <ts file>..."""
import re, sys, os, json
sys.path.insert(0, ".")
import modtree as M
R = M.REPO + "/"
H = R + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
O = os.path.dirname(os.path.abspath(__file__)) + "/orig/"
roots = {"he": "⚡️effects", "hg": "🎭️guest-runtime", "hr": "🧭️routers", "hs": "🧵️shard"}
orig = {k: open(O + k + ".rs", encoding="utf-8").read() for k in roots}
tree = {k: M.tree(H + d + "/🦀️.rs") for k, d in roots.items()}
# only the own children for hs (pre-existing mounts excluded): original text = orig; tree text = root + own children
STR = re.compile(r'"((?:[^"\\\n]|\\.){8,})"')
def dec(s):
    try: return json.loads('"' + s + '"')
    except Exception: return s
tot = 0
for f in sys.argv[1:]:
    t = open(f if f.startswith("/") else R + f, encoding="utf-8").read()
    seen = set()
    for m in STR.finditer(t):
        lit = dec(m.group(1))
        if lit in seen: continue
        seen.add(lit)
        for k in roots:
            if lit in orig[k] and lit not in tree[k]:
                tot += 1
                print("REGRESSION", k, f.split("/")[-3], repr(lit[:100]))
print("regressions", tot)
