import { spawnSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const files: string[] = JSON.parse(readFileSync(join(import.meta.dir,"stage-files.json"),"utf8"));
if(process.argv.includes("--ticket")) {
 const ticket=import.meta.dir.replaceAll("\\","/");
 const relative=ticket.slice(process.cwd().replaceAll("\\","/").length+1);
 for(const name of readdirSync(ticket))if(!["🗑️generated","📌️important"].includes(name))files.push(relative+"/"+name);
}
for(let i=0;i<files.length;i+=40){const p=spawnSync("git",["add","--",...files.slice(i,i+40)],{stdio:"inherit",windowsHide:true});if(p.status)process.exit(p.status);}
console.log("Staged explicit task file paths: "+files.length);
