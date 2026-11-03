import re
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import leaves as L
A = Art("en1997")
CASES = "semio_s_artifact_norm_en1997_checks_catalogue::catalogue::En1997Cases"
proto = A.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
top = max(int(x) for x in re.findall(r"tag=(\d+)", rd(proto)))
tags = {k: top + 1 + i for i, k in enumerate(KINDS)}
L.install_leaf_dirs(A, CASES, tags)
cname, consumers = L.family_crate(A, [A.crate + "checks-catalogue"])
L.root_enum(A, cname)
ext = L.root_json(A)
L.root_ts(A, ext)
s = rd(proto).rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k, tags[k]) for k in KINDS)
wr(proto, s)
print(len(LOG), "files; consumers:", [os.path.relpath(c, A.root) for c in consumers], "external-tag:", ext)
