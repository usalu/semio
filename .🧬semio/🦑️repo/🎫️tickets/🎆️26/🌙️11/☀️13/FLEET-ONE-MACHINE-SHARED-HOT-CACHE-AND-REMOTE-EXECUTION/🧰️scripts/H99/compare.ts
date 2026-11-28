/** 🧭️ H99 proof: the TypeScript verdict of every build script of the cargo cache agrees with the Python inventory (independent regex model). Usage: bun compare.ts <inv.json> <probe module> <out.json>. */
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";

const [inventory,module,out]=process.argv.slice(2) as [string,string,string];
const {buildScriptProbeV1,buildScriptVerdictV1}=await import(module) as typeof import("./stage/buildscript/🧭️probe/🟦️.ts");
const rows=JSON.parse(readFileSync(inventory,"utf8")) as Array<{crate:string;path:string;probe:string;class:string}>;
const declared=(name:string):boolean=>["TARGET","HOST","PROFILE","OPT_LEVEL","DEBUG","NUM_JOBS","OUT_DIR","CARGO_MANIFEST_DIR"].includes(name)||/^(CARGO_PKG_|CARGO_CFG_|CARGO_FEATURE_)/u.test(name);
const dependencies=(path:string):string[]=>{
 const manifest=join(dirname(path),"Cargo.toml");if(!existsSync(manifest))return[];
 const text=readFileSync(manifest,"utf8"),names:string[]=[];
 for(const block of text.matchAll(/^\[(?:[\w.'"() -]+\.)?build-dependencies\]([^[]*)/gmu))for(const row of block[1]!.matchAll(/^\s*"?([A-Za-z0-9_-]+)"?\s*[=.]/gmu))names.push(row[1]!.replaceAll("-","_"));
 return names;
};
const result=rows.map(row=>{
 const source=readFileSync(row.path,"utf8"),deps=dependencies(row.path),targetOs="macos";
 const probe=buildScriptProbeV1({source,dependencies:deps,targetOs}).class;
 const verdict=buildScriptVerdictV1({source,dependencies:deps,environment:{CARGO_CFG_TARGET_OS:targetOs},declared});
 return{crate:row.crate,python:row.probe,typescript:probe,verdict:verdict.kind==="portable"?"portable":verdict.reason,agree:probe===row.probe};
});
writeFileSync(out,JSON.stringify(result,null,1));
const disagree=result.filter(row=>!row.agree);
console.log(JSON.stringify({total:result.length,agree:result.length-disagree.length,disagree:disagree.map(row=>[row.crate,row.python,row.typescript])}));
const counts:Record<string,number>={};for(const row of result)counts[row.verdict]=(counts[row.verdict]??0)+1;
console.log(JSON.stringify(counts));
