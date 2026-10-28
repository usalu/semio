/** usage: bun recreate-run.ts <dry|apply> <graph|products|rust>... : launches recreate-child.ts under an official script envelope (graph-free), modeled on ver-f-tools/proc-run.ts. */
import {spawn} from "node:child_process";
import {createBuildBootstrapCapabilitiesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const root="/Users/ueli/Documents/semio",args=process.argv.slice(2);
const base:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",...process.env,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_MODE:"prototype",SEMIO_BUILD_TOOL:"semio"};
issueRepositoryScriptProvidersV1(base);delete base.SEMIO_SCRIPT_STALL_MS;
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["run","@semio-tech/framework-process:test-testing"],base);
const envelope=createScriptProcessEnvelope({version:1,owner:"store-g:recreate",maximumElapsedMilliseconds:3600000},capabilities,Date.now());
const child=spawn("nice",["-n","10","bun","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/store-g-tools/recreate-child.ts",...args],{cwd:root,env:scriptProcessEnvironment(envelope,base) as NodeJS.ProcessEnv,stdio:["ignore","inherit","inherit"]});
child.on("close",code=>process.exit(code??1));
