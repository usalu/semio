import {admitConformanceProvider,conformanceLawGroups,ConformanceProviderRefusal,ConformanceProviderWorkspace,type ConformanceLawGroup,type ConformanceProviderControl} from "../📦️providers/🟦️.ts";

/** 🔎️ Current owner inputs carry explicit eligibility independently of domain selection. */
export interface ConformanceContribution {readonly eligible:boolean;readonly name:string;readonly manifest:string;readonly declaration:unknown;}
export interface ConformanceContributionWorkspace {readonly input:ConformanceContribution;readonly provider:ConformanceProviderWorkspace;}
export class ConformanceDiscoveryWorkspace {
 readonly contributions:ConformanceContributionWorkspace[]=[];
 readonly names=new Set<string>();
 readonly manifests=new Set<string>();
 readonly groups:ConformanceLawGroup[]=[];
 iterator:AsyncIterator<ConformanceContribution>|null=null;
 completed=0;ownedBytes=0;sealed=false;
}
/** 🧭️ Admission conserves every partial owner workspace across refusal and cancellation. */
export async function discoverConformanceTargets(inputs:AsyncIterable<ConformanceContribution>,workspace:ConformanceDiscoveryWorkspace,control:ConformanceProviderControl):Promise<readonly ConformanceLawGroup[]> {
 if(workspace.sealed||workspace.contributions.length||workspace.groups.length)throw new ConformanceProviderRefusal("invalid-declaration");
 if(!Number.isSafeInteger(control.maximumUnits)||control.maximumUnits<=0||!Number.isSafeInteger(control.maximumOwnedBytes)||control.maximumOwnedBytes<0)throw new ConformanceProviderRefusal("budget");
 let turn=0;
 const step=async(bytes=0):Promise<void>=>{
  if(control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");
  if(workspace.ownedBytes+bytes>control.maximumOwnedBytes)throw new ConformanceProviderRefusal("budget");
  control.onProgress(workspace.completed,workspace.ownedBytes);
  if(control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");
  if(turn===control.maximumUnits){await control.yieldContinuation();turn=0;if(control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");}
  workspace.completed++;workspace.ownedBytes+=bytes;turn++;
 };
 const iterator=inputs[Symbol.asyncIterator]();workspace.iterator=iterator;
 for(;;){
  await step();const next=await iterator.next();if(control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");if(next.done)break;
  const input=next.value;if(typeof input.eligible!=="boolean")throw new ConformanceProviderRefusal("invalid-declaration");if(!input.eligible)continue;
  if(typeof input.name!=="string"||!input.name.length||typeof input.manifest!=="string"||!input.manifest.length)throw new ConformanceProviderRefusal("invalid-declaration");
  for(let index=0;index<input.name.length+input.manifest.length;index++)await step();
  await step(224);if(workspace.names.has(input.name)||workspace.manifests.has(input.manifest))throw new ConformanceProviderRefusal("invalid-declaration");
  workspace.names.add(input.name);workspace.manifests.add(input.manifest);const provider=new ConformanceProviderWorkspace();workspace.contributions.push({input,provider});
  const completed=workspace.completed,bytes=workspace.ownedBytes;
  const nested:ConformanceProviderControl={...control,maximumOwnedBytes:control.maximumOwnedBytes-bytes,onProgress:(count,owned)=>{workspace.completed=completed+count;workspace.ownedBytes=bytes+owned;control.onProgress(workspace.completed,workspace.ownedBytes);}};
  let groups:readonly ConformanceLawGroup[];
  try{const admitted=await admitConformanceProvider(input.declaration,provider,nested);groups=await conformanceLawGroups(input.name,admitted,provider,nested);}
  finally{workspace.completed=completed+provider.completed;workspace.ownedBytes=bytes+provider.ownedBytes;}
  for(const group of groups){await step(8);workspace.groups.push(group);}
 }
 await step();workspace.iterator=null;workspace.sealed=true;return workspace.groups;
}
