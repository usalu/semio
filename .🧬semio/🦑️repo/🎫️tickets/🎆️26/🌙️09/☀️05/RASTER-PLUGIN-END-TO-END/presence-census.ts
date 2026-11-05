import {readdirSync,readFileSync}from"node:fs";
const groups=new Map<string,string[]>();
const walk=(path:string)=>{for(const entry of readdirSync(path,{withFileTypes:true})){if(entry.name.startsWith(".")||["node_modules","target","vendor"].includes(entry.name))continue;const file=path+"/"+entry.name;if(entry.isDirectory())walk(file);else if(entry.isFile()&&file.endsWith(".rs")){const source=readFileSync(file,"utf8");for(const match of source.matchAll(/\btype\s+Presence\s*=\s*([^;]+);/g)){const value=match[1]!.trim();const files=groups.get(value)??[];files.push(file);groups.set(value,files);}}}};
for(const path of process.argv.slice(2))walk(path);
console.log(JSON.stringify([...groups].map(([type,files])=>({type,count:files.length,files:/NoPresence$|::Presence$/.test(type)?[]:files}))));
