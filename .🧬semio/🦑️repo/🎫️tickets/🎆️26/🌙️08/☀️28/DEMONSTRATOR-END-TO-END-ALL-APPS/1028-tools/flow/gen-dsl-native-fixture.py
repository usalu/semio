import json,sys,os
root=sys.argv[1]
def cam(x=0.0,y=0.0,z=1.0): return {"x":x,"y":y,"zoom":z}
def syn(i,a,b,fp="out",tp="in"): return {"id":i,"from":a,"to":b,"fromPort":fp,"toPort":tp}
def snap(widgets=(),synapses=(),layout=None,camera=None):
    return {"schema":"flow.host_snapshot","camera":camera or cam(),"widgets":list(widgets),"synapses":list(synapses),"layout":layout or {}}
def neuron(i,k,params=None,tree=None):
    n={"id":i,"kind":k,"params":params or {}}
    if tree is not None: n["tree"]=tree
    return n
def tree(ns=(),ss=()): return {"neurons":list(ns),"synapses":list(ss)}
nested=tree([neuron("outer","cluster",{},tree([neuron("middle","cluster",{},tree([neuron("inner","math.add",{"a":1,"b":2.5})]))]))])
flowui=lambda: {"camera":cam(10.0,-4.5,0.5),"nodes":{"outer":{"layout":{"x":1.0,"y":2.0},"chrome":{"kind":"plain","preview":True}}},"previews":[{"id":"p","source":{"neuron":"outer","channel":"out"},"mode":"text","preview":{"k":"v"},"expanded":["k"],"layout":{"x":3.0,"y":4.0}}]}
kinds=[
 {"kind":"neuron","id":"n","neuronKind":"math.add","params":{"a":1,"b":2.5,"flag":True,"nothing":None,"label":"x","dict":{"deep":{"deeper":3}}},"inputPorts":["a","b"],"outputPorts":["sum"],"preview":False},
 {"kind":"inputSlider","id":"s","label":"Number","value":3.0,"min":0.0,"max":10.0,"step":0.5},
 {"kind":"inputNote","id":"note","text":"line one\nline \"two\" \\ tab\t"},
 {"kind":"inputImage","id":"img","src":"data:image/png;base64,AAAA"},
 {"kind":"variable","id":"v","name":"x","schema":"flow.number"},
 {"kind":"outputPreview","id":"pv","preview":{"text":"hi","n":2},"expanded":["text","n"]},
 {"kind":"outputAction","id":"act","action":"export"},
 {"kind":"outputExport","id":"ex","format":"json"},
 {"kind":"cluster","id":"cl","name":"Cluster","tree":tree([neuron("a","math.add",{"x":1}),neuron("b","math.mul")],[syn("t1","a","b","sum","a")]),"flow":flowui()},
]
big_w=[{"kind":"inputNote","id":f"note-{i:03}","text":f"note {i}"} for i in range(120)]
big_s=[syn(f"s{i:03}",f"note-{i:03}",f"note-{i+1:03}") for i in range(119)]
big_l={f"note-{i:03}":{"x":float(i*10),"y":float(-i)} for i in range(120)}
cases=[
 {"id":"empty-document","description":"no widgets, synapses or layout","snapshot":snap()},
 {"id":"default-three-widget-chain","description":"slider -> add -> preview","snapshot":snap([kinds[1],{"kind":"neuron","id":"add","neuronKind":"math.add","params":{},"inputPorts":[],"outputPorts":[],"preview":True},{"kind":"outputPreview","id":"preview","preview":{},"expanded":[]}],[syn("s1","s","add","number","a"),syn("s2","add","preview","sum","")],{"s":{"x":0.0,"y":0.0}})},
 {"id":"every-widget-kind","description":"one widget of each of the nine kinds","snapshot":snap(kinds,[syn("e1","s","n","number","a")],{"s":{"x":-5.5,"y":2.0},"n":{"x":40.0,"y":2.0}},cam(12.5,-3.0,2.0))},
 {"id":"nested-cluster-depth-3","description":"cluster tree whose neuron trees nest three levels","snapshot":snap([{"kind":"cluster","id":"deep","name":"Deep","tree":nested,"flow":flowui()}])},
 {"id":"unicode-identifiers","description":"non-ASCII ids, labels and note text","snapshot":snap([{"kind":"inputNote","id":"notiz-ä","text":"Grüße 世界 🌍"},{"kind":"inputSlider","id":"schieber-ö","label":"Größe","value":1.0,"min":0.0,"max":2.0,"step":0.25}],[syn("verbindung-ß","notiz-ä","schieber-ö","aus","ein")],{"notiz-ä":{"x":1.0,"y":1.0}})},
 {"id":"large-layout","description":"120 note widgets chained with 119 synapses and 120 layout entries","snapshot":snap(big_w,big_s,big_l)},
]
schema={
 "$schema":"https://json-schema.org/draft/2020-12/schema","$id":"semio:flow/dsl-native-snapshot","title":"FlowHostSnapshot canonical JSON projection","type":"object","additionalProperties":False,
 "required":["schema","camera","widgets","synapses","layout"],
 "properties":{"schema":{"const":"flow.host_snapshot"},"camera":{"$ref":"#/$defs/camera"},"widgets":{"type":"array","items":{"$ref":"#/$defs/widget"}},"synapses":{"type":"array","items":{"$ref":"#/$defs/synapseSpec"}},"layout":{"type":"object","additionalProperties":{"$ref":"#/$defs/point"}}},
 "$defs":{
  "num":{"type":"number"},
  "point":{"type":"object","additionalProperties":False,"required":["x","y"],"properties":{"x":{"type":"number"},"y":{"type":"number"}}},
  "camera":{"type":"object","additionalProperties":False,"required":["x","y","zoom"],"properties":{"x":{"type":"number"},"y":{"type":"number"},"zoom":{"type":"number"}}},
  "value":{"anyOf":[{"type":["null","boolean","integer","number","string"]},{"$ref":"#/$defs/dictionary"}]},
  "dictionary":{"type":"object","additionalProperties":{"$ref":"#/$defs/value"}},
  "strings":{"type":"array","items":{"type":"string"},"uniqueItems":True},
  "synapseSpec":{"type":"object","additionalProperties":False,"required":["id","from","to","fromPort","toPort"],"properties":{k:{"type":"string"} for k in["id","from","to","fromPort","toPort"]}},
  "tree":{"type":"object","additionalProperties":False,"required":["neurons","synapses"],"properties":{"neurons":{"type":"array","items":{"$ref":"#/$defs/neuron"}},"synapses":{"type":"array","items":{"$ref":"#/$defs/synapseSpec"}}}},
  "neuron":{"type":"object","additionalProperties":False,"required":["id","kind"],"properties":{"id":{"type":"string"},"kind":{"type":"string"},"params":{"$ref":"#/$defs/dictionary"},"tree":{"$ref":"#/$defs/tree"}}},
  "chrome":{"oneOf":[
    {"type":"object","additionalProperties":False,"required":["kind"],"properties":{"kind":{"const":"plain"},"preview":{"type":"boolean"}}},
    {"type":"object","additionalProperties":False,"required":["kind","label","min","max","step"],"properties":{"kind":{"const":"slider"},"label":{"type":"string"},"min":{"type":"number"},"max":{"type":"number"},"step":{"type":"number"},"value":{"type":"number"}}},
    {"type":"object","additionalProperties":False,"required":["kind"],"properties":{"kind":{"const":"note"},"text":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind"],"properties":{"kind":{"const":"image"},"src":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","name","schema"],"properties":{"kind":{"const":"variable"},"name":{"type":"string"},"schema":{"type":"string"}}}]},
  "flowUi":{"type":"object","additionalProperties":False,"required":["camera","nodes"],"properties":{"camera":{"$ref":"#/$defs/camera"},"nodes":{"type":"object","additionalProperties":{"type":"object","additionalProperties":False,"required":["layout","chrome"],"properties":{"layout":{"$ref":"#/$defs/point"},"chrome":{"$ref":"#/$defs/chrome"}}}},"previews":{"type":"array","items":{"type":"object","additionalProperties":False,"required":["id","mode"],"properties":{"id":{"type":"string"},"source":{"oneOf":[{"type":"null"},{"type":"object","additionalProperties":False,"required":["neuron","channel"],"properties":{"neuron":{"type":"string"},"channel":{"type":"string"}}}]},"mode":{"type":"string"},"preview":{"$ref":"#/$defs/dictionary"},"expanded":{"$ref":"#/$defs/strings"},"layout":{"oneOf":[{"type":"null"},{"$ref":"#/$defs/point"}]}}}}}},
  "widget":{"oneOf":[
    {"type":"object","additionalProperties":False,"required":["kind","id","neuronKind"],"properties":{"kind":{"const":"neuron"},"id":{"type":"string"},"neuronKind":{"type":"string"},"params":{"$ref":"#/$defs/dictionary"},"inputPorts":{"type":"array","items":{"type":"string"}},"outputPorts":{"type":"array","items":{"type":"string"}},"preview":{"type":"boolean"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id","label"],"properties":{"kind":{"const":"inputSlider"},"id":{"type":"string"},"label":{"type":"string"},"value":{"type":"number"},"min":{"type":"number"},"max":{"type":"number"},"step":{"type":"number"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"inputNote"},"id":{"type":"string"},"text":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"inputImage"},"id":{"type":"string"},"src":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"variable"},"id":{"type":"string"},"name":{"type":"string"},"schema":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"outputPreview"},"id":{"type":"string"},"preview":{"$ref":"#/$defs/dictionary"},"expanded":{"$ref":"#/$defs/strings"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"outputAction"},"id":{"type":"string"},"action":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id"],"properties":{"kind":{"const":"outputExport"},"id":{"type":"string"},"format":{"type":"string"}}},
    {"type":"object","additionalProperties":False,"required":["kind","id","tree"],"properties":{"kind":{"const":"cluster"},"id":{"type":"string"},"name":{"type":"string"},"tree":{"$ref":"#/$defs/tree"},"flow":{"$ref":"#/$defs/flowUi"}}}]}
 }}
os.makedirs(root,exist_ok=True)
json.dump({"schema":"semio.flow.dsl-native-snapshot-law/v1","note":"Canonical JSON projection of FlowHostSnapshot; the DSL text (flow.flow envelope) round-trip is checked through ArtifactDsl::print_dsl/parse_dsl and must equal the JSON projection.","cases":cases},open(root+"/🔣️.json","w"),ensure_ascii=False,indent=1); open(root+"/🔣️.json","a").write("\n")
json.dump(schema,open(root+"/schema.json","w"),ensure_ascii=False,indent=1); open(root+"/schema.json","a").write("\n")
print(len(cases))
