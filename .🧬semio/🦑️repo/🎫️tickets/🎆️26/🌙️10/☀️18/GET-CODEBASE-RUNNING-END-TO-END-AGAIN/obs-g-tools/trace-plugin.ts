// [obs-g] preload: records every ts/mjs module loaded by the process into $LOADED_MODULES_OUT (same recorder as the door startup law)
import {readFileSync,writeFileSync} from "node:fs";
const seen:string[]=[];
Bun.plugin({name:"loaded-modules",setup(build){build.onLoad({filter:/\.(ts|mjs)$/},args=>{seen.push(args.path);return {contents:readFileSync(args.path,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"};});}});
process.on("exit",()=>{writeFileSync(process.env.LOADED_MODULES_OUT!,JSON.stringify(seen));});
