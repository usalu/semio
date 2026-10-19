#!/bin/zsh
# deferred production edits (build CLI source-guarded set): apply when no `bun repo` process is running
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
V="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts"
python3 $C/repot-tools/ed.py "$V" <<'X'
OLD<<<
projection=projectViteCommandEnvironmentV1({version:1,mode,platform:process.platform,layers:[{...process.env},{...options.environment}]}),environment=projection.environment;
NEW<<<
projection=projectViteCommandEnvironmentV1({version:1,mode,platform:process.platform,layers:[{...process.env},{...options.environment}]}),environment=projection.environment,configLoader=readViteConfigLoaderV1(options.configLoader);
X
python3 $C/repot-tools/ed.py "$V" <<'X'
OLD<<<
"--configLoader",readViteConfigLoaderV1(options.configLoader),"--host",host,"--port",String(port),...(strictPort?["--strictPort"]:[]),"--clearScreen","false"],root,owner,compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,{...child.environment,...viteRuntimeEnvironmentV1(runtimeChannels)}),signal:control.signal,output:authority.transport
NEW<<<
"--configLoader",configLoader,"--host",host,"--port",String(port),...(strictPort?["--strictPort"]:[]),"--clearScreen","false"],root,owner,compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,{...child.environment,...viteRuntimeEnvironmentV1(runtimeChannels)}),signal:control.signal,output:authority.transport
X
P="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/📦️transaction/🟦️.ts"
python3 $C/repot-tools/ed.py "$P" <<'X'
OLD<<<
function removeNext(path:string,row:Row){if(!existsSync(path))return;const stat=regular(path,policy.maximumFileBytes);if(stat.size!==BigInt(row.next.bytes)||
NEW<<<
function removeNext(path:string,row:Row){if(!existsSync(path))return;const stat=regular(path,policy.maximumFileBytes);if(row.next.identity===null&&stat.size<=BigInt(row.next.bytes)){unlinkSync(path);return;}if(stat.size!==BigInt(row.next.bytes)||
X
LEASES="🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts"
python3 $C/repot-tools/ed.py "$LEASES" <<'X'
OLD<<<
await delay(Math.max(1, Math.min(40, remaining(options, started,cleanup))), undefined, { signal: options.signal });
NEW<<<
await delay(Math.max(1, Math.min(40, remaining(options, started,cleanup))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
X
python3 $C/repot-tools/ed.py "$LEASES" <<'X'
OLD<<<
await delay(Math.max(1, Math.min(250, remaining(options, started))), undefined, { signal: options.signal });
NEW<<<
await delay(Math.max(1, Math.min(250, remaining(options, started))), undefined, { signal: options.signal }).catch(error=>{options.signal.throwIfAborted();throw error;});
X
