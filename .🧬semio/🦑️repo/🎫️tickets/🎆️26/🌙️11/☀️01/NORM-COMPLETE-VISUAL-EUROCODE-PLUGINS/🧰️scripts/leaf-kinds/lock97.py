"""Adds the new EN 1997 mutation crates to the Cargo.lock files: package blocks sorted like cargo, and the dependency on them in every package that depends on the anchor crate (idempotent; reads before it writes)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

SHORT = "en1997"
ART = next(NORM + "/🗿️artifacts/" + n for n in os.listdir(NORM + "/🗿️artifacts") if n.endswith(SHORT))
ANCHOR = "semio-s-artifact-norm-en1997-mut-ground"
LOCKS = [ART + "/Cargo.lock", SEMIO + "/🌎️hub/Cargo.lock", NORM + "/🏭️bridge/Cargo.lock", SEMIO + "/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock"]
names = sorted(set(re.findall(r"\[workspace\.dependencies\.(semio-s-artifact-norm-en1997-mut-[a-z0-9-]+)\]", rd(ART + "/Cargo.toml"))))


def deps_of(b):
    m = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
    return (m, set(re.findall(r'"([^"]+)"', m.group(1)))) if m else (None, set())


def fix(t):
    parts = re.split(r"(?m)^(?=\[\[package\]\]\n)", t)
    head, pk = parts[0], parts[1:]
    tpl = next(b for b in pk if b.startswith('[[package]]\nname = "%s"\n' % ANCHOR))
    have = {re.search(r'name = "([^"]+)"', b).group(1) for b in pk}
    for n in names:
        if n not in have:
            pk.append(tpl.replace(ANCHOR, n))
    out = []
    for b in pk:
        nm = re.search(r'name = "([^"]+)"', b).group(1)
        m, deps = deps_of(b)
        if m and nm not in names:
            want = deps | set(names) if ANCHOR in deps else deps - set(names)
            if want != deps:
                b = b[: m.start(1)] + "".join(' "%s",\n' % x for x in sorted(want)) + b[m.end(1):]
        out.append((nm, b))
    out.sort(key=lambda x: x[0])
    return head + "".join(b if b.endswith("\n\n") or i == len(out) - 1 else b + "\n" for i, (n, b) in enumerate(out))


for p in LOCKS:
    t = rd(p)
    if not t or ANCHOR not in t:
        print("skip", p[len(SEMIO):])
        continue
    new = fix(t)
    assert len(new) >= len(t) * 0.9
    open(p, "w", encoding="utf-8").write(new)
    print("ok", p[len(SEMIO):], len(t), len(new))
