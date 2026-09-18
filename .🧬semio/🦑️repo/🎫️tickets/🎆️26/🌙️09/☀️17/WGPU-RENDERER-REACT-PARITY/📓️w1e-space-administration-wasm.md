# W1e — Space Administration (and the directory/identity/check-in lane) on the browser wgpu build

Packet W1e of `26/09/17/WGPU-RENDERER-REACT-PARITY`, answering packet 2 of
`📓️audit-runtime-boot-input.md` ("Space administration is native-only — P0 missing on browser").
Date: 2026-09-17/18.

## The defect, restated

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` carried Space Administration, the directory command FIFO, identity
and the pure check-in reducers under `#[cfg(not(target_arch = "wasm32"))]` — not because the code was
platform-specific (every one of those regions is pure state machinery over `DirectoryClient<T>`), but
because the only transport wired was `os_directory::client::native::NativeDirectoryTransport` (ureq
over a `TokioHostRuntime`). The gate was in the wrong place: it belonged at `T`.

## Transport design

One seam, two implementations, everything above it compiled identically:

```
crate::directory_door (🎯️targets/🧊️wgpu/📇️directory-door/🦀️.rs)
  ShellDirectoryTransport = NativeDirectoryTransport<TokioHostRuntime>   (not wasm32)
  ShellDirectoryTransport = BrowserDoorDirectoryTransport                (wasm32)
  ShellDirectoryClient    = DirectoryClient<ShellDirectoryTransport>     (both)
```

`BrowserDoorDirectoryTransport` implements the kernel's existing `DirectoryTransport` trait. Its
`http` encodes one request/response mailbox message and hands it to the ONE door the wgpu shell
already uses for file open/download — `globalThis.semioWgpuHostIo` (`🚪️host-io/🟦️.ts`), whose page
install services it and whose Worker install is a `postMessage` bridge to that same page
implementation (`🎞️frame-worker/🟦️.ts` ⇄ `🚚️browser-frame-transport/🟦️.ts`). No new global, no second
bridge, no TypeScript dependency: the page half is one `fetch`.

Why the door and not `web_sys::fetch` directly (the kernel's own experimental
`os_directory::client::browser` transport): the wgpu shell runs inside the dedicated Worker that owns
the `OffscreenCanvas`. That isolate has no `window` and no `document`, so `web_sys::window()` is
`None` there — the same defect that made `Export Document…`/`Import Document…` silently no-op before
the door existed. Routing through the page is also what makes the same-site session cookie travel.

### Door wire (new `op` on the existing vocabulary)

| direction | JSON |
| --- | --- |
| request | `{"op":"directory-http","method":"GET"\|"POST"\|"DELETE","url":…,"bearer"?:…,"body"?:…}` |
| answer (hub replied) | `{"status":<u16>,"body":"<text>"}` |
| answer (fetch refused) | `{"error":"<message>"}` |

A refusal is never encoded as a synthetic status: a fetch that never reached the hub and a hub that
answered 5xx are different outcomes, and `DirectoryClient`'s own retry/terminal policy branches on
exactly that distinction (`TransportError::Io` vs `DirectoryCommandErrorCodeV1::from_status`).
Bodies are text because every payload on the frozen hub surface is JSON; a non-UTF-8 body is refused
at the encoder rather than lossily transcoded.

## Endpoints and payloads vs React

The Rust `DirectoryClient` (`📇️directory/🔌️client/🦀️.rs`) and the TypeScript `DirectoryClient`
(`💻️os/🟦️.ts:4560+`) already speak the same frozen surface; this packet only had to give the Rust one
a browser transport that sends the same *credentials* the TS one sends.

