# Nx 23 Plugin Syntax Origin Audit

Read-only audit of TextNative61054 bootstrap failure, 2026-10-13. No invocation selector, arguments, configuration, patch, installed module or source changes. No goal GUI rerun or verbose flag. Only node syntax check and isolated devkit module loading were run; generated evidence is `🗑️generated/nx-patch-parse-audit.txt` and `🗑️generated/nx-devkit-load-audit.txt`.

Concrete broken file is `.nx/installation/node_modules/nx/dist/src/tasks-runner/cache.js:374` under immutable installation digest `15e6c5f8fa966ab30ea5adb83117ed08dbfbc49d3999934ff418f64564174fe1`. The source has `if (process.env.SEMIO_NX_SELECTED_GRAPH_RECEIPT ...` directly in the class body, followed by two more injected statements at375–376, before `async put(task, terminalOutput, outputs, code) {` at377. Node v24.15.0 `--check` reproduces `SyntaxError: Unexpected token '.'` with caret at `process.env`. This is malformed installed placement, not unsupported optional chaining or an old Node runtime.

All patched JS targets listed in current `nx@23.2.0.patch` were checked; this was the only failing target. Directly loading selected `nx/dist/src/devkit-internals.js` reproduces the same syntax error through its `require("./tasks-runner/cache")` at234:15. Both plugins consume Nx devkit, explaining their shared failure before the native task runs. The captured stack is:

```
SyntaxError: Unexpected token '.'
    at wrapSafe (node:internal/modules/cjs/loader:1763:18)
    at Module._compile (node:internal/modules/cjs/loader:1804:20)
    at Module._extensions..js (node:internal/modules/cjs/loader:1961:10)
    at Module.load (node:internal/modules/cjs/loader:1553:32)
    at Module._load (node:internal/modules/cjs/loader:1355:12)
    at wrapModuleLoad (node:internal/modules/cjs/loader:255:19)
    at Module.require (node:internal/modules/cjs/loader:1576:12)
    at require (node:internal/modules/helpers:153:16)
    at Object.<anonymous> (.../nx/dist/src/devkit-internals.js:234:15)
```

Patch origin: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch:825–831`, second cache `put` hunk `@@ -369,6 +372,9 @@`. Authored additions follow the `async put` context line, but installed bytes place them before it. Root `node_modules/nx/dist/src/tasks-runner/cache.js:369–372` is an independent unpatched local witness: return results369, close370, async put371, return tryAndRetry372. The intended additions therefore belong inside that method. Avoid editing immutable installed bytes; repair the authored installation/patch application at its owner and reacquire the resulting new identity.

Actual bootstrap owner `.../🚀️bootstrap/📜️script.ts:377–387,427` provisions and activates pinned tools, derives identity from modulePath, then runs selected Nx CLI through `node`. `.../🚀️bootstrap/🛠️tools/📜️script.ts:80–138` reads recipe and patch bytes, uses content identity, installs Bun frozen dependencies with ignore-scripts/copyfile backend, then receipts installed files and publishes immutable destination. Its `🩹️patch/🟦️.ts:19–29` checks addition-line presence and hashes actual files; it does not check syntax or placement, so the malformed second hunk survives its receipt checks.

This audit establishes only the bootstrap parse failure. It makes no claim about the native feature or prior SourceRoot result.

## Changed Frontier

Subsequent readback observes selected immutable installation `fc99bcae5b37fa6bb381d766f4a43c93452ad678e19556910775fa2b9ceb514a`. Its `cache.js:374` now opens `async put`; guards are inside at375–377. Root reports fresh `node --check` EXIT0 for this installation. Earlier captured failure remains authentic for digest15e6..., but is no longer the current frontier. This audit agent made no patch/installation change or fresh native dispatch. Concurrent owner correction was preserved.
