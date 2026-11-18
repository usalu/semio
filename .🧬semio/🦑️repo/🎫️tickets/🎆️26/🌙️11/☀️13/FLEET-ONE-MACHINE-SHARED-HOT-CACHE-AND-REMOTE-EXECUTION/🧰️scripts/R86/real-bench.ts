/** R86 real-process bench: real closure layers of this machine's closure store (read only) as the parts of a settle; the node store and the part index live in scratch. usage: bun real-bench.ts MODULE N PER_CLOSURE   (MODULE: path of the closure-kinds module) */
import{brotliDecompressSync}from"node:zlib";
import{lstatSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
const[,,modulePath,countText,perText,skipText]=process.argv;
const R="/Users/ueli/Documents/semio",L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/";
const{CLOSURE_SETTLE_V1,createClosureSettlerV1}=await import(modulePath!);const{createNodeStoreV1}=await import(L+"💾️store/🟨️.mjs");
const ce=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce/l",count=Number(countText),per=Number(perText||3),skip=Number(skipText||0);
const files=readdirSync(ce).filter(name=>name.endsWith(".json.br")).sort().slice(skip,skip+count);
const layers:any[][]=files.map(name=>JSON.parse(brotliDecompressSync(readFileSync(join(ce,name))).toString("utf8")));
const totalRows=layers.reduce((sum,rows)=>sum+rows.length,0),distinct=new Set(layers.flatMap(rows=>rows.map((row:any)=>JSON.stringify([row.kind,row.path,row.fact])))).size;
const names=Array.from({length:count},(_a,at)=>"q"+String(at).padStart(6,"0")),partsOf=(at:number)=>Array.from({length:per},(_a,k)=>layers[(at+k*7)%count]!);
const root=realpathSync(mkdtempSync(join(tmpdir(),"r86-bench-")));
const PATHY=new Set(["lstat","exists","stat","fileExists","directoryExists"]);
const settleOnce=async(label:string)=>{
 const base=createNodeStoreV1({root}),reads:string[]=[];let rowsHeld=0;const store={...base,get:async(id:string)=>{if(id.startsWith("closure-part\0"))reads.push(id);return base.get(id);},put:base.put,flush:base.flush};
 const settler=createClosureSettlerV1({store,limits:{...CLOSURE_SETTLE_V1,deriveConcurrency:4},farm:false,partIds:{directory:root+"-ids"}});
 const lines:string[]=[];let derived=0;const began=performance.now(),cpu=process.cpuUsage();
 const settled=await settler.settle({pseudoRows:new Map(names.map(name=>[name,{kind:"derived",path:name,fact:"old"}])),derive:async(row:{path:string})=>{derived++;return{fact:"old",parts:partsOf(Number(row.path.slice(1)))};},hold:async(rows:any[])=>{for(const row of rows){rowsHeld++;if(PATHY.has(row.kind)&&typeof row.path==="string"&&row.path.startsWith("/")){try{lstatSync(row.path);}catch{}}}return true;},progress:(row:any)=>{if(row.phase.startsWith("flush"))lines.push(row.phase+" "+row.done+" of "+row.total);}});
 await settler.markIndexed({held:settled.held});await settler.release?.();
 const used=process.cpuUsage(cpu);
 console.log(JSON.stringify({label,ms:Math.round(performance.now()-began),cpuMs:Math.round((used.user+used.system)/1000),derived,failed:settled.failed,partReads:reads.length,rowsHeld,parts:settled.parts,lastProgress:lines.at(-1),progressLines:lines.length,rssMB:Math.round(process.memoryUsage().rss/1048576)}));
};
console.log(JSON.stringify({layers:layers.length,totalRows,distinct}));
try{await settleOnce("cold");await settleOnce("warm");await settleOnce("warm2");}finally{rmSync(root,{recursive:true,force:true});rmSync(root+"-ids",{recursive:true,force:true});}
process.exit(0);
