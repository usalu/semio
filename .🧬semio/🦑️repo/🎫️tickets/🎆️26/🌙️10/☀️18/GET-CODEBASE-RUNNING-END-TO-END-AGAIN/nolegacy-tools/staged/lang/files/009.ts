import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};

const source=readFileSync(join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),start=source.indexOf("async function retireManagedToolingGenerationsOwnedV1"),end=source.indexOf("/** 🔧️ Drains every registered original writer"),owned=source.slice(start,end);
const identity=(seed:number)=>seed.toString(16).padStart(64,"0");

test("a count-bound retirement only changes which generations are tried and when to stop: every removal still passes the same guards in the same order",()=>{
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);
 const order=["requireRepositoryCacheStoreMaintenanceV1(store,workspace,\"tools\")","requireResourceLeaseCustodyV1(registry","const candidates:string[]=[]","for(const identity of candidates)","if(identity===basename(current)){kept.push(identity);continue;}","resource:protocol.owner+identity,mode:\"exclusive\"}],control,signal:control.signal,timeoutMs:0","nxToolingInstallIdentityV1(","authenticated=originalIdentity===identity","quiescentManagedToolingRetirementAllowedV1({...observation,imageExclusive:true})","if(copied&&!quiescent)return;","await rm(directory,{recursive:true})","retired.push(identity);"].map(token=>owned.indexOf(token));
 expect(order.every(position=>position>=0),JSON.stringify(order)).toBe(true);
 expect([...order].sort((left,right)=>left-right)).toEqual(order);
 const known=["selection?:ToolingRetirementSelectionV1","if(!selection&&await available()>=request.minimumAvailableBytes)return{retired,held,kept,availableBytes:await available()};","if(selection){if(selection.identities.length>protocol.maximumCandidates||new Set(selection.identities).size!==selection.identities.length||selection.identities.some(row=>!/^[a-f0-9]{64}$/u.test(row)))throw refuseBootstrapV1(\"invalid\",\"tools.selection-bounded\",{maximum:protocol.maximumCandidates});candidates.push(...selection.identities);}else{","if(selection?.limit!==undefined&&retired.length>=selection.limit)break;await selection?.progress?.(identity,reached++,candidates.length);if(!selection&&await available()>=request.minimumAvailableBytes)break;","if(selection&&(error as NodeJS.ErrnoException).code===\"ENOENT\")return undefined;"];
 let rest=owned;for(const snippet of known){expect(rest).toContain(snippet);rest=rest.replace(snippet,"");}
 expect(rest).not.toContain("selection");
 expect(owned.match(/held\.push\(identity\)/gu)?.length).toBe(1);
 expect(owned.indexOf("held.push(identity)")).toBeGreaterThan(owned.indexOf("Resource lease timed out: \"+protocol.owner+identity"));
 expect(owned.match(/retired\.push\(identity\)/gu)?.length).toBe(1);
 expect(owned).toContain("if(!retired.includes(identity))kept.push(identity);");
 expect(owned).not.toMatch(/selection[^;]{0,40}(quiescent|authenticated|exclusive)\s*=/u);
});

test("the headroom recovery lists every generation, oldest first, instead of failing above 64 of them",()=>{
 expect(schema.definitions.Protocol.const.maximumCandidates).toBe(1024);expect(schema.definitions.SelectedIntent.properties.identities.maxItems).toBe(64);
 expect(owned).toContain("candidates.sort((left,right)=>born.get(left)!-born.get(right)!||(left<right?-1:left>right?1:0))");
 expect(owned).toContain("birthtimeMs>0?row.birthtimeMs:row.mtimeMs");
 expect(owned).toContain("if(candidates.length>=protocol.maximumCandidates)throw Error(\"Bounded Original tooling generation census required\")");
 expect(owned).not.toContain("candidates.sort();");
});

test("the count-bound request is a closed, sorted-free set of identities with an optional positive limit, and Ajv agrees with the repository validator",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.CountBoundRequest),{validateJsonSchemaSubset}=await import("../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts");
 const good=[{version:1,mode:"count-bound",identities:[identity(1)]},{version:1,mode:"count-bound",identities:[identity(2),identity(1)],limit:16},{version:1,mode:"count-bound",identities:Array.from({length:1024},(_,index)=>identity(index+1)),limit:1024}];
 const bad:unknown[]=[{version:1,mode:"count-bound",identities:[]},{version:1,mode:"count-bound",identities:[identity(1),identity(1)]},{version:1,mode:"count-bound",identities:["abc"]},{version:1,mode:"count-bound",identities:[identity(10).toUpperCase()]},{version:1,mode:"count-bound",identities:Array.from({length:1025},(_,index)=>identity(index+1))},{version:1,mode:"count-bound",identities:[identity(1)],limit:0},{version:1,mode:"count-bound",identities:[identity(1)],limit:1025},{version:1,mode:"count-bound",identities:[identity(1)],limit:1.5},{version:1,mode:"quiescent-generations",identities:[identity(1)]},{version:2,mode:"count-bound",identities:[identity(1)]},{version:1,mode:"count-bound",identities:[identity(1)],extra:true},{mode:"count-bound",identities:[identity(1)]}];
 for(const row of good){expect(validate(row)).toBe(true);expect(validateJsonSchemaSubset(schema.definitions.CountBoundRequest,row,schema).length).toBe(0);}
 for(const row of bad){expect(validate(row),JSON.stringify(row).slice(0,60)).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.CountBoundRequest,row,schema).length>0,JSON.stringify(row).slice(0,60)).toBe(true);}
});
