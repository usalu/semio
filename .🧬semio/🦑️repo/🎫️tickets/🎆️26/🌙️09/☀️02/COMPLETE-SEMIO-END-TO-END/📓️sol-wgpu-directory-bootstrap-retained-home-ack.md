# WGPU Directory Bootstrap Retained-Home ACK Implementation

Status: source implementation and registered language-neutral proof are green. The former document-socket and stdio taxonomy blockers are repaired. The superseded orphan native build was deliberately cancelled after exact process-tree verification; no current-tree native verdict is claimed. The real two-session process proof is owned by the coordinated Home process packet.

## Implemented boundary

- `ShellState` now owns one private `DirectoryHomeProjection` for the shell lifetime. It retains the landing Home plugin/app/instance address, the shared `DirectoryEventPageBootstrapV1`, one cancellable page task, one cancellable Home-publication task, one acknowledged stream runner, the retry deadline, and one-use destruction authority.
- Boot shares the exact retained Home instance with the visible landing session. Managed/app switches preserve the hidden Home view and reuse the same instance when returning Home. Hot reload and drop cancel page/publication/stream ownership, clear the pending canonical page through `bootstrap.close()`, and consume the Home destruction authority at most once.
- Authenticated identity completion begins a new epoch at raw cursor zero. Page I/O uses `DirectoryClient::event_page` on the renderer worker pool with a child cancel token and five-second request deadline.
- Page publication invokes `applyDirectoryEventPage` directly on the retained Home address. It does not route through the visible session or the generic shell action helper.
- ACK is constructed only from the exact `semio.space.home.directory-projection-receipt.v1` `PublishEvent` emitted after the Home Config dispatch completed. Unknown fields, duplicate receipts, non-receipt effects, document mutations, or any mismatch in epoch, binding, authorization generation, through frontier, or receipt are terminal and cannot advance the bootstrap.
- Final ACK creates `stream_acknowledged(since)` only. The deleted global path no longer calls `client.stream(0)`, `foldDirectoryEvents`, or a visible-session raw event dispatch.
- Live event and heartbeat messages coalesce into one stream close and refetch at the last Home-ACKed raw cursor. `RebootstrapRequired` closes the old owner, increments the epoch, and fetches at zero. Late task results are fenced by epoch and retained instance identity.
- A non-cancelled terminal authenticated-stream turn is latched once by the I/O runner. The UI consumes that latch, retires the old stream, increments the Home epoch at raw cursor zero, clears the stale identity/client, and starts fresh identity bootstrap. A 4401 therefore cannot strand Home or enter a reconnect storm.
- Retryable transport/deadline/5xx failures and a matching Home rejection retain the acknowledged cursor and use one bounded 50 ms frame-driven retry deadline. Cancellation, hot reload, identity replacement, and close emit no ACK.

## Schema-first and independent proof

The existing language-neutral bootstrap trace/schema now also owns:

- the exact five independently forgeable ACK fields;
- retained Home versus unrelated visible instance identity and one destruction count;
- identical-cursor transport/Home retry;
- late-result cancellation with zero ACK;
- a strictly newer rebootstrap epoch at cursor zero.
- terminal close 4401 producing exactly one newer epoch at cursor zero and zero reconnects.

The owning WGPU `📜️script.ts` validates the trace with AJV 2020 and independently executes the ordering state machine in Bun. It also audits the native production cutover markers and rejects reintroduction of `client.stream(0)` or the raw event-fold path.

## Exact laws

Registered native selectors:

1. `shell::command_registry_tests::directory_home_bootstrap_waits_for_terminal_config_ack_and_retains_home_across_visibility`
2. `shell::command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss`
3. `shell::command_registry_tests::directory_home_terminal_receipt_rejects_unknown_fields_and_nonreceipt_effects`

The laws use a fake canonical page transport that records exact request URLs and original page bytes, a fake retained Home bridge that derives the terminal receipt from the exact published page, all five forged ACKs, retained Home/visible-app separation, matching rejection retry, and one-use destruction authority. Page two is not even requested until page one's terminal receipt is accepted. The controller law also drives a real acknowledged stream through unscoped close code 4401, proves the terminal stream cannot reconnect, advances to epoch 9 at raw cursor zero, denies the pre-terminal receipt, coalesces duplicate live wakeups, cancels both pending page and Home-publication tasks, removes their receivers, and proves both late senders have no surviving ACK path.

## Permanent gates

- `@semio-tech/framework-renderer-wgpu:directory-retained-home-bootstrap-source-check`
- `@semio-tech/framework-renderer-wgpu:directory-retained-home-bootstrap-native-check`
- launch entries `⚖️gate🧊️wgpu📇️retained-home-bootstrap-source` and `⚖️gate🧊️wgpu📇️retained-home-bootstrap-native`

