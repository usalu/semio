"""🔢 rii: argument count of every call of the imported infinite free functions versus the definition. usage: arity.py <rs-file>..."""
import json, os, re, sys
GEN = os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated"
idx = json.load(open(GEN + "/rii-infinite-fn-index.json"))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rows import NAMES
def params(sig):
    inner = sig[sig.index("(") + 1: sig.rindex(")")] if ")" in sig else ""
    d = 0; n = 0; cur = ""
    for ch in inner:
        if ch in "<([{": d += 1
        if ch in ">)]}": d -= 1
        if ch == "," and d == 0:
            if cur.strip(): n += 1
            cur = ""
        else:
            cur += ch
    if cur.strip(): n += 1
    return n
def args(text, start):
    d = 0; n = 0; cur = ""; i = start
    while i < len(text):
        ch = text[i]
        if ch in "([{": d += 1
        if ch in ")]}":
            d -= 1
            if d == 0:
                if cur.strip(): n += 1
                return n
        if ch == "|" and d == 1:
            pass
        if ch == "," and d == 1:
            if cur.strip(): n += 1
            cur = ""
        elif d >= 1 and i > start:
            cur += ch
        i += 1
    return -1
bad = 0
total = 0
for f in sys.argv[1:]:
    t = open(f, encoding="utf8").read()
    t = re.sub(r"//[^\n]*", "", t)
    for name in NAMES:
        defs = [e for e in idx.get(name, [])]
        if not defs:
            continue
        want = params(defs[0][3])
        for m in re.finditer(r"(?<![\w.:])" + name + r"\(", t):
            got = args(t, m.end() - 1)
            total += 1
            if got != want:
                bad += 1
                print("ARITY", os.path.basename(os.path.dirname(f)), t.count("\n", 0, m.start()) + 1, name, "got", got, "want", want)
print("calls checked", total, "mismatches", bad)
