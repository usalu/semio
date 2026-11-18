#!/usr/bin/env python3
"""Independent closure check: every crate hash a lane's `.rmeta` records must be matched by a lane file of that crate name and suffix with that hash.
usage: unitcache-closure-check.py <rustc> <build-directory> [crate-prefix]"""
import os,re,subprocess,sys
rustc,build=sys.argv[1],sys.argv[2];prefix=sys.argv[3] if len(sys.argv)>3 else ""
env={**os.environ,"RUSTC_BOOTSTRAP":"1"}
def root(path):
    out=subprocess.run([rustc,"-Z","ls=root",path],capture_output=True,text=True,env=env)
    if out.returncode:return None
    own=re.search(r"^hash ([0-9a-f]{32}) stable_crate_id",out.stdout,re.M)
    deps=re.findall(r"^\d+ (\S+) hash ([0-9a-f]{32}) host_hash",out.stdout,re.M)
    return (own.group(1) if own else None,deps)
sysroot=subprocess.run([rustc,"--print","sysroot"],capture_output=True,text=True).stdout.strip()
provided=set()
for dirpath,_,names in os.walk(os.path.join(sysroot,"lib")):
    for name in names:
        if name.startswith("lib"):provided.add(name[3:].rpartition(".")[0])
files={}
for dirpath,_,names in os.walk(build):
    for name in names:
        stem,dot,extension=name.rpartition(".")
        if name.startswith("lib") and extension in("rmeta","dylib","so"):files.setdefault(stem[3:],os.path.join(dirpath,name))
hashes={k:(root(v) or (None,[]))[0] for k,v in files.items()}
bad=0
for ident,path in sorted(files.items()):
    if not ident.startswith(prefix.replace("-","_")):continue
    info=root(path)
    if info is None:print("UNREADABLE",path);bad+=1;continue
    for dep,hash in info[1]:
        if dep not in hashes:
            if dep in provided:continue
            print("MISSING",ident,"needs",dep,hash);bad+=1
        elif hashes[dep]!=hash:print("MISMATCH",ident,"needs",dep,hash,"lane has",hashes[dep]);bad+=1
print("checked",len(files),"units;",bad,"inconsistencies")
sys.exit(1 if bad else 0)
