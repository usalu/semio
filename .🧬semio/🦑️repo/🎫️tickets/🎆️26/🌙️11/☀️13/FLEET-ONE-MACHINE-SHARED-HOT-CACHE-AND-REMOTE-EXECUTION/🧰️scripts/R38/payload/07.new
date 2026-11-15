import {createHash} from "node:crypto";
import {graphMemoryRefusalOfV1} from "../../../🧠️memory/🟨️.mjs";
import {homedir} from "node:os";
import {lstatSync} from "node:fs";
import {join} from "node:path";
import {SERVICE_ROUTES_V1,consentIdentityV1,protectedFolderOfV1} from "../../🔐️consent/🟨️.mjs";
import {diskFullOfV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts";
import {fullImageOfV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🖼️full/🟦️.ts";
import {DISK_RECLAIM_COMMAND_V1,formatDiskBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/💽️disk/🟦️.ts";

const IMAGE_RECLAIM_COMMAND_V1="repo cache gc";

/** 🔐️ Recognises the macOS file access consent: a relative filesystem call of the service that fails with `EPERM` while the worktree lies below Documents, Desktop or Downloads of the user is a process without the consent of this session (a service whose responsible process is not the launcher `repo`, or a `repo` whose binary was replaced), not a defect of the request. Returns the folder, or `undefined` for every other failure. */
export function accessDeniedOfV1(error,{root=process.cwd(),platform=process.platform,home=homedir()}={}){
 if(!(error?.code==="EPERM"||/\(EPERM\)/u.test(String(error?.message??""))))return undefined;
 return protectedFolderOfV1(root,{platform,home});
}

const errorPathsV1=(error,depth=4)=>typeof error!=="object"||error===null||depth<0?[]:[error.path,error.dest].filter(value=>typeof value==="string"&&value.length>0).concat(errorPathsV1(error.cause,depth-1));

/** 📍️ What a refused filesystem call names: the root and the entry that carry the error (or its cause), as the line the service log prints below the failure;  when the error names nothing. */
export function deniedPlaceOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return"root="+error.root+(typeof error.directory==="string"?" directory="+error.directory:"")+(typeof error.entry==="string"?" entry="+error.entry:"")+(typeof error.code==="string"?" code="+error.code:"")+(typeof error.reads==="number"?" reads="+error.reads:"");
 return deniedPlaceOfV1(error.cause,depth-1);
}

/** 🛣️ The facts of the service process that go below every refused place: the route it was started by (`SEMIO_SERVICE_ROUTE`: `launchd` a job under the launcher, `direct` a child of the calling process, else `foreground`), its pid and its parent pid, so that the log says which start produced a process without access. */
export function serviceFactsV1({environment=process.env,pid=process.pid,ppid=process.ppid}={}){return"route="+serviceRouteV1(environment)+" pid="+pid+" ppid="+ppid;}

/** 🛣️ The route of this service process (`launchd`, `direct`, else `foreground`). */
export function serviceRouteV1(environment=process.env){return SERVICE_ROUTES_V1.includes(environment.SEMIO_SERVICE_ROUTE)?environment.SEMIO_SERVICE_ROUTE:"foreground";}

const refusedByPermissionV1=(error,depth=4)=>typeof error==="object"&&error!==null&&depth>=0&&(error.code==="EPERM"||/\(EPERM\)/u.test(String(error.message??""))||refusedByPermissionV1(error.cause,depth-1));

const placePathV1=(error,depth=4)=>{
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(typeof error.root==="string")return typeof error.entry==="string"?join(typeof error.directory==="string"?error.directory:error.root,error.entry):error.root;
 return placePathV1(error.cause,depth-1);
};

const relativeCallOfV1=(error,depth=4)=>typeof error!=="object"||error===null||depth<0?undefined:typeof error.reads==="number"&&typeof error.root==="string"?error:relativeCallOfV1(error.cause,depth-1);

/** 🔬️ Whether a relative filesystem call that the helper reports as refused (`EPERM`, `reads` agreeing readings of its failure number) is really a refusal of the access of this process: the same place is read once more by path, with the node file system, which does not share the native call. A refusal by path as well (`EPERM`) is the macOS privacy refusal and gives `undefined`; any other answer (the place exists, is absent, or is an ordinary permission) is returned with the place, the reads and the answer, because then the consent of the user is not what is missing. @see https://developer.apple.com/documentation/bundleresources/protecting-the-user-s-files */
export function relativeCallDivergenceV1(error,{inspect=path=>lstatSync(path)}={}){
 const call=relativeCallOfV1(error);if(call===undefined)return undefined;
 const place=placePathV1(call);
 try{inspect(place);return{place,reads:call.reads,found:"present"};}
 catch(refusal){const found=typeof refusal?.code==="string"?refusal.code:"?";return found==="EPERM"?undefined:{place,reads:call.reads,found};}
}

const WANTED_FIELDS_V1=Object.freeze(["wanted","arrived","seconds","missing","local","heavy"]);

/** 🙋️ The numbers of a graph read that waited for the fleet and was refused (`build.graph.wanted-expired`): the values of the refusal itself, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🪪️graph-state/🙋️wanted/📖️read/🟦️.ts */
export function wantedRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 const named=error.key==="build.graph.wanted-expired"||/graph\.wanted-expired/u.test(String(error.message??""));
 if(named){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\.wanted-expired (\{.*\})\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return Object.fromEntries(WANTED_FIELDS_V1.map(name=>[name,String(values[name]??(name==="heavy"?"":0))]));
 }
 return wantedRefusalOfV1(error.cause,depth-1);
}

/** 🪜️ The words of a graph read whose plan (the assembly of a wave) grew past what this machine gives a plan (`build.graph.plan-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🧠️memory/🪜️plan-stage/🟨️.mjs */
export function planRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.plan-too-heavy"||/graph\.plan-too-heavy/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\.plan-too-heavy (\{.*\})\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return{stage:String(values.stage??"?"),operation:String(values.operation??"?"),megabytes:String(values.megabytes??0),limit:String(values.limit??0),available:String(values.available??"?"),reserve:String(values.reserve??"?")};
 }
 return planRefusalOfV1(error.cause,depth-1);
}