A process target is deliberately not registered yet: the acceptance process law must run the real two-authenticated-session hub plus WGPU/Home component path and inject an event after the terminal page response but before socket dial. Rebranding the deterministic fake-transport native law as a process gate would be false evidence.

## Evidence

- Initial registered canonical Nx source session `23561`: 30 checks, exit 0.
- Strengthened registered canonical Nx source session `09c87d`: 33 checks, exit 0.
- Final registered canonical Nx source completion `b76a3a`: 39 checks including exact delayed page ordering, live-wake coalescing, late publication denial, real native-runner cancellation, and terminal-close fencing past every reconnect deadline, exit 0.
- Shared OS-kernel neutral bootstrap oracle remains green at 11 checks after the trace extension.
- Coordinated plugin-registry generation refreshed 59 plugin crates, 60 playgrounds, and 45 framework packages; generated launch contains the WGPU source/native gates at lines 4062/4069 and retains `SEMIO_BUILD_BUDGET_MS: 86400000` at line 4076. The direct registered Nx freshness command `NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache` then returned `plugin registry generated catalog and launch bytes are fresh.`
- Fixture schema, fixture, and WGPU `project.json` parse successfully.
- Focused source diff passes `git diff --check`.
- The first exact native attempt, receipt `exact-cargo-laws-taCqig/00`, was killed by the repository's 20-minute build budget after compiling 43 cold dependencies; its `build.json` records `status: null`, `signal: SIGKILL`, and `reason: timeout`, so it is not a compiler or law verdict.
- The cache-preserving replacement receipt `exact-cargo-laws-Mw4OqK/00` ran with the permanent launch budget of 86,400,000 ms and reached OS-kernel compilation. Its terminal `build.json` records `status: 101`, `signal: null`, and `reason: exit`. All seven parsed compiler errors are in the concurrently owned document-socket surface taxonomy: five E0425 references to removed `DocumentSocketSurfaceExpectationV1`, one E0560 use of removed `DocumentSocketExpectationV1.surface`, and one E0599 use of removed `DocumentSocketAuthorityV1.matches_surface`. The WGPU crate emitted no diagnostic and none of the three laws reached discovery, so this receipt is an upstream blocker rather than a WGPU verdict.
- The cache-preserving receipt `wgpu-directory-retained-home-exact/exact-cargo-laws-27xmIe/00` terminated RED before law discovery after about 1 hour 40 minutes. Its two errors were generated stdio registry includes for nonexistent `🌳️pdf` paths; current registry source was already concurrently repaired to the real existing `📖️pdf` paths when the receipt was inspected. This is an upstream frozen-snapshot build failure, not a retained-Home law verdict.
- The immediate cache-preserving rerun `wgpu-directory-retained-home-exact/exact-cargo-laws-MwnzEe/00` was orphaned by the fleet reset. Its receipt contains only `build.stdout`/`build.stderr`, no terminal `build.json` or law receipt, and its process no longer exists; no verdict is claimed.
- After re-auditing the process table and receipt, the registered gate was restarted against the preserved target and repaired then-current tree as `wgpu-directory-retained-home-exact/exact-cargo-laws-gSGgcg/00`, with an 86,400,000 ms inner build budget. The outer repo launcher nevertheless retained its default 14,400,000 ms command budget and terminated with `spawnSync node ETIMEDOUT`, exit 1. The receipt still has only `build.stdout`/`build.stderr`, no terminal `build.json` or law receipt. Cargo PID 28150 was orphaned to PPID 1 with Stdio rustc child PID 36825. Once the current preview and Store tree superseded that snapshot, the exact process group was verified (`PGID 28150`, dedicated `wgpu-directory-retained-home-sol-target`) and gracefully terminated with `SIGTERM`; both exact PIDs then disappeared. This is a cancelled, unqualified attempt, not a Cargo failure or native-law verdict. The seed now also sets `SEMIO_TEST_ORCHESTRATION_BUDGET_MS=86700000` and `SEMIO_CMD_BUDGET_MS=86700000`; coordinated plugin-registry generation and `check-generated --skip-nx-cache` both exited 0, and generated launch line 4076 preserves all three budgets.
- A fresh current-tree registered source rerun remains green at 39 checks. The cancelled orphan's last Cargo diagnostics were warnings only, but source moved after its snapshot, so it could not qualify the current tree.
- After the shared GIS inference page gained an optional floating-point preview, the WGPU test page constructor was updated with `preview: None`; the remaining WGPU status constructor uses `..Default`, and no WGPU equality-derived type embeds the status DTO. The current-tree registered source gate remains green at 39 checks after that parity edit. The already-running native process predates this edit and therefore cannot qualify the new source snapshot even if it eventually terminates green.

## Honest remaining boundary

A current-tree exact native attempt and the two-session real process timing law remain required before this packet is fully green.
