import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import{AsyncLocalStorage}from"node:async_hooks";
import{Database}from"bun:sqlite";
import ts from"typescript";
import Ajv from"ajv";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

type Round={outcome:"value"|"failure"|"conflict";message?:string;code?:string;undefined?:boolean;young?:boolean;holds?:boolean};

/** 🔁️ Extracts the actual settlement owner from the observation source (or from SETTLE_SOURCE for a patched copy) and runs it over the closed decision corpus. */
async function settlementOwner(){
 const source=await readFile(process.env.SETTLE_SOURCE??new URL("../../../../👁️observation/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("observation.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="settleObservedComputationV1");
 if(!node)throw Error("Actual observed settlement owner required");
 return ts.transpileModule(node.getText(tree).replace(/^export\s/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
}

test("Observed settlement stops repeating a failure that returns unchanged and never repeats a demanded restart",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const code=await settlementOwner(),db=new Database(":memory:");
 db.run("create table terminal(scenario text, outcome text, round integer)");
 try{
  for(const scenario of corpus.scenarios){
   const state={consistent:true,holds:true},scopes=new AsyncLocalStorage<Map<string,unknown>>(),bindings={scopes,sourceCaptures:new AsyncLocalStorage(),inferenceValidationScopes:new AsyncLocalStorage(),retainObservationsV1:()=>{},consistentObservationsV1:()=>state.consistent,observationsHold:async()=>state.holds,conflictingInferenceErrorV1:()=>Object.freeze({path:"modeled Source conflict"}),RACY_MS:0},run=new Function(...Object.keys(bindings),code+";return settleObservedComputationV1;")(...Object.values(bindings));
   let calls=0;const events:string[]=[];let settled:string,failure:unknown;
   try{
    await run(async()=>{
     const row=scenario.rounds[calls++] as Round;if(!row)throw Error("Scripted rounds exhausted: "+scenario.name);
     state.consistent=row.outcome!=="conflict";state.holds=row.holds??true;
     if(row.young??true)scopes.getStore()!.set("young",{kind:"file",path:"/modeled/young",fact:{mtimeMs:Date.now()+15,ctimeMs:0}});
     if(row.outcome==="failure"){if(row.undefined)throw undefined;throw Object.assign(Error(row.message),row.code?{code:row.code}:{});}
     return "value";
    },{limit:corpus.limit,onRound:(event:{failure?:unknown;conflict?:string})=>{events.push(Object.hasOwn(event,"failure")?"failure":"conflict");}});
    settled="value";
   }catch(error){failure=error;settled=(error as {path?:string})?.path==="modeled Source conflict"?"conflict":"failure";}
   const expected=scenario.expect;
   expect({name:scenario.name,settled,rounds:calls,events}).toEqual({name:scenario.name,settled:expected.settled,rounds:expected.rounds,events:expected.events});
   if(expected.settled==="failure")expect(expected.failure===null?failure:(failure as Error).message).toBe(expected.failure===null?undefined:expected.failure);
   db.run("insert into terminal values (?,?,?)",[scenario.name,settled,calls]);
  }
  const worst=db.query("select max(round) r from terminal where outcome='failure' and scenario like 'repeated-%' or scenario in ('thrown-undefined-repeats','restart-required-is-never-retried')").get() as {r:number};
  expect(worst.r).toBeLessThanOrEqual(2);
 }finally{db.close();}
});
