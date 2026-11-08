import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const root="/Users/ueli/Documents/semio/",read=(p:string)=>JSON.parse(readFileSync(root+p,"utf8"));
const list=(flags:string[])=>spawnSync("git",["ls-files","-z",...flags,"--","*project.json",":!.🧬semio"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(Boolean);
const files=[...new Set([...list([]),...list(["-o","--exclude-standard"])])];
const workspace=Object.keys(read("nx.json").namedInputs??{});
const bad:string[]=[];
const walk=(row:unknown,declared:Set<string>,where:string)=>{
 if(typeof row==="string"){const name=row.startsWith("^")?row.slice(1):row;if(!name||name.includes("{")||declared.has(name))return;bad.push(where+" -> "+JSON.stringify(row));return;}
 if(Array.isArray(row))for(const item of row)walk(item,declared,where);
 else if(row&&typeof row==="object"&&typeof (row as {input?:unknown}).input==="string")walk((row as {input:string}).input,declared,where);
};
for(const file of files){let project:any;try{project=read(file);}catch{continue;}
 const declared=new Set([...workspace,...Object.keys(project.namedInputs??{}),...Object.keys(project.metadata?.sourceInputs??{})]);
 for(const [target,body] of Object.entries<any>(project.targets??{}))walk(body?.inputs,declared,file+" #"+target);
}
console.log(files.length,"project.json files;",bad.length,"undeclared named inputs");for(const line of bad)console.log(line);
process.exit(bad.length?1:0);
