// 🔬 Third-party cross-check of the program derivation: the engine's closure against `tsc -p <tsconfig> --listFilesOnly` (workspace files outside node_modules).
import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {programInputsV1,globMatchesV1,readInputPrecisionV1} from "/private/tmp/ip/run/p/🟦️.ts";
import {programHost} from "./program-host.ts";
import {readFileSync} from "node:fs";
import {realHost,W} from "./real-host.ts";
const tsconfig=process.argv[2]!,out=join(import.meta.dir,"..","🗑️generated","input-precision");
const {host:base}=await realHost(""),document=readInputPrecisionV1(JSON.parse(readFileSync(join(import.meta.dir,"staged2","rules.json"),"utf8"))),host={...base,program:await programHost({document,data:process.env.IP_DATA})};
const t0=performance.now(),program=await programInputsV1(tsconfig,host),engineMs=Math.round(performance.now()-t0);
const declaredPatterns=program.patterns.filter(p=>!p.startsWith("!")).map(p=>p.replace("{workspaceRoot}/","")),negatives=program.patterns.filter(p=>p.startsWith("!")).map(p=>p.slice(1).replace("{workspaceRoot}/",""));
const files=new Set(program.files),covered=(path:string)=>files.has(path)||(declaredPatterns.some(p=>globMatchesV1(p,path))&&!negatives.some(p=>globMatchesV1(p,path)||globMatchesV1(p+"/**",path)));
const t1=performance.now(),run=Bun.spawnSync([process.execPath,join(W,"node_modules/typescript/bin/tsc"),"-p",join(W,tsconfig),"--listFilesOnly"],{cwd:W,maxBuffer:512*1024*1024}),tscMs=Math.round(performance.now()-t1);
const listed=run.stdout.toString().split("\n").filter(Boolean).filter(p=>p.startsWith(W+"/")&&!p.includes("/node_modules/")).map(p=>p.slice(W.length+1));
const missing=listed.filter(p=>!covered(p)),extraRoots=program.roots.filter(p=>!listed.includes(p));
const summary={tsconfig,engineMs,tscMs,tscExit:run.exitCode,tscWorkspaceFiles:listed.length,engineRoots:program.roots.length,engineExtraFiles:program.files.length,missingFromEngine:missing.length,rootsNotInTsc:extraRoots.length,missingSample:missing.slice(0,15),extraRootSample:extraRoots.slice(0,15)};
writeFileSync(join(out,"crosscheck-"+tsconfig.replaceAll("/","_").slice(-40)+".json"),JSON.stringify({summary,missing,extraRoots}));
console.log(JSON.stringify(summary,null,1));
process.exit(0);
