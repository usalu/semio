import {readFileSync,writeFileSync} from "node:fs";
const dir=process.argv[2];
const text=readFileSync(dir+"/compile-plan.md","utf8").split("\n");
const out=[];let section="";
for(const line of text){if(line.startsWith("### "))section=line.slice(4).split(" ")[0]+" "+line.slice(4).split("(")[0].trim().slice(0,60);else if(line.startsWith("## "))section=line.slice(3,10);
 if(line.startsWith("pl ")){const cmd=line.replace(/\s+#.*$/,"").trim();out.push({section,cmd});}}
writeFileSync(dir+"/cc-tools/batches.json",JSON.stringify(out,null,1));
console.log(out.length);
for(const [i,b] of out.entries())console.log(i,b.section.slice(0,50),"|",b.cmd.slice(0,110));
