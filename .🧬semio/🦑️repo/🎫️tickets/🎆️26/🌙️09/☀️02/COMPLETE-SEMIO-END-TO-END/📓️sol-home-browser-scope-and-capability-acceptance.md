# Retained Home Browser Scope and Capability Acceptance

## Outcome

The retained visible Home identity bridge, invitation clipboard capability, and browser document lifecycle now have executable React/worker acceptance gates. Browser document ownership is keyed by the canonical Hub runtime key rather than the bare document id, and the Shell roster is projected only from a worker-verified scope and surface into host-only state.

The real Hub process journey is not yet green. Its first exact registered native build reached the former 7,200,000 ms build budget before producing the Hub binary. The launch seed now carries an 86,400,000 ms build/orchestration/command budget. The actual mounted Shell has additionally reached the real shard worker, but current producer assets fail closed on missing extension descriptors and a terminating Space shard; see `📓️sol-actual-shell-bootstrap-producer-frontier.md`.

## Implemented boundaries

- A resolved Hub identity is applied with `setClient` to the same visible retained Home instance before directory bootstrap and again before page acknowledgement.
- The worker retains an invitation token until an operation- and transfer-epoch-bound clipboard success result. Clipboard absence/denial is visible and retryable; success erases; duplicate/mismatched results reject without redisclosure.
- The exact `os.directory.open-administration` Home effect now passes through one pure Shell route validator into `{phase:"loading"}` and one typed `directory-administration-open` request. Missing, non-string, empty, oversized, or wrong-action inputs never open a pane.
- Capability disclosure is now author-page-bound at both renderer and worker boundaries. A canonical refresh that returns `member`/`public` erases any retained token before publishing the refreshed page; a hostile spectator-side synthetic copy intent and a direct worker capability request both fail closed.
- The pane carries English and German live-status text, restores keyboard focus to its heading on every settled phase, and structurally omits invite controls from the spectator projection.
- Hub worker lifecycle responses carry private `DocumentScope`; Presence additionally carries `verifiedSurfaceId` only after the exact Session actor confirms the plan/socket authority for that physical socket.
- Socket waiters, bootstrap rows, execution-target rows, presence beats, rebootstrap state, plugin routes, and close/revoke paths use `documentRuntimeKeyV1({kind:"hub", spaceId, documentId})`.
- Plugin actor URIs use `actor://<runtimeKey>`. The relay treats the URI suffix as an opaque retained key, then sends the original exact document and space to the worker; it never parses authority from the URI.
- The Shell filters normalized peers to the worker-verified surface and retains `actor`, `userId`, `label`, normalized role, `connectedAtMs`, and color in host-only state. Presence is not written to plugin view state.
- Closing the current socket publishes one exact-scope empty roster before its authority is erased. A stale socket cannot clear a replacement socket's authority.

## Executed evidence

### Invitation capability

`bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:directory-invite-capability-check --skip-nx-cache`

- source/schema oracle: 21 checks, green
- React administration tests: 6 passed, 61 skipped in the focused file run
- inline worker administration tests: 12 passed

The gate was driven red first on the missing `authorPage` disclosure fence, then green after the exact route, renderer author-page gate, worker author-page retention, authority-loss erasure, and EN/DE focus/status laws landed.

The route law begins with the real Home command id `os.directory.open-administration` and proves its strict `spaceId`/epoch arguments become the canonical typed `directory-administration-open` worker request plus a loading pane. Spectator projection has no invitation token or copy control in either language; German status text and denied-copy focus restoration are asserted. This is browser-neutral mounted React/worker evidence. The full live Shell observation remains gated by exact Space closure publication and the all-host descriptor fleet, so it is not represented as a current end-to-end browser success.

### Scope-safe presence

`bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:scoped-presence-check --skip-nx-cache`

- source/schema oracle: 18 checks, green
- language-neutral AJV plus React projection/lifecycle tests: 4 passed
- inline worker socket-authority/isolation test: 1 passed, 55 skipped

