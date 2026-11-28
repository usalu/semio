import os,sys,shutil
LIVE="/Users/ueli/Documents/semio";T=sys.argv[1];FARM=sys.argv[2]
skip={"node_modules","target",".cache","dist","⚡️cache","🗑️generated",".git","bazel-out"}
shutil.rmtree(T,ignore_errors=True);os.makedirs(T)
n=0
def link(src,dst):
    global n
    n+=1
    shutil.copy2(src,dst)
def walk(s,d):
    os.makedirs(d,exist_ok=True)
    for e in os.scandir(s):
        if e.name in skip: continue
        t=os.path.join(d,e.name)
        if e.is_symlink(): os.symlink(os.readlink(e.path),t)
        elif e.is_dir(follow_symlinks=False): walk(e.path,t)
        elif e.is_file(follow_symlinks=False): link(e.path,t)
walk(LIVE+"/🧰️framework",T+"/🧰️framework")
for f in ["package.json","bun.lock","tsconfig.json","MODULE.bazel","go.work","nx.json","📋️project.json"]:
    if os.path.exists(LIVE+"/"+f): shutil.copy2(LIVE+"/"+f,T+"/"+f)
os.symlink(LIVE+"/node_modules",T+"/node_modules")
# overlay my farm real files (the module + patched sidecar), replacing hardlinks
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def overlay(rel):
    src=os.path.join(FARM,rel);dst=os.path.join(T,rel)
    for d,ds,fs in os.walk(src):
        for f in fs:
            sp=os.path.join(d,f);dp=os.path.join(T,os.path.relpath(sp,FARM))
            os.makedirs(os.path.dirname(dp),exist_ok=True)
            if os.path.lexists(dp): os.unlink(dp)
            shutil.copy2(sp,dp)
overlay(B+"🏛️sealed/🧰️tool-snapshot")
sp=os.path.join(FARM,B+"🌐️remote-execution/🛰️sidecar/🟦️.ts");dp=os.path.join(T,B+"🌐️remote-execution/🛰️sidecar/🟦️.ts")
os.unlink(dp);shutil.copy2(sp,dp)
print("linked",n)
