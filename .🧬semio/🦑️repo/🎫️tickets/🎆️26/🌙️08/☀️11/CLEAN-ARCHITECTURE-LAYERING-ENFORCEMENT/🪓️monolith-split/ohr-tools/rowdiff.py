"""🧮 OHR row diff (read only): compares the non-stub rows of two runs of the same family and configuration (key = code, repo-relative file without the line, normalised message), prints the rows that disappeared and the new ones.
usage: rowdiff.py <old family-label> <new family-label> <lib|test|wasm> [--show-new] [--codes E0599,...]"""
import collections, os, re, sys
GEN = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + "/🗑️generated"
def load(fl, cfg):
    fam, label = fl.rsplit(":", 1)
    p = f"{GEN}/ohr-{fam}-{label}-cur-{cfg}-it1-nonstub.txt"
    if not os.path.exists(p):
        p = f"{GEN}/ohr-{fam}-{label}-cur-{cfg}-it0-nonstub.txt"
    t = open(p, encoding="utf-8").read()
    out = []
    for blk in re.split(r"(?m)^#\d+ ", t)[1:]:
        head, _, body = blk.partition("\n")
        code = head.split(" ")[0]
        m = re.search(r"-->\s+(\S+):(\d+)", body)
        f = re.sub(r"^__m_[A-Za-z0-9_]+/", "", m.group(1)) if m else ""
        msg = re.sub(r"\s+", " ", re.sub(r"[0-9]+", "N", body.split("\n")[0]))[:140]
        out.append(((code, f, msg), m.group(2) if m else "", body))
    return out
a = load(sys.argv[1], sys.argv[3]); b = load(sys.argv[2], sys.argv[3])
ka = collections.Counter(k for k, _, _ in a); kb = collections.Counter(k for k, _, _ in b)
gone, new = ka - kb, kb - ka
codes = sys.argv[sys.argv.index("--codes") + 1].split(",") if "--codes" in sys.argv else None
print("old", len(a), "new", len(b), "gone", sum(gone.values()), "new rows", sum(new.values()))
print("gone by code", dict(collections.Counter(k[0] for k in gone.elements())))
print("new by code", dict(collections.Counter(k[0] for k in new.elements())))
if "--show-new" in sys.argv:
    for k, v in new.items():
        if codes is None or k[0] in codes:
            print("NEW", v, k[0], k[1][-90:], k[2][:100])
