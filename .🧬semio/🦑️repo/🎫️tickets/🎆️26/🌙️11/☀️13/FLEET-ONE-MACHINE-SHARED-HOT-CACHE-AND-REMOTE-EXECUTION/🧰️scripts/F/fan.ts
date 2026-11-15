import {mkdtempSync} from "node:fs";import {tmpdir} from "node:os";import {join} from "node:path";
import {startRemoteCacheServerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
const server=await startRemoteCacheServerV1({directory:mkdtempSync(join(tmpdir(),"fan-")),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
const t=Date.now();
const procs=Array.from({length:8},(_,i)=>Bun.spawn(["/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun",import.meta.dir+"/child.ts",server.url,"m"+i],{stdout:"pipe",stderr:"pipe"}));
const rows=await Promise.all(procs.map(async p=>{const text=await new Response(p.stdout).text();await p.exited;return text.trim();}));
console.log(rows.join("\n"));console.log("wall ms",Date.now()-t);
await server.stop();
