import {spawn,type ChildProcess} from "node:child_process";
import {resolve} from "node:path";
import {CargoController} from "../../🎛️control/🟦️.ts";
import type {CargoDiscoveryOperation} from "../../📁️physical/🟦️.ts";
import type {CargoOwnerPreparation,CargoPreparationExecution} from "../🟦️.ts";

export interface CargoPreparationProcessPort {readonly maximumMilliseconds:number;terminate(pid:number):void;forward(bytes:Uint8Array):Promise<void>;}
export class CargoPreparationProcessOwner {
 private child:ChildProcess|null=null;private readonly pending=new Set<Uint8Array>();
 state:"prepared"|"running"|"complete"|"refused"="prepared";failure:unknown=null;terminalFailure:unknown=null;terminationFailure:unknown=null;terminalResult:Readonly<{code:number|null;signal:string|null}>|null=null;private terminalObservation:Promise<void>|null=null;
 constructor(readonly recipe:CargoOwnerPreparation){}
 async execute(root:string,operation:CargoDiscoveryOperation,port:CargoPreparationProcessPort):Promise<void>{
  const control=new CargoController(operation,operation.workspace);if(this.state!=="prepared")throw Error("Cargo preparation process is already admitted");await control.step("preparation-process-arguments",this.recipe.manifest,1,64);const args:string[]=[this.recipe.script];operation.workspace.partialRecords.push(args);for(const argument of this.recipe.command){await control.step("preparation-process-argument",this.recipe.manifest,1,16);args.push(argument);}await control.step("preparation-process-spawn",this.recipe.manifest,1,256);this.child=spawn(process.execPath,args,{cwd:resolve(root,this.recipe.directory),env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_ACTIVE:this.recipe.script},stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32"});this.state="running";const child=this.child;
  const terminal=new Promise<Readonly<{code:number|null;signal:string|null}>>((accept,reject)=>{child.once("error",reject);child.once("close",(code,signal)=>accept({code,signal}));});let stopped:string|null=null;const stop=(reason:string)=>{stopped??=reason;if(child.pid&&child.exitCode===null&&child.signalCode===null)port.terminate(child.pid);},abort=()=>stop("cancelled"),timer=setTimeout(()=>stop("time budget refused"),port.maximumMilliseconds);operation.signal.addEventListener("abort",abort,{once:true});if(operation.signal.aborted)abort();
  const forward=async(stream:NonNullable<ChildProcess["stdout"]>)=>{for await(const bytes of stream){if(!(bytes instanceof Uint8Array))throw Error("Cargo preparation requires binary output");this.pending.add(bytes);await control.step("preparation-process-output",this.recipe.manifest,bytes.byteLength+1,bytes.byteLength+32);await port.forward(bytes);control.check();this.pending.delete(bytes);operation.workspace.ownedBytes-=bytes.byteLength+32;}};
  try{if(!child.stdout||!child.stderr)throw Error("Cargo preparation output owner is absent");const result=await Promise.all([terminal,forward(child.stdout),forward(child.stderr)]);if(stopped||result[0].signal||result[0].code!==0)throw Error("Cargo owner preparation failed: "+(stopped??result[0].signal??String(result[0].code)));await control.finish("preparation-process-complete",this.recipe.manifest);this.state="complete";}catch(error){this.state="refused";this.failure=error;try{stop("refused");}catch(terminationFailure){this.terminationFailure=terminationFailure;}if(this.terminationFailure!==null){this.terminalObservation=terminal.then(result=>{this.terminalResult=result;},terminalFailure=>{this.terminalFailure=terminalFailure;});}else try{this.terminalResult=await terminal;}catch(terminalFailure){this.terminalFailure=terminalFailure;}throw error;}finally{clearTimeout(timer);operation.signal.removeEventListener("abort",abort);}
 }
}

/** 🏃️ Retains each actual subprocess before execution and forwards output through the caller's owned port. */
export class CargoPreparationExecutor implements CargoPreparationExecution {
 readonly processes:CargoPreparationProcessOwner[]=[];
 constructor(readonly root:string,private readonly port:CargoPreparationProcessPort){if(!Number.isSafeInteger(port.maximumMilliseconds)||port.maximumMilliseconds<=0)throw Error("Cargo preparation requires a finite process time budget");}
 async execute(recipe:CargoOwnerPreparation,operation:CargoDiscoveryOperation):Promise<void>{const control=new CargoController(operation,operation.workspace);await control.step("preparation-process-owner",recipe.manifest,1,128);const owner=new CargoPreparationProcessOwner(recipe);this.processes.push(owner);operation.workspace.partialRecords.push(owner);await owner.execute(this.root,operation,this.port);}
}
