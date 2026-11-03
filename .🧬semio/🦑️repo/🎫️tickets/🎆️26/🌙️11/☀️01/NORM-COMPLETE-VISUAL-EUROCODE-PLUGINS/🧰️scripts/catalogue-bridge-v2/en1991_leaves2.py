import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib2
from lib import *
import leaves as L
A = Art("en1991")
CASES = "semio_s_artifact_norm_en1991_checks::catalogue::En1991Cases"
import re
proto = rd(A.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio")
for k in ("change-case-number", "change-case-text"):
    src = T.mut + "/" + LEAF_DIRS[k]
    dst = A.mut + "/" + LEAF_DIRS[k]
    assert os.path.isdir(src) and not os.path.exists(dst)
    tag = int(re.search(r"record %s tag=(\d+)" % k, proto).group(1))
    for dp, dn, fn in os.walk(src):
        for f in fn:
            s = os.path.join(dp, f)
            d = os.path.join(dst, os.path.relpath(s, src))
            text = L.subst(A, rd(s), CASES)
            if os.path.relpath(s, src) == "🔣️.json":
                j = json.loads(text); j["binaryTag"] = tag; text = json.dumps(j, indent=2, ensure_ascii=False) + "\n"
            wr(d, text)
print(len(LOG), [k for k in KINDS])
