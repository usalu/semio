"""Adds `semio-s-artifact-norm-<art>-mut-placements` to the outer Cargo.lock files (hub, bridge, fixture-sweep): python -I lock_placements.py en1996 en1997. Reads before writing, idempotent."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

ANCHORS = {"en1994": "semio-s-artifact-norm-en1994-mut-studs", "en1998": "semio-s-artifact-norm-en1998-mut-bridges-towers", "en1991": "semio-s-artifact-norm-en1991-mut-gravity", "en1996": "semio-s-artifact-norm-en1996-mut-plan", "en1997": "semio-s-artifact-norm-en1997-mut-ground"}
LOCKS = [SEMIO + "/🌎️hub/Cargo.lock", NORM + "/🏭️bridge/Cargo.lock", SEMIO + "/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock"]


def deps_of(b):
    m = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
    return (m, set(re.findall(r'"([^"]+)"', m.group(1)))) if m else (None, set())


def fix(t, anchor, names):
    parts = re.split(r"(?m)^(?=\[\[package\]\]\n)", t)
    head, pk = parts[0], parts[1:]
    tpl = next(b for b in pk if b.startswith('[[package]]\nname = "%s"\n' % anchor))
    for name in names:
        if not any(b.startswith('[[package]]\nname = "%s"\n' % name) for b in pk):
            pk.append(tpl.replace(anchor, name))
    out = []
    for b in pk:
        nm = re.search(r'name = "([^"]+)"', b).group(1)
        m, deps = deps_of(b)
        if m and nm not in names and not nm.startswith(anchor.rsplit("-mut-", 1)[0] + "-mut-") and anchor in deps and not set(names) <= deps:
            b = b[: m.start(1)] + "".join(' "%s",\n' % x for x in sorted(deps | set(names))) + b[m.end(1):]
        out.append((nm, b))
    out.sort(key=lambda x: x[0])
    return head + "".join(b if b.endswith("\n\n") or i == len(out) - 1 else b + "\n" for i, (n, b) in enumerate(out))


for p in LOCKS:
    t = rd(p)
    new = t
    for short in sys.argv[1:]:
        anchor = ANCHORS[short]
        if anchor in new:
            base = anchor.rsplit("-mut-", 1)[0] + "-mut-"
            new = fix(new, anchor, [base + x for x in (["bridge-lanes", "construction-stages", "roof-surfaces", "placements"] if short == "en1991" else ["placements"])])
    if new == t:
        print("unchanged", p[len(SEMIO):])
        continue
    assert len(new) >= len(t) * 0.99 and len(t) > 1000
    open(p, "w", encoding="utf-8").write(new)
    print("ok", p[len(SEMIO):], len(t), len(new))
