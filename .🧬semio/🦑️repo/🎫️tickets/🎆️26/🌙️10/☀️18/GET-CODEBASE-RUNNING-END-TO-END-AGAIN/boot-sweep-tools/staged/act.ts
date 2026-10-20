import{lstatSync,rmSync,rmdirSync,unlinkSync}from"node:fs";
import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";
import{acquireResourceLease,retireResourceQueueClaimsV1,resourceLeasePathV1,type ResourceLeaseControl}from"../../../../../../../🔒️leases/🟦️.ts";
import ticketSchema from"../../../../../../../🔒️leases/🧬️schema/📜️ticket.json" with{type:"json"};
import{flushNativeDirectoryV1}from"../../../../📁️directory/🟦️.ts";
import{observeNativeBootTargetsV1,type NativeBootClockV1,type NativeBootTargetV1}from"../📡️observe/🟦️.ts";
import type{NativeBootFactV1,NativeBootObservationV1,NativeBootStepV1}from"../📋️plan/🟦️.ts";
import type{NativeBootPortV1}from"../🟦️.ts";
const custody=ticketSchema.definitions.ClaimCustodyPolicy.properties,claimTurns=64;

/** 🧯️ Supplies the four effects that need kernel authority rather than a plain file operation. */
export interface NativeBootEffectsV1{
 retireTicket(scope:string,queue:string,name:string):Promise<"done"|"skipped">;
 retireSettledClaims(scope:string):Promise<"done"|"skipped">;
 recoverDirectorySwap(scope:string):Promise<"done"|"skipped">;
 detachImage(scope:string,key:string):Promise<"done"|"skipped">;
}

function confine(root:string,target:string):string{
 if(!isAbsolute(root)||resolve(root)!==root)throw Error("Canonical recovery root required");
 const path=resolve(root,target),inside=relative(root,path);
 if(inside===""||inside.startsWith("..")||isAbsolute(inside)||inside.split(sep).includes(".."))throw Error("Recovery target must stay inside its root");
 for(let parent=dirname(path);parent!==root;parent=dirname(parent)){const row=lstatSync(parent);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Recovery target parents must be ordinary directories");}
 return path;
}
function removeFile(root:string,target:string):"done"|"skipped"{
 const path=confine(root,target);let row;try{row=lstatSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return"skipped";throw error;}
 if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1)throw Error("Recovery removes only ordinary single-link files");
 unlinkSync(path);flushNativeDirectoryV1(dirname(path));return"done";
}
function removeDirectory(root:string,target:string):"done"|"skipped"{
 const path=confine(root,target);
 try{rmdirSync(path);}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")return"skipped";throw error;}
 flushNativeDirectoryV1(dirname(path));return"done";
}

/** 🧲️ Executes plain file effects inside the target's own root and delegates kernel-authority effects. */
export function createNativeBootFsPortV1(input:Readonly<{clock:NativeBootClockV1;targets:readonly NativeBootTargetV1[];effects:NativeBootEffectsV1;close?:()=>Promise<void>}>):NativeBootPortV1&{close():Promise<void>}{
 const roots=new Map<string,string>();
 for(const target of input.targets)roots.set(target.scope,"root"in target?target.root:"directory"in target?target.directory:"workspace"in target?target.workspace:dirname(target.final));
 const observe=(targets:readonly NativeBootTargetV1[])=>observeNativeBootTargetsV1(input.clock,targets);
 return Object.freeze({
  observe:():Promise<NativeBootObservationV1>=>observe(input.targets),
  observeScope:async(scope:string):Promise<readonly NativeBootFactV1[]>=>(await observe(input.targets.filter(target=>target.scope===scope))).facts,
  execute:async(step:NativeBootStepV1):Promise<"done"|"skipped">=>{
   const root=roots.get(step.scope);if(root===undefined)throw Error("Known recovery scope required");
   switch(step.action){
    case"remove-file":return removeFile(root,step.target);
    case"remove-directory":return removeDirectory(root,step.target);
    case"retire-ticket":{const[queue,name]=step.target.split("/");if(!queue||!name||step.target.split("/").length!==2)throw Error("Queue ticket target required");return input.effects.retireTicket(step.scope,queue,name);}
    case"retire-settled-claims":return input.effects.retireSettledClaims(step.scope);
    case"recover-directory-swap":return input.effects.recoverDirectorySwap(step.scope);
    case"detach-image":return input.effects.detachImage(step.scope,step.target);
   }
  },
  close:async():Promise<void>=>{await input.close?.();}
 });
}

/** 🎟️ Retires queue tickets and settled claims with the repository's own kernel proof: an exclusive claim database proves its requester gone. */
export function createNativeBootLeaseEffectsV1(input:Readonly<{directories:ReadonlyMap<string,string>;control:ResourceLeaseControl;swap?:(scope:string)=>"done"|"skipped";detach?:(scope:string,key:string)=>Promise<"done"|"skipped">}>):NativeBootEffectsV1{
 const directoryOf=(scope:string)=>{const directory=input.directories.get(scope);if(directory===undefined)throw Error("Known lease catalog scope required");return directory;};
 return Object.freeze({
  async retireTicket(scope:string,queue:string,name:string):Promise<"done"|"skipped">{
   const directory=directoryOf(scope),stem=queue.slice(0,-".queue".length),claims=join(directory,custody.claimDirectory.const),resource=custody.resourcePrefix.const+stem+":"+name;
   resourceLeasePathV1(claims,resource);const ticket=join(directory,queue,name);
   let lease;try{lease=await acquireResourceLease({directory:claims,resource,mode:"exclusive",signal:input.control.signal,control:input.control,timeoutMs:0,maximumDatabaseBytes:custody.maximumDatabaseBytes.const});}
   catch(error){if(String(error).includes("Resource lease timed out"))return"skipped";throw error;}
   try{const row=lstatSync(ticket,{throwIfNoEntry:false});if(!row)return"skipped";if(!row.isFile()||row.isSymbolicLink())throw Error("Recovery removes only ordinary queue tickets");rmSync(ticket,{force:true});flushNativeDirectoryV1(join(directory,queue));return"done";}
   finally{lease.release();}
  },
  async retireSettledClaims(scope:string):Promise<"done"|"skipped">{let total=0;for(let turn=0;turn<claimTurns;turn++){input.control.signal.throwIfAborted();const retired=(await retireResourceQueueClaimsV1(directoryOf(scope),input.control,0)).retired;total+=retired;if(!retired)break;}return total>0?"done":"skipped";},
  async recoverDirectorySwap(scope:string):Promise<"done"|"skipped">{if(!input.swap)throw Error("Directory swap recovery unavailable");return input.swap(scope);},
  async detachImage(scope:string,key:string):Promise<"done"|"skipped">{if(!input.detach)throw Error("Image detach unavailable");return input.detach(scope,key);}
 });
}