| lane | React (TS `DirectoryClient`) | wgpu browser (Rust client + door) | match |
| --- | --- | --- | --- |
| identity | `GET /auth/sessions/me`, `credentials: "include"`, canonical `DirectorySessionAuthorityV1` | same path via `DirectoryClient::me`; door sends `credentials: "include"` | ✅ |
| administration page | `GET /directory/spaces/{id}` / `…?cursor={cursor}`, raw canonical text preserved, 48 KiB cap, space-id match | `DirectoryClient::space_administration_page` — same path build, same cursor charset guard, same cap, same canonical parse | ✅ |
| command | `POST /directory/commands`, `content-type: application/json`, sealed `directoryCommandRequestJson`, receipt cap, closed error code from status | `DirectoryClient::command` → `request_bytes_limited` — same path, same body, same caps, same `from_status` mapping | ✅ |
| space list | `GET /directory/spaces` | `DirectoryClient::spaces` | ✅ (unused by this chrome) |
| event page | `GET /directory/event-page/v1?after=` | `DirectoryClient::event_page` | ✅ path, ❌ not driven on browser (see gaps) |
| request base | `requestBaseUrl: "/_semio/hub"` (same-origin reverse proxy) | `shell_directory_request_base_url()` → `"/_semio/hub"` on wasm32 | ✅ byte-identical literal |
| stream | `GET /directory/socket/v1?since=` (WebSocket) | refused by the door transport | ❌ gap |
| socket grants | `POST …/socket-grants` | refused by the door transport | ❌ gap |

Note: React's browser directory transport is a **WebSocket**, not EventSource — `streamFor`/
`directory_ws_url` on both sides. There is no EventSource anywhere in the directory lane.

## Gates removed

Region-level laws now hold (asserted by tests, see below):

| region in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | before | after |
| --- | --- | --- |
| `🏛️SpaceAdministration` (operation, phases, turns, capability gating, bilingual labels, controls) | 11 item gates | 0 |
| `🔖️CheckInPure` (auto-checkin policy, history fold, uncommitted count, viewer guard, detach gate) | 8 item gates | 0 |
| `🎮️DirectoryCommandQueue` (new region marker around the bounded FIFO) | 6 item gates | 0 |
| `📇️DirectoryLane` (new region: replay funnel, open/close/acknowledge/pump administration, contexts, dispatch + flush commands) | 9 item gates | 0 item gates (2 in-method branches remain, both named below) |

Also ungated / unified:

- `mint_shell_session_id`, `resolve_identity_env` (browser arm reads the `semioWgpuSetHubEnv` boot
  environment), `directory_command_from_action`.
- `ShellState` fields `identity`, `identity_offline`, `identity_env`, `shell_session_id`,
  `directory_client`, `directory_transport`, `directory_cancel`, `directory_commands`,
  `space_administration`, `space_administration_epoch`, and all seven check-in fields.
- `session_identity_view` was a cfg PAIR whose browser half returned a hard `None` — every guest saw
  an anonymous session on the browser renderer alone. It is now one accessor over the real identity.
- `refresh_history_snapshot` stays native (document attach), `observe_invocation_history`,
  `poll_auto_checkin`, `dispatch_checkpoint`, `handle_checkin_action` are now shared.
- Renamed `NativeDirectoryCommandQueueV1`/`NativeDirectoryCommandResultV1` →
  `ShellDirectoryCommandQueueV1`/`ShellDirectoryCommandResultV1` (the "Native" prefix is a lie now).

### New browser plumbing

- `ShellState::poll_browser_identity` — retry-floored (`BROWSER_IDENTITY_RETRY_MS = 5 s`)
  `GET /auth/sessions/me` through an unauthenticated client, exactly what React's
  `directoryClient.me()` does. There is nothing to `restore_claimed` on this target: the hub session
  is a same-origin cookie, not an inherited one-shot fd-3 credential.
- `ShellState::pump_directory_events` (wasm32) — identity → `pump_space_administration` →
  `poll_auto_checkin` → `flush_pending_directory_commands`, driven from the SAME
  `FrameDeferredWork::PumpSync` slot the native `pump_sync_events` uses (already scheduled every
  100 ms on both targets; the browser arm used to be an empty `cfg` branch).
- `ShellState::directory_now_ms` — the clock a directory deadline is measured against must be the one
  the linked transport compares it to: the shared `WorkerPool` millis natively, `Date.now()` in the
  browser.

### Two clock fixes this exposed (both real runtime panics, not compile errors)

