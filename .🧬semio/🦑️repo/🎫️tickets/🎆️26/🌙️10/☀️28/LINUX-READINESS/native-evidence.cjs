const fs=require("node:fs"),crypto=require("node:crypto");
const fields=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"];
const same=(a,b)=>fields.every(name=>a[name]===b[name]);
const ordinary=stat=>stat.isFile()&&!stat.isSymbolicLink()&&stat.nlink===1n;
/** 🔬 Streams bounded ordinary-file evidence while retaining descriptor and leaf identity. */
async function readOrdinaryEvidenceV1(file,name,{maxBytes,signal,onProgress,capture=false}={}){
 if(!Number.isSafeInteger(maxBytes)||maxBytes<1)throw Error("Finite evidence byte bound required");
 if(typeof capture!=="boolean"||(capture&&maxBytes>65536))throw Error("Metadata capture bound exceeded");
 signal?.throwIfAborted();
 const before=await fs.promises.lstat(file,{bigint:true});
 if(!ordinary(before)||before.size>BigInt(maxBytes))throw Error("Bounded ordinary single-link evidence required");
 const fd=await fs.promises.open(file,fs.constants.O_RDONLY|(fs.constants.O_NOFOLLOW||0));
 try{
  if(!same(before,await fd.stat({bigint:true})))throw Error("Evidence identity changed before open");
  const hash=crypto.createHash("sha256"),block=Buffer.alloc(262144),chunks=capture?[]:undefined;let bytes=0;
  for(;;){
   signal?.throwIfAborted();
   const read=await fd.read(block,0,Math.min(block.length,maxBytes-bytes+1),null);
   if(!read.bytesRead)break;
   bytes+=read.bytesRead;if(bytes>maxBytes)throw Error("Evidence byte bound exceeded");
   hash.update(block.subarray(0,read.bytesRead));chunks?.push(Buffer.from(block.subarray(0,read.bytesRead)));
   await onProgress?.({name,bytes,total:Number(before.size)});
   signal?.throwIfAborted();
   if(!same(before,await fd.stat({bigint:true})))throw Error("Evidence changed during read");
  }
  if(bytes===0){await onProgress?.({name,bytes:0,total:0});signal?.throwIfAborted()}
  if(BigInt(bytes)!==before.size||!same(before,await fd.stat({bigint:true}))||!same(before,await fs.promises.lstat(file,{bigint:true})))throw Error("Evidence changed while observed");
  return {name,dev:String(before.dev),ino:String(before.ino),mode:String(before.mode),nlink:String(before.nlink),bytes:String(before.size),mtimeNs:String(before.mtimeNs),ctimeNs:String(before.ctimeNs),sha256:hash.digest("hex"),...(chunks?{content:Buffer.concat(chunks)}:{})};
 }finally{await fd.close()}
}
module.exports={readOrdinaryEvidenceV1};
