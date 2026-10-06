/** 📦️ Explicit owned target recipes admit work under caller controls. */
export interface ConformanceLawTarget {readonly kind:"lib"|"test"|"bin";readonly name:string;readonly laws:readonly string[];readonly features:readonly string[];readonly defaultFeatures:boolean;}
export interface ConformanceProvider {readonly schemaVersion:1;readonly targets:readonly ConformanceLawTarget[];}
export interface ConformanceLawGroup {readonly package:string;readonly target:{readonly kind:"lib"|"test"|"bin";readonly name:string;};readonly laws:readonly string[];readonly cargoArgs:readonly string[];}
export interface ConformanceProviderControl {readonly signal:AbortSignal;readonly maximumUnits:number;readonly maximumOwnedBytes:number;readonly onProgress:(completed:number,ownedBytes:number)=>void;readonly yieldContinuation:()=>Promise<void>;}
export class ConformanceProviderRefusal extends Error {constructor(readonly code:"missing-declaration"|"invalid-declaration"|"cancelled"|"budget"){super(code);}}
export class ConformanceProviderWorkspace {readonly targets:ConformanceLawTarget[]=[];readonly groups:ConformanceLawGroup[]=[];readonly lists:string[][]=[];readonly indices:Set<string>[]=[];completed=0;ownedBytes=0;sealed=false;}
function record(value:unknown):value is Record<string,unknown> {return value!==null&&typeof value==="object"&&!Array.isArray(value);}
class Operation {
  private turn=0;
  constructor(readonly workspace:ConformanceProviderWorkspace,readonly control:ConformanceProviderControl){if(!Number.isSafeInteger(control.maximumUnits)||control.maximumUnits<=0||!Number.isSafeInteger(control.maximumOwnedBytes)||control.maximumOwnedBytes<0)throw new ConformanceProviderRefusal("budget");}
  async step(bytes=0):Promise<void>{
    if(this.control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");
    if(this.workspace.ownedBytes+bytes>this.control.maximumOwnedBytes)throw new ConformanceProviderRefusal("budget");
    this.control.onProgress(this.workspace.completed,this.workspace.ownedBytes);
    if(this.control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");
    if(this.turn===this.control.maximumUnits){await this.control.yieldContinuation();this.turn=0;if(this.control.signal.aborted)throw new ConformanceProviderRefusal("cancelled");}
    this.workspace.ownedBytes+=bytes;this.workspace.completed++;this.turn++;
  }
  async keys(value:Record<string,unknown>,expected:readonly string[]):Promise<void>{let count=0;for(const key in value){await this.step();if(!Object.hasOwn(value,key)||!expected.includes(key))throw new ConformanceProviderRefusal("invalid-declaration");count++;}if(count!==expected.length)throw new ConformanceProviderRefusal("invalid-declaration");}
  async text(value:unknown,pattern:RegExp):Promise<string>{if(typeof value!=="string"||value.length===0)throw new ConformanceProviderRefusal("invalid-declaration");for(let index=0;index<value.length;index++)await this.step();if(!pattern.test(value))throw new ConformanceProviderRefusal("invalid-declaration");return value;}
  async strings(value:unknown,nonempty:boolean,pattern:RegExp):Promise<readonly string[]>{
    if(!Array.isArray(value)||(nonempty&&value.length===0))throw new ConformanceProviderRefusal("invalid-declaration");
    await this.step(128);const seen=new Set<string>(),result:string[]=[];this.workspace.indices.push(seen);this.workspace.lists.push(result);
    for(const item of value){const text=await this.text(item,pattern);await this.step(48);if(seen.has(text))throw new ConformanceProviderRefusal("invalid-declaration");seen.add(text);result.push(text);}
    return result;
  }
}
export async function admitConformanceProvider(value:unknown,workspace:ConformanceProviderWorkspace,control:ConformanceProviderControl):Promise<ConformanceProvider>{
  const operation=new Operation(workspace,control);await operation.step();
  if(workspace.sealed||workspace.targets.length||workspace.groups.length)throw new ConformanceProviderRefusal("invalid-declaration");
  if(value===null||value===undefined)throw new ConformanceProviderRefusal("missing-declaration");
  if(!record(value))throw new ConformanceProviderRefusal("invalid-declaration");
  await operation.keys(value,["schema-version","law-targets"]);
  if(value["schema-version"]!==1||!Array.isArray(value["law-targets"])||!value["law-targets"].length)throw new ConformanceProviderRefusal("invalid-declaration");
  for(const target of value["law-targets"]){
    await operation.step();if(!record(target))throw new ConformanceProviderRefusal("invalid-declaration");
    await operation.keys(target,["target-kind","target-name","laws","features","default-features"]);
    const kind=target["target-kind"];if(kind!=="lib"&&kind!=="test"&&kind!=="bin")throw new ConformanceProviderRefusal("invalid-declaration");
    const name=await operation.text(target["target-name"],/^.+$/su),laws=await operation.strings(target.laws,true,/^[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z_][A-Za-z0-9_]*)*$/u),features=await operation.strings(target.features,false,/^[A-Za-z0-9_-]+$/u),defaultFeatures=target["default-features"];
    if(typeof defaultFeatures!=="boolean")throw new ConformanceProviderRefusal("invalid-declaration");
    await operation.step(128);workspace.targets.push({kind,name,laws,features,defaultFeatures});
  }
  await operation.step(32);workspace.sealed=true;return {schemaVersion:1,targets:workspace.targets};
}
export async function conformanceLawGroups(packageName:string,provider:ConformanceProvider,workspace:ConformanceProviderWorkspace,control:ConformanceProviderControl):Promise<readonly ConformanceLawGroup[]>{
  const operation=new Operation(workspace,control);await operation.text(packageName,/^[A-Za-z0-9_-]+$/u);
  if(!workspace.sealed||workspace.groups.length)throw new ConformanceProviderRefusal("invalid-declaration");
  for(const target of provider.targets){
    let featureLength=Math.max(0,target.features.length-1);for(const feature of target.features){for(let index=0;index<feature.length;index++)await operation.step();featureLength+=feature.length;}
    await operation.step(128+featureLength*2);const cargoArgs:string[]=[];
    if(!target.defaultFeatures)cargoArgs.push("--no-default-features");
    if(target.features.length)cargoArgs.push("--features",target.features.join(","));
    workspace.groups.push({package:packageName,target:{kind:target.kind,name:target.name},laws:target.laws,cargoArgs});
  }
  return workspace.groups;
}
