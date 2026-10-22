/** 🩹️ Contract: the tool patch admits, applies and receipts exactly what the language-agnostic corpora in `./🔣️.json` (the unit's own apply cases) and `../🔣️.json` say, agreeing with the third-party `diff` library on application and with an independent Python reading of the unified-diff rules. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {applyPatch} from "diff";
import Ajv from "ajv";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {applyToolingPatchImagesV1,assertToolingPatchV1,toolPatchAdditions,toolPatchReceipt} from "@UNIT/🟦️.ts";
import policySchema from "@UNIT/🧬️schema/🔣️.json";
import existing from "@UNIT/🧪️tests/🔣️.json";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE"),policy=policySchema.definitions.Protocol.const,encoder=new TextEncoder(),decoder=new TextDecoder();
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const post=(before:string,patch:string):string|null=>{const out=refusal(()=>applyToolingPatchImagesV1(new Map([["source.js",encoder.encode(before)]]),patch));return out===""?decoder.decode(applyToolingPatchImagesV1(new Map([["source.js",encoder.encode(before)]]),patch).get("source.js")):null;};

test("both corpora are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate({definitions:policySchema.definitions,type:"array",items:{$ref:"#/definitions/Case"}},existing),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({definitions:policySchema.definitions,type:"array",items:{$ref:"#/definitions/Case"}},[{...existing[0],accept:"yes"}])).toBe(false);
});

test("the admission of a patch matches the corpus and the Python reading with the same refusal",()=>{
 const answers=ask({op:"validate",patches:corpus.validate.map(row=>row.patch)});
 for(const[index,row]of corpus.validate.entries()){
  const message=refusal(()=>assertToolingPatchV1(row.patch));
  expect(message===""?"":message,row.name).toBe(row.error);expect(answers[index],row.name).toBe(row.error);expect(message==="",row.name).toBe(row.accept);
 }
});

test("owned additions match the corpus and the Python reading",()=>{
 const answers=ask({op:"additions",patches:corpus.additions.map(row=>row.patch)});
 for(const[index,row]of corpus.additions.entries()){
  let seen:Record<string,string[]>|null=null;const message=refusal(()=>{seen=Object.fromEntries([...toolPatchAdditions(row.patch)].map(([path,lines])=>[path,[...lines].sort()]));});
  expect(message,row.name).toBe(row.error);expect(seen,row.name).toEqual(row.expected);
  expect(answers[index].error??"",row.name).toBe(row.error);if(row.expected!==null)expect(answers[index].files,row.name).toEqual(row.expected);
 }
});

test("application gives the exact post-image: the unit's own corpus, the extra cases, the diff library and the Python applier agree",()=>{
 const rows=[...existing.map(row=>({...row,library:true})),...corpus.apply],answers=ask({op:"apply",rows});
 for(const[index,row]of rows.entries()){
  const image=post(row.before,row.patch);
  expect(image!==null,row.name).toBe(row.accept);if(row.accept)expect(image,row.name).toBe(row.after);
  expect(answers[index],row.name).toBe(row.accept?row.after:null);
  if(row.library){const library=applyPatch(row.before,row.patch);expect(library===false?null:library,row.name).toBe(row.accept?row.after:null);}
 }
});

test("a multi-file patch yields independent images and ordered hunks keep the file otherwise as it was",()=>{
 const patch="diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -2,1 +2,1 @@\n-b\n+B\ndiff --git a/dir/b.js b/dir/b.js\n--- a/dir/b.js\n+++ b/dir/b.js\n@@ -1,1 +1,2 @@\n x\n+y\n";
 const source=new Map([["a.js",encoder.encode("a\nb\nc\n")],["dir/b.js",encoder.encode("x\n")]]),images=applyToolingPatchImagesV1(source,patch);
 expect([...images.keys()]).toEqual(["a.js","dir/b.js"]);expect(decoder.decode(images.get("a.js"))).toBe("a\nB\nc\n");expect(decoder.decode(images.get("dir/b.js"))).toBe("x\ny\n");
 expect(decoder.decode(source.get("a.js"))).toBe("a\nb\nc\n");
 images.get("a.js")![0]=0;expect(decoder.decode(source.get("a.js"))).toBe("a\nb\nc\n");
});

test("the byte, file, path, image and encoding ceilings refuse before any output is produced",()=>{
 expect(refusal(()=>applyToolingPatchImagesV1(new Map(),"x".repeat(policy.maximumBytes+1)))).toBe("SDK patch input byte ceiling exceeded");
 const long="p".repeat(policy.maximumPathBytes+1),section=(path:string)=>"diff --git a/"+path+" b/"+path+"\n--- a/"+path+"\n+++ b/"+path+"\n@@ -1,1 +1,1 @@\n-a\n+b\n";
 expect(refusal(()=>applyToolingPatchImagesV1(new Map([[long,encoder.encode("a\n")]]),section(long)))).toBe("SDK patch path ceiling exceeded");
 const names=Array.from({length:policy.maximumFiles+1},(_,index)=>"f"+index+".js"),source=new Map(names.map(name=>[name,encoder.encode("a\n")]));
 expect(refusal(()=>applyToolingPatchImagesV1(source,names.map(section).join("")))).toBe("SDK patch output byte or file ceiling exceeded");
 expect(applyToolingPatchImagesV1(new Map(source),names.slice(0,policy.maximumFiles).map(section).join("")).size).toBe(policy.maximumFiles);
 expect(refusal(()=>applyToolingPatchImagesV1(new Map(),section("missing.js")))).toBe("Original bounded SDK patch image required");
 expect(()=>applyToolingPatchImagesV1(new Map([["f.js",Uint8Array.of(0xff,0xfe)]]),section("f.js"))).toThrow();
});

test("receipts hash the real bytes and refuse unapplied lines and moved context, as the corpus and Python hashlib say",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"tool-patch-")));
 try{
  for(const[index,row]of corpus.receipt.entries()){
   const base=join(root,"c"+index);for(const[name,text]of Object.entries(row.files)){mkdirSync(dirname(join(base,...name.split("/"))),{recursive:true});writeFileSync(join(base,...name.split("/")),text);}
   let receipt:{version:1;files:Record<string,string>}|null=null;const message=refusal(()=>{receipt=toolPatchReceipt(base,row.patch);});
   expect(message,row.name).toBe(row.error);
   if(row.error===""){
    const names=Object.keys(row.files),hashes=ask({op:"sha256",texts:names.map(name=>row.files[name as keyof typeof row.files] as string)});
    expect(receipt!.version).toBe(1);expect(receipt!.files).toEqual(Object.fromEntries(names.map((name,at)=>[name,hashes[at]])));
    expect(receipt!.files).toEqual(Object.fromEntries(names.map(name=>[name,createHash("sha256").update(row.files[name as keyof typeof row.files] as string).digest("hex")])));
   }
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
