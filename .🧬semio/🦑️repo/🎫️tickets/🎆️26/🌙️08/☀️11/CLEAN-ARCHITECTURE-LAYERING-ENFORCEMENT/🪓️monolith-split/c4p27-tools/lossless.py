import sys, collections, re
def lines(p):
    return [l.strip() for l in open(p, encoding="utf8").read().split("\n") if l.strip()]
def kind(l):
    if re.match(r"^import\b", l) or re.match(r"^\} from ", l): return "import"
    if re.match(r"^//\s*#(end)?region", l): return "region"
    return "code"
def main():
    label, orig, news = sys.argv[1], sys.argv[2], sys.argv[3:]
    o = lines(orig); n = []
    for p in news: n += lines(p)
    bad = 0
    for k in ("code", "region", "import"):
        co = collections.Counter(l for l in o if kind(l) == k)
        cn = collections.Counter(l for l in n if kind(l) == k)
        lost = co - cn; gained = cn - co
        print(f"{label} [{k}] orig={sum(co.values())} new={sum(cn.values())} only-in-orig={sum(lost.values())} only-in-new={sum(gained.values())}")
        if k != "import":
            for l, c in list(lost.items())[:25]: print("   LOST  ", c, l[:160])
            for l, c in list(gained.items())[:25]: print("   GAINED", c, l[:160])
main()
