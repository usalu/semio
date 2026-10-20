// 🔒 Runs the patched law over its corpus with the strict schema, the glob oracle and, with LAW_REPO=1, over the real repository records resolved through the live inference policy (rules from the staged document).
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {cacheCorrectnessRepositoryFindingsV1,repositoryCacheRecordsV1} from "/private/tmp/ip/law/🟦️.ts";
import {readInputPrecisionV1} from "/private/tmp/ip/run/p/🟦️.ts";
const O="/private/tmp/ip/law",corpus=JSON.parse(readFileSync(O+"/tests/🔣️.json","utf8")),schema=JSON.parse(readFileSync(O+"/tests/🧬️schema/🔣️.json","utf8"));
const ajv=new Ajv({strict:true,allErrors:true});
if(!ajv.validate(schema,corpus))throw new Error("corpus schema: "+JSON.stringify(ajv.errors));
const pairs=(rows:any[])=>rows.map(r=>JSON.stringify([r.id??r[0],r.rule??r[1]])).sort();
let bad=0;
for(const row of corpus.cases){const got=pairs(cacheCorrectnessRepositoryFindingsV1(row.records)),want=pairs(row.expect);if(JSON.stringify(got)!==JSON.stringify(want)){bad++;console.log("CASE FAIL",row.id,got,want);}}
console.log("corpus cases",corpus.cases.length,"failed",bad);
if(process.env.LAW_REPO!=="1")process.exit(bad?1:0);
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const plugin:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await plugin.libraryBootstrap;
const document=readInputPrecisionV1(JSON.parse(readFileSync(process.env.IP_RULES??join(import.meta.dir,"staged2","rules.json"),"utf8")));
const t0=performance.now(),records=repositoryCacheRecordsV1(W,plugin.cacheInternals,undefined,(process.env.LAW_NO_RULES==="1"?undefined:document) as any),findings=cacheCorrectnessRepositoryFindingsV1(records);
const by:Record<string,number>={};for(const f of findings)by[f.rule]=(by[f.rule]??0)+1;
console.log(JSON.stringify({records:records.length,seconds:Math.round(performance.now()-t0)/1000,findings:findings.length,by}));
const ev:Record<string,number>={},rules:Record<string,number>={};for(const r of records){const d=(r.target.metadata as any)?.semio?.inputPrecision;if(d){ev[d.evidence]=(ev[d.evidence]??0)+1;rules[d.rule]=(rules[d.rule]??0)+1;}}
console.log(JSON.stringify({decided:Object.values(ev).reduce((a,b)=>a+b,0),evidence:ev,rules}));
for(const f of findings.filter(f=>f.rule==="whole-repository-input").slice(0,60))console.log(f.id);
process.exit(bad?1:0);
