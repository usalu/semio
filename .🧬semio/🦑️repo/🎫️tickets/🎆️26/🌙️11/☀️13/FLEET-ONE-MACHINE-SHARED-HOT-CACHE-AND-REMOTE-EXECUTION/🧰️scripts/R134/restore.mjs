import {readFileSync,writeFileSync,readdirSync,rmSync,rmdirSync,existsSync} from "node:fs";
import {dirname} from "node:path";
import {SC} from "./patches.mjs";
const BACKUP=SC+"backup";
for(const name of readdirSync(BACKUP).filter(n=>n.endsWith(".path"))){const file=readFileSync(BACKUP+"/"+name,"utf8"),orig=BACKUP+"/"+name.replace(/\.path$/u,".orig");if(existsSync(orig)){writeFileSync(file,readFileSync(orig));console.log("restored",file.split("/").pop());}}
if(existsSync(BACKUP+"/created.json"))for(const file of JSON.parse(readFileSync(BACKUP+"/created.json","utf8"))){rmSync(file,{force:true});try{rmdirSync(dirname(file));}catch{}try{rmdirSync(dirname(dirname(file)));}catch{}console.log("removed",file.split("/").pop());}
