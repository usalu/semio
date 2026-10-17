import json
PP="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection"
path=PP+"/🧪️tests/🧫️fixtures/🎯️ajv-oracle/🔣️.json"
c=json.load(open(path,encoding="utf8"))
if any(case["name"]=="closed" for case in c["cases"]): raise SystemExit("closed case already present")
c["description"]=("Language-agnostic corpus of the schema parser projection: each case lists schema documents (with the hand-written TypeScript a leaf already declares), "
 "and per export the instances every implementation must accept (valid), refuse (invalid), the ones the normative schema accepts but a projection may refuse because it follows a declared type or a safe-integer rule (stricter), "
 "and the ones both admit although they carry a property the schema does not declare (tolerated: only where the schema leaves the object open, because the parser drops that property). "
 "An object states additionalProperties: false exactly where the projection must refuse an unknown property, so every verdict equals the verdict of an independent validator.")
for case in c["cases"]:
    for probe in case["probes"]:
        if "lenient" in probe:
            probe["invalid"] = probe.get("invalid", []) + probe.pop("lenient")
NULL = None
def obj(props, required=None, closed=True, **extra):
    d={"type":"object"}
    if closed: d["additionalProperties"]=False
    if required: d["required"]=required
    d["properties"]=props
    d.update(extra)
    return d
S={"type":"string"}; I={"type":"integer"}
schema={
 "$schema":"http://json-schema.org/draft-07/schema#",
 "$id":"https://json.schemas.assets.semio-tech.com/s/oracle/closed.json",
 "title":"closed",
 "$defs":{
  "Strict":{"title":"Strict",**obj({"a":S,"b":I},["a"])},
  "Loose":{"title":"Loose",**obj({"a":S},["a"],closed=False)},
  "Free":{"title":"Free",**obj({"a":S},["a"],closed=False,additionalProperties=True)},
  "Empty":{"title":"Empty",**obj({})},
  "Nest":{"title":"Nest",**obj({"inner":obj({"n":I},["n"]),"list":{"type":"array","items":obj({"k":S})},"loose":obj({"z":S},closed=False)},["inner"])},
  "MaybeRef":{"anyOf":[{"$ref":"#/$defs/Strict"},{"type":"null"}]},
  "MaybeInline":{"anyOf":[obj({"q":S}),{"type":"null"}]},
  "Dict":{"type":"object","additionalProperties":obj({"v":I},["v"])},
  "Pair":{"type":"array","items":[obj({"a":S},["a"]),S],"minItems":2,"maxItems":2},
  "Tagged":{"oneOf":[obj({"kind":{"const":"n"},"n":I},["kind","n"]),obj({"kind":{"const":"s"},"s":S},["kind"],closed=False)]},
  "External":{"oneOf":[obj({"A":I},["A"]),obj({"B":S},["B"])]},
  "Holder":{"title":"Holder",**obj({"strict":{"$ref":"#/$defs/Strict"},"loose":{"$ref":"#/$defs/Loose"}},["strict","loose"])},
 }}
doc={"name":"closed","seed":"","schema":schema}
def probe(name, valid, invalid, tolerated=None):
    p={"document":"closed","export":name,"valid":valid,"invalid":invalid}
    if tolerated: p["tolerated"]=tolerated
    return p
probes=[
 probe("Strict",[{"a":"x"},{"a":"x","b":1},{"a":"","b":-3}],[{},{"b":1},{"a":"x","b":"1"},{"a":"x","extra":1},{"a":"x","b":1,"extra":NULL},{"a":"x","A":"y"},[],NULL,1]),
 probe("Loose",[{"a":"x"}],[{},{"a":1},NULL],[{"a":"x","extra":1},{"a":"x","extra":{"deep":[1]}}]),
 probe("Free",[{"a":"x"}],[{},{"a":1}],[{"a":"x","extra":1}]),
 probe("Empty",[{}],[{"x":1},{"a":NULL},[],NULL,"x"]),
 probe("Nest",[{"inner":{"n":1}},{"inner":{"n":1},"list":[{"k":"a"},{}],"loose":{"z":"y"}}],
   [{"inner":{"n":1,"extra":1}},{"inner":{"n":1},"extra":1},{"inner":{"n":1},"list":[{"k":"a","extra":1}]},{"inner":{"n":1},"list":[{"extra":1}]},{"inner":{"n":1},"loose":{"z":1}},{"inner":{}}],
   [{"inner":{"n":1},"loose":{"z":"y","extra":1}},{"inner":{"n":1},"loose":{"extra":1}}]),
 probe("MaybeRef",[NULL,{"a":"x"}],[{"a":"x","extra":1},{},"x"]),
 probe("MaybeInline",[NULL,{},{"q":"x"}],[{"q":"x","extra":1},{"extra":1},{"q":1},"x"]),
 probe("Dict",[{},{"k":{"v":1}},{"k":{"v":1},"other":{"v":2}}],[{"k":{"v":1,"extra":1}},{"k":{}},{"k":1},{"k":NULL}]),
 probe("Pair",[[{"a":"x"},"y"]],[[{"a":"x","extra":1},"y"],[{},"y"],[{"a":"x"}],[{"a":"x"},"y","z"],[{"a":"x"},1]]),
 probe("Tagged",[{"kind":"n","n":1},{"kind":"s"},{"kind":"s","s":"x"}],[{"kind":"n","n":1,"extra":1},{"kind":"n"},{"kind":"n","n":"1"},{"kind":"x"},{"n":1},{"kind":"s","s":1}],[{"kind":"s","s":"x","extra":1}]),
 probe("External",[{"A":1},{"B":"x"}],[{"A":1,"B":"x"},{"A":1,"extra":1},{"B":"x","extra":1},{},{"C":1},{"A":"1"},{"B":1}]),
 probe("Holder",[{"strict":{"a":"x"},"loose":{"a":"y"}}],[{"strict":{"a":"x","extra":1},"loose":{"a":"y"}},{"strict":{"a":"x"},"loose":{"a":"y"},"extra":1},{"strict":{"a":"x"}}],[{"strict":{"a":"x"},"loose":{"a":"y","extra":1}}]),
]
raw=[
 {"document":"closed","export":"Strict","text":"{\"a\": \"x\", \"__proto__\": {}}","valid":False},
 {"document":"closed","export":"Strict","text":"{\"__proto__\": 1, \"a\": \"x\"}","valid":False},
 {"document":"closed","export":"Loose","text":"{\"a\": \"x\", \"__proto__\": {}}","valid":True},
 {"document":"closed","export":"Empty","text":"{\"__proto__\": {}}","valid":False},
]
c["cases"].append({"name":"closed","documents":[doc],"probes":probes,"rawProbes":raw})
open(path,"w",encoding="utf8").write(json.dumps(c,indent=2,ensure_ascii=False)+"\n")
print("ok")
