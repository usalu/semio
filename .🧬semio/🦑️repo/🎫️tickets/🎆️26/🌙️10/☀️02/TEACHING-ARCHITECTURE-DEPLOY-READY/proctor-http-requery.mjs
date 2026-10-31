import assert from "node:assert/strict";
import {readFileSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {resolve} from "node:path";
const ticket=process.argv[2],previous=JSON.parse(readFileSync(resolve(ticket,"🗑️generated/proctor-http-journey-result.json"),"utf8")),origin="http://127.0.0.1:6061";
async function read(value){const r=await fetch("http://127.0.0.1:8791/queries",{method:"POST",headers:{"Content-Type":"application/json",Origin:origin},body:JSON.stringify({queryId:randomBytes(16).toString("hex"),kind:"course."+value.type,version:10,scope:"architecture",principal:{kind:"anonymous"},arguments:[...Buffer.from(JSON.stringify(value))],consistency:{kind:"authority"},cursor:null}),signal:AbortSignal.timeout(3000)});assert.equal(r.status,200);assert.equal(r.headers.get("access-control-allow-origin"),origin);const b=await r.json();assert.equal(b.kind,"snapshot");return JSON.parse(Buffer.from(b.value).toString("utf8"));}
for(const row of previous.runs){const run=await read({type:"run",run:row.id});assert.equal(run.status,"submitted");assert.equal(run.result.score,1);assert.equal(run.result.points,row.points);}
const learner=await read({type:"learner",learner:previous.learner});assert.equal(learner.runs.length,4);assert.equal(learner.total,400);
const out={at:new Date().toISOString(),independentClientProcess:true,submittedRuns:4,authorityReads:5,bestPoints:learner.total,cors:origin};writeFileSync(resolve(ticket,"🗑️generated/proctor-http-journey-requery.json"),JSON.stringify(out,null,2)+"\n");console.log(JSON.stringify(out));
