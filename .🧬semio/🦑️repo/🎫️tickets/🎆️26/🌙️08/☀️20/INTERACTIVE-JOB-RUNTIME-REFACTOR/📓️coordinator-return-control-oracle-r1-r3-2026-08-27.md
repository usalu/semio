# Coordinator Canonical Return Control Oracle R1–R3 — 2026-08-27

## Outcome

After two actual strict-schema compilation failures, R3 exited 0 with all six shared drives matching the independent @webassemblyjs LEB128 encoder and seven malformed receipt JSON cases rejected. The exact maximum drive is 43 bytes. The production return schema, fixture schema and fixture hashes were stable across R3.

```text
[DEBUG] actor-return-control-oracle {"fixtureValid":true,"independentLeb128":[{"kind":"execute","bytes":3},{"kind":"poll","bytes":5},{"kind":"inputAck","bytes":8},{"kind":"cancel","bytes":5},{"kind":"retiredAck","bytes":5},{"kind":"inputAck","bytes":43}],"receiptNegatives":7,"maxObservedBytes":43,"scope":"control fixture/schema only; no native or host state-machine proof"}
```

This is schema/vector proof only. No native return owner, host page state machine, semantic section codec, runtime cancellation or 8 ms timing proof is inferred.

## Actual REDs

R1 failed before codec assertions: missing string type for pattern in u64's allOf/not. R2 failed before assertions: missing number type for minimum in pageReceipt's conditional length branch. Dag repaired both schema type declarations; strict Ajv remained enabled. R2's production schema changed during the captured window, so the failure is preserved as live cutover evidence rather than attributed to the later released hash.

```text
169 | function checkStrictMode(it, msg, mode = it.opts.strictSchema) {
170 |     if (!mode)
171 |         return;
172 |     msg = `strict mode: ${msg}`;
173 |     if (mode === true)
174 |         throw new Error(msg);
                        ^
error: strict mode: missing type "string" for keyword "pattern" at "semio.actor.retained-return.v1#/allOf/1/not" (strictTypes)
      at checkStrictMode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/util.js:174:19)
      at strictTypesError (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:285:16)
      at checkKeywordTypes (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:261:17)
      at checkStrictTypes (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:233:5)
      at schemaKeywords (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:189:9)
      at typeAndKeywords (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:128:5)
      at subSchemaObjCode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:115:5)
      at subschemaCode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:91:13)
      at subschema (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:438:9)
      at code (/Users/ueli/Documents/semio/node_modules/ajv/dist/vocabularies/applicator/not.js:15:13)

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "await eval(process.env.SEMIO_RETURN_CONTROL_ORACLE_EVAL)"
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
  pid: 24472,
  stdout: null,
  stderr: null
}
```

```text
169 | function checkStrictMode(it, msg, mode = it.opts.strictSchema) {
170 |     if (!mode)
171 |         return;
172 |     msg = `strict mode: ${msg}`;
173 |     if (mode === true)
174 |         throw new Error(msg);
                        ^
error: strict mode: missing type "number" for keyword "minimum" at "semio.actor.retained-return.v1#/allOf/0/then/properties/length" (strictTypes)
      at checkStrictMode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/util.js:174:19)
      at strictTypesError (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:285:16)
      at checkKeywordTypes (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:261:17)
      at checkStrictTypes (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:233:5)
      at schemaKeywords (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:189:9)
      at typeAndKeywords (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:128:5)
      at subSchemaObjCode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:115:5)
      at subschemaCode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:91:13)
      at subschema (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:438:9)
      at applyPropertySchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/vocabularies/applicator/properties.js:45:17)

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "await eval(process.env.SEMIO_RETURN_CONTROL_ORACLE_EVAL)"
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
  pid: 25998,
  stdout: null,
  stderr: null
}
```

## R3 Stable Source Census

