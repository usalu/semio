#!/usr/bin/env python3
"""Rehearsal overlay: writes the edited files of payload.py to a scratch dir and a bun preload that serves them under the live paths, so tests run against the edits without touching the tree. usage: overlay.py <scratch-dir>; prints the preload path."""
import os,sys,json
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import payload
R="/Users/Shared/semio";out=os.path.abspath(sys.argv[1]);os.makedirs(out,exist_ok=True)
files={};n=0
for rel in dict.fromkeys(e[0] for e in payload.EDITS):
    text=open(R+"/"+rel,encoding="utf8").read()
    for path,old,new in payload.EDITS:
        if path!=rel:continue
        if old!=new and new in text:continue
        assert text.count(old)==1,(rel,old[:60]);text=text.replace(old,new)
    n+=1;name="%d%s"%(n,".json" if rel.endswith(".json") else ".ts");open(out+"/"+name,"w",encoding="utf8").write(text);files[R+"/"+rel]=out+"/"+name
json.dump(files,open(out+"/map.json","w"))
import re
filt="^("+"|".join(re.escape(k) for k in files)+")$"
open(out+"/preload.ts","w").write('import{plugin}from"bun";import{readFileSync}from"node:fs";\nconst map=JSON.parse(readFileSync("%s/map.json","utf8")) as Record<string,string>;\nplugin({name:"r1-overlay",setup(build){build.onLoad({filter:new RegExp(%s)},args=>{const file=map[args.path]!;return file.endsWith(".json")?{exports:{default:JSON.parse(readFileSync(file,"utf8"))},loader:"object"}:{contents:readFileSync(file,"utf8"),loader:"ts"};});}});\n'%(out,json.dumps(filt)))
print(out+"/preload.ts")
