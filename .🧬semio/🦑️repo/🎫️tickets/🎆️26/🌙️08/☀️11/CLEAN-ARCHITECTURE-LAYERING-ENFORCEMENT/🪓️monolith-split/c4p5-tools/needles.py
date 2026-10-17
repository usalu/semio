"""🔎 C4P5 reader needles: the Rust test targets that read the editor root source assert `source.contains(<literal>)` and mutate it with `source.replace(<literal>, ..)`.
For every such literal in the consumer files this compares its occurrence count in the ORIGINAL root text with the count in the split tree text (children first, root last, `pub(super) ` widening NOT removed),
so a needle broken by the split (moved across a file seam, or broken by a `pub(super) ` prefix) shows. usage: needles.py stage|live"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
E = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
mode = sys.argv[1]
base = F + "/🗑️generated/c4p5-stage" if mode == "stage" else REPO
KIDS = ["🧰️retained-commands", "📬️store-preparation", "✂️clipboard-job", "🧩️play-app", "📋️manifest"]
orig = open(F + "/🗑️generated/c4p5-orig/5.rs", encoding="utf-8").read()
new = "\n".join([open(f"{base}/{E}/{k}/🦀️.rs", encoding="utf-8").read() for k in KIDS] + [open(f"{base}/{E}/🦀️.rs", encoding="utf-8").read()])
consumers = [f"{E}/🧪️tests/🔬️unit/🦀️.rs", f"{E}/🧪️tests/🧵️retained-wiring/🦀️.rs", "../c4p5-orig/6.rs"]
if mode == "live":
    consumers[2] = f"{E}/🧪️tests/🔬️budgets/🦀️.rs"
STR = r'(?:r(?P<h>#*)"(?P<raw>.*?)"(?P=h)|"(?P<norm>(?:[^"\\]|\\.)*)")'
CALL = re.compile(r"\.(contains|replace|replacen|matches|find|split)\(\s*" + STR, re.S)


def unescape(s):
    return s.replace('\\"', '"').replace("\\n", "\n").replace("\\\\", "\\").replace("\\t", "\t")


def texts(rel):
    if rel.startswith("../"):
        return open(F + "/🗑️generated/" + rel[3:], encoding="utf-8").read()
    p = f"{base}/{rel}" if mode == "stage" and os.path.isfile(f"{base}/{rel}") else f"{REPO}/{rel}"
    return open(p, encoding="utf-8").read()


def fragments(rel):
    if mode != "stage" or "budgets" not in rel and not rel.startswith("../"):
        return texts(rel)
    root = texts(rel)
    return root


bad = 0
total = 0
for rel in consumers:
    src = texts(rel)
    if rel.startswith("../") or "budgets" in rel:
        d = f"{base}/{E}/🧪️tests/🔬️budgets" if mode == "stage" else f"{REPO}/{E}/🧪️tests/🔬️budgets"
        if mode == "stage":
            src = "\n".join(open(os.path.join(dp, f), encoding="utf-8").read() for dp, dn, fn in os.walk(d) for f in fn if f.endswith(".rs"))
        else:
            src = "\n".join(open(os.path.join(dp, f), encoding="utf-8").read() for dp, dn, fn in os.walk(d) for f in fn if f.endswith(".rs"))
    rows = 0
    for m in CALL.finditer(src):
        needle = m.group("raw") if m.group("raw") is not None else unescape(m.group("norm") or "")
        if len(needle) < 6:
            continue
        rows += 1
        total += 1
        a, b = orig.count(needle), new.count(needle)
        if a != b:
            bad += 1
            print(f"DIFF {os.path.basename(os.path.dirname(rel)) or rel}: original={a} split={b} needle={needle[:110]!r}")
    print(f"{rel.split('/')[-2]}: {rows} needles")
print(f"NEEDLES total={total} differing={bad}")
sys.exit(1 if bad else 0)
