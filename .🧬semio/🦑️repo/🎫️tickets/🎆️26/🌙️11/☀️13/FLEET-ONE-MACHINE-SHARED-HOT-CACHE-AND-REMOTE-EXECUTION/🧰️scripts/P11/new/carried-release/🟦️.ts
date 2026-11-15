/** 🔁️ Contract: the host is told whether more waves of the same read follow when it releases what a wave filled, so the setup of the whole workspace and the closure entries it holds are carried to the next wave and dropped by the last one and by the end of the load; the stages the load adds around the discovery are told with the wave, a load tells what its reader and its prefetch took with the build that follows them, the first build of the process tells its start and a wave tells the seconds the read paused after the wave before it. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GENERATION_POLICY_V1,carriedStagesV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

const BASE=GENERATION_POLICY_V1.ownerWave,COUNT=BASE*2+7,sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const carried=${at};\n`);};
type Info={phases?:Record<string,number>};

test("a load tells the seconds of its reader and of its prefetch with the build that follows them, and only the first build of the process tells its start",async()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-boot-stages-")),first:Info[]=[],later:Info[]=[];
 try{
  await seed(dir,ringFiles(8));
  const cold=createMockHost(dir);
  await loadGenerationGraphV1({...cold.host,published:async(_generation:number,info:Info)=>{first.push(info);}} as never);
  expect(first.length).toBe(1);
  expect(Number.isSafeInteger(first[0]!.phases!.start)).toBe(true);expect(Number.isSafeInteger(first[0]!.phases!["start.reader"])).toBe(true);
  expect(first[0]!.phases!["start.prefetch"]).toBeUndefined();
  await edit(dir,3);
  const mock=createMockHost(dir,{prefetch:async()=>{}});
  await loadGenerationGraphV1({...mock.host,builderIdle:async()=>false,published:async(_generation:number,info:Info)=>{later.push(info);}} as never);
  expect(mock.events).toContain("prefetch");
  expect(later.length).toBe(1);
  expect(later[0]!.phases!.start).toBeUndefined();
  for(const name of ["start.reader","start.prefetch"])expect(Number.isSafeInteger(later[0]!.phases![name]),name).toBe(true);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("every release between waves says more follows, the last wave and the end of the load say nothing follows, and the load adds its cutoff and release stages",async()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-carried-release-")),releases:unknown[]=[],infos:Info[]=[];
 try{
  await seed(dir,ringFiles(COUNT+3));
  const first=createMockHost(dir);
  await loadGenerationGraphV1({...first.host,release:async()=>{}} as never);
  await edit(dir,COUNT);
  const mock=createMockHost(dir);
  const host={...mock.host,release:async(options?:{more?:boolean})=>{releases.push(options);},published:async(_generation:number,info:Info)=>{infos.push(info);carriedStagesV1.yield=7;}};
  await loadGenerationGraphV1(host as never);
  expect(releases).toEqual([{more:true},{more:true},{more:false},undefined]);
  expect(infos.length).toBe(3);
  for(const info of infos)for(const name of ["compute.cutoff","compute.release"])expect(Number.isSafeInteger(info.phases![name]),name).toBe(true);
  expect(infos.map(info=>info.phases!.yield)).toEqual([undefined,7,7]);expect(carriedStagesV1.yield).toBe(7);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
