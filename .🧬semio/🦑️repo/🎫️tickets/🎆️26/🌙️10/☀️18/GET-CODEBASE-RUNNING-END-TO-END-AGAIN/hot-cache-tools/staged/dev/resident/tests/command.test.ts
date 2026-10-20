import {afterAll,describe,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {runResidentCommandV1,RESIDENT_EXCLUDES_V1} from "../command/index.mjs";
import {createResidentServiceV1} from "../service/index.mjs";
import {endpointsV1} from "../protocol/endpoint.mjs";
import {POLICY_V1} from "../protocol/policy.mjs";
import {cleanup,sleep,tempRoot,waitFor} from "./common.ts";

const cleanups:(()=>Promise<void>|void)[]=[];afterAll(async()=>{for(const fn of cleanups.reverse())await fn();});
const sink=()=>{const parts:string[]=[];return{parts,write:(t:string)=>{parts.push(String(t));return true;},text:()=>parts.join("")};};

async function serviceFor(){
 const root=tempRoot("cmd-");cleanups.push(()=>cleanup(root));mkdirSync(root+"/pkgs/a",{recursive:true});writeFileSync(root+"/pkgs/a/x","x");
 const base=mkdtempSync("/tmp/sr-");cleanups.push(()=>cleanup(base));
 const environment={...process.env,XDG_RUNTIME_DIR:base};
 const runtime=endpointsV1(root,environment);
 const service=createResidentServiceV1({root,runtime,implementation:"2".repeat(64),backend:"none",exclude:{rooted:[],names:[]},fenceDirectory:".semio-fence",policy:POLICY_V1,
  ports:{graph:{load:async()=>({generation:1,owners:[{name:"a",root:"pkgs/a",deps:[],tasks:[]}],env:{paths:[],prefixes:[]}}),environmentDigest:async()=>"s",listing:async()=>({directories:[["",1,1]],members:[["pkgs/a/x",33188]]})},execution:{run:async()=>({exit:0})}}});
 await service.start();cleanups.push(()=>service.stop("test"));
 return{root,environment,service,runtime};
}

describe("repo service",()=>{
 test("status, probe, issue, drop-caches and stop in the requested language",async()=>{
  const s=await serviceFor();
  const run=async(args:string[],languages=["de","en"])=>{const out=sink(),err=sink();const exit=await runResidentCommandV1({root:s.root,args,languages,environment:s.environment,stdout:out,stderr:err});return{exit,out:out.text(),err:err.text()};};
  const status=await run(["status","--json"]);expect(status.exit).toBe(0);expect(JSON.parse(status.out).watcher.backend).toBe("none");
  expect((await run(["probe"])).exit).toBe(0);
  const token=await run(["issue","--classes","run","--ttl","30"]);expect(token.exit).toBe(0);expect(token.out.trim()).toMatch(/^v1\./u);
  expect((await run(["drop-caches"])).out).toContain("dropped");
  const usage=await run(["frobnicate"],["de"]);expect(usage.exit).toBe(1);expect(usage.err).toContain("Verwendung");
  expect((await run(["frobnicate"],["en"])).err).toContain("Usage");
  const stopped=await run(["stop"],["de"]);expect(stopped.exit).toBe(0);expect(stopped.out).toContain("beendet");
  expect(await waitFor(async()=>(await run(["probe"])).exit===1,3000)).toBe(true);
  const down=await run(["status"],["en"]);expect(down.exit).toBe(1);expect(down.out).toContain("No resident service");
  expect((await run(["stop"],["en"])).out).toContain("No resident service");
 });

 test("start detaches a child and waits until it answers; an exiting child is reported",async()=>{
  const root=tempRoot("cmd-start-");cleanups.push(()=>cleanup(root));
  const base=mkdtempSync("/tmp/sr-");cleanups.push(()=>cleanup(base));
  const environment={...process.env,XDG_RUNTIME_DIR:base};
  const out=sink(),err=sink();
  const exit=await runResidentCommandV1({root,args:["start"],languages:["en"],environment,stdout:out,stderr:err,execPath:process.execPath,scriptArguments:["-e","process.exit(3)","--"],startTimeoutMilliseconds:5000});
  expect(exit).toBe(1);expect(err.text()).toContain("exited");
 });

 test("excludes keep caches, VCS data and dependency trees out of the watch",()=>{
  expect(RESIDENT_EXCLUDES_V1.rooted).toContain(".git");expect(RESIDENT_EXCLUDES_V1.names).toContain("node_modules");expect(RESIDENT_EXCLUDES_V1.rooted.length).toBeLessThanOrEqual(8+8);
 });
});