`std::time::SystemTime::now()` has NO implementation on `wasm32-unknown-unknown` — it **panics**
rather than erroring, so `.map_or(0, …)` does not save it. Two functions on the newly-shared path hit
it, and both now read `Date.now()` on the browser:

- `os_identity::time_ordered_id` (`🔨️modules/🪪️identity/🦀️.rs`) — reached on EVERY browser boot the
  moment `mint_shell_session_id` was ungated, and by `mint_directory_command_request_id` on every
  directory command.
- `os_directory::client::wall_now_ms` (`🔨️modules/📇️directory/🔌️client/🦀️.rs`) — socket-grant and
  document-open expiry comparisons.

## Tests

`🎯️targets/🧊️wgpu/📇️directory-door/🧪️tests/🔬️unit/🦀️.rs` (new, mounted from the door module):

- 10 wire tests: op/verb/url encoding, bearer + sealed JSON body, `DELETE` spelling, non-UTF-8
  refusal, status+body decode, empty body, 5xx stays a response, fetch refusal → `TransportError`,
  statusless answer refused, unreadable answer refused, full round trip.
- 5 source laws over the shell file: no `cfg(not(target_arch = "wasm32"))` inside
  `🏛️SpaceAdministration`, `🔖️CheckInPure` or `🎮️DirectoryCommandQueue`; no method of `📇️DirectoryLane`
  gated out of the browser build without a wasm32 twin (it has exactly one gated item, the
  `directory_now_ms` clock, and that twin exists); and the six administration/command entry points are
  still declared in the lane.

## Verification