/** 🛑️ The words of a graph read that stopped projects too heavy for this machine (`build.graph.owner-too-heavy`): the refusal\'s own values, or, when it crossed a process boundary as text, the JSON its message ends with; `undefined` for every other failure. @see ../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs */
export function heavyRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.owner-too-heavy"||/graph\.owner-too-heavy/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/graph\.owner-too-heavy (\{.*\})\s*$/u.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  return{named:String(values.named??""),limit:String(values.limit??0)};
 }
 return heavyRefusalOfV1(error.cause,depth-1);
}

/** 🏷️ The text a failure is logged by: its stack, led by its name, code, message and the project it belongs to (`semioProject`) whenever the stack does not carry them, so that a failure whose stack lost its message still says what failed and for which project. */
export function failureDetailV1(error){
 const stack=String(error?.stack??error);
 if(typeof error!=="object"||error===null)return stack;
 const message=String(error.message??""),code=typeof error.code==="string"?error.code:undefined,project=typeof error.semioProject==="string"?error.semioProject:undefined,lead=[String(error.name??"Error")+(message?": "+message:""),code&&!stack.includes(code)?"code="+code:undefined,project&&!stack.includes(project)?"project="+project:undefined].filter(Boolean);
 return stack.startsWith(lead[0])&&lead.length===1?stack:lead.join(" ")+"\n"+stack;
}

