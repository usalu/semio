import {resolve} from "node:path";
import {closure} from "./closure-of.ts";
const drivers=["/🏃️process/🧪️testing/🦀️cargo/","/🏃️process/📦️artifacts/🏗️native-build/","/🏃️process/📦️artifacts/🕸️wasm-build/"];
for(const a of process.argv.slice(2)){const set=closure([resolve(a)]);const hit=[...set].filter(f=>drivers.some(d=>f.includes(d)));console.log(set.size,hit.length,a.slice(-60));for(const h of hit.slice(0,3))console.log("   ",h.slice(-90));}
