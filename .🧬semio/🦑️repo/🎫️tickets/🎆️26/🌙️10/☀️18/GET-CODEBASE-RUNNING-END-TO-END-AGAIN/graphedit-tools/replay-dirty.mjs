/** 🎯 Read-only replay on the live generation: which projects an import-preserving byte edit of each file of the course's declared TypeScript programs makes stale, before and after `ts-closure` (which stops the course from recording a raw `file` fact per program file); plus the cost of re-deriving the programs (the derived fact's own cutoff). */
import{readFileSync}from"node:fs";
import{join,dirname}from"node:path";
import{createRequire}from"node:module";
import{brotliDecompressSync}from"node:zlib";
const [root,owner,...configs]=process.argv.slice(2);
const ts=createRequire(join(root,"package.json"))("typescript");
const t0=performance.now(),program=new Set();
for(const rel of configs){const config=join(root,rel),source=ts.readJsonConfigFile(config,ts.sys.readFile),parsed=ts.parseJsonSourceFileConfigFileContent(source,ts.sys,dirname(config),undefined,config);
 for(const f of ts.createProgram({rootNames:parsed.fileNames,options:{...parsed.options,noEmit:true,incremental:false,composite:false},projectReferences:parsed.projectReferences}).getSourceFiles())if(f.fileName.startsWith(root+"/")&&!f.fileName.includes("/node_modules/")&&!f.fileName.slice(root.length+1).startsWith(".cache/"))program.add(f.fileName);}
const deriveMs=performance.now()-t0;
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const raw=new Set(["file","typescriptText","digest"]),byPath=new Map();
for(let i=0;i<idx.rows.length;i++){const k=idx.rowKindOf(i);if(raw.has(k)||k==="analysis"){const r=idx.rows[i];if(program.has(r.path)){const s=byPath.get(r.path)??[];s.push(i);byPath.set(r.path,s);}}}
const holders=new Map();idx.classes.forEach(c=>{for(const r of c.r)if(holders.has(r)||true){}});
const want=new Set([...byPath.values()].flat());idx.classes.forEach(c=>{for(const r of c.r)if(want.has(r)){const s=holders.get(r)??new Set();for(const o of c.o)s.add(idx.owners[o]);holders.set(r,s);}});
const derived=n=>n.startsWith("@derived:");
let before=0,after=0,questions=0;const dirtyBefore=new Map(),dirtyAfter=new Map();
for(const path of program){
 const b=new Set(),a=new Set(),q=new Set();
 for(const i of byPath.get(path)??[]){const k=idx.rowKindOf(i);for(const o of holders.get(i)??[]){if(derived(o)){q.add(o);continue;}if(k==="analysis")continue;b.add(o);if(!(k==="file"&&o===owner))a.add(o);}}
 if(b.size)before++;if(a.size)after++;questions+=q.size;
 for(const o of b)dirtyBefore.set(o,(dirtyBefore.get(o)??0)+1);for(const o of a)dirtyAfter.set(o,(dirtyAfter.get(o)??0)+1);
}
const top=m=>JSON.stringify([...m].sort((x,y)=>y[1]-x[1]).slice(0,6));
console.log(JSON.stringify({generation:cur.generation,programFiles:program.size,deriveBothProgramsMs:Math.round(deriveMs),filesWhoseEditDirtiesAProject:{before,after},derivedQuestionsPerEditAvg:+(questions/program.size).toFixed(2)}));
console.log("owners dirtied (files) before",top(dirtyBefore));console.log("owners dirtied (files) after ",top(dirtyAfter));
const bench=join(root,"🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts"),bb=new Set(),ba=new Set();for(const i of byPath.get(bench)??[]){const k=idx.rowKindOf(i);for(const o of holders.get(i)??[])if(!derived(o)&&k!=="analysis"){bb.add(o);if(!(k==="file"&&o===owner))ba.add(o);}}
console.log("bench2-g edit file: dirty before",[...bb],"after",[...ba],"rss MB",Math.round(process.memoryUsage().rss/2**20));
