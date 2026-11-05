/** 📁️ Collects scoped task paths for the eventual ticket close, without changing repository source. */
const ticket=import.meta.dir,forms="✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms",any=forms+"/🏅️standards/🔖️1/🪆️subsets/✳️any",renderer="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine",ui="🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values";
const sections:{title:string;paths:string[];removed?:boolean}[]=[];
const run=(args:string[])=>{const r=Bun.spawnSync(args);if(r.exitCode!==0)throw Error(r.stderr.toString());return r.stdout.toString();};
const find=(root:string)=>run(["rg","--files","--hidden","--no-ignore",root]).trim().split("\n").filter(Boolean);
for(const report of ["native-prerequisite-files.md","extension-package-prerequisite.md"]){const s=await Bun.file(ticket+"/"+report).text();const paths=s.split("\n").filter(l=>l.startsWith("- ✏️s/")).map(l=>l.slice(2).trim());if(report==="native-prerequisite-files.md")paths.push("🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs");sections.push({title:report,paths});}
const oldLog=await Bun.file(ticket+"/🗑️generated/renderer-check-final.log").text();
const renderPaths=[...oldLog.matchAll(/\.\.\/\.\.\/\.\.\/\.\.\/([^\n(]+)\(\d+,\d+\): error TS/g)].map(m=>renderer+"/"+m[1]);
renderPaths.push(renderer+"/🧱️elements/🏛️ShellHost/🔶️core/🟦️.tsx",...find(renderer+"/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document/🔏️archive-owner"),renderer+"/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json",renderer+"/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts");
sections.push({title:"Renderer prerequisites (renderer-typecheck-results.md)",paths:renderPaths});
sections.push({title:"Generic bounded text chunks",paths:[...find(ui+"/🔤️text-chunks"),ui+"/🦀️.rs"]});
sections.push({title:"Public PatchQuestions transport and law",paths:find(any+"/✏️editor/🎮️commands/🩹️patch-questions")});
const core=await Bun.file(ticket+"/core-results.md").text();
const corePaths=core.split("\n").filter(l=>l.startsWith("- "+String.fromCharCode(96))&&l.endsWith(String.fromCharCode(96))).map(l=>l.slice(3,-1)).filter(p=>p.startsWith("✏️s/")||p.startsWith("🧰️framework/"));
sections.push({title:"Core Forms and contributor subscription changes (core-results.md)",paths:corePaths});
const viewer=any+"/👁️viewer";
sections.push({title:"Viewer implementation and law ownership",paths:["🦀️.rs","🧵️retained-commands/🦀️.rs","🎮️commands/🧩️set-contributions/🦀️.rs","📦️packages/🦀️rust/🦀️.rs","📦️packages/🦀️rust/Cargo.toml","🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs","🧪️tests/🔬️integration/🦀️.rs","🧪️tests/🟦️.ts","🧪️tests/🧫️fixtures/🔣️.json"].map(p=>viewer+"/"+p)});
const refs=run(["rg","-l","--hidden","--no-ignore","--glob","*.rs","--glob","*.ts","--glob","*.tsx","splitUiTextChunks|joinUiTextChunks|setTryPartialValue|validateTryValueTarget|FORMS_PAGED_VALUE_RAW_BYTES",forms,"🧰️framework/🔨️modules/🖱️ui/🧬️contract"]);
sections.push({title:"New transport and partial/target admission references",paths:refs.trim().split("\n").filter(Boolean)});
const old=[any+"/✏️editor/❓️questions/🧩️extensions",any+"/✏️editor/🎚️config",any+"/👁️viewer/🎚️config"];
const removed=run(["git","diff","--name-status","--",...old]).trim().split("\n").filter(l=>l.startsWith("D\t")).map(l=>l.slice(2));
sections.push({title:"Removed prior ownership subtrees (read-only scoped diff)",paths:removed,removed:true});
for(const section of sections){section.paths=[...new Set(section.paths)].sort();for(const path of section.paths){const exists=await Bun.file(path).exists();if(exists===Boolean(section.removed))throw Error("Inventory existence mismatch: "+path);}}
const all=[...new Set(sections.flatMap(s=>s.paths))];
const quote=String.fromCharCode(96);
await Bun.write(ticket+"/root-files.md","# Root And Shared File Inventory — Draft\n\nThis is closure preparation, not completion. Exact authored reports, known feature roots and new API references identify these task paths. Removed paths come from a read-only diff scoped only to the explicitly relocated config/extensions and obsolete Viewer configuration. No source was copied or changed by this inventory.\n\nMerge and deduplicate this inventory with browser-files.md, integration-files.md, provider-files.md, locale-results.md and final generated descriptor/index/catalog paths. Ticket reports and retained input scripts are a separate final artifact list. Unrelated repository-wide changes are not inferred from global status.\n\n"+sections.map(s=>"## "+s.title+"\n\n"+s.paths.map(p=>"- "+quote+p+quote).join("\n")).join("\n\n")+"\n\n"+all.length+" unique source/config/test paths are currently recorded in these coordinator/shared sections. Reconcile final ownership before ticket closure and after all actual runtime checks.\n");
console.log(JSON.stringify({uniquePaths:all.length,removed:removed.length,sections:sections.map(s=>({title:s.title,count:s.paths.length}))}));
