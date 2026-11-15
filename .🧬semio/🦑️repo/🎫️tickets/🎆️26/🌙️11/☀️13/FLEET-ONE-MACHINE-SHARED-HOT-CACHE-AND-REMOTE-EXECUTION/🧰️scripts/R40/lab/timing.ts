import {interfaceFileFactsV1} from "./iface-b/🟦️.ts";
import {readFileSync} from "node:fs";
const files=JSON.parse(readFileSync("rsfiles.json","utf8")) as string[];
let bytes=0,voidN=0,spliceN=0,bodies=0;const t0=performance.now();let tr=0;
for(const f of files){let b;const a=performance.now();try{b=readFileSync("/Users/Shared/semio/"+f);}catch{continue;}tr+=performance.now()-a;bytes+=b.length;const x=interfaceFileFactsV1(b);if(x.void)voidN++;if(x.splice)spliceN++;bodies+=x.bodies;}
const t1=performance.now();
console.log(JSON.stringify({files:files.length,MB:+(bytes/1e6).toFixed(1),readMs:Math.round(tr),totalMs:Math.round(t1-t0),lexMs:Math.round(t1-t0-tr),voidFiles:voidN,spliceFiles:spliceN,maskedBodies:bodies}));