| command | result |
| --- | --- |
| `nx run @semio-tech/framework-renderer-wgpu:lint` (`NX_DAEMON=false`) | ✅ passed (color-literal + artifact-home, 2 checks) |
| `nx run …:generate-browser-boot` | ✅ regenerated; `🚀️browser-boot/🤖️generated/🟨️.js` now carries the `directory-http` door op |
| `nx run …:generate-frame-worker` + `…:check-frame-worker` | ✅ fresh (the frame-worker bundle is the postMessage bridge, so it carries no op vocabulary of its own) |
| `nx run …:check-browser-worker` | ✅ passed (both browser isolates bundle) |
| `nx run …:test-browser-worker` | 78 passed / 3 failed — all three are peer-owned and unrelated: `⏱️wgpu-ui-turn-budget` (diagnostics switch), `⏱️wgpu-worker-step-budget` (`phaseUs=` trace), `📨️browser-frame-transport` (`location.search.length` ordering, removed by the concurrent `🧭️boot-descriptor` refactor) |
| `cargo check … --target wasm32-unknown-unknown --keep-going` | **19 errors, ZERO of them this packet's** (`🗑️generated/w1e-wasm-2.txt`): 18 in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (`IconName`, `canvas_sat`/`canvas_set_sat`/`canvas_lum`/`canvas_set_lum` unresolved) and 1 in `🐚️Shell/…/🦀️.rs:15911` (`AgentApprovalPaintOp` match arms `()` vs `usize`) — both peer lanes' in-flight code. `grep` for every symbol this packet introduced (`directory_door`, `space_administration`, `poll_browser_identity`, `ShellDirectoryCommand*`, `directory_now_ms`, `shell_directory_*`) over that log returns 0 hits. The E0308 at Shell line 15911 is a TYPE error, which proves type-checking reached the end of the shell module rather than aborting during name resolution — so the newly un-gated regions were really checked, not skipped. |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` (native) | **18 errors, ZERO of them this packet's** (`🗑️generated/w1e-native-5.txt`) — all 18 in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (`IconName`, `canvas_*`, `InkDocumentJson.grid_*`), the same peer lane. `grep` for this packet's symbols returns 0 hits. The Shell file is clean on native. |
| `cargo test -p semio-framework-os-renderer-wgpu --lib directory_door` | ✅ **`test result: ok. 16 passed; 0 failed`** (`🗑️generated/w1e-tests.txt`) — 11 wire tests + 5 source laws. It took three attempts: the lib-test binary was blocked twice by peer call sites that had fallen behind their own lane's signatures (listed under Repairs below), which this packet caught up. |

⚠️ The shared cargo build-dir was saturated during this packet: up to 21 concurrent
`cargo check -p semio-framework-os-renderer-wgpu` invocations from peer agents, and one confirmed
`prebuild_lock_exclusive` flock deadlock (sampled: the lock holder itself blocked in `flock`, zero
`rustc` processes system-wide) that SIGKILLed a first run at exit 137.

## Remaining gaps (honest)

1. **No directory WebSocket on the browser door.** `open_ws`/`issue_socket_grant` refuse. The
   retained Home projection (`DirectoryHomeProjection`, `ShellDirectoryRunner`, event-page bootstrap,
   `DirectoryStreamMessage::Presence`) therefore stays native-only, and so does live re-bootstrap on
   `RebootstrapRequired`. Space Administration does not need it (it is pure REST: fetch page, post
   command, refresh page), but "directory presence" in the audit's sense does. Wiring it needs a
   second, streaming door — the request/response mailbox cannot carry a duplex channel.
2. **Document sync stays native.** `ArtifactHost`, `sync_channel`, `presence_peers`/`presence_surface`
   and the `os.open-artifact` opening relay all end in the native document backbone; the browser
   build links none of it. Two in-method branches in the shared lane name exactly this
   (`handle_replay_shell_command`'s relay tail, `observe_invocation_history`'s space-index touch).
3. **No pane yet.** This packet makes the operation and its control set (`shell_space_administration_controls`)
   live on the browser; it does not add a wgpu Space-Administration *window*. The controls builder is
   the same one the native build renders from, so the pane is a chrome packet, not a transport one.
4. **`should_checkpoint_before_detach` is unused on wasm32** (its only caller,
   `checkpoint_before_detach`, needs the native sync channel), so the browser build emits one
   `dead_code` warning for it. Gating it would break the `🔖️CheckInPure` zero-gate law; it stops being
   dead the moment a browser document-attach path exists.
5. **Peer-owned failures seen in passing**, recorded so they are not misread as this packet's:
   three `test-browser-worker` cases (listed under Verification); the 18 `🎞️Scenes` compile errors on
   both targets (the peer's own lane fixed the `IconName`/`canvas_*` half while this packet ran); and,
   transiently, a
   `check-browser-worker` "import is not schema-owned" refusal of `🧭️boot-descriptor/🟦️.ts` that had
   cleared by the time the bundles were regenerated.

## Repairs to code this packet does not own

Every one of these was a hard blocker for verifying this packet at all, and every one is a mechanical
catch-up of a call site to a signature its own lane had already changed — none changes anyone's
intent:

- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `agent_approvals_paint_step`: the `AgentApprovalPaintOp::Modal` arm
  returned `push_glass`'s `usize` while every sibling arm is `()`. Wrapped in a block so the value is
  dropped. Without this the whole crate failed to compile on native.
- `🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` `UiSurfaceToken::new`: gated `#[cfg(test)]` while its only
  non-test caller, `📌️mounted_layout`'s `layout_tree_now`, is gated
  `#[cfg(any(test, feature = "testkit"))]`. Widened to match. Without this EVERY downstream
  `cargo test` that enables the `testkit` feature fails to build `semio-framework-ui`.
- `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` `InkDocumentJson`: the file's own `render_ink_*` reads
  `grid_visible`/`grid_spacing`/`grid_subdivisions`/`grid_opacity`, which the struct did not declare.
  Added as `Option<…>` beside `snap_enabled`/`snap_grid_spacing`, `None` in `Default` — the shape the
  neighbouring optional fields already use.
- `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs` (2 call sites): `layout_stack_cap` gained an `action_count: usize`
  parameter; passed `0`.
- `🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs`: `render_canvas_shape_fill` gained a `backdrop: Rgba`
  parameter; passed `Theme::default().canvas_clear`, the value its production call site passes.
- `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`: `DslValue::as_array` now yields a slice, so
  `.map(Vec::len)` no longer resolves; `.map(<[_]>::len)`.
