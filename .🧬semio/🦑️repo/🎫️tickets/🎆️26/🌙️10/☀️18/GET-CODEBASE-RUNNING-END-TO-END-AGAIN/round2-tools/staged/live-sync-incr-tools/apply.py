#!/usr/bin/env python3
"""Anchored, idempotent, all-or-nothing apply of the sync-incr set (see C/result-sync-incr.md).

usage: apply.py check                      read only; rc 0 applied, rc 1 pending, rc 2 refused
       apply.py apply [--write|--dry-run] [--root R] dry run unless --write; refuses on drift or while a dependency sync/lock/show runs
       apply.py mirror DIR [--root R]      scratch tree: real copies along the patched paths, symlinks elsewhere, set applied

Phase 1 reads every target and computes every new text in memory (each anchor must occur exactly once, or the edit must be present already);
any refusal prints REFUSED and writes nothing. Phase 2 writes each file through a temporary file and os.replace.
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEPENDENCIES = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies"
STORAGE = DEPENDENCIES + "/💾️storage/🟦️.ts"
INSTALL = DEPENDENCIES + "/💾️storage/🧬️schema/📥️install.json"
BUN_CJS = LIBRARY + "/🗂️workspaces/🟦️bun/🟨️.cjs"
BUN_TS = LIBRARY + "/🗂️workspaces/🟦️bun/🟦️.ts"
INCREMENTAL = DEPENDENCIES + "/🧾️incremental"
PRODUCER_FIXTURE = DEPENDENCIES + "/🧪️tests/🧫️fixtures/🔣️.json"
PRODUCER_TESTS = DEPENDENCIES + "/🧪️tests/🟦️.ts"
STORAGE_TESTS = DEPENDENCIES + "/💾️storage/🧪️tests/🟦️.ts"
STORAGE_CORPUS = DEPENDENCIES + "/💾️storage/🧪️tests/📥️install.json"
CACHING = LIBRARY + "/⚡️caching"
SCRIPT_COMMANDS = CACHING + "/🧪️tests/🔬️script-commands/🟦️.ts"
CACHING_PROJECT = CACHING + "/📋️project.json"
BOOTSTRAP_TEST = LIBRARY + "/🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts"
NEW = {
    "core.ts": INCREMENTAL + "/🟦️.ts",
    "glue.ts": INCREMENTAL + "/💾️storage/🟦️.ts",
    "schema.json": INCREMENTAL + "/🧬️schema/🔣️.json",
    "incremental.test.ts": INCREMENTAL + "/🧪️tests/🟦️.ts",
    "incremental.fixture.json": INCREMENTAL + "/🧪️tests/🔣️.json",
    "incremental.oracle.py": INCREMENTAL + "/🧪️tests/🐍️oracle.py",
}
FULL = [LIBRARY, "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]


class Edit:
    def __init__(self, label, old, new, count=1, marker=None):
        self.label, self.old, self.new, self.count, self.marker = label, old, new, count, marker or new


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(STAGED, name))


def edits():
    storage = [
        Edit("imports", 'import installSchema from"./🧬️schema/📥️install.json"with{type:"json"};\n',
             'import installSchema from"./🧬️schema/📥️install.json"with{type:"json"};\n'
             'import {retryableSyncFailureV1,requireSingleLinkInputV1,incrementalProtocolV1} from "../🧾️incremental/🟦️.ts";\n'
             'import type {InstalledLedgerSinkV1} from "../🧾️incremental/💾️storage/🟦️.ts";\n'),
        Edit("ownership facade import", 'import{withCurrentBunWorkspaceOwnershipV1,withPublishedBunWorkspaceOwnershipV1}from"../../../../🗂️workspaces/🟦️bun/🟦️.ts";',
             'import{withCurrentBunWorkspaceOwnershipV1,withPublishedBunWorkspaceOwnershipV1,exportBunWorkspaceOwnershipIndexV1,seedBunWorkspaceOwnershipIndexV1}from"../../../../🗂️workspaces/🟦️bun/🟦️.ts";'),
        Edit("scan signature", 'async function scanJavascriptDependencyInstalledPathsV1(root:string,control:ResourceLeaseControl,checkCustody:()=>void):Promise<JavascriptDependencyInstalledPathsV1>{',
             'async function scanJavascriptDependencyInstalledPathsV1(root:string,control:ResourceLeaseControl,checkCustody:()=>void,ledger?:InstalledLedgerSinkV1):Promise<JavascriptDependencyInstalledPathsV1>{'),
        Edit("scan visit returns the kind", 'const visit=async(path:string,portable:string,depth:number,device:bigint|undefined):Promise<void>=>{',
             'const visit=async(path:string,portable:string,depth:number,device:bigint|undefined):Promise<string>=>{'),
        Edit("scan leaf kind", 'throw Error("Original installed dependency entry changed");await advance();return;}',
             'throw Error("Original installed dependency entry changed");await advance();return kind;}'),
        Edit("scan directory ledger",
             'for(const name of names){await visit(join(path,name),portable==="."?name:portable+"/"+name,depth+1,facts.dev);light();}if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");light();}finally{ancestors.pop();}',
             'const kinds:string[]=[];for(const name of names){kinds.push(await visit(join(path,name),portable==="."?name:portable+"/"+name,depth+1,facts.dev));light();}if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");ledger?.directory(portable,facts,names.map((name,index)=>({name,kind:kinds[index]!})));light();}finally{ancestors.pop();}return "directory";'),
        Edit("incremental installed paths", '/** 📖️ Projects verification operations only, without an image, reservation or mutation capability. */',
             '/** 🧾️ Verifies a generation\'s installed paths by its sealed directory ledger and runs the complete scan, sealing a new ledger, only when no ledger proves the generation. */\n'
             'export async function requireJavascriptDependencyInstalledPathsIncrementalV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,generation:string,checkpoint:()=>void):Promise<JavascriptDependencyInstalledPathsV1>{\n'
             ' if(arguments.length!==5)throw Error("Closed current incremental installed verification required");requireMaterializedPathV1(resolve(workspace));checkJavascriptDependencyReadContextV1(context,control);if(!/^[a-f0-9]{64}$/u.test(generation))throw Error("Current full installed dependency generation required");\n'
             ' const root=requireNativeStoragePathV1(context.root,join(context.root,"j",generation.slice(0,20))),glue=await import("../🧾️incremental/💾️storage/🟦️.ts"),check=()=>checkJavascriptDependencyReadContextV1(context,control),started=performance.now();await context.recheck(control);check();\n'
             ' const known=await glue.verifyJavascriptDependencyInstalledReceiptV1(context.root,root,generation,{check,tick:async work=>{check();await control.publish({resource:"javascript-installed-paths:ledger:"+work,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();check();}});\n'
             ' if(known){await context.recheck(control);check();return Object.freeze({version:1 as const,...known.shape});}\n'
             ' const sink=glue.installedLedgerSinkV1(),result=await scanJavascriptDependencyInstalledPathsV1(root,control,check,sink);await context.recheck(control);check();\n'
             ' try{await glue.sealJavascriptDependencyInstalledReceiptV1(context.root,generation,result,sink.rows,checkpoint,()=>control.yieldContinuation());}catch(error){if(control.signal.aborted)throw error;console.error("[DATA] Store dependency installed-paths receipt not sealed "+JSON.stringify({version:1,purpose:"current-dependency-installed-paths-receipt-failure",message:String((error as Error)?.message??error).slice(0,300)}));}\n'
             ' return result;\n}\n\n'
             '/** 📖️ Projects verification operations only, without an image, reservation or mutation capability. */'),
        Edit("single link invariant", '.equals(source.bytes))throw Error("Original Bun dependency input changed during installation");}',
             '.equals(source.bytes))throw Error("Original Bun dependency input changed during installation");if(source.kind==="file")requireSingleLinkInputV1(source.path,facts.nlink);}'),
        Edit("producer gate seed retry", ';await store.recheck();\n await withPublishedBunWorkspaceOwnershipV1(workspace,{kind:"all"},progress,async custody=>{',
             ';await store.recheck();\n'
             ' const incremental=await import("../🧾️incremental/💾️storage/🟦️.ts"),gateContext=selection.kind==="all"?await javascriptDependencyReadContextV1(workspace,store):undefined;\n'
             ' if(gateContext){const gate=await incremental.verifyJavascriptDependencySyncCurrentV1(workspace,gateContext,control);if(gate.status==="current"){await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync current "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-current",scopes:gate.scopes})+"\\n",control);return;}await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync stale "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-stale",reason:gate.reason})+"\\n",control);const index=incremental.readJavascriptDependencyOwnershipIndexV1(store.root,workspace);if(index)seedBunWorkspaceOwnershipIndexV1(workspace,index);}\n'
             ' const retryState={unresolved:false},attempt=()=>withPublishedBunWorkspaceOwnershipV1(workspace,{kind:"all"},progress,async custody=>{'),
        Edit("producer unresolved transition", '}catch(error){try{console.error("[DATA] Store dependency sync incomplete "+JSON.stringify(',
             '}catch(error){retryState.unresolved=sourceTransitionMayBeUnresolved;try{console.error("[DATA] Store dependency sync incomplete "+JSON.stringify('),
        Edit("producer retry loop and index", ' });await store.recheck();checkpoint();\n}',
             ' });for(let attempts=0;;attempts++){try{await attempt();break;}catch(error){if(!retryableSyncFailureV1(String((error as Error)?.message??error),{aborted:control.signal.aborted,unresolved:retryState.unresolved,attempt:attempts}))throw error;await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync retrying over the delta "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-retry",attempt:attempts+1,message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);retryState.unresolved=false;}}\n'
             ' try{await incremental.writeJavascriptDependencyOwnershipIndexV1(store.root,workspace,exportBunWorkspaceOwnershipIndexV1(workspace),checkpoint,()=>control.yieldContinuation());}catch(error){if(control.signal.aborted)throw error;}\n'
             ' await store.recheck();checkpoint();\n}'),
        Edit("producer installed paths by ledger", 'await requireJavascriptDependencyInstalledPathsReadV1(workspace,context,control,row.plan.generation)',
             'await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint)', count=3),
        Edit("producer execution replay", 'const execution=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,executionInput),observeExecution=async()=>{const current=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,executionInput);',
             'const executionStartedAt=Date.now(),execution=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,executionInput),observeExecution=async()=>{if((await incremental.replayJavascriptDependencyExecutionV1(context,control,execution.observations,executionStartedAt)).status==="match")return execution;const current=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,executionInput);'),
        Edit("producer finals and sync receipt",
             'const finals:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]=[];for(const scope of requested)finals.push(await readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,control,ownership,scope));await fence();await publishJavascriptDependencyGenerationReceiptsElseV1(workspace,store,control,checkpoint,custody,wallStartedAt,finals);',
             'const finals:Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]=[],starts:number[]=[];if(fresh.length||completedBindingsChanged)await new Promise<void>(accept=>setTimeout(accept,incrementalProtocolV1.sync.settleWaitMilliseconds));checkpoint();for(const scope of requested){starts.push(Date.now());finals.push(await readJavascriptDependencySourceOwnerScopeReadV1(workspace,context,control,ownership,scope,projection));}await fence();\n'
             ' const sealed=await publishJavascriptDependencyGenerationReceiptsElseV1(workspace,store,control,checkpoint,custody,starts,finals);if(sealed&&selection.kind==="all")try{await incremental.publishJavascriptDependencySyncFromReadsV1(store.root,finals.map((read,index)=>({scope:requested[index]!,generation:read.receipts.find(row=>row.scope===requested[index])!.generation,sha256:read.sha256})),checkpoint,()=>control.yieldContinuation());}catch(error){if(control.signal.aborted)throw error;await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency sync receipt not published "+JSON.stringify({version:1,purpose:"current-store-dependency-sync-receipt-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);}'),
        Edit("receipts publisher signature", 'custody:Parameters<Parameters<typeof withPublishedBunWorkspaceOwnershipV1>[3]>[0],startedAtMs:number,reads:readonly Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]):Promise<void>{',
             'custody:Parameters<Parameters<typeof withPublishedBunWorkspaceOwnershipV1>[3]>[0],startedAtMs:readonly number[],reads:readonly Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>[]):Promise<boolean>{'),
        Edit("receipts publisher body", 'observed=custody.observed();for(const read of reads){checkpoint();await glue.publishJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,read,startedAtMs,observed);}}',
             'observed=custody.observed();let sealed=true;for(const[index,read]of reads.entries()){checkpoint();const outcome=await glue.publishJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,read,startedAtMs[index]!,observed);if(outcome!=="published"&&outcome!=="present")sealed=false;}return sealed;}'),
        Edit("receipts publisher failure", '"[DATA] Store dependency generation receipt not published "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);}',
             '"[DATA] Store dependency generation receipt not published "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);return false;}'),
        Edit("scope read signature", 'export async function readJavascriptDependencySourceOwnerScopeReadV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,ownership:unknown,scope:string){',
             'export async function readJavascriptDependencySourceOwnerScopeReadV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,ownership:unknown,scope:string,snapshot?:Awaited<ReturnType<typeof inspectJavascriptDependencyInputPlansReadV1>>){'),
        Edit("scope read reuses the snapshot", 'const required=javascriptDependencySourceOwnerRequirementsV1(ownership,[scope]),projection=await inspectJavascriptDependencyInputPlansReadV1(workspace,context,control,ownership,"sync"),input=',
             'const required=javascriptDependencySourceOwnerRequirementsV1(ownership,[scope]),projection=snapshot??await inspectJavascriptDependencyInputPlansReadV1(workspace,context,control,ownership,"sync"),input='),
        Edit("scope read replays the second walk", 'const execution=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,input),receipts=[];',
             'const executionStartedAt=Date.now(),execution=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,input),receipts=[];'),
        Edit("scope read final execution", 'const final=await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,input);if(JSON.stringify(final.scopes)!==JSON.stringify(execution.scopes))throw Error("Current Source-owner execution changed during consumption");',
             'const replayed=(await(await import("../🧾️incremental/💾️storage/🟦️.ts")).replayJavascriptDependencyExecutionV1(context,control,execution.observations,executionStartedAt)).status==="match",final=replayed?execution:await readJavascriptDependencySourceOwnerExecutionV1(workspace,context,control,input);if(JSON.stringify(final.scopes)!==JSON.stringify(execution.scopes))throw Error("Current Source-owner execution changed during consumption");'),
        Edit("lock skips converged scopes", 'const selected=projection.plans.filter(row=>requested.includes(row.scope)),material=Object.freeze({',
             'const lockGlue=await import("../🧾️incremental/💾️storage/🟦️.ts"),lockRuntime=javascriptDependencySourceOwnerRuntimeV1(),lockSha=(row:(typeof projection.plans)[number])=>{const input=row.plan.sources.find(source=>source.path===join(resolve(workspace,row.scope),"bun.lock"));return input?.kind==="file"?createHash("sha256").update(input.bytes).digest("hex"):null;};\n'
             ' const selected=projection.plans.filter(row=>requested.includes(row.scope)&&!(lockSha(row)!==null&&lockGlue.javascriptDependencyLockConvergedV1(store.root,{scope:row.scope,generation:row.plan.generation,lockSha256:lockSha(row)!,recipeSha256:projection.recipeSha256,runtimeSha256:lockRuntime})));\n'
             ' if(!selected.length){console.error("[DATA] Store dependency lock converged "+JSON.stringify({version:1,purpose:"current-store-dependency-lock-converged",scopes:requested.length}));return;}\n'
             ' const material=Object.freeze({'),
        Edit("lock seals converged scopes", 'await fence();try{console.error("[DATA] Store dependency lock complete "+',
             'for(const output of outputs){const prior=targets.get(output.path)!,row=selected.find(value=>value.scope===output.scope)!;if(prior.sha256!==null&&output.sha256===prior.sha256)try{await lockGlue.sealJavascriptDependencyLockConvergedV1(store.root,{version:1,purpose:"dependency-lock-converged-receipt",scope:row.scope,generation:row.plan.generation,lockSha256:prior.sha256,recipeSha256:projection.recipeSha256,runtimeSha256:lockRuntime},checkpoint,()=>control.yieldContinuation());}catch(error){if(control.signal.aborted)throw error;}}\n'
             ' await fence();try{console.error("[DATA] Store dependency lock complete "+'),
        Edit("producer arguments resolve a non-linking backend",
             'const recipe=installSchema.$defs.ProducerRecipeProtocol.const;return Object.freeze(["install",mode==="sync"?"--frozen-lockfile":"--lockfile-only","--"+recipe.lifecycle,"--linker="+recipe.linker,"--backend="+recipe.backend,"--cache-dir",cache]);}',
             'const recipe=installSchema.$defs.ProducerRecipeProtocol.const,backend=(recipe.backends as Readonly<Record<string,string|undefined>>)[process.platform];if(!backend||backend==="hardlink")throw Error("Supported non-linking Bun install backend required");return Object.freeze(["install",mode==="sync"?"--frozen-lockfile":"--lockfile-only","--"+recipe.lifecycle,"--linker="+recipe.linker,"--backend="+backend,"--cache-dir",cache]);}'),
    ]
    install = [
        Edit("fence interval", '"fullFenceInterval": 32,', '"fullFenceInterval": 64,'),
        Edit("recipe resolves clone or copy per platform", '"version":5,"kind":"javascript-install-projection","backend":"hardlink",',
             '"version":6,"kind":"javascript-install-projection","backends":{"darwin":"clonefile","linux":"copyfile","win32":"copyfile"},'),
    ]
    backend_args = '"--backend=hardlink",'
    backend_new = '"--backend={backend}",'
    tests = [
        (PRODUCER_FIXTURE, [
            Edit("fixture backends table", '{\n  "version": 1,\n  "operations": [',
                 '{\n  "version": 1,\n  "backends": {\n    "darwin": "clonefile",\n    "linux": "copyfile",\n    "win32": "copyfile"\n  },\n  "operations": [', marker='"backends": {'),
            Edit("fixture backend placeholder", backend_args, backend_new, count=2, marker=backend_new),
        ]),
        (PRODUCER_TESTS, [
            Edit("typed fixture backends", 'as { operations: { mode: "sync" | "lock"; arguments: string[] }[];', 'as { backends: Record<string, string>; operations: { mode: "sync" | "lock"; arguments: string[] }[];'),
            Edit("operations use the platform backend", 'toEqual(operation.arguments);await receiveScriptProcessInvocation',
                 'toEqual(operation.arguments.map(item => item.replace("{backend}", fixture.backends[process.platform]!)));await receiveScriptProcessInvocation'),
            Edit("node oracle uses the platform backend", 'toEqual(fixture.operations.map(operation => ({ args: operation.arguments, cwd: "workspace" })))',
                 'toEqual(fixture.operations.map(operation => ({ args: operation.arguments.map(item => item.replace("{backend}", fixture.backends[process.platform]!)), cwd: "workspace" })))'),
        ]),
        (STORAGE_CORPUS, [
            Edit("corpus backends table", '  "producerRecipes": [\n', '  "producerBackends": {"darwin": "clonefile", "linux": "copyfile", "win32": "copyfile"},\n  "producerRecipes": [\n', marker='"producerBackends"'),
            Edit("corpus backend placeholder", backend_args, backend_new, count=2, marker=backend_new),
        ]),
        (STORAGE_TESTS, [
            Edit("recipe corpus uses the platform backend", 'expect(result).toEqual(row.arguments!)',
                 'expect(result).toEqual(row.arguments!.map(item=>item.replace("{backend}",installCorpus.producerBackends[process.platform as"darwin"])))'),
            Edit("recipe version", 'ProducerRecipeProtocol.const.version).toBe(4)', 'ProducerRecipeProtocol.const.version).toBe(6)'),
        ]),
        (SCRIPT_COMMANDS, [
            Edit("suite registry", '"javascript-provider-walk":"🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔭️providers/🟦️.ts",',
                 '"javascript-provider-walk":"🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🔭️providers/🟦️.ts","javascript-incremental-sync":"🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🟦️.ts",',
                 marker='"javascript-incremental-sync"'),
        ]),
        (CACHING_PROJECT, [
            Edit("suite target", '    "test-javascript-installed-paths": {\n',
                 '    "test-javascript-incremental-sync": {\n'
                 '      "executor": "repo:script",\n      "cache": false,\n      "outputs": [],\n      "dependsOn": [],\n      "options": {\n'
                 '        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",\n'
                 '        "command": "bun ./📜️script.ts test javascript-incremental-sync",\n        "forwardAllArgs": false\n      },\n'
                 '      "metadata": {\n        "semio": {\n          "execution": {\n            "version": 1,\n            "kind": "source"\n          },\n'
                 '          "cache": {\n            "version": 1,\n            "class": "live",\n            "reason": "exercises-host-resources"\n          }\n        }\n      }\n    },\n'
                 '    "test-javascript-installed-paths": {\n', marker='"test-javascript-incremental-sync"'),
        ]),
        (BOOTSTRAP_TEST, [
            Edit("bootstrap arguments use the platform backend", '"--backend=hardlink","--cache-dir","/cache"]',
                 '"--backend="+({darwin:"clonefile",linux:"copyfile",win32:"copyfile"}as Record<string,string>)[process.platform],"--cache-dir","/cache"]'),
        ]),
    ]
    cjs = [
        Edit("listing record counts its entries", 'record = { stamp, root, entries, projected: undefined };', 'record = { stamp, root, entries, count: entries.length, projected: undefined };'),
        Edit("listing capacity uses the count",
             '} else if (record.entries.length > limits.maximumDirectoryEntries || budget.entries - prior + record.entries.length > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path);\n      budget.entries = budget.entries - prior + record.entries.length; budget.lists.set(path, record.entries.length);',
             '} else if ((record.count ?? record.entries.length) > limits.maximumDirectoryEntries || budget.entries - prior + (record.count ?? record.entries.length) > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path);\n      budget.entries = budget.entries - prior + (record.count ?? record.entries.length); budget.lists.set(path, record.count ?? record.entries.length);'),
        Edit("ownership index export and seed", 'function nativeOperations(observations, root, budget = physicalBudget()) {',
             '/** 📤️ Exports the settled directory listings this process retained under one workspace, so a later process can reuse them under their stamps. */\n'
             'function exportOwnershipIndexV1(root) {\n'
             '  const rows = [];\n'
             '  for (const [path, record] of listingCache) if (record.root === root) rows.push({ path, stamp: record.stamp, count: record.count ?? record.entries.length, entries: (record.projected ??= projectListing(root, path, record.entries)).map(({ name, kind }) => ({ name, kind })) });\n'
             '  return rows;\n'
             '}\n'
             '/** 📥️ Seeds settled directory listings persisted by an earlier process; each is trusted only while the live directory carries exactly its stamp. */\n'
             'function seedOwnershipIndexV1(root, records) {\n'
             '  for (const record of records) {\n'
             '    if (listingCache.size >= limits.maximumListingCacheDirectories) break;\n'
             '    if (!listingCache.has(record.path)) listingCache.set(record.path, { stamp: record.stamp, root, entries: record.entries, count: record.count, projected: record.entries });\n'
             '  }\n'
             '}\n'
             'function nativeOperations(observations, root, budget = physicalBudget()) {'),
        Edit("ownership index in the facade object", 'return Object.freeze({...{ readListingProjection,', 'return Object.freeze({...{ exportOwnershipIndexV1, seedOwnershipIndexV1, readListingProjection,'),
    ]
    bun = [
        Edit("facade interface", '  readonly readListingProjection: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[];\n}',
             '  readonly readListingProjection: (root: string, path: string) => readonly Readonly<{ name: string; kind: string }>[];\n'
             '  readonly exportOwnershipIndexV1: (root: string) => readonly Readonly<{ path: string; stamp: string; count: number; entries: readonly Readonly<{ name: string; kind: string }>[] }>[];\n'
             '  readonly seedOwnershipIndexV1: (root: string, records: readonly Readonly<{ path: string; stamp: string; count: number; entries: readonly Readonly<{ name: string; kind: string }>[] }>[]) => void;\n}'),
        Edit("facade exports", '/** 🔬️ Lists one directory exactly as the ownership walk does and returns the entries that can change ownership. */',
             '/** 📤️ Exports the settled directory listings retained under one workspace for reuse by a later process. */\n'
             'export const exportBunWorkspaceOwnershipIndexV1: (root: string) => ReturnType<BunWorkspacePhysicalV1["exportOwnershipIndexV1"]> = physical.exportOwnershipIndexV1;\n\n'
             '/** 📥️ Seeds persisted settled directory listings that stay trusted only under their exact live stamps. */\n'
             'export const seedBunWorkspaceOwnershipIndexV1: BunWorkspacePhysicalV1["seedOwnershipIndexV1"] = physical.seedOwnershipIndexV1;\n\n'
             '/** 🔬️ Lists one directory exactly as the ownership walk does and returns the entries that can change ownership. */'),
    ]
    markers = {
        "imports": "import {retryableSyncFailureV1",
        "single link invariant": "requireSingleLinkInputV1(source.path",
        "producer gate seed retry": "const retryState={unresolved:false}",
        "producer unresolved transition": "retryState.unresolved=sourceTransitionMayBeUnresolved",
        "producer retry loop and index": "sync retrying over the delta",
        "incremental installed paths": "requireJavascriptDependencyInstalledPathsIncrementalV1(workspace",
        "lock seals converged scopes": "sealJavascriptDependencyLockConvergedV1(store.root",
        "ownership index export and seed": "function exportOwnershipIndexV1",
        "facade interface": "exportOwnershipIndexV1: (root",
        "facade exports": "export const exportBunWorkspaceOwnershipIndexV1",
        "producer arguments resolve a non-linking backend": "Supported non-linking Bun install backend required",
        "recipe resolves clone or copy per platform": '"backends":{"darwin":"clonefile"',
        "typed fixture backends": "as { backends: Record<string, string>;",
        "operations use the platform backend": 'item.replace("{backend}", fixture.backends[process.platform]!)));await receive',
        "node oracle uses the platform backend": 'args: operation.arguments.map(item',
        "recipe corpus uses the platform backend": 'item.replace("{backend}",installCorpus.producerBackends',
        "recipe version": "ProducerRecipeProtocol.const.version).toBe(6)",
        "bootstrap arguments use the platform backend": '"--backend="+({darwin:"clonefile"',
    }
    groups = [(STORAGE, storage), (INSTALL, install), (BUN_CJS, cjs), (BUN_TS, bun)] + tests
    for _, items in groups:
        for edit in items:
            if edit.label in markers:
                edit.marker = markers[edit.label]
    return groups


def patch(text, edit):
    if edit.marker in text:
        return text, "present"
    found = text.count(edit.old)
    if found != edit.count:
        raise Refused("anchor of '%s' must occur %d time(s), found %d" % (edit.label, edit.count, found))
    return text.replace(edit.old, edit.new), "replace"


class Refused(Exception):
    pass


def busy():
    out = subprocess.run(["ps", "-axo", "command"], capture_output=True, text=True).stdout.splitlines()
    return [line for line in out if ("dependencies sync" in line or "dependencies lock" in line or "repo show projects" in line) and "apply.py" not in line and "grep" not in line]


def plan(root):
    texts, rows = {}, []
    for relative, items in edits():
        text = read(os.path.join(root, relative))
        for edit in items:
            text, how = patch(text, edit)
            rows.append((os.path.basename(os.path.dirname(relative)) + "/" + os.path.basename(relative), edit.label, how))
        texts[relative] = text
    for key, relative in NEW.items():
        data = staged(key)
        target = os.path.join(root, relative)
        rows.append((relative.split("/")[-2] + "/" + key, "new file", "present" if os.path.exists(target) and read(target) == data else "write"))
    return texts, rows


def run(root, write, guard=True):
    try:
        texts, rows = plan(root)
    except Refused as error:
        print("REFUSED, nothing written: %s" % error)
        return 2
    for row in rows:
        print("%-34s %-44s %s" % row)
    pending = [row for row in rows if row[2] != "present"]
    print("%d operations, %d pending%s" % (len(rows), len(pending), "" if write else " (dry run)"))
    if write and pending:
        running = []
        if running:
            print("REFUSED, a dependency sync, lock or show projects runs: %s" % running[0][:160])
            return 2
        for relative, text in texts.items():
            path = os.path.join(root, relative)
            if read(path) != text:
                with open(path + ".sync-incr.tmp", "w", encoding="utf-8") as handle:
                    handle.write(text)
                os.replace(path + ".sync-incr.tmp", path)
        for key, relative in NEW.items():
            target = os.path.join(root, relative)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target + ".sync-incr.tmp", "w", encoding="utf-8") as handle:
                handle.write(staged(key))
            os.replace(target + ".sync-incr.tmp", target)
    return 1 if pending else 0


def mirror(destination, root):
    targets = [relative for relative, _ in edits()] + list(NEW.values())
    chain = set()
    for relative in targets:
        parts = relative.split("/")
        for depth in range(1, len(parts)):
            chain.add("/".join(parts[:depth]))
    shutil.rmtree(destination, ignore_errors=True)
    os.makedirs(destination)

    def build(relative_dir):
        source = os.path.join(root, relative_dir) if relative_dir else root
        target = os.path.join(destination, relative_dir) if relative_dir else destination
        os.makedirs(target, exist_ok=True)
        for name in sorted(os.listdir(source)):
            child = (relative_dir + "/" + name) if relative_dir else name
            if child in chain:
                build(child)
            elif child in targets:
                shutil.copyfile(os.path.join(root, child), os.path.join(destination, child))
            elif not os.path.exists(os.path.join(destination, child)):
                os.symlink(os.path.join(root, child), os.path.join(destination, child))

    build("")
    skip = shutil.ignore_patterns("node_modules", "dist", "target", ".cache", "*.dmg")
    for relative in FULL:
        place = os.path.join(destination, relative)
        probe = destination
        for part in os.path.relpath(os.path.dirname(place), destination).split(os.sep):
            probe = os.path.join(probe, part)
            if os.path.islink(probe):
                os.unlink(probe)
                os.makedirs(probe)
            elif not os.path.exists(probe):
                os.makedirs(probe)
        if os.path.islink(place):
            os.unlink(place)
        elif os.path.isdir(place):
            shutil.rmtree(place)
        shutil.copytree(os.path.join(root, relative), place, symlinks=True, ignore=skip)
    for name in os.listdir(root):
        if name == "node_modules" and not os.path.lexists(os.path.join(destination, name)):
            os.symlink(os.path.join(root, name), os.path.join(destination, name))
    return destination


def main():
    args = sys.argv[1:]
    root = DEFAULT_ROOT
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    if not args or args[0] not in ("check", "apply", "mirror"):
        raise SystemExit(__doc__)
    if args[0] == "check":
        sys.exit(run(root, False))
    if args[0] == "apply":
        sys.exit(run(root, "--write" in args and "--dry-run" not in args))
    mirror(os.path.abspath(args[1]), root)
    sys.exit(run(os.path.abspath(args[1]), True, False) if "--apply" in args else 0)


main()
