/** 🪪️ Publishes only the guarded canonical Store error join regions while preserving concurrent Store work. */
import {readFileSync,writeFileSync} from "node:fs";
const pair=JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/root-store-command-canonical-refusal-held-producer-regions.json","utf8")) as {path:string,regions:{before:string,after:string}[]};
let current=readFileSync(pair.path,"utf8");const initial=current;for(const region of pair.regions){if(current.includes(region.after))continue;const count=current.split(region.before).length-1;if(count!==1)throw Error("Changed Store error region count="+count);current=current.replace(region.before,region.after);}
if(readFileSync(pair.path,"utf8")!==initial)throw Error("Concurrent Store publication changed guarded file");if(current!==initial)writeFileSync(pair.path,current);
console.log("[DEBUG] Mounted canonical Store refusal/source regions="+pair.regions.length);

