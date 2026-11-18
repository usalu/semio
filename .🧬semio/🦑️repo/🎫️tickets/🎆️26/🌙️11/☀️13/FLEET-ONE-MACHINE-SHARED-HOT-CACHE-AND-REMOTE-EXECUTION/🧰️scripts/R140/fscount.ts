import {createRequire} from "node:module";
import {writeFileSync} from "node:fs";
const require=createRequire(import.meta.url),fs=require("fs");
export const counts:Record<string,{n:number;ms:number;distinct:Set<string>}>={};
const started=performance.now(),budget=Number(process.env.R140_BUDGET_MS??0);
let dumped=false;
export function dump(path:string){if(dumped)return;dumped=true;writeFileSync(path,JSON.stringify(Object.fromEntries(Object.entries(counts).map(([k,v])=>[k,{n:v.n,distinct:v.distinct.size,ms:Math.round(v.ms)}])),null,1));}
for(const name of ["lstatSync","statSync","existsSync","accessSync","readdirSync","readFileSync","realpathSync","openSync"]){
 const original=fs[name];
 fs[name]=function(path:any,...rest:any[]){const s=performance.now();try{return original.call(this,path,...rest);}finally{const row=counts[name]??={n:0,ms:0,distinct:new Set()};row.n++;row.ms+=performance.now()-s;if(row.distinct.size<3e6)row.distinct.add(String(path));if(budget>0&&performance.now()-started>budget){dump(process.env.R140_OUT+"/fscount.json");process.exit(0);}}};
}
