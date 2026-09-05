# Retained Home Browser Scope and Capability Acceptance

## Outcome

The retained visible Home identity bridge, invitation clipboard capability, and browser document lifecycle now have executable React/worker acceptance gates. Browser document ownership is keyed by the canonical Hub runtime key rather than the bare document id, and the Shell roster is projected only from a worker-verified scope and surface into host-only state.

The real Hub process journey is not yet green. Its first exact registered native build reached the former 7,200,000 ms build budget before producing the Hub binary. The launch seed now carries an 86,400,000 ms build/orchestration/command budget. The actual mounted Shell has additionally reached the real shard worker, but current producer assets fail closed on missing extension descriptors and a terminating Space shard; see `📓️sol-actual-shell-bootstrap-producer-frontier.md`.

## Implemented boundaries

- A resolved Hub identity is applied with `setClient` to the same visible retained Home instance before directory bootstrap and again before page acknowledgement.
- The worker retains an invitation token until an operation- and transfer-epoch-bound clipboard success result. Clipboard absence/denial is visible and retryable; success erases; duplicate/mismatched results reject without redisclosure.
- Hub worker lifecycle responses carry private `DocumentScope`; Presence additionally carries `verifiedSurfaceId` only after the exact Session actor confirms the plan/socket authority for that physical socket.
- Socket waiters, bootstrap rows, execution-target rows, presence beats, rebootstrap state, plugin routes, and close/revoke paths use `documentRuntimeKeyV1({kind:"hub", spaceId, documentId})`.
- Plugin actor URIs use `actor://<runtimeKey>`. The relay treats the URI suffix as an opaque retained key, then sends the original exact document and space to the worker; it never parses authority from the URI.
- The Shell filters normalized peers to the worker-verified surface and retains `actor`, `userId`, `label`, normalized role, `connectedAtMs`, and color in host-only state. Presence is not written to plugin view state.
- Closing the current socket publishes one exact-scope empty roster before its authority is erased. A stale socket cannot clear a replacement socket's authority.

## Executed evidence

### Invitation capability

`bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:directory-invite-capability-check --skip-nx-cache`

- source/schema oracle: 15 checks, green
- React administration tests: 10 passed
- inline worker administration tests: 4 passed, 51 skipped

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
- generated launch includes `⚖️gate🎟️directory-invite-capability🌐️browser-worker` and `⚖️gate👥️scoped-presence🌐️browser-worker`

### Full TypeScript boundary

`@semio-tech/framework-renderer-react:typecheck --skip-nx-cache` remains red on concurrent/pre-existing repository errors (tutorial tuples/snapshots, replication typed arrays, directory reexports, parser export, PluginRuntime/UI contracts, Flow declaration, and existing worker execution-target errors). It reported no line attributable to the scoped-presence implementation or its tests.

### Native/process status

- Home Hub process session `39691`: red before process launch; `cargo build --all-features --bin os-hub` exceeded 7,200,000 ms. No Hub route or process runtime claim is made.
- Space visible Home row native session `50225`: terminal red before laws at receipt `home-directory-identity-rows-exact/exact-cargo-laws-RgN8W1/00`; the async plugin-host converter did not cover the concurrently added typed `RequestInferenceProposal` WIT variant. The exact conversion and an async semantic law are now present, and registered source parity is green (`wit=1 sync=1 async=1 mutation=1`). Coordinated retry session `33039`, receipt `exact-cargo-laws-1uMOf1/00`, also stopped during the build before laws because concurrent durable-group source imported `CursorRevisionAccumulator` from the stale `crate::os_store` boundary. The public cache was released to that owner; neither red receipt is a Home identity-row or inference-conversion semantic verdict.
- Root-owned Hub execution-target/presence exact laws own `space-public-boundary-sol-target`; no duplicate Home process build was launched there.
- Physical Space WASI component materialization session `22205` was externally terminated with outer Nx exit `143` while Cargo compiled a valid declared guest dependency. No compiler or budget failure was emitted and no child survived. The same registered one-crate producer resumed on the warm dedicated `home-space-component-sol-target` as session `45110`; static or mixed-date metadata is not treated as materialization evidence.

## Remaining acceptance frontier

1. Take a terminal receipt from resumed Space materialization session `45110`, then rerun the registered two-group Home identity native target after the coordinated public cache is released.
2. Produce and mirror the missing all-host descriptor/module owners enumerated in `📓️terra-framework-shell-boot-catalog-assets-current-frontier.md`; do not synthesize browser metadata.
3. Re-observe the actual mounted Shell with a live Space shard and complete selected asset fleet.
4. Retry the registered Home Hub process gate with the current 24-hour seed budget after the root-owned Hub cache is released.
5. Do not claim static GIS plugin activation: authenticated execution-target dependency closure and private activation handoff remain separate P0 work.
