import {existsSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
const R="/Users/Shared/semio/",G=JSON.parse(readFileSync(process.argv[2],"utf8")) as {graph:Record<string,string[]>;names:Record<string,string>;top:{n:number;nt:number;name:string;dir:string}[]};
const owner=new Map<string,string>(),memo=new Map<string,string>();
const projectOf=(dir:string):string=>{
 const hit=memo.get(dir);if(hit!==undefined)return hit;
 let x=dir,r="";
 while(true){if(existsSync(join(R,x,"📋️project.json"))){r=x;break;}const up=dirname(x);if(up===x||up===".")break;x=up;}
 memo.set(dir,r);return r;
};
for(const d of Object.keys(G.graph))owner.set(d,projectOf(d));
const closure=new Map<string,Set<string>>();
const clo=(d:string):Set<string>=>{let s=closure.get(d);if(s)return s;s=new Set();closure.set(d,s);for(const e of G.graph[d]??[]){s.add(e);for(const f of clo(e))s.add(f);}return s;};
for(const d of Object.keys(G.graph))clo(d);
const projects=new Set(owner.values());
console.log("crates",Object.keys(G.graph).length,"projects owning crates",projects.size);
const out:any[]=[];
for(const t of G.top.slice(0,5)){
 const q=owner.get(t.dir)!;
 const depCrates=[...Object.keys(G.graph)].filter(c=>c!==t.dir&&clo(c).has(t.dir));
 const stalesProjectLevel=[...Object.keys(G.graph)].filter(c=>owner.get(c)!==q&&[...clo(c)].some(x=>owner.get(x)===q));
 const projs=new Set(stalesProjectLevel.map(c=>owner.get(c)!));
 const siblings=[...Object.keys(G.graph)].filter(c=>owner.get(c)===q).length;
 out.push({crate:t.name,project:q,crateDependents:depCrates.length,projectLevelStaledCrates:stalesProjectLevel.length,staledProjects:projs.size,cratesInEditedProject:siblings});
}
console.log(JSON.stringify(out,null,1));
