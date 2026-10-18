"""📖 C6DB1 source-law re-pointer: replaces `include_str!("<rel>/🦀️.rs")` literals that name a split root by a const that concatenates every child and the root (children first, spec order, root last).
usage: reread.py <tag> <test file rel to DB> <const name> <emoji> <doc> [--append] [--rfind marker...]"""
import importlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/specs")
V = "️"
REPO = "/Users/ueli/Documents/semio"
DB = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
tag, testrel, const, emoji, doc = sys.argv[1:6]
append = "--append" in sys.argv
rfinds = [a for a in sys.argv[6:] if not a.startswith("--")]
sp = importlib.import_module("specs." + tag).SPEC
root_dir = os.path.dirname(os.path.join(REPO, sp["file"]))
tp = os.path.join(REPO, DB, testrel)
text = open(tp, encoding="utf-8").read()
tdir = os.path.dirname(tp)
rel_root = os.path.relpath(root_dir, tdir)
lit_root = rel_root + "/🦀️.rs"
pat = re.compile(r'include_str!\("' + re.escape(lit_root) + r'"\)')
if const + ": &str" in text:
    sys.exit("already re-pointed")
n = len(pat.findall(text))
assert n > 0, (tag, "no literal", lit_root)
parts = ['    include_str!("%s/%s/🦀️.rs"),' % (rel_root, c["folder"]) for c in sp["children"]] + ['    include_str!("%s"),' % lit_root]
block = "/// %s%s %s\nconst %s: &str = concat!(\n%s\n);\n" % (emoji, V, doc, const, "\n".join(parts))
text = pat.sub(const, text)
for m in rfinds:
    old = '.find("%s")' % m
    assert old in text, (tag, "rfind marker missing", m)
    text = text.replace(old, '.rfind("%s")' % m)
lines = text.split("\n")
if append:
    while lines and lines[-1] == "":
        lines.pop()
    lines += ["", block.rstrip("\n"), ""]
else:
    i = 0
    last_use = -1
    while i < len(lines):
        l = lines[i]
        if l.startswith("use ") or l.startswith("pub use "):
            j = i
            while not lines[j].rstrip().endswith(";"):
                j += 1
            last_use = j
            i = j + 1
            continue
        if l.strip() == "" or l.startswith("//") and not l.startswith("///"):
            i += 1
            continue
        break
    assert last_use >= 0, (tag, "no use block")
    lines[last_use + 1:last_use + 1] = ["", block.rstrip("\n")]
open(tp, "w", encoding="utf-8").write("\n".join(lines))
print("re-pointed", tag, testrel[-60:], "literals", n, "children", len(sp["children"]))
