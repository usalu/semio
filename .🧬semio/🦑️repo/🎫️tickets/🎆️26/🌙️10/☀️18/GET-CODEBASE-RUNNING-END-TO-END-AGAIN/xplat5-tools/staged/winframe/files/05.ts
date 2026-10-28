/** 🪟️ Contract: a membership frontier captured by the native scan of a platform that cannot name inodes and POSIX modes (Windows: `0` for both) still proves unchanged membership against the `lstat` of that platform, and every fact the scan does know still contradicts a change. */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import * as generations from "../../🟨️.mjs";
import {createFrontierV1,diffFrontiersV1,frontierHoldsV1,stableDirectoriesV1} from "../../🟨️.mjs";
import {memoStampsHoldV1,packMemoStampsV1} from "../../../../🧭️invocation/🕸️graph/🧠️memo/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py"),python=process.platform==="win32"?"python":"python3";
const roots:string[]=[];
afterAll(()=>roots.forEach(root=>rmSync(root,{recursive:true,force:true})));
const now=1_700_000_000_000,old=now-3_600_000,windowsNtfsId=281474976710913,libuvFile=0o100666;
const windowsScan=()=>createFrontierV1({capturedAt:now,directories:[[".",old,0],["src",old,0]],members:[["a.ts",0],["src/b.ts",0]]});
const posixScan=()=>createFrontierV1({capturedAt:now,directories:[[".",old,10],["src",old,11]],members:[["a.ts",0o100644],["src/b.ts",0o100644]]});
const present=(table:Record<string,{directory:boolean;mtimeMs:number;ino:number;mode:number}|undefined>)=>async(path:string)=>{const row=table[path];return row===undefined?undefined:{directory:row.directory,file:!row.directory,mtimeMs:row.mtimeMs,ino:row.ino,mode:row.mode};};
const windowsStat=(overrides:Record<string,Partial<{mtimeMs:number;ino:number;mode:number}>>={})=>present({
 ".":{directory:true,mtimeMs:old,ino:windowsNtfsId,mode:0o040777,...overrides["."]},"src":{directory:true,mtimeMs:old,ino:windowsNtfsId+1,mode:0o040777,...overrides.src},
 "a.ts":{directory:false,mtimeMs:old,ino:windowsNtfsId+2,mode:libuvFile,...overrides["a.ts"]},"src/b.ts":{directory:false,mtimeMs:old,ino:windowsNtfsId+3,mode:libuvFile,...overrides["src/b.ts"]},
});

test("the fact rule equals an independent Python reading on every vector",()=>{
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify(fixture.facts.map(row=>({present:row.present,recorded:row.recorded}))),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as boolean[];
 fixture.facts.forEach((row,index)=>{
  expect(expected[index],row.name+" (oracle)").toBe(row.holds);
  expect((generations as unknown as {frontierFactHoldsV1:(present:number,recorded:number)=>boolean}).frontierFactHoldsV1(row.present,row.recorded),row.name).toBe(row.holds);
 });
});

test("a frontier from the Windows scan (inode 0, mode 0) holds against Windows stats, and a moved timestamp or a missing entry still breaks it",async()=>{
 expect(await frontierHoldsV1(windowsScan(),windowsStat())).toBe(true);
 expect(await frontierHoldsV1(windowsScan(),windowsStat({src:{mtimeMs:old+1}}))).toBe(false);
 expect(await frontierHoldsV1(windowsScan(),present({".":{directory:true,mtimeMs:old,ino:1,mode:0}}))).toBe(false);
 expect(await frontierHoldsV1(windowsScan(),windowsStat({"a.ts":{mtimeMs:now}}))).toBe(true);
});

test("a frontier from a POSIX scan still breaks on a moved inode, a moved mode or a directory touched inside the racy window",async()=>{
 const stat=(overrides:Record<string,Partial<{mtimeMs:number;ino:number;mode:number}>>={})=>present({".":{directory:true,mtimeMs:old,ino:10,mode:0o040755,...overrides["."]},"src":{directory:true,mtimeMs:old,ino:11,mode:0o040755,...overrides.src},"a.ts":{directory:false,mtimeMs:old,ino:12,mode:0o100644,...overrides["a.ts"]},"src/b.ts":{directory:false,mtimeMs:old,ino:13,mode:0o100644,...overrides["src/b.ts"]}});
 expect(await frontierHoldsV1(posixScan(),stat())).toBe(true);
 expect(await frontierHoldsV1(posixScan(),stat({src:{ino:99}}))).toBe(false);
 expect(await frontierHoldsV1(posixScan(),stat({"a.ts":{mode:0o100755}}))).toBe(false);
 const racy=createFrontierV1({capturedAt:now,directories:[[".",now-10,10]],members:[]});
 expect(await frontierHoldsV1(racy,present({".":{directory:true,mtimeMs:now-10,ino:10,mode:0o040755}}))).toBe(false);
});

test("a stat frontier next to a Windows scan frontier names no phantom change and no lost stability, and a real change is still named",()=>{
 const stat=createFrontierV1({capturedAt:now+1,directories:[[".",old,windowsNtfsId],["src",old,windowsNtfsId+1]],members:[["a.ts",libuvFile],["src/b.ts",libuvFile]]});
 expect(diffFrontiersV1(windowsScan(),stat)).toEqual([]);
 expect([...stableDirectoriesV1(windowsScan(),stat)].sort()).toEqual([".","src"]);
 const moved=createFrontierV1({capturedAt:now+1,directories:[[".",old,windowsNtfsId],["src",old+5,windowsNtfsId+1]],members:[["a.ts",libuvFile],["src/b.ts",libuvFile],["src/c.ts",libuvFile]]});
 expect(diffFrontiersV1(windowsScan(),moved).map(change=>change.kind+":"+change.path).sort()).toEqual(["added:src/c.ts","changed:src"]);
 expect([...stableDirectoriesV1(windowsScan(),moved)]).toEqual(["."]);
 const posixMoved=createFrontierV1({capturedAt:now+1,directories:[[".",old,10],["src",old,77]],members:[["a.ts",0o100644],["src/b.ts",0o100644]]});
 expect(diffFrontiersV1(posixScan(),posixMoved).map(change=>change.kind+":"+change.path)).toEqual(["changed:src"]);
});

test("the memo directory stamp of a Windows scan (inode 0) holds for the directory it stamped and breaks on a moved timestamp, and a POSIX stamp still breaks on a moved inode",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"native-frontier-")));roots.push(root);
 mkdirSync(join(root,"src"));writeFileSync(join(root,"src","a.ts"),"x");
 const status=lstatSync(join(root,"src"));
 const table=(ino:number,mtimeMs=status.mtimeMs)=>packMemoStampsV1(new Map([["src",[mtimeMs,ino] as const]]),false);
 expect(memoStampsHoldV1(root,table(0),["src"],false)).toBe(true);
 expect(memoStampsHoldV1(root,table(0,status.mtimeMs+1),["src"],false)).toBe(false);
 expect(memoStampsHoldV1(root,table(status.ino),["src"],false)).toBe(true);
 expect(memoStampsHoldV1(root,table(status.ino+1),["src"],false)).toBe(false);
});
