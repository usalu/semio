const fs=require("node:fs/promises"),path=require("node:path"),assert=require("node:assert/strict"),{test}=require("node:test"),Ajv=require("ajv");
const vectors=require("./native-evidence-vectors.json"),{readOrdinaryEvidenceV1}=require("./native-evidence.cjs");
const valid=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["cases"],properties:{cases:{type:"array",minItems:8,items:{type:"object",additionalProperties:false,required:["name","kind","bytes","maxBytes"],properties:{name:{type:"string",pattern:"^[a-z-]+$"},kind:{enum:["bytes","limit","symlink","hardlink","mutation","cancel","capture"]},bytes:{type:"integer",minimum:0,maximum:600000},maxBytes:{type:"integer",minimum:1,maximum:600000}}}}}});
assert.equal(valid(vectors),true,JSON.stringify(valid.errors));
for(const row of vectors.cases)test(row.name,async()=>{
 const {sha256}=await import("@noble/hashes/sha2.js"),directory=await fs.mkdtemp(path.join(__dirname,"🗑️generated","native-evidence-")),file=path.join(directory,"value"),target=path.join(directory,"target"),bytes=Buffer.alloc(row.bytes);
 for(let i=0;i<bytes.length;i++)bytes[i]=i%251;
 await fs.writeFile(file,bytes);
 try{
  if(row.kind==="symlink"){await fs.rename(file,target);await fs.symlink("target",file)}
  if(row.kind==="hardlink")await fs.link(file,target);
  let events=0;const controller=new AbortController();
  const options={maxBytes:row.maxBytes,capture:row.kind==="capture",signal:controller.signal,onProgress:async()=>{events++;if(events===1&&row.kind==="cancel")controller.abort();if(events===1&&row.kind==="mutation"){const fd=await fs.open(file,"r+");try{await fd.write(Buffer.from([255]),0,1,0);await fd.sync()}finally{await fd.close()}}}};
  if(row.kind!=="bytes"&&row.kind!=="capture"){await assert.rejects(readOrdinaryEvidenceV1(file,"value",options));if(row.kind==="mutation"||row.kind==="cancel")assert.equal(events,1);return}
  const actual=await readOrdinaryEvidenceV1(file,"value",options),stat=await fs.lstat(file,{bigint:true});
  if(row.kind==="capture")assert.deepEqual(actual.content,bytes);assert.equal(actual.sha256,Buffer.from(sha256(bytes)).toString("hex"));assert.equal(actual.bytes,String(bytes.length));assert.equal(actual.ino,String(stat.ino));assert.equal(actual.dev,String(stat.dev));assert.equal(actual.ctimeNs,String(stat.ctimeNs));assert.equal(actual.nlink,"1");assert.ok(events>=1);
 }finally{await fs.rm(directory,{recursive:true,force:true})}
});
