import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import leaves as L
A = Art("en1996")
CASES = "semio_s_artifact_norm_en1996_masonry::En1996Cases"
proto = A.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
top = max(int(x) for x in re.findall(r"tag=(\d+)", rd(proto)))
tags = {k: top + 1 + i for i, k in enumerate(KINDS)}
L.install_leaf_dirs(A, CASES, tags)
cname, consumers = L.family_crate(A, [A.crate + "masonry"])
L.root_enum(A, cname)
ext = L.root_json(A)
L.root_ts(A, ext)
wr(proto, rd(proto).rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k, tags[k]) for k in KINDS))
L.edit_rules(A)
print(len(LOG), [os.path.relpath(c, A.root) for c in consumers], ext)
