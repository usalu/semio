# Dashboard Developer Control Plane

Run `bun run dashboard` from the repository. One workspace daemon owns the development processes; dashboard views can detach and reattach. The command chooser discovers Nx servers, builds, tests, renderer variants and examples. It supports output inspection, restart, cancellation, process-tree termination and workspace shutdown. Editor launch configurations are optional entry points.

The installed Bun 1.4.2 launches successfully. Bootstrap acquires and verifies the repository-pinned Bun 1.3.14 for its tools and children without replacing the global installation. English/German and native/reuse terminology are explicit selections and isolate each watcher namespace.

## Verified Runtime Behavior

| Check | Observed result |
| --- | --- |
| Default `bun run dashboard` after final execution refactor | Real language picker, command frame, live session status and clean detach; Nx exit 0 |
| Single workspace process owner | Real renderer and test sessions use the same daemon; current daemon PID 28656 |
| Concurrent servers, builds and tests | Native CLI suite verified real Nx dev/build/test, restart, child termination and port release |
| React Draw demo and demo-session | Actual Chrome application readiness, fresh process after restart, dashboard stop; both audits exit 0 |
| WebGPU browser Draw demo and demo-session | Actual Chrome/WebGPU application readiness without admitted console/page errors; restart and stop; both audits exit 0 |
| Native Draw demo smoke | Actual selected Draw editor session and window document, truthful boot report; target and audit exit 0 |
| Native Draw demo window | Actual GPU RuntimeReady and descendant HWND twice; managed root 16308 restarted to 30856; stop acknowledged and audit exit 0 |
| Build while a native executable runs | Actual changed Nx development/release publication succeeds; new outputs match independent Cargo output while the old process retains its original image |
| Native runtime consumers | Development/release cold, warm and restored Nx cycles; live asset HTTP, credential exclusion, exact bytes, page codecs and cancellation; exit 0 |

The native restart executes an immutable digest-addressed image. Dashboard, native window/smoke/scale and declared MCP hosts share cancellable publication acquisition. Running images no longer lock their mutable producer output. The native publication regression reproduced Windows EPERM before correction.

Native boot repairs cover event-thread surface capture, bounded retained I/O progress at the kernel worker boundary, local development components up to the producer's existing 256 MiB ceiling, and failure status when a selected smoke cannot mount an application. Network component admission retains its separate 64 MiB limit.

## Completed Contract Verification

| Contract | Passing evidence |
| --- | --- |
| Dashboard lifecycle/protocol | Earlier full 41-case native run, independent Ajv protocol vectors; later 44 Rust cases with separately exercised actual CLI paths |
| Bootstrap, pinned child runtime and watcher selections | 12 tests, 97 assertions through dashboard daemon |
| Immutable dashboard execution | 1 test, 5 expectations; independent Bun SHA-256 oracle |
| Native retained I/O, workers, surface capture, component bounds and failed smoke | 8 tests, 0 failures through dashboard-managed Nx task |
| Native foreground outcomes | 6 tests, 40 assertions including installed Nx failing-child oracle |
| Frame-worker source ownership | 6 tests, 92 assertions; independent esbuild and actual Bun bundling |
| Flow publication ownership | 4 owners, 17 consumers, 39 exact inputs; cold publication and cache restoration |
| Pinned Cargo loader policy | Actual large Cargo graph plus independent TOML/schema validation; current metadata policy and July 20 pin |
| Source launch/catalog contracts | 9 tests, 89 assertions; choices remain available while stale compiled descriptors are withheld |

Detailed red/green attempts, interrupted runs and runtime observations are retained in [the end-to-end record](./📓️end-to-end.md). [Control-plane architecture and shortcuts](./📓️control-plane.md) documents daemon ownership, local event replay, limits, reconnect and terminal controls.

## Verification Limits

Windows runtime behavior is verified. Native Linux/macOS runtime was not exercised; portable environment/path contracts were tested. Actual application lifecycle audits cover the Draw examples listed above. The entire monorepo cache suite and every product were not run. A separate outer `nx exec` diagnostic encountered a current workspace project cycle before assertions; the isolated native publication fixture ran actual Nx independently. No compatibility admission or global environment replacement was added.

## Finalization

The final current-channel retained-I/O run passed all 8 cases in 17.14 s through the shared daemon, including an actual booted=false smoke report. Repository MCP ticket_close succeeded with management integration disabled; persisted ticket status is closed. Generated outputs, including MCP diagnostics, are removed. Authored inputs and Markdown records are retained. No modifying Git commands, worktrees or AGENTS.md changes were used. Authored diagnostic inputs and Markdown records will be retained.
