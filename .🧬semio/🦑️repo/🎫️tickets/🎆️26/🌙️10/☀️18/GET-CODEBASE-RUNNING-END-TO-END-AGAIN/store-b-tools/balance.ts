import {rustTokens,rustTokenPairs} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import {readFileSync} from "node:fs";
let bad=0;
for(const f of process.argv.slice(2)){try{const t=readFileSync(f,"utf8");const toks=rustTokens(t);rustTokenPairs(toks);}catch(e){bad++;console.log("BAD",f.slice(-90),String(e).slice(0,160));}}
console.log("checked",process.argv.length-2,"bad",bad);
