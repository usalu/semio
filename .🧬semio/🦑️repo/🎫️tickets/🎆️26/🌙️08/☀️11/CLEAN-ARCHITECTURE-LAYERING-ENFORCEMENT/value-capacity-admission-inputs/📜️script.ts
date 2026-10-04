import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import assert from "node:assert/strict";
import Ajv from "ajv";
import Decimal from "decimal.js";
import { spawnSync } from "node:child_process";
import { parse as parseJsonc } from "jsonc-parser";
import JSON5 from "json5";

const ticket=resolve(import.meta.dir,".."),root=resolve(ticket,"../../../../../../.."),output=join(ticket,"🗑️generated","value-capacity-admission");
const owner="🧰️framework/🔨️modules/🌱️value",schemaPath=`${owner}/♻️retirement/🧬️schema/🔣️.json`,fixturePath=`${owner}/♻️retirement/🧫️fixtures/🔣️.json`,controlPath=`${owner}/🛫️encode/🦀️.rs`,nativePath=`${owner}/♻️retirement/🧪️tests/🔬️unit/🦀️.rs`;
const hash=(body:string)=>createHash("sha256").update(body).digest("hex");

/** 🧾 Retains immutable full source pairs without touching production. */
function stage(){
  const definitions=JSON.parse(readFileSync(join(import.meta.dir,"contract-definitions-authored-1.json"),"utf8")),cases=JSON.parse(readFileSync(join(import.meta.dir,"capacity-cases-authored-1.json"),"utf8"));
  const schemaBefore=readFileSync(join(root,schemaPath),"utf8"),schema=JSON.parse(schemaBefore);
  assert.equal(schema.$defs.OwnedRetirement.additionalProperties,false);
  for(const key of ["leases","capacityAdmission","continuation"])assert.equal(schema.$defs.OwnedRetirement.properties[key],undefined);
  Object.assign(schema.$defs,definitions);Object.assign(schema.$defs.OwnedRetirement.properties,{leases:{$ref:"#/$defs/SourceLeaseLaw"},capacityAdmission:{$ref:"#/$defs/CapacityAdmissionLaw"},continuation:{$ref:"#/$defs/ContinuationLaw"}});
  schema.$defs.OwnedRetirement.required.push("leases","capacityAdmission","continuation");
  const fixtureBefore=readFileSync(join(root,fixturePath),"utf8"),fixture=JSON.parse(fixtureBefore),oldCapacity=fixture.capacityAdmission;
  const literal='  "capacityAdmission": '+JSON.stringify(oldCapacity)+',';
  assert.equal(fixtureBefore.split(literal).length,2);
  const newCapacity={...oldCapacity,cases};const fixtureAfter=fixtureBefore.replace(literal,'  "capacityAdmission": '+JSON.stringify(newCapacity)+',');
  const controlBefore=readFileSync(join(root,controlPath),"utf8"),anchor="    /// 📐️ Applies a narrower physical stage ceiling without resetting ownership.";
  assert.equal(controlBefore.includes("pub fn admit_capacity("),false);assert.equal(controlBefore.split(anchor).length,2);
  const method='    /// 🪙️ Consumes the same admission owner under an exact checked source capacity.\n    pub fn admit_capacity(mut self,source_bytes:usize,multiples:usize,scaffold_bytes:usize)->Result<Self,(Self,ValueError)>{match source_bytes.checked_mul(multiples).and_then(|bytes|bytes.checked_add(scaffold_bytes)).filter(|bytes|*bytes<=isize::MAX as usize&&*bytes>=self.owned_bytes){Some(maximum)=>{self.maximum_bytes=maximum;Ok(self)},None=>Err((self,ValueError::new(ValueRefusalKind::OwnershipLimit,"native encoding capacity exceeds source policy or admitted ownership")))}}\n';
  const nativeBefore=readFileSync(join(root,nativePath),"utf8"),nativeLaw=readFileSync(join(import.meta.dir,"native-law-authored-1.rs"),"utf8");
  const portableLaw=readFileSync(join(import.meta.dir,"portable-law-authored-1.ts"),"utf8");
  const portablePath=`${owner}/🧪️tests/🧩️neutral-owner/🟦️.ts`,portableBefore=readFileSync(join(root,portablePath),"utf8");
  const packageBefore=readFileSync(join(root,"package.json"),"utf8"),library=JSON.parse(readFileSync(join(root,"node_modules/decimal.js/package.json"),"utf8"));
  const packageAnchor='    "dependency-cruiser":';assert.equal(packageBefore.split(packageAnchor).length,2);assert.equal(JSON.parse(packageBefore).devDependencies["decimal.js"],undefined);
  const packageAfter=packageBefore.replace(packageAnchor,'    "decimal.js": '+JSON.stringify(library.version)+',\n'+packageAnchor);
  const rows=[[schemaPath,schemaBefore,JSON.stringify(schema,null,2)+"\n"],[fixturePath,fixtureBefore,fixtureAfter],[controlPath,controlBefore,controlBefore.replace(anchor,method+anchor)],[nativePath,nativeBefore,nativeBefore+"\n"+nativeLaw],[portablePath,portableBefore,portableBefore+'\n'+portableLaw],["package.json",packageBefore,packageAfter]].map(([path,before,after])=>({path,before,after,inverse:before,beforeSha256:hash(before),afterSha256:hash(after)}));
  const version=process.argv[3]??"1";assert.match(version,/^[1-9][0-9]*$/);
  mkdirSync(output,{recursive:true});const path=join(output,`source-ready-${version}.json`);assert.equal(existsSync(path),false);writeFileSync(path,JSON.stringify({schemaVersion:1,sourceWrites:0,rows},null,2)+"\n");
  console.log(`[DEBUG] ${JSON.stringify({proof:path,rows:rows.length,cases:cases.length,sourceWrites:0})}`);
}

