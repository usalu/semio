"""Second pass: finds where each drifting codec's real op tags come from."""
import json, os, re, glob, collections
root = "/Users/ueli/Documents/semio/"
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
items = json.load(open(root + ".tmp-ticket/wp-t2/generated/protocol-survey.json"))
res = collections.Counter()
for o in items:
    proto = root + o["proto"]; binary = os.path.dirname(proto); vocab = os.path.dirname(binary)
    ptxt = open(proto, encoding="utf-8").read()
    rs = open(f"{binary}/🦀️.rs", encoding="utf-8").read() if os.path.exists(f"{binary}/🦀️.rs") else ""
    how = []
    if re.search(r"Direct identities:", ptxt): how.append("direct-comment")
    if "REGISTRY" in rs: how.append("registry")
    m = re.search(r"use [\w:]+::(\w+Mutation)\b", rs)
    enum = m.group(1) if m else None
    if enum:
        for f in [f"{vocab}/🦀️.rs", f"{vocab}/📝️text/🦀️.rs"]:
            if os.path.exists(f):
                t = open(f, encoding="utf-8").read()
                d = re.search(r"#\[derive\(([^)]*)\)\][^\n]*\n(?:#\[[^\n]*\n)*pub enum " + enum + r"\b", t)
                if d: how.append("derive:" + ("OpBinary" if "OpBinary" in d.group(1) else "") + ("Dsl" if "Dsl" in d.group(1) else "") + "@" + os.path.basename(os.path.dirname(f))[-6:])
    if "OP_KEYWORDS" in open(f"{vocab}/🦀️.rs", encoding="utf-8").read() if os.path.exists(f"{vocab}/🦀️.rs") else False: how.append("op-keywords")
    o["how"] = how; o["enum"] = enum
    res[tuple(how)] += 1
json.dump(items, open(root + ".tmp-ticket/wp-t2/generated/protocol-survey2.json", "w"), ensure_ascii=False, indent=1)
for k, v in res.most_common(): print(v, k)
