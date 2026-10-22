#!/usr/bin/env python3
"""Renders src/* into staged/ (flat) with a manifest of destinations. Tokens @@NAME@@ become emoji path parts; @@UP@@ is the relative path up to the build directory, @@PROC@@ and @@VALID@@ the relative paths to the process framework and the schema validator. Fails on unresolved tokens and on over-long destinations."""
import hashlib,json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
from names import B,T,TS,JSON,PY,MJS,V,p
def D(*parts): return p(B,*[T[x] if x in T else x for x in parts[:-1]],parts[-1]) if False else None
def d(*parts):
    out=[]
    for x in parts:
        out.append(T[x] if x in T else x)
    return p(B,*out)
bins=lambda *x:d("TOOLS",*x)
dest={
 "backends.schema.json":d("BACKENDS","SCHEMA",JSON),"backends.registry.json":d("BACKENDS","REGISTRY",JSON),"backends.ts":d("BACKENDS",TS),
 "taskmodel.ts":d("TASKMODEL",TS),
 "semio.schema.json":d("TOOLS","SEMIO","SCHEMA",JSON),"semio.ts":d("TOOLS","SEMIO",TS),"semio_store.ts":d("TOOLS","SEMIO","STORE",TS),"semio_remote.ts":d("TOOLS","SEMIO","REMOTE",TS),
 "semio_exec.ts":d("TOOLS","SEMIO","EXEC",TS),"semio_scheduler.ts":d("TOOLS","SEMIO","SCHED",TS),"semio_driver.ts":d("TOOLS","SEMIO","CONF",TS),
 "conformance.schema.json":d("CONF","SCHEMA",JSON),"conformance.scenarios.json":d("CONF","SCENARIOS",JSON),"stub.ts":d("CONF","STUB",TS),"conformance.ts":d("CONF",TS),"conformance_runner.ts":d("CONF","RUNNER",TS),
}
extra=os.path.join(here,"dest-extra.py")
if os.path.exists(extra): exec(open(extra,encoding="utf8").read())
def depth(destination):
    rel=os.path.dirname(destination)[len(B)+1:]
    return len(rel.split("/")) if rel else 0
doc=re.compile(r"/\*\* (\S)")
def normalise(text,name):
    for m in doc.finditer(text):
        if ord(m.group(1)[0])<0x2000: raise SystemExit(f"{name}: docstring must start with an emoji: {text[m.start():m.start()+40]!r}")
    res=[];last=0
    for m in doc.finditer(text):
        res.append(text[last:m.end()]);last=m.end()
        if text[m.end():m.end()+1]!=V: res.append(V)
    res.append(text[last:]);return "".join(res)
out=os.path.join(here,"staged");os.makedirs(out,exist_ok=True)
manifest={};seen={}
for src,dst in dest.items():
    t=open(os.path.join(here,"src",src),encoding="utf8").read();k=depth(dst)
    tokens=dict(names.TOKENS);up="../"*k;fw="../"*(k+5)
    tokens["UP"]=up;tokens["PROC"]=fw+"🔨️modules/🏃️process/📦️packages/🟦️typescript".replace("🔨️","🔨"+V).replace("🏃️","🏃"+V).replace("📦️","📦"+V).replace("🟦️","🟦"+V)
    tokens["VALID"]=fw+f"{T['MODULES'] if False else names.n(0x1F528,'modules')}/{names.n(0x1F9EC,'schema')}/{names.n(0x2705,'validator')}/{names.n(0x1F4E6,'packages')}/{names.n(0x1F7E6,'typescript')}/{TS}"
    tokens["PROC"]=fw+f"{names.n(0x1F528,'modules')}/{names.n(0x1F3C3,'process')}/{names.n(0x1F4E6,'packages')}/{names.n(0x1F7E6,'typescript')}"
    tokens["RUNTIME"]=fw+f"{names.n(0x1F528,'modules')}/{names.n(0x2699,'runtime')}"
    tokens["PROCROOT"]=fw+f"{names.n(0x1F528,'modules')}/{names.n(0x1F3C3,'process')}"
    t=re.sub(r"@@([A-Z0-9_]+)@@",lambda m:tokens[m.group(1)] if m.group(1) in tokens else m.group(0),t)
    if re.search(r"@@[A-Z]",t): raise SystemExit(f"{src}: unresolved token {re.search(r'@@[A-Z0-9_]+@@',t).group(0)}")
    if src.endswith(".ts") or src.endswith(".mjs"):
        t=normalise(t,src)
        for m in re.finditer(r"/\*\* (\S)",t): seen.setdefault(m.group(1),[]).append(src)
    open(os.path.join(out,src),"w",encoding="utf8").write(t)
    manifest[src]={"destination":dst,"sha256":hashlib.sha256(t.encode()).hexdigest(),"length":len(os.path.join(names.R,dst))}
json.dump(manifest,open(os.path.join(out,"manifest.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
long=[(f,m["length"]) for f,m in manifest.items() if m["length"]>255]
print("rendered",len(manifest),"files; longest",max(m["length"] for m in manifest.values()),"; over 255:",long)
print("docstring emoji used more than once:",{k:v for k,v in seen.items() if len(v)>1 and len(set(v))>=1 and len(v)>1})
