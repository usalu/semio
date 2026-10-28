/** 🪢️ Contract: the publication of a receipt on a file system without hard links creates the destination name exclusively, so a peer that publishes the name at any moment is never replaced, whatever it left there (other bytes, the same bytes, a link that leads nowhere). The oracle is the destination as it stands on disk afterwards, byte for byte. */
import {expect,test} from "bun:test";
import {createHash}from"node:crypto";
import {lstatSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {publishWithoutLinkV1} from "../../🟦️.ts";

const scratch=()=>realpathSync(mkdtempSync(join(tmpdir(),"link-less-"))),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("a free name is published with the bytes of the receipt and the temporary stays the caller's",()=>{
 const root=scratch();
 try{
  const temporary=join(root,".r.tmp"),destination=join(root,"r.json");writeFileSync(temporary,'{"a":1}');
  expect(publishWithoutLinkV1(temporary,destination,sha('{"a":1}'))).toBe("published");
  expect(readFileSync(destination,"utf8")).toBe('{"a":1}');expect(readdirSync(root).sort()).toEqual([".r.tmp","r.json"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a name a peer holds with the same bytes is the same receipt, with other bytes a conflict, and neither is replaced",()=>{
 const root=scratch();
 try{
  const temporary=join(root,".r.tmp");writeFileSync(temporary,"mine");
  writeFileSync(join(root,"same.json"),"mine");writeFileSync(join(root,"other.json"),"theirs");
  expect(publishWithoutLinkV1(temporary,join(root,"same.json"),sha("mine"))).toBe("same");
  expect(publishWithoutLinkV1(temporary,join(root,"other.json"),sha("mine"))).toBe("conflict");
  expect(readFileSync(join(root,"other.json"),"utf8")).toBe("theirs");expect(readFileSync(join(root,"same.json"),"utf8")).toBe("mine");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a name that looks free to an existence check but is taken (a link that leads nowhere) is a conflict and the entry survives",()=>{
 const root=scratch();
 try{
  const temporary=join(root,".r.tmp"),destination=join(root,"r.json");writeFileSync(temporary,"mine");symlinkSync(join(root,"nowhere"),destination);
  expect(publishWithoutLinkV1(temporary,destination,sha("mine"))).toBe("conflict");
  expect(lstatSync(destination).isSymbolicLink()).toBe(true);expect(readlinkSync(destination)).toBe(join(root,"nowhere"));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a copy that does not read back as the receipt is removed again and refused",()=>{
 const root=scratch();
 try{
  const temporary=join(root,".r.tmp"),destination=join(root,"r.json");writeFileSync(temporary,"mine");
  expect(()=>publishWithoutLinkV1(temporary,destination,sha("something else"))).toThrow("changed while it was retained");
  expect(readdirSync(root)).toEqual([".r.tmp"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