The neutral fixture opens `{space-a,same-document}` and `{space-b,same-document}` on distinct verified surfaces. It proves distinct runtime/actor keys, independent normalized rosters and bootstrap/execution rows, A-only close, and empty results for missing scope, mismatched scope, missing surface, and mismatched surface.

The matching in-app Chromium run is green at the explicitly narrow boundary `mounted-react-contract-probe-plus-real-browser-worker`. Its retained evidence is `🗑️generated/scoped-presence-chromium-runtime.json`: A began as `Ada:author:2`, B as `Berta:spectator:5`, wrong-surface data was absent, closing A cleared only A, and a subsequent B heartbeat left B live.

### Launch generation

- `@semio-tech/plugin-registry:generate --skip-nx-cache`: green; 59 plugin crates, 60 playgrounds, 45 framework packages
- `@semio-tech/plugin-registry:check-generated --skip-nx-cache`: green; generated catalog and launch bytes fresh
- generated launch includes `⚖️gate🎟️directory-invite-capability🌐️browser-worker`, `⚖️gate👥️scoped-presence🌐️browser-worker`, the settled deferred-wake order `411.0791`, and closed browser component-factory order `411.082`

### Full TypeScript boundary

`@semio-tech/framework-renderer-react:typecheck --skip-nx-cache` remains red on concurrent/pre-existing repository errors (tutorial tuples/snapshots, replication typed arrays, PluginRuntime/UI contracts, AgentBridge, Flow declaration, and existing worker execution-target errors). This run also caught required `Button.icon` props on the new administration pane and an inference-preview fixture whose ring inferred as an unbounded array. Those two scoped type failures were repaired with explicit semantic icons and an exact `GisMapInferencePreviewV1` fixture type; the focused invitation gate was rerun green afterward. A second full typecheck was not launched under current compiler pressure, so no whole-project green claim is made.

### Native/process status

- Home Hub process session `39691`: red before process launch; `cargo build --all-features --bin os-hub` exceeded 7,200,000 ms. No Hub route or process runtime claim is made.
- Space visible Home row native session `90899` remains active on `public-member-open-sol-target`, currently compiling `wasmtime-wasi`. It captured a superseded selector list before the OS-host/Stdio graph converged, so its terminal can qualify only that build snapshot. Current source keeps OS host independent of Stdio, retains the transformed-SVG workflow law, retires the deleted DWG selectors, and is source-green at 12 checks; the next warm run must execute that settled four-law group.
- Root-owned Hub execution-target/presence exact laws own `space-public-boundary-sol-target`; no duplicate Home process build was launched there.
- Physical Space WASI component materialization session `22205` was externally terminated with outer Nx exit `143` while Cargo compiled a valid declared guest dependency. No compiler or budget failure was emitted and no child survived. Registered producer session `59241` built the current 64,328,895-byte WASIp2 Space component and native descriptor emitter but terminated red at the emitter's former five-minute owned-execution deadline (`epoch deadline exceeded`) without atomically replacing the descriptor pair. Warm current session `63308` is rebuilding current-source WASIp2 dependencies with the two-billion fuel limit, a finite 30-minute wall limit, and exact compile/execute/fuel/wall progress telemetry; no runtime or publication claim is made before its terminal receipt.

## Remaining acceptance frontier

1. Take terminal receipts from current Space materialization session `63308` and old-snapshot Home build session `90899`; then rerun the settled current-source Home identity group on the released warm public cache.
2. Produce and mirror the missing all-host descriptor/module owners enumerated in `📓️terra-framework-shell-boot-catalog-assets-current-frontier.md`; do not synthesize browser metadata.
3. Re-observe the actual mounted Shell with a live Space shard and complete selected asset fleet.
4. Retry the registered Home Hub process gate with the current 24-hour seed budget after the root-owned Hub cache is released.
5. Do not claim static GIS plugin activation: authenticated execution-target dependency closure and private activation handoff remain separate P0 work.
