/** 🧾️ Publishes the paired Semio native allocation-stage joins after authentic test-only demand receipts. */
import {readFileSync,writeFileSync} from "node:fs";
const pairs=JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/root-semio-native-cumulative-admission-held-provider-pairs.json","utf8")) as {path:string,before:string,after:string}[];
for(const pair of pairs){const current=readFileSync(pair.path,"utf8");if(current!==pair.before&&current!==pair.after)throw Error("Concurrent Semio native control change: "+pair.path);}
for(const pair of pairs){const current=readFileSync(pair.path,"utf8");if(current===pair.after)continue;if(current!==pair.before)throw Error("Concurrent Semio native control publication change: "+pair.path);writeFileSync(pair.path,pair.after);}
console.log("[DEBUG] Mounted Semio native cumulative allocation-stage provider joins="+pairs.length);

