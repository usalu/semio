// usage (inside the overlay, see overlay.sh): SEMIO_REPO_WATCH_BINARY=... SEMIO_WORLD_WITNESS_MODULE=... bun bench-world.ts [files] [dirs]
import {lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {createChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {changeJournalDirectoryV1,startChangeJournalServiceV1} from "../../🟦️.ts";
import {checkWorldViaJournalV1,type WorldRowV1} from "../../🔭️world/🟦️.ts";

const files=Number(process.argv[2]??50000),dirs=Number(process.argv[3]??2500),runs=7;
const sandbox=realpathSync(mkdtempSync(join(tmpdir(),"bench-world-")));
const environment={...process.env,XDG_RUNTIME_DIR:sandbox};
const directory=changeJournalDirectoryV1(sandbox,environment);
const median=(values:number[])=>[...values].sort((a,b)=>a-b)[Math.floor(values.length/2)]!;
const identity=(file:string)=>{const f=lstatSync(file);return[f.dev,f.ino,f.mode,f.size,f.mtimeMs,f.ctimeMs].join(":");};
try{
 const paths:string[]=[];
 for(let d=0;d<dirs;d++){const base=join(sandbox,"src","p"+(d%50),"d"+d);mkdirSync(base,{recursive:true});for(let f=0;f<files/dirs;f++){const path=join(base,"f"+f+".ts");writeFileSync(path,"x"+f);paths.push(path);}}
 await startChangeJournalServiceV1({directory,root:sandbox,environment});
 const client=createChangeJournalClientV1({directory,root:sandbox,environment});
 const {verifyWorldWitnessV1}=await import(pathToFileURL(process.env.SEMIO_WORLD_WITNESS_MODULE!).href);
 await new Promise(resolve=>setTimeout(resolve,100));
 const t0=performance.now();const cursor=await client.cursor() as {epoch:string;seq:number};const cursorMs=performance.now()-t0;
 const rows:WorldRowV1[]=paths.map(path=>["l",path,identity(path)]);
 const stat=(subset:readonly WorldRowV1[])=>verifyWorldWitnessV1({observations:subset},{});
 const time=async<T>(body:()=>Promise<T>)=>{const start=performance.now(),cpu=process.cpuUsage(),value=await body(),used=process.cpuUsage(cpu);return{ms:performance.now()-start,cpu:(used.user+used.system)/1000,value};};
 const before=[],after=[],fences=[];
 for(let run=0;run<runs;run++){before.push(await time(()=>stat(rows)));after.push(await time(()=>checkWorldViaJournalV1({client,cursor,workspace:sandbox,rows,stat})));}
 for(let run=0;run<50;run++){const s=performance.now();await client.since(cursor,["src/p1"]);fences.push(performance.now()-s);}
 const one=await time(()=>(utimesSync(paths[7]!,new Date(),new Date(Date.now()+3000)),checkWorldViaJournalV1({client,cursor,workspace:sandbox,rows,stat})));
 for(let i=0;i<100;i++)writeFileSync(paths[i*17]!,"edited "+i);
 const many=await time(()=>checkWorldViaJournalV1({client,cursor,workspace:sandbox,rows,stat}));
 const brief=(v:{ms:number;cpu:number}[])=>({wallMs:Math.round(median(v.map(x=>x.ms))),cpuMs:Math.round(median(v.map(x=>x.cpu)))});
 console.log(JSON.stringify({files:paths.length,dirs,cursorMs:Math.round(cursorMs),statWalk:brief(before),journalQuiet:{...brief(after),via:after[0]!.value.via,statted:after[0]!.value.statted,journalled:after[0]!.value.journalled},fenceP50Ms:+median(fences).toFixed(1),fenceMaxMs:+Math.max(...fences).toFixed(1),oneEdit:{wallMs:Math.round(one.ms),statted:one.value.statted,holds:one.value.verdict.holds,via:one.value.via,refusal:one.value.refusal},hundredEdits:{wallMs:Math.round(many.ms),statted:many.value.statted,holds:many.value.verdict.holds,via:many.value.via,refusal:many.value.refusal}}));
}finally{
 try{const{readChangeStateV1}=await import("../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts");const state=readChangeStateV1(directory);if(state)process.kill(state.pid,"SIGTERM");}catch{}
 rmSync(sandbox,{recursive:true,force:true});
}
