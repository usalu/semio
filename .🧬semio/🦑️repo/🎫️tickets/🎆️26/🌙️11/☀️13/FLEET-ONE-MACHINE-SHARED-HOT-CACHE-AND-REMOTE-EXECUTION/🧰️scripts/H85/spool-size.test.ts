import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {spoolBoundV1,spoolDirectoryV1,spoolLengthV1} from "../🟦️.ts";

const fleet=schema.definitions.Policy.const;

test("the spool directory is the declared state and spool layout of the cache root, the length counts the queued jobs only and a missing directory is empty",()=>{
 const root=mkdtempSync(join(tmpdir(),"spool-size-"));
 try{
  expect(spoolDirectoryV1(root)).toBe(join(root,fleet.layout.state,fleet.layout.spool));
  expect(spoolLengthV1(root)).toBe(0);
  mkdirSync(spoolDirectoryV1(root),{recursive:true});
  for(const name of ["a.json","b.json",".c.tmp","d.txt"])writeFileSync(join(spoolDirectoryV1(root),name),"");
  expect(spoolLengthV1(root)).toBe(2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the bound stays between the declared minimum and maximum and does not move inside its time to live",()=>{
 const root=mkdtempSync(join(tmpdir(),"spool-bound-"));
 try{
  const bound=spoolBoundV1(root);
  expect(bound).toBeGreaterThanOrEqual(fleet.spool.minimumEntries);
  expect(bound).toBeLessThanOrEqual(fleet.spool.maximumEntries);
  expect(spoolBoundV1(root)).toBe(bound);
  expect(spoolBoundV1(join(root,"absent"))).toBe(fleet.spool.minimumEntries);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the module imports nothing but the file system, the path module and its schema, so a status reader never loads the uploader",()=>{
 const text=readFileSync(join(import.meta.dir,"..","🟦️.ts"),"utf8"),imports=[...text.matchAll(/^import .* from "([^"]+)"/gmu)].map(row=>row[1]);
 expect(imports.sort()).toEqual(["../🧬️schema/🔣️.json","node:fs","node:path"]);
});
