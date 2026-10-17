"""🔬 c4p29 oracle row diff by SOURCE TEXT: keys every rustc JSON error by (code, normalised message, primary label, stripped text of the primary span's source line), so a row survives the move of its line to another file.
usage: C4P29_FAMILY=c4p29 python3 c4pposline.py <labelA> <labelB> [cfg...]"""
import collections, json, os, re, sys
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
PATHRX = re.compile(r"\b(?:_*[a-z][a-z0-9_]*::)+(?=[A-Za-z_{(<])")
fam = os.environ.get("C4P29_FAMILY", "c4p29")
a_label, b_label = sys.argv[1], sys.argv[2]
cfgs = sys.argv[3:] or ["lib", "test"]


def nrm(s):
    s = re.sub(r"(?:crate::)?__m_[A-Za-z0-9_]+::", "", s)
    s = re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", s)
    s = re.sub(r"\bTr\d+\b", "Tr", s)
    s = re.sub(r"\{(closure|async block|async fn body)@[^}]*\}", r"{\1}", s)
    s = re.sub(r"(?:\./)+", "", s)
    return PATHRX.sub("", s)


def load(label, cfg):
    p = f"{GEN}/c4p29-{fam}-{label}-cur-{cfg}-final.err"
    keys = collections.Counter()
    for line in open(p, encoding="utf-8", errors="replace"):
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error", "error: internal compiler error"):
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or "previous error" in msg or msg.startswith("could not compile"):
            continue
        prim = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        if prim and prim.get("file_name", "").endswith("__pos_root.rs"):
            continue
        txt = ""
        if prim and prim.get("text"):
            txt = " ".join(t.get("text", "").strip() for t in prim["text"])
        code = (d.get("code") or {}).get("code") or ""
        keys[(cfg, code, nrm(msg), nrm((prim or {}).get("label") or ""), nrm(txt))] += 1
    return keys


bad = 0
for cfg in cfgs:
    a, b = load(a_label, cfg), load(b_label, cfg)
    oa, ob = a - b, b - a
    print(f"== {cfg}: rows {a_label}={sum(a.values())} {b_label}={sum(b.values())} only-{a_label}={sum(oa.values())} only-{b_label}={sum(ob.values())}")
    for tag, c in ((f"-{a_label}", oa), (f"+{b_label}", ob)):
        for k, v in sorted(c.items(), key=lambda kv: -kv[1])[:25]:
            print(f"   {tag} {v}x {k[1]} {k[2][:110]} | {k[4][:110]}")
    bad += 1 if (oa or ob) else 0
print("POSLINE", "IDENTICAL" if not bad else f"{bad} cfg differ")
