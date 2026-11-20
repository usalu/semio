/** 🌀️ Contract: a row log is a process-private spill of the graph writer, written under the closure store as `rl/rl-<pid>-<time>-<serial>.rows`; the closure proof knows it as a spill (never read, never a member, never retired) instead of refusing the whole store with "Unknown closure reference publication", and every file shape a read writes under the store is either a known closure role or a known spill, never neither. */
import {expect,test} from "bun:test";
import {readFileSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import {physicalSourceOriginV1} from "../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {rowLogFileNameV1} from "../../../../../../../../../🏗️build/🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs";

const origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
const source=(url:string)=>readFileSync(new URL(url,origin),"utf8");
const tree=ts.createSourceFile("closures.ts",source("../../🟦️.ts"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const declared=(name:string)=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!;
const load=Function(ts.transpileModule(declared("role").getText(tree)+"\n"+declared("spill").getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return{role,spill}")() as{role:(name:string)=>string;spill:(name:string)=>boolean};
const hash="8d8f21d7".repeat(8),log=rowLogFileNameV1(38258,1763470000000,0);

test("the exact refusal: a row log named by the writer is no closure role, and it is a spill in every writer directory",()=>{
 expect(log).toMatch(/^rl-38258-[0-9a-z]+-0\.rows$/u);
 const names=["inference/selections/"+hash+"/rl/"+log,"inference/contexts/"+hash+"/rl/"+log,"inference/rl/"+log];
 for(const name of names){expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);expect(load.spill(name),name).toBe(true);}
});

test("nothing near a spill is one",()=>{
 const near=["inference/selections/"+hash+"/rl/"+log+".tmp","inference/selections/"+hash+"/np/"+log,"inference/selections/short/rl/"+log,"rl/"+log,"inference/selections/"+hash+"/rl/rl-x-1-0.rows","inference/selections/"+hash+"/rl/sub/"+log,"inference/selections/"+hash+"/rl/"+log.toUpperCase(),"inference/selections/"+hash+"/rl/other.rows","generations/rl/"+log];
 for(const name of near)expect(load.spill(name),name).toBe(false);
});

test("every file name the writers of the store can produce is exactly one of a closure role or a spill",()=>{
 const roles=["inference/members.jsonl","build-input-closures/"+"a".repeat(64)+".json","snapshots/"+"a".repeat(64)+".gz","authority.json","graph.json","generations/current.json","inference/selections/"+hash+"/np/"+"a".repeat(24)+".json","inference/selections/"+hash+"/np/"+"a".repeat(24)+".json.fleet","inference/selections/"+hash+"/ob/"+"a".repeat(64)+".br"];
 for(const name of roles){expect(()=>load.role(name),name).not.toThrow();expect(load.spill(name),name).toBe(false);}
 for(const serial of[0,1,4096])for(const pid of[1,38258,4194304])expect(load.spill("inference/selections/"+hash+"/rl/"+rowLogFileNameV1(pid,Date.now(),serial))).toBe(true);
});

test("no source of the build names a row log file by hand: the one name factory is the only writer of the shape",()=>{
 const found:string[]=[];
 const walk=(directory:string)=>{for(const name of readdirSync(directory)){if(name==="🧪️tests"||name==="node_modules")continue;const path=join(directory,name);let stat;try{stat=statSync(path);}catch{continue;}if(stat.isDirectory())walk(path);else if(/\.(?:ts|mjs)$/u.test(name)&&readFileSync(path,"utf8").includes('"rl-"'))found.push(name);}};
 walk(fileURLToPath(new URL("../../../../../../../../../🏗️build/",origin)));
 expect(found).toEqual(["🟨️.mjs"]);
});
