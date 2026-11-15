# -*- coding: utf-8 -*-
"""R32 payload builder: payload/base = the live files now, payload/new = base with the edits below (each must match exactly once) plus the new test files. usage: build_payload.py"""
import os,shutil
HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload";S=HERE+"/src";LIVE="/Users/Shared/semio/"
FW="🧰️framework/";DESC=FW+"🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
B=FW+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/";FAIL=B+"🏛️service/🧯️failure/🟨️.mjs";SCH=B+"🧬️schema/🔣️.json";FT=B+"🧪️tests/🧯️failure/🟦️.ts"
NEWTESTS={FW+"🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎯️errno-consensus/🟦️.ts":"errno-consensus.test.ts",FW+"🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎯️errno-consensus/🐍️oracle.py":"oracle.py"}
def rd(p):return open(p,encoding="utf8").read()
def sub(text,old,new,count=1):
    assert text.count(old)==count,(old[:90],text.count(old));return text.replace(old,new)
SETTLE_FN='''/** 🎯️ Settles a read-only native call whose failure number is read by a second native call: `readErrno` runs after the failed call, and the runtime (collector, compiler, signal handlers) may overwrite `errno` in between (measured with the pinned runtime: 1 to 3 of 1 000 000 failed `fstatat` calls of an absent name read EPERM or 316 instead of ENOENT). A number counts only when two failed calls in a row report it (`reads` 2); a success on a repeat is the answer; EINTR and numbers without a name are retried without counting; the attempts bound the loop and the last reading is then returned alone (`reads` 1, or 0 for a number without a name).
 * @see https://pubs.opengroup.org/onlinepubs/9799919799/functions/errno.html */
export function settleNativeCallV1(run:()=>number,readErrno:()=>number,transient:(number:number)=>boolean,attempts:number,checkpoint:()=>void):{status:number;number:number;reads:number}{
  let previous=0;
  for(let attempt=1;;attempt++){
    const status=Number(run());if(status>=0)return{status,number:0,reads:0};
    const number=readErrno();
    if(!transient(number)){if(number===previous)return{status,number,reads:2};previous=number;}
    if(attempt>=attempts)return{status,number,reads:transient(number)?0:1};
    checkpoint();
  }
}
'''
def e_desc(t):
    t=sub(t,"/** 🐚️ Uses qualified POSIX system ABIs",SETTLE_FN+"/** 🐚️ Uses qualified POSIX system ABIs")
    t=sub(t,'settle=(run:()=>number):{status:number;number:number}=>{for(let attempt=1;;attempt++){const status=Number(run());if(status>=0)return{status,number:0};const number=errno().getInt32(0,true);if(attempt>=policy.maximumErrnoAttempts||number!==4&&!getSystemErrorName(-number).startsWith("Unknown"))return{status,number};checkpoint();}},',
      'settle=(run:()=>number):{status:number;number:number;reads:number}=>settleNativeCallV1(run,()=>errno().getInt32(0,true),number=>number===4||getSystemErrorName(-number).startsWith("Unknown"),policy.maximumErrnoAttempts,checkpoint),')
    t=sub(t,'failure=(operation:string,number:number,entry?:string):Error=>Object.assign(Error("Relative filesystem "+operation+" failed ("+getSystemErrorName(-number)+")"),{code:getSystemErrorName(-number),errno:number,root,...entry===undefined?{}:{entry}})',
      'failure=(operation:string,number:number,entry?:string,found?:{directory:string;reads:number}):Error=>Object.assign(Error("Relative filesystem "+operation+" failed ("+getSystemErrorName(-number)+")"),{code:getSystemErrorName(-number),errno:number,root,...entry===undefined?{}:{entry},...found===undefined?{}:found})')
    t=sub(t,'proofs=new WeakMap<object,{frame:ArtifactRelativeFrameV1;name:string;value:ArtifactRelativeEntryV1;parent:ArtifactReadFileIdentityV1}>();','proofs=new WeakMap<object,{frame:ArtifactRelativeFrameV1;name:string;value:ArtifactRelativeEntryV1;parent:ArtifactReadFileIdentityV1}>(),framePlaces=new WeakMap<ArtifactRelativeFrameV1,string>();')
    t=sub(t,'const withDirectory=<R>(open:()=>number,parent:ArtifactRelativeFrameV1|undefined,run:(directory:ArtifactRelativeDirectoryV1)=>Promise<R>):Promise<R>=>withSynchronousArtifactDescriptorV1(open,closeSync,descriptor=>directoryBody(descriptor,parent,run,parent?undefined:root.replace(/\\/$/u,"").length));',
      'const withDirectory=<R>(open:()=>number,parent:ArtifactRelativeFrameV1|undefined,run:(directory:ArtifactRelativeDirectoryV1)=>Promise<R>,childName?:string):Promise<R>=>withSynchronousArtifactDescriptorV1(open,closeSync,descriptor=>directoryBody(descriptor,parent,run,parent?undefined:root.replace(/\\/$/u,"").length,childName));')
    t=sub(t,'const directoryBody=async<R>(descriptor:number,parent:ArtifactRelativeFrameV1|undefined,run:(directory:ArtifactRelativeDirectoryV1)=>Promise<R>,preservationPathUnits?:number):Promise<R>=>{','const directoryBody=async<R>(descriptor:number,parent:ArtifactRelativeFrameV1|undefined,run:(directory:ArtifactRelativeDirectoryV1)=>Promise<R>,preservationPathUnits?:number,childName?:string):Promise<R>=>{')
    t=sub(t,'...(parent?{parent}:{})};let expected=artifactRelativeIdentityV1(descriptor);if(kind(expected)!=="directory"||rootDevice','...(parent?{parent}:{})},place=parent===undefined?root:(framePlaces.get(parent)??root).replace(/\\/$/u,"")+"/"+(childName??"?");framePlaces.set(frame,place);let expected=artifactRelativeIdentityV1(descriptor);if(kind(expected)!=="directory"||rootDevice')
    t=sub(t,'const buffer=Buffer.alloc(protocol.darwin.stat.bytes),{status,number}=settle(()=>calls[abi.statCall]!(descriptor,bytes,buffer,protocol.darwin.atSymlinkNoFollow));if(status!==0){if(number===2){check();return null;}throw failure(abi.statCall,number,name);}',
      'const buffer=Buffer.alloc(protocol.darwin.stat.bytes),{status,number,reads}=settle(()=>calls[abi.statCall]!(descriptor,bytes,buffer,protocol.darwin.atSymlinkNoFollow));if(status!==0){if(number===2){check();return null;}throw failure(abi.statCall,number,name,{directory:place,reads});}')
    t=sub(t,'const{status:fd,number}=settle(()=>calls.openat!(descriptor,bytes,0x200000|(darwin?0:abi.nofollow)|abi.cloexec|(darwin?abi.nonblock:0)));if(fd<0){if(number===2)throw absent;throw failure("openat",number,name);}return fd;},closeSync,async child=>{check();const facts=artifactRelativeIdentityV1(child),type=kind(facts);',
      'const{status:fd,number,reads}=settle(()=>calls.openat!(descriptor,bytes,0x200000|(darwin?0:abi.nofollow)|abi.cloexec|(darwin?abi.nonblock:0)));if(fd<0){if(number===2)throw absent;throw failure("openat",number,name,{directory:place,reads});}return fd;},closeSync,async child=>{check();const facts=artifactRelativeIdentityV1(child),type=kind(facts);')
    t=sub(t,'return withDirectory(()=>invoke("openat",descriptor,bytes,flags|abi.directory),frame,async child=>{if(!sameArtifactRelativeIdentityV1(before,child.stat()))throw Error("Relative child directory replaced before open");return run(child);});',
      'return withDirectory(()=>invoke("openat",descriptor,bytes,flags|abi.directory),frame,async child=>{if(!sameArtifactRelativeIdentityV1(before,child.stat()))throw Error("Relative child directory replaced before open");return run(child);},name);')
    return t
