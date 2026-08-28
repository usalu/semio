# Independent Permanent Typed-Command Fixture Diagnostic — 2026-08-27

## Actual Result and Scope

Final read-only Bun/Nx diagnostic R4 exited0: strict Ajv Draft2020-12 validation passes, the nine declared law families total40 rows, and all7 output rows match independent Node Buffer and TextEncoder byte counts including the existing one-byte UI-scope field. This is schema/output validation only. It does not run the native owned state machine, the33 other semantic rows, Plugin integration, or any timing gate.

The new permanent fixture explicitly says it is newly authored evidence, not reconstruction of missing ticket bytes. The Plugin native include now targets that exact domain fixture; native compilation/runtime remains pending the coherent lifecycle and metadata boundary.

## Diagnostic Corrections

R1 failed before validation because Nx exec re-quoted an inline program. It also exposed Nx exec's default all-project selection; failure occurred in the first selected project and no mutation was attempted. Subsequent runs explicitly select only workspace. R2 failed before validation because direct eval did not accept top-level await; the final diagnostic uses an async function expression. R3 validated schema and row count, then failed because the coordinator's temporary output oracle omitted the existing one-byte UI-scope field. Direct source inspection of both OwnedTypedCommandCensus and SerdeJsonTypedCommandCensus corrected the diagnostic; no fixture or production code was changed in response. These three failures are not product regression claims.

No temporary executable file, runtime dependency, source edit, fixture reconstruction, cleanup or Git mutation was used. Permanent native selectors remain the existing language_neutral_empty_single_max_and_plus_one_match_the_test_only_oracle and every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle.

## R1

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec -- bun -e 'const {default: Ajv} = await import("ajv/dist/2020.js"); const {Buffer} = await import("node:buffer"); const fixture=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️fixture.json").json(); const schema=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️schema.json").json(); const validate=new Ajv({strict:true,allErrors:true}).compile(schema); if(!validate(fixture)) throw new Error(JSON.stringify(validate.errors)); const rows=["cases","grantLaws","freshnessLaws","admissionLaws","publicationLaws","laneTrace","rawPageLaws","faultLaws","closeLaws"]; const counts=Object.fromEntries(rows.map(key=>[key,fixture[key].length])); const total=Object.values(counts).reduce((sum,n)=>sum+n,0); if(total!==40) throw new Error("Wrong row count"); for(const row of fixture.cases){const bytes=Buffer.byteLength(row.description,"utf8")+Buffer.byteLength(row.coalesceKey,"utf8"); const encoded=new TextEncoder().encode(row.description).byteLength+new TextEncoder().encode(row.coalesceKey).byteLength; if(bytes!==row.expectedBytes || bytes!==encoded || (bytes<=fixture.capacities.maxOutputBytes)!==row.accepted) throw new Error("Output case disagrees: "+JSON.stringify(row)); } console.log("[DEBUG] permanent-full-operation-fixture schema=true rows="+total+" outputOracle="+fixture.cases.length+" counts="+JSON.stringify(counts));'
```

```text
1 | {default: Ajv} = await import(ajv/dist/2020.js); const {Buffer} = await import(node:buffer); const fixture=await Bun.fil
                                                 ^
error: Syntax Error
    at /Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/✨️macros/📦️packages/🦀️rust/[eval]:1:46

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "const {default: Ajv} = await import("ajv/dist/2020.js"); const {Buffer} = await import("node:buffer"); const fixture=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️fixture.json").json(); const schema=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️schema.json").json(); const validate=new Ajv({strict:true,allErrors:true}).compile(schema); if(!validate(fixture)) throw new Error(JSON.stringify(validate.errors)); const rows=["cases","grantLaws","freshnessLaws","admissionLaws","publicationLaws","laneTrace","rawPageLaws","faultLaws","closeLaws"]; const counts=Object.fromEntries(rows.map(key=>[key,fixture[key].length])); const total=Object.values(counts).reduce((sum,n)=>sum+n,0); if(total!==40) throw new Error("Wrong row count"); for(const row of fixture.cases){const bytes=Buffer.byteLength(row.description,"utf8")+Buffer.byteLength(row.coalesceKey,"utf8"); const encoded=new TextEncoder().encode(row.description).byteLength+new TextEncoder().encode(row.coalesceKey).byteLength; if(bytes!==row.expectedBytes || bytes!==encoded || (bytes<=fixture.capacities.maxOutputBytes)!==row.accepted) throw new Error("Output case disagrees: "+JSON.stringify(row)); } console.log("[DEBUG] permanent-full-operation-fixture schema=true rows="+total+" outputOracle="+fixture.cases.length+" counts="+JSON.stringify(counts));"
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
  pid: 576,
  stdout: null,
  stderr: null
}

