import{mkdirSync,mkdtempSync,writeFileSync,appendFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";

/** 🧪️ Proof N8 on one machine: a private cache server on a free port with test tokens (never the production cache), a reader that reads the project graph constrained to the shape `SHAPE_MB` (default 16384) with the fleet-first step, and, after `SERVER_DELAY_S`, the serving side at full capacity (the graph sidecar role). Usage: bun proof.ts <output directory> [command words of the reader, default: show projects]. Everything it starts ends with it. */
const root=resolve(import.meta.dir,"../../../../../../../../.."),out=resolve(process.argv[2]??"."),reader=process.argv.slice(3).length?process.argv.slice(3):["show","projects"];
mkdirSync(out,{recursive:true});
const log=(line:string)=>appendFileSync(join(out,"proof.log"),`${new Date().toISOString()} ${line}\n`);
const scratch=mkdtempSync(join(tmpdir(),"n8-proof-")),store=join(scratch,"store");mkdirSync(join(store,"v1","origins","g1-n8-reader"),{recursive:true});
const cache=await startRemoteCacheServerV1({directory:join(scratch,"cache"),host:"127.0.0.1",port:0,maximumBytes:2**33,maximumEntryBytes:2**30,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"n8-read",write:"n8-write"},validation:"off"});
log(`cache server ${cache.url} (private, test tokens)`);
const base={...process.env,SEMIO_REMOTE_CACHE_ENDPOINT:cache.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"n8-write",SEMIO_REMOTE_CACHE_INSTANCE:"main"} as Record<string,string>;
for(const name of Object.keys(base))if(/^SEMIO_FLEET_(GROUP|NFS_HOST|NETWORK)$/u.test(name))delete base[name];
const repo=process.env.SEMIO_REPO_BIN??join(process.env.HOME!,".local","bin","repo");
const readerEnv={...base,SEMIO_FLEET_ORIGIN:"g1-n8-reader",SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:process.env.SHAPE_MB??"16384",SEMIO_GRAPH_FLEET_WAIT_MILLISECONDS:process.env.WAIT_MS??"3600000"};
const started=Date.now(),readerChild=Bun.spawn([repo,...reader],{cwd:root,env:readerEnv,stdout:Bun.file(join(out,"reader.out")),stderr:Bun.file(join(out,"reader.err"))});
log(`reader pid ${readerChild.pid}: ${reader.join(" ")} (shape ${readerEnv.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES} MB)`);
writeFileSync(join(out,"reader.pid"),String(readerChild.pid));
let serverChild:ReturnType<typeof Bun.spawn>|undefined;
const delay=Number(process.env.SERVER_DELAY_S??"45")*1000;
setTimeout(()=>{
 const entry=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts");
 serverChild=Bun.spawn([process.execPath,entry,"graph"],{cwd:root,env:{...base,SEMIO_SIDECAR_CHILD:"1",SEMIO_FLEET_ROOT:root,SEMIO_FLEET_ORIGIN:"g1-n8-server",SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_PARENT_PID:String(process.pid)},stdout:Bun.file(join(out,"server.out")),stderr:Bun.file(join(out,"server.err"))});
 log(`server pid ${serverChild.pid} (graph role, full capacity)`);writeFileSync(join(out,"server.pid"),String(serverChild.pid));
},delay);
const code=await readerChild.exited;
log(`reader ended with status ${code} after ${Math.round((Date.now()-started)/1000)} s`);
try{serverChild?.kill("SIGTERM");}catch{}
await Bun.sleep(3000);try{serverChild?.kill("SIGKILL");}catch{}
await cache.stop(true);
log("cache server stopped");
process.exit(0);
