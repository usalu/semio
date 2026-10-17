import json,sys,os
# usage: pst-registry.py add <reason> <rows.json>   (rows: [{path,name}])    |  pst-registry.py reasons
ROOT="/Users/ueli/Documents/semio/"
REG=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧱️unowned/🔣️.json"
REASONS={
 "in-memory-model":"The parser validates the owned in-memory model of the artifact (exact Binary64 and Binary32 words, bigint counters, Uint8Array octets, transports of those words), while the normative JSON Schema states the JSON transport (plain numbers, decimal strings, bare objects). Regenerating it from the schema would change what it admits; the schema must first state the in-memory model through a binding to the framework value parsers.",
 "iterative-walk":"The parser walks an unbounded tree with an explicit stack so that a deep document cannot overflow the call stack; the generator only emits recursion.",
 "unrenderable":"No renderer covers the schema form of the export (an untyped node, a form the projection does not know), or the parser is generic over type arguments, so there is no projection to compare with.",
 "normalizing-reader":"The parser is a lenient reader: it fills defaults for fields its schema requires (and builds a default object through a function), so it admits documents the schema refuses. The caller depends on that normalisation; the owner must decide whether the schema stops requiring those fields.",
 "cross-field-rule":"The parser enforces a rule across fields (for example a pixel selection must agree with the configured paint target) that the schema does not state; a projection of the schema alone would admit what the rule refuses.",
 "schema-disagrees-with-type":"The normative schema and the declared TypeScript type of the export disagree (fields, variants or element types the other side lacks), so a parser projected from the schema does not produce the declared type; the owner must decide which side is right.",
}
def load():
    return json.load(open(REG,encoding="utf8"))
def save(d):
    d["parsers"]=sorted(d["parsers"],key=lambda r:(r["path"].encode(),r["name"].encode()))
    open(REG,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
if sys.argv[1]=="add":
    d=load(); d["reasons"]={**d.get("reasons",{}),**{k:v for k,v in REASONS.items() if k==sys.argv[2] or k in d.get("reasons",{})}}
    have={(r["path"],r["name"]) for r in d["parsers"]}
    n=0
    for r in json.load(open(sys.argv[3],encoding="utf8")):
        if (r["path"],r["name"]) in have: continue
        d["parsers"].append({"path":r["path"],"name":r["name"],"reason":sys.argv[2]}); n+=1
    save(d); print("added",n,"total",len(d["parsers"]))
elif sys.argv[1]=="remove":
    d=load(); drop={(r["path"],r["name"]) for r in json.load(open(sys.argv[2],encoding="utf8"))}
    d["parsers"]=[r for r in d["parsers"] if (r["path"],r["name"]) not in drop]; save(d); print("total",len(d["parsers"]))
