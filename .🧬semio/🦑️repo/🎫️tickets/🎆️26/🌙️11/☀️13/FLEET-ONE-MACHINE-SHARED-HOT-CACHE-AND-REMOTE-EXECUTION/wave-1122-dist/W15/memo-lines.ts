/** 🔬️ W15 prototype: the worker-size admission memo of `memo-scale.ts` (IN) rewritten as lines (header, observation blocks in the authority row encoding, one line per owner) to OUT, then read back in a fresh process line by line from 4 MiB reads, rows shared as they arrive; prints file size and peak resident size. MODE=write|read. */
import {openSync,readSync,closeSync,writeFileSync,statSync,readFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const rt=await import(L+"👁️observation/🔢️row-table/🟨️.mjs"),obs=await import(L+"👁️observation/🟨️.mjs"),kinds=(await import(L+"🧬️schema/📥️authority.json",{with:{type:"json"}})).default.definitions.ObservationKind.enum;
const MB=1048576,peak=()=>Math.round(process.resourceUsage().maxRSS/1024),BLOCK=4096;
if(process.env.MODE==="write"){
 const memo=JSON.parse(readFileSync(process.env.IN!,"utf8"));const lines:string[]=[JSON.stringify({version:3,format:memo.format,scope:memo.scope,admission:memo.admission,observations:memo.observations.length,owners:memo.owners.length})];
 for(let at=0;at<memo.observations.length;at+=BLOCK)lines.push(rt.encodeAuthorityObservationsV1(memo.observations.slice(at,at+BLOCK),2**31));
 for(const owner of memo.owners)lines.push(JSON.stringify(owner));
 writeFileSync(process.env.OUT!,lines.join("\n")+"\n");console.log(JSON.stringify({mode:"write",fileMB:+(statSync(process.env.OUT!).size/MB).toFixed(1)}));
}else{
 const file=openSync(process.env.OUT!,"r"),chunk=Buffer.alloc(4*MB);let carry:Buffer=Buffer.alloc(0),header:any,observations:unknown[]=[],owners:{suffix:string;facts:unknown;rows:unknown[]}[]=[];
 const line=(text:string)=>{if(header===undefined){header=JSON.parse(text);return;}if(observations.length<header.observations){for(const row of obs.shareObservationRowsV1(rt.decodeAuthorityObservationsV1(JSON.parse(text),kinds)))observations.push(row);return;}const o=JSON.parse(text);owners.push({suffix:o.suffix,facts:o.facts,rows:o.rows.map((at:number)=>observations[at])});};
 for(;;){const n=readSync(file,chunk,0,chunk.length,null);if(n===0)break;let data=carry.length?Buffer.concat([carry,chunk.subarray(0,n)]):chunk.subarray(0,n),start=0;for(let nl=data.indexOf(10,start);nl>=0;nl=data.indexOf(10,start)){line(data.toString("utf8",start,nl));start=nl+1;}carry=Buffer.from(data.subarray(start));}
 closeSync(file);Bun.gc(true);
 console.log(JSON.stringify({mode:"read",observations:observations.length,owners:owners.length,peakMB:peak(),liveMB:Math.round((process.memoryUsage().heapUsed+process.memoryUsage().external)/MB)}));
}
