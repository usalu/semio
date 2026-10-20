#!/usr/bin/env python3
"""Renders src/* (emoji path tokens, docstring emoji normalised) into staged/ with a manifest of destinations and hashes; fails on unresolved tokens, a docstring without a leading emoji, a docstring emoji used twice inside the module (live incremental module included) and any path over 255 characters."""
import json,os,re,sys,hashlib
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
M=names.M;V=names.V;n=names.n
dest={
 "schema.json":f"{M}/{names.SCHEMA}/{names.JSON}","workloads.json":f"{M}/{names.WORKLOADS}/{names.JSON}","model.ts":f"{M}/{names.TS}",
 "runner.ts":f"{M}/{names.RUNNER}/{names.TS}","command.ts":f"{M}/{names.COMMAND}/{names.TS}",
 "test.ts":f"{M}/{names.TESTS}/{names.TS}","fixture.json":f"{M}/{names.TESTS}/{names.FIXTURES}/{names.JSON}","oracle.py":f"{M}/{names.TESTS}/{names.ORACLE}/{names.PY}","stub.ts":f"{M}/{names.TESTS}/{names.STUB}/{names.TS}",
}
tokens=dict(names.TOKENS);out=os.path.join(here,"staged");os.makedirs(out,exist_ok=True)
doc=re.compile(r"/\*\* (\S)")
def leading(text):
    first=re.compile(r"/\*\* (\S+)").finditer(text);res=[]
    for m in first:
        ch=m.group(1)
        if ord(ch[0])<0x2000: raise SystemExit(f"docstring must start with an emoji: {text[m.start():m.start()+50]!r}")
        res.append(ch[0] if ord(ch[0])>0xFFFF or True else ch)
    return res
def normalise(text):
    parts=[];last=0
    for m in doc.finditer(text):
        parts.append(text[last:m.end()]);last=m.end()
        if text[m.end():m.end()+1]!=V: parts.append(V)
    parts.append(text[last:]);return "".join(parts)
def emoji_of(text):
    found=[]
    for m in re.finditer(r"/\*\* (\S)",text):
        ch=m.group(1)
        if ord(ch)<0x2000: raise SystemExit(f"docstring must start with an emoji: {text[m.start():m.start()+50]!r}")
        found.append(ch)
    return found
live=set()
liveDir=os.path.join(names.R,names.I)
for base,_,files in os.walk(liveDir):
    if names.CONC in base: continue
    for f in files:
        if f.endswith(".ts"):
            for ch in emoji_of(open(os.path.join(base,f),encoding="utf8").read()): live.add(ch)
manifest={};used={}
for src,d in dest.items():
    t=open(os.path.join(here,"src",src),encoding="utf8").read()
    if "@@" in t: t=re.sub(r"@@([A-Z0-9_]+)@@",lambda m:tokens[m.group(1)],t)
    if re.search(r"@@[A-Z]",t): raise SystemExit(f"{src}: unresolved token")
    if src.endswith(".ts"):
        t=normalise(t)
        for ch in emoji_of(t): used.setdefault(ch,[]).append(src)
    open(os.path.join(out,src),"w",encoding="utf8").write(t)
    manifest[src]={"destination":d,"sha256":hashlib.sha256(t.encode()).hexdigest(),"length":len(os.path.join(names.R,d))}
json.dump(manifest,open(os.path.join(out,"manifest.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
dups={k:v for k,v in used.items() if len(v)>1};clash={k:v for k,v in used.items() if k in live}
long=[(f,m["length"]) for f,m in manifest.items() if m["length"]>255]
print("rendered",len(manifest),"files; longest destination",max(m["length"] for m in manifest.values()),"chars; over 255:",long)
print("docstring emoji used more than once in the set:",{k:len(v) for k,v in dups.items()})
print("docstring emoji already used by the live incremental module:",{k:v for k,v in clash.items()})
if dups or clash or long: sys.exit(1)
