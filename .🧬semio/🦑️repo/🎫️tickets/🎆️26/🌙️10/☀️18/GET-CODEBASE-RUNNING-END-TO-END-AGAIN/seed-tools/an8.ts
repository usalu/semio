import { resolveAll, projects } from "./graph.ts";
const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
console.log("projects",projects.size);
for(const [lab,doc] of [["L",L],["S",S]] as const){
 const routes=resolveAll(doc.configurations,doc.inputs);
 const c:Record<string,number>={};for(const r of routes)c[r.status]=(c[r.status]??0)+1;
 console.log(lab,routes.length,JSON.stringify(c));
 const bad=routes.filter(r=>!["ok","ok-noTarget"].includes(r.status));
 const g=new Map<string,any>();for(const r of bad){const k=r.status+" "+r.project+(r.target?":"+r.target:"");const x=g.get(k)??{n:0,cfg:[]};x.n++;if(x.cfg.length<2)x.cfg.push(r.config);g.set(k,x)}
 const rows=[...g].sort((a,b)=>b[1].n-a[1].n);
 console.log("distinct",rows.length);
 Bun.write(`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/seed-run/unresolved-${lab}.json`,JSON.stringify(rows.map(([k,v])=>({k,...v})),null,1));
}
