// 🧮 Applies the staged input precision rules to the reconstructed build graph, the way the inference hook does, and writes graph-after.json plus a per-rule report.
import {readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {readInputPrecisionV1,selectRuleV1,deriveInputsV1,commandWordsV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {staticHost,W} from "./static-host.ts";
import {realHost} from "./real-host.ts";
const T=import.meta.dir,G=join(T,"..","🗑️generated","input-precision");
const rulesPath=process.env.IP_RULES??join(T,"staged2","rules.json");
const document=readInputPrecisionV1(JSON.parse(readFileSync(rulesPath,"utf8")));
const graph=JSON.parse(readFileSync(join(G,"graph.json"),"utf8"));
const real=process.env.IP_HOST==="real"?await realHost(""):undefined,host=real?.host??staticHost();
const report:any={rules:{},unmatched:0,failures:[] as string[]};
const closureCache=new Map<string,string[]>();
const closureOf=async(script:string)=>{let c=closureCache.get(script);if(!c){c=real?await real.internals.relativeScriptInputs([script],W,real.scripts,false):(await host.importClosure([script.replace(W+"/","")])).map(f=>"{workspaceRoot}/"+f);closureCache.set(script,c!);}return c!;};
let matched=0;const started=performance.now();
const only=process.env.IP_ONLY?new Set(process.env.IP_ONLY.split(",")):undefined;
for(const project of Object.values<any>(graph.projects)){
 for(const [name,target] of Object.entries<any>(project.targets)){
  const command=target.execution?.command;if(typeof command!=="string")continue;
  const rule=selectRuleV1(document,{project:project.name,target:name,command});if(!rule||rule.evidence==="assumed"||rule.evidence==="whole"||(only&&!only.has(rule.id)))continue;
  try{
   const derived=await deriveInputsV1(document,rule,{project:project.name,target:name,command},host);
   const base=(target.inputs??["default"]).filter((x:unknown)=>!(typeof x==="string"&&derived.drops.includes(x)));
   const parsed=commandWordsV1(command),script=parsed?resolve(W,target.execution.cwd?.replace("{projectRoot}",project.root)??project.root,parsed.script):undefined;
   const closure=script?await closureOf(script):[];
   target.inputs=[...base,...derived.patterns,...derived.groups,...derived.files.map(f=>"{workspaceRoot}/"+f),...closure];
   matched++;
   const row=report.rules[rule.id]??={targets:0,patterns:0,files:0,closure:0,unresolved:new Set<string>(),reads:{}};
   row.targets++;row.patterns+=derived.patterns.length;row.files+=derived.files.length;row.closure+=closure.length;for(const u of derived.unresolved)row.unresolved.add(u);
   for(const r of derived.reads){const k=row.reads[r.read]??={patterns:0,files:0};k.patterns+=r.patterns;k.files+=r.files;}
  }catch(error:any){report.failures.push(`${project.name}:${name}: ${error.message}`);}
 }
}
for(const row of Object.values<any>(report.rules))row.unresolved=[...row.unresolved];
report.matched=matched;report.seconds=Math.round((performance.now()-started)/100)/10;
writeFileSync(join(G,process.env.IP_AFTER??"graph-after.json"),JSON.stringify(graph));
writeFileSync(join(G,process.env.IP_REPORT??"derivation-report.json"),JSON.stringify(report,null,1));
console.log(JSON.stringify({matched,failures:report.failures.length,seconds:report.seconds,rules:Object.fromEntries(Object.entries<any>(report.rules).map(([k,v])=>[k,{t:v.targets,p:v.patterns,f:v.files,c:v.closure,u:v.unresolved.length}]))}));
if(report.failures.length)console.log(report.failures.slice(0,10).join("\n"));
process.exit(0);
