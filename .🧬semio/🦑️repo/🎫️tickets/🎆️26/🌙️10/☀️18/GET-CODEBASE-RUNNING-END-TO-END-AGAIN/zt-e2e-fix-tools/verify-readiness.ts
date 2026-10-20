/** ✅️ Replays the readiness-budget and progress assertions of the service-readiness law against a repo root (default: cwd) without the vite parts. */
import assert from "node:assert/strict";
import {createRequire} from "node:module";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
const workspace=resolve(process.argv[2]??process.cwd()),directory=join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session");
const api=await import(join(directory,"🟦️.ts")),fixture=JSON.parse(readFileSync(join(directory,"🧫️cases.json"),"utf8")),schema=JSON.parse(readFileSync(join(directory,"🧬️schema/🔣️.json"),"utf8"));
const ajv=new (createRequire("/Users/ueli/Documents/semio/package.json")("ajv").default)();ajv.addSchema(schema);
const root=mkdtempSync(join(realpathSync(tmpdir()),"zt-ready-"));
try{
  const budgetSchema={$ref:schema.$id+"#/$defs/ReadinessBudget"},budgets:any[]=[];
  for(const row of fixture.readiness.products){const budget=JSON.parse(readFileSync(join(workspace,row.budget),"utf8"));budgets.push(budget);assert.ok(ajv.validate(budgetSchema,budget),`${row.budget}: ${ajv.errorsText()}`);assert.deepEqual(api.parseServiceReadinessBudgetV1(budget),budget);assert.ok(budget.readyTimeoutMs>=budget.evidence.coldBootMs.maximum*budget.evidence.marginFactor);assert.ok(budget.readyTimeoutMs>=row.minimumTimeoutMs);}
  for(const row of fixture.readiness.invalid){const value=structuredClone(budgets[0]);let target=value;for(const key of row.path.slice(0,-1))target=target[key];if(row.remove)delete target[row.path.at(-1)];else target[row.path.at(-1)]=row.value;assert.equal(ajv.validate(budgetSchema,value),row.schema,row.reason);assert.throws(()=>api.parseServiceReadinessBudgetV1(value),/readiness budget/i,row.reason);}
  const waiting=join(root,"progress"),idle=await api.openServiceSession(waiting,fixture.owner,fixture.pids[0]),progress=fixture.readiness.progress,events:any[]=[];
  await assert.rejects(()=>api.waitForServiceReady(waiting,idle,new AbortController().signal,progress.timeoutMs,{intervalMs:progress.intervalMs,report:(event:any)=>events.push(event)}),new RegExp(`within ${progress.timeoutMs} ms`));
  assert.ok(events.length>=progress.minimumEvents,`progress events: ${events.length}`);assert.ok(events[0].elapsedMs<progress.intervalMs);
  assert.ok(events.every((event,index)=>event.timeoutMs===progress.timeoutMs&&event.phase==="awaiting-ready-record"&&(index===0||event.elapsedMs>=events[index-1].elapsedMs+progress.intervalMs-5)));
  await assert.rejects(()=>api.waitForServiceReady(waiting,idle,new AbortController().signal,0),/declared positive timeout/i);
  for(const row of progress.lines)for(const language of ["en","de"])assert.equal(api.serviceReadyProgressLine(row.owner,row.progress,language),row[language]);
  for(const row of progress.languages)assert.equal(api.serviceReadyProgressLanguage(row.environment),row.language,JSON.stringify(row.environment));
  console.log(JSON.stringify({ok:true,events:events.length,products:budgets.length,invalidRows:fixture.readiness.invalid.length}));
}finally{rmSync(root,{recursive:true,force:true});}
