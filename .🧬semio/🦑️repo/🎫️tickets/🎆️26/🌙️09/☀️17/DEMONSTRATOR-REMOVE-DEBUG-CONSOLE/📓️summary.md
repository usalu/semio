# Demonstrator — remove temporary `[DEBUG]` console instrumentation

## Problem

The Entwerfen-mit-Bestand demonstrator console was flooded with host-side `[DEBUG]` lines (often at `console.error` level), especially during multi-pane boot:

- `buildShardClientOptions heartbeatTimeoutMs=…`
- `contributions document sources` / `contributions push` / `contributions publish`
- wgpu bridge boot-phase and render-surface traces
- ShellHost history, hot-swap, and extension diagnostics

These were temporary investigation logs (repo convention: `[DEBUG] ` prefix), not user-facing errors.

## Approach

1. Removed all `console.log|debug|info|warn|error` calls whose message includes `[DEBUG]` from the OS renderer stack, kernel `ActivationRegistry` / plugin sources, actor `ShardClient`, and wgpu `plugin-bridge` / `PluginRuntime` / related hosts.
2. Left real error paths that use non-`[DEBUG]` messages (e.g. `setContributions command failed`, `Framework OS boot failed`).
3. Left guest stderr routing that maps guest `[DEBUG]` lines to `console.debug` when `SEMIO_RUNTIME_DIAGNOSTICS` is armed (preview2 shim) — that path is opt-in and not the boot flood.
4. Removed shard-worker template arm confirmation logs from `browser-bundle/🏗️materialization` (generated worker bundle).
5. Updated `🩺️window-fault` contract test to assert `fromExamples` scoping instead of removed log string literals.

## Tooling

Ticket scripts (not part of product):

- `🐍️strip-debug-console.mjs` — TypeScript-AST codemod for most files
- `🐍️strip-shellhost-debug.mjs` — abandoned line-based attempt; ShellHost was fixed via codemod + manual repair of if/else damage

## Verification

- TypeScript parse check: `🏛️ShellHost`, `🐚️plugin-bridge`, `🔌️PluginRuntime`, `🎠️kernel`, `📮️shard-client` — parse clean after fixes.
- `rg 'console\.(log|debug|info|warn|error).*\[DEBUG\]'` on renderer + actor + kernel (non-test) — **0** matches.

## Follow-up

Rebuild / refresh the demonstrator dev bundle so `🟨️shard-worker.js` picks up materialization template changes. Re-run demonstrator acceptance or landing probes if you want runtime confirmation on a live server.
