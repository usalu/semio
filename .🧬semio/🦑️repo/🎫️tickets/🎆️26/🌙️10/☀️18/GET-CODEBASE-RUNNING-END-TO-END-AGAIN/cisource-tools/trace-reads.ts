import {mock} from "bun:test";
import * as real from "node:fs";
const out=process.env.CISOURCE_TRACE_OUT!,root=process.env.CISOURCE_TRACE_ROOT!,actual={...real} as any,names=["readFileSync","existsSync","statSync","readdirSync","openSync"];
const wrap=(name:string)=>(...args:any[])=>{if(typeof args[0]==="string"&&args[0].startsWith(root))actual.appendFileSync(out,name+"\t"+args[0].slice(root.length)+"\n");return actual[name](...args);};
const traced=Object.fromEntries(names.map(name=>[name,wrap(name)]));
mock.module("node:fs",()=>({...actual,...traced,default:{...actual,...traced}}));
