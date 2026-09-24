"""Classifies every drifting 📡️.protocol.semio by how its sibling Rust codec frames an op."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
rows = json.load(open(sys.argv[1]))
out = []
for x in rows:
    if x["id"] != "binary-protocol-drift": continue
    proto = root + x["scope"]; binary = os.path.dirname(proto); vocab = os.path.dirname(binary)
    kinds = sorted(KEBAB.search(d).group(0) for d in os.listdir(vocab) if os.path.isdir(f"{vocab}/{d}") and KEBAB.search(d))
    rs = open(f"{binary}/🦀️.rs", encoding="utf-8").read() if os.path.exists(f"{binary}/🦀️.rs") else ""
    text_rs = f"{vocab}/📝️text/🦀️.rs"
    mode = "variants_binary" if "variants_binary" in rs else ("derive-OpBinary" if "OpBinary" in rs and "derive" in rs else ("no-rs" if not rs else "other"))
    dsl = None
    if os.path.exists(text_rs):
        m = re.search(r"pub enum ([A-Za-z0-9]+OperationDsl)", open(text_rs, encoding="utf-8").read())
        dsl = m.group(1) if m else None
    out.append({"proto": x["scope"], "kinds": len(kinds), "mode": mode, "dsl": dsl, "head": open(proto, encoding="utf-8").read().split("\n")[:3]})
json.dump(out, open(sys.argv[2], "w"), ensure_ascii=False, indent=1)
import collections
print(collections.Counter((o["mode"], o["dsl"] is not None) for o in out))
for o in out:
    if o["mode"] != "variants_binary" or not o["dsl"]: print(o["mode"], o["dsl"], o["proto"].split("🗿️artifacts/")[-1][:90])
