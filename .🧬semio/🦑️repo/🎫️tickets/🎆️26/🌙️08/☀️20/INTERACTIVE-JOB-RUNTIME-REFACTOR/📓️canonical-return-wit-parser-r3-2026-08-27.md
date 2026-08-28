# Canonical Return WIT Parser R3

Actual read-only parser/metadata gate exit 0. The installed Bytecode Alliance component tools parsed the current entire Plugin WIT, including the neutral `byte-page` and type-only `return-page` declarations, and emitted a 16,185-byte dummy metadata module in memory. No file was generated, guest compiled, guest instantiated or guest executed. The old poll signature remains unchanged, so this is not a return-poll code-generation or runtime acceptance gate.

## Failed Bun Runtime Attempts

R1 imported `@bytecodealliance/jco`; R2 narrowed to `@bytecodealliance/jco-transpile/wasm-tools`. Both printed the parser result but exited 1 because the installed tool's worker path calls unsupported Bun `process.binding("tcp_wrap")`. Their success log is not a successful command result. Exact failure excerpt:

```text
error: process.binding("tcp_wrap") is not implemented in Bun.
[DEBUG] installed jco WIT parser/metadata embedding PASS 16185 bytes; no guest compilation/execution
Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "await eval(process.env.SEMIO_RETURN_WIT_ORACLE)"
status: 1
```

## Successful Existing Node Runtime Behind Nx

Bun remained the package manager and Nx the runner; only the external tool evaluation used the existing Node runtime. No dependencies, scripts, Nx targets or global configuration changed.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- node --input-type=module -e 'await eval(process.env.SEMIO_RETURN_WIT_ORACLE)'
```

The environment variable held this exact evaluation source:

```javascript
(async()=>{const {readFileSync}=await import("node:fs");const{componentEmbed}=await import("@bytecodealliance/jco-transpile/wasm-tools");const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit";const source=readFileSync(path,"utf8");const bytes=await componentEmbed({witSource:source,world:"actor",dummy:true,features:{tag:"all"}});console.log("[DEBUG] installed jco WIT parser/metadata embedding PASS "+bytes.length+" bytes; no guest compilation/execution");})()
```

Actual output, exit 0:

```text
[DEBUG] installed jco WIT parser/metadata embedding PASS 16185 bytes; no guest compilation/execution
```
