import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {closeBuildOwnerAdmissionV1,observeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1} from "../../../🟦️.ts";

/** 🏃️ Runs one closure in a fresh process over a world kept on disk, so reuse across processes is the only reuse that can happen; prints `{computed,result|error}` as its last line. */
const request=JSON.parse(process.argv[2]!) as {dir:string;world:{owners:any[];catalogProviders:any[]};selection:any;implementation:string;registry:string;scope:string;cancelAfter?:number};
const observation=await import("../../../../../👁️observation/🟨️.mjs") as typeof import("../../../../../👁️observation/🟨️.mjs");
const computed:string[]=[];
const declaration=(name:string)=>JSON.parse(observation.observedFs.readFileSync(join(request.dir,"world",name+".json"),"utf8")) as any;
const factsFor=(name:string,targets:readonly string[],inputs:readonly string[])=>{
 const entry=declaration(name),files=new Set<string>([entry.manifest,...entry.base.files]),dependencies=new Set<string>(entry.base.dependencies),dependencyInputs:Record<string,Set<string>>={},generated:Record<string,string[]>={};
 for(const target of targets){const row=entry.targets[target];for(const file of row.files)files.add(file);for(const dependency of row.dependencies)dependencies.add(dependency);for(const[owner,groups]of Object.entries<string[]>(row.dependencyInputs))for(const group of groups)(dependencyInputs[owner]??=new Set()).add(group);if(row.generatedDependencies.length)generated[target]=[...row.generatedDependencies].sort();}
 for(const group of inputs)for(const file of entry.inputFiles[group]??[])files.add(file);
 return {project:{name,targets:Object.fromEntries(Object.entries<any>(entry.targets).map(([target,row])=>[target,{dependsOn:row.dependsOn}]))},files:[...files].sort(),dependencies:[...dependencies].sort(),...(Object.keys(generated).length?{generatedDependencies:generated}:{}),dependencyInputs:Object.fromEntries(Object.entries(dependencyInputs).map(([owner,groups])=>[owner,[...groups].sort()]))};
};
const registry=readBuildOwnerRegistryV1({version:1,owners:request.world.owners.map(row=>({name:row.name,root:row.root,manifest:row.manifest,packages:[],...(row.nativeRoots?{nativeRoots:row.nativeRoots}:{})})),catalogProviders:request.world.catalogProviders});
const ports={directory:join(request.dir,"data"),implementation:request.implementation,scope:request.scope,registry:request.registry,check:()=>{if(request.cancelAfter!==undefined&&computed.length>=request.cancelAfter)throw Object.assign(Error("cancelled"),{code:"FIXTURE_CANCELLED"});},advance:async()=>{},observe:(options:any,compute:any)=>observation.observeProjectInferenceV1(options,compute)};
try{
 const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(()=>closeBuildOwnerAdmissionV1(request.selection,registry,(owner,targets,inputs)=>observeBuildOwnerAdmissionV1(ports as any,owner,targets,inputs,async()=>{computed.push(owner.name+"|"+targets.join(",")+"|"+inputs.join(","));return factsFor(owner.name,targets,inputs) as any;})),{limit:1}));
 console.log(JSON.stringify({computed,result:captured.value,observations:captured.observations.length}));
}catch(error){console.log(JSON.stringify({computed,error:(error as Error).message}));}
