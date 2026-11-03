"""🔧 Add `x-semio-rust` {type} to one property of a mutation leaf payload schema (schema-first fix for bare type names in generated leaf Rust). Usage: <schema.json> <property> <rust type> [--also <rs file> <bare> ]"""
import json, sys
f, prop, ty = sys.argv[1:4]
s = open(f, encoding="utf-8").read()
j = json.loads(s)
assert json.dumps(j, indent=2, ensure_ascii=False) + "\n" == s, "not canonical 2-space json"
p = j["properties"][prop]
if p.get("x-semio-rust", {}).get("type") == ty:
    print("already", f)
else:
    p["x-semio-rust"] = {"type": ty}
    open(f, "w", encoding="utf-8").write(json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    print("set", f.split("/")[-3], prop)
