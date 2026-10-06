import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {createRequire} from "node:module";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {decodeRuntimeMutationInventory} from "../🟦️.ts";
import {JsonSyntaxWorkspace} from "../../../../../🎒️pack/🔤️json/📥️decode/🟦️.ts";
const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../../../🧬️schema/🔣️.json"),"utf8")),corpus=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
const operation=()=>({maximumBytes:1000000,maximumNodes:10000,maximumDepth:256,maximumUnits:1000000,maximumOwnedBytes:16000000,workspace:new JsonSyntaxWorkspace(),chunk:128,cancelled:()=>false,progress:async(_value:unknown)=>{},yield:async()=>{}});
test("neutral receiver cases agree with independent AJV and SQLite JSON ownership",async()=>{
 const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,$defs:schema.$defs,$ref:"#/$defs/RuntimeMutationInventory"}),database=new Database(":memory:");try{for(const row of corpus.cases){const original=JSON.parse(row.source);expect(validate(original)).toBe(row.accepted);const control=operation();if(row.accepted){const decoded=await decodeRuntimeMutationInventory(row.source,control);expect(decoded).toEqual(original);const result=database.query("select json_extract(?,'$.artifact') artifact,json_array_length(?,'$.mutations') mutations").get(row.source,row.source);expect(result).toEqual({artifact:decoded.artifact,mutations:decoded.mutations.length});expect(control.workspace.owners).toContain(decoded);}else{await expect(decodeRuntimeMutationInventory(row.source,control)).rejects.toThrow();expect(control.workspace.refusals).toHaveLength(1);}expect(control.workspace.sources).toEqual([row.source]);}}finally{database.close();}
});
test("every actual parse or typed projection callback revokes without losing partial owners",async()=>{
 const source=corpus.cases[1].source,complete=operation();let callbacks=0;complete.progress=async()=>{callbacks++;};await decodeRuntimeMutationInventory(source,complete);expect(callbacks).toBeGreaterThan(50);
 for(let frontier=1;frontier<=callbacks;frontier++){const control=operation();let count=0,cancelled=false,prefix:unknown[]=[];control.cancelled=()=>cancelled;control.progress=async()=>{if(++count===frontier){prefix=[...control.workspace.owners];cancelled=true;}};await expect(decodeRuntimeMutationInventory(source,control)).rejects.toThrow();expect(control.workspace.owners.length).toBe(prefix.length);for(let i=0;i<prefix.length;i++)expect(control.workspace.owners[i]).toBe(prefix[i]);expect(control.workspace.refusals).toHaveLength(1);expect(control.workspace.active).toBe(false);}
});
test("unit and owned byte refusals happen before accepting a runtime record",async()=>{
 for(const field of ["maximumUnits","maximumOwnedBytes"] as const){const control=operation();control[field]=1;await expect(decodeRuntimeMutationInventory(corpus.cases[0].source,control)).rejects.toThrow();expect(control.workspace.results).toHaveLength(0);expect(control.workspace.refusals).toHaveLength(1);}
});
test("typed receiver uses only declared first-party API types and syntax owner",()=>{
 const require=createRequire(import.meta.url),ts=require("typescript"),path=resolve(import.meta.dir,"../🟦️.ts"),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true);expect(tree.parseDiagnostics).toEqual([]);const program=ts.createProgram([path],{noEmit:true,strict:true,skipLibCheck:true,target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true,types:["node","bun"]});expect(ts.getPreEmitDiagnostics(program).map((value:any)=>ts.flattenDiagnosticMessageText(value.messageText,"\n"))).toEqual([]);
});
