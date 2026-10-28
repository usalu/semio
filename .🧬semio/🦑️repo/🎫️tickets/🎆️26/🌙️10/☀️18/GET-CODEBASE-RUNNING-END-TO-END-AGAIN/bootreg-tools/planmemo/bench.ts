/** ⏱️ Cost of a planning memo hit on the real root workspace (read-only): questions from a captured `cargo metadata`, answering them, record size gzip; `bun bench.ts <module-root> <metadata.json>` */
import {readFileSync} from "node:fs";import {gzipSync,gunzipSync} from "node:zlib";
const [moduleRoot,metaPath]=process.argv.slice(2) as [string,string],P="/🧰️framework/🔨️modules/🏃️process";
const memo=await import(moduleRoot+P+"/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo/🟦️.ts"),routing=await import(moduleRoot+P+"/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"cargo:plan-memo-bench",maximumElapsedMilliseconds:120000},{native:{fixture:"bench"}},Date.now()),async(original:unknown)=>{
 const control=routing.createIssuedResourceControlV1(original,"cargo:plan-memo-bench",0),text=readFileSync(metaPath,"utf8");
 let a=performance.now();const metadata=JSON.parse(text);console.log(`[DEBUG] parse ${(performance.now()-a).toFixed(0)} ms`);
 a=performance.now();const questions=await memo.cargoPlanQuestionsV1(metadata);console.log(`[DEBUG] questions ${questions.length} in ${(performance.now()-a).toFixed(0)} ms`);
 for(let i=0;i<2;i++){a=performance.now();const answers=await memo.cargoPlanAnswersV1(questions,null,control);console.log(`[DEBUG] answers ${answers.length} in ${(performance.now()-a).toFixed(0)} ms`);}
 a=performance.now();const packed=gzipSync(JSON.stringify({metadata,units:{}}),{level:1});console.log(`[DEBUG] gzip ${(packed.length/1e6).toFixed(1)} MB in ${(performance.now()-a).toFixed(0)} ms`);a=performance.now();JSON.parse(gunzipSync(packed).toString("utf8"));console.log(`[DEBUG] gunzip+parse ${(performance.now()-a).toFixed(0)} ms`);
});
