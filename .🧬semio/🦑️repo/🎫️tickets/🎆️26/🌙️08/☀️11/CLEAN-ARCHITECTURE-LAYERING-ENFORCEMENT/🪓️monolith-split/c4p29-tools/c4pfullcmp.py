"""🧮 c4p29 full-fidelity oracle comparison. The oracle truncates `*-final.err` at 30,000,000 characters, so a split that lengthens diagnostic paths moves the cut and `c4pposline.py` shows phantom lost rows.
These two artefacts are written from the COMPLETE rustc output and are therefore comparable: the privacy-line file `*-priv.err` (every E0603/E0616/E0624/E0451 diagnostic) keyed by (code, message, primary label, source line text),
and the non-stub rows of `*-it1-nonstub.txt` keyed by (code, first rendered line without paths). It also compares the report's `by code` histogram (count of every error code over the full output).
usage: C4P29_FAMILY=c4p29 python3 c4pfullcmp.py <labelA> <labelB> [cfg]"""
import collections, json, os, re, sys
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
fam = os.environ.get("C4P29_FAMILY", "c4p29")
a, b = sys.argv[1], sys.argv[2]
cfg = sys.argv[3] if len(sys.argv) > 3 else "lib"
PATHRX = re.compile(r"\b(?:_*[a-z][a-z0-9_]*::)+(?=[A-Za-z_{(<])")


def nrm(s):
    s = re.sub(r"(?:crate::)?__m_[A-Za-z0-9_]+::", "", s)
    s = re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", s)
    s = re.sub(r"(?:\./)+", "", s)
    return re.sub(r"\s+", " ", PATHRX.sub("", s)).strip()


def priv(label):
    keys = collections.Counter()
    for line in open(f"{GEN}/c4p29-{fam}-{label}-cur-{cfg}-priv.err", encoding="utf-8", errors="replace"):
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        prim = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        txt = " ".join(t.get("text", "").strip() for t in (prim or {}).get("text", []))
        keys[((d.get("code") or {}).get("code"), nrm(d.get("message", "")), nrm((prim or {}).get("label") or ""), nrm(txt))] += 1
    return keys


def nonstub(label):
    keys = collections.Counter()
    p = f"{GEN}/c4p29-{fam}-{label}-cur-{cfg}-it1-nonstub.txt"
    text = open(p, encoding="utf-8", errors="replace").read()
    for block in re.split(r"(?m)^#\d+ ", text):
        if not block.strip():
            continue
        head, _, rest = block.partition("\n")
        code = head.split(" ")[0]
        first = nrm(rest.split("\n")[0])
        src = " ".join(l.strip() for l in rest.split("\n") if re.match(r"^\s*\d+\s*\|", l))
        keys[(code, first, nrm(re.sub(r"\s*\d+\s*\|", " ", src)))] += 1
    return keys


def bycode(label):
    for line in open(f"{GEN}/c4p29-{fam}-{label}-report.txt", encoding="utf-8", errors="replace"):
        if line.strip().startswith("by code:"):
            return line.strip()
    return ""


bad = 0
for name, fn in (("privacy lines", priv), ("non-stub rows", nonstub)):
    ka, kb = fn(a), fn(b)
    oa, ob = ka - kb, kb - ka
    print(f"== {name}: {a}={sum(ka.values())} {b}={sum(kb.values())} only-{a}={sum(oa.values())} only-{b}={sum(ob.values())}")
    for tag, c in ((f"-{a}", oa), (f"+{b}", ob)):
        for k, v in list(c.items())[:12]:
            print(f"   {tag} {v}x {k[0]} {str(k[1])[:100]} | {str(k[-1])[:100]}")
    bad += 1 if (oa or ob) else 0
ca, cb = bycode(a), bycode(b)
print("== by code histogram (all errors of the complete output):", "IDENTICAL" if ca == cb and ca else "DIFFERENT")
if ca != cb:
    bad += 1
    print("   ", a, ca[:300])
    print("   ", b, cb[:300])
print("FULLCMP", "IDENTICAL" if not bad else f"{bad} differ")
