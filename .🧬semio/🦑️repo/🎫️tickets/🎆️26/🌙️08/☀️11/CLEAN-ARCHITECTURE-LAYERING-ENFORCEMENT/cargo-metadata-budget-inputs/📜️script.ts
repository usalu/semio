import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch="1"]=process.argv.slice(2),require=createRequire(join(root,"package.json")),ts=require("typescript"),out=join(ticket,"🗑️generated/cargo-metadata-budget"),library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",execution=library+"/🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution",budget=execution+"/⏱️budget",sha=(source:string)=>createHash("sha256").update(source).digest("hex");mkdirSync(out,{recursive:true});
const fixture={schemaVersion:1,cases:[{id:"preparation-wait-is-separate",budgetMs:30000,steps:[{phase:"prepare",milliseconds:90000},{phase:"capture",milliseconds:12000},{phase:"prepare",milliseconds:60000},{phase:"capture",milliseconds:17999}],remainingMs:1,refused:false},{id:"aggregate-native-budget-is-preserved",budgetMs:30000,steps:[{phase:"capture",milliseconds:15000},{phase:"prepare",milliseconds:0},{phase:"capture",milliseconds:15000}],remainingMs:0,refused:true},{id:"overrun-is-refused",budgetMs:30000,steps:[{phase:"prepare",milliseconds:100000},{phase:"capture",milliseconds:30001}],remainingMs:0,refused:true},{id:"no-native-work-spends-nothing",budgetMs:30000,steps:[{phase:"prepare",milliseconds:200000}],remainingMs:30000,refused:false},{id:"zero-duration-capture",budgetMs:10,steps:[{phase:"capture",milliseconds:0},{phase:"prepare",milliseconds:100},{phase:"capture",milliseconds:1.25}],remainingMs:8.75,refused:false}],invalidBudgets:[0,-1],invalidCaptures:[-1]};
const schema={$schema:"http://json-schema.org/draft-07/schema#",type:"object",additionalProperties:false,required:["schemaVersion","cases","invalidBudgets","invalidCaptures"],properties:{schemaVersion:{const:1},cases:{type:"array",minItems:1,items:{type:"object",additionalProperties:false,required:["id","budgetMs","steps","remainingMs","refused"],properties:{id:{type:"string",minLength:1},budgetMs:{type:"number",exclusiveMinimum:0},steps:{type:"array",items:{type:"object",additionalProperties:false,required:["phase","milliseconds"],properties:{phase:{enum:["prepare","capture"]},milliseconds:{type:"number",minimum:0}}}},remainingMs:{type:"number",minimum:0},refused:{type:"boolean"}}}},invalidBudgets:{type:"array",items:{type:"number",maximum:0}},invalidCaptures:{type:"array",items:{type:"number",exclusiveMaximum:0}}}};
const api=`/** ⏱️ Preparation and owner waits consume no part of the aggregate native capture allowance. */
export class CargoMetadataCaptureBudget {
  private capturedMs = 0;
  constructor(private readonly maximumMs: number) {
    if (!Number.isFinite(maximumMs) || maximumMs <= 0) throw Error("Invalid Cargo metadata capture budget");
  }
  get remainingMs(): number { return Math.max(0, this.maximumMs - this.capturedMs); }
  charge(milliseconds: number): void {
    if (!Number.isFinite(milliseconds) || milliseconds < 0) throw Error("Invalid Cargo metadata capture duration");
    this.capturedMs += milliseconds;
    if (this.capturedMs >= this.maximumMs) throw Error("Cargo metadata capture budget exhausted");
  }
}
`;
const test=`import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧫️fixtures/🛂️schema/🔣️.json";
import { CargoMetadataCaptureBudget } from "../🟦️.ts";
test("queued preparation leaves the complete aggregate native capture budget intact", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:true})).toBe(false);
  const oracle = new Database(":memory:");
  try {
    oracle.exec("CREATE TABLE duration(phase TEXT NOT NULL, milliseconds REAL NOT NULL)");
    for (const row of fixture.cases) {
      oracle.exec("DELETE FROM duration");
      for (const step of row.steps) oracle.query("INSERT INTO duration VALUES (?, ?)").run(step.phase,step.milliseconds);
      const sum = (oracle.query("SELECT COALESCE(SUM(milliseconds),0) AS elapsed FROM duration WHERE phase='capture'").get() as {elapsed:number}).elapsed;
      const independent = {remainingMs:Math.max(0,row.budgetMs-sum),refused:sum>=row.budgetMs};
      expect(independent,row.id).toEqual({remainingMs:row.remainingMs,refused:row.refused});
      const budget = new CargoMetadataCaptureBudget(row.budgetMs);let refused=false;
      for (const step of row.steps) if(step.phase==="capture") {
        try { budget.charge(step.milliseconds); } catch { refused=true;break; }
      }
      expect({remainingMs:budget.remainingMs,refused},row.id).toEqual(independent);
      console.log("[DEBUG] Cargo capture budget "+JSON.stringify({id:row.id,remainingMs:budget.remainingMs,refused}));
    }
    for (const value of [...fixture.invalidBudgets,NaN,Infinity]) expect(()=>new CargoMetadataCaptureBudget(value)).toThrow();
    for (const value of [...fixture.invalidCaptures,NaN,Infinity]) expect(()=>new CargoMetadataCaptureBudget(30000).charge(value)).toThrow();
  } finally { oracle.close(); }
});
`;
if(command==="stage"){
 const destination=join(out,"source-"+epoch+".json");assert.equal(existsSync(destination),false);const pairs:any[]=[],pair=(path:string,after:string):void=>{const before=existsSync(join(root,path))?readFileSync(join(root,path),"utf8"):null;pairs.push({path,before,after,beforeHash:before===null?null:sha(before),afterHash:sha(after)});};
 pair(budget+"/🟦️.ts",api);pair(budget+"/🧫️fixtures/🔣️.json",JSON.stringify(fixture,null,2)+"\n");pair(budget+"/🧫️fixtures/🛂️schema/🔣️.json",JSON.stringify(schema,null,2)+"\n");pair(budget+"/🧪️tests/🟦️.ts",test);
 const executionPath=execution+"/🟦️.ts",before=readFileSync(join(root,executionPath),"utf8");let after='import { CargoMetadataCaptureBudget } from "./⏱️budget/🟦️.ts";\n'+before;for(const [from,to]of [['const metadata: unknown[] = [], deadline = Date.now() + 30_000;','const metadata: unknown[] = [], budget = new CargoMetadataCaptureBudget(30_000);'],['const remaining = deadline - Date.now();','const remaining = budget.remainingMs;'],['if (remaining <= 0) throw new Error("Cargo direction metadata timeout 30000ms");','if (remaining <= 0) throw new Error("Cargo metadata capture budget exhausted");'],['const child = Bun.spawn(["cargo", ...args],','const captureStarted = performance.now();\n    const child = Bun.spawn(["cargo", ...args],'],['process.off("SIGTERM", terminate); }','process.off("SIGTERM", terminate); budget.charge(performance.now() - captureStarted); }']]as const){assert.equal(after.split(from).length-1,1,from);after=after.replace(from,to);}pair(executionPath,after);
 const originalTest=library+"/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts";pair(originalTest,'import "../../🕸️dependencies/🧭️direction/🦀️cargo/🏃️execution/⏱️budget/🧪️tests/🟦️.ts";\n'+readFileSync(join(root,originalTest),"utf8"));for(const row of pairs)assert.ok((existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null)===row.before,row.path);writeFileSync(destination,JSON.stringify({producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},pairs,fixture,schema,originalCaptureBudgetMs:30000,sourceWrites:0}));console.log("[DEBUG] Cargo capture budget staged "+JSON.stringify({pairs:pairs.length,cases:fixture.cases.length}));
}else if(command==="model"){
 const sourcePath=join(out,"source-"+epoch+".json"),raw=readFileSync(sourcePath,"utf8"),source=JSON.parse(raw),mirror=join(out,"model-"+epoch);assert.equal(existsSync(mirror),false);mkdirSync(mirror,{recursive:true});for(const row of source.pairs){const path=join(mirror,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.after);}
 const actual=Bun.spawnSync([process.execPath,"test",join(mirror,budget,"🧪️tests/🟦️.ts")],{cwd:root,stdout:"pipe",stderr:"pipe"});writeFileSync(join(mirror,"actual.log"),Buffer.concat([actual.stdout,actual.stderr]));const postGaps=source.pairs.filter((row:any)=>(existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null)!==row.before).map((row:any)=>row.path);assert.equal(readFileSync(sourcePath,"utf8"),raw);writeFileSync(join(mirror,"terminal.json"),JSON.stringify({sourceHash:sha(raw),exitCode:actual.exitCode,postGaps,sourceWrites:0,originalCaptureBudgetMs:30000}));console.log(JSON.stringify({exitCode:actual.exitCode,postGaps:postGaps.length}));process.exit(actual.exitCode);
}else throw Error("stage|model");
