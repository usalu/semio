#!/usr/bin/env python3
"""R52 landing: takes landing.lock, applies the anchored source edit (compactSourceInputs resolves the writer directory only after the first roster capture), runs the test files of the touched members, records sha256 in landings.md. Test-file edits of the same package were made in place beforehand (tests only). usage: land.py [--dry-run]"""
import os,sys,subprocess,hashlib,datetime
R="/Users/ueli/Documents/semio"
T=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/"
SRC=B+"📥️inputs/🟨️.mjs"
SEAL=B+"🌐️remote-cache/🔐️sealed/🟦️.ts"
SEAL_EDITS=[('export class SealedErrorV1 extends Error{constructor(readonly code:"malformed"|"clock"|"replay"|"decrypt"|"large"){super(`Sealed exchange refused: ${code}`);this.name="SealedErrorV1";}}','export class SealedErrorV1 extends Error{readonly code:"malformed"|"clock"|"replay"|"decrypt"|"large";constructor(code:"malformed"|"clock"|"replay"|"decrypt"|"large"){super(`Sealed exchange refused: ${code}`);this.code=code;this.name="SealedErrorV1";}}')]
EDITS=[(', files = digest === "interface" ? [...interfaceClosureFilesV1(listed)] : listed, directory = repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures");',', files = digest === "interface" ? [...interfaceClosureFilesV1(listed)] : listed; let directory;'),
 ('key = sha256HexV1(contract); publishInputClosureV1(join(directory, `${key}.json`), contract); return { env: `SEMIO_SOURCE_CLOSURE_${key}` };','key = sha256HexV1(contract); directory ??= repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures"); publishInputClosureV1(join(directory, `${key}.json`), contract); return { env: `SEMIO_SOURCE_CLOSURE_${key}` };')]
TESTS=[B+"🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight",B+"🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests",B+"🧪️tests/🕸️graph/🔌️nx-engine",L+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🧪️tests",B+"🌐️remote-cache/🔐️sealed/🧪️tests",B+"📥️inputs/🧪️tests",B+"📥️inputs/📋️data/🧪️tests",B+"📥️inputs/🧮️digest/🧪️tests",B+"📥️inputs/🧮️digest/🧪️tests/🧬️inference" if False else B+"📥️inputs/🌳️owner-view/🧪️tests/🧬️inference",B+"🧷️format-stamps/🧪️tests",B+"🕸️graph/📥️inference/🧪️tests/🧷️closure-facts",L+"⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🌱️incremental",L+"⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🧱️compaction-helpers" if False else B+"📥️inputs/🧪️tests/🧱️compaction-helpers",L+"⚡️caching/🦀️cargo/🗃️unit-cache/🔭️interface/🧪️tests"]
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12]
ART="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R52/art"
os.makedirs(ART,exist_ok=True)
dry="--dry-run" in sys.argv
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("landing.lock is held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R52-%d %s\n"%(os.getpid(),datetime.datetime.now().isoformat()))
try:
    befores={};news={}
    for src,edits in ((SRC,EDITS),(SEAL,SEAL_EDITS)):
        p=R+"/"+src;text=open(p,encoding="utf8").read();befores[src]=sha(p);new=text
        for old,rep in edits:
            if rep in new:continue
            if new.count(old)!=1:sys.exit("anchor not unique/absent: "+old[:60])
            new=new.replace(old,rep)
        news[src]=(p,text,new)
    if dry:print("dry run ok",[n!=t for _,t,n in news.values()]);sys.exit(0)
    for p,text,new in news.values():
        if new!=text:open(p,"w",encoding="utf8").write(new)
    res=[]
    for rel in TESTS:
        d=R+"/"+rel
        if not os.path.isdir(d):res.append((rel,-1,["missing dir"]));continue
        files=[f for f in sorted(os.listdir(d)) if f=="🟦️.ts"]
        for f in files:
            env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0";env.setdefault("SEMIO_TEST_ARTIFACT_DIR",ART)
            r=subprocess.run([BUN,"test","./"+f,"--timeout","180000"],cwd=d,env=env,capture_output=True,text=True)
            out=r.stdout+r.stderr;failed=[l[:160] for l in out.splitlines() if l.startswith("(fail)")]
            res.append((rel,r.returncode,failed));print("ok  " if r.returncode==0 else "RED ",rel[len(L):],failed,flush=True)
    with open(T+"/landings.md","a",encoding="utf8") as f:
        f.write("\n## Package R52 landing (%s): `compactSourceInputs` takes the writer directory only after its first roster capture (admission before writer effects); test repairs of the red baseline\n"%datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
        for src in befores:f.write("- edit `%s` before %s after %s (R52: writer directory after first capture; sealed error without parameter property so node strip-only loads it)\n"%(src[len(L):],befores[src],sha(R+"/"+src)))
        for rel,code,failed in res:f.write("- test `LIB/%s` exit %d, %d red%s\n"%(rel[len(L):],code,len(failed),": "+"; ".join(failed) if failed else ""))
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
