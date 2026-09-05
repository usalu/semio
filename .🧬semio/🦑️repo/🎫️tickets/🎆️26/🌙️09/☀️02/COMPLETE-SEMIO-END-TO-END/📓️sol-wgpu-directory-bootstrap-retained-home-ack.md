# WGPU Directory Bootstrap Retained-Home ACK Implementation

Status: source implementation and registered language-neutral proof are green; native and real two-session process proof remain pending the shared Rust execution lease.

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

The laws use a fake canonical page transport that records exact request URLs and original page bytes, an explicit delayed terminal Config-receipt boundary, all five forged ACKs, retained Home/visible-app separation, matching rejection retry, live dirty wake, rebootstrap, stale-epoch denial, and one-use destruction authority.

## Permanent gates

- `@semio-tech/framework-renderer-wgpu:directory-retained-home-bootstrap-source-check`
- `@semio-tech/framework-renderer-wgpu:directory-retained-home-bootstrap-native-check`
- launch entries `⚖️gate🧊️wgpu📇️retained-home-bootstrap-source` and `⚖️gate🧊️wgpu📇️retained-home-bootstrap-native`

A process target is deliberately not registered yet: the acceptance process law must run the real two-authenticated-session hub plus WGPU/Home component path and inject an event after the terminal page response but before socket dial. Rebranding the deterministic fake-transport native law as a process gate would be false evidence.

## Evidence

- Initial registered canonical Nx source session `23561`: 30 checks, exit 0.
- Final registered canonical Nx source session `e21b50`: 32 checks including terminal-close fencing, exit 0.
- Shared OS-kernel neutral bootstrap oracle remains green at 11 checks after the trace extension.
- Concurrent canonical plugin-registry generation included both ordered WGPU launch entries; registered freshness session `54523` exits 0.
- Fixture schema, fixture, and WGPU `project.json` parse successfully.
- Focused source diff passes `git diff --check`.
- Native selectors are staged but have not executed. No native assertion or WGPU process behavior is claimed yet.

## Honest remaining boundary

The real native laws, all-feature compilation, and the two-session real process timing law remain required before this packet is green. The previously completed hub event-page packet also remains source-green/native-process-pending under the same shared Rust execution constraint.
