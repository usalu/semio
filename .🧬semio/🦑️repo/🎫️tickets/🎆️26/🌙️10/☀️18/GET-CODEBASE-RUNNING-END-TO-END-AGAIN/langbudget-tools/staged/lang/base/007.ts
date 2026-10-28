/** 🧭️ Contract: a capture that shares one ancestry ledger reads exactly the bytes a ledger-free read returns, looks at each directory once, and still refuses when a directory it relied on is replaced before the capture ends. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNxAuthorizerAncestryV1,readNxAuthorizerSourceV1,verifyNxAuthorizerAncestryV1} from "../../🟦️.ts";

const signal=new AbortController().signal;
const tree=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"ancestry-")));mkdirSync(join(root,"a/b/c"),{recursive:true});mkdirSync(join(root,"elsewhere"));for(const[path,text]of Object.entries({"a/one.ts":"export const one=1;\n","a/b/two.ts":"export const two=2;\n","a/b/c/three.ts":"export const three=3;\n"}))writeFileSync(join(root,path),text);return root;};

test("a ledger changes how often directories are looked at, never what is read",async()=>{
 const root=tree();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  for(const path of["a/one.ts","a/b/two.ts","a/b/c/three.ts"]){
   const plain=await readNxAuthorizerSourceV1(root,path,signal),shared=await readNxAuthorizerSourceV1(root,path,signal,ledger);
   expect(Buffer.from(shared).equals(Buffer.from(plain)),path).toBe(true);
  }
  expect([...ledger.known.keys()].map(path=>path.slice(root.length)).sort()).toEqual(["","/a","/a/b","/a/b/c"]);
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a directory that is replaced while the capture runs is refused when the capture ends",async()=>{
 const root=tree();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  await readNxAuthorizerSourceV1(root,"a/b/two.ts",signal,ledger);
  renameSync(join(root,"a/b"),join(root,"a/b.moved"));symlinkSync(join(root,"elsewhere"),join(root,"a/b"));
  await expect(verifyNxAuthorizerAncestryV1(ledger,signal)).rejects.toThrow("ancestry changed during capture");
  rmSync(join(root,"a/b"),{force:true});renameSync(join(root,"a/b.moved"),join(root,"a/b"));
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a symlinked ancestor is refused at once and a missing one keeps its error code for the caller",async()=>{
 const root=tree();
 try{
  symlinkSync(join(root,"a"),join(root,"linked"));
  const ledger=createNxAuthorizerAncestryV1();
  await expect(readNxAuthorizerSourceV1(root,"linked/one.ts",signal,ledger)).rejects.toThrow("No-follow authorizer directory required");
  await expect(readNxAuthorizerSourceV1(root,"absent/one.ts",signal,ledger)).rejects.toMatchObject({code:"ENOENT"});
  await expect(readNxAuthorizerSourceV1(root,"absent/two.ts",signal,ledger)).rejects.toMatchObject({code:"ENOENT"});
  await verifyNxAuthorizerAncestryV1(ledger,signal);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a cancelled capture does not finish the final look",async()=>{
 const root=tree(),abort=new AbortController();
 try{
  const ledger=createNxAuthorizerAncestryV1();
  await readNxAuthorizerSourceV1(root,"a/one.ts",signal,ledger);abort.abort(Error("cancelled"));
  await expect(verifyNxAuthorizerAncestryV1(ledger,abort.signal)).rejects.toThrow("cancelled");
 }finally{rmSync(root,{recursive:true,force:true});}
});
