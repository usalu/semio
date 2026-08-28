# Coordinator WIT Return Parser R1–R3

Independent root R3 exited 0. The installed Bytecode Alliance parser read the current complete Plugin WIT and returned a **16,185-byte dummy metadata module in memory**. The WIT hash was identical before and after. No files were generated and no component was compiled, instantiated or run. The return-page interface is still type-only; the existing poll signature has not been cut over.

## Coordinator Invocation Failures

R1 exited 1 because Nx exec reconstructed the inline Node command with broken nested quotation; the parser did not execute. R2 exited 1 because the environment-provided eval body contained top-level await outside an async function. These are coordinator command-construction failures, not WIT or implementation REDs. R3 uses the existing documented async IIFE inside a task-specific environment variable, with Node only as the installed external tool runtime, Bun as package manager and Nx as runner. No dependency or target changes were made.

## Successful Command

```sh
SEMIO_ROOT_RETURN_WIT_ORACLE='(async()=>{const {readFileSync}=await import("node:fs"); const {componentEmbed}=await import("@bytecodealliance/jco-transpile/wasm-tools"); const source=readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit","utf8"); const bytes=await componentEmbed({witSource:source,world:"actor",dummy:true,features:{tag:"all"}}); console.log("[DEBUG] independent current WIT parser/metadata bytes="+bytes.length+"; no guest execution");})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- node --input-type=module -e 'await eval(process.env.SEMIO_ROOT_RETURN_WIT_ORACLE)'
```

## Stable SHA-256

```text
facc99a3b56cf976d51ff6466e9ce98992cddcceadc021561bd879aae8c2039d  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit
```

## Actual Output

```text
[DEBUG] independent current WIT parser/metadata bytes=16185; no guest execution
exit_code: 0
```

## Failed R1 and R2 Output

```text
file:///Users/ueli/Documents/semio/[eval1]:1
const {readFileSync}=await import(node:fs); const {componentEmbed}=await import(@bytecodealliance/jco-transpile/wasm-tools); const source=readFileSync(🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit,utf8); const bytes=await componentEmbed({witSource:source,world:actor,dummy:true,features:{tag:all}}); console.log([DEBUG]
                                      ^

SyntaxError: Unexpected token ':'
    at compileSourceTextModule (node:internal/modules/esm/utils:318:16)
    at ModuleLoader.createModuleWrap (node:internal/modules/esm/loader:231:12)
    at ModuleLoader.eval (node:internal/modules/esm/loader:270:23)
    at node:internal/process/execution:77:24
    at asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:101:11)
    at Object.runEntryPointWithESMLoader (node:internal/modules/run_main:123:19)
    at evalModuleEntryPoint (node:internal/process/execution:76:47)
    at node:internal/main/eval_string:37:3

Node.js v24.15.0
/bin/sh: no: command not found
Error: Command failed: "node" "--input-type=module" "-e" "const {readFileSync}=await import("node:fs"); const {componentEmbed}=await import("@bytecodealliance/jco-transpile/wasm-tools"); const source=readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit","utf8"); const bytes=await componentEmbed({witSource:source,world:"actor",dummy:true,features:{tag:"all"}}); console.log("[DEBUG] independent current WIT parser/metadata bytes="+bytes.length+"; no guest execution");"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 127,
  signal: null,
  output: [ null, null, null ],
  pid: 89900,
  stdout: null,
  stderr: null
}

<anonymous_script>:1
const {readFileSync}=await import("node:fs"); const {componentEmbed}=await import("@bytecodealliance/jco-transpile/wasm-tools"); const source=readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit","utf8"); const bytes=await componentEmbed({witSource:source,world:"actor",dummy:true,features:{tag:"all"}}); console.log("[DEBUG] independent current WIT parser/metadata bytes="+bytes.length+"; no guest execution");
                     ^^^^^

SyntaxError: await is only valid in async functions and the top level bodies of modules
    at file:///Users/ueli/Documents/semio/[eval1]:1:24
    at ModuleJob.run (node:internal/modules/esm/module_job:437:25)
    at async onImport.tracePromise.__proto__ (node:internal/modules/esm/loader:246:26)
    at async ModuleLoader.executeModuleJob (node:internal/modules/esm/loader:243:20)
    at async asyncRunEntryPointWithESMLoader (node:internal/modules/run_main:101:5)

Node.js v24.15.0
Error: Command failed: "node" "--input-type=module" "-e" "await eval(process.env.SEMIO_ROOT_RETURN_WIT_ORACLE)"
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
  pid: 92045,
  stdout: null,
  stderr: null
}

```

