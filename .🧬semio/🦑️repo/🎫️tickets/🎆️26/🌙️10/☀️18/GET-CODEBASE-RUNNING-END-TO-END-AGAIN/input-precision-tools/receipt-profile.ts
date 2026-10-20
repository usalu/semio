// ⏱️ Times the validation of a stored program receipt per fact kind (which answers cost what).
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
import {readInputPrecisionV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {programHost} from "./program-host.ts";
const dir=process.argv[2]!,name=readdirSync(join(dir,"ip"))[0]!;
const receipt=JSON.parse(brotliDecompressSync(readFileSync(join(dir,"ip",name))).toString("utf8"));
const document=readInputPrecisionV1(JSON.parse(readFileSync(join(import.meta.dir,"staged2","rules.json"),"utf8")));
const program=await programHost({document,data:dir});
const by:Record<string,{n:number;ms:number}>={};
for(const fact of receipt.facts){const t=performance.now();program.fact(fact.kind,fact.path);const row=by[fact.kind]??={n:0,ms:0};row.n++;row.ms+=performance.now()-t;}
const t=performance.now();let n=0;for(const file of Object.keys(receipt.files)){program.fact("digest",file);n++;}
console.log(JSON.stringify({facts:receipt.facts.length,files:n,digestMs:Math.round(performance.now()-t),by:Object.fromEntries(Object.entries(by).map(([k,v])=>[k,{n:v.n,ms:Math.round(v.ms)}]))}));
