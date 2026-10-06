import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../../🧬️schema/🔎️discovery/🔣️.json";
import corpus from "../../🧫️fixtures/🔎️discovery/🔣️.json";
import {discoverConformanceTargets,ConformanceDiscoveryWorkspace,type ConformanceContribution} from "../🟦️.ts";
import {ConformanceProviderRefusal} from "../../📦️providers/🟦️.ts";

async function* inputs(values:readonly ConformanceContribution[]):AsyncIterable<ConformanceContribution>{for(const value of values)yield value;}
async function run(values:readonly ConformanceContribution[],cancelAt=Infinity,maximumOwnedBytes=1_000_000){const controller=new AbortController(),workspace=new ConformanceDiscoveryWorkspace(),events:number[]=[];let outcome="ready";try{await discoverConformanceTargets(inputs(values),workspace,{signal:controller.signal,maximumUnits:3,maximumOwnedBytes,onProgress:(completed)=>{events.push(completed);if(events.length===cancelAt)controller.abort();},yieldContinuation:async()=>{await Promise.resolve();}});}catch(error){if(!(error instanceof ConformanceProviderRefusal))throw error;outcome=error.code;}return{workspace,outcome,events};}

test("closed current owner corpus agrees with independent SQLite discovery",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(validate({...corpus,softSkip:true})).toBe(false);
 const db=new Database(":memory:");try{for(const row of corpus.cases){const actual=await run(row.inputs);expect(actual.outcome).toBe(row.outcome);expect(actual.workspace.groups.map(group=>group.package)).toEqual(row.packages);if(row.outcome==="ready"){const oracle=db.query("SELECT json_extract(value,'$.name') AS name FROM json_each(?1,'$.inputs') WHERE json_extract(value,'$.eligible') ORDER BY CAST(key AS INTEGER)").all(JSON.stringify(row)) as {name:string}[];expect(actual.workspace.groups.map(group=>group.package)).toEqual(oracle.map(value=>value.name));expect(actual.workspace.sealed).toBe(true);}}}finally{db.close();}
});
test("every callback cancellation preserves an owned partial prefix",async()=>{
 for(const row of corpus.cases.filter(row=>row.outcome==="ready")){const complete=await run(row.inputs);for(let at=1;at<=complete.events.length;at++){const stopped=await run(row.inputs,at);expect(stopped.outcome).toBe("cancelled");expect(stopped.workspace.sealed).toBe(false);expect(stopped.workspace.groups).toEqual(complete.workspace.groups.slice(0,stopped.workspace.groups.length));expect(stopped.events).toEqual(complete.events.slice(0,at));expect(stopped.workspace.contributions.length).toBeLessThanOrEqual(complete.workspace.contributions.length);}}
});
test("zero backing budget refuses before contribution allocation",async()=>{const value=await run(corpus.cases[1]!.inputs,Infinity,0);expect(value.outcome).toBe("budget");expect(value.workspace.contributions).toEqual([]);expect(value.workspace.groups).toEqual([]);expect(value.workspace.ownedBytes).toBe(0);});
