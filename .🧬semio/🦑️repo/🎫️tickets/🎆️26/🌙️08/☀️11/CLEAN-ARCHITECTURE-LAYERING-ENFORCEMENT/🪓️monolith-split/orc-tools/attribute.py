"""Attributes the differential errors per file: for each configuration, errors of the baseline (A) and current (B) sandbox grouped by source file, resolution/privacy class only, surviving files compared one by one.
Usage: attribute.py <label> <cfg>"""
import os, re, sys, collections
from orclib import *
sys.path.insert(0, F + "/orc-tools")
label, cfg = sys.argv[1], sys.argv[2]
RES = {"E0432", "E0433", "E0412", "E0425", "E0422", "E0423", "E0424", "E0426", "E0603", "E0616", "E0624", "E0451", "E0404", "E0405", "E0407", "E0583", "E0599", "E0609", "E0560", "E0063"}
HEAD = re.compile(r"^(error(?:\[(E\d+)\])?): (.*)$")
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")
def parse(path):
    L = open(path, encoding="utf8").read().split("\n")
    out, i = [], 0
    while i < len(L):
        m = HEAD.match(L[i])
        if m and "aborting" not in L[i]:
            loc = ""; label_ = ""
            for j in range(i + 1, min(i + 14, len(L))):
                if HEAD.match(L[j]) or L[j].startswith("warning"): break
                mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
                if mm and not loc: loc = mm.group(1) + ":" + mm.group(2)
                ml = re.search(r"\^+ ?(.*)$", L[j])
                if ml and not label_ and "|" in L[j]: label_ = ml.group(1)
            out.append((m.group(2) or "", PATHRX.sub("", m.group(3)), PATHRX.sub("", label_), loc))
        i += 1
    return out
a = parse(G + "/orc-oracle-%s-%s-A.err" % (label, cfg))
b = parse(G + "/orc-oracle-%s-%s-B.err" % (label, cfg))
def fileof(loc):
    f = loc.rsplit(":", 1)[0].replace("<S>/", "")
    f = f.replace("🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/", "")
    return os.path.normpath(f)
SURV = {"🦀️.rs", "🌉️bridge/🦀️.rs", "🧪️tests/🔬️unit/🦀️.rs", "🧪️tests/🧬️record-codec-laws/🦀️.rs", "🧪️tests/🧬️record-codec-laws-unit/🦀️.rs"}
def group(es):
    d = collections.defaultdict(collections.Counter)
    for code, msg, lab, loc in es:
        if code in RES: d[fileof(loc)][(code, msg, lab)] += 1
    return d
ga, gb = group(a), group(b)
removed = [f for f in ga if f not in SURV]
print("baseline resolution-class errors in files that were removed:", sum(sum(ga[f].values()) for f in removed), "in", len(removed), "files")
print("current  resolution-class errors in files that were removed:", sum(sum(gb[f].values()) for f in gb if f not in SURV))
GLOB_NAMES = {"PackDecodeOptions", "PackEncodeOptions", "PackError", "PackRefusal", "write_varint_u64", "ByteReader"}
def klass(code, msg):
    if code == "E0432" and "unresolved import" in msg and set(re.findall(r"`(\w+)`", msg)) <= GLOB_NAMES: return "glob-neutralised-name"
    if code in ("E0599", "E0609") and "`()`" in msg: return "stub-unit-result"
    return "other"
tot = collections.Counter()
other = []
for f in sorted(SURV):
    ca, cb = ga.get(f, collections.Counter()), gb.get(f, collections.Counter())
    print("== surviving file", f, "baseline", sum(ca.values()), "current", sum(cb.values()))
    for k, n in sorted((cb - ca).items()):
        c = klass(k[0], k[1]); tot[c] += n
        if c == "other": other.append((f, k, n))
        print("   only-current %dx [%s] %s %s | %s" % (n, c, k[0], k[1][:120], k[2][:60]))
    if "-v" in sys.argv:
        for k, n in sorted((ca - cb).items()): print("   only-baseline %dx %s %s | %s" % (n, k[0], k[1][:140], k[2][:70]))
print("ONLY-CURRENT CLASSES", dict(tot), "OTHER", len(other))
for f, k, n in other: print("   UNEXPLAINED", f, n, k)
