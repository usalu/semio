import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import * as journal from "../../🟦️.ts";
import type {ChangeWatchFactoryV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py"),python=process.platform==="win32"?"python":"python3",policy=CHANGE_JOURNAL_POLICY_V1;
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-lock-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const {ChangeJournalRunningErrorV1,runChangeJournalServiceV1}=journal;
const factory:ChangeWatchFactoryV1=()=>({async start(){return{backend:"fsevents",trust:"exact"};},async stop(){}});
const workspace=()=>{const root=join(sandbox,"w"+counter),directory=join(sandbox,"j"+counter++);mkdirSync(root,{recursive:true});mkdirSync(directory,{recursive:true});return{root,directory,lock:join(directory,CHANGE_JOURNAL_FILES_V1.lock)};};
const start=(root:string,directory:string)=>runChangeJournalServiceV1({root,directory,fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},createSupervisor:factory,residentBytes:()=>0});

test("the lock decision equals an independent Python reading on every vector, and no process id that is not a positive integer ever holds a lock by being alive",()=>{
 const run=spawnSync(python,["-I",oracle],{input:JSON.stringify({vectors:fixture.vectors,heartbeatStaleMilliseconds:policy.heartbeatStaleMilliseconds,startTimeoutMilliseconds:policy.startTimeoutMilliseconds}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout).held as boolean[];
 fixture.vectors.forEach((vector,index)=>{
  const actual=(journal as unknown as {changeLockHeldV1:(facts:unknown,policy:unknown)=>boolean}).changeLockHeldV1({pid:vector.pid,alive:vector.alive,heartbeatAgeMilliseconds:vector.heartbeatAgeMilliseconds,ageMilliseconds:vector.ageMilliseconds},policy);
  expect(actual,vector.name).toBe(expected[index]!);
  expect(actual,vector.name+" (fixture)").toBe(vector.held);
 });
});

test("a lock directory whose owner file is not written yet is a service that is starting: a second start is refused, and the same lock after the start timeout is reclaimed",async()=>{
 const{root,directory,lock}=workspace();
 mkdirSync(lock,{mode:0o700});
 await expect(start(root,directory)).rejects.toBeInstanceOf(ChangeJournalRunningErrorV1);
 const past=new Date(Date.now()-policy.startTimeoutMilliseconds*4);
 utimesSync(lock,past,past);
 const service=await start(root,directory);
 await service.stop("test");
});

test("an owner file that names process id 0 never keeps a lock beyond the start timeout",async()=>{
 const{root,directory,lock}=workspace();
 mkdirSync(lock,{mode:0o700});
 writeFileSync(join(lock,"owner.json"),JSON.stringify({pid:0,at:Date.now()-policy.startTimeoutMilliseconds*4}));
 const service=await start(root,directory);
 await service.stop("test");
});
