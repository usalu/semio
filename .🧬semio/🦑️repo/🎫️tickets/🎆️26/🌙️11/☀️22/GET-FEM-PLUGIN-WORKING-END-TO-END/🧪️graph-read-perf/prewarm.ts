import {readdirSync,lstatSync} from "node:fs";
import {readdir} from "node:fs/promises";
import {join} from "node:path";
const [mode,root,width]=[process.argv[2],process.argv[3],Number(process.argv[4]??64)];
const skip=new Set(["node_modules","target",".git","🗑️generated"]);
const syncWalk=(dir:string)=>{let n=0;const pending=[dir];while(pending.length){const d=pending.pop()!;try{if(!lstatSync(d).isDirectory())continue;}catch{continue;}for(const e of readdirSync(d,{withFileTypes:true})){n++;if(e.isDirectory()&&!e.isSymbolicLink()&&!skip.has(e.name))pending.push(join(d,e.name));}}return n;};
const parallelWalk=async(dir:string)=>{let n=0,active=0;const pending=[dir];await new Promise<void>(done=>{const pump=()=>{while(active<width&&pending.length){const d=pending.pop()!;active++;readdir(d,{withFileTypes:true}).then(entries=>{for(const e of entries){n++;if(e.isDirectory()&&!e.isSymbolicLink()&&!skip.has(e.name))pending.push(join(d,e.name));}},()=>{}).finally(()=>{active--;if(!active&&!pending.length)done();else pump();});}if(!active&&!pending.length)done();};pump();});return n;};
const t=performance.now();
if(mode==="sync")console.log(JSON.stringify({mode,entries:syncWalk(root),ms:Math.round(performance.now()-t)}));
else{const n=await parallelWalk(root);const mid=performance.now();const m=syncWalk(root);console.log(JSON.stringify({mode,entries:n,prewarmMs:Math.round(mid-t),thenSyncMs:Math.round(performance.now()-mid),syncEntries:m}));}
