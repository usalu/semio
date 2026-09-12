import { readFileSync, writeFileSync, readdirSync, existsSync, renameSync } from "node:fs";
import { join, resolve, relative } from "node:path";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=resolve(import.meta.dir,"../..");
const owner="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle",test=owner+"/🧪️tests/🌊️actor-import",report=join(ticket,"📓️browser-dispatch-taxonomy-2026-09-12.md");
const guard=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts")).href);
const sources=readdirSync(join(root,test)).map(name=>({path:test+"/"+name,source:readFileSync(join(root,test,name),"utf8")}));
const findings=guard.inspectTestLayoutSources(guard.testTaxonomy(root),sources,[test]);
const section="\n## "+process.argv[2]+"\n\n```json\n"+JSON.stringify(findings,null,2)+"\n```\n";
if(process.argv[2]==="baseline"){
 writeFileSync(report,"# Browser Actor Import Dispatch Taxonomy — 2026-09-12\n\nThe assertion implementation remains a direct case leaf. Nx project metadata and executable dispatch belong to the browser-bundle semantic owner.\n"+section);
 writeFileSync(join(import.meta.dir,"📥️input.md"),"# Browser Dispatch Relocation Input\n\n"+sources.filter(s=>!s.path.endsWith("/🟦️.ts")).map(s=>"## "+s.path+"\n\nSHA-256 `"+createHash("sha256").update(s.source).digest("hex")+"`\n\n```text\n"+s.source+"\n```").join("\n"));
 if(!findings.length)throw Error("Expected current case metadata violations");
}else if(process.argv[2]==="verify"){
 writeFileSync(report,readFileSync(report,"utf8")+section);
 if(findings.length)throw Error("Browser case still violates taxonomy");
 const project=JSON.parse(readFileSync(join(root,owner,"📋️project.json"),"utf8"));
 if(!existsSync(resolve(root,owner,project.$schema)))throw Error("Nx schema missing");
 const ts=await import("typescript");
 const parsed=ts.createSourceFile("📜️script.ts",readFileSync(join(root,owner,"📜️script.ts"),"utf8"),ts.ScriptTarget.Latest,true);
 if(parsed.parseDiagnostics.length)throw Error(JSON.stringify(parsed.parseDiagnostics));
 for(const p of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
 const value=ts.parseConfigFileTextToJson(p,readFileSync(join(root,p),"utf8"));if(value.error)throw Error("Launch JSONC invalid");
 const commands=value.config.configurations.map((c:any)=>c.command).filter((v:any)=>typeof v==="string");
 for(const name of Object.keys(project.targets))if(!commands.includes("bun nx run "+project.name+":"+name))throw Error("Launch target missing: "+name);
 }
 writeFileSync(report,readFileSync(report,"utf8")+"\nThe actual layout guard reports zero case findings. The TypeScript parser accepted the owner dispatcher, JSONC parsing succeeded for both launch files, and both launch commands resolve to the retained project name and targets. No full JCO actor runtime success is inferred from these structural checks.\n\nExact edits: removed `"+test+"/📜️script.ts`, moved `"+test+"/📋️project.json` to `"+owner+"/📋️project.json`, extended `"+owner+"/📜️script.ts`.\n");
}
console.log(JSON.stringify({phase:process.argv[2],findings:findings.length,report}));

