import {readdirSync,statSync,existsSync} from "node:fs";
const base="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(base);const keys=new Set(names.filter(n=>n.endsWith(".json.records")).map(n=>n.slice(0,24)));
const json=new Set(names.filter(n=>n.endsWith(".json")).map(n=>n.slice(0,24)));const fleet=new Set(names.filter(n=>n.endsWith(".json.fleet")).map(n=>n.slice(0,24)));
const h=(t)=>new Date(t).toISOString().slice(5,13);
const cat={both:{},recOnly:{},jsonOnly:{}};let bytes={both:0,recOnly:0};
for(const k of keys){const d=base+k+".json.records/";let newest=0,sz=0,cnt=0;for(const f of readdirSync(d)){try{const s=statSync(d+f);newest=Math.max(newest,s.mtimeMs);sz+=s.size;cnt++}catch{}}
 const c=json.has(k)?"both":"recOnly";const e=cat[c][h(newest)]=(cat[c][h(newest)]||0)+1;bytes[c]+=sz}
console.log("records dirs",keys.size,"json",json.size,"fleet",fleet.size,"bytes MB",{both:bytes.both/1e6|0,recOnly:bytes.recOnly/1e6|0});
console.log("recOnly newest-pack hour:",JSON.stringify(Object.fromEntries(Object.entries(cat.recOnly).sort())));
console.log("both newest-pack hour:",JSON.stringify(Object.fromEntries(Object.entries(cat.both).sort())));
let jo=0;for(const k of json)if(!keys.has(k))jo++;console.log("json without records",jo);