exit_code=1
```

## R2

```sh
SEMIO_FIXTURE_EVAL='const {default: Ajv} = await import("ajv/dist/2020.js"); const {Buffer} = await import("node:buffer"); const fixture=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️fixture.json").json(); const schema=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️schema.json").json(); const validate=new Ajv({strict:true,allErrors:true}).compile(schema); if(!validate(fixture)) throw new Error(JSON.stringify(validate.errors)); const rows=["cases","grantLaws","freshnessLaws","admissionLaws","publicationLaws","laneTrace","rawPageLaws","faultLaws","closeLaws"]; const counts=Object.fromEntries(rows.map(key=>[key,fixture[key].length])); const total=Object.values(counts).reduce((sum,n)=>sum+n,0); if(total!==40) throw new Error("Wrong row count"); for(const row of fixture.cases){const bytes=Buffer.byteLength(row.description,"utf8")+Buffer.byteLength(row.coalesceKey,"utf8"); const encoded=new TextEncoder().encode(row.description).byteLength+new TextEncoder().encode(row.coalesceKey).byteLength; if(bytes!==row.expectedBytes || bytes!==encoded || (bytes<=fixture.capacities.maxOutputBytes)!==row.accepted) throw new Error("Output case disagrees: "+JSON.stringify(row)); } console.log("[DEBUG] permanent-full-operation-fixture schema=true rows="+total+" outputOracle="+fixture.cases.length+" counts="+JSON.stringify(counts));' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'eval(process.env.SEMIO_FIXTURE_EVAL)'
```

```text
1 | eval(process.env.SEMIO_FIXTURE_EVAL)
    ^
SyntaxError: Unexpected keyword 'import'. Expected ';' after variable declaration.
      at /Users/ueli/Documents/semio/[eval]:1:1

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "eval(process.env.SEMIO_FIXTURE_EVAL)"
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
  pid: 6408,
  stdout: null,
  stderr: null
}

exit_code=1
```

## R3

```sh
SEMIO_FIXTURE_EVAL='(async () => { const {default: Ajv} = await import("ajv/dist/2020.js"); const {Buffer} = await import("node:buffer"); const fixture=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️fixture.json").json(); const schema=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️schema.json").json(); const validate=new Ajv({strict:true,allErrors:true}).compile(schema); if(!validate(fixture)) throw new Error(JSON.stringify(validate.errors)); const rows=["cases","grantLaws","freshnessLaws","admissionLaws","publicationLaws","laneTrace","rawPageLaws","faultLaws","closeLaws"]; const counts=Object.fromEntries(rows.map(key=>[key,fixture[key].length])); const total=Object.values(counts).reduce((sum,n)=>sum+n,0); if(total!==40) throw new Error("Wrong row count"); for(const row of fixture.cases){const bytes=Buffer.byteLength(row.description,"utf8")+Buffer.byteLength(row.coalesceKey,"utf8"); const encoded=new TextEncoder().encode(row.description).byteLength+new TextEncoder().encode(row.coalesceKey).byteLength; if(bytes!==row.expectedBytes || bytes!==encoded || (bytes<=fixture.capacities.maxOutputBytes)!==row.accepted) throw new Error("Output case disagrees: "+JSON.stringify(row)); } console.log("[DEBUG] permanent-full-operation-fixture schema=true rows="+total+" outputOracle="+fixture.cases.length+" counts="+JSON.stringify(counts)); })()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_FIXTURE_EVAL)'
```

```text
error: Output case disagrees: {"name":"empty","description":"","coalesceKey":"","expectedBytes":1,"accepted":true}
      at <anonymous> (file:///Users/ueli/Documents/semio/%5Beval%5D:1:1294)

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "await eval(process.env.SEMIO_FIXTURE_EVAL)"
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
  pid: 7087,
  stdout: null,
  stderr: null
}

exit_code=1
```

## R4

```sh
SEMIO_FIXTURE_EVAL='(async () => { const {default: Ajv} = await import("ajv/dist/2020.js"); const {Buffer} = await import("node:buffer"); const fixture=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️fixture.json").json(); const schema=await Bun.file("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️full-operation/🧪️schema.json").json(); const validate=new Ajv({strict:true,allErrors:true}).compile(schema); if(!validate(fixture)) throw new Error(JSON.stringify(validate.errors)); const rows=["cases","grantLaws","freshnessLaws","admissionLaws","publicationLaws","laneTrace","rawPageLaws","faultLaws","closeLaws"]; const counts=Object.fromEntries(rows.map(key=>[key,fixture[key].length])); const total=Object.values(counts).reduce((sum,n)=>sum+n,0); if(total!==40) throw new Error("Wrong row count"); for(const row of fixture.cases){const bytes=1+Buffer.byteLength(row.description,"utf8")+Buffer.byteLength(row.coalesceKey,"utf8"); const encoded=1+new TextEncoder().encode(row.description).byteLength+new TextEncoder().encode(row.coalesceKey).byteLength; if(bytes!==row.expectedBytes || bytes!==encoded || (bytes<=fixture.capacities.maxOutputBytes)!==row.accepted) throw new Error("Output case disagrees: "+JSON.stringify(row)); } console.log("[DEBUG] permanent-full-operation-fixture schema=true rows="+total+" outputOracle="+fixture.cases.length+" counts="+JSON.stringify(counts)); })()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_FIXTURE_EVAL)'
```

```text
[DEBUG] permanent-full-operation-fixture schema=true rows=40 outputOracle=7 counts={"cases":7,"grantLaws":4,"freshnessLaws":3,"admissionLaws":4,"publicationLaws":3,"laneTrace":9,"rawPageLaws":3,"faultLaws":3,"closeLaws":4}

exit_code=0
```


