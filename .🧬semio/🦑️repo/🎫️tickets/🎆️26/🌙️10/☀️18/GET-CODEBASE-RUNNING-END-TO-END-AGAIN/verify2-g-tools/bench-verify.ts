/** Bench of the dependency receipt verification on the LIVE workspace (read-only apart from the journal's fence files): complete walk vs vouched walk with a private journal service. Usage: bun bench-verify.ts <mirror root> <live root> */
import {lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,realpathSync,mkdtempSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
const [mirror,live]=process.argv.slice(2) as [string,string];
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",N="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes";
const gen=await import(join(mirror,L,"⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts"));
const glue=await import(join(mirror,L,"⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts"));
const bun=await import(join(mirror,L,"🗂️workspaces/🟦️bun/🟦️.ts"));
const journal=await import(join(mirror,L,"🏗️build/🏡️resident/📜️journal/🟦️.ts"));
const client=await import(join(mirror,N,"🧭️client/🟦️.ts"));
const vouchModule=await import(join(mirror,N,"🪪️vouch/🟦️.ts"));
const storeRoot=join(live,".cache/b/d/m"),dir=join(storeRoot,"generation");
const name=readdirSync(dir).filter(n=>n.endsWith(".receipt.json")).find(n=>{try{return JSON.parse(readFileSync(join(dir,n),"utf8")).scope===".";}catch{return false;}})!;
const receipt=gen.parseGenerationReceiptV1(readFileSync(join(dir,name)));
const stat=(path:string)=>lstatSync(path,{bigint:true});
const observeBindings=(_s:string,pairs:{sourceDirectory:string;targetDirectory:string}[])=>{const rows:string[]=[];for(const b of pairs){const rel=relative(storeRoot,b.targetDirectory);let d=storeRoot;for(const part of rel.split("/")){d=join(d,part);const f=lstatSync(d);rows.push(JSON.stringify([d,String(f.dev),String(f.ino),f.mode]));}const link=lstatSync(b.sourceDirectory);rows.push(JSON.stringify([b.sourceDirectory,String(link.dev),String(link.ino),link.mode,b.targetDirectory]));}return JSON.stringify(rows);};
const readReady=(path:string)=>{const before=stat(path),bytes=readFileSync(path);return["dev","ino","size","mode","mtimeNs","ctimeNs"].map(k=>String((before as never)[k])).join(":")+":"+createHash("sha256").update(bytes).digest("hex");};
const subject=glue.javascriptDependencyGenerationSubjectV1();
const priv=mkdtempSync(join(tmpdir(),"v2g-bench-")),jdir=join(priv,"journal");mkdirSync(jdir,{recursive:true});
const script=join(mirror,L,"🏗️build/🏡️resident/📜️journal/⌨️main/🟦️.ts");
const jc=client.createChangeJournalClientV1({directory:jdir,root:live,start:()=>journal.startChangeJournalServiceV1({directory:jdir,root:live,script})});
function input(recheck:boolean,vouched?:(paths:readonly string[])=>Promise<ReadonlySet<string>>|ReadonlySet<string>){
 return{workspace:live,storeRoot,receipt,recheck,readEvidence:(n:string)=>readFileSync(join(dir,n)),runtime:glue.javascriptDependencyGenerationRuntimeV1,producer:()=>receipt.producer,conditionsClosed:()=>true,
  project:(p:string)=>bun.readBunWorkspaceListingProjectionV1(live,p),observeBindings,readReady,control:{signal:new AbortController().signal,check(){},async tick(){}},...(vouched?{vouched}:{})};
}
const time=async<T>(label:string,body:()=>Promise<T>)=>{const t=performance.now(),cpu=process.cpuUsage();const value=await body();const c=process.cpuUsage(cpu);console.log(label.padEnd(34),Math.round(performance.now()-t)+" ms","cpu",Math.round((c.user+c.system)/1000)+" ms");return value;};
console.log("receipt",name.slice(0,12),"frontier rows",receipt.membership.frontier.rows,"texts",receipt.membership.texts.length,"inputs",receipt.inputs.reduce((s:number,g:any)=>s+g.sources.length,0),"provider rows",receipt.execution.evidence.rows);
const v=(r:any)=>r.status==="match"?"match":"mismatch:"+r.class+" "+(r.path??"");
console.log("A complete walk x2 (today)");
console.log(" ",v(await time(" pass 1 fast",()=>gen.verifyGenerationReceiptV1(input(false)))));
console.log(" ",v(await time(" pass 2 recheck",()=>gen.verifyGenerationReceiptV1(input(true)))));
const open=()=>vouchModule.openVouchV1({client:jc,directory:join(jdir,"vouch"),consumer:"bench",key:receipt.sha256,insensitive:true});
for(const round of[1,2,3]){
 console.log("B vouched round",round,round===1?"(no sealed cursor: complete walk + journal recheck)":"(sealed cursor)");
 const vouch=await time(" open vouch",open);console.log("  mode",vouch.mode,vouch.refusal??"");
 let skipped=0;
 console.log(" ",v(await time(" pass 1 fast",()=>gen.verifyGenerationReceiptV1(input(false,paths=>{const s=vouch.skippable(paths);skipped+=s.size;return s;})))),"skipped",skipped);
 let settled=0;
 console.log(" ",v(await time(" pass 2 recheck",()=>gen.verifyGenerationReceiptV1(input(true,async paths=>{const s=await vouch.settled(paths);settled+=s.size;return s;})))),"settled",settled);
 await vouch.seal();
}
await new Promise(r=>setTimeout(r,200));
rmSync(priv,{recursive:true,force:true});
process.exit(0);
