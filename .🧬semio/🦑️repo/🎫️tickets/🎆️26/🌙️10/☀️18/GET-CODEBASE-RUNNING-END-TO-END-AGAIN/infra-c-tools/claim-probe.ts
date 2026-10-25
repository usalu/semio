import {spawn} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
const root=process.cwd(),bazel=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"),port_=await import(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"));
const base=mkdtempSync(join(realpathSync(tmpdir()),"claim-probe-")),directories={leases:join(base,"leases"),server:join(base,"p/server"),workspaces:join(base,"p/workspaces")};
for(const dir of Object.values(directories))mkdirSync(dir,{recursive:true});
const script=join(base,"holder.ts");
writeFileSync(script,`import {bazelSlotPortV1} from ${JSON.stringify(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim("s0",new AbortController().signal);console.log(claim?"held":"busy");if(claim){await new Promise(done=>process.stdin.on("close",done).resume());claim();}`);
const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]});
await new Promise<void>(resolve=>{child.stdout.setEncoding("utf8");child.stdout.on("data",()=>resolve());});
const port=port_.bazelSlotPortV1(directories,{write:()=>undefined}),t0=Date.now();
for(let i=0;i<12;i++){const claim=await port.claim("s0",new AbortController().signal);console.log(`t=${Date.now()-t0}ms claim ${claim?"SUCCEEDED (holder lost it)":"refused"}; holder alive=${child.exitCode===null}`);claim?.();await new Promise(done=>setTimeout(done,500));}
child.kill("SIGKILL");rmSync(base,{recursive:true,force:true});
