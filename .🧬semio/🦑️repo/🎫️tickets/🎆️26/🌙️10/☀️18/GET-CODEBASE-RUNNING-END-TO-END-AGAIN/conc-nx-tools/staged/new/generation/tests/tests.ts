/** 📌️ Pinned generation laws over `./🧫️fixtures/🔣️.json`: a receipt holds exactly while the three graph files and every identity file are unchanged and the claim is the same, every receipt satisfies `../🧬️schema/🔣️.json` (Ajv, third party) and the identity digest equals an independent Python hashlib model (`./🐍️oracle.py`). */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {appendFileSync,linkSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import {NX_GENERATION_FILE,nxGenerationClaimV1,nxGenerationHoldsV1,nxGenerationIdentityV1,writeNxGenerationReceiptV1} from "../🟦️.ts";

const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),"utf8")) as any;
const validate=new Ajv({strict:false,allErrors:true}).compile(JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")));
const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());
mkdirSync(artifacts,{recursive:true});

function build():{root:string;data:string;environment:Record<string,string>}{
 const root=mkdtempSync(join(artifacts,"nx-generation-")),data=join(root,"data");
 for(const [path,content] of Object.entries<string>(fixture.workspace)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
 mkdirSync(data,{recursive:true});
 for(const [name,content] of Object.entries<string>(fixture.graph))writeFileSync(join(data,name),content);
 return {root,data,environment:{SEMIO_BUILD_GRAPH_DIGEST:fixture.claim.graphDigest,SEMIO_NX_IMPLEMENTATION_IDENTITY:fixture.claim.implementation,SEMIO_NX_TOOLING_IDENTITY:fixture.claim.tooling}};
}

test("identity digest equals the independent model",async()=>{
 const {root}=build();
 try{
  const model=spawnSync("python3",["-I",resolve(import.meta.dir,"./🐍️oracle.py"),root,resolve(import.meta.dir,"./🧫️fixtures/🔣️.json")],{encoding:"utf8"});
  expect(model.error).toBeUndefined();expect(model.status).toBe(0);
  expect(await nxGenerationIdentityV1(root)).toBe(model.stdout.trim());
 }finally{rmSync(root,{recursive:true,force:true});}
});

for(const scenario of fixture.scenarios){
 test("generation: "+scenario.name,async()=>{
  const {root,data,environment}=build();
  try{
   const claim=(await nxGenerationClaimV1(root,environment))!;
   expect(await writeNxGenerationReceiptV1(root,data,claim)).toBe(true);
   expect(validate(JSON.parse(readFileSync(join(data,NX_GENERATION_FILE),"utf8")))).toBe(true);
   expect(await nxGenerationHoldsV1(root,data,claim)).toBe(true);
   const mutation=scenario.mutation,base=mutation.inData||fixture.graphFiles.includes(mutation.path)?data:root,path=mutation.path===undefined?"":join(base,mutation.path);
   if(mutation.kind==="none"){}
   else if(mutation.kind==="replace"){const temporary=path+".next";writeFileSync(temporary,mutation.content);renameSync(temporary,path);}
   else if(mutation.kind==="append")appendFileSync(path,mutation.content);
   else if(mutation.kind==="delete")rmSync(path);
   expect(await nxGenerationHoldsV1(root,data,claim)).toBe(scenario.holds);
   if(fixture.identityFiles.includes(mutation.path))expect(await nxGenerationHoldsV1(root,data,(await nxGenerationClaimV1(root,environment))!)).toBe(false);
  }finally{rmSync(root,{recursive:true,force:true});}
 });
}

test("generation: a different pin, implementation or tooling never holds",async()=>{
 const {root,data,environment}=build();
 try{
  const claim=(await nxGenerationClaimV1(root,environment))!;
  expect(await writeNxGenerationReceiptV1(root,data,claim)).toBe(true);
  for(const key of fixture.claimMismatch)expect(await nxGenerationHoldsV1(root,data,{...claim,[key]:"f".repeat(64)})).toBe(false);
  expect(await nxGenerationClaimV1(root,{...environment,SEMIO_BUILD_GRAPH_DIGEST:"nothex"})).toBeUndefined();
  expect(await nxGenerationClaimV1(root,{})).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

for(const refusal of fixture.refusals){
 test("generation refuses to certify: "+refusal.name,async()=>{
  const {root,data,environment}=build();
  try{
   const claim=(await nxGenerationClaimV1(root,environment))!;
   if(refusal.remove)rmSync(join(data,refusal.remove));
   if(refusal.link)linkSync(join(data,refusal.link),join(root,"alias"));
   expect(await writeNxGenerationReceiptV1(root,data,claim)).toBe(false);
   expect(await nxGenerationHoldsV1(root,data,claim)).toBe(false);
  }finally{rmSync(root,{recursive:true,force:true});}
 });
}

test("generation refuses when the identity moved between claim and write",async()=>{
 const {root,data,environment}=build();
 try{
  const claim=(await nxGenerationClaimV1(root,environment))!;
  appendFileSync(join(root,"nx.json")," ");
  expect(await writeNxGenerationReceiptV1(root,data,claim)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("concurrent writers leave one valid receipt",async()=>{
 const {root,data,environment}=build();
 try{
  const claim=(await nxGenerationClaimV1(root,environment))!;
  await Promise.all(Array.from({length:8},()=>writeNxGenerationReceiptV1(root,data,claim)));
  expect(validate(JSON.parse(readFileSync(join(data,NX_GENERATION_FILE),"utf8")))).toBe(true);
  expect(await nxGenerationHoldsV1(root,data,claim)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

for(const race of [{name:"a graph file is replaced while the receipt is sealed",apply:(root:string,data:string)=>{const path=join(data,"project-graph.json");writeFileSync(path+".next","{\"nodes\":{}}");renameSync(path+".next",path);}},{name:"nx.json moves while the receipt is sealed",apply:(root:string)=>appendFileSync(join(root,"nx.json")," ")}]){
 test("generation refuses when "+race.name,async()=>{
  const {root,data,environment}=build();
  try{
   const claim=(await nxGenerationClaimV1(root,environment))!;
   expect(await writeNxGenerationReceiptV1(root,data,claim,undefined,async()=>race.apply(root,data))).toBe(false);
   expect(await nxGenerationHoldsV1(root,data,claim)).toBe(false);
  }finally{rmSync(root,{recursive:true,force:true});}
 });
}
