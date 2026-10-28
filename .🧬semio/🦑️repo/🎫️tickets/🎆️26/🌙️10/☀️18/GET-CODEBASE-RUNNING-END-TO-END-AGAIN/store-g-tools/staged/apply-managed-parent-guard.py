"""Staged for after the measurement window (11:30): (A) an unfunded image can never mount inside a managed parent,
(B) a foreign-mount verdict needs a stable observation (grace), so a fixture child being born or retired by a sibling
process (gf-*, cc-*, pc-*, silence-* race class) cannot kill an unrelated run.
usage: python3 apply-managed-parent-guard.py --check | --apply"""
import json,os,sys
R="/Users/ueli/Documents/semio/"
S=R+"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/"
HERE=os.path.dirname(os.path.abspath(__file__))
def rd(p): return open(p,encoding="utf8").read()
edits={}
def add(path,a,b): edits.setdefault(path,[]).append((a,b))
# storage schema: managed parent pattern + bilingual refusal
add(S+"🧬️schema/🔣️.json",'        "mountWithoutOwnersFlag": "noowners",',
'''        "mountWithoutOwnersFlag": "noowners",
        "managedParentMountPattern": "(?:/\\\\.cache/b/[a-z]/|/rust/fixed/)m$",
        "managedParentRefusal": {
          "en": "Native image at {directory} would mount inside the managed cache partition mounted at {parent}, which funds no such child. Create test and scratch images below the per-run sandbox (testScratchRootV1), never below .cache/b or the rust cache.",
          "de": "Das native Image unter {directory} würde in der verwalteten Cache-Partition eingehängt, die unter {parent} gemountet ist und kein solches Kind finanziert. Lege Test- und Scratch-Images unter dem Sandbox-Verzeichnis des Laufs an (testScratchRootV1), nie unter .cache/b oder dem Rust-Cache."
        },''')
# owners module
add(S+"🍎️image/👤️owners/🟦️.ts",'/** 🛑️ Refuses a nested attach inside a parent mounted without owners',
'''/** 🚧️ Refuses to attach an image that its managed parent does not fund: the innermost mount hosting the directory must not be a managed cache partition. */
export function requireNativeImageOutsideManagedParentV1(mountOutput:string,directory:string):void{
 const host=nativeImageHostMountV1(mountOutput,directory);
 if(!new RegExp(protocol.managedParentMountPattern,"u").test(host.mountPoint))return;
 const fill=(text:string)=>text.replaceAll("{directory}",directory).replaceAll("{parent}",host.mountPoint);
 throw Object.assign(Error(fill(protocol.managedParentRefusal.en)+"\\n"+fill(protocol.managedParentRefusal.de)),{name:"NativeStorageManagedParentError"});
}
/** 🛑️ Refuses a nested attach inside a parent mounted without owners''')
# cleanup command: guard on activation, grace on foreign verdicts
C=S+"🧹️cleanup/🎛️command/🟦️.ts"
add(C,'if(scope.parent)requireNativeImageHostOwnersV1(await captureOwnedCommandV1(imageSchema.definitions.ImageProtocol.const.mountProgram,[],scope.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}}),scope.directory);',
 '{const table=await captureOwnedCommandV1(imageSchema.definitions.ImageProtocol.const.mountProgram,[],scope.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}});if(scope.parent)requireNativeImageHostOwnersV1(table,scope.directory);else requireNativeImageOutsideManagedParentV1(table,scope.directory);}')
add(C,'/** 🪪️ Describes the nearest sandbox owner row above a path',
'''/** ⏳️ Decides a refused child read from a STABLE observation: a foreign verdict must survive the grace of the receiver protocol (the image still attached, still nobody using it), so a sibling process that is birthing or retiring a fixture child of this parent, or finishing its detach, never kills an unrelated run; a mount that vanished meanwhile is tolerated as `live`. */
export async function unfundedChildDecisionStableV1(error:unknown,attached:()=>boolean,live:()=>boolean,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))):Promise<"live"|"foreign"|"other">{
 const first=unfundedChildDecisionV1(error,attached(),live);if(first!=="foreign")return first;
 const grace=receiverSchema.definitions.Protocol.const.foreignGrace;
 for(let attempt=0;attempt<grace.attempts;attempt++){await pause(grace.pauseMilliseconds);if(!attached()||live())return"live";}
 return"foreign";
}
/** 🪪️ Describes the nearest sandbox owner row above a path''')
add(C,'.catch(error=>{const decision=unfundedChildDecisionV1(error,nativeImageAttachedV1(row.image,row.root),()=>nativeLiveChildV1(dirname(row.root),receiver.options.leaseDirectory));',
      '.catch(async error=>{const decision=await unfundedChildDecisionStableV1(error,()=>nativeImageAttachedV1(row.image,row.root),()=>nativeLiveChildV1(dirname(row.root),receiver.options.leaseDirectory));')
add(C,'.catch(error=>{if(unfundedChildDecisionV1(error,nativeImageAttachedV1(row.image,row.root),()=>nativeLiveChildV1(dirname(row.root),receiver.options.leaseDirectory))==="foreign")',
      '.catch(async error=>{if(await unfundedChildDecisionStableV1(error,()=>nativeImageAttachedV1(row.image,row.root),()=>nativeLiveChildV1(dirname(row.root),receiver.options.leaseDirectory))==="foreign")')
add(S+"🧹️cleanup/📥️receiver/🧬️schema/🔣️.json",'        "concurrentChangeAttempts": 6,','        "concurrentChangeAttempts": 6,\n        "foreignGrace": {\n          "attempts": 4,\n          "pauseMilliseconds": 1500\n        },')
add(C,'import{nativeImageAttachArgumentsV1,requireNativeImageHostOwnersV1}from"../../🍎️image/👤️owners/🟦️.ts";','import{nativeImageAttachArgumentsV1,requireNativeImageHostOwnersV1,requireNativeImageOutsideManagedParentV1}from"../../🍎️image/👤️owners/🟦️.ts";')
def plan():
    out={}
    for path,pairs in edits.items():
        t=rd(path)
        for a,b in pairs:
            n=t.count(a)
            if n!=1: print("ANCHOR",n,path[-70:],a[:80]); sys.exit(1)
            t=t.replace(a,b)
        out[path]=t
    return out
out=plan()
# the owners module needs the storage schema constant and the cleanup command needs the new import
print("anchors ok:",sum(len(v) for v in edits.values()))
if "--apply" in sys.argv:
    o=S+"🍎️image/👤️owners/🟦️.ts"; t=out[o]
    assert "const protocol=" in t
    for path,t in out.items():
        if path.endswith(".json"): json.loads(t)
        open(path,"w",encoding="utf8").write(t)
    print("applied; next: copy staged/managed/* tests into place and run them")
