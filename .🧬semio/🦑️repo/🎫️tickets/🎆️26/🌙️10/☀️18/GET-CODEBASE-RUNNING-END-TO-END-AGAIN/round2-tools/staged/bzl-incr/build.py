#!/usr/bin/env python3
"""Renders src/* (emoji path tokens, docstring emoji normalised) into staged/ with a manifest of destinations; fails on unresolved tokens or duplicate docstring emoji."""
import json,os,re,sys,hashlib
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
M=names.M;V=names.V
dest={
 "schema.json":f"{M}/{names.SCHEMA}/{names.JSON}","scenarios.json":f"{M}/{names.SCENARIOS}/{names.JSON}","model.ts":f"{M}/{names.TS}",
 "observers.ts":f"{M}/{names.OBSERVERS}/{names.TS}","runner.ts":f"{M}/{names.RUNNER}/{names.TS}","command.ts":f"{M}/{names.COMMAND}/{names.TS}",
 "test.ts":f"{M}/{names.TESTS}/{names.TS}","fixture.json":f"{M}/{names.TESTS}/{names.FIXTURES}/{names.JSON}","oracle.py":f"{M}/{names.TESTS}/{names.n(0x1F40D,'oracle')}/{names.PY}",
}
tokens=dict(names.TOKENS)
out=os.path.join(here,"staged");os.makedirs(out,exist_ok=True)
manifest={};seen={}
doc=re.compile(r"/\*\* (\S)")
def normalise(text,name):
    def fix(m):
        ch=m.group(1)
        if ord(ch[0])>0x2000 and not text[m.end():m.end()+1]==V: pass
        return m.group(0)
    for m in doc.finditer(text):
        first=m.group(1)
        if ord(first)<0x2000: raise SystemExit(f"{name}: docstring must start with an emoji: {text[m.start():m.start()+40]!r}")
    def put(m):
        after=text[m.end():m.end()+1]
        return m.group(0)+("" if after==V else V)
    # insert U+FE0F after the leading emoji when missing
    res=[];last=0
    for m in doc.finditer(text):
        res.append(text[last:m.end()]);last=m.end()
        if text[m.end():m.end()+1]!=V: res.append(V)
    res.append(text[last:]);return "".join(res)
for src,d in dest.items():
    t=open(os.path.join(here,"src",src),encoding="utf8").read()
    t=re.sub(r"@@([A-Z0-9_]+)@@",lambda m:tokens[m.group(1)],t) if "@@" in t else t
    if re.search(r"@@[A-Z]",t): raise SystemExit(f"{src}: unresolved token")
    if src.endswith(".ts"):
        t=normalise(t,src)
        for m in re.finditer(r"/\*\* (\S)",t):
            e=m.group(1)
            if e in seen and seen[e]!=src+str(m.start()): pass
            seen.setdefault(e,[]).append(src) if isinstance(seen.get(e),list) else seen.__setitem__(e,[src])
    flat=src
    open(os.path.join(out,flat),"w",encoding="utf8").write(t)
    manifest[flat]={"destination":d,"sha256":hashlib.sha256(t.encode()).hexdigest(),"length":len(d)}
dups={k:v for k,v in seen.items() if len(v)>1}
json.dump(manifest,open(os.path.join(out,"manifest.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
long=[(f,len(os.path.join(names.R,m["destination"]))) for f,m in manifest.items() if len(os.path.join(names.R,m["destination"]))>255]
print("rendered",len(manifest),"files; longest destination",max(len(os.path.join(names.R,m['destination'])) for m in manifest.values()),"chars; over 255:",long)
print("docstring emoji used more than once:",{k:len(v) for k,v in dups.items()})
