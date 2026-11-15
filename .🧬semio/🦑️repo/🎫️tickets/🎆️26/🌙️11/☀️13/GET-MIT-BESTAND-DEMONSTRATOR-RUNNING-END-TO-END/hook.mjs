import {readFileSync} from "node:fs";
import {plugin} from "bun";
const swaps=JSON.parse(process.env.SWAPS??"{}"),strip=process.env.STRIP_DIAG==="1";
const tails=[...Object.keys(swaps),...(strip?["script-inputs/🟨️.mjs"]:[])];
if(tails.length)plugin({name:"swap",setup(b){b.onLoad({filter:new RegExp("("+tails.map(t=>t.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|")+")$")},({path})=>{
 for(const [tail,file] of Object.entries(swaps))if(path.endsWith(tail))return{contents:readFileSync(file,"utf8"),loader:"js"};
 const src=readFileSync(path,"utf8");
 return{contents:strip?src.split("\n").filter(l=>!l.trimStart().startsWith("if (!holds) { try { const bad")).join("\n"):src,loader:"js"};});}});
