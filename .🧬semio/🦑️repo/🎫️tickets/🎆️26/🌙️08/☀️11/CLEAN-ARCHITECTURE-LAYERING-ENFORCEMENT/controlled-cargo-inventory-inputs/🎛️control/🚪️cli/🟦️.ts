import {CargoDiscoveryWorkspace,closeCargoDiscoveryOwners,type CargoDiscoveryOperation} from "../../📁️physical/🟦️.ts";
import {CargoController,type CargoRetirementOperation,type CargoProgress} from "../🟦️.ts";

export type CargoOperationLimits=Readonly<{maximumUnits:number;maximumOwnedBytes:number;maximumDepth:number}>;
export type CargoCliPolicy=Readonly<{schemaVersion:1;discovery:CargoOperationLimits;retirement:CargoOperationLimits;progressIntervalMilliseconds:number}>;
export interface CargoCliProgressPort {publish(progress:CargoProgress):void;yieldContinuation():Promise<void>;}
const table=(value:unknown):value is Record<string,unknown>=>Boolean(value)&&typeof value==="object"&&!Array.isArray(value);
function limits(value:unknown):CargoOperationLimits{if(!table(value)||Object.keys(value).some(key=>!["maximumUnits","maximumOwnedBytes","maximumDepth"].includes(key)))throw Error("Cargo CLI limits are invalid");const maximumUnits=value.maximumUnits,maximumOwnedBytes=value.maximumOwnedBytes,maximumDepth=value.maximumDepth;if(typeof maximumUnits!=="number"||typeof maximumOwnedBytes!=="number"||typeof maximumDepth!=="number"||![maximumUnits,maximumOwnedBytes,maximumDepth].every(number=>Number.isSafeInteger(number)&&number>=0)||maximumDepth>4096)throw Error("Cargo CLI requires finite explicit limits");return{maximumUnits,maximumOwnedBytes,maximumDepth};}

/** 🎛️ Admits a closed authored policy without implicit budgets or field defaults. */
export function parseCargoCliPolicy(value:unknown):CargoCliPolicy{if(!table(value)||Object.keys(value).some(key=>!["schemaVersion","discovery","retirement","progressIntervalMilliseconds"].includes(key))||value.schemaVersion!==1||typeof value.progressIntervalMilliseconds!=="number"||!Number.isSafeInteger(value.progressIntervalMilliseconds)||value.progressIntervalMilliseconds<0||value.progressIntervalMilliseconds>60000)throw Error("Cargo CLI policy is invalid");return{schemaVersion:1,discovery:limits(value.discovery),retirement:limits(value.retirement),progressIntervalMilliseconds:value.progressIntervalMilliseconds};}

/** 🚪️ Retains the real signal-controlled command workspace and its separately controlled cleanup. */
export class CargoCliOperationOwner {
 readonly generations:CargoDiscoveryWorkspace[]=[];readonly policy:CargoCliPolicy;
 private currentWorkspace:CargoDiscoveryWorkspace;private currentOperation:CargoDiscoveryOperation;
 private readonly discoverySignal=new AbortController();private readonly retirementSignal=new AbortController();private readonly retirement:CargoRetirementOperation;
 private lastProgress=Number.NEGATIVE_INFINITY;private attached=true;private completedBefore=0;private bytesBefore=0;private retirementGeneration=0;
 private readonly onInterrupt=()=>this.discoverySignal.abort();
 constructor(policy:CargoCliPolicy,private readonly progress:CargoCliProgressPort){
  this.policy=parseCargoCliPolicy(policy);this.currentWorkspace=new CargoDiscoveryWorkspace();this.generations.push(this.currentWorkspace);
  this.currentOperation=this.operationFor(this.currentWorkspace);this.retirement={accounting:{completed:0,ownedBytes:0},signal:this.retirementSignal.signal,...this.policy.retirement,onProgress:row=>progress.publish(row),yieldContinuation:()=>progress.yieldContinuation()};process.once("SIGINT",this.onInterrupt);process.once("SIGTERM",this.onInterrupt);
 }
 get workspace():CargoDiscoveryWorkspace{return this.currentWorkspace;}
 get operation():CargoDiscoveryOperation{return this.currentOperation;}
 get retirementCompleted():number{return this.retirement.accounting.completed;}
 private operationFor(workspace:CargoDiscoveryWorkspace):CargoDiscoveryOperation{const completedBefore=this.completedBefore,bytesBefore=this.bytesBefore;return{workspace,signal:this.discoverySignal.signal,maximumUnits:this.policy.discovery.maximumUnits-this.completedBefore,maximumOwnedBytes:this.policy.discovery.maximumOwnedBytes-this.bytesBefore,maximumDepth:this.policy.discovery.maximumDepth,onProgress:row=>this.publish({...row,completed:row.completed+completedBefore,ownedBytes:row.ownedBytes+bytesBefore}),yieldContinuation:()=>this.progress.yieldContinuation()};}
 private publish(row:CargoProgress):void{const now=Date.now();if(now-this.lastProgress>=this.policy.progressIntervalMilliseconds){this.lastProgress=now;this.progress.publish(row);}}
 /** 🆕️ Admits a fresh discovery view after completed physical execution or publication without resetting cumulative budgets. */
 async advanceWorkspace():Promise<CargoDiscoveryOperation>{if(this.currentWorkspace.state!=="published"&&this.currentWorkspace.state!=="prepared")throw Error("Cargo fresh discovery requires its completed physical phase");await new CargoController(this.currentOperation,this.currentWorkspace).step("discovery-generation","",1,512);this.completedBefore+=this.currentWorkspace.completed;this.bytesBefore+=this.currentWorkspace.ownedBytes;const workspace=new CargoDiscoveryWorkspace();this.generations.push(workspace);this.currentWorkspace=workspace;this.currentOperation=this.operationFor(workspace);return this.currentOperation;}
 cancelDiscovery():void{this.discoverySignal.abort();}
 get retirementAborted():boolean{return this.retirementSignal.signal.aborted;}
 cancelRetirement():void{this.retirementSignal.abort();}
 detachSignals():void{if(this.attached){process.removeListener("SIGINT",this.onInterrupt);process.removeListener("SIGTERM",this.onInterrupt);this.attached=false;}}
 async closeSystemOwners():Promise<void>{for(;this.retirementGeneration<this.generations.length;this.retirementGeneration++)await closeCargoDiscoveryOwners(this.generations[this.retirementGeneration]!,this.retirement);this.detachSignals();}
}
