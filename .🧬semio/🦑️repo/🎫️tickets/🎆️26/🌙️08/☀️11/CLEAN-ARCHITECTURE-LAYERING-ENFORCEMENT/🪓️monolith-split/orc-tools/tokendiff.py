import sys, difflib, re
from orclib import *
import os
a = os.environ.get("ORC_TOKEN_FILE", FW + "/🧬️schema/📡️record-token/🦀️.rs")
b = FW + "/🌱️value/🛬️decode/🦀️.rs"
def key(it):
    if it["kind"] == "impl" or it["name"] is None:
        return ("impl", re.sub(r"\s+", "", it["head"].split("{")[0]))
    return (it["kind"], it["name"])
fb = {}
for it in items(b): fb.setdefault(key(it), []).append(it)
def tn(body):
    s = norm(body, False)
    s = re.sub(r"(semio_framework_dsl_record|semio_framework_value|semio_framework_pack_error|semio_framework_pack_format|semio_framework_replication_codec|protocol::value|protocol::codec|protocol|crate::format|crate::value|crate|super|pack::format|pack)::", "", s)
    return s
for it in items(a):
    if it["kind"] == "use": continue
    k = key(it); c = fb.get(k)
    if not c: print("ONLY-TOKEN", k); continue
    if tn(it["body"]) == tn(c[0]["body"]): print("same", k)
    else:
        print("DIFFERS", k)
        if "-v" in sys.argv:
            for x in list(difflib.unified_diff(it["body"].split("\n"), c[0]["body"].split("\n"), "token", "retained", lineterm="", n=0))[:60]: print("    ", x[:220])
