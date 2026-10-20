#!/usr/bin/env bun
/** 🚪 Reference forwarding entry: `main.mjs --root R --languages en,de -- <command> <args…>`; exit 75 asks the caller to run in-process. */
import {forwardCommandV1} from "./index.mjs";

const argv=process.argv.slice(2),split=argv.indexOf("--");
const flags=split>=0?argv.slice(0,split):[],rest=split>=0?argv.slice(split+1):argv;
const flag=(name,fallback)=>{const at=flags.indexOf(name);return at>=0?flags[at+1]:fallback;};
const controller=new AbortController();
process.on("SIGINT",()=>controller.abort());
const result=await forwardCommandV1({root:flag("--root",process.cwd()),command:rest[0],args:rest.slice(1),cwd:process.cwd(),languages:flag("--languages","en").split(","),lane:flag("--lane","default"),signal:controller.signal,progress:flags.includes("--progress"),deadlineMs:flag("--deadline")?Number(flag("--deadline")):undefined});
if(!result.handled){process.stderr.write("[fallback] "+result.reason+"\n");process.exitCode=75;}else{
if(flags.includes("--timing"))process.stderr.write(JSON.stringify({served:result.served,service:result.serviceTiming,client:{connectMs:result.timing.connectedAt-result.timing.startedAt,resultMs:result.timing.resultAt-result.timing.startedAt}})+"\n");
process.exitCode=result.exit;}
