import type {ToolRowFactsV1} from "../../🟦️.ts";
import type {DoctorProbeV1} from "../🟦️.ts";

type RegisteredTool=Readonly<{id:string;platforms:readonly string[];pin:Readonly<{state:string}>}>;
type Resolution=Readonly<{command:string|null;pin:Readonly<{version:string|null}>}>;
type ToolRegistryApi=Readonly<{
 TOOL_REGISTRY_V1:Readonly<{tools:readonly RegisteredTool[]}>;
 resolveRegisteredToolV1(entry:RegisteredTool,root:string,options:Readonly<{env:Readonly<Record<string,string|undefined>>}>):Resolution;
 probeToolV1(entry:RegisteredTool,resolution:Resolution,environment:Readonly<Record<string,string|undefined>>,root:string):string|null;
 toolRootV1(directory:string,environment:Readonly<Record<string,string|undefined>>):string;
}>;

/** 📌️ The pin comparison of `repo tools verify`: a pinned tool differs when the found version is neither a prefix nor an extension of the pinned one. */
export function toolPinDriftV1(state:string,found:string|null,pinned:string|null):boolean{
 return state==="pinned"&&found!==null&&pinned!==null&&/^\d+(?:\.\d+){0,2}$/u.test(pinned)&&!pinned.startsWith(found)&&!found.startsWith(pinned);
}

/** 🔧️ The registered tools against their pins: where each is found, its version and whether it drifted; the tools after the probe budget stay unprobed. */
export const toolsProbeV1:DoctorProbeV1=Object.freeze({
 area:"tools",phase:1,
 async read(ports,context){
  const api=await ports.optional("tools") as ToolRegistryApi|undefined;
  if(!api||!api.TOOL_REGISTRY_V1)return {unavailable:"The tool registry is not installed in this checkout"};
  const root=api.toolRootV1(ports.root,ports.environment),rows:ToolRowFactsV1[]=[];
  for(const entry of api.TOOL_REGISTRY_V1.tools){
   if(!entry.platforms.includes(ports.platform))continue;
   if(ports.signal.aborted||ports.now()>=context.deadlineMs){rows.push({id:entry.id,state:"unprobed",pin:null,found:null});continue;}
   const resolution=api.resolveRegisteredToolV1(entry,root,{env:ports.environment});
   if(resolution.command===null){rows.push({id:entry.id,state:"missing",pin:resolution.pin.version,found:null});continue;}
   const found=api.probeToolV1(entry,resolution,ports.environment,root);
   rows.push({id:entry.id,state:toolPinDriftV1(entry.pin.state,found,resolution.pin.version)?"drift":"ok",pin:resolution.pin.version,found});
  }
  return {sections:{tools:{rows}}};
 },
});
