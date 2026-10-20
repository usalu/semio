import {plugin} from "bun";
import {readFileSync} from "node:fs";
const map=JSON.parse(readFileSync(process.env.EP2_OVERLAY!,"utf8")) as Record<string,string>;
const escape=(value:string)=>value.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
if(Object.keys(map).length)plugin({name:"ep2-overlay",setup(build){build.onLoad({filter:new RegExp("^(?:"+Object.keys(map).map(escape).join("|")+")$")},args=>({contents:readFileSync(map[args.path]!,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"}));}});
