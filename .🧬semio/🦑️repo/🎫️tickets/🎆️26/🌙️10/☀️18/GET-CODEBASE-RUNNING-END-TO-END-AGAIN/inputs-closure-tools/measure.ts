// 📏 Real-tree measurement: the shipped script-inputs component versus the staged patch, in one process over the tracked project scripts.
import {writeFileSync} from "node:fs";import {join,dirname} from "node:path";
import {G,W,loadBoth,scriptEntries} from "./lib.ts";
const [limitArg="100000",outArg=join(G,"measure.json"),offsetArg="0",only="all",strideArg="1"]=process.argv.slice(2);
const {cacheInternals,createScriptInputCache,patched,shipped}=await loadBoth();
const entries=scriptEntries(Number(strideArg),Number(offsetArg),Number(limitArg));
const fallback=cacheInternals.genericCommandFallbackInputs(W);
const phase=async(label:string,api:any)=>{
 const cache=api.createScriptInputCache({workspaceRoot:W}),rows:{entry:string;closure:number;targetInputs:number;ms:number}[]=[],started=performance.now(),cpu0=process.cpuUsage();let failures=0,closureMs=0;const visited=new Set<string>();
 for(const entry of entries){
  const root=dirname(entry),t0=performance.now();
  try{
   const files:string[]=await api.relativeScriptInputs([join(W,entry)],W,cache,true);for(const f of files)visited.add(f);closureMs+=performance.now()-t0;
   const inputs:any[]=await api.genericTargetCommandInputs({options:{command:'bun ./📜️script.ts check',cwd:"{projectRoot}"},inputs:["default","^default"]},W,fallback,cache,root);
   rows.push({entry,closure:files.length,targetInputs:inputs.length,ms:performance.now()-t0});
  }catch(e:any){failures++;rows.push({entry,closure:-1,targetInputs:-1,ms:performance.now()-t0});}
 }
 const cpu=process.cpuUsage(cpu0);return {label,seconds:(performance.now()-started)/1000,cpuSeconds:(cpu.user+cpu.system)/1e6,closureSeconds:closureMs/1000,failures,distinct:visited.size,rows};
};
const results=only==="shipped"?[await phase("shipped-cold",shipped)]:only==="patched"?[await phase("patched-cold",patched)]:[await phase("shipped-cold",shipped),await phase("shipped-warm-parse",shipped),await phase("patched",patched)];
if(only!=="all"){const r=results[0]!;writeFileSync(outArg,JSON.stringify({label:r.label,seconds:r.seconds,distinct:r.distinct,failures:r.failures,rows:r.rows}));console.log(JSON.stringify({label:r.label,entries:entries.length,seconds:+r.seconds.toFixed(1),cpuSeconds:+(r as any).cpuSeconds.toFixed(1),closureSeconds:+(r as any).closureSeconds.toFixed(1),failures:r.failures,distinct:r.distinct,medianClosure:[...r.rows.map(x=>x.closure)].sort((x,y)=>x-y)[r.rows.length>>1],medianMsPerEntry:Math.round([...r.rows.map(x=>x.ms)].sort((x,y)=>x-y)[r.rows.length>>1]!)}));console.log(JSON.stringify((globalThis as any).__t??null));process.exit(0);}
const old=results[1]!,next=results[2]!,med=(a:number[])=>[...a].sort((x,y)=>x-y)[a.length>>1]!;
const pairs=old.rows.map((r,i)=>({entry:r.entry,old:r.closure,next:next.rows[i]!.closure})).filter(r=>r.old>=0&&r.next>=0);
const grow=pairs.map(r=>r.next-r.old);
const summary={entries:entries.length,runs:results.map(r=>({label:r.label,seconds:+r.seconds.toFixed(1),failures:r.failures,distinct:r.distinct,medianMsPerEntry:Math.round(med(r.rows.map(x=>x.ms)))})),comparable:pairs.length,medianOld:med(pairs.map(r=>r.old)),medianNew:med(pairs.map(r=>r.next)),medianGrowth:med(grow),maxGrowth:Math.max(...grow),entriesGrown:pairs.filter(r=>r.next>r.old).length,newFailures:next.rows.filter((r,i)=>r.closure<0&&old.rows[i]!.closure>=0).map(r=>r.entry).slice(0,10)};
writeFileSync(outArg,JSON.stringify({summary,results:results.map(r=>({label:r.label,seconds:r.seconds,rows:r.rows}))}));
console.log(JSON.stringify(summary));
