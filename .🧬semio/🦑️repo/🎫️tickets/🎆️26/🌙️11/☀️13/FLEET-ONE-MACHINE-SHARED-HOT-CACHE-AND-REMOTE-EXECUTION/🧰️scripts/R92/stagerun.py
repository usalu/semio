#!/usr/bin/env python3
"""R92: runs staged test files against the live tree without touching it: every staged file is copied to RUN/f<i>.<ext> with its relative imports made absolute (to the live tree, or to another staged copy). usage: stagerun.py STAGE RUN test-index..."""
import os,re,sys,subprocess,shutil
R="/Users/ueli/Documents/semio"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
S,RUN=sys.argv[1],sys.argv[2];ids=sys.argv[3:]
rows=[l.split(" ") for l in open(S+"/index.txt",encoding="utf8").read().splitlines() if l]
byrel={rel:i for i,rel,kind in rows}
shutil.rmtree(RUN,ignore_errors=True);os.makedirs(RUN)
def ext(rel):return os.path.splitext(rel)[1] if not rel.endswith(".d.mts") else ".d.mts"
def target(live_dir,spec):
    t=os.path.normpath(os.path.join(live_dir,spec));rel=os.path.relpath(t,R)
    return RUN+"/f"+byrel[rel]+ext(rel) if rel in byrel else t
pat=re.compile(r'''((?:from|import)\s*\(?\s*)(["'])(\.{1,2}/[^"']*)\2''')
for i,rel,kind in rows:
    if rel.endswith(".d.mts"):continue
    text=open(S+"/"+i+".new",encoding="utf8").read();ld=os.path.dirname(R+"/"+rel)
    text=pat.sub(lambda m:m.group(1)+m.group(2)+target(ld,m.group(3))+m.group(2),text)
    if os.environ.get("STRIP"):text=re.sub(r"^import\{libraryBootstrap\}from[^\n]*\n","",text,flags=re.M).replace(" await libraryBootstrap;\n"," (await import(\""+RUN+"/f0.mjs\")).wire({coordinates:{},sourceAnalyses:{prepareSourceAnalysesV1:()=>({})},revisionIdentity:{url:\"stage\"},register:a=>a,nativeManifests:{},bunLock:{}});\n")
    open(RUN+"/f"+i+ext(rel),"w",encoding="utf8").write(text)
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
for i in ids:
    f=[x for x in os.listdir(RUN) if x.startswith("f"+i+".")][0]
    r=subprocess.run([BUN,"test",RUN+"/"+f,"--timeout","180000"],cwd=R,env=env,capture_output=True,text=True)
    print((r.stdout+r.stderr)[-6000:]);print("EXIT",f,r.returncode)
