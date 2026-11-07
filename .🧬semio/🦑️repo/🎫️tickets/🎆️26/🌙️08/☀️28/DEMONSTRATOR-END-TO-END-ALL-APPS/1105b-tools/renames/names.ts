import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,dirname} from "node:path";
const R="/Users/ueli/Documents/semio",SKIP=new Set(["node_modules","target",".git","🗑️generated","🎫️tickets","dist",".🧬semio",".cache"]);
const out:string[]=[];
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(SKIP.has(e.name)||e.name.startsWith(".cache"))continue;walk(join(d,e.name));}else if(e.name==="Cargo.toml"){const m=readFileSync(join(d,e.name),"utf8").match(/^\[package\][^\[]*?^name\s*=\s*"([^"]+)"/ms);if(m)out.push(m[1]+"\t"+d.slice(R.length+1));}}};
walk(R);writeFileSync("names.tsv",out.join("\n"));
const c=new Map<string,number>();for(const l of out){const n=l.split("\t")[0];const k=n.split("-").slice(0,3).join("-");c.set(k,(c.get(k)??0)+1);}
console.log(out.length,[...c].sort((a,b)=>b[1]-a[1]).slice(0,25));
