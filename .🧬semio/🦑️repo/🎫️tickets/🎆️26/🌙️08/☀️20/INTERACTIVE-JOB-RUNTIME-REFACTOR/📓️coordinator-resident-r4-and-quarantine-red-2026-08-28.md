# Resident R4 Replay and Independent Fault-Quarantine RED

## Executed Results

The coordinator ran the canonical value-resident target against the R27 source. Nx exited 0; the full printed cohort below includes all five mandatory-cell wrappers, eight finalizer frontiers, exact terminal alias detachment, and strict component TypeScript 0. Ten selected source/schema/fixture/router hashes were identical before and after. This verifies the executed cohort, not all forward-operation quarantine or live Shard/UI integration.

A separate independent runtime probe then admitted real pages/external storage through their original cells and handed an exact fault into each cell. All four resources still accepted forward work: allocate=ready, write=pending, seal=ready, beginReceive=pending. The expected closed contract is rejection after the resource's own cell is quarantined. The same original fault remained held in all four cases. This is an actual semantic RED (Nx exit 1), not missing API or orchestration failure. Immer supplies the independent quarantined-state expectation. No production source was changed. The ten selected inputs remained stable through this probe too.

The observed cause is that page allocate/write/seal and external beginReceive consult the parent owner but omit the resource's own admission cell state. The assigned follow-up must preserve retirement custody for an already-posted external backing: a late reply may still need to transfer into retirement ownership, but cannot issue a live custody receipt from a faulted source. Reader admission/read laws and exact closed-cell reuse also need coverage before general quarantine acceptance.

## Source Review Boundary

The coordinator read all 422 lines of the current neutral component and the complete R27 API report. The report's claim that all intrinsic parent drivers use private closures is broader than current source: Admission-to-resource and owner-to-child still dispatch through public facade methods. The UI owner acknowledged this distinction. Retained exact-state dispatch and hostile-wrapper laws are assigned without widening public authority or weakening current tests. Unbounded external exception retirement remains explicitly open.

## Canonical Command and Complete Tool Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/value-resident:test --skip-nx-cache
```

```text

> nx run @semio-tech/value-resident:test

> bun ./📜️script.ts test

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] Resident capacity=6 actualOverflow=2 ownerReader=1 partialExtent=4 simultaneousRawUiScratch=1 postedCancel=1 unsubmittedCancel=1 transferredViewFault=1 controlAxes=3 childClose=5 domainRecord=1 recordOverflow=3 finalizerFrontiers=8 admissionFailures=5 admissionBootstrap=7 firstFault=4 resourceWrapper=5 terminalAliasDetach=1 strictTS=0 oracle=Ajv+Immer+Buffer+BigInt



 NX   Successfully ran target test for project @semio-tech/value-resident



```

## Independent RED Command and Complete Tool Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'import assert from '\''node:assert/strict'\'';import {produce} from '\''immer'\'';import {OwnedResidentLedger} from '\''./🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts'\'';const grant={maxItems:1,maxBytes:4096};const ledger=new OwnedResidentLedger({bytes:100000,slots:1000,owners:1000,control:{bytes:0,slots:0,owners:0}});function cell(){const consumer=Object.freeze({});assert.equal(ledger.prepareAdmission(consumer,'\''data'\'',grant).kind,'\''pending'\'');const c=ledger.preparedAdmission(consumer);assert(c);assert.equal(ledger.claimAdmission(consumer,c,grant).kind,'\''ready'\'');return c;}const ownerCell=cell();const owner=ledger.beginOwner('\''data'\'',ownerCell,grant).owner;assert(owner);const rows=[];for(const kind of ['\''allocate'\'','\''write'\'','\''seal'\'','\''beginReceive'\'']){const c=cell();const resource=kind==='\''beginReceive'\''?owner.reserveExternalBacking(32,c,grant).slot:owner.reservePage(kind==='\''seal'\''?0:1,c,grant).page;assert(resource);if(kind==='\''write'\''||kind==='\''seal'\'')assert.equal(resource.allocate(grant).kind,'\''ready'\'');const fault=Object.freeze({original:kind});assert.equal(c.retainFailure(fault,grant).kind,'\''pending'\'');const actual=kind==='\''allocate'\''?resource.allocate(grant):kind==='\''write'\''?resource.writeByte(73,grant):kind==='\''seal'\''?resource.seal(grant):resource.beginReceive(grant);const oracle=produce({forwardAllowed:true},draft=>{draft.forwardAllowed=false;});rows.push({kind,outcome:actual.kind,expected:oracle.forwardAllowed?'\''ready'\'':'\''rejected'\'',sameFault:c.failure===fault});}console.log('\''[DEBUG] admission own-cell fault quarantine '\''+JSON.stringify(rows));assert.deepEqual(rows.map(x=>x.outcome),rows.map(x=>x.expected));'
```

