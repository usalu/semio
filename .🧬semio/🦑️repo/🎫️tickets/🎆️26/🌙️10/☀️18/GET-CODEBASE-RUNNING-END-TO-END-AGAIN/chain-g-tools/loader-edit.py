#!/usr/bin/env python3
"""Applies (apply) or reverses (revert) the chain-g loader edit on the live 🍞️bun/🟦️.ts; refuses when an anchor is missing or ambiguous."""
import glob,sys
F=glob.glob("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts")[0]
OLD=[
"physicalSelectionText=JSON.stringify({version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))}),resolutionSelectionText=JSON.stringify({version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))});",
"'import.meta.semioCapturedPhysicalSources':physicalSelectionText,'import.meta.semioCapturedSourceResolutions':resolutionSelectionText,",
"images.set(name,bytes);resolutions.set(name,row.imports);}",
"recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();}});check();",
]
NEW=[
"freezeSelection=item=>{if(item&&typeof item==='object'&&!Object.isFrozen(item)){Object.freeze(item);for(const member of Object.values(item))freezeSelection(member);}return item;};Object.defineProperty(globalThis,'__semioCapturedSelectionV1',{value:freezeSelection({physical:{version:1,root:value.root,sources:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256}))},resolutions:{version:1,root:value.root,files:orderedSourceFiles.map(row=>({path:row.path,sha256:row.sha256,imports:row.imports}))}}),enumerable:false,writable:false,configurable:false});",
"'import.meta.semioCapturedPhysicalSources':'globalThis.__semioCapturedSelectionV1.physical','import.meta.semioCapturedSourceResolutions':'globalThis.__semioCapturedSelectionV1.resolutions',",
"images.set(name,bytes);resolutions.set(name,row.imports);row.base64='';}",
"recheck:()=>{for(const[name,bytes]of images)currentImage(name,bytes);check();}${auxiliary?\",text:()=>encoded.toString('utf8')\":\"\"}});check();",
]
mode=sys.argv[1]
src,dst=(OLD,NEW) if mode=="apply" else (NEW,OLD)
s=open(F,encoding="utf8").read()
for a,b in zip(src,dst):
    if s.count(a)!=1: sys.exit("anchor missing or ambiguous: "+a[:70])
for a,b in zip(src,dst): s=s.replace(a,b)
open(F,"w",encoding="utf8").write(s)
print(mode,"ok")
