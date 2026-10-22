#!/usr/bin/env python3
"""Writes the declared inference protocol (schema const) from the rule table below. Run with --write to create/overwrite the schema file; without it prints the JSON."""
import json,os,sys
WS="/Users/ueli/Documents/semio"
LIB=WS+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DIR=LIB+"/🏗️build/🕸️graph/📥️inference/"+"\U0001F516️stamps"
SCHEMA=DIR+"/\U0001F9EC️schema/\U0001F523️.json"
I="🏗️build/🕸️graph/📥️inference/"
def inf(name): return I+name+"/"
STAMP=lambda i:"semio.graph-inferrer."+i+"/1"
inferrers=[
 ("coordinates","project",[inf("📍️coordinates")]),
 ("bootstrap","project",[inf("🔁️bootstrap")]),
 ("helpers","project",["🕸️dependencies/🟦️typescript/🟨️.mjs","🕸️dependencies/🧩️runtime/🟨️.mjs","🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs","🎮️playground/🧩️composition/🟨️.mjs","🎮️playground/🖥️native-host/","📇️catalog/🚚️deployment/","🪪️identity/📁️installation/"]),
 ("source-analyses","project",[inf("🧬️source-analyses"),"⚡️caching/📥️inference/"]),
 ("bun-lock","project",[inf("🧶️bun-lock")]),
 ("target-policy","project",[inf("🎯️target-policy"),"🏗️build/🎚️mode/","🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/","🏃️process/🧪️testing/🦀️cargo/🏃️execution/"]),
 ("native-manifests","native",[inf("🦀️native-manifests")]),
 ("script-inputs","project",[inf("🧭️script-inputs"),"🏗️build/📥️inputs/🟨️.mjs","🏗️build/📥️inputs/🧮️digest/🟨️.mjs","🏗️build/📥️inputs/🧮️digest/🔣️schema.json","🏗️build/📥️inputs/🔒️lock/","🏗️build/📤️arguments/"]),
 ("generator-outputs","project",[inf("🏭️generator-outputs")]),
 ("component-targets","project",[inf("🧩️component-targets")]),
 ("playground-targets","project",[inf("🎮️playground-targets")]),
 ("project-assembly","project",[inf("🧱️project-assembly")]),
 ("owner-discovery","project",[inf("🔎️owner-discovery")]),
 ("project-nodes","project",[inf("📦️project-nodes")]),
 ("dependency-edges","environment",[inf("🕸️dependency-edges")]),
 ("observation","all",["🏗️build/👁️observation/","🏗️build/📥️inputs/📋️data/","🏗️build/📥️inputs/🧮️digest/📥️read/","⚡️caching/🗃️records/","🧬️schema/✅️validator/"]),
 ("graph-assembly","all",["🏗️build/🕸️graph/🟦️.ts","🏗️build/🕸️graph/🚪️admission/","🏗️build/🕸️graph/🧭️file-owner/","🏗️build/🧭️selection/","🏗️build/🧬️schema/"]),
]
none=["🏗️build/📥️inputs/💾️storage/","⚡️caching/💾️storage/","🏗️build/📥️source/🔏️closure/","🏗️build/📥️inputs/🔏️revision/","🏗️build/📥️inputs/🦀️resolution/",inf("♻️revision"),inf("🚦️invocation"),inf("🧰️internals"),inf("🪢️revision-graph"),inf("🗂️components"),I+"🟨️.mjs",inf("🔖️stamps"),"🏗️build/🕸️graph/🕰️generations/","🏗️build/🕸️graph/🗂️generation-files/","🏗️build/🕸️graph/🔁retry/"]
protocol={"version":1,"scopes":["project","native","environment","all"],
 "inferrers":{i:{"stamp":STAMP(i),"scope":s,"sources":src} for i,s,src in inferrers},"none":none}
schema={
 "$schema":"http://json-schema.org/draft-07/schema#",
 "$id":"semio://repo/build/inference-stamps/v1",
 "title":"Graph inference protocol stamps",
 "description":"The declared, versioned protocol of the graph inference. Every inferrer carries one stamp; a record or generation fact names the stamp of each inferrer that took part in its output (the format of the persisted project record carries its own stamp in the observation schema), never the bytes of the code that ran. Bump the stamp of an inferrer when the output of its code changes for the same inputs: the law over the output corpus fails until the stamp and the corpus digest move together. `scope` says who consumes the stamp: `project` every valid project record, `native` the projects with a native target, `environment` only the cross-project edges, `all` every record and the environment. `sources` lists the workspace path fragments of the code the stamp covers; `none` lists the fragments of infrastructure that never changes an output. Data files that match neither stay content facts.",
 "definitions":{
  "Protocol":{"const":protocol},
  "Id":{"type":"string","pattern":"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"},
  "Stamp":{"type":"string","pattern":"^semio\\.graph-inferrer\\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*/[1-9][0-9]*$"},
  "Fragments":{"type":"array","minItems":1,"uniqueItems":True,"items":{"type":"string","minLength":2,"maxLength":200}},
  "Inferrer":{"type":"object","additionalProperties":False,"required":["stamp","scope","sources"],"properties":{"stamp":{"$ref":"#/definitions/Stamp"},"scope":{"enum":["project","native","environment","all"]},"sources":{"$ref":"#/definitions/Fragments"}}},
  "Document":{"type":"object","additionalProperties":False,"required":["version","scopes","inferrers","none"],"properties":{"version":{"const":1},"scopes":{"type":"array","items":{"enum":["project","native","environment","all"]},"uniqueItems":True},"inferrers":{"type":"object","minProperties":1,"propertyNames":{"$ref":"#/definitions/Id"},"additionalProperties":{"$ref":"#/definitions/Inferrer"}},"none":{"$ref":"#/definitions/Fragments"}}},
 },
}
text=json.dumps(schema,ensure_ascii=False,indent=1)+"\n"
if "--write" in sys.argv:
    os.makedirs(os.path.dirname(SCHEMA),exist_ok=True)
    open(SCHEMA,"w",encoding="utf8").write(text);print("written",SCHEMA)
else: print(text)
