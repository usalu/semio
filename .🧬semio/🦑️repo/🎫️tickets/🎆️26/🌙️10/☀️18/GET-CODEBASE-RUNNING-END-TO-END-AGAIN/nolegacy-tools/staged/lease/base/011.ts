import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "../🧫️fixtures/🕸️wait-for.json";
import {findLeaseCycleV1,type LeaseNodeV1} from "../../🟦️.ts";
import {leaseNodesV1 as registryNodesV1} from "../../🗂️registry/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url));
const answers=(()=>{const run=spawnSync("python3",["-I",join(here,"🐍️.py")],{input:JSON.stringify(fixture),encoding:"utf8"});if(run.status!==0)throw Error("Python wait-for oracle failed: "+run.stderr);return JSON.parse(run.stdout) as Record<string,boolean>;})();

for(const graph of fixture.graphs)test(`wait-for graph ${graph.id}: queue edges and awaited descendants close the cycle exactly where the Python graphlib oracle finds one`,()=>{
 const nodes=graph.nodes as unknown as readonly LeaseNodeV1[],cyclic=nodes.some(node=>findLeaseCycleV1(nodes,node.id)!==undefined);
 expect(answers[graph.id],"oracle").toBe(graph.cycle);
 expect(cyclic,"product").toBe(graph.cycle);
 if(graph.cycle){const found=nodes.map(node=>findLeaseCycleV1(nodes,node.id)).find(Boolean)!;expect(found.members.includes(found.victim)).toBe(true);}
});

test("registry entries carry the task ancestry, so the nodes built from other processes link a waiting child to its holding ancestor",()=>{
 const directory="/w/d",entry={version:1 as const,pid:4242,startedAtMs:1,updatedAtMs:2,command:"x",directory,holds:[{resource:"a",mode:"shared" as const,sinceMs:1,task:1}],waits:[{resource:"b",mode:"exclusive" as const,sinceMs:2,task:2,queued:true}],ancestry:{"2":[1]}};
 const nodes=registryNodesV1([entry],{holds:[],waits:[],ancestry:{}},directory,1);
 expect(nodes.find(node=>node.id==="4242/2")?.ancestors).toEqual(["4242/1"]);expect(nodes.find(node=>node.id==="4242/1")?.ancestors).toBeUndefined();
});
