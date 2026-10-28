import{spawn}from"node:child_process";import{createReadStream,openSync,readFileSync,readdirSync}from"node:fs";import{join}from"node:path";
/** ⏱️ Phase profile of the sealed loader (run INSIDE a mirror root with a fresh snapshot): spawns the trampoline's exact child (--eval loader, snapshot on fd 3, ready on fd 4) with timing marks injected into a COPY of the stored loader ([DEBUG] lines on stderr).
 * usage: bun profile-loader.mjs [runs] [--plain] [--cpu-prof] [--file] [command words...]   (--plain: no marks, only wall) */
const args=process.argv.slice(2),runs=Number(/^\d+$/.test(args[0]??"")?args.shift():3),plain=args[0]==="--plain"&&args.shift()!==undefined,cpuProf=args[0]==="--cpu-prof"&&args.shift()!==undefined,viaFile=args[0]==="--file"&&args.shift()!==undefined,words=args.length?args:["repo","--help"];
const base=".🧬semio/🦑️repo/⚡️cache/implementation",dir=join(base,readdirSync(base)[0]),pointer=JSON.parse(readFileSync(join(dir,"current.json"),"utf8"));
let loader=readFileSync(join(dir,pointer.loader+".loader.js"),"utf8");
const inject=(anchor,code)=>{if(loader.split(anchor).length!==2)throw Error("anchor "+anchor);loader=loader.replace(anchor,()=>code+anchor);},mark=n=>`globalThis.__m(${JSON.stringify(n)});`;
if(!plain&&!cpuProf){
 loader="globalThis.__t0=performance.now();globalThis.__acc={n:0,tr:0,rd:0,tc:0,rc:0,ol:0};globalThis.__cpu=()=>{const u=process.cpuUsage();return (u.user+u.system)/1000;};globalThis.__m=n=>process.stderr.write('[DEBUG] '+n+' '+(performance.now()-globalThis.__t0).toFixed(0)+' ms rss '+(process.memoryUsage().rss/1048576).toFixed(0)+' MB cpu '+((u=>(u.user+u.system)/1000)(process.cpuUsage())).toFixed(0)+' ms\\n');process.on('exit',()=>process.stderr.write('[DEBUG] cpu transform '+__acc.tc.toFixed(0)+' redirect '+__acc.rc.toFixed(0)+' onLoad '+__acc.ol.toFixed(0)+' total '+__cpu().toFixed(0)+' ms\\n[DEBUG] modules '+__acc.n+' transform '+__acc.tr.toFixed(0)+' ms redirect '+__acc.rd.toFixed(0)+' ms\\n'));"+loader;
 inject("const redirectCapturedBuiltinReadsV1=(",mark("snapshot-read+sha"));
 inject("const{images,resolutions}=admitCapturedImagesV1(",mark("json-parsed"));
 inject("const capturedSourceImportDiagnosticV1=(",mark("files-admitted"));
 inject("const module=await import(entry);",mark("plugin-ready"));
 inject("if(typeof module.runCapturedBuildSourceV1!=='function')",mark("entry-imported"));
 loader=loader.replace("build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{check();","build.onLoad({filter:/.*/,namespace:'semio-source'},item=>{const c0=__cpu();check();");loader=loader.replace("transformed=compiler.transformSync(bytes.toString('utf8'));","c1=__cpu(),t1=performance.now(),transformed=compiler.transformSync(bytes.toString('utf8'));__acc.n++;__acc.tr+=performance.now()-t1;__acc.tc+=__cpu()-c1;process.env.S2G_MODULES&&require('node:fs').appendFileSync(process.env.S2G_MODULES,name+'\\n');").replace("return{contents:redirect(transformed),loader:'js'};","const t2=performance.now(),c2=__cpu(),out=redirect(transformed);__acc.rd+=performance.now()-t2;__acc.rc+=__cpu()-c2;__acc.ol+=__cpu()-c0;return{contents:out,loader:'js'};");
}
for(let n=1;n<=runs;n++){
 const t0=performance.now();
 await new Promise(done=>{
  const child=spawn(process.execPath,[...(cpuProf?["--cpu-prof","--cpu-prof-dir="+(process.env.S2G_PROF??"/tmp")]:[]),"--eval",loader,"--",pointer.digest,"--semio-role=door",...words],{stdio:["ignore","ignore","inherit",viaFile?openSync(join(dir,pointer.digest+".json"),"r"):"pipe","pipe"],env:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}});let ready=0;
  if(!viaFile){child.stdio[3].on("error",()=>{});createReadStream(join(dir,pointer.digest+".json")).pipe(child.stdio[3]);}
  child.stdio[4].on("data",()=>{ready||=performance.now()-t0;});child.once("close",(code)=>{console.log(`run ${n}  ready ${(ready/1000).toFixed(2)} s  exit ${((performance.now()-t0)/1000).toFixed(2)} s  code ${code}`);done();});
 });
}
