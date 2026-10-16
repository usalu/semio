"""Test-file comparison os-pack vs framework: fn-by-fn normalised bodies."""
import sys, difflib, re
from orclib import *
PV = OS + "/🌱️value"; FV = FW + "/🌱️value"
pairs = [
 (PV + "/🧪️tests/🔬️unit/🦀️.rs", [FV + "/🧪️tests/🔬️unit/🦀️.rs", FV + "/🛬️decode/🧪️tests/🔬️unit/🦀️.rs"]),
 (PV + "/🧪️tests/🚦️refusals/🦀️.rs", [FV + "/🧪️tests/🚦️refusals/🦀️.rs"]),
]
def key(it):
    if it["kind"] == "impl" or it["name"] is None:
        return ("impl", re.sub(r"\s+", "", it["head"].split("{")[0]))
    return (it["kind"], it["name"])
def tn(body):
    s = norm(body, False)
    s = re.sub(r"(semio_framework_dsl_record|semio_framework_value|semio_framework_pack_error|semio_framework_pack_format|semio_framework_replication_codec|protocol::value|protocol::codec|protocol|crate::format|crate::value|crate|super|pack::format|pack)::", "", s)
    return s
for osf, fwfs in pairs:
    fw = {}
    for f in fwfs:
        for it in items(f): fw.setdefault(key(it), []).append(it)
    print("=====", osf.replace(OS, "OS"))
    for it in items(osf):
        if it["kind"] == "use": continue
        k = key(it); c = fw.get(k)
        if not c: print("  ONLY-OS ", k); continue
        a = tn(it["body"]); b = tn(c[0]["body"])
        if a == b: print("  same    ", k)
        else:
            print("  DIFFERS ", k)
            if "-v" in sys.argv:
                for x in difflib.unified_diff(it["body"].split("\n"), c[0]["body"].split("\n"), "os", "fw", lineterm="", n=0): print("      ", x[:200])