```text
[DEBUG] admission own-cell fault quarantine [{"kind":"allocate","outcome":"ready","expected":"rejected","sameFault":true},{"kind":"write","outcome":"pending","expected":"rejected","sameFault":true},{"kind":"seal","outcome":"ready","expected":"rejected","sameFault":true},{"kind":"beginReceive","outcome":"pending","expected":"rejected","sameFault":true}]
1 | import assert from 'node:assert/strict';import {produce} from 'immer';import {OwnedResidentLedger} from './🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts';const grant={maxItems:1,maxBytes:4096};const ledger=new OwnedResidentLedger({bytes:100000,slots:1000,owners:1000,control:{bytes:0,slots:0,owners:0}});function cell(){const consumer=Object.freeze({});assert.equal(ledger.prepareAdmission(consumer,'data',grant).kind,'pending');const c=ledger.preparedAdmission(consumer);assert(c);assert.equal(ledger.claimAdmission(consumer,c,grant).kind,'ready');return c;}const ownerCell=cell();const owner=ledger.beginOwner('data',ownerCell,grant).owner;assert(owner);const rows=[];for(const kind of ['allocate','write','seal','beginReceive']){const c=cell();const resource=kind==='beginReceive'?owner.reserveExternalBacking(32,c,grant).slot:owner.reservePage(kind==='seal'?0:1,c,grant).page;assert(resource);if(kind==='write'||kind==='seal')assert.equal(resource.allocate(grant).kind,'ready');const f

AssertionError: Expected values to be strictly deep-equal:
+ actual - expected

  [
+   'ready',
+   'pending',
+   'ready',
+   'pending'
-   'rejected',
-   'rejected',
-   'rejected',
-   'rejected'
  ]

 generatedMessage: true,
     actual: [ "ready", "pending", "ready", "pending" ],
   expected: [ "rejected", "rejected", "rejected", "rejected" ],
   operator: "deepStrictEqual",
       code: "ERR_ASSERTION"

      at /Users/ueli/Documents/semio/[eval]:1:1548

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "import assert from 'node:assert/strict';import {produce} from 'immer';import {OwnedResidentLedger} from './🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts';const grant={maxItems:1,maxBytes:4096};const ledger=new OwnedResidentLedger({bytes:100000,slots:1000,owners:1000,control:{bytes:0,slots:0,owners:0}});function cell(){const consumer=Object.freeze({});assert.equal(ledger.prepareAdmission(consumer,'data',grant).kind,'pending');const c=ledger.preparedAdmission(consumer);assert(c);assert.equal(ledger.claimAdmission(consumer,c,grant).kind,'ready');return c;}const ownerCell=cell();const owner=ledger.beginOwner('data',ownerCell,grant).owner;assert(owner);const rows=[];for(const kind of ['allocate','write','seal','beginReceive']){const c=cell();const resource=kind==='beginReceive'?owner.reserveExternalBacking(32,c,grant).slot:owner.reservePage(kind==='seal'?0:1,c,grant).page;assert(resource);if(kind==='write'||kind==='seal')assert.equal(resource.allocate(grant).kind,'ready');const fault=Object.freeze({original:kind});assert.equal(c.retainFailure(fault,grant).kind,'pending');const actual=kind==='allocate'?resource.allocate(grant):kind==='write'?resource.writeByte(73,grant):kind==='seal'?resource.seal(grant):resource.beginReceive(grant);const oracle=produce({forwardAllowed:true},draft=>{draft.forwardAllowed=false;});rows.push({kind,outcome:actual.kind,expected:oracle.forwardAllowed?'ready':'rejected',sameFault:c.failure===fault});}console.log('[DEBUG] admission own-cell fault quarantine '+JSON.stringify(rows));assert.deepEqual(rows.map(x=>x.outcome),rows.map(x=>x.expected));"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 1,
  signal: null,
  output: [ null, null, null ],
  pid: 59341,
  stdout: null,
  stderr: null
}

```

## Selected Stable Inputs

This is a selected non-atomic source capture, not a complete transitive closure.

```text
4222503031fb4971dde72c8fd1c18c959b31c5e576588a755b0bbac94b3f916c  🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
7daa136be6864cff2f13c6a496172d6ebe6fd83287080e2f96e50b0796a90e87  🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
6a684a67751efb699db63d374dcc9375fc6f895785802d5c14949e8a57e617a0  🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json
467ebe40db9a178add8253017ea0f7338e4e8141f5d060dd4e058c6e7ff7bf35  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️fixture.json
e47d9862883e75478a1a159de0866b48c22221f5d394ee98d4023a6995a52353  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️schema.json
e68f67e4dd0fa72901dc679d641c89f368caded5a7b9a8d3c2cd5e5f8ff87309  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️contract.json
ac8db5fe34a5efc825b66d1e56a24cdda8e929a70de850baa24808865a1e0424  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json
f3c2421772aa3df552de94fbeecbf6d6d43e967f72a9abdaf8d0683f0f085c3a  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
fb6c3a950c784797c8c7e0733d78decae9e3ed5aa151274f1c8d4adee857157e  🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
c6e193d70e668a86a475cb00bdb8a59eec6ac6fb481e66b1c70f85b62877042d  🧰️framework/🔨️modules/🌱️value/💾️resident/📋️project.json

```

