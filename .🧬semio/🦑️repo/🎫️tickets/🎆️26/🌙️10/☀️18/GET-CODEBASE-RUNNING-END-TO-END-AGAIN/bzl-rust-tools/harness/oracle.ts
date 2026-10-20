import {readFileSync,readdirSync,existsSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {spawnSync} from "node:child_process";

/** 🔬️ Third-party oracle: rustc's own dep-info lists exactly the files each crate read; one ingress check of the leaf crate emits the artifact paths of all three. */
const root=process.cwd(),ws=process.env.BZLRUST_WS!,wsRel=relative(root,ws);
const walk=(dir:string):string[]=>readdirSync(dir,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?walk(join(dir,entry.name)):[join(dir,entry.name)]);
const manifest=walk(join(ws,"geometry")).find(path=>path.includes("bulge")&&path.endsWith("Cargo.toml"))!;
const run=spawnSync(process.execPath,["repo","cargo","--store=shared","--configuration=prototype","--parallelism=1","--","check","--manifest-path",relative(root,manifest),"-p","bzlrust-bulge","--lib","--message-format=json-render-diagnostics"],{cwd:root,encoding:"utf8",maxBuffer:1<<28});
const lines=(run.stdout+run.stderr).split("\n"),result:Record<string,string[]>={};
for(const line of lines){
 if(!line.startsWith('{"reason":"compiler-artifact"'))continue;
 const message=JSON.parse(line),pack=/#([^@]+)@/.exec(message.package_id)![1]!,rmeta=(message.filenames as string[]).find(file=>file.endsWith(".rmeta"))!;
 const dep=readdirSync(dirname(rmeta)).filter(name=>name.endsWith(".d")&&name.includes(pack.replaceAll("-","_")));
 const files=new Set<string>();
 for(const name of dep)for(const row of readFileSync(join(dirname(rmeta),name),"utf8").split("\n")){const at=row.indexOf(": ");if(at<0||row.startsWith("#"))continue;for(const token of row.slice(at+2).split(" ")){if(!token)continue;const path=resolve(ws,token);if(existsSync(path))files.add(relative(root,path));}}
 result[pack]=[...files].sort();
}
console.log(run.status,Object.fromEntries(Object.entries(result).map(([key,value])=>[key,value.length])));
writeFileSync(join(import.meta.dir,"oracle-depinfo.json"),JSON.stringify(result,null,1));
