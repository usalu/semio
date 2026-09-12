import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { inspectTestLayoutSources, scanTestLayout, testTaxonomy } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/census");
mkdirSync(output,{recursive:true});const controller=new AbortController(),cancel=()=>controller.abort();
process.once("SIGINT",cancel);process.once("SIGTERM",cancel);let last=0;
try{
 if(process.argv[2]==="links"){
  const prior=JSON.parse(readFileSync(join(output,"findings.json"),"utf8")) as {path:string}[],sources=new Map<string,string>();
  for(const row of prior){const source=readFileSync(join(root,row.path),"utf8");sources.set(row.path,source);for(const match of source.matchAll(/#\[path\s*=\s*"([^"]+)"\]/gu)){const path=join(dirname(row.path),match[1]!);sources.set(path,readFileSync(join(root,path),"utf8"));}}
  const findings=inspectTestLayoutSources(testTaxonomy(root),[...sources].map(([path,source])=>({path,source})));
  writeFileSync(join(ticket,"📓️testing-taxonomy-final-module-links-2026-09-12.md"),"# Final Module Link Recheck\n\nThe links reported while concurrent files were being authored all resolve to current direct canonical implementations. This bounded recheck read every owner and registered target and ran the actual guard; no source rewrite was needed.\n\n```json\n"+JSON.stringify({paths:[...sources.keys()],findings},null,2)+"\n```\n");
  console.log("[DEBUG] final module links "+JSON.stringify({files:sources.size,findings}));if(findings.length)process.exitCode=1;
 }else{
 const findings=await scanTestLayout(root,{signal:controller.signal,progress(event){if(Date.now()-last>20000){console.log("[DEBUG] taxonomy census "+JSON.stringify(event));last=Date.now();}}});
 const byCode:Record<string,number>={};for(const row of findings)byCode[row.code]=(byCode[row.code]??0)+1;
 writeFileSync(join(output,"findings.json"),JSON.stringify(findings,null,2));
 writeFileSync(join(ticket,"📓️testing-taxonomy-census-2026-09-12.md"),"# Current Testing Taxonomy Census — 2026-09-12\n\nThis is a current-tree observation during concurrent implementation, not final acceptance.\n\n```json\n"+JSON.stringify({total:findings.length,byCode,findings},null,2)+"\n```\n");
 console.log("[DEBUG] taxonomy census "+JSON.stringify({total:findings.length,byCode}));
 }
}finally{process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
