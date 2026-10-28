import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {stateDirectoryV1} from "../../../📨️protocol/📍️endpoint/🟨️.mjs";
import {changeJournalDirectoryV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py"),python=process.platform==="win32"?"python":"python3";
const ask=(root:string)=>{const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({vectors:fixture.vectors,root}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as {expected:string[];account:{uid:string;home:string;id:string}};};
const win=(path:string)=>path.replaceAll("\\","/");

test("The journal state directory follows the platform layout, never reads TMPDIR, TMP or TEMP, and an independent Python computation agrees on every vector",()=>{
 const answer=ask("/w/semio");
 fixture.vectors.forEach((vector,index)=>{
  const actual=win(stateDirectoryV1(vector.root,vector.environment,vector.platform,vector.user,vector.home))+"/journal";
  expect(actual,vector.name).toBe(win(answer.expected[index]!));
  for(const name of ["TMPDIR","TMP","TEMP"] as const){const value=(vector.environment as Record<string,string|undefined>)[name];if(value)expect(actual.startsWith(win(value))).toBe(false);}
 });
});

test("Two environments that differ only in TMPDIR, TMP and TEMP reach the same journal directory, in this process and in separate processes",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"journal-dir-")));
 try{
  const base={PATH:process.env.PATH??"",HOME:process.env.HOME??""} as Record<string,string>,variants=[{},{TMPDIR:"/a/scratch"},{TMPDIR:"/b",TMP:"/b",TEMP:"/b"}].map(extra=>({...base,...extra}));
  const directories=variants.map(environment=>changeJournalDirectoryV1(root,environment));
  expect(new Set(directories).size).toBe(1);
  const answer=ask(root);
  if(process.platform==="darwin")expect(directories[0]).toBe(join(answer.account.home,"Library/Caches","semio","repo",answer.account.id,"journal"));
  if(process.platform==="linux")expect(directories[0]).toBe(join(answer.account.home,".cache","semio","repo",answer.account.id,"journal"));
  for(const directory of directories)for(const scratch of ["/a/scratch","/b",tmpdir()])expect(directory.startsWith(scratch+"/")).toBe(false);
  const program="import {changeJournalDirectoryV1} from "+JSON.stringify(join(here,"..","..","🟦️.ts"))+";process.stdout.write(changeJournalDirectoryV1("+JSON.stringify(root)+"));";
  const scratch=[mkdtempSync(join(tmpdir(),"journal-a-")),mkdtempSync(join(tmpdir(),"journal-b-"))];
  try{
   const seen=scratch.map(directory=>{const run=spawnSync(process.execPath,["-e",program],{env:{...base,TMPDIR:directory,TMP:directory,TEMP:directory},encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return run.stdout;});
   expect(seen[0]).toBe(seen[1]);
   expect(seen[0]).toBe(directories[0]);
  }finally{for(const directory of scratch)rmSync(directory,{recursive:true,force:true});}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("An explicit XDG_RUNTIME_DIR still isolates a journal, which the journal laws rely on",()=>{
 const one=changeJournalDirectoryV1("/w/a",{XDG_RUNTIME_DIR:"/run/one"}),two=changeJournalDirectoryV1("/w/a",{XDG_RUNTIME_DIR:"/run/two"});
 if(process.platform==="win32")return;
 expect(one.startsWith("/run/one/")).toBe(true);
 expect(two.startsWith("/run/two/")).toBe(true);
 expect(one.endsWith("/journal")).toBe(true);
});
