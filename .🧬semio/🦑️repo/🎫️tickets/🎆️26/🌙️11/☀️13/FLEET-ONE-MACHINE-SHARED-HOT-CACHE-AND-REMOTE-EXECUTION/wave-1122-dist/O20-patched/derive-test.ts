import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {remoteExecutionHostPropertiesV1} from "../../../🟦️.ts";
import type {RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {detectWorkerRustcV1,offloadRustcUnitV1} from "../../🟦️.ts";
import {createTestCheckoutV1,testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";

const DERIVE_CRATE=join(import.meta.dir,"..","..","..","..","..","..","..","..","..","🛍️products","💻️os","🔨️modules","🗣️dsl","✨️derive");

test("the derive crate applies the derives that the planner treats as workspace-bound, and no other derive reads the workspace authority",()=>{
 const entry=readFileSync(join(DERIVE_CRATE,"📦️packages","🦀️rust","🦀️.rs"),"utf8"),derives=[...entry.matchAll(/proc_macro_derive\(\s*([A-Za-z]+)/gu)].map(match=>match[1]!);
 expect(derives.sort()).toEqual(["CompositeMutation","DslArtifact","MutationLeaf","Mutations"]);
 const reading=["🪪️mutation-leaf-attrs-and-schema-index","🔁️mutations-expansion"].map(name=>readFileSync(join(DERIVE_CRATE,name,"🦀️.rs"),"utf8"));
 expect(reading[0]).toContain("expand_mutation_leaf");expect(reading[1]).toContain("expand_derive_mutations");
 expect(reading.join("")).toContain("mutation_authority_refusal");
 const authority=readFileSync(join(DERIVE_CRATE,"🔖️mutation-source-authority","🦀️.rs"),"utf8");
 expect(authority).toContain("semio_offload_inputs_incomplete");
});

test("a unit that applies the mutation derive is declined before the store is touched and without a remote attempt; a unit whose #[path] module lies outside its manifest directory goes on to the upload",async()=>{
 const checkout=createTestCheckoutV1("derive");checkout.compile("a","pub fn a()->u32{1}\n");
 mkdirSync(join(checkout.root,"pk","x"),{recursive:true});writeFileSync(join(checkout.root,"pk","x","m.rs"),"pub fn m()->u32{3}\n");
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-derive-unit-"))),environment={...checkout.environment("c"),...testRustEnvironmentV1()};
 const touched:string[]=[],content=new Proxy({},{get:(_target,name)=>async()=>{touched.push(String(name));throw Error("content store touched");}}) as RemoteExecutionContentV1;
 const offer=detectWorkerRustcV1(checkout.root,environment,["rustc"])!,rows=[...remoteExecutionHostPropertiesV1(),...offer.properties,{name:"semio-toolchain",value:offer.toolchain.digest},{name:"semio-target",value:offer.toolchain.host}];
 const ports={content,endpoint:"grpc://127.0.0.1:1",token:"t",instance:"main",workers:{own:"me",workers:[{origin:"g1-m02",properties:rows,concurrency:4,running:0,draining:false}]}};
 const run=()=>offloadRustcUnitV1({args:checkout.arguments("c",["a"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"],ports});
 try{
  checkout.crate("c",'#[path = "../../x/m.rs"]\nmod m;\n#[derive(dsl::MutationLeaf)]\npub struct L;\npub fn c()->u32{m::m()}\n');
  const started=performance.now(),declined=await run(),took=performance.now()-started;
  expect(declined).toMatchObject({kind:"local",reason:"not-portable"});expect((declined as {detail:string}).detail).toContain("mutation derive");
  expect(touched,"no upload, no store call").toEqual([]);expect(took,"ms until the decline").toBeLessThan(2000);
  checkout.crate("c",'#[path = "../../x/m.rs"]\nmod m;\npub fn c()->u32{m::m()}\n');
  expect(await run()).toMatchObject({kind:"local",reason:"infrastructure"});expect(touched.length,"the shipped module reaches the store").toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});
