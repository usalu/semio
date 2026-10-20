// 📊 Classifies every task of the static whole-graph reconstruction by tool family and by how the Bazel adapter executes it (before: the live adapter; after: the adapter with the staged coverage set).
// usage: BZL_COVERAGE_ADAPTER=<adapter script> BZL_COVERAGE_CLASSIFIER=<coverage.ts> BZL_COVERAGE_MODE=before|after bun census.ts <out.json>
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {join,normalize} from "node:path";
const root="/Users/ueli/Documents/semio",mode=process.env.BZL_COVERAGE_MODE??"before";
const api=await import(process.env.BZL_COVERAGE_ADAPTER!),neutral=await import(process.env.BZL_COVERAGE_CLASSIFIER!);
const dir=join(import.meta.dir,"..","🗑️generated","bzl-coverage"),graph=JSON.parse(readFileSync(join(dir,"graph.json"),"utf8"));
const names=new Set<string>(),dangling:string[]=[];for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
for(const p of Object.values<any>(graph.projects))for(const [tn,t] of Object.entries<any>(p.targets)){if(!Array.isArray(t.dependsOn))continue;t.dependsOn=t.dependsOn.filter((d:any)=>{const s=typeof d==="string"?d:d.target,op=s.replace(/^\^/,"");if(!op.includes(":"))return true;if(names.has(op.split(":").slice(0,2).join(":")))return true;dangling.push(p.name+":"+tn+" -> "+s);return false;});}
const all=[...names],plan=api.planBazelTasksV1(graph,{tasks:all,args:[]}),emitted=api.bazelActionsV1(graph,plan,false);
const scripts=new Map<string,string|undefined>();
const scriptOf=(task:any):string|undefined=>{
 const e=task.definition.execution,command:string|undefined=e.command??(typeof e.commands?.[0]==="string"?e.commands[0]:e.commands?.[0]?.command);
 const m=command&&/^bun\s+"?(\.{0,2}\/?[^"\s]*📜️script\.ts)"?/u.exec(command);if(!m)return undefined;
 const path=normalize(join(root,e.cwd??task.root,m[1]!));
 if(!scripts.has(path))scripts.set(path,existsSync(path)?readFileSync(path,"utf8").slice(0,400000):undefined);
 return scripts.get(path);
};
const rows:any[]=[];
for(const action of emitted.values()){
 const task=plan.tasks[action.id]!,script=scriptOf(task);
 const e=task.definition.execution,first=e.command??(typeof e.commands?.[0]==="string"?e.commands[0]:e.commands?.[0]?.command);
 const input={project:task.project,target:task.target,root:task.root,kind:e.kind==="noop"?"noop":"command",...(first!==undefined?{command:first}:{}),...(script!==undefined?{script}:{}),...(Array.isArray(task.definition.outputs)?{outputs:task.definition.outputs}:{}),cache:action.cache.cache?{cache:true}:{cache:false,reason:action.cache.reason}};
 const family=neutral.taskFamilyV1(input),today=action.kind==="launch"?"run-target":action.kind==="noop"?"cached-action":action.cache.cache?(action.kind==="test"?"cached-test":"cached-action"):(action.cache.reason==="unbounded"||action.cache.reason==="undeclared")?"fail-closed":"no-cache";
 const coverage=neutral.taskCoverageV1(input);
 rows.push({id:action.id,project:task.project,target:task.target,family,kind:action.kind,today,reason:action.cache.reason??null,disposition:coverage.disposition,scriptText:script!==undefined,vitest:family==="vitest"});
}
const count=(key:(row:any)=>string)=>{const out:Record<string,number>={};for(const row of rows)out[key(row)]=(out[key(row)]??0)+1;return Object.fromEntries(Object.entries(out).sort(([a],[b])=>a<b?-1:a>b?1:0));};
const table:Record<string,Record<string,number>>={};for(const row of rows){const cell=row.today==="no-cache"||row.today==="fail-closed"?row.today+":"+row.reason:row.today;(table[row.family]??={})[cell]=(table[row.family]![cell]??0)+1;}
const report={mode,tasks:rows.length,danglingRefused:dangling,byToday:count(r=>r.today),byDisposition:count(r=>r.disposition),byFamily:count(r=>r.family),table,
 testDispositionNotConverted:rows.filter(r=>r.disposition==="test"&&r.kind!=="test").length,testTargets:rows.filter(r=>/^test(-|$)/.test(r.target)).length,
 scriptTextFound:rows.filter(r=>r.scriptText).length};
writeFileSync(process.argv[2]!,JSON.stringify({report,rows}));
console.log(JSON.stringify(report,null,1));
