/** 📏️ [td-leases] measures every row of the door startup law (modules, bytes, forbidden groups reached) under a given root, without stopping at the first failing row. usage: bun startup-measure.ts <root> [row-id] [--list] */
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
const [root,only,flag]=process.argv.slice(2);
const dir=join(root!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/🧪️tests/🚦️startup"),fixture=JSON.parse(readFileSync(join(dir,"🔣️.json"),"utf8"));
const scratch=mkdtempSync(join(tmpdir(),"tl-startup-")),plugin=join(scratch,"p.ts"),driver=join(scratch,"d.ts"),bare=join(scratch,"bunfig.toml");
writeFileSync(bare,"");
writeFileSync(plugin,'import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"m",setup(build){build.onLoad({filter:/\\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});\n');
writeFileSync(driver,'await import(process.argv[2]!);\n');
const entry=resolve(dir,"../../📜️script.ts");
for(const row of fixture.rows){
 if(only&&only!==row.id)continue;
 const out=join(scratch,row.id+".json"),args=row.kind==="door"?[entry,...row.args]:[driver,resolve(dir,row.module)];
 const r=spawnSync(process.execPath,["--config="+bare,"--preload",plugin,...args],{encoding:"utf8",timeout:60000,env:{...process.env,SEMIO_LANGUAGE:"en",SEMIO_REPO_SEAL:"live",LOADED_MODULES_OUT:out}});
 const loaded:string[]=JSON.parse(readFileSync(out,"utf8")),bytes=loaded.reduce((s,p)=>s+statSync(p).size,0);
 const reached=fixture.groups.filter((g:any)=>(row.forbid??[]).includes(g.id)).map((g:any)=>[g.id,loaded.filter(p=>g.match.some((m:string)=>p.includes(m))&&!(g.except??[]).some((m:string)=>p.includes(m))).length]).filter(([,n]:any)=>n>0);
 console.log(`${row.id}: exit ${r.status} modules ${loaded.length}/${row.maximumModules??"-"} bytes ${bytes}/${row.maximumBytes??"-"} reached ${JSON.stringify(reached)}${row.expectViolations?" expect "+row.expectViolations:""}`);
 if(flag==="--list")for(const p of loaded)console.log("  "+statSync(p).size+" "+p.replace(root!+"/",""));
}
rmSync(scratch,{recursive:true,force:true});
