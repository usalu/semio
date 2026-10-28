/** 🧪️ Graph-free cost of a graph-memo HIT, step by step, read-only against the live memo record and journal: `bun memohit.ts <root> <memo-dir> <record.json>`.
 * Replays exactly the steps of `openGraphMemoV1().lookup()` (record, pointer, journal answer, relevance, recheck parse, identities, engine re-check, plan read + sha256) and then the executor's `readBuildGraphV1(JSON.parse(plan))`, printing wall, CPU and RSS after each step. Nothing is written. */
import {readFileSync,statSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
const [root,directory,recordFile]=process.argv.slice(2) as [string,string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const memo=await import(join(L,"🧭️invocation/🕸️graph/🧠️memo/🟦️.ts")),journal=await import(join(L,"🏡️resident/📜️journal/🟦️.ts")),endpoint=await import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs")),observation=await import(join(L,"👁️observation/🟨️.mjs")),graphs=await import(join(L,"🕸️graph/🟦️.ts")),changes=await import(join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts"));
const t0=performance.now(),c0=process.cpuUsage();let tl=t0,cl=c0;
const step=(name:string,detail:unknown="")=>{const t=performance.now(),c=process.cpuUsage(),mb=(n:number)=>Math.round(n/1048576);console.log(JSON.stringify({step:name,wallMs:Math.round(t-tl),cpuMs:Math.round((c.user-cl.user+c.system-cl.system)/1000),rssMb:mb(process.memoryUsage().rss),heapMb:mb(process.memoryUsage().heapUsed),detail}));tl=t;cl=c;};
step("modules");
const record=JSON.parse(readFileSync(recordFile,"utf8"));step("record",record.state);
const pointer=JSON.parse(readFileSync(record.pointer.file,"utf8"));step("pointer",pointer.generation===record.pointer.generation);
const canonical=endpoint.canonicalRootV1(root),client=journal.connectChangeJournalV1(canonical,process.env);
const answer=await client.since(changes.parseChangeCursorV1(record.cursor),[""]);step("journal",answer.exact?{rows:answer.rows.length}:answer.reason);
const rel=readFileSync(join(directory,record.relevance.id+".rel")),chk=readFileSync(join(directory,record.relevance.id+".chk"));
const id=createHash("sha256").update(Buffer.concat([rel,Buffer.from("\n"),chk])).digest("hex"),relevance=memo.unpackMemoRelevanceV1(rel);step("relevance",{bytes:rel.length+chk.length,idOk:id===record.relevance.id});
const changed=answer.exact?memo.firstRelevantChangeV1(relevance,answer.rows):undefined;step("relevant-change",changed?changed[2]:null);
const recheck=JSON.parse(chk.toString("utf8"));step("recheck-parse",{rows:recheck.rows.length,identities:recheck.identities.length});
const identities=recheck.identities.every((identity:any)=>memo.memoIdentityHoldsV1(root,identity));step("identities",identities);
const holds=await observation.observationsHold(recheck.rows,64,new Set(),true);step("engine-recheck",holds);
const plan=readFileSync(join(directory,record.plan.sha256+".plan")),planOk=createHash("sha256").update(plan).digest("hex")===record.plan.sha256;step("plan-read",{bytes:plan.length,planOk});
const text=plan.toString("utf8");step("plan-decode");
const graph=graphs.readBuildGraphV1(JSON.parse(text));step("plan-parse",{projects:Object.keys(graph.projects).length});
const c=process.cpuUsage();console.log(JSON.stringify({total:{wallMs:Math.round(performance.now()-t0),cpuMs:Math.round((c.user-c0.user+c.system-c0.system)/1000),rssMb:Math.round(process.memoryUsage().rss/1048576)},sizes:{plan:statSync(join(directory,record.plan.sha256+".plan")).size}}));
process.exit(0);