def e_fail(t):
    t=sub(t,'import {join} from "node:path";','import {lstatSync} from "node:fs";\nimport {join} from "node:path";')
    t=sub(t,'''export function deniedPlaceOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return"root="+error.root+(typeof error.entry==="string"?" entry="+error.entry:"")+(typeof error.code==="string"?" code="+error.code:"");''','''export function deniedPlaceOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return"root="+error.root+(typeof error.directory==="string"?" directory="+error.directory:"")+(typeof error.entry==="string"?" entry="+error.entry:"")+(typeof error.code==="string"?" code="+error.code:"")+(typeof error.reads==="number"?" reads="+error.reads:"");''')
    t=sub(t,'if(typeof error.root==="string")return typeof error.entry==="string"?join(error.root,error.entry):error.root;','if(typeof error.root==="string")return typeof error.entry==="string"?join(typeof error.directory==="string"?error.directory:error.root,error.entry):error.root;')
    t=sub(t,'const WANTED_FIELDS_V1=','''const relativeCallOfV1=(error,depth=4)=>typeof error!=="object"||error===null||depth<0?undefined:typeof error.reads==="number"&&typeof error.root==="string"?error:relativeCallOfV1(error.cause,depth-1);

/** 🔬️ Whether a relative filesystem call that the helper reports as refused (`EPERM`, `reads` agreeing readings of its failure number) is really a refusal of the access of this process: the same place is read once more by path, with the node file system, which does not share the native call. A refusal by path as well (`EPERM`) is the macOS privacy refusal and gives `undefined`; any other answer (the place exists, is absent, or is an ordinary permission) is returned with the place, the reads and the answer, because then the consent of the user is not what is missing. @see https://developer.apple.com/documentation/bundleresources/protecting-the-user-s-files */
export function relativeCallDivergenceV1(error,{inspect=path=>lstatSync(path)}={}){
 const call=relativeCallOfV1(error);if(call===undefined)return undefined;
 const place=placePathV1(call);
 try{inspect(place);return{place,reads:call.reads,found:"present"};}
 catch(refusal){const found=typeof refusal?.code==="string"?refusal.code:"?";return found==="EPERM"?undefined:{place,reads:call.reads,found};}
}

const WANTED_FIELDS_V1=''')
    t=sub(t,'retry="later",root,platform,home,fullImage=fullImageOfV1,','retry="later",root,platform,home,fullImage=fullImageOfV1,inspect,')
    t=sub(t,''' log(facts?"disk-full":"request-failed",reference+" "+detail+(place||permission?"\\nwhere: "+(place??"root=? entry=? code="+(typeof error?.code==="string"?error.code:"EPERM"))+" "+service:""));''',''' const divergence=folder!==undefined&&!facts?relativeCallDivergenceV1(error,inspect?{inspect}:{}):undefined;
 log(facts?"disk-full":"request-failed",reference+" "+detail+(place||permission?"\\nwhere: "+(place??"root=? entry=? code="+(typeof error?.code==="string"?error.code:"EPERM"))+" "+service+(divergence?" pathcheck="+divergence.found:""):""));''')
    t=sub(t,' if(folder!==undefined)return{code:"consent",',''' if(divergence)return{code:"unserved",key:"host-relative-call-refused",fields:{place:divergence.place,reads:String(divergence.reads),found:divergence.found,reference,log:logPath},retry};
 if(folder!==undefined)return{code:"consent",''')
    return t
