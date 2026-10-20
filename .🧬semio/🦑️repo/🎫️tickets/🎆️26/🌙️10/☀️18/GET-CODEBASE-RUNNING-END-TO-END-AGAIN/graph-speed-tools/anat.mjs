import {readdirSync,readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const dir="/Users/ueli/Documents/semio/.cache/b/g/m/inference/np";
const files=readdirSync(dir).filter(f=>f.endsWith(".json")).sort();
const agg=new Map();let total=0,count=0;
function walk(v,path){
 if(Array.isArray(v)){const s=JSON.stringify(v).length;add(path+"[]",s);for(const x of v)if(x&&typeof x==="object")walk(x,path+"[]");return;}
 if(v&&typeof v==="object"){for(const[k,x]of Object.entries(v)){const key=(path?path+".":"")+(path.split(".").length>3?"*":k);const s=JSON.stringify(x)?.length??0;add(key,s);if(x&&typeof x==="object")walk(x,key);}}
}
function add(k,s){agg.set(k,(agg.get(k)||0)+s);}
for(let i=0;i<files.length;i+=12){
 let text;try{text=brotliDecompressSync(readFileSync(dir+"/"+files[i])).toString("utf8")}catch{continue}
 const body=JSON.parse(text.slice(text.indexOf("\n")+1));total+=text.length;count++;
 walk(body,"");
}
console.log(count,total/count);
console.log([...agg.entries()].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>k+" "+Math.round(v/count)).join("\n"));
