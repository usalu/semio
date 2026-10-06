import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {parseTree,type Node} from "jsonc-parser";
import * as owner from "../../🟦️.ts";
const root=resolve(import.meta.dir,"../.."),corpus=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🪢️ownership/🔣️.json"),"utf8"));
const operation=()=>({maximumBytes:1000000,maximumNodes:10000,maximumDepth:128,maximumUnits:1000000,maximumOwnedBytes:16000000,workspace:new owner.JsonSyntaxWorkspace(),chunk:1,cancelled:()=>false,progress:async(_value:unknown)=>{},yield:async()=>{}});
function oracle(source:string,node:Node):unknown {if(node.type==="object")return {kind:"object",members:node.children!.map(member=>({name:member.children![0]!.value,value:oracle(source,member.children![1]!)}))};if(node.type==="array")return {kind:"array",items:node.children!.map(child=>oracle(source,child))};if(node.type==="number")return {kind:"number",text:source.slice(node.offset,node.offset+node.length)};return node.type==="null"?{kind:"null"}:{kind:node.type,value:node.value};}
test("closed neutral ownership cases retain syntax and refused partial owners",async()=>{
 const schema=JSON.parse(readFileSync(resolve(root,"🔣️.json"),"utf8")),ajv=new Ajv({strict:true}),validate=ajv.compile(schema),bounds=new Ajv({strict:true}).compile({...schema,$ref:"#/$defs/limits"});
 for(const row of corpus.cases){const control=operation();if(row.maximumUnits)control.maximumUnits=row.maximumUnits;if(row.maximumOwnedBytes)control.maximumOwnedBytes=row.maximumOwnedBytes;expect(bounds({maximumBytes:control.maximumBytes,maximumNodes:control.maximumNodes,maximumDepth:control.maximumDepth,maximumUnits:control.maximumUnits,maximumOwnedBytes:control.maximumOwnedBytes,chunk:control.chunk})).toBe(true);
 if(row.error){await expect(owner.decodeJsonSyntax(row.source,row.policy,control)).rejects.toMatchObject({code:row.error});expect(control.workspace.refusals).toHaveLength(1);expect(control.workspace.results).toHaveLength(0);}else{const value=await owner.decodeJsonSyntax(row.source,row.policy,control);expect(value).toEqual(row.expected);expect(validate(value)).toBe(true);expect(control.workspace.results[0]).toBe(value);if(row.policy==="Reject")expect(value).toEqual(oracle(row.source,parseTree(row.source)!));}
 expect(control.workspace.sources).toEqual([row.source]);expect(control.workspace.active).toBe(false);expect(control.workspace.units).toBeLessThanOrEqual(control.maximumUnits);expect(control.workspace.ownedBytes).toBeLessThanOrEqual(control.maximumOwnedBytes);
 }
});
test("each callback revocation preserves exact previous partial owner prefix",async()=>{
 const source='{"names":["alpha","beta"],"nested":{"x":[1,2,3]}}',complete=operation();let callbacks=0;complete.progress=async()=>{callbacks++;};await owner.decodeJsonSyntax(source,"Reject",complete);expect(callbacks).toBeGreaterThan(30);
 for(let frontier=1;frontier<=callbacks;frontier++){const control=operation();let count=0,cancelled=false,owners:unknown[]=[];control.cancelled=()=>cancelled;control.progress=async()=>{if(++count===frontier){owners=[...control.workspace.owners];cancelled=true;}};await expect(owner.decodeJsonSyntax(source,"Reject",control)).rejects.toMatchObject({code:"cancelled"});expect(control.workspace.owners.length).toBe(owners.length);for(let i=0;i<owners.length;i++)expect(control.workspace.owners[i]).toBe(owners[i]);expect(control.workspace.refusals).toHaveLength(1);expect(control.workspace.active).toBe(false);}
});
test("replaced duplicate values remain owned and reused workspaces retain a shared ledger",async()=>{
 const control=operation();await owner.decodeJsonSyntax('{"a":[1],"a":[2]}',"Replace",control);expect(control.workspace.owners.some(value=>value!==null&&typeof value==="object"&&"kind" in value&&value.kind==="number"&&"text" in value&&value.text==="1")).toBe(true);
 const units=control.workspace.units,owned=control.workspace.ownedBytes;await owner.decodeJsonSyntax("false","Reject",control);expect(control.workspace.results).toHaveLength(2);expect(control.workspace.units).toBeGreaterThan(units);expect(control.workspace.ownedBytes).toBeGreaterThan(owned);
});
