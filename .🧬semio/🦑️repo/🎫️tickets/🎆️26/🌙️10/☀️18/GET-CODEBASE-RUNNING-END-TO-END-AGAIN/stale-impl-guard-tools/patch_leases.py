import sys
sys.path.insert(0,".")
from anchored import patch
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts"
old_trace='''async function traceLeaseV1(options:LeaseOptions,acquire:()=>Promise<ResourceLease>):Promise<ResourceLease>{
  const path=process.env.SEMIO_LEASE_TRACE;
  if(!path)return acquire();
  const startedMs=Date.now(),lease=await acquire(),acquiredMs=Date.now();
  if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\\n");}catch{}
  return lease;
}'''
new_trace='''async function traceLeaseV1(options:LeaseOptions,acquire:()=>Promise<ResourceLease>):Promise<ResourceLease>{
  const path=process.env.SEMIO_LEASE_TRACE;
  if(!path)return admitResourceLeaseV1(options,await acquire());
  const startedMs=Date.now(),lease=await acquire(),acquiredMs=Date.now();
  if(acquiredMs-startedMs>=100)try{appendFileSync(path,JSON.stringify({version:1,resource:options.resource,mode:options.mode,startedMs,acquiredMs,pid:process.pid})+"\\n");}catch{}
  return admitResourceLeaseV1(options,lease);
}
/** 🛡️ Admission guards of the lease layer, registered by name in a process-wide set: a guard receives every granted exclusive lease and every wait round of a queued exclusive request, and a guard that throws makes the holder yield the resource (a granted lease is released before the error leaves, a waiting ticket is withdrawn). */
export type ResourceLeaseAdmissionV1=Readonly<{directory:string;resource:string;mode:"shared"|"exclusive";phase:"admitted"|"waiting"}>;
const admissionGuardRegistryV1=Symbol.for("semio.resourceLeaseAdmissionGuards.v1");
function runAdmissionGuardsV1(options:Pick<LeaseOptions,"directory"|"resource"|"mode">,phase:ResourceLeaseAdmissionV1["phase"]):void{
  const guards=(globalThis as Record<symbol,Set<(admission:ResourceLeaseAdmissionV1)=>void>|undefined>)[admissionGuardRegistryV1];
  if(!guards?.size||options.mode!=="exclusive"||typeof options.resource!=="string")return;
  const admission:ResourceLeaseAdmissionV1=Object.freeze({directory:options.directory,resource:options.resource,mode:options.mode,phase});
  for(const guard of [...guards])guard(admission);
}
function admitResourceLeaseV1(options:Pick<LeaseOptions,"directory"|"resource"|"mode">,lease:ResourceLease):ResourceLease{
  try{runAdmissionGuardsV1(options,"admitted");}catch(error){finishResourceLeaseCleanupV1(true,error,[()=>lease.release()]);throw error;}
  return lease;
}'''
old_wait='''      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info='''
new_wait='''      if (elapsedMs >= nextProgress) {
        runAdmissionGuardsV1(options,"waiting");
        const progress = { resource: options.resource, mode: options.mode, elapsedMs },info='''
old_up='''  guard?.hold(upgraded);return upgraded;
}'''
new_up='''  guard?.hold(upgraded);return admitResourceLeaseV1(options,upgraded);
}'''
patch(P,[(old_trace,new_trace),(old_wait,new_wait),(old_up,new_up)])
