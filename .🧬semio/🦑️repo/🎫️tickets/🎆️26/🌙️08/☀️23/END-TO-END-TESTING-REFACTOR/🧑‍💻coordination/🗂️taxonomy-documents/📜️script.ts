import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_TAXONOMY_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const generated = join(ticket, "🗑️generated/testing-taxonomy/documents");
mkdirSync(generated, {recursive:true});
const report = join(ticket, "📓️document-oracle-classification-2026-09-12.md");
const oldPath = "🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
const newPath = oldPath.replace("🧪️testkit", "🔮️oracles");
const inputPath = join(import.meta.dir, "📥️input.md");
const command = process.argv[2];
if (command === "capture") {
 const child = Bun.spawn(["rg", "-l", "-F", "🧪️testkit/🪪️document-contract", "✏️s", "-g", "*.ts", "-g", "!node_modules", "-g", "!🗑️generated"], {cwd:root,stdout:"pipe",stderr:"inherit"});
 const paths = (await new Response(child.stdout).text()).trim().split("\n").filter(Boolean).sort();
 assert.equal(await child.exited,0); assert.equal(paths.length,11);
 const input = {oldPath,newPath,sha256:createHash("sha256").update(readFileSync(join(root,oldPath))).digest("hex"),consumers:paths};
 writeFileSync(inputPath,"# Document Oracle Relocation Input\n\n```json\n"+JSON.stringify(input,null,2)+"\n```\n");
 console.log("[DEBUG] captured unchanged comparator and 11 current consumers");
} else if (command === "baseline" || command === "verify") {
 const input = JSON.parse(readFileSync(inputPath,"utf8").match(/```json\n([\s\S]*?)\n```/)![1]);
 if(command==="verify") {
  assert(!existsSync(join(root,oldPath)));
  assert.equal(createHash("sha256").update(readFileSync(join(root,newPath))).digest("hex"),input.sha256);
  for(const path of input.consumers) {const source=readFileSync(join(root,path),"utf8");assert(source.includes("🔮️oracles/🪪️document-contract"));assert(!source.includes("🧪️testkit/🪪️document-contract"));}
 }
 const results = [];
 for(const path of input.consumers) {
  try {
   const module = await import(pathToFileURL(join(root,path)).href);
   const entries=Object.entries(module).filter(([name,value])=>/^test.*DocumentContractOracle$/.test(name)&&typeof value==="function");
   assert.equal(entries.length,1,path);
   await (entries[0]![1] as Function)();
   results.push({path,status:"passed"});
  } catch(error) {results.push({path,status:"failed",message:error instanceof Error?error.message:String(error)});}
 }
 writeFileSync(join(generated,command+".json"),JSON.stringify(results,null,2)+"\n");
 const summary={phase:command,passed:results.filter(x=>x.status==="passed").length,failed:results.filter(x=>x.status==="failed").length};
 console.log("[DEBUG] "+JSON.stringify(summary));
 const text="# Document Oracle Classification — 2026-09-12\n\nThe shared comparator is an independent Ajv oracle for owner document facets and committed mutation fixtures. It belongs under the schema owner's canonical oracle collection. Its body is byte-preserved; only its path and direct test imports change.\n\n## Runtime "+command+"\n\n```json\n"+JSON.stringify({summary,results},null,2)+"\n```\n\n## Exact Authored Paths\n\n```json\n"+JSON.stringify([oldPath,newPath,...input.consumers],null,2)+"\n```\n";
 if(command==="baseline") writeFileSync(report,text);
 else writeFileSync(report,readFileSync(report,"utf8")+"\n## Final Relocation Verification\n\nThe original path is absent, the relocated comparator's SHA-256 matches the retained input, and all 11 imports name the canonical oracle collection.\n\n```json\n"+JSON.stringify({summary,results},null,2)+"\n```\n");
} else throw new Error("Expected capture, baseline or verify");

