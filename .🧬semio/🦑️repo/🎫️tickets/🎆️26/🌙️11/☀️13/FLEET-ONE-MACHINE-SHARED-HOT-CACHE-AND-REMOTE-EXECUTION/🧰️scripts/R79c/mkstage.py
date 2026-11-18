#!/usr/bin/env python3
"""R79b stage builder (landing 2): a dependency store hold whose registered pid is alive but whose process started after the hold was registered is a reused pid (Windows reuses pids within hours): the entry is removed and the wait ends, instead of waiting up to six hours for a process that is not the holder (g1-m05 waited on pid 48352 of 2026-11-14)."""
import os,shutil,sys
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FD=L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def put(rel,base,text,kind="mod"):
    i="f%d"%n[0];n[0]+=1
    if base is not None:open(H+i+".base","w",encoding="utf8").write(base)
    open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s %s"%(i,rel,kind))
def stage(rel,edits):
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:100]))
        text=text.replace(old,rep)
    put(rel,base,text)
# the funding module edit is already live (land.py crashed in its probe branch after applying it, before this stage was rebuilt); this stage only carries the corrected law
TEST='''/** 🧬️ Contract: a registered dependency store hold whose pid is alive but whose process started after the registration is a reused pid; its entry is removed and it never blocks the store (g1-m05 waited up to six hours on pid 48352 of two days before), while a holder that started with its registration, an unknown start and this process stay live. */
import{expect,test}from"bun:test";
import{observeDependencyStoreHoldersV1,dependencyStorePidReusedV1,DEPENDENCY_STORE_WAIT_V1,type DependencyStoreRegistryPortsV1}from"../../🟦️.ts";

const registered=1_000_000_000_000,entry=(pid:number)=>({path:"/registry/"+pid+".json",entry:{version:1,pid,startedAtMs:registered,command:"3a31 show projects",directory:"/d",holds:[{resource:"repository:dependencies-store",mode:"shared",sinceMs:registered+10}]}}) as never;
const ports=(started:(pid:number)=>number|undefined,removed:string[]):DependencyStoreRegistryPortsV1=>({entries:()=>[entry(48352),entry(7)],alive:()=>true,osStartedAtMs:started,ancestors:()=>[],remove:path=>{removed.push(path);},pid:1,startedAtMs:registered+999999});

test("a reused pid is gone and removed; the true holder, an unknown start and the tolerance stay live",()=>{
 const removed:string[]=[],tolerance=DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;
 const seen=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",ports(pid=>pid===48352?registered+2*86_400_000:registered+tolerance,removed));
 expect(seen.gone.map(row=>row.pid)).toEqual([48352]);expect(removed).toEqual(["/registry/48352.json"]);expect(seen.live.map(row=>row.pid)).toEqual([7]);
 const unknown=observeDependencyStoreHoldersV1("/d","repository:dependencies-store",ports(()=>undefined,[]));expect(unknown.live.map(row=>row.pid).sort((a,b)=>a-b)).toEqual([7,48352]);expect(unknown.gone).toEqual([]);
});

test("the rule is exact around the tolerance and ignores unknown starts",()=>{
 const tolerance=DEPENDENCY_STORE_WAIT_V1.holderStartToleranceMilliseconds;
 expect(dependencyStorePidReusedV1(registered,registered+tolerance)).toBe(false);expect(dependencyStorePidReusedV1(registered,registered+tolerance+1)).toBe(true);expect(dependencyStorePidReusedV1(registered,registered-86_400_000)).toBe(false);expect(dependencyStorePidReusedV1(registered,undefined)).toBe(false);expect(dependencyStorePidReusedV1(undefined,registered)).toBe(false);
});
'''
put(FD+"🧪️tests/🧬️reused-pid/🟦️.ts",None,TEST,"new")
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n");print("staged",len(rows))
