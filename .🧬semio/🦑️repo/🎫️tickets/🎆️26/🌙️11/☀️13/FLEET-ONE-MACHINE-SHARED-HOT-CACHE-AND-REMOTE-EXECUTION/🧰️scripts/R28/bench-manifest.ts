import{brotliCompressSync as slow,brotliDecompressSync,constants}from"node:zlib";
const brotliCompressSync=(bytes:Buffer)=>slow(bytes,{params:{[constants.BROTLI_PARAM_QUALITY]:4}});
const E=(process.env.R5_OVERLAY??"/Users/Shared/semio")+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🟨️.mjs";
const m=await import(E);
const LIMIT=64*1024*1024,PARTS=25095,PATHS=204124,ENTRIES=8129,PER=1678;
let seed=7;const draw=(n:number)=>{seed=(seed+0x6D2B79F5)|0;let t=Math.imul(seed^(seed>>>15),1|seed);t=(t+Math.imul(t^(t>>>7),61|t))^t;return((t^(t>>>14))>>>0)%n;};
const book=m.createManifestV1(),ids=Array.from({length:PARTS},(_,at)=>"p"+at.toString(16).padStart(19,"0"));
const pathName=(at:number)=>"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"+"x".repeat(60)+"/"+at+".ts";
for(let part=0;part<PARTS;part++)m.indexPartV1(book,ids[part],{paths:Array.from({length:Math.round(PATHS/PARTS)},(_,k)=>pathName((part*8+k)%PATHS)),dirs:[],bare:[],config:false,volatile:false,outside:[],size:1});
for(let entry=0;entry<ENTRIES;entry++){const base=draw(PARTS),list=new Set<string>();while(list.size<PER)list.add(ids[(base+draw(PER*3))%PARTS]);m.recordEntryV1(book,"@derived:"+entry.toString(16).padStart(32,"0"),{digest:"d".repeat(64),fact:JSON.stringify("f".repeat(64)),length:PER,ids:[...list]});}
const out:Record<string,unknown>={entries:book.entries.size,parts:book.partNames.length,paths:book.paths.size};
let began=performance.now();
if(typeof m.planSaveV1==="function"){
 const plan=m.planSaveV1(book),sizes=plan.writes.map(([,value]:[string,unknown])=>JSON.stringify(value).length);
 out.format="sharded";out.saveMs=Math.round(performance.now()-began);out.headBytes=JSON.stringify(plan.head).length;out.records=plan.writes.length+1;out.largestRecordBytes=Math.max(...sizes);out.totalJsonBytes=sizes.reduce((a:number,b:number)=>a+b,plan.headBytes??JSON.stringify(plan.head).length);
 out.fitsLimit=out.largestRecordBytes<LIMIT;plan.commit();
 const disk=new Map<string,Buffer>();for(const[subject,value]of plan.writes)disk.set(subject,brotliCompressSync(Buffer.from(JSON.stringify(value))));disk.set("all",brotliCompressSync(Buffer.from(JSON.stringify(plan.head))));
 out.compressedBytes=[...disk.values()].reduce((a,b)=>a+b.length,0);
 began=performance.now();const back=await m.loadManifestV1(async(subject:string)=>JSON.parse(brotliDecompressSync(disk.get(subject)!).toString()));out.loadMs=Math.round(performance.now()-began);out.loadedEntries=back.entries.size;
 began=performance.now();const again=m.planSaveV1(back);out.idleReplanMs=Math.round(performance.now()-began);out.idleWrites=again.writes.length;
 m.recordEntryV1(back,"@derived:"+"0".repeat(32),{digest:"x",fact:"y",length:1,ids:[ids[3]]});began=performance.now();const one=m.planSaveV1(back);out.oneChangeReplanMs=Math.round(performance.now()-began);out.oneChangeWrites=one.writes.length;
}else{
 const value=m.writeManifestV1(book),text=JSON.stringify(value);out.format="single";out.saveMs=Math.round(performance.now()-began);out.records=1;out.largestRecordBytes=text.length;out.totalJsonBytes=text.length;out.fitsLimit=text.length<LIMIT;out.compressedBytes=brotliCompressSync(Buffer.from(text)).length;
 began=performance.now();try{brotliDecompressSync(brotliCompressSync(Buffer.from(text)),{maxOutputLength:LIMIT});out.loadMs=Math.round(performance.now()-began);}catch(error){out.loadFails=String((error as Error).message).slice(0,60);}
}
console.log(JSON.stringify(out));
