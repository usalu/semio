import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync,symlinkSync,utimesSync,writeFileSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{foreignSourceInventoryV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import{foreignDigestMemoV1}from"../🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};

/** 📸️ A foreign crate directory is hashed once: the second walk recalls every digest by stamp and reads no byte, a rewrite that keeps size and mtime still misses on ctime, added and removed files show up, the book survives a new process, and Python's walk and sha-256 give the same inventory. */
const python=Bun.which("python3")??Bun.which("python"),later=()=>Date.now()+3_600_000,hooks={check:()=>{},tick:async()=>{}};
function build(root:string){for(const row of fixture.files){const path=join(root,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.text);}}
function counted(memo:ReturnType<typeof foreignDigestMemoV1>){const counts={recalled:0,remembered:0};return{counts,memo:{open:(directory:string,startedMs:number)=>{const book=memo.open(directory,startedMs);return{recall:(path:string,stamp:any)=>{const digest=book.recall(path,stamp);if(digest!==undefined)counts.recalled++;return digest;},remember:(path:string,stamp:any,digest:string)=>{counts.remembered++;book.remember(path,stamp,digest);},close:()=>book.close()};}}};}
const asMap=(rows:readonly{path:string;digest:string;bytes:number}[])=>Object.fromEntries(rows.map(row=>[row.path,{digest:row.digest,bytes:row.bytes}]));

test("cold walk hashes, warm walk recalls, a ctime change misses, and the book outlives the memo object",async()=>{
 const base=mkdtempSync(join(tmpdir(),"rf-foreign-")),root=join(base,"crate"),cache=join(base,"cache"),expected=fixture.files.filter(row=>!row.skipped).length;
 try{
  build(root);
  const one=counted(foreignDigestMemoV1(cache)),cold=await foreignSourceInventoryV1(root,one.memo,hooks,later);
  expect(cold.length).toBe(expected);expect(one.counts).toEqual({recalled:0,remembered:expected});
  const two=counted(foreignDigestMemoV1(cache)),warm=await foreignSourceInventoryV1(root,two.memo,hooks,later);
  expect(warm).toEqual(cold);expect(two.counts).toEqual({recalled:expected,remembered:0});
  const target=join(root,fixture.rewrite.path),before=statSync(target);writeFileSync(target,fixture.rewrite.sameSizeText);utimesSync(target,before.atime,before.mtime);
  const three=counted(foreignDigestMemoV1(cache)),changed=await foreignSourceInventoryV1(root,three.memo,hooks,later);
  expect(three.counts).toEqual({recalled:expected-1,remembered:1});expect(asMap(changed)[target]!.digest).not.toBe(asMap(cold)[target]!.digest);
  writeFileSync(join(root,"added.rs"),"pub fn added(){}\n");rmSync(join(root,fixture.removed));
  const four=counted(foreignDigestMemoV1(cache)),moved=await foreignSourceInventoryV1(root,four.memo,hooks,later);
  expect(moved.length).toBe(expected);expect(four.counts.remembered).toBe(1);expect(Object.keys(asMap(moved))).toContain(join(root,"added.rs"));expect(Object.keys(asMap(moved))).not.toContain(join(root,fixture.removed));
  expect(python,"an independent Python is required").toBeTruthy();
  const out=spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),root,...fixture.skip],{encoding:"utf8"});expect(out.status,out.stderr).toBe(0);
  expect(JSON.parse(out.stdout)).toEqual(asMap(moved));
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a young file is never remembered, a poisoned book is ignored, and a symlink refuses the walk",async()=>{
 const base=mkdtempSync(join(tmpdir(),"rf-foreign-")),root=join(base,"crate"),cache=join(base,"cache");
 try{
  build(root);
  const young=counted(foreignDigestMemoV1(cache));await foreignSourceInventoryV1(root,young.memo,hooks);
  const again=counted(foreignDigestMemoV1(cache));await foreignSourceInventoryV1(root,again.memo,hooks);expect(again.counts.recalled).toBe(0);
  const fresh=counted(foreignDigestMemoV1(cache));await foreignSourceInventoryV1(root,fresh.memo,hooks,later);
  const books=spawnSync("find",[cache,"-name","*.json"],{encoding:"utf8"}).stdout.trim().split("\n");expect(books.length).toBe(1);writeFileSync(books[0]!,'{"version":1,"directory":"x","files":{"y":["1","2","3","4","5","not-a-digest"]}}');
  const poisoned=counted(foreignDigestMemoV1(cache));await foreignSourceInventoryV1(root,poisoned.memo,hooks,later);expect(poisoned.counts.recalled).toBe(0);
  symlinkSync(join(root,"Cargo.toml"),join(root,"link.rs"));
  await expect(foreignSourceInventoryV1(root,undefined,hooks)).rejects.toThrow("symlink");
 }finally{rmSync(base,{recursive:true,force:true});}
});
