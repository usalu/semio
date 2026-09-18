# Audit — Collaboration Between Users Over the Hub

Read-only audit, 2026-09-18. `repo`/`semio` MCP servers failed to connect (`CONNECTION_CLOSED`) for this
session, so ticket bookkeeping and `repo://goals` were not consulted; findings below come entirely from
reading the working tree and running one cargo test. Command captures are under
`🗑️generated/collab-*.txt` in this ticket folder.

## 0. Headline finding

This is **not a stub**. There is a real, hub-authoritative, event-sourced, no-CRDT replication stack
with a working WebSocket transport on both the hub server and the os client (native and browser-wasm),
schema-first binary framing with cross-language fixtures, reconnect/resume/backoff, and a real two-browser
Playwright E2E harness that has been run repeatedly against a live hub binary. The open item is not
"build the transport" — it is "finish wiring the browser UI to the already-working wire protocol": the
collab E2E was last measured at **2 of 8 steps** (ticket `26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END`,
still open) after a `SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION` ticket (also open)
extended the wire/presence model further (color assignment, per-window views) without re-running the
browser E2E to completion.

---

## 1. Intended architecture and where each stage lives

```
UI action → 🎯️action-bus / 🔀️dispatch → mutation → 🏪️store (canonical-edit ledger, local commit)
   → 🏪️store/🔄️sync ArtifactActor (per-document actor)
       → encodes ClientFrame (📡️replication/📡️wire) → WebSocket → 🌎️hub (🏗️bootstrap document_ws_v1/handle_ws)
       → hub decides (accept / transform / reject) against its authoritative event log
       → hub encodes ServerFrame → WebSocket → every other connected peer's ArtifactActor
   → peer decodes ServerFrame::Commands/Ack/Presence/Session → applies to its own store → UI re-renders
```

- **Local mutation path**: `🧰️framework/🔨️modules/🎯️action-bus`, `🔀️dispatch`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs` (the edit ledger/sealer —
  this is where the task's "editor engagement/edit ledger" concern actually lives; the standalone
  `🧰️framework/🔨️modules/✍️editor/🦀️.rs` is a small, generic framework utility, not the ledger).
- **Wire schema, schema-first**: authoritative Rust source is
  `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` — `pub enum ClientFrame` at line 48,
  `pub enum ServerFrame` at line 519, `pub enum ApplyOutcome` (Transformed/Rejected, no CRDT merge) at
  line 404. Cross-language fixtures (one folder per frame, used as the schema-first neutral vectors) sit
  under `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/📡️wire/` — 20 directories including
  `📸️server-welcome-snapshot-inline`, `✅️server-ack-accepted`, `⛔️server-ack-rejected`,
  `🔀️server-ack-transformed`, `🎫️server-credit-grant`, `🪪️server-session`, `👥️server-presence`,
  `🚫️legacy-client-hello-rejected`. There is a JSON schema root at
  `🧰️framework/🔨️modules/📡️replication/🧬️schema/🔣️.json`, plus per-concern schemas under
  `🎮️mutation/🧬️schema`, `🔗️causal/🧬️schema/🧮️document-backbone-batch-v1`,
  `📐️format/🔎️verification/🧬️schema`.
- **Who encodes/decodes which side**: both sides use the *same* `semio-framework-replication` crate.
  Hub depends on it directly (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:59`, `package = "semio-framework-replication"`).
  Hub decodes `ClientFrame`/encodes `ServerFrame` in `🌎️hub/🏗️bootstrap/🦀️.rs`. The os client decodes
  `ServerFrame`/encodes `ClientFrame` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` (imported as `crate::os_spr::wire::{ClientFrame, ServerFrame, ...}`,
  e.g. line 20's import list, `on_hub_frame` at line 2506 and its wasm twin at ~3693). There is no
  separate JS/TS re-implementation of the frame codec for the browser — the browser build compiles the
  *same Rust wire code* to `wasm32-unknown-unknown` and drives `web_sys::WebSocket` from inside wasm
  (see §2). The two TS "packages" under
  `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts` and
  `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` are both 2-line re-export barrels, not client implementations.
- **Directory-level (space/session listing) sync** is a parallel, simpler channel: HTTP for
  commands/paging, WS for live push, in
  `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` (see §3) against hub's
  `directory_ws_v1`/`handle_directory_ws_v1` (`🌎️hub/🏗️bootstrap/🦀️.rs:6207,6220,6346`).
- **Conflict handling**: server-authoritative sequencing, not CRDT merge. `ApplyOutcome::Transformed`
  (server rebases the client's op against intervening history and returns the transformed envelope) or
  `Rejected{reason,messages}` — `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:404-427`, exercised
  by `server_frame_ack_round_trips_for_every_stage_and_apply_outcome_variant` (passed, see §5). This
  matches `AGENTS.md`'s "MUST NOT use CRDTs" / "MUST use CQRS with event-sourcing" mandate.
- **A second, generic implementation exists and looks unwired**: `🧰️framework/🛍️products/🖥️server`
  (`semio-framework-server`) has its own axum gateway with a durable event lane and a document lane
  (`🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs:983` `handle_event_stream`, `:1067`
  `handle_document`). Its header states the intent explicitly: *"The document engine is a port, not a
  dependency... the server product deliberately does not depend on the os product"* (lines 17-20). No
  other crate in the repo depends on `semio-framework-server` except itself
  (`grep -rl semio-framework-server --include=Cargo.toml` returns only the root workspace listing and its
  own `Cargo.toml`). This is a domain-neutral gateway abstraction that has not yet been instantiated for
  the actual os↔hub collaboration flow, which instead runs on hub's own bespoke axum app in
  `🌎️hub/🏗️bootstrap/🦀️.rs`. Worth resolving (either wire hub onto it, or retire one) — see gap list §7.

---

## 2. Is there a working client↔hub transport today?

**Yes — real WebSockets, both directions, both native and browser-wasm builds.**

- **Hub server handler** (`🌎️hub/🏗️bootstrap/🦀️.rs`, 465 KB file, real `axum::serve` in `async fn main()`
  at line 8294, `axum::serve(...)` at line 8469):
  - Document socket: `document_ws_v1` (`WebSocketUpgrade` handler) at line 3832, dispatches to
    `handle_ws` at line 4082. Rebootstrap path `send_socket_document_rebootstrap` at line 3889.
  - Directory (space-listing/presence) socket: `directory_ws_v1` at line 6207,
    `directory_scoped_ws_v1` at line 6220, `handle_directory_ws_v1` at line 6346.
  - A prior legacy WS admission scheme was removed; `client_frame_tag_zero_is_terminally_rejected` test
    and hub integration test `"removed legacy WebSocket admission" / "terminally rejects the removed
    tag-zero bearer Hello frame"` (`🌎️hub/🧪️tests/🔬️bin-unit`, `🌎️hub/🧪️tests/🤝️integration/🟦️.ts:804-806`).