```text
0cf9197fa556d4c0b382465d825425f0787e6de4a718bc8f489efb7ee1db5bb1  🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema.json
7395952af17577d25e40d737b8d1a1d7ef50d2ae872717de06319a1f2a3bf45a  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️schema.json
7e75ffbce0eadc7ba189605f234b0ba5929ec7693ee1748b95faeb5714351ec3  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️fixture.json
```

## Independent Command

The inline evaluation creates no script file and reads only the canonical source schemas/fixture.

```sh
SEMIO_RETURN_CONTROL_ORACLE_EVAL='(async()=>{const {readFileSync}=await import("node:fs");const {Buffer}=await import("node:buffer");const {default:Ajv}=await import("ajv");const {strictEqual}=await import("node:assert");const base="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/";const read=p=>JSON.parse(readFileSync(base+p,"utf8"));const schema=read("📤️return/🧬️schema.json");const fixture=read("📤️return/🧪️fixture.json");const ajv=new Ajv({strict:true,allErrors:true}).addSchema(read("🚪️lifetime/🧬️schema.json")).addSchema(read("📄️page/🧬️schema.json")).addSchema(schema);const valid=ajv.compile(read("📤️return/🧪️schema.json"));strictEqual(valid(fixture),true,JSON.stringify(valid.errors));const module=await import("@webassemblyjs/leb128/lib/leb.js");const encode=(module.default??module).encodeUIntBuffer;const integer=v=>{const input=Buffer.alloc(8);input.writeBigUInt64LE(BigInt(v));return Buffer.from(encode(input));};const origin=v=>[integer(v.activationGeneration),integer(v.requestSequence)];const identity=v=>[...origin(v.origin),integer(v.returnSequence)];const receipt=v=>[...identity(v.identity),integer(v.pageSequence),integer(v.length),Buffer.from([v.final?1:0])];const rows=[];for(const row of fixture.wireVectors){const value=row.value;const chunks=[Buffer.from([fixture.driveTags[value.kind]])];if(value.kind==="execute")chunks.push(...origin(value.origin));else{const c=value.control;chunks.push(Buffer.from([fixture.controlTags[c.kind]]));chunks.push(...(c.kind==="inputAck"?receipt(c.receipt):identity(c.identity)));}const bytes=Buffer.concat(chunks);strictEqual(bytes.toString("hex"),row.hex);rows.push({kind:value.kind==="execute"?"execute":value.control.kind,bytes:bytes.length});}const v=ajv.compile({$ref:schema.$id+"#/definitions/pageReceipt"});let negatives=0;for(const bad of [{...fixture.receipt,pageSequence:"0"},{...fixture.receipt,length:4097},{...fixture.receipt,length:0,final:false},{...fixture.receipt,final:1},{...fixture.receipt,foreign:true},{...fixture.receipt,identity:{...fixture.identity,returnSequence:"0"}},{...fixture.receipt,identity:{...fixture.identity,origin:{...fixture.origin,requestSequence:9007199254740992}}}]){strictEqual(v(bad),false);negatives++;}strictEqual(v({...fixture.receipt,length:0,final:true}),true);console.log("[DEBUG] actor-return-control-oracle "+JSON.stringify({fixtureValid:true,independentLeb128:rows,receiptNegatives:negatives,maxObservedBytes:Math.max(...rows.map(x=>x.bytes)),scope:"control fixture/schema only; no native or host state-machine proof"}));})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_RETURN_CONTROL_ORACLE_EVAL)'
```

## Review Boundary

The coordinator read all three original canonical files and the complete released contract report. The protocol distinguishes the frozen original transport poll origin from semantic Open/Close sequences and from later ACK correlation ids. Native issuance and page sequences remain independent authority. Raw page-input ACK, UI publication ACK and instance-lifecycle retirement are separate; controls do not execute semantic work or recursively produce variable output. The added monotonic replay frontier must also be exercised by state-machine tests, not credited from its fixture descriptions. Variable semantic content framing remains a mandatory forthcoming part of this single cutover; no interim whole-result or compatibility result branch is approved.