/** 🧯 Turns an unexpected failure of a request into the refusal the client receives. A full disk (`ENOSPC`, `database or disk is full`) is the typed `disk` refusal: when a cache image of the worktree holds the failed path or is itself full, it names that image, its size, fill, largest subtrees and `repo cache gc` (`image-full`); otherwise it names the host partition, its free bytes and the command that frees space; anything else is the `internal` refusal that carries a reference, and the failure itself (with its stack) is written to the service log under that same reference, so the message can say which log file holds the details. @see ../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🆓️free-space/💽️disk-full/🟦️.ts */
export function failureRefusalV1(error,{log,logPath,retry="later",root,platform,home,fullImage=fullImageOfV1,inspect,environment=process.env,pid=process.pid,ppid=process.ppid}){
 const plan=planRefusalOfV1(error);
 if(plan!==undefined){log("graph-plan-too-heavy",JSON.stringify(plan));return{code:"unserved",key:"host-graph-plan-too-heavy",fields:plan,retry:"later"};}
 const heavy=heavyRefusalOfV1(error);
 if(heavy!==undefined){log("graph-owner-too-heavy",JSON.stringify(heavy));return{code:"unserved",key:"host-graph-owner-too-heavy",fields:heavy,retry:"later"};}
 const refused=graphMemoryRefusalOfV1(error);
 if(refused!==undefined){const megabytes=bytes=>String(Math.round(bytes/1048576)),fields={key:refused.detail.key,need:megabytes(refused.detail.needBytes),granted:megabytes(refused.detail.availableBytes),free:megabytes(refused.detail.freeBytes),seconds:String(refused.detail.seconds)};log("graph-memory-refused",JSON.stringify(fields));return{code:"unserved",key:"host-graph-memory-refused",fields,retry:"later"};}
 const wanted=wantedRefusalOfV1(error);
 if(wanted!==undefined){log("graph-wanted-expired",JSON.stringify(wanted));return{code:"unserved",key:"host-graph-wanted-expired",fields:wanted,retry:"later"};}
 const facts=diskFullOfV1(error),image=facts&&root?fullImage(root,{paths:errorPathsV1(error)}):undefined,folder=accessDeniedOfV1(error,{root,platform,home}),detail=failureDetailV1(error),reference=createHash("sha256").update(detail).digest("hex").slice(0,8);
 const place=deniedPlaceOfV1(error),permission=refusedByPermissionV1(error),service=serviceFactsV1({environment,pid,ppid});
 const divergence=folder!==undefined&&!facts?relativeCallDivergenceV1(error,inspect?{inspect}:{}):undefined;
 log(facts?"disk-full":"request-failed",reference+" "+detail+(place||permission?"\nwhere: "+(place??"root=? entry=? code="+(typeof error?.code==="string"?error.code:"EPERM"))+" "+service+(divergence?" pathcheck="+divergence.found:""):""));
 if(image)return{code:"disk",key:"image-full",fields:{image:image.id,directory:image.directory,size:formatDiskBytesV1(image.sizeBytes),used:formatDiskBytesV1(image.usedBytes),free:formatDiskBytesV1(image.freeBytes),host:image.hostFreeBytes===undefined?"?":formatDiskBytesV1(image.hostFreeBytes),filledBy:image.filledBy||"?",command:IMAGE_RECLAIM_COMMAND_V1},retry};
 if(facts)return{code:"disk",key:facts.freeBytes===undefined?"disk-full-unmeasured":"disk-full",fields:{partition:facts.partition,...(facts.freeBytes===undefined?{}:{free:formatDiskBytesV1(facts.freeBytes)}),command:DISK_RECLAIM_COMMAND_V1},retry};
 if(divergence)return{code:"unserved",key:"host-relative-call-refused",fields:{place:divergence.place,reads:String(divergence.reads),found:divergence.found,reference,log:logPath},retry};
 if(folder!==undefined)return{code:"consent",key:"access-denied",fields:{folder,reference,log:logPath,place:placePathV1(error)??"?",route:serviceRouteV1(environment),pid:String(pid),ppid:String(ppid),identity:consentIdentityV1({platform:platform??process.platform,home:home??homedir()})?.path??"repo"},retry};
 return{code:"internal",key:"internal",fields:{reference,log:logPath},retry};
}
