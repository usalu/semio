import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{appendCargoTimingLedgerV1,cargoTimingLedgerRowV1}from"../../🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};

/** 📒️ Every Cargo launch leaves one compact row per phase mark in a bounded ledger next to the store, whatever the environment says, so a Bazel action whose stderr is swallowed still shows where its seconds went; Python parses the same lines. */
const python=Bun.which("python3")??Bun.which("python");
test("ledger rows validate, append, rotate at the bound, and Python reads them back",()=>{
 const directory=mkdtempSync(join(tmpdir(),"rf-ledger-")),file=join(directory,"timing","phases.jsonl");
 try{
  const rows=fixture.marks.map((mark,index)=>cargoTimingLedgerRowV1(4242,"cargo:build",mark.phase,mark.point,1_700_000_000_000+index,mark.elapsed));
  appendCargoTimingLedgerV1(file,rows.join(""));
  expect(()=>cargoTimingLedgerRowV1(1,"cargo:build","nonsense","begin",1,1)).toThrow();expect(()=>cargoTimingLedgerRowV1(1,"cargo:build","graph","begin",1,-1)).toThrow();
  expect(python,"an independent Python is required").toBeTruthy();
  const out=spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),file],{encoding:"utf8"});expect(out.status,out.stderr).toBe(0);
  expect(JSON.parse(out.stdout)).toEqual(fixture.expected);
  const bound=statSync(file).size;appendCargoTimingLedgerV1(file,rows.join(""),bound);expect(existsSync(file+".1")).toBe(true);expect(readFileSync(file,"utf8")).toBe(rows.join(""));
  writeFileSync(join(directory,"blocker"),"x");appendCargoTimingLedgerV1(join(directory,"blocker","phases.jsonl"),rows[0]!);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
