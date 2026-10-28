/** 🤝️ Contract: a farm worker serves under the sealed record authority of its parent. A parent whose command read workspace files before its first project (every graph child does) seals those reads into its authority, a fresh worker never makes them, so before this law the worker announced other records and was refused (`identity differs: records`, gate run g44). The worker now adopts the rows its parent sends, unless one of its own load rows says something else about the same fact. Oracles: real parent and worker processes, and the identity the parent itself computes. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";

const parent=join(dirname(fileURLToPath(import.meta.url)),"..","🧫️fixtures","🪆️parent-history","🟨️.mjs");
const run=(mode:string)=>{const out=Bun.spawnSync([process.execPath,parent,mode],{stderr:"pipe",env:{...process.env,SEMIO_SCRIPT_PROCESS_INVOCATION:undefined}}),line=out.stdout.toString().trim().split("\n").pop()!;expect(out.exitCode,out.stderr.toString().slice(-2000)).toBe(0);return JSON.parse(line);};

test("a worker is adopted by a parent that read workspace files before its first project, as by one that read only the generator contracts it hands its workers",()=>{
 for(const mode of["history","plain"]){const result=run(mode);expect(result.refused,mode).toEqual([]);expect(result.ready,mode).toBe(true);}
},300000);

test("a worker adopts its parent's authority only when none of its own load rows says something else about the same fact",()=>{
 expect(run("adopt")).toEqual({conflicts:[expect.stringMatching(/^file:/u)],unchanged:true,adopted:[],changed:true,restored:true});
},120000);
