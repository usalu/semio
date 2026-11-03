import sys, json, re
sys.path.insert(0, ".")
from spec import *
exec(open("sqlite_gen.py", encoding="utf8").read().split("def sql_table")[0])
import io, contextlib
with contextlib.redirect_stdout(io.StringIO()):
    from fixtures_gen import NEWKEYS
Q = S + "/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests"
p = Q + "/🧫️fixtures/🔣️.json"
d = json.load(open(p, encoding="utf8"))
if "anchors" not in d["snapshot"]:
    for k, v in NEWKEYS.items(): d["snapshot"][k] = v
    d["tableNames"] += [t.table for t in tabs]
    open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
clear_rs = "".join(f"s.{e.coll}.clear();" for e in ENTITIES)
p = Q + "/🦀️.rs"
s = open(p, encoding="utf8").read()
if clear_rs not in s:
    s = s.replace("assert_eq!(d.tables.len(),9)", "assert_eq!(d.tables.len(),16)")
    s = s.replace("s.uplift_cases.clear();", "s.uplift_cases.clear();" + clear_rs)
    s = s.replace("snapshot.upliftCases.clear" , "snapshot.upliftCases.clear")
    s = s.replace("snapshot.uplift_cases.clear();", "snapshot.uplift_cases.clear();" + "".join(f"snapshot.{e.coll}.clear();" for e in ENTITIES))
    s = s.replace("source.uplift_cases.clear();", "source.uplift_cases.clear();" + "".join(f"source.{e.coll}.clear();" for e in ENTITIES))
    open(p, "w", encoding="utf8").write(s)
p = Q + "/🟦️.ts"
s = open(p, encoding="utf8").read()
ts_clear = ",".join(f"{e.camel_coll}:[]" for e in ENTITIES)
if ts_clear not in s:
    s = s.replace("slopes:[],upliftCases:[]}", "slopes:[],upliftCases:[]," + ts_clear + "}")
    s = s.replace("source.upliftCases=[];", "source.upliftCases=[];" + "".join(f"source.{e.camel_coll}=[];" for e in ENTITIES))
    open(p, "w", encoding="utf8").write(s)
print("ok")
