import sys, collections, re
def lines(p, strip_docs=False):
    out = []
    for l in open(p, encoding="utf8").read().split("\n"):
        s = l.strip()
        if not s.strip(): continue
        out.append(s)
    return out
def classify(l):
    s = l.strip()
    if s.startswith("import ") : return "import"
    if re.match(r"^//\s*#(end)?region", s): return "region"
    return "code"
def run(label, orig, news):
    o = lines(orig); n = []
    for p in news: n += lines(p)
    for kind in ("code", "region", "import"):
        co = collections.Counter(l for l in o if classify(l) == kind)
        cn = collections.Counter(l for l in n if classify(l) == kind)
        lost = co - cn; gained = cn - co
        print(f"{label} [{kind}] orig={sum(co.values())} new={sum(cn.values())} only-in-orig={sum(lost.values())} only-in-new={sum(gained.values())}")
        if kind != "import":
            for l, c in list(lost.items())[:12]: print("   LOST  ", c, l[:150])
            for l, c in list(gained.items())[:12]: print("   GAINED", c, l[:150])
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/taf-snap/"
L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
IA = L + "⚖️laws/🔬️interactivity-audit/"
run("database", G + "database.orig.ts", [IA + "🗄️database/🟦️.ts", IA + "🗄️database/🌱️database-lifecycle/🟦️.ts"])
run("mounted", G + "mounted.orig.ts", [IA + "🖥️mounted/🟦️.ts", IA + "🖥️mounted/🖼️mounted-surface/🟦️.ts"])
V = L + "🎛️commands/✅️verify/"
run("verify", G + "verify.orig.ts", [V + "🟦️.ts", V + "🎨️storybook-freshness/🟦️.ts"])
