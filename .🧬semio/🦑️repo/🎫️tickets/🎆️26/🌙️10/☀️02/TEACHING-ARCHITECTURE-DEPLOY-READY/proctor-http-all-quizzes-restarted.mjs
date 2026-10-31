import assert from "node:assert/strict";
import {readFileSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {resolve} from "node:path";
const ticket=process.argv[2],previous=JSON.parse(readFileSync(resolve(ticket,"🗑️generated/proctor-http-all-quizzes-result.json"),"utf8")),origin="http://127.0.0.1:6061",deadline=Date.now()+120000;
const output=process.argv[3],currentServer=process.argv[4];
assert.equal(typeof output,"string");assert.equal(typeof currentServer,"string");assert(currentServer.length>0);
assert(/^[a-z0-9-]+\.json$/.test(output));
let reads=0;
async function read(value){assert(Date.now()<deadline);const response=await fetch("http://127.0.0.1:8791/queries",{method:"POST",headers:{"Content-Type":"application/json",Origin:origin},body:JSON.stringify({queryId:randomBytes(16).toString("hex"),kind:"course."+value.type,version:10,scope:"architecture",principal:{kind:"anonymous"},arguments:[...Buffer.from(JSON.stringify(value))],consistency:{kind:"authority"},cursor:null}),signal:AbortSignal.timeout(3000)});assert.equal(response.status,200);assert.equal(response.headers.get("access-control-allow-origin"),origin);const body=await response.json();assert.equal(body.kind,"snapshot");reads++;return JSON.parse(Buffer.from(body.value).toString("utf8"));}
const instanceResponse=await fetch("http://127.0.0.1:8791/instance",{signal:AbortSignal.timeout(3000)});assert.equal(instanceResponse.status,200);assert.equal((await instanceResponse.json()).id,"teaching-proctor");
assert.equal(previous.runs.length,40);let answers=0;
for(const expected of previous.runs){const run=await read({type:"run",run:expected.id});assert.equal(run.run,expected.id);assert.equal(run.learner,previous.learner);assert.equal(run.quiz,expected.quiz);assert.equal(run.sheet.quiz,expected.quiz);assert.equal(run.result.quiz,expected.quiz);assert.equal(run.status,"submitted");assert.equal(run.result.score,expected.score);assert.equal(run.result.points,expected.points);assert.equal(run.sheet.challenge,expected.challenge);assert.equal(Object.keys(run.answers).length,expected.tasks);answers+=Object.keys(run.answers).length;}
const learner=await read({type:"learner",learner:previous.learner});assert.equal(learner.learner,previous.learner);assert.equal(learner.runs.length,40);
const roster=rows=>rows.map(row=>({run:row.run,quiz:row.quiz,challenge:row.challenge,status:row.status,score:row.score,points:row.points})).sort((a,b)=>a.run.localeCompare(b.run));
assert.deepEqual(roster(learner.runs),roster(previous.runs.map(row=>({...row,run:row.id,status:"submitted"}))));assert.equal(learner.total,previous.total);assert.deepEqual(learner.best,previous.best);assert.deepEqual(learner.badges.map(row=>row.badge).sort(),previous.badges);
const board=await read({type:"leaderboard",period:"all-time",learner:previous.learner});assert(board.own);assert.equal(board.own.total,4000);assert(!JSON.stringify(board).includes(previous.learner));
assert.equal(reads,42);assert.equal(answers,112);assert.equal(learner.badges.length,13);
const result={at:new Date().toISOString(),baselineAt:previous.at,baselineServer:"41024; expired inherited deadline; authoritative exit 1",currentServer,instance:"teaching-proctor",queryReads:reads,submittedRuns:40,exactRunRoster:true,runQuizIdentity:true,answerBodyEquality:false,quizzes:new Set(previous.runs.map(row=>row.quiz)).size,levels:new Set(previous.runs.map(row=>row.challenge)).size,answers,bestPoints:learner.total,bestQuizzes:Object.keys(learner.best).length,badges:learner.badges.length,leaderboardOwn:true,cors:origin,mutations:0,directDatabaseReads:0,persistence:"HTTP authority requery across actual server expiry and normal managed restart"};
writeFileSync(resolve(ticket,"🗑️generated",output),JSON.stringify(result,null,2)+"\n",{flag:"wx"});console.log(JSON.stringify(result));
