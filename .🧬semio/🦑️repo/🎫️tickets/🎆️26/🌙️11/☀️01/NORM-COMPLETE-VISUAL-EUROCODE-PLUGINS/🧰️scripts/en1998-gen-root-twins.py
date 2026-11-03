#!/usr/bin/env python3
"""Regenerates the EN 1998 mutation root `🔗️.graphql` and `🛰️.proto` from the root `KINDS` and each kind's payload schema (idempotent)."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, os, re
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ART + "/🧬️schema/🧬️mutations"
root = open(MUT + "/🦀️.rs", encoding="utf-8").read()
k0 = root.index("pub const KINDS"); kinds = re.findall(r'"([a-z0-9-]+)"', root[k0:root.index("];", k0)])
dirs = {}
for d in os.listdir(MUT):
    c = MUT + "/" + d + "/🔣️.json"
    if os.path.isfile(c) and os.path.isfile(MUT + "/" + d + "/🧬️schema/🔣️.json"):
        j = json.load(open(c, encoding="utf-8")); dirs[j["semanticKind"]] = (d, j["aggregateVariant"])
missing = [k for k in kinds if k not in dirs]
assert not missing, missing
def snake(c): return re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", c).replace("__", "_").lower() if False else "".join("_" + x.lower() if x.isupper() else x for x in c)
def gql(p):
    t = p.get("type")
    if "$ref" in p or t == "object" or t == "array": return "JSON!"
    if isinstance(t, list): return "Int"
    return {"integer": "Int!", "number": "Float!", "boolean": "Boolean!", "string": "String!"}[t]
def pro(p):
    t = p.get("type")
    if "$ref" in p or t == "object" or t == "array": return "bytes"
    if isinstance(t, list): return "optional uint32"
    return {"integer": "uint32", "number": "double", "boolean": "bool", "string": "string"}[t]
g = ["# 🧬️ EN 1998 mutation schema — semantic kinds matching Rust `KINDS`.\n", "union En1998Mutation =\n    " + "\n  | ".join(dirs[k][1] for k in kinds) + "\n", "# KINDS: " + ", ".join(kinds) + "\n"]
p = ['// 🧬️ EN 1998 mutation schema — semantic kinds matching Rust `KINDS`.\nsyntax = "proto3";\npackage semio.s.norm.en1998.mutation;\n']
oneof = []
for i, k in enumerate(kinds, 1):
    d, rec = dirs[k]
    props = json.load(open(MUT + "/" + d + "/🧬️schema/🔣️.json", encoding="utf-8"))["properties"]
    props = {n: v for n, v in props.items() if n != "mutation"}
    g.append("# kind: \"%s\"\ntype %s { %s }" % (k, rec, " ".join("%s: %s @state(class: ARTIFACT)" % (n, gql(v)) for n, v in props.items())))
    p.append("// kind: %s\nmessage %s { %s }" % (k, rec, " ".join("/* @state artifact */ %s %s = %d;" % (pro(v), snake(n), j) for j, (n, v) in enumerate(props.items(), 1))))
    oneof.append("    %s %s = %d;" % (rec, k.replace("-", "_"), i))
p.append("message En1998Mutation {\n  oneof payload {\n" + "\n".join(oneof) + "\n  }\n}\n")
open(MUT + "/🔗️.graphql", "w", encoding="utf-8").write("\n".join(g) + "\n")
open(MUT + "/🛰️.proto", "w", encoding="utf-8").write("\n".join(p) + "\n")
print("kinds", len(kinds))
