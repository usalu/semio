/** 🧪️ Replays canonical worker publication and timer cancellation in all three JavaScript hosts. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020.js";
import ts from "typescript";

const print=resolve(process.cwd(),"🧰️framework/🛍️products/📓️print"),out=resolve(import.meta.dir,"../🗑️generated/async-runtime-final");
mkdirSync(out,{recursive:true});
const entry=join(print,"🧬️schema/💡️inferences/🟦️.ts"),worker=join(print,"🧬️schema/💡️inferences/🧵️worker/🟦️.ts"),validation=join(print,"🧬️schema/💡️inferences/✅️validation/🟦️.ts"),schema=JSON.parse(readFileSync(join(print,"🧬️schema/💡️inferences/🔣️.json"),"utf8"));
/** 🔐️ Fingerprints the complete first-party runtime import closure, including the separate worker. */
function sourceClosure():{path:string;sha256:string}[]{
  const seen=new Set<string>(),pending=[entry,worker,validation,join(print,"🧬️schema/💡️inferences/🔣️.json")];
  while(pending.length){
    const path=pending.pop()!;if(seen.has(path))continue;seen.add(path);
    if(path.endsWith(".ts")){
      const source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
      for(const node of source.statements)if((ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly||ts.isExportDeclaration(node)&&!node.isTypeOnly)&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier)&&node.moduleSpecifier.text.startsWith("."))pending.push(resolve(dirname(path),node.moduleSpecifier.text));
    }
  }
  return [...seen].sort().map(path=>({path,sha256:createHash("sha256").update(readFileSync(path)).digest("hex")}));
}
const initialSources=sourceClosure(),initialSha256=createHash("sha256").update(JSON.stringify(initialSources)).digest("hex");
const {inferVizChart}=await import(pathToFileURL(entry).href),{validateVizChartInference}=await import(pathToFileURL(validation).href);
const snapshot=JSON.parse(readFileSync(join(print,"🧫️fixtures/🧬️chart-mutations/🔣️.json"),"utf8")).snapshot;
snapshot.chart.scales.push({name:"paint",kind:"ordinal",domain:["A","B"],range:["#ff0000","#0000ff"]});
snapshot.chart.guides=[{kind:"axis",scale:"x",title:{en:"Axis",de:"Achse"}},{kind:"legend",scale:"paint",title:{en:"Palette",de:"Palette"},options:{channel:"fill"}}];

async function replay(infer:any,snapshot:any):Promise<any> {
  const progress:any[]=[],valid=await infer(snapshot,{onProgress:(value:any)=>progress.push(value)}),again=await infer(snapshot);
  if(!valid.complete)throw new Error(JSON.stringify(valid.diagnostics));
  const earlyController=new AbortController();earlyController.abort();let earlyProgress=0;
  const early=await infer(snapshot,{signal:earlyController.signal,onProgress:()=>earlyProgress++});
  const controller=new AbortController(),expensive={chart:{...snapshot.chart,tables:[{...snapshot.chart.tables[0],rows:Array.from({length:50000},(_,index)=>({x:index%3,y:index%5}))}]}};
  let started=false,fired=false,timer:any,startProgress:any;
  let cancelled:any;
  try{cancelled=await infer(expensive,{signal:controller.signal,onProgress:(value:any)=>{if(!started&&value.completed>1&&value.completed<value.total-2){started=true;startProgress=value;timer=setTimeout(()=>{fired=true;controller.abort();},0);}}});}finally{if(timer!==undefined)clearTimeout(timer);}
  const done=progress.at(-1),texts=valid.plan?.items.filter((item:any)=>item.kind==="text"&&item.font!==undefined).map((item:any)=>[item.content,item.font]),sceneTexts=valid.scene?.nodes.filter((item:any)=>item.node.kind==="text"&&item.node.font!==undefined).map((item:any)=>[item.node.content,item.node.font]);
  return {valid,early,cancelled,evidence:{complete:valid.complete,deterministic:JSON.stringify(valid)===JSON.stringify(again),progress,progressValid:progress.length>2&&progress[0].completed===0&&done.completed===done.total&&progress.every((value:any,index:number)=>value.total===done.total&&(index===0||value.completed>progress[index-1].completed)),earlyProgress,earlyAtomic:!early.complete&&early.tikz===""&&early.plan===undefined&&early.scene===undefined,started,fired,startProgress,cancelAtomic:!cancelled.complete&&cancelled.tikz===""&&cancelled.plan===undefined&&cancelled.scene===undefined&&cancelled.diagnostics[0]?.code==="print.chart.cancelled",texts,sceneTexts,fontTransport:JSON.stringify(texts)===JSON.stringify(sceneTexts)&&texts.length===9&&["Palette","Axis"].every(title=>texts.some((item:any)=>item[0]===title&&item[1]==="Share Tech Mono"))&&["0","0.5","1","1.5","2","A","B"].every(label=>texts.some((item:any)=>item[0]===label&&item[1]==="Anta")),tikzFontSelectors:valid.tikz.includes("\\SemioMono")&&valid.tikz.includes("\\SemioSans")}};
}

