#!/usr/bin/env python3
"""🔏 audit4 `gatelock`: fences the machine-wide host-capacity gate lock (anchored, exact-count edits; refuses on a lost anchor, a no-op when already applied).
usage: python3 -I edit_gatelock.py <root>
A holder gives the lock back only while the owner file still holds the text it wrote, writes a grant/upgrade/growth/handover only while it still holds the lock, and publishes its progress after the lock is released."""
import os, stat, sys

H = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
EDITS = {
    H + "🎫️gate/📋️contract/🟦️.ts": [
        ("takeLock:(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number)=>boolean;",
         "takeLock:(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number)=>boolean;releaseLock:(directory:string,pid:number)=>void;holdsLock:(directory:string,pid:number)=>boolean;"),
    ],
    H + "🎫️gate/🟦️.ts": [
        ("function takeLock(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number,afterCreate?:()=>void):boolean{",
         "const lockOwners=new Map<string,string>(),lockKey=(lock:string,pid:number):string=>lock+\"\\n\"+pid,ownerText=(lock:string):string|undefined=>{try{return readFileSync(join(lock,\"owner\"),\"utf8\");}catch{return undefined;}};\n"
         "/** 🔏️ Whether the gate lock is still the one `pid` took in this process: the owner file holds exactly the text it wrote. A holder that stalled past `wait.lockStaleMilliseconds` (swap, a stopped process) may have been broken by a peer that then took the lock; such a holder must write nothing more. */\n"
         "function holdsLock(directory:string,pid:number):boolean{const lock=join(directory,\"lock\"),mine=lockOwners.get(lockKey(lock,pid));return mine!==undefined&&ownerText(lock)===mine;}\n"
         "/** 🔓️ Gives the gate lock back only while it is still the one `pid` took (see `holdsLock`), so a holder whose lock was broken never removes the lock of the peer that holds it now. */\n"
         "function releaseLock(directory:string,pid:number):void{const lock=join(directory,\"lock\"),key=lockKey(lock,pid),mine=lockOwners.get(key);lockOwners.delete(key);if(mine!==undefined&&ownerText(lock)===mine)rmSync(lock,{recursive:true,force:true});}\n"
         "function takeLock(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number,afterCreate?:()=>void):boolean{", 1),
        ("try{writeFileSync(join(lock,\"owner\"),`${pid}:${now}`);return true;}",
         "try{const text=`${pid}:${now}`;writeFileSync(join(lock,\"owner\"),text);lockOwners.set(lockKey(lock,pid),text);return true;}", 1),
        ("export const takeHostLockV1=takeLock;",
         "export const takeHostLockV1=takeLock;\n/** 🗝️ The fenced release of the ledger lock (`releaseLock`), for laws. */\nexport const releaseHostLockV1=releaseLock;\n/** 🧿️ The ownership check of the ledger lock (`holdsLock`), for laws. */\nexport const holdsHostLockV1=holdsLock;", 1),
        ("   try{utimesSync(waiterPath,new Date(),new Date());}catch{}\n   if(takeLock(directory,now(),policy,pid)){",
         "   try{utimesSync(waiterPath,new Date(),new Date());}catch{}\n   let progressAt:number|undefined;\n   if(takeLock(directory,now(),policy,pid)){", 1),
        ("     if(mine.verdict===\"admit\"){\n      grant=requireHostCapacityV1<HostGrantV1>(\"Grant\",",
         "     if(mine.verdict===\"admit\"){\n      if(!holdsLock(directory,pid))continue;\n      grant=requireHostCapacityV1<HostGrantV1>(\"Grant\",", 1),
        ("      lastProgress=at;write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000)));\n      await options.control.publish({resource:protocol.resource,mode:\"shared\",elapsedMs:at-arrivedAtMs});checkResourceLeaseControl(options.control);await options.control.yieldContinuation();\n     }\n    }finally{rmSync(join(directory,\"lock\"),{recursive:true,force:true});}\n   }\n   if(grant)break;\n",
         "      lastProgress=at;write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000)));progressAt=at;\n     }\n    }finally{releaseLock(directory,pid);}\n   }\n   if(grant)break;\n   if(progressAt!==undefined){await options.control.publish({resource:protocol.resource,mode:\"shared\",elapsedMs:progressAt-arrivedAtMs});checkResourceLeaseControl(options.control);await options.control.yieldContinuation();}\n", 1),
        ("Object.freeze({protocol,takeLock,grantPath,",
         "Object.freeze({protocol,takeLock,releaseLock,holdsLock,grantPath,", 1),
    ],
    H + "🔼️upgrade/🟦️.ts": [
        ("import {existsSync,readFileSync,rmSync,unlinkSync,utimesSync,writeFileSync} from \"node:fs\";",
         "import {existsSync,readFileSync,unlinkSync,utimesSync,writeFileSync} from \"node:fs\";", 1),
        ("  if(settled.jobs===undefined)return undefined;\n",
         "  if(settled.jobs===undefined||!ledger.holdsLock(directory,pid))return undefined;\n", 1),
        (" }finally{rmSync(join(directory,\"lock\"),{recursive:true,force:true});}\n}\n",
         " }finally{ledger.releaseLock(directory,pid);}\n}\n", 1),
        ("   try{utimesSync(waiterPath,new Date(),new Date());}catch{}\n   if(takeLock(directory,now(),policy,pid)){",
         "   try{utimesSync(waiterPath,new Date(),new Date());}catch{}\n   let progressAt:number|undefined;\n   if(takeLock(directory,now(),policy,pid)){", 1),
        ("     if(mine.verdict===\"admit\"){\n      const{quiet:_,...rest}=state;",
         "     if(mine.verdict===\"admit\"){\n      if(!ledger.holdsLock(directory,pid))continue;\n      const{quiet:_,...rest}=state;", 1),
        ("      await control.publish({resource:protocol.resource,mode:\"shared\",elapsedMs:at-started});checkResourceLeaseControl(control);await control.yieldContinuation();\n     }\n    }finally{rmSync(join(directory,\"lock\"),{recursive:true,force:true});}\n   }\n   if(done)return \"upgraded\";\n",
         "      progressAt=at;\n     }\n    }finally{ledger.releaseLock(directory,pid);}\n   }\n   if(done)return \"upgraded\";\n   if(progressAt!==undefined){await control.publish({resource:protocol.resource,mode:\"shared\",elapsedMs:progressAt-started});checkResourceLeaseControl(control);await control.yieldContinuation();}\n", 1),
    ],
    H + "🎫️gate/🤝️handover/🟦️.ts": [
        ("import {rmSync,utimesSync} from \"node:fs\";\nimport {join} from \"node:path\";\n",
         "import {utimesSync} from \"node:fs\";\n", 1),
        ("const {protocol,takeLock,grantPath,", "const {protocol,takeLock,releaseLock,holdsLock,grantPath,", 1),
        ("  writeJson(path!,taken);\n", "  if(!holdsLock(directory,pid))return undefined;\n  writeJson(path!,taken);\n", 1),
        (" }finally{rmSync(join(directory,\"lock\"),{recursive:true,force:true});}\n}\n",
         " }finally{releaseLock(directory,pid);}\n}\n", 1),
    ],
}

def write_keep_mode(path, text):
    mode = stat.S_IMODE(os.stat(path).st_mode)
    tmp = path + ".a4tmp"
    with open(tmp, "w", encoding="utf8", newline="") as f: f.write(text)
    os.chmod(tmp, mode); os.replace(tmp, path)

def main():
    root = os.path.abspath(sys.argv[1])
    plans = {}
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel); text = open(path, encoding="utf8", newline="").read()
        for row in edits:
            old, new = row[0], row[1]
            if new in text: continue
            count = text.count(old)
            if count != 1: sys.exit(f"anchor count {count} != 1 in {rel[-60:]}: {old[:90]!r}")
            text = text.replace(old, new, 1)
        plans[path] = text
    for path, text in plans.items():
        if open(path, encoding="utf8", newline="").read() != text: write_keep_mode(path, text); print("edited", path[-70:])
        else: print("unchanged", path[-70:])
main()