def e_sch(t):
    t=sub(t,'''      "host-graph-wanted-expired": "The project graph could not be read''','''      "host-relative-call-refused": "A filesystem call of the resident service was refused with EPERM for {place} ({reads} readings agreed), but the service itself reads the same place by path without a refusal ({found}), so this is not the proven missing macOS file access consent: the service keeps running and nothing was changed. Run the command again; if this message returns, send the line that starts with `where:` below reference {reference} in {log}.",
      "host-graph-wanted-expired": "The project graph could not be read''')
    t=sub(t,'''      "host-graph-wanted-expired": "Der Projektgraph konnte''','''      "host-relative-call-refused": "Ein Dateisystemaufruf des residenten Dienstes wurde für {place} mit EPERM abgelehnt ({reads} Messungen stimmten überein), aber der Dienst selbst liest denselben Ort über den Pfad ohne Ablehnung ({found}); das ist daher nicht die belegte fehlende macOS-Dateizugriffserlaubnis: der Dienst läuft weiter und nichts wurde verändert. Führe den Befehl erneut aus; wenn diese Meldung wiederkehrt, sende die Zeile, die mit `where:` beginnt, unter der Referenz {reference} in {log}.",
      "host-graph-wanted-expired": "Der Projektgraph konnte''')
    return t
def e_ft(t):
    t=sub(t,'import {readFileSync} from "node:fs";','import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {join} from "node:path";')
    return t.rstrip("\n")+"\n"+rd(S+"/failure-additions.ts")
EDITS={DESC:e_desc,FAIL:e_fail,SCH:e_sch,FT:e_ft}
if os.path.isdir(P):shutil.rmtree(P)
names=[]
for rel,fn in EDITS.items():
    t=rd(LIVE+rel);os.makedirs(os.path.dirname(P+"/base/"+rel),exist_ok=True);os.makedirs(os.path.dirname(P+"/new/"+rel),exist_ok=True)
    open(P+"/base/"+rel,"w",encoding="utf8",newline="").write(t);open(P+"/new/"+rel,"w",encoding="utf8",newline="").write(fn(t));names.append(rel)
for rel,src in NEWTESTS.items():
    os.makedirs(os.path.dirname(P+"/new/"+rel),exist_ok=True);open(P+"/new/"+rel,"w",encoding="utf8",newline="").write(rd(S+"/"+src));names.append(rel)
open(P+"/files.txt","w",encoding="utf8").write("\n".join(names)+"\n");print(len(names),"files")
