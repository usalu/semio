// [diag1-g] graph-free: infers ONE project with the real code under a byte overlay of the extent facade and a scratch staging store. run: bun --preload ../cutoff-g-tools/overlay-plugin.mjs h1.mjs
import{setOverlay}from"../cutoff-g-tools/overlay.mjs";
import{readFileSync,mkdtempSync,realpathSync,readdirSync}from"node:fs";import{tmpdir}from"node:os";import{execFileSync}from"node:child_process";import{join}from"node:path";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const E=process.env,EXT="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts";
const ROOT=E.PROJECT_ROOT??"🧰️framework/🔨️modules/⏳️async/⏰️timer-wheel/📦️packages/🦀️rust";
const oldBytes=execFileSync("git",["show",":"+EXT],{cwd:R,maxBuffer:1<<26}),newBytes=readFileSync(R+EXT);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs"),nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"),si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs"),pin=await import(L+"📥️inputs/🔏️revision/🟨️.mjs"),admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R+"📋️build-owners.json","utf8")));
const staging=process.env.STAGING??realpathSync(mkdtempSync(tmpdir()+"/diag1-"));
const files=execFileSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:R,maxBuffer:1<<29}).toString().split("\0").filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||f.endsWith(".patch")||f==="bun.lock");
console.log("staging",staging,"files",files.length,"sha old",require("crypto").createHash("sha256").update(oldBytes).digest("hex").slice(0,12),"new",require("crypto").createHash("sha256").update(newBytes).digest("hex").slice(0,12));
async function once(label,bytes){
 setOverlay(new Map(bytes?[[R+EXT,bytes]]:[]));
 si.resetClosureEntriesV1?.();obs.releaseInferenceMemoryV1?.();
 const rows={};const options={analyzeLockfile:false,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots:new Set([ROOT]),onOwnerObservations:(name,r,d)=>{rows[name]=r;}};
 const context={workspaceRoot:WS,registry,preparationControl:{advance:async()=>{}}};
 const out=await pin.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published:()=>{}},()=>obs.observeInferenceSnapshotV1(()=>inference.discoverBuildProjectsV1(files,options,context))));
 const mine=out.flatMap(([,c])=>Object.values(c.projects)).filter(p=>p.root===ROOT),un=out.flatMap(([,c])=>c.unavailable??[]);
 const names=Object.keys(rows);
 console.log(label,JSON.stringify({projects:mine.map(p=>({n:p.name,targets:Object.keys(p.targets??{}).length})),unavailable:un.filter(u=>u.root===ROOT).map(u=>({src:u.source.slice(-40),sha:u.sha256.slice(0,12)})),rowOwners:names.map(n=>[n,rows[n].length,rows[n].filter(r=>JSON.stringify(r).includes("extent")).length])}));
 if(E.DUMP)for(const r of rows[names[0]]??[])console.log("  row",r.kind,String(r.path).slice(-70),JSON.stringify(r.fact).slice(0,60));
 return rows;
}
const mode=E.MODE??"seq";
if(mode==="seq"){await once("OLD",oldBytes);await once("NEW",newBytes);await once("NEW again",newBytes);}
console.log(readdirSync(staging));
process.exit(0);
