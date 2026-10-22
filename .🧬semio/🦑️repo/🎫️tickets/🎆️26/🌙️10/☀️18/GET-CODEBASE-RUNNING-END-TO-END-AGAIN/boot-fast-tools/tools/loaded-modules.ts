import {readFileSync,writeFileSync} from "node:fs";
const seen:string[]=[];
Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});
process.on("exit",()=>{if(process.env.LOADED_MODULES_OUT)writeFileSync(process.env.LOADED_MODULES_OUT,JSON.stringify(seen));});
