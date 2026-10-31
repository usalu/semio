const fs=require("node:fs/promises"),sync=require("node:fs"),path=require("node:path"),vm=require("node:vm"),assert=require("node:assert/strict"),{test}=require("node:test"),{DatabaseSync}=require("node:sqlite"),ts=require("typescript"),Ajv=require("ajv"),{readOrdinaryEvidenceV1}=require("./native-evidence.cjs");
const schema=JSON.parse(sync.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json"),"utf8")),storage=schema.$defs.StorageProtocol.const,validateShards=new Ajv({strict:true}).compile({...schema.$defs.ShardManifest,$defs:{Shard:schema.$defs.Shard}}),vectors=require("./native-index-vectors.json");
const valid=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["cases"],properties:{cases:{type:"array",minItems:9,items:{type:"object",additionalProperties:false,required:["name","count","expectedCount","change"],properties:{name:{type:"string",pattern:"^[a-z-]+$"},count:{type:"integer",minimum:1,maximum:64},expectedCount:{type:["integer","null"],minimum:1,maximum:64},change:{enum:["none","hash","bytes","duplicate","unordered","range","documents"]}}}}}});assert.equal(valid(vectors),true,JSON.stringify(valid.errors));
const text=sync.readFileSync(path.join(__dirname,"native-layout-acceptance.cjs"),"utf8"),ast=ts.createSourceFile("acceptance.cjs",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),functions=ast.statements.filter(n=>ts.isFunctionDeclaration(n)&&["evidence","observe","indexState"].includes(n.name?.text));
assert.equal(functions.length,3);
for(const row of vectors.cases)test(row.name,async()=>{
 const {sha256}=await import("@noble/hashes/sha2.js"),directory=await fs.mkdtemp(path.join(__dirname,"🗑️generated","native-index-")),index=path.join(directory,"index.search"),manifest={schema:"semio.search.shards/1",snapshot:"fixture",documents:row.count,shards:[]},db=new DatabaseSync(":memory:");
 try{
  await fs.mkdir(index);await fs.writeFile(path.join(index,"meta.json"),"{}");
  db.exec("CREATE TABLE shards(position INTEGER,name TEXT,bytes INTEGER,actual_bytes INTEGER,sha TEXT,actual_sha TEXT,documents INTEGER)");
  const add=db.prepare("INSERT INTO shards VALUES(?,?,?,?,?,?,?)");
  for(let i=0;i<row.count;i++){
   const name="s"+String(i).padStart(2,"0"),bytes=Buffer.from("event-"+i+"\n"),digest=Buffer.from(sha256(bytes)).toString("hex"),shard={name,bytes:bytes.length,sha256:digest,documents:1,events:1};
   await fs.mkdir(path.join(index,name));await fs.writeFile(path.join(index,name,"events.jsonl"),bytes);manifest.shards.push(shard);
  }
  if(row.change==="hash")manifest.shards[0].sha256="0".repeat(64);
  if(row.change==="bytes")manifest.shards[0].bytes++;
  if(row.change==="duplicate")manifest.shards[1].name=manifest.shards[0].name;
  if(row.change==="unordered")manifest.shards.reverse();
  if(row.change==="range")manifest.shards[0].name="s64";
  if(row.change==="documents")manifest.documents++;
  for(let i=0;i<manifest.shards.length;i++){
   const shard=manifest.shards[i];let bytes;try{bytes=await fs.readFile(path.join(index,shard.name,"events.jsonl"))}catch(error){if(error.code!=="ENOENT")throw error;bytes=Buffer.alloc(0)}
   add.run(i,shard.name,shard.bytes,bytes.length,shard.sha256,Buffer.from(sha256(bytes)).toString("hex"),shard.documents);
  }
  const oracle=db.prepare("SELECT count(*) AS count,count(DISTINCT name) AS names,sum(documents) AS documents,sum(CASE WHEN bytes<>actual_bytes OR sha<>actual_sha OR NOT(name GLOB 's[0-5][0-9]' OR name GLOB 's6[0-3]') THEN 1 ELSE 0 END) AS bad FROM shards").get(),order=db.prepare("SELECT count(*) AS bad FROM (SELECT name,lag(name) OVER(ORDER BY position) AS prior FROM shards) WHERE prior>=name").get(),accepted=oracle.bad===0&&order.bad===0&&oracle.names===oracle.count&&oracle.documents===manifest.documents&&(row.expectedCount===null||oracle.count===row.expectedCount);
  await fs.writeFile(path.join(index,"manifest.json"),JSON.stringify(manifest));
  const context={fs:sync,path,storage,validateShards,cache:directory,readOrdinaryEvidenceV1,AbortController,process,console,observedBytes:0,nextProgress:16777216};
  const actual=vm.runInNewContext(functions.map(n=>n.getText(ast)).join("\n")+"\nindexState",context);
  if(accepted){const result=await actual(row.expectedCount??undefined);assert.equal(result.length,row.count+2);for(const shard of manifest.shards){const found=result.find(x=>x.name===shard.name+"/events.jsonl");assert.equal(found.sha256,shard.sha256);assert.equal(found.bytes,String(shard.bytes));assert.equal(found.nlink,"1")}}
  else await assert.rejects(actual(row.expectedCount??undefined));
 }finally{db.close();await fs.rm(directory,{recursive:true,force:true})}
});
