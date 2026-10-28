import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.const});

/** 🧾️ Declares exact final Cargo arguments before repository-specific input admission. */
export interface CargoCompilerInputsRequestV1{readonly manifest:string;readonly cwd:string;readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl}
/** 📥️ Retains admitted arguments, environment and the actual consumed phase together. */
export interface CargoCompilerInputsViewV1{readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly control:ResourceLeaseControl}
/** 🔌️ Holds domain-specific Cargo configuration through a neutral compiler continuation. */
export interface CargoCompilerInputsPortV1{<T>(request:CargoCompilerInputsRequestV1,body:(actual:CargoCompilerInputsViewV1)=>Promise<T>):Promise<T>}
/** 🔐️ Requires genuine phase refinement before any admitted compiler continuation executes. */
export function withCargoCompilerInputsV1<T>(request:CargoCompilerInputsRequestV1,port:CargoCompilerInputsPortV1|undefined,body:(actual:CargoCompilerInputsViewV1)=>Promise<T>):Promise<T>{
 try{const control=request.control;requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);return withIssuedResourceOperationV1(control,async()=>{
  let accepting=true,used=0,primary:{error:unknown}|undefined,consumerFailure:{error:unknown}|undefined,result:T|undefined,rejected:Promise<T>|undefined;const pending:Promise<T>[]=[];
  const observe=(task:Promise<T>):Promise<T>=>{void task.then(()=>{},error=>{consumerFailure??={error};});return task;};
  const refuse=(error:unknown):Promise<T>=>{if(rejected)return rejected;try{rejected=withIssuedResourceOperationV1(control,async()=>{throw error;});}catch(failure){rejected=Promise.reject(failure);}return observe(rejected);};
  const consume=(actual:CargoCompilerInputsViewV1):Promise<T>=>{if(!accepting||used>=policy.maximumContinuations)return refuse(Error("One active Cargo input continuation required"));used++;let task:Promise<T>;try{const phase=actual.control;requireIssuedResourceControlRefinementV1(control,phase);const view=Object.freeze({arguments:actual.arguments,environment:actual.environment,control:phase}),loan=withRequestedIssuedResourceControlV1(phase,0,async bodyControl=>{const value=await body({...view,control:bodyControl});checkResourceLeaseControl(bodyControl);return value;});task=withIssuedResourceOperationV1(control,()=>loan);}catch(error){task=refuse(error);}pending.push(task);return observe(task);};
  try{result=await(port?port(request,consume):consume(request));}catch(error){primary={error};}finally{accepting=false;await Promise.allSettled(pending);if(rejected)await Promise.allSettled([rejected]);}if(primary)throw primary.error;if(consumerFailure)throw consumerFailure.error;if(!used)throw Error("Actual Cargo input continuation required");checkResourceLeaseControl(control);return result as T;
 });}catch(error){const failed=Promise.reject<T>(error);void failed.then(()=>{},()=>{});return failed;}
}
