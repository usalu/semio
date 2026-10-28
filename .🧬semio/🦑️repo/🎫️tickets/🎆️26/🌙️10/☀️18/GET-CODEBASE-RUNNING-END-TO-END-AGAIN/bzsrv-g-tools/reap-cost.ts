import {mkdirSync,mkdtempSync,rmSync,writeFileSync,realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawn} from "node:child_process";
import {bazelSlotPortV1,reapBazelServersV1,stopBazelServerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🔌️port/🟦️.ts";
const root=mkdtempSync(join(realpathSync(tmpdir()),"reap-cost-")),directories={leases:join(root,"leases"),server:join(root,"server"),workspaces:join(root,"ws")};
const child=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});mkdirSync(join(directories.server,"s5/server"),{recursive:true});writeFileSync(join(directories.server,"s5/server/server.pid.txt"),String(child.pid));
const port=bazelSlotPortV1(directories),signal=new AbortController().signal,plan={slots:2,services:4};
for(const budget of [5,5,5]){const t=performance.now();const stopped=await reapBazelServersV1(directories,port,plan,signal,async()=>undefined,{budget,language:"en"});console.log("reap with one live idle server, budget",budget,"ms",Math.round(performance.now()-t),"stopped",stopped);}
rmSync(join(directories.server,"s5/server/server.pid.txt"));const t=performance.now();await reapBazelServersV1(directories,port,plan,signal,async()=>undefined,{budget:2,language:"en"});console.log("no live server ms",Math.round(performance.now()-t));
child.kill();rmSync(root,{recursive:true,force:true});
