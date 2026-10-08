import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,readdirSync,mkdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createRequire} from "node:module";
import Ajv from "ajv";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2),plugin="✏️s/🔌️plugins/🪐️space",output=join(ticket,"🗑️generated/space-child-inventory",command+"-"+epoch),original=join(import.meta.dir,"original");
assert.ok(["capture","admit"].includes(command)&&/^\d+$/u.test(epoch??""));assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const controller=new AbortController(),stop=()=>controller.abort(),started=Date.now(),paths:string[]=[],records:any[]=[],results:any[]=[];
process.once("SIGINT",stop);process.once("SIGTERM",stop);
const checkpoint=()=>{controller.signal.throwIfAborted();assert.ok(Date.now()-started<60000);};
const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){checkpoint();if(["node_modules","target","dist",".venv","__pycache__"].includes(entry.name))continue;const path=join(directory,entry.name);assert.ok(!entry.isSymbolicLink(),"Source refuses symlink: "+path);if(entry.isDirectory())walk(path);else if(/\.(?:tsx?|rs|toml|json|lock|semio)$/u.test(path))paths.push(path);}};
walk(join(root,plugin));paths.push(import.meta.path);paths.sort();
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return{path,source,sha256:createHash("sha256").update(source).digest("hex")};};
const sources=paths.map(capture);
let code=1,error:string|undefined;
try{
 if(command==="capture"){
  assert.ok(!existsSync(original));
  for(const source of sources.filter(row=>row.path!==import.meta.path)){checkpoint();const destination=join(original,source.path.slice(root.length+1));mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,source.source,{flag:"wx"});records.push({path:source.path,sha256:source.sha256});}
  console.log("[DEBUG] Space original custody files="+records.length);
 }else{
  const oracle=createRequire(join(root,"package.json"))("@iarna/toml"),ajv=new Ajv({strict:false}),retained=readFileSync(join(original,plugin,"🏭️bridge/🦀️.rs"),"utf8"),coordinates:any[]=[],pairs:string[]=[];
  const core=join(root,plugin,"🫀️core"),authority=JSON.parse(readFileSync(join(core,"🧫️fixtures/🔣️.json"),"utf8"));assert.ok(ajv.compile(JSON.parse(readFileSync(join(core,"🧬️schema/🔣️.json"),"utf8")))(authority));
  for(const path of [join(core,"Cargo.toml"),join(core,"📦️packages/🦀️rust/Cargo.toml")]){const text=readFileSync(path,"utf8");assert.deepEqual(Bun.TOML.parse(text),oracle.parse(text));const parsed:any=Bun.TOML.parse(text);assert.ok(!Object.keys(parsed.dependencies??parsed.workspace.dependencies).some(name=>name.startsWith("semio-s-artifact-space-")));}
  const {testUtcMinuteFixtureAgainstIntl}=await import(join(core,"🕰️time/🧪️tests/🔬️unit/🟦️.ts"));testUtcMinuteFixtureAgainstIntl(root);
  const originalTime=join(original,plugin,"🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🕰️utc-minute/🔣️.json");assert.equal(readFileSync(join(core,"🕰️time/🧫️fixtures/🕰️utc-minute/🔣️.json"),"utf8"),readFileSync(originalTime,"utf8"));
  const {prepareSpaceDocumentSources}=await import(join(core,"📄️documents/🟦️.ts")),documents=JSON.parse(readFileSync(join(core,"📄️documents/🧫️fixtures/🔣️.json"),"utf8")),validateSources=ajv.compile(JSON.parse(readFileSync(join(core,"📄️documents/🧬️schema/🔣️.json"),"utf8"))),codecs=[{id:"shape-v1",decode:(format:string,text:string)=>{if(format==="json")return text;if(format==="dsl"&&text==="shape id=owned")return '{"schema":"shape.v1","id":"owned"}';throw Error("fixture codec refuses source");}}];
  for(const vector of documents.vectors){assert.ok(validateSources([vector.source]));assert.deepEqual(JSON.parse(prepareSpaceDocumentSources([vector.source],codecs)[0].document),vector.expected);}
  for(const vector of documents.invalid)assert.throws(()=>prepareSpaceDocumentSources(vector.sources,codecs),vector.id);
  assert.equal(readFileSync(join(core,"📄️documents/🧫️fixtures/🔣️.json"),"utf8"),readFileSync(join(original,plugin,"🫀️core/📄️documents/🧫️fixtures/🔣️.json"),"utf8"));
  assert.ok(!/SSpaceSnapshot|SSpaceMutation|S_SPACE_INDEX_DOCUMENT_SCHEMA/u.test(readFileSync(join(core,"🦀️.rs"),"utf8")));results.push({sharedAuthorityAjvValidated:true,independentIntlTimeOracle:true,originalTimeVectorsRetained:true,documentVectors:documents.vectors.length,documentRefusals:documents.invalid.length,noConcreteChildDependency:true});
  for(const [folder,label,other]of [["🏠️home","home","space"],["🪐️space","space","home"]]){
   const owner=join(root,plugin,"🗿️artifacts",folder),fixture=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧬️schema/🔣️.json"),"utf8"));assert.ok(ajv.compile(schema)(fixture));assert.equal(fixture.artifact,"s.space."+label);assert.deepEqual(fixture.refusedArtifacts,["s.space."+other]);coordinates.push(...fixture.coordinates);
   const texts=[join(owner,"Cargo.toml"),join(owner,"📦️packages/🦀️rust/Cargo.toml")].map(path=>readFileSync(path,"utf8"));for(const text of texts)assert.deepEqual(Bun.TOML.parse(text),oracle.parse(text));
   const workspace:any=Bun.TOML.parse(texts[0]),pkg:any=Bun.TOML.parse(texts[1]);assert.deepEqual(workspace.workspace.metadata.semio["mutation-inventory"],{script:"🏭️bridge/📜️script.ts",roots:["."]});assert.ok(pkg.bin.some((bin:any)=>bin.name==="semio-space-"+label+"-mutation-bridge"&&JSON.stringify(bin["required-features"])==='["mutation-inventory"]'));assert.deepEqual(pkg.features["mutation-inventory"],["dep:semio-framework-test-mutation-inventory"]);assert.equal(pkg.dependencies["semio-s-space-core"].path,"../../../../🫀️core/📦️packages/🦀️rust");assert.ok(pkg.features["component-app-assembly"].includes("semio-s-space-core/component-app-assembly"));assert.ok(!Object.keys(pkg.dependencies).some(name=>name.startsWith("semio-s-artifact-space-")));assert.ok(!Object.keys(workspace.workspace.dependencies).some(name=>name.startsWith("semio-s-artifact-space-")));
   const producer=readFileSync(join(owner,"🏭️bridge/🦀️.rs"),"utf8");for(const match of producer.matchAll(/<([^<>]+) as Mutation<([^<>]+)>>::DESCRIPTORS/gu))pairs.push(match[2]+","+match[1]);
   const lockPath=join(plugin,"🗿️artifacts",folder,"Cargo.lock"),before=readFileSync(join(original,lockPath),"utf8"),after=readFileSync(join(root,lockPath),"utf8"),foreign=(text:string)=>text.split(/(?=^\[\[package\]\])/mu).filter(block=>/^source\s*=/mu.test(block));const old=foreign(before);assert.ok(old.length>0);assert.deepEqual(foreign(after),old);assert.deepEqual((Bun.TOML.parse(before) as any).package.filter((pkg:any)=>pkg.source),(Bun.TOML.parse(after) as any).package.filter((pkg:any)=>pkg.source));results.push({artifact:fixture.artifact,coordinates:fixture.coordinates.length,fixtureAjvValidated:true,tomlOracleExact:true,foreignCount:old.length,foreignEntriesByteIdentical:true});
  }
  const expectedCoordinates=[...retained.matchAll(/\("(s\.space\.[^"]+)", "([^"]+)", "([^"]+)", "([^"]*)", "([^"]+)", "[^"]*"\)/gu)].map(match=>[match[1],match[2],match[3],match[4],match[5]]),actualCoordinates=coordinates.map(row=>[row.artifact,row.standard,row.subset,row.surface,row.owner]);assert.equal(expectedCoordinates.length,2);assert.deepEqual(actualCoordinates.map(row=>JSON.stringify(row)).sort(),expectedCoordinates.map(row=>JSON.stringify(row)).sort());
  const expectedPairs=[...retained.matchAll(/descriptors::<([^<>]+)>/gu)].map(match=>match[1].split(",").map(value=>value.trim()).join(",")).sort();assert.equal(expectedPairs.length,2);assert.deepEqual([...new Set(pairs)].sort(),expectedPairs);console.log("[DEBUG] Space portable admission children="+(results.length-1)+" coordinates="+coordinates.length+" descriptorPairs="+expectedPairs.length);
 }
 code=0;
}catch(failure){error=String(failure);console.error("[DEBUG] Space source refusal "+error);}finally{
 const post=paths.map(capture),changed=sources.filter((source,index)=>source.sha256!==post[index].sha256).map(source=>source.path),exact=changed.length===0;
 writeFileSync(join(output,"terminal.json"),JSON.stringify({code,error,command,epoch,records,sources:sources.map(({path,sha256})=>({path,sha256})),post:post.map(({path,sha256})=>({path,sha256})),changed,exact,results,nativeJobs:0,wholeRootAccepted:false}));console.log("[DEBUG] Space source terminal code="+code+" exact="+exact);process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=code;
}