- **Browser client code location**: it is *inside* the os kernel crate, compiled to wasm, not a
  separate JS client. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`:
  - `mod wasm_actor` at line 3401, gated `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`
    (i.e. the real browser build, as distinct from `wasm32-wasip2` plugins) — doc comment at lines
    3394-3399: *"Browser wgpu build: the actor runs on `spawn_local` with a `web_sys::WebSocket`
    semio_hub transport."*
  - Native path (dev servers, native wgpu shell, CLI) uses `tokio_tungstenite::WebSocketStream` — type
    alias `WsStream` at line 1437, gated `#[cfg(not(target_arch = "wasm32"))]`.
  - Both paths converge on `on_hub_frame`/frame handling: native at line 2506, wasm twin at line 3693;
    both switch on the same `ServerFrame::{Welcome,SnapshotChunk,RebootstrapRequired,ArtifactBootstrapChunk,
    ArtifactBootstrapDone,Commands,Ack,Preview,Presence,Session,CreditGrant,Error}`.
  - **Does the os host ever call it?** Yes: `ArtifactHost::open` (line 1253) constructs an `ArtifactActor`
    per document; `ArtifactActor::start_connect_hub` (referenced at lines 1779-1837) drives the
    connect/backoff/reconnect state machine every drive tick. The React "wgpu" host wraps this in
    `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` and
    `🔗️AgentBridge/🟦️.tsx` (both matched the WebSocket/EventSource grep — they host/bridge the wasm
    module's messages into the React tree, they do not implement their own transport).
- **A genuinely separate, plain-browser JS/TS client does not exist.** No TS file under `🌎️hub` or
  `💻️os` opens a `new WebSocket(...)` against `/directory/socket/v1` or `/artifact/.../socket`; the only
  TS hits for "WebSocket" in the whole repo are re-export barrels, test probes, and vite-plugin dev
  tooling (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`), confirmed by the
  full-repo grep captured in `🗑️generated/collab-ws-grep.txt`.

---

## 3. Presence and identity

- **Identity/session bootstrap**: one-shot "broker proof" scheme.
  `🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json` — fixed TTLs
  (`bootstrapMaximumMs: 120000`, `activeMaximumMs: 15000`), and expected outcomes `unarmed:401,
  admitted:200, expired:401, replayed:401` (i.e. the proof is single-use and time-boxed; a replayed proof
  is rejected). Consumed client-side as the `#semio-broker=` one-shot URL fragment, per
  `🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️status.md:16` ("the one-shot `#semio-broker=` proof
  (read + stripped, React's own rule)").
- **Session mint / capability**: `SessionMintResponse`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:216`), `LocalHubCredential`
  (line 625, wiped on drop — `Drop` impls at 638/644/728), read from an inherited pipe fd via
  `LocalHubCredential::read_inherited` (line 735) for the native/local-bootstrap case, or minted over
  HTTP for the browser/broker case.
- **Socket admission (per-document)**: `issue_socket_grant`/`open_ws` trait methods on `DirectoryTransport`
  (lines 122-124); `DocumentSocketAdmissionV1`/`DocumentSocketExpectationV1` (lines 496-516);
  `admit_document_socket` (line 516) — a capability-scoped grant (`valid_socket_grant`/`valid_socket_actor`,
  lines 545-552, format `hub.v1.<64 lower-hex chars>`) is fetched over HTTP before the WS upgrade, then
  presented in the WS handshake (`SocketGrantAdmissionV1` consumed by `handle_ws`,
  `🌎️hub/🏗️bootstrap/🦀️.rs:4082`, and by `handle_directory_ws_v1`, line 6346). Schema for the grant's
  shape: `🌎️hub/🧬️schema/🧱️socket-grant-command-source/🔣️.json`.
- **Joining a shared session**: a client sends `ClientFrame::SocketHelloV1{wire_version, protocol_version,
  schema, pack_schema_hash, resume_token, frontier}` (constructed at
  `🏪️store/🔄️sync/🦀️.rs:2258`/2435) once connected; hub replies `ServerFrame::Welcome{session_id,
  resume_token, server_frontier, bootstrap}` (handled at line 2508/3693) — `bootstrap` carries either an
  inline snapshot or a chunked `ArtifactBootstrapChunk`/`ArtifactBootstrapDone` transfer (lines
  2542-2568, 3720-3747) for first-join / rebootstrap.
- **Peer roster / presence**: `ServerFrame::Presence{peers}` (line 2597/3... ) fans out
  `PresencePeer` (binary-coded, `presence_to_bytes`/decode, `🏪️store/🔄️sync/🦀️.rs:928-943`).
  Presence is scoped `(space, document, surface)` per the `26/08/17` final summary — proven at hub level
  by `presence_roster_is_scoped_per_surface` and a three-client test (two peers on the same surface see
  each other; a third on another surface does not, but still receives command relay). Session color
  assignment is hub-authoritative: `ServerFrame::Session{actor, color}` sent once per connection after
  `Welcome` (`🏪️store/🔄️sync/🦀️.rs:565-566, 2610, 3... `), lowest-free-index over a 12-hue palette,
  released on last disconnect — this is the subject of the still-open
  `SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION` ticket (§6).
- **Hub admin visibility**: a real admin SPA exists (`🌎️hub/🔨️modules/🛡️admin`, per the `26/08/17` final
  summary) served at `/admin`, showing live connections; not itself part of the collaboration data path.

---

## 4. Offline / short connection-shortage handling

`AGENTS.md`'s "MUST support short connection-shortages and not freeze the app... SHOULD NOT accept long
offline periods" is implemented concretely, not just aspirationally:

- **Exponential backoff with cap**: `ArtifactActor.backoff_ms` starts at 500 ms
  (`🏪️store/🔄️sync/🦀️.rs:1711`), doubles on failure and caps at 30 000 ms (`* 2).min(30_000)` at line
  2272), surfaced to callers as `RemoteState::Backoff{retry_in_ms}` (line 526/2270) so the UI can show a
  reconnecting state rather than freezing.
- **Reconnect resumes rather than replays from scratch**: `resume_token` is stored from the last
  `Welcome` and echoed in the next `SocketHelloV1` (doc comment lines 1608-1609); `server_frontier` /
  `RuntimeFrontierSummary` tracks the last acknowledged position so a reconnect only needs the delta
  (lines 1606, doc comment). `ArtifactDrivePhase::Reconnect` is a first-class state in the actor's drive
  loop (line 1790/1834-1837).
  `RebootstrapRequired{control}` (`ServerFrame`, handled at 2535/3713) is the hub's explicit signal when
  a resume is no longer possible (frontier too far behind / retention expired) — the client then
  re-runs full bootstrap instead of silently diverging. `🌎️hub/🛰️lag-rebootstrap/🦀️.rs` is the
  hub-side crate that decides when to force this.
- **Flow control**: `ClientFrame::CreditGrant{n}` / `ServerFrame::CreditGrant{n}`
  (`📡️wire/🦀️.rs:54,552`, decode at 884/973, wire tags at 914/1060) — a credit-based backpressure
  scheme so a slow/reconnecting peer doesn't get flooded once it comes back.
- **The app itself never blocks on the socket**: `ArtifactHost`/`ArtifactActor` run on the injected
  `WorkerPool` (native) or `spawn_local` (wasm) independent of the UI thread (module doc comment,
  `🏪️store/🔄️sync/🦀️.rs:1-14`); local edits keep committing to the local store/WAL regardless of
  connection state (`ArtifactDrivePhase::Folder`/local persistence path), and are relayed once
  reconnected (`relay_operations_to_hub`, lines 1896/1918).
- **Durable local log**: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs` +
  `🧬️schema/🔣️.json` back the local commit so a short outage cannot lose an edit that was already
  applied locally.
- **What's not proven**: none of this backoff/resume machinery has been exercised by the browser E2E
  past step 2 (see §5/§6) — it is unit/integration-tested at the Rust level (hub restart-survival test,
  §5) but "two real browser tabs survive a hub restart and reconcile" is only asserted at the *directory*
  layer (`collabRunRestartStep` in the dev collaboration harness, which restarts the hub mid-scenario),
  not yet at the *document* WS layer with concurrent edits in flight.

---

## 5. Tests covering multi-client replication

- **`semio-framework-replication --lib` (the wire protocol crate) — ran now, foreground:**
  **274 passed, 0 failed, 1.21 s.** Full output: `🗑️generated/collab-cargo-test-replication.txt`.
  Covers: every `ClientFrame`/`ServerFrame` variant round-trip (including `Ack` for all three apply
  outcomes, `Session`, `Presence`, `CreditGrant`, `SnapshotChunk/Done`, `ArtifactBootstrap*`), the legacy
  tag-zero rejection, presence binary codec (incl. hostile-count rejection), and the causal/local-interaction
  transport layers. This is real coverage of the schema-first wire contract, run against the actual
  `wire::codec`/`wire::frames` module, not a mock.
- **Hub-side, per the `26/08/17` final summary (not re-run in this session — cited from the ticket's
  own recorded log, `🧪️5-b-hub-test-all-features-1.txt` and siblings in that ticket folder):**
  `cargo test -p semio-hub --lib` — 11 passed / 0 failed; `cargo test -p semio-hub --bin os-hub` — 18
  passed / 0 failed; includes a **real integration test that boots the actual hub binary twice against
  one `OS_HUB_DATA` directory** to assert restart-survival of the event-sourced directory projection
  (lane 3-E), and the three-client presence-scoping test named above.
- **Hub `🤝️integration-harness`** (`🌎️hub/🤝️integration-harness/🟦️.ts`) spins up the *real compiled hub
  binary* (`startHub`, line 201; `resolveHubBinaryPath`, line 174) and polls it over real HTTP
  (`waitForHttpReady`, line 154) for its tests
  (`🌎️hub/🧪️tests/🤝️integration/🟦️.ts`, 52 KB, ~14 `describe`/`it` blocks). These are schema/contract
  tests (checkpoint hashing, socket-grant HMAC recomputation, trusted-catalog closure, chunk-CAS,
  lag-rebootstrap encoding) — **none of the `describe` blocks in this file drive two live document
  sockets against each other**; the closest is the removed-legacy-admission rejection test.
- **The actual two-user, two-browser, one-hub E2E lives in**
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (40 KB). It:
  spawns a real hub daemon (`collabStartHub`, line 136), two independent user dev servers on different
  ports (`collabStartUserDevServer`, line 220), drives **two real Playwright browser pages** through
  toolbar/dialog interactions (`collabRunScenario`, line 313), asserts row-appears-on-the-other-user's-screen
  behaviour (`collabWaitForNewRow`, line 279), and includes a **hub-restart-mid-scenario** step
  (`collabRunRestartStep`, line 544) — this is the file that most directly tests "does collaboration over
  the hub actually work" end to end, and its own harness is real (not a stub), but its *last recorded
  pass rate* (`26/08/17` ticket) was **2 of 8 steps**. Not re-run in this session (needs plugin
  prebuilds + two dev servers + Playwright — well past a 5-minute budget; the `26/08/17` ticket's own
  runs took multiple `smoke-run` iterations).
- Nothing was run beyond the one `cargo test -p semio-framework-replication --lib` in this session, per
  the "fastest test <5 min, foreground" instruction; hub/`os-kernel`/collaboration-E2E were all judged too
  slow or too stateful (spawns servers, browsers, plugin builds) for this budget and are reported from
  the tickets' own recorded logs instead, with the exact source ticket cited so the numbers are
  independently checkable.

---

## 6. Known gaps from tickets (`26/07`–`26/09`, keyword-matched folders)

Matched folders (grep for COLLAB/REPLICATION/RELAY/MULTI-USER/PRESENCE/SESSION/HUB/SYNC across
`26/07`, `26/08`, `26/09`): the two directly on point are both **still open**:

- **`26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END`** (`status: "open"`, description quoted in
  full in the ticket's own `🎫️ticket.json`). Its predecessor's `📓️final-summary.md` (97 lines, read in
  full) states plainly: browser E2E is **2/8 steps** (STEP 1 create-space-visible-to-other-user: PASS;
  STEP 7 admin API: PASS; STEPS 2-6, 8: FAIL with real recorded failure text). Current diagnosed blocker:
  a `plugin instance busy` / `readHistory: missing HistorySnapshot frame` retry storm in
  `PluginRuntime/🟦️component.tsx`, deliberately left unfixed because a concurrent peer session was
  mid-rewriting that exact file (correctly deferred rather than edited over someone else's in-flight
  work). Also open: Postgres/Neo4j directory backends never compiled (empty Cargo features, pre-existing
  since 2026-08-12); wgpu shell's collaboration wiring is unit-tested but **never observed running** (no
  native window was actually driven); auto-checkin/checkpoint-on-close/TouchArtifact ported to React only,
  nothing to hang them on in the native wgpu shell; 13/33 plugin crates still fail to build for wasm.
- **`26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`** (`status: "open"`).
  Explicitly scoped as "make collaboration observable end to end for dev user 1 and user 2 against one
  hub," extending the wire (`PresencePeer` flag byte → varint bitmask, `views: Vec<PresenceWindowView>`
  replacing single cursor/viewport, `ServerFrame::Session{actor,color}`, `CHANNEL_VERSION` bump to 12).
  Its own `📌️important.md` names live hazards: `PluginRuntime/🟦️component.tsx` and
  `🔌️plugin/🦀️component.rs` "mid-rewrite by peer sessions twice in this ticket family." Its latest
  lane report (`📓️p6-report.md`) shows work still landing piecemeal with a `sharedFileRequest` to
  another lane (a wgpu-side struct literal needs a new `color` field added or it will fail to compile)
  and two test results marked `PLACEHOLDER` pending a shared cargo lock — i.e. as of this ticket's last
  recorded state, the extended presence/color wire change was not yet fully green everywhere.
- **`26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`** is the ticket that started this
  thread (its folder holds the 30+ original lane reports; per `26/08/17`'s own ticket description its
  status was affected by an unrelated rename by another session, hence the `26/08/17` follow-up ticket
  exists to finish it).
- **`26/07/12/OS-VCS-HUB-CQRS-EVENT-SOURCING-REFACTOR`** — earlier foundational work establishing the
  event-sourced VCS/hub split; only a `verify-rust-only-vcs.md` note remains in that folder (no open
  blockers surfaced there for this audit's purposes).
- No ticket folder matched RELAY, MULTI-USER, or SYNC as a *primary* keyword in `26/09` besides the
  audit's own new ticket and unrelated camera/view-sync tickets (`*-CAMERA-AS-SESSION-ONLY-VIEW-ACTION*`,
  which are about per-window camera state, not hub collaboration).

---

## 7. Prioritized gap list for working collaboration over the hub

**P0 — nothing is P0.** There is a working transport (hub WS handlers, os client actor native+wasm,
274-test-covered wire codec, backoff/resume/credit-grant). No "no transport" gap exists.

**P1 — missing/incomplete stages (blocks a real two-tab-editing demo today):**

1. **`PluginRuntime` retry storm** — `readHistory: missing HistorySnapshot frame` / `plugin instance busy`
   loop blocking browser E2E steps 2-6/8. File:
   `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/PluginRuntime/🟦️component.tsx`
   (confirm current owner/state before touching — flagged mid-rewrite by concurrent sessions twice
   already per `26/08/17`'s `important.md`). This is the single highest-leverage fix: it is what stops
   the existing, otherwise-working wire protocol from reaching the plugin-hosted editor UI.
2. **wgpu (native) shell collaboration path never observed running** — compiled and unit-tested only.
   Needs an actual driven native window against a live hub to confirm the same
   `🏪️store/🔄️sync/🦀️.rs` actor that passes 274 unit tests also works when driven by the real native
   event loop, not just `#[cfg(test)]` harnesses.
3. **Presence/session-color wire extension (`SHARED-PRESENCE...` ticket) not fully landed** — a
   downstream wgpu struct literal (`presence_peer_rows_for_surface`,
   `🧱️elements/Shell/🧊️component.rs:316-326` per that ticket's own cross-lane note) needs the new
   `color` field or it fails to compile once picked up; two test runs were left `PLACEHOLDER`. Re-run
   `cargo test -p semio-framework-ui --lib --features wgpu presence` and
   `cargo check -p semio-framework-plugin` after confirming that lane's current state.
4. **Two independent server implementations of the document/event WS lane**
   (`🌎️hub/🏗️bootstrap/🦀️.rs` vs. `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs`), with
   the generic one (`semio-framework-server`) depended on by nothing else in the repo. Either wire hub
   onto the generic port (`DocumentAuthority`) to remove duplication, or explicitly document why hub
   stays bespoke — left ambiguous today.
5. **Postgres/Neo4j directory backends** (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`) — empty Cargo
   features, never compiled; blocks anything beyond a single-process/SQLite hub deployment and blocks
   `--all-features` CI.

**P2 — polish / hardening once P1 lands:**

6. Auto check-in / checkpoint-on-close / `TouchArtifact` ported to React only — no equivalent hook in the
   native wgpu shell's edit lifecycle.
7. 13/33 plugin crates still fail `wasm32-wasip2` build (crate-local bugs, individually attributed in
   `26/08/17/📓️w4-e-report.md`) — limits which artifact kinds can even be tested for collaboration in
   the browser.
8. No test exercises a *document-level* (not just directory-level) hub restart with concurrent in-flight
   edits from two clients — the existing `collabRunRestartStep` covers directory/space listing restart,
   not mid-document-edit reconnect/resume.
9. `verify gate`'s dependency-cruiser step has 828 pre-existing violations (baseline, not new) — outside
   collaboration scope but noted since it means `verify gate` cannot currently be used as a collaboration
   regression signal either.

**Proposed minimal end-to-end slice (two browser tabs editing one document via a local hub):**

1. Confirm/fix the `PluginRuntime` retry storm (P1.1) — this is very likely the only hard blocker between
   "wire protocol works" (proven, §5) and "two tabs visibly collaborate."
2. Run `collabRunScenario` (`🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`) end to end against a fresh
   `dev s 👤️1`/`👤️2` + local hub (ports already registered in `launch.json` per the `26/08/17` summary:
   `6072`/`6073` React, `6067`/`6068` wgpu, hub `8787`) and drive it to 8/8, fixing whatever the runner
   surfaces (it reports real failures, not skips — trustworthy signal).
3. Once 8/8, add one assertion the current scenario lacks: a live text/selection edit from user 1's tab
   appearing in user 2's tab within one `ServerFrame::Commands` round-trip, to directly exercise the
   `ArtifactActor` sync path (§1-§2) rather than only directory-level space/artifact creation.
4. Extend `collabRunRestartStep` (currently hub-restart-between-directory-actions) to restart the hub
   *while a document socket has an unacknowledged in-flight edit*, to prove the resume-token/frontier
   path (§4) under the exact "short connection shortage" condition `AGENTS.md` calls out.

---

### Appendix — command captures

- `🗑️generated/collab-cargo-test-replication.txt` — `cargo test -p semio-framework-replication --lib`
  (274 passed, 0 failed, 1.21 s), run in this session.
