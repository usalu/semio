#!/usr/bin/env bun
/** ▶ Starts a resident service over a prototype workspace: `serve.mjs <root> [--backend auto|native|portable] [--no-scan]`. */
import {createResidentServiceV1} from "../resident/service/index.mjs";
import {endpointsV1} from "../resident/protocol/endpoint.mjs";
import {createCustodyPort,createExecutionPort,createGraphPort} from "./ports.mjs";
import {realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {nativeDirectoryV1,nativeToolV1} from "./environment.mjs";

const[,, rawRoot,...flags]=process.argv,root=realpathSync(rawRoot);
const flag=(name,fallback)=>{const at=flags.indexOf(name);return at>=0?flags[at+1]:fallback;};
const bin=nativeDirectoryV1(flag("--bin")),watcher=nativeToolV1(bin,"semio-repo-watch"),stamps=process.env.STAMPS??join(tmpdir(),"semio-bench-stamps");
const runtime=endpointsV1(root);
const policyOverride=JSON.parse(flag("--policy","{}"));
const service=createResidentServiceV1({
 root,runtime,implementation:"0".repeat(64),
 policy:policyOverride,
 backend:flag("--backend","auto"),watcherBinary:flag("--backend","auto")==="portable"?undefined:watcher,
 exclude:{rooted:[".git",".cache"],names:["node_modules","target"]},fenceDirectory:".semio-fence",
 ports:{graph:createGraphPort({root,scanBinary:flags.includes("--js-scan")?undefined:watcher}),execution:createExecutionPort({tool:nativeToolV1(bin,"faketool"),stampDirectory:stamps,workMs:Number(flag("--work","30"))}),custody:createCustodyPort({acquireMs:Number(flag("--acquire","0"))})},
 log:(kind,detail)=>{if(flags.includes("--verbose"))process.stderr.write("[service] "+kind+" "+detail+"\n");},
});
process.on("SIGTERM",()=>void service.stop("SIGTERM"));
process.on("SIGINT",()=>void service.stop("SIGINT"));
const started=await service.start();
process.stdout.write(JSON.stringify({ready:true,...started,runtime})+"\n");
await service.closed;
process.exit(0);
