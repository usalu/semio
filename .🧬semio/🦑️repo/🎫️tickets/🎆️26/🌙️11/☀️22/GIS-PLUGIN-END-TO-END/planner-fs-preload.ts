import {plugin} from "bun";
import realFs from "node:fs";
const names=["statSync","lstatSync","readdirSync","realpathSync","readFileSync","existsSync","openSync","readlinkSync","accessSync"];
const counts:Record<string,{calls:number;paths:Map<string,number>}>={};
(globalThis as any).__fsCounts=counts;
const wrapped:Record<string,any>={...realFs};
for(const name of names){const original=(realFs as any)[name];const fn=function(this:unknown,...args:any[]){const c=counts[name]??={calls:0,paths:new Map()};c.calls++;const k=String(args[0]);c.paths.set(k,(c.paths.get(k)??0)+1);return original.apply(this,args);};Object.assign(fn,original);wrapped[name]=fn;}
(globalThis as any).__fsWrapped=wrapped;
plugin({name:"fs-count",setup(build){
 build.onResolve({filter:/^(node:)?fs$/},()=>({path:"fs-count",namespace:"fscount"}));
 build.onLoad({filter:/.*/,namespace:"fscount"},()=>({contents:"const w=globalThis.__fsWrapped;export default w;"+Object.keys(wrapped).filter(k=>/^[A-Za-z_$][\w$]*$/.test(k)&&k!=="default").map(k=>`export const ${k}=w.${k};`).join(""),loader:"js"}));
}});
process.on("exit",()=>{for(const [name,entry] of Object.entries(counts)){const top=[...entry.paths].sort((a,b)=>b[1]-a[1]).slice(0,3).map(([p,n])=>n+"x "+p.slice(-70));console.error("fs."+name,"calls",entry.calls,"distinct",entry.paths.size,"repeat",(entry.calls/entry.paths.size).toFixed(1),JSON.stringify(top));}});
