/** 🔓️ Contract: a project farm pool whose workers are refused (their identity differs from the parent's) gives its host capacity grant back at once, not when the graph construction closes, so the memory and cores it reserved for workers that never run are free for the other runs of the machine.
 * @see ../../🟨️.mjs
 * @see ../../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-agents16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {EventEmitter} from "node:events";
import {openProjectFarmV1} from "../../🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";

const roots:string[]=[],scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"graph-farm-release-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

/** 🪪️ A scripted worker whose identity differs from the parent's: it answers `init` as another implementation would. */
class StrangerChild extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 send(message:{type:string}){
  if(message.type==="init")setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:{implementation:"another-implementation",records:inferenceRecordIdentityV1()}}));
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("a farm whose host-granted workers are refused gives the host grant back before the graph construction closes",async()=>{
 const released:string[]=[],lines:string[]=[];let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const farm=openProjectFarmV1({workspaceRoot:scratch(),context:{registry:null},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8"},processed:()=>24,cores:10,write:line=>lines.push(line),
  spawnProcess:(()=>new StrangerChild()) as never,
  request:(()=>({granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;}),release:()=>{released.push("host");}})) as never})!;
 const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 for(let index=0;index<24;index++)await farm.prepare(job);
 grant({jobs:2,megabytes:3584});
 for(let wait=0;wait<100&&released.length===0;wait++)await Bun.sleep(20);
 expect(released,lines.join("\n")).toEqual(["host"]);
 expect(await farm.prepare(job)).toBe(false);
 await farm.close();
});
