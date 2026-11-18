#!/usr/bin/env python3
"""R77 stage builder: reads the live files, applies counted literal edits, writes <id>.base/<id>.new/index.txt into ./stage (new files have kind "new" and no .base). Re-run it to restage from the current live tree."""
import os,shutil,sys
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
CL=L+"🏗️build/📥️source/🔏️closure/"
B=L+"🏗️build/"
AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def stage(rel,edits=None,new=None):
    i="f%d"%n[0];n[0]+=1
    if new is not None:
        open(H+i+".new","w",encoding="utf8").write(new);rows.append("%s %s new"%(i,rel));return
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:80]))
        text=text.replace(old,rep)
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))

RV=B+"📥️inputs/🔏️revision/"
GT=AU+"🧪️tests/📈️growth/🟦️.ts"
stage(RV+"🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',2),('"maximum": 2048\n','"maximum": 16384\n',2)])
stage(RV+"🧪️tests/🔣️.json",[('"sources": 2048,','"sources": 16384,',1),('"sourceCount": 2048\n','"sourceCount": 16384\n',1)])
stage(B+"🔧️tools/🏗️bazel/🎬️scopes/🧬️schema/🔣️.json",[('"maxItems": 2048,','"maxItems": 16384,',3)])
stage(GT,[
 ('import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";','import closureSchema from"../../../../../../🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json";\nimport revisionSchema from"../../../../../../🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json";',1),
 (' const diagnostic=schema.$defs.SourceCapacityDiagnostic.properties;',' const revision=revisionSchema.definitions;\n expect(revision.CapturedPhysicalSources.properties.sources.maxItems).toBe(policy.maximumFiles);\n expect(revision.CapturedPhysicalSourceImages.properties.images.maxItems).toBe(policy.maximumFiles);\n const diagnostic=schema.$defs.SourceCapacityDiagnostic.properties;',1),
])
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print(len(rows),"files staged")
