"""Item-by-item comparison of os-pack record codec files against the framework pack-record crates."""
import sys, difflib, re
from orclib import *

PV = OS + "/🌱️value"
FV = FW + "/🌱️value"
os_files = [PV + "/🧱️canonical-forms/🦀️.rs", PV + "/📝️record-encode/🦀️.rs", PV + "/📖️record-decode/🦀️.rs", PV + "/🧮️table-soa/🦀️.rs", PV + "/📄️document-codec/🦀️.rs", PV + "/🛫️encode/🦀️.rs", PV + "/🦀️.rs", OS + "/🔎️scalar-witness/🦀️.rs"]
fw_files = [FV + "/🦀️.rs", FV + "/🛫️encode/🦀️.rs", FV + "/🔎️scalar-witness/🦀️.rs"]

def key(it):
    if it["kind"] == "impl" or it["name"] is None:
        return ("impl", re.sub(r"\s+", "", it["head"].split("{")[0]))
    return (it["kind"], it["name"])

fw = {}
for f in fw_files:
    for it in items(f):
        fw.setdefault(key(it), []).append(it)

show = "-v" in sys.argv
same = diff = missing = 0
for f in os_files:
    print("=====", f.replace(OS, "OS"))
    for it in items(f):
        if it["kind"] == "use": continue
        k = key(it)
        c = fw.get(k)
        if not c:
            print("  ONLY-OS   ", k, it["start"] + 1); missing += 1; continue
        a = norm(it["body"]); b = norm(c[0]["body"])
        if a == b:
            print("  same      ", k); same += 1
        else:
            diff += 1
            print("  DIFFERS   ", k, "os:%d fw:%s:%d" % (it["start"] + 1, c[0]["file"].replace(FW, "FW"), c[0]["start"] + 1))
            if show:
                d = difflib.unified_diff(it["body"].split("\n"), c[0]["body"].split("\n"), "os", "fw", lineterm="", n=0)
                print("\n".join("      " + x for x in d))
print(same, diff, missing)
