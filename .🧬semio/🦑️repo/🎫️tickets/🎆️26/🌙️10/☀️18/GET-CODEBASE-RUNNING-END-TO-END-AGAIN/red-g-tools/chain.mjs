import {createRequire} from "node:module";
const require=createRequire(import.meta.url);
const [entry,...needles]=process.argv.slice(2);
const b=await require("/Users/ueli/Documents/semio/node_modules/esbuild").build({entryPoints:[entry],bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",external:["bun:*","esbuild"]});
const inputs=b.metafile.inputs,parent=new Map(),queue=[Object.keys(inputs).find(k=>entry.endsWith(k))];
const seen=new Set(queue);
while(queue.length){const p=queue.shift();for(const i of inputs[p]?.imports??[]){if(!seen.has(i.path)){seen.add(i.path);parent.set(i.path,p);queue.push(i.path);}}}
for(const n of needles){const hit=[...seen].find(k=>k.includes(n));if(!hit){console.log(n,"NOT REACHED");continue;}const chain=[];for(let c=hit;c;c=parent.get(c))chain.push(c);console.log(chain.reverse().map(x=>x.split("/").slice(-4).join("/")).join("\n  -> "));}
