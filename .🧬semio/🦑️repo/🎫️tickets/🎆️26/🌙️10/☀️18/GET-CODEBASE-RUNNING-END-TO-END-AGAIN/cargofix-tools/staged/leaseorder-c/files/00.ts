import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.const});

/** 🧾️ Declares exact final Cargo arguments before repository-specific input admission. */
export interface CargoCompilerInputsRequestV1{readonly manifest:string;readonly cwd:string;readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl}
/** ✍️ Runs one compile continuation under a port's compiler writer custody; the caller enters it only after taking its own Cargo leases, so the writer is never held while they are requested. */
export type CargoCompilerWriterV1=<T>(control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>)=>Promise<T>;
/** 📥️ Retains admitted arguments, environment and the actual consumed phase together, with the writer custody a port offers. */
export interface CargoCompilerInputsViewV1{readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl;readonly writer?:CargoCompilerWriterV1}
/** 🧱️ The admitted view a compile continuation receives: `withWriter` enters the offered writer custody, or runs directly when the port offers none, and must be entered when one is offered. */
export interface CargoCompilerInputsActualV1{readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl;withWriter<T>(body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>}
/** 🔌️ Holds domain-specific Cargo configuration through a neutral compiler continuation. */
export interface CargoCompilerInputsPortV1{<T>(request:CargoCompilerInputsRequestV1,body:(actual:CargoCompilerInputsViewV1)=>Promise<T>):Promise<T>}
/** 🔐️ Requires genuine phase refinement before any admitted compiler continuation executes. */
export function withCargoCompilerInputsV1<T>(request:CargoCompilerInputsRequestV1,port:CargoCompilerInputsPortV1|undefined,body:(actual:CargoCompilerInputsActualV1)=>Promise<T>):Promise<T>{
 try{const control=request.control;requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);return withIssuedResourceOperationV1(control,async()=>{
  let accepting=true,used=0,primary:{error:unknown}|undefined,consumerFailure:{error:unknown}|undefined,result:T|undefined,rejected:Promise<T>|undefined;const pending:Promise<T>[]=[];
  const observe=(task:Promise<T>):Promise<T>=>{void task.then(()=>{},error=>{consumerFailure??={error};});return task;};
  const refuse=(error:unknown):Promise<T>=>{if(rejected)return rejected;try{rejected=withIssuedResourceOperationV1(control,async()=>{throw error;});}catch(failure){rejected=Promise.reject(failure);}return observe(rejected);};
  const consume=(actual:CargoCompilerInputsViewV1):Promise<T>=>{if(!accepting||used>=policy.maximumContinuations)return refuse(Error("One active Cargo input continuation required"));used++;let task:Promise<T>;try{const phase=actual.control,writer=actual.writer;requireIssuedResourceControlRefinementV1(control,phase);const loan=withRequestedIssuedResourceControlV1(phase,0,async bodyControl=>{let entered=false;const value=await body(Object.freeze({arguments:actual.arguments,environment:actual.environment,control:bodyControl,withWriter:<R>(run:(control:ResourceLeaseControl)=>Promise<R>):Promise<R>=>{entered=true;return writer?writer(bodyControl,run):run(bodyControl);}}));checkResourceLeaseControl(bodyControl);if(writer&&!entered)throw Error("Offered Cargo compiler writer custody was never entered");return value;});task=withIssuedResourceOperationV1(control,()=>loan);}catch(error){task=refuse(error);}pending.push(task);return observe(task);};
  try{result=await(port?port(request,consume):consume(request));}catch(error){primary={error};}finally{accepting=false;await Promise.allSettled(pending);if(rejected)await Promise.allSettled([rejected]);}if(primary)throw primary.error;if(consumerFailure)throw consumerFailure.error;if(!used)throw Error("Actual Cargo input continuation required");checkResourceLeaseControl(control);return result as T;
 });}catch(error){const failed=Promise.reject<T>(error);void failed.then(()=>{},()=>{});return failed;}
}
