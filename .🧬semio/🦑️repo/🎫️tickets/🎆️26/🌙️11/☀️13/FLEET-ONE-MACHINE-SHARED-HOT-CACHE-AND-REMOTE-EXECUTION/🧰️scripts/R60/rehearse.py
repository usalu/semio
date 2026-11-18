#!/usr/bin/env python3
"""R60 rehearsal: runs the staged files and their staged tests outside the worktree. The staged files keep their relative layout under `run/`; every import that leaves the staged set is rewritten to the absolute path of the live tree, imports between staged files stay relative. Usage: rehearse.py <scratch> <group> [test-file-name-filter]  (group: nodes | lru | present | wave)"""
import os,re,sys,shutil,subprocess
R="/Users/ueli/Documents/semio"
L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
PN=B+"🕸️graph/📥️inference/📦️project-nodes/"
MB=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/"
scratch=sys.argv[1];group=sys.argv[2];flt=sys.argv[3] if len(sys.argv)>3 else ""
work=os.path.join(scratch,"work");run=os.path.join(scratch,"run",group)
shutil.rmtree(run,ignore_errors=True)
assert os.path.realpath(run).startswith(os.path.realpath(scratch))
GROUPS={
 "present":(PN,["📋️present/"],PN+"📋️present/🧪️tests",None),
 "wave":(PN,["📋️present/","🟨️.mjs","🧪️tests/🐌️slow-last/"],PN+"🧪️tests/🐌️slow-last",None),
"nodes":(PN,["📋️present/","🧪️tests/🛟️stopped-wave/"],PN+"🧪️tests/📈️inferred","🧪️tests/🛟️stopped-wave/🟦️.ts"),
 "records":(B,["🧪️tests/🕸️graph/🛟️records-first/"],B+"🧪️tests/🕸️graph/🧊️optional-cache",None),
 "lru":(MB,["🔎️scan/","🧪️tests/♻️lru/"],MB+"🧪️tests/♻️lru",None),
}
base,prefixes,cwd,entry=GROUPS[group]
rel_of=lambda live:live[len(L):]
workroot=work+"/"+(rel_of(base))
staged=[]
for dp,dn,fn in os.walk(workroot):
    for f in fn:
        p=os.path.join(dp,f);rel=p[len(workroot):]
        if any(rel.startswith(x) for x in prefixes):staged.append(rel)
stagedset=set(staged)
def resolve(origin_dir,spec):
    target=os.path.normpath(os.path.join(origin_dir,spec))
    if target.startswith(base):
        r=target[len(base):]
        if r in stagedset or any(s.startswith(r.rstrip("/")+"/") for s in stagedset):return None
    return target
def fix(text,origin_dir):
    def sub(m):
        spec=m.group(2)
        if not(spec.startswith("../") or spec.startswith("./")):return m.group(0)
        t=resolve(origin_dir,spec)
        return m.group(0) if t is None else m.group(1)+t+m.group(3)
    text=re.sub(r'((?:from|import)\s*\(?\s*")([^"]+)(")',sub,text)
    text=re.sub(r'(new URL\(")([^"]+)(", ?import\.meta\.url)',lambda m:m.group(0) if not m.group(2).startswith(("../","./")) or resolve(origin_dir,m.group(2)) is None else m.group(1)+resolve(origin_dir,m.group(2))+m.group(3),text)
    text=text.replace('const B="../../../../../../"','const B="'+B+'"')
    text=re.sub(r'new URL\(B\+"([^"]+)",import\.meta\.url\)',lambda m:'new URL("'+os.path.normpath(B+m.group(1))+'","file:///")',text)
    return text
for rel in staged:
    src=os.path.join(workroot,rel.lstrip("/"))
    if os.environ.get("R60_BASE")=="1" and "scan" in rel and os.path.exists(os.path.join(scratch,"base",rel_of(base)+rel.lstrip("/"))):src=os.path.join(scratch,"base",rel_of(base)+rel.lstrip("/"))
    out=os.path.join(run,rel.lstrip("/"))
    os.makedirs(os.path.dirname(out),exist_ok=True)
    origin_dir=os.path.dirname(base+rel.lstrip("/"))
    if src.endswith((".ts",".mjs",".mts")):open(out,"w",encoding="utf8").write(fix(open(src,encoding="utf8").read(),origin_dir))
    else:shutil.copy(src,out)
art=os.path.join(scratch,"art");os.makedirs(art,exist_ok=True)
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["SEMIO_TEST_ARTIFACT_DIR"]=art;env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
tests=[os.path.join(run,e) for e in ([entry] if entry else [])]
if not tests:
    for rel in staged:
        if "/🧪️tests/" in "/"+rel and rel.endswith("🟦️.ts") and "🧒️child" not in rel:tests.append(os.path.join(run,rel))
tests=[t for t in tests if os.path.exists(t)]
code=0
for t in tests:
    args=[BUN,"test",t,"--timeout","600000"]+(["-t",flt] if flt else [])
    print("==",t[len(run):],flush=True)
    code=subprocess.run(args,cwd=cwd if os.path.isdir(cwd) else os.path.dirname(t),env=env).returncode or code
sys.exit(code)
