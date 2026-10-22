const expectedNode="24.15.0";
if(process.release.name!=="node"||process.versions.node!==expectedNode){console.error(JSON.stringify({version:1,phase:"public-node-pin",expected:expectedNode,actual:process.versions.node,release:process.release.name,status:"unmet"}));process.exit(78);}
console.log(JSON.stringify({version:1,phase:"public-node-pin",actual:process.versions.node,status:"passed"}));
const [{readFile,lstat},{resolve},{pathToFileURL},{createHash},{DatabaseSync},{brotliCompressSync,brotliDecompressSync,constants},assert]=await Promise.all([import("node:fs/promises"),import("node:path"),import("node:url"),import("node:crypto"),import("node:sqlite"),import("node:zlib"),import("node:assert/strict")]);
const root=resolve(process.argv[2]),plan=JSON.parse(await readFile(new URL("./e2e-generation-pack-probe-input.json",import.meta.url),"utf8")),observations=[];
async function observe(relativepath){const path=resolve(root,relativepath),before=await lstat(path,{bigint:true}),bytes=await readFile(path),after=await lstat(path,{bigint:true}),keys=["dev","ino","mode","size","mtimeNs","ctimeNs"];assert.ok(before.isFile()&&!before.isSymbolicLink());assert.ok(keys.every(key=>before[key]===after[key]));return{relativepath,sha256:createHash("sha256").update(bytes).digest("hex"),stamp:keys.map(key=>String(after[key])).join(":")};}
for(const source of plan.sources){const row=await observe(source.path);assert.equal(row.sha256,source.sha256);observations.push(row);}
const {GENERATION_POLICY_V1,canonicalJsonV1,packGenerationDocumentV1}=await import(pathToFileURL(resolve(root,plan.module)));
const fixture=JSON.parse(await readFile(resolve(root,plan.fixture),"utf8")),schema=JSON.parse(await readFile(resolve(root,plan.schema),"utf8")),database=new DatabaseSync(":memory:"),results=[];
assert.equal(fixture.documentSizes.length,10);
try{
 for(const row of fixture.documentSizes){
  const json=JSON.stringify(row.value),oracle=database.prepare("SELECT length(CAST(json(?) AS BLOB)) AS bytes").get(json),canonical=canonicalJsonV1(row.value);
  assert.equal(oracle.bytes,row.expandedBytes);assert.equal(canonical,json);assert.equal(Buffer.byteLength(canonical),oracle.bytes);
  const packedOracle=brotliCompressSync(Buffer.from(json),{params:{[constants.BROTLI_PARAM_QUALITY]:5,[constants.BROTLI_PARAM_LGWIN]:22,[constants.BROTLI_PARAM_SIZE_HINT]:oracle.bytes}}),refusal=oracle.bytes>row.maximumBytes?"expanded":packedOracle.length>row.maximumBytes?"packed":null;
  assert.equal(refusal,row.refusal);
  if(refusal){assert.throws(()=>packGenerationDocumentV1(row.value,row.maximumBytes),/Stored document exceeds its finite limit/u);results.push({name:row.name,outcome:"expected-refusal",refusal,expanded:oracle.bytes,packedOracle:packedOracle.length,maximum:row.maximumBytes});}
  else{const actual=packGenerationDocumentV1(row.value,row.maximumBytes);assert.ok(actual instanceof Uint8Array);assert.deepEqual(Buffer.from(actual),packedOracle);assert.ok(actual.length<=row.maximumBytes);assert.equal(brotliDecompressSync(actual,{maxOutputLength:row.maximumBytes}).toString("utf8"),canonical);assert.deepEqual(JSON.parse(brotliDecompressSync(actual).toString("utf8")),row.value);results.push({name:row.name,outcome:"passed",expanded:oracle.bytes,packed:actual.length,maximum:row.maximumBytes,sha256:createHash("sha256").update(actual).digest("hex")});}
 }
 assert.equal(schema.definitions.DocumentByteLimit.maximum,GENERATION_POLICY_V1.maximumIndexBytes);
 const invalid=[0,-1,0.5,NaN,Infinity,Number.MAX_SAFE_INTEGER,GENERATION_POLICY_V1.maximumIndexBytes+1];
 for(const maximum of invalid)assert.throws(()=>packGenerationDocumentV1(null,maximum),/Finite generation document byte limit required/u);
 for(const before of observations)assert.deepEqual(await observe(before.relativepath),before);
 console.log(JSON.stringify({version:1,phase:"isolated-actual-helper",node:process.versions.node,sqlite:database.prepare("select sqlite_version() version").get().version,fixtureCases:results.length,invalidLimits:invalid.length,results,sourceObservations:observations,status:"passed",qualification:"Actual pure helper only; registered Source0 eleven Bun laws and whole remain required"}));
}finally{database.close();}
