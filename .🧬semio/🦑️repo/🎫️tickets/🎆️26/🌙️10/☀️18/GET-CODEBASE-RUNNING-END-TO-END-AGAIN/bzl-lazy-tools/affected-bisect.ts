// 🔍 Finds the changed files for which Nx marks every project affected while the neutral rule does not.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {affectedBuildProjectsV1} from "./run/selection.ts";
const dir=join(import.meta.dir,"..","🗑️generated","bzl-lazy");
const mod=await import("./affected-xcheck-lib.ts");
const graph=JSON.parse(readFileSync(join(dir,"graph.json"),"utf8")),sets:Record<string,string[]>=JSON.parse(readFileSync(join(dir,"change-sets.json"),"utf8"));
const total=Object.keys(graph.projects).length;
for(const name of ["root-manifests-teaching-mit-bestand"]){
 let files=sets[name]!;
 const all=(f:string[])=>mod.nx(graph,f).length===total;
 if(!all(files)){console.log(name,"not all");continue;}
 const culprits:string[]=[];
 const search=(f:string[])=>{if(f.length===1){culprits.push(f[0]!);return;}const half=Math.ceil(f.length/2),a=f.slice(0,half),b=f.slice(half);for(const part of [a,b])if(all(part))search(part);};
 search(files);
 console.log(name,JSON.stringify(culprits.slice(0,10)),"neutral for culprit:",culprits[0]?affectedBuildProjectsV1(graph,[culprits[0]]).length:0);
}
