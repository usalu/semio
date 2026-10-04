/** 🧪️ Mounts the exact test-only cumulative Semio native admission demand after whole-file guards. */
import {readFileSync,writeFileSync} from "node:fs";
const pairs=JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/root-semio-native-cumulative-admission-test-demand-pairs.json","utf8")) as {path:string,before:string,after:string}[];
for(const pair of pairs){const current=readFileSync(pair.path,"utf8");if(current!==pair.before&&current!==pair.after)throw Error("Concurrent demand change: "+pair.path);}
for(const pair of pairs){const current=readFileSync(pair.path,"utf8");if(current===pair.after)continue;if(current!==pair.before)throw Error("Concurrent demand publication change: "+pair.path);writeFileSync(pair.path,pair.after);}
console.log("[DEBUG] Mounted Semio admission test-only demand files="+pairs.length);
