import{readFileSync}from"node:fs";import{resolve,dirname}from"node:path";
const root=resolve(import.meta.dir,"../../../../../../..");const plugin=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const files:string[]=[];const walk=(p:string)=>{if(files.includes(p))return;files.push(p);for(const m of readFileSync(p,"utf8").matchAll(/#\[path\s*=\s*"([^"\n]+)"\]|include!\(\s*"([^"\n]+)"\s*\)/g))walk(resolve(dirname(p),(m[1]??m[2])!))};
walk(resolve(plugin,"🧪️testing/🧪️artifact-app-laws/🦀️.rs"));console.log(files.map(f=>f.slice(plugin.length)).join("\n"));
