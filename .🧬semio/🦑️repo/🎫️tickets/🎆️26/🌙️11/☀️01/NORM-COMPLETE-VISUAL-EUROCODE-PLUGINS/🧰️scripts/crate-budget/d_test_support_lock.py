"""🔒 Adds the new dev-dependency edges of `d_test_support.py` to every Cargo.lock that knows the crate."""
import os, re, sys, tomllib
sys.path.insert(0, os.path.dirname(__file__))
from lib import *
ARTS = NORM + "/🗿️artifacts"
for dp, dn, fn in os.walk(ARTS):
    dn[:] = [d for d in dn if d not in ("target", "node_modules")]
    if "Cargo.toml" not in fn:
        continue
    t = rd(dp + "/Cargo.toml")
    if 'features = ["test-support"]' not in t:
        continue
    c = tomllib.loads(t)
    t_name = c["package"]["name"]
    models = [k for k, v in c.get("dev-dependencies", {}).items() if isinstance(v, dict) and "test-support" in v.get("features", [])]
    for p in lock_files(t_name):
        head, pk = lock_blocks(rd(p))
        names = [pkg_name(b) for b in pk]
        i = names.index(t_name)
        m = re.search(r"dependencies = \[\n(.*?)\]\n", pk[i], re.S)
        deps = set(re.findall(r"(?m)^ \"([^\"\n]+)\",$", m.group(1))) if m else set()
        add = [x for x in models if x not in deps]
        if not add:
            continue
        deps |= set(add)
        body = "dependencies = [\n" + "".join(' "%s",\n' % d for d in sorted(deps)) + "]\n"
        pk[i] = (pk[i][: m.start()] + body + pk[i][m.end():]) if m else re.sub(r'(version = "[^"]+"\n)', lambda x: x.group(1) + body, pk[i], 1)
        wr(p, head + "".join(pk))
        print("lock", t_name, p[len(SEMIO):])
