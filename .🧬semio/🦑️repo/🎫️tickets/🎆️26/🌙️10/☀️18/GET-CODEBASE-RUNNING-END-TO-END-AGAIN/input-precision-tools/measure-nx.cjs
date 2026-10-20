// ⏱️ Nx hash cost of declared inputs: the native WorkspaceContext globs and hashes the workspace files matching a task's input list (what Nx's task hasher does per task); once for the workspace scan, then per task before and after.
const path=require("path"),fs=require("fs");
const W="/Users/ueli/Documents/semio",native=require(W+"/node_modules/nx/dist/src/native");
const G=process.argv[2],expanded=JSON.parse(fs.readFileSync(path.join(G,"expanded.json"),"utf8"));
const split=l=>({pos:l.filter(x=>!x.startsWith("!")),neg:l.filter(x=>x.startsWith("!")).map(x=>x.slice(1))});
const t0=process.hrtime.bigint(),ctx=new native.WorkspaceContext(W,path.join(G,"nx-cache"));
ctx.allFileData();
const init=Number(process.hrtime.bigint()-t0)/1e6;
const time=(l)=>{const {pos,neg}=split(l),runs=[];let files=0,hash;for(let i=0;i<3;i++){const s=process.hrtime.bigint();hash=ctx.hashFilesMatchingGlob(pos,neg);runs.push(Number(process.hrtime.bigint()-s)/1e6);}
 const matched=ctx.glob(pos,neg).length;return {ms:Math.round(Math.min(...runs)*10)/10,msFirst:Math.round(runs[0]*10)/10,matched,globs:l.length};};
const out={workspaceScanMs:Math.round(init),tasks:{}};
for(const [id,v] of Object.entries(expanded)){out.tasks[id]={before:time(v.before),after:time(v.after)};}
console.log(JSON.stringify(out,null,1));