/** 🧪 Executes the exact added portable callback against original or authored full bodies. */
async function law(phase:string,version="1",receipt=version){
  assert.ok(["before","authored"].includes(phase));const proof=JSON.parse(readFileSync(join(output,`source-ready-${version}.json`),"utf8"));
  const rows=new Map<string,any>(proof.rows.map((row:any)=>[row.path,row]));
  const readFixture=(path:string,_encoding:string)=>{const local=path.slice(root.length+1),row=rows.get(local);assert.ok(row,local);return phase==="before"?row.before:row.after;};
  let assertions=0;const expect=(value:any)=>({toBe:(expected:any)=>{assertions++;assert.equal(value,expected);},toEqual:(expected:any)=>{assertions++;assert.deepEqual(value,expected);}});
  let observed:any=null;const pending:Promise<void>[]=[];const test=(name:string,callback:()=>unknown)=>{pending.push((async()=>{try{await callback();observed={name,status:"PASS"};}catch(error){observed={name,status:"FAIL",message:error instanceof Error?error.message:String(error)};}})());};
  const body=await Bun.file(join(import.meta.dir,"portable-law-authored-1.ts")).text();
  const runnable=new Function("test","expect","readFileSync","join","root","owner","Ajv","Decimal",Bun.Transpiler?new Bun.Transpiler({loader:"ts"}).transformSync(body):body);
  runnable(test,expect,readFixture,join,root,owner,Ajv,Decimal);await Promise.all(pending);assert.ok(observed);
  observed.assertions=assertions;const destination=join(output,`portable-law-proof-${receipt}-${phase}.json`);assert.equal(existsSync(destination),false);writeFileSync(destination,JSON.stringify({schemaVersion:1,phase,inputVersion:version,observed,callbackBody:body,callbackSha256:hash(body),sourceWrites:0},null,2)+"\n");console.log(`[DEBUG] ${JSON.stringify({proof:destination,...observed,sourceWrites:0})}`);if(observed.status!=="PASS")process.exitCode=1;return observed;
}

