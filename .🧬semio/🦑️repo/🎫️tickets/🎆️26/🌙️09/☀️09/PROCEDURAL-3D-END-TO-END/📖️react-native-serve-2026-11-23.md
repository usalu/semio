# React Native Serve 2026-11-23

URL: `http://127.0.0.1:6018/?plugin=generation3d`

Curl (no listener): exit 7, `http_code` 000, `Couldn't connect to server`. Port 6018 was not in `LISTEN`.

The serve scripts were not started. `wasm-bindgen` output, a staged `semio:procedural` module, and a real activation receipt were all absent, so a listener would die in Vite config load. A receipt was not written over empty modules.

## Command

From the workspace root, detached, the ticket script (not `repo run`):

```sh
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/serve-generation3d-native.sh
```

That execs bun on `serve-generation3d-native.ts` and appends to `🗑️generated/serve-native-6018.txt`. The TypeScript host sets `SEMIO_PLUGIN=generation3d`, `SEMIO_RENDERER=react`, `SEMIO_BUILD_MODE=prototype`, `S_OS_PORT=6018`, and Vite `configFile` `✏️s/🧑‍💻dev/🎭️variants/🌀️procedural/🏗️builder/🌐️vite/🟦️.ts`.

## Prior failure

`🗑️generated/serve-native-6018.txt` (2026-11-23 00:50): Vite `createFrameworkOsDevConfig` calls `readActivationReceipt` and throws `ENOENT` on

`.cache/b/p/m/browser/prototype/runtime/react/generation3d/activation/🔣️receipt.json`

Call site: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (`receiptDirectory` = `developmentRuntimeRoot(...) + "/activation"`). Reader: `readActivationReceipt` in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`. Writers: `nextActivationReceipt`, `publishActivationReceipt` in the same file. A valid receipt names real staged plugin artifacts (`pluginId`, `artifactSha256`). The only runtime directory present is `runtime/react/raster` (receipt for plugin `raster`). `runtime/react/generation3d` does not exist.

## Plugin module

`.cache/b/p/m/browser/prototype/modules` contains `🖨️raster`, `🧵️shard`, and `🪞️vendor`. No `🌀️procedural` directory. Component metadata: plugin id `semio:procedural` (`package.metadata.component.package`), crate `semio-hub-procedural-component`, cdylib, deployment directory `🌀️procedural`, manifest `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml`.

`publishComponentBrowserV1` (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts`) publishes under `builtPluginModulesRootIn(repo, mode)` plus the deployment directory, after assemble + `describe()` probe. The payload it adds on the assembled module is `🛂️.descriptor.semio`, `🔣️.json`, the bridge file, and `🧾️receipt.json` (component receipt, distinct from the activation receipt). Prototype profile ships unoptimized cores (`optimizeComponentCoresV1` returns null when the profile is not shipped). That publisher was not run: there is no wasm to assemble.

## Component driver block

`wgpu-native-components.sh` (pid 58486, log `🗑️generated/wgpu-native-components.txt`) and `native-component-procedural-3.txt` never printed `[native-build]`. `runNativeBuildDriverV1` is only entered from `runRepositoryComponentDriverV1` after the compiler image opens.

Both logs heartbeat on `native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d` mode `shared` (components log past 3321 s, still no `[native-build]`). Call chain before the driver:

1. `buildComponentClosureV1` (`driver-stages/📜️script.ts`, command `component:driver`)
2. `withRepositoryCompilerStoreV1` → `withRepositoryCompilerStoreOwnedV1` phase `cargo.compiler.image-open`
3. `openRepositoryCachePartitionV1` → `openRepositoryCachePartitionAttemptV1`
4. `openNativePlatformStorageV1` → fixed-image open → `fixedOptionsGuardV1` → `acquireQueuedResourceLease` on resource `native-fixed-image:<.cache/b/d>`, mode `shared`

The lease file is `.cache/b/l/m/data/aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.sqlite` (sha256 of that resource). `lsof` shows it held by bun pid 17436, child of `repo tools setup all` (pid 65332). A readonly sqlite open returned `database is locked`. Setup was not killed. Brief `resource-directory-admission:v1` lines are the lease directory gate, then the wait returns to the image lease.

`native-component-procedural-3.txt` sealed the project graph, then entered the same image wait (`repository:command`, shared, hundreds of seconds). It is not a second compile.

## Direct component build

Not started. Host gate at check time: memory free 54%, 1-minute load about 7.7, and 3 cargo processes already running (`cargo test` job-context pid 34300, `cargo check` wgpu frame-transaction pid 37689, `cargo test` procedural io-text pid 97721). The rule is fewer than 3 cargos, `-j` 1 or 2, and no new cargo under 40% free or load above 20. The procedural crate is a WIT cdylib (`plugin_exports!`), so a real module is `cargo build -p semio-hub-procedural-component --target wasm32-wasip2` into `.cache/g3d-native/t`, then the jco transpile `publishComponentBrowserV1` consumes. `wasm-bindgen` is the renderer path, not this crate.

## Remaining blockers

- Shared lease on `.cache/b/d` held by setup-all, so the native component driver never reaches `runNativeBuildDriverV1`.
- No staged `🌀️procedural` module, so `publishActivationReceipt` must not run.
- Compile slot full (3 cargos), so the direct wasip2 build was not started.
- React listener stays down until a real module and `runtime/react/generation3d/activation/🔣️receipt.json` exist.
