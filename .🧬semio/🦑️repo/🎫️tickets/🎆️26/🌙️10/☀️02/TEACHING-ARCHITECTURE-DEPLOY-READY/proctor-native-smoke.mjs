import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {randomBytes} from "node:crypto";
const [origin,coursePath]=process.argv.slice(2),allowedOrigin="http://localhost:6061";
assert(origin&&coursePath,"origin and actual course source required");
const expected=JSON.parse(readFileSync(coursePath,"utf8"));
const quizzes=expected.quizzes.map(path=>JSON.parse(readFileSync(resolve(dirname(coursePath),path),"utf8")));
const ids=quizzes.map(quiz=>quiz.id);
assert.equal(ids.length,10);assert.equal(new Set(ids).size,10);
const deadline=Date.now()+15000;
let instance,last;
while(Date.now()<deadline){
 try{
  const response=await fetch(origin+"/instance",{headers:{Origin:allowedOrigin},signal:AbortSignal.timeout(Math.max(1,Math.min(3000,deadline-Date.now())))});
  assert.equal(response.status,200);assert.equal(response.headers.get("access-control-allow-origin"),allowedOrigin);
  const body=await response.json();assert.equal(body.id,"teaching-proctor");instance=body;break;
 }catch(error){last=error;const remaining=deadline-Date.now();if(remaining>0)await new Promise(resolve=>setTimeout(resolve,Math.min(250,remaining)));}
}
assert(instance,"bounded /instance readiness failed: "+String(last));
const preflight=await fetch(origin+"/queries",{method:"OPTIONS",headers:{Origin:allowedOrigin,"Access-Control-Request-Method":"POST","Access-Control-Request-Headers":"content-type"},signal:AbortSignal.timeout(3000)});
assert([200,204].includes(preflight.status),"query preflight status "+preflight.status);
assert.equal(preflight.headers.get("access-control-allow-origin"),allowedOrigin);
const methods=(preflight.headers.get("access-control-allow-methods")??"").split(",").map(value=>value.trim().toUpperCase());
const headers=(preflight.headers.get("access-control-allow-headers")??"").split(",").map(value=>value.trim().toLowerCase());
assert(methods.includes("POST")||methods.includes("*"),"query POST must be allowed");
assert(headers.includes("content-type")||headers.includes("*"),"query content-type must be allowed");
const query={queryId:randomBytes(16).toString("hex"),kind:"course.course",version:10,scope:expected.id,principal:{kind:"anonymous"},arguments:Array.from(Buffer.from(JSON.stringify({type:"course"}),"utf8")),consistency:{kind:"authority"},cursor:null};
const response=await fetch(origin+"/queries",{method:"POST",headers:{"Content-Type":"application/json",Origin:allowedOrigin},body:JSON.stringify(query),signal:AbortSignal.timeout(3000)});
const body=await response.json();
assert.equal(response.status,200,JSON.stringify(body));assert.equal(response.headers.get("access-control-allow-origin"),allowedOrigin);assert.equal(body.kind,"snapshot");
assert(Array.isArray(body.value)&&body.value.every(byte=>Number.isInteger(byte)&&byte>=0&&byte<=255));
const view=JSON.parse(Buffer.from(body.value).toString("utf8"));
assert.equal(view.id,expected.id);assert.deepEqual(view.title,expected.title);
assert.deepEqual(view.quizzes.map(quiz=>quiz.id).sort(),ids.slice().sort());
for(const quiz of view.quizzes){
 assert.deepEqual(quiz.title,quizzes.find(expected=>expected.id===quiz.id).title);
 assert(Array.isArray(quiz.tasks)&&quiz.tasks.every(task=>!Object.hasOwn(task,"items")&&!Object.hasOwn(task,"solution")));
}
console.log(JSON.stringify({instance:instance.id,cors:allowedOrigin,preflight:preflight.status,query:response.status,kind:body.kind,course:view.id,quizzes:ids,titles:Object.keys(view.title)},null,2));