/** 📌 Publishes only immediate exact predecessors and retains every completed write. */
function publish(){
  const proof=JSON.parse(readFileSync(join(output,"source-ready-2.json"),"utf8")),before=JSON.parse(readFileSync(join(output,"portable-law-proof-2-before.json"),"utf8")),authored=JSON.parse(readFileSync(join(output,"portable-law-proof-3-authored.json"),"utf8"));
  assert.equal(before.observed.status,"FAIL");assert.equal(before.observed.assertions,1);assert.equal(authored.observed.status,"PASS");assert.equal(before.callbackSha256,authored.callbackSha256);
  for(const row of proof.rows){assert.equal(hash(row.before),row.beforeSha256);assert.equal(hash(row.after),row.afterSha256);assert.equal(readFileSync(join(root,row.path),"utf8"),row.before,row.path);}
  const path=join(output,"publication-1.json");assert.equal(existsSync(path),false);const journal:any={schemaVersion:1,rows:[],completed:false};writeFileSync(path,JSON.stringify(journal,null,2)+"\n");
  for(const row of proof.rows){const current=readFileSync(join(root,row.path),"utf8");assert.equal(current,row.before,row.path);writeFileSync(join(root,row.path),row.after);assert.equal(readFileSync(join(root,row.path),"utf8"),row.after);journal.rows.push({...row,freshBefore:current,freshBeforeSha256:hash(current)});writeFileSync(path,JSON.stringify(journal,null,2)+"\n");}
  journal.completed=true;writeFileSync(path,JSON.stringify(journal,null,2)+"\n");console.log(`[DEBUG] ${JSON.stringify({proof:path,sourceWrites:journal.rows.length,completed:true})}`);
}

/** 🔐 Captures normal lock-only and frozen replay without changing external resolutions. */
function lock(){
  const publication=JSON.parse(readFileSync(join(output,"publication-1.json"),"utf8"));assert.equal(publication.completed,true);
  const packageBody=readFileSync(join(root,"package.json"),"utf8"),packageRow=publication.rows.find((row:any)=>row.path==="package.json");assert.equal(packageBody,packageRow.after);
  const path=join(output,"lock-authority-1.json");assert.equal(existsSync(path),false);
  const parse=(body:string)=>{const errors:any[]=[];const value=parseJsonc(body,errors,{allowTrailingComma:true});assert.deepEqual(errors,[]);assert.deepEqual(value,JSON5.parse(body));return value;};
  const before=readFileSync(join(root,"bun.lock"),"utf8"),old=parse(before);const proof:any={schemaVersion:1,packageBody,packageSha256:hash(packageBody),before,inverse:before,beforeSha256:hash(before),commands:[],completed:false};writeFileSync(path,JSON.stringify(proof,null,2)+"\n");
  const invoke=(args:string[])=>{const result=spawnSync(process.execPath,args,{cwd:root,encoding:"utf8",timeout:300000});const row={args,status:result.status,signal:result.signal,stdout:result.stdout,stderr:result.stderr,error:result.error?.message??null};proof.commands.push(row);writeFileSync(path,JSON.stringify(proof,null,2)+"\n");assert.equal(result.status,0,result.stderr);assert.equal(result.error,undefined);};
  invoke(["install","--lockfile-only","--ignore-scripts"]);const current=readFileSync(join(root,"bun.lock"),"utf8"),value=parse(current);proof.current=current;proof.currentSha256=hash(current);writeFileSync(path,JSON.stringify(proof,null,2)+"\n");
  assert.deepEqual(value.packages,old.packages);const expected=structuredClone(old.workspaces);expected[""].devDependencies["decimal.js"]=JSON.parse(packageBody).devDependencies["decimal.js"];assert.deepEqual(value.workspaces,expected);
  invoke(["install","--frozen-lockfile","--lockfile-only","--ignore-scripts"]);proof.frozen=readFileSync(join(root,"bun.lock"),"utf8");proof.frozenSha256=hash(proof.frozen);assert.equal(proof.frozen,current);assert.equal(readFileSync(join(root,"package.json"),"utf8"),packageBody);
  proof.completed=true;proof.externalResolutionsConserved=true;proof.otherWorkspaceEntriesConserved=true;writeFileSync(path,JSON.stringify(proof,null,2)+"\n");console.log(`[DEBUG] ${JSON.stringify({proof:path,commands:proof.commands.map((row:any)=>({args:row.args,status:row.status})),externalResolutionsConserved:true,otherWorkspaceEntriesConserved:true,frozenExact:true})}`);
}

switch(process.argv[2]){case"stage":stage();break;case"law":await law(process.argv[3],process.argv[4],process.argv[5]);break;case"verify":process.argv[3]="2";stage();{const before=await law("before","2");assert.equal(before.status,"FAIL");assert.equal(before.assertions,1);assert.match(before.message,/false !== true/);process.exitCode=0;const authored=await law("authored","2");assert.equal(authored.status,"PASS");}break;case"publish":publish();break;case"lock":lock();break;default:throw new Error("Expected stage, law <before|authored>, verify, publish or lock");}
