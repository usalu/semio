import {runRemoteExecutionCommandV1} from "../⌨️command/🟦️.ts";

/** 🏃️ Runs `remote-execution workers` as a process of its own, the way a second machine, a devcontainer or the local workers of `serve` run it: the arguments after the script are the command's, SIGTERM and SIGINT cancel and let the worker leave the scheduler in order. */
const controller=new AbortController();
for(const signal of["SIGTERM","SIGINT"] as const)process.on(signal,()=>{controller.abort();});
const result=await runRemoteExecutionCommandV1(["workers",...process.argv.slice(2)],{root:process.cwd(),environment:process.env,signal:controller.signal,progress:async()=>{},write:line=>{console.log(line);},spawnWorker:()=>{throw Error("A worker does not start workers");}});
process.exit(result.exitCode);
