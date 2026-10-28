import {spawnSync} from "node:child_process";import {mkdtempSync,readFileSync,writeFileSync} from "node:fs";import {tmpdir} from "node:os";import {join,resolve} from "node:path";
const module_=resolve(process.argv[2]!),scratch=mkdtempSync(join(tmpdir(),"red-g-trace-")),plugin=join(scratch,"p.ts"),driver=join(scratch,"d.ts"),bare=join(scratch,"bunfig.toml"),out=join(scratch,"o.json");
writeFileSync(bare,"");
writeFileSync(plugin,'import {readFileSync,writeFileSync} from "node:fs";const seen:string[]=[];Bun.plugin({name:"l",setup(build){build.onLoad({filter:/\\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});process.on("exit",()=>{writeFileSync(process.env.O!,JSON.stringify(seen));});\n');
writeFileSync(driver,'await import(process.argv[2]!);\n');
const r=spawnSync(process.execPath,["--config="+bare,"--preload",plugin,driver,module_],{encoding:"utf8",env:{...process.env,SEMIO_LANGUAGE:"en",SEMIO_REPO_IMPLEMENTATION:"live",O:out}});
const seen=JSON.parse(readFileSync(out,"utf8")) as string[];
import {statSync} from "node:fs";const loaded=seen.filter(p=>!p.startsWith(scratch)),bytes=loaded.reduce((sum,p)=>sum+statSync(p).size,0);console.log("status",r.status,"modules",loaded.length,"bytes",bytes);
const root="/Users/ueli/Documents/semio/";for(const p of seen)console.log(p.replace(root,"").replace("🧰️framework/","").replace("🛍️products/🦑️repo/🔨️modules/📚️library/","L/"));
