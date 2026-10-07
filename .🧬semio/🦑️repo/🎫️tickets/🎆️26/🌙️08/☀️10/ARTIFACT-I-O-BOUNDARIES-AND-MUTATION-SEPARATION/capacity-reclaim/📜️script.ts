import {existsSync,readdirSync,lstatSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {scanCargoBuildUnits,scanCargoIncrementalUnits,deleteUnit,formatBytes} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),base=join(root,".🧬semio/🦑️repo/⚡️cache/cargo/build");
const owners=new Set(["semio-s-artifact-stdio-semio","semio-framework-os-kernel","semio-s-artifact-stdio-zip"]);
if(process.argv.includes("--reference-consumers"))for(const file of execFileSync("rg",["-l","semio-framework-artifact-reference","🧰️framework","✏️s","-g","Cargo.toml"],{encoding:"utf8"}).trim().split("\n")){const name=readFileSync(file,"utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1];if(name)owners.add(name);}
const active=()=>execFileSync("ps",["-Ao","pid,command"],{encoding:"utf8",maxBuffer:16*1024*1024}).split("\n").filter(row=>/\bcargo\s+(?:check|build|nextest|test)\b|\brustc\s|semio_s_artifact_stdio_semio-|semio_framework_os_kernel-|semio_s_artifact_stdio_zip-/.test(row)&&!row.includes("capacity-reclaim")&&(row.includes(base)||row.includes(root)&&!row.includes("/🗑️generated/")));
const controller=new AbortController();
const lease=await acquireResourceLease({directory:join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:"cache-prune",mode:"exclusive",signal:controller.signal});
try{
 if(process.argv.includes("--stale-incremental")){
  const candidates=scanCargoIncrementalUnits(base,controller.signal).staleSessions.filter(unit=>{const name=unit.path.split("/").at(-2)!;return owners.has(name.slice(0,name.lastIndexOf("-")).replaceAll("_","-"))&&unit.recencyMs<Date.now()-90*60*1000;});
  let bytes=0;const removed:typeof candidates=[];
  for(const unit of candidates){if(bytes>=10*1024**3)break;const owner=unit.path.split("/").at(-2)!.split("-")[0]!;if(active().some(row=>row.includes(join(base,unit.path))||row.includes("--crate-name "+owner+" ")))continue;deleteUnit(base,unit);bytes+=unit.bytes;removed.push(unit);}
  const report=join(ticket,"capacity-owned-compiler-output-eviction.md");writeFileSync(report,readFileSync(report,"utf8")+"\n## Stale Incremental Session Compaction "+new Date().toISOString()+"\n\nThe repository scanner identifies finalized sessions other than each incremental crate directory's newest session. Those obsolete sessions are independently safe to compact. Scope remained the task's canonical reference consumers; only sessions older than90minutes were selected, with every live compiler owner/path protected. Every newest finalized session and every working session was preserved. The cache-prune lease remained held; no source, package output unit, third-party incremental directory or process changed.\n\nReclaimed "+formatBytes(bytes)+".\n\n"+removed.map(unit=>"- `"+unit.path+"` — "+formatBytes(unit.bytes)).join("\n")+"\n");process.stdout.write(JSON.stringify({sessions:removed.length,bytes,report})+"\n");process.exitCode=0;
 }else{
 const isProtected=(unit:{path:string})=>{const owner=unit.path.split("/")[2]!,absolute=join(base,unit.path);return active().some(row=>row.includes(absolute)||row.includes("--crate-name "+owner.replaceAll("-","_")+" ")||[...row.matchAll(/--manifest-path\s+(\S+)/g)].some(match=>{const manifest=match[1]!;return existsSync(manifest)&&readFileSync(manifest,"utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1]===owner;}));};
 const units=scanCargoBuildUnits(base,controller.signal).filter(unit=>unit.path.startsWith("debug/build/")&&owners.has(unit.path.split("/")[2]!));
 const now=Date.now(),selected=units.filter(unit=>unit.recencyMs<now-90*60*1000).filter(unit=>{
  const pkg=unit.path.split("/")[2]!,recent=units.filter(x=>x.path.split("/")[2]===pkg).sort((a,b)=>b.recencyMs-a.recencyMs);
  return !recent.slice(0,3).some(x=>x.path===unit.path);
 }).sort((a,b)=>a.recencyMs-b.recencyMs);
 let bytes=0;const removed:typeof units=[];
 for(const unit of selected){if(bytes>=(process.argv.includes("--reference-consumers")?10:3)*1024**3)break;if(isProtected(unit))continue;deleteUnit(base,unit);bytes+=unit.bytes;removed.push(unit);}
 const report=join(ticket,"capacity-owned-compiler-output-eviction.md");
 const previous=existsSync(report)?readFileSync(report,"utf8"):"# Scoped Completed Compiler Output Eviction\n";
 writeFileSync(report,previous+"\n## Recovery Run "+new Date().toISOString()+"\n\nDisk exhaustion prevented native execution and source writes. This recovery used the repository cache unit scanner and deletion implementation under its exclusive cache-prune lease. Scope: "+(process.argv.includes("--reference-consumers")?"first-party package consumers whose canonical artifact-reference dependency was authored in this task":"the three main-agent verification owners Semio, OS kernel and ZIP")+". Only debug units older than 90 minutes were selected, retaining each owner's three most recently used units. Live process checks were repeated before every deletion; every unit named in a live compiler/test command and every actively compiling Cargo/Rust package was protected. Isolated outputs of unrelated ticket jobs were excluded from eviction. No source, test input, audit, third-party unit, release/wasm output or process was removed. Current reference/model source changes supersede these old outputs; Cargo can recreate them.\n\nReclaimed "+formatBytes(bytes)+".\n\n"+removed.map(unit=>"- \`"+unit.path+"\` — "+formatBytes(unit.bytes)+", last use "+new Date(unit.recencyMs).toISOString()).join("\n")+"\n");
 process.stdout.write(JSON.stringify({units:removed.length,bytes,report})+"\n");
 }
}finally{lease.release();}