const ajv=new Ajv2020({strict:false}),admit=ajv.compile(schema),records:any[]=[];
function record(host:string,version:string,result:any):void {
  const admissions=[result.valid,result.early,result.cancelled].map(value=>({owned:validateVizChartInference(value).length===0,ajv:admit(value),errors:admit.errors}));
  const fontAdmission=["plan","scene"].flatMap(surface=>[undefined,"Custom Family","",42,"non-text"].map(font=>{
    const value=structuredClone(result.valid),items=surface==="plan"?value.plan.items:value.scene.nodes.map((entry:any)=>entry.node),item=items.find((item:any)=>font==="non-text"?item.kind!=="text":item.kind==="text");
    if(font===undefined)delete item.font;else item.font=font==="non-text"?"Anta":font;
    const expected=font===undefined||font==="Custom Family",owned=validateVizChartInference(value).length===0,external=admit(value);
    return {surface,input:font??"absent",expected,owned,ajv:external,pass:owned===expected&&external===expected};
  }));
  const evidence=result.evidence,pass=evidence.complete&&evidence.deterministic&&evidence.progressValid&&evidence.earlyProgress===0&&evidence.earlyAtomic&&evidence.started&&evidence.fired&&evidence.cancelAtomic&&evidence.fontTransport&&evidence.tikzFontSelectors&&admissions.every(value=>value.owned&&value.ajv)&&fontAdmission.every(value=>value.pass);
  records.push({host,version,pass,evidence,admissions,fontAdmission});console.log("[DEBUG] "+JSON.stringify(records.at(-1)));
}
record("Bun",Bun.version,await replay(inferVizChart,snapshot));

const bundles:any[]=[];
for(const target of ["browser","node"] as const){
  const directory=join(out,target);mkdirSync(join(directory,"🧵️worker"),{recursive:true});
  for(const [source,destination] of [[entry,"main.mjs"],[worker,"🧵️worker/🟦️.ts"]]){
    const built=await Bun.build({entrypoints:[source!],target});if(!built.success)throw new Error(built.logs.map(String).join("\n"));
    const text=await built.outputs[0]!.text(),path=join(directory,destination!);writeFileSync(path,text);bundles.push({target,path,sha256:createHash("sha256").update(text).digest("hex"),bytes:Buffer.byteLength(text)});
  }
}
const main=readFileSync(join(out,"browser/main.mjs")),workerBundle=readFileSync(join(out,"browser/🧵️worker/🟦️.ts"));
const server=Bun.serve({hostname:"127.0.0.1",port:0,fetch:request=>new Response(new URL(request.url).pathname==="/main.mjs"?main:new URL(request.url).pathname.endsWith(".ts")?workerBundle:"",{headers:{"Content-Type":"text/javascript"}})});
const browserSource=`(async()=>{const {chromium}=await import("playwright"),browser=await chromium.launch({headless:true,timeout:30000});try{const page=await browser.newPage();await page.goto(${JSON.stringify(server.url.href)});const result=await page.evaluate(async ({url,source,snapshot})=>{const {inferVizChart}=await import(url),replay=(0,eval)("("+source+")");return await replay(inferVizChart,snapshot);},${JSON.stringify({url:new URL("main.mjs",server.url).href,source:replay.toString(),snapshot})});console.log(JSON.stringify({version:browser.version(),result}));}finally{await browser.close();}})().catch(error=>{console.error(error);process.exitCode=1;});`;
try{
  const controller=Bun.spawn(["node","-e",browserSource],{stdout:"pipe",stderr:"pipe"});
  const [stdout,stderr,status]=await Promise.all([new Response(controller.stdout).text(),new Response(controller.stderr).text(),controller.exited]);
  writeFileSync(join(out,"browser-controller-terminal.log"),stdout+stderr);if(status!==0)throw Error("Browser controller exit "+status+": "+stderr);
  const result=JSON.parse(stdout.trim());record("Chromium",result.version,result.result);
}finally{await server.stop(true);}

const nodeSource=`const {inferVizChart}=await import(${JSON.stringify(pathToFileURL(join(out,"node/main.mjs")).href)});const replay=${replay.toString()};const result=await replay(inferVizChart,${JSON.stringify(snapshot)});console.log(JSON.stringify({version:process.version,result}));`;
const node=Bun.spawn(["node","-e","(async()=>{"+nodeSource+"})().catch(error=>{console.error(error);process.exitCode=1;});"],{stdout:"pipe",stderr:"pipe"});
const [stdout,stderr,exit]=await Promise.all([new Response(node.stdout).text(),new Response(node.stderr).text(),node.exited]);
writeFileSync(join(out,"node-terminal.log"),stdout+stderr);if(exit!==0)throw new Error("Node exit "+exit+": "+stderr);
const nodeResult=JSON.parse(stdout.trim());record("Node",nodeResult.version,nodeResult.result);
const sources=sourceClosure(),finalSha256=createHash("sha256").update(JSON.stringify(sources)).digest("hex"),closure={initialSha256,finalSha256,stable:initialSha256===finalSha256,files:sources.length};
writeFileSync(join(out,"evidence.json"),JSON.stringify({at:new Date().toISOString(),platform:process.platform,architecture:process.arch,records,bundles,sources,closure},null,2));
if(!closure.stable||records.some(record=>!record.pass))throw new Error("canonical host replay failed");
