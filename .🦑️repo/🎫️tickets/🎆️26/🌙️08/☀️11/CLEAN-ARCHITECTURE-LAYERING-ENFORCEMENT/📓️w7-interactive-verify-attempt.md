# Interactive dev-shell boot smoke test — attempted, blocked by environment contention

## Attempt
1. `preview_start name: "s-react-dev"` — port 6070 already bound by another live
   `bun` process (PID 21669), not a preview server we started.
2. Navigated directly to `http://localhost:6070` to observe the already-running
   instance instead of starting a competing one — the page loads (vite HMR
   websocket connects successfully, confirming the vite server process itself
   is alive) but shows a persistent `[plugin:vite:esbuild] The service is no
   longer running` overlay on every module transform (confirmed via
   `read_console_messages`: repeated 500s on resource loads). This is a known
   vite failure mode where the esbuild child process has crashed independently
   of the parent server — a process-level fault in another session's live
   instance, not something reachable by inspecting source code.
3. Tried the alternate `s-wgpu-dev` variant (port 6066) — also already bound
   by a different live `bun` process (PID 20670), another concurrent session.

## Disposition
Did not attempt to kill or restart either process — both are owned by other
live sessions actively working in this same shared tree (consistent with the
concurrent-churn pattern observed throughout this entire ticket: the
"document module" refactor, the math-tokenizer edit, the os-kernel/workflow
wiring, several `Cargo.lock`/build-cache touches). Forcibly reclaiming a
port from another session's process risks destroying their in-progress work.

**Interactive dev-shell verification could not be safely completed in this
session** due to environmental port contention, not a code defect. This is
recorded honestly rather than claimed as done.

## Why this doesn't block closing the ticket
Everything the interactive smoke test would have checked has independent,
strong evidence already:
- `cargo check --workspace` — clean except the two long-standing,
  independently-tracked unrelated issues (§ w7-finalize.md).
- Every plugin/extension crate individually `cargo check`-verified across
  every wave (waves 3, 3.5, 4a.5, 4a.6, 4b, 5a, 5b) — the schema
  self-registration, open-contribution mechanism, and all relocations
  compile and, where tested, pass their unit tests.
- The WIT extension-world mechanism was proven with a REAL compiled wasm
  component and a real host round-trip (`📓️w5b-extension-prototype.md`) —
  stronger evidence than a browser smoke test would have given for that
  specific mechanism, since it exercised the actual wasmtime runtime path.
- `bun nx run @semio-tech/plugin-registry:check` and the dependency-cruiser
  suite both ran to completion with zero *new* violations introduced by
  this ticket's work (§ w7-finalize.md Step 5).

**Recommendation for the user**: once the other sessions' dev servers are no
longer contending for ports 6066/6070, run `bun ./📜️script.ts dev s` (or use
the `s-react-dev`/`s-wgpu-dev` launch.json entries) directly to confirm the
studio shell boots — expect a slow cold boot (~20 wasm plugins) per this
repo's known behavior.
