import{readFileSync,readdirSync,statSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{join,resolve,relative}from"node:path";
import{tmpdir}from"node:os";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const cl=await import(join(G,"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs")),obs=await import(join(B,"👁️observation/🟨️.mjs")),cas=await import(join(B,"🗄️cas/🗃️store/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const limit=Number(process.argv[2]??100000),derived=index.rows.filter((r:any)=>r.kind==="derived").slice(0,limit);
const pseudoRows=new Map<string,any>();for(const row of derived)pseudoRows.set(cut.derivedOwnerNameV1(row),row);
let asked=0;const askedRows:any[]=[];const deriver=await memo.openPlannerDeriverV1(process.cwd()),derive=async(row:any,signal?:AbortSignal,pass?:object)=>{asked++;askedRows.push(row);return deriver(row,signal,pass);};
const hold=async(rows:any[],signal?:AbortSignal)=>{const own=rows.filter(r=>r.kind!=="closure"&&r.kind!=="derived"&&!(r.kind==="field"&&r.fact?.extract!==undefined));return own.length===0?true:obs.observationsHold(own,64,new Set(),true,signal);};
const cacheRoot=process.env.CACHE_ROOT??mkdtempSync(join(resolve(".🧬semio/🦑️repo/⚡️cache/tests"),"settle-edits-"));
const settler=cl.createClosureSettlerV1({workspace:{workspaceRoot:process.cwd(),cacheRoot,blobs:cas.createCasStoreV1()}});
const mem=()=>Math.round(process.memoryUsage().rss/1048576);
const round=async(label:string,dirty?:any,rows:Map<string,any>=pseudoRows)=>{asked=0;askedRows.length=0;const t=performance.now(),c0=process.cpuUsage();const r=await settler.settle({pseudoRows:rows,derive,hold,dirty,progress:async({done,total}:any)=>{if(process.env.PER&&done%500===0)console.log(label,"progress",done,total,Math.round((performance.now()-t)/1000),"s rss",mem());}});const c1=process.cpuUsage(c0);console.log(JSON.stringify({label,closures:rows.size,recomputed:asked,ms:Math.round(performance.now()-t),cpuMs:Math.round((c1.user+c1.system)/1000),rssMB:mem(),failed:r.failed,reused:r.reused,kept:r.kept.size,held:r.held.size}));const published=new Map<string,any>([...r.held,...[...r.answers].map(([name,answer]:any)=>[name,{digest:answer.digest,length:answer.length,ids:answer.ids,canonical:answer.canonical}])]);await settler.markIndexed({held:published});if(rows===pseudoRows)for(const[name,answer]of r.answers)pseudoRows.set(name,{...pseudoRows.get(name),fact:(answer as any).fact});};
import*as fsReal from"node:fs";
const target=resolve(process.env.EDIT_FILE??"🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"),rel=relative(process.cwd(),target).split("\\").join("/"),original=readFileSync(target);
let edited:Buffer|undefined;
const overlay=new Proxy(fsReal,{get(real:any,key:string){
 if(key==="readFileSync")return(path:any,options?:any)=>{if(edited&&String(path)===target)return options&&(options==="utf8"||options.encoding)?edited.toString("utf8"):Buffer.from(edited);return real.readFileSync(path,options);};
 if(key==="statSync"||key==="lstatSync")return(path:any,options?:any)=>{const st=real[key](path,options);if(edited&&String(path)===target){const now=Date.now();return Object.assign(Object.create(Object.getPrototypeOf(st)),st,{size:edited.length,mtimeMs:now,ctimeMs:now});}return st;};
 return real[key];}});
const underEdit=(bytes:Buffer|undefined,work:()=>Promise<void>)=>{edited=bytes;return obs.withObservedWorldV1(overlay,work);};
if(!process.env.SKIP_COLD)await round("cold");if(process.env.ONLY_COLD){process.exit(0);}
await round("warm-clean",{paths:[]});if(process.env.STOP_AFTER_CLEAN){if(!process.env.CACHE_ROOT)rmSync(cacheRoot,{recursive:true,force:true});process.exit(0);}
let suspects=new Map<string,any>();
await underEdit(Buffer.concat([original,Buffer.from("\n// body edit that adds no import\n")]),async()=>{await round("body-edit",{paths:[rel]});suspects=new Map(askedRows.map(row=>[cut.derivedOwnerNameV1(row),row]));});
await underEdit(Buffer.concat([original,Buffer.from("\n// body edit that adds no import\n")]),()=>round("in-plan-body-edit",{paths:[rel]},suspects));
await round("in-plan-clean",{paths:[]},new Map());
await underEdit(Buffer.concat([original,Buffer.from('\nimport "../../../🏗️build/🗣️language/🚨️error/🟦️.ts";\n')]),()=>round("import-edit",{paths:[rel]}));
rmSync(cacheRoot,{recursive:true,force:true});process.exit(0);
