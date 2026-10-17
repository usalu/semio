"""🧮 C4P5 oracle row diff: compares the final rustc JSON diagnostics of two pos.py runs (`c4p5-<family>-<label>-cur-<cfg>-final.err`) as multisets keyed by
(cfg, code, normalised message, normalised primary label, crate). File paths and line numbers are dropped, because the split moves code between files;
the stub-layer noise is kept in the comparison, so a single new or lost diagnostic shows. usage: posdiff.py <labelA> <labelB> [cfg...]"""
import collections, json, os, re, sys

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
PATHRX = re.compile(r"\b(?:_*[a-z][a-z0-9_]*::)+(?=[A-Za-z_{(<])")
a_label, b_label = sys.argv[1], sys.argv[2]
cfgs = sys.argv[3:] or ["lib", "test"]
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import injlib
OVERLAY = F + "/🗑️generated/c4p5-stage"
INJ = injlib.ranges(OVERLAY) if os.path.isdir(OVERLAY) else {}
SKIPPED = collections.Counter()


def nrm(s):
    s = re.sub(r"(?:crate::)?__m_[A-Za-z0-9_]+::", "", s)
    s = re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", s)
    s = re.sub(r"\bTr\d+\b", "Tr", s)
    s = re.sub(r"\{(closure|async block|async fn body)@[^}]*\}", r"{\1}", s)
    return PATHRX.sub("", s)


def load(label, cfg):
    p = f"{GEN}/c4p5-c4p5-{label}-cur-{cfg}-final.err"
    keys = collections.Counter()
    where = {}
    for line in open(p, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error", "error: internal compiler error"):
            continue
        msg = d.get("message", "")
        if msg.startswith("aborting due to") or msg.startswith("could not compile") or "previous error" in msg:
            continue
        code = (d.get("code") or {}).get("code") or ""
        prim = next((s for s in d.get("spans", []) if s.get("is_primary")), (d.get("spans") or [None])[0])
        file_, label_ = "", ""
        if prim:
            file_ = prim.get("file_name", "")
            label_ = prim.get("label") or ""
        m = re.match(r"^(__m_[A-Za-z0-9_]+)/", file_)
        crate = m.group(1) if m else ""
        if file_.endswith("__pos_root.rs"):
            continue
        if label == b_label and prim:
            hit = next((r for r, (a, b) in INJ.items() if file_.endswith(r) and a <= prim.get("line_start", 0) <= b), None)
            if hit:
                SKIPPED[cfg] += 1
                continue
        key = (cfg, code, nrm(msg), nrm(label_), crate)
        keys[key] += 1
        where.setdefault(key, []).append(file_.split("/")[-2] + ":" + str(prim.get("line_start", 0) if prim else 0))
    return keys, where


bad = 0
for cfg in cfgs:
    a, wa = load(a_label, cfg)
    b, wb = load(b_label, cfg)
    only_a, only_b = a - b, b - a
    print(f"   (emulation rows inside injected parent imports of staged children, not counted: {SKIPPED[cfg]})")
    print(f"== {cfg}: rows {a_label}={sum(a.values())} {b_label}={sum(b.values())} only-{a_label}={sum(only_a.values())} only-{b_label}={sum(only_b.values())}")
    for k, n in sorted(only_b.items())[:80]:
        print(f"   +{b_label} {n}x {k[1]} {k[0]} {k[2][:150]} | {k[3][:60]} | {k[4][-40:]}  @ {wb[k][:2]}")
    for k, n in sorted(only_a.items())[:80]:
        print(f"   -{a_label} {n}x {k[1]} {k[0]} {k[2][:150]} | {k[3][:60]} | {k[4][-40:]}  @ {wa[k][:2]}")
    if only_a or only_b:
        bad += 1
print("POSDIFF", "IDENTICAL" if bad == 0 else f"{bad} cfg differ")
sys.exit(1 if bad else 0)
