# G7w — wgpu native shell: two users collaborating over a real hub

Worker G7w (session 10, 2026-09-24, started 17:45). Gap: `📓️g11-unowned-gaps-after-session-10.md` §C G7,
`📓️g10-goal-gap-reaudit.md` outcome 3 last row ("wgpu native shell's collaboration path observed running: never").
Constraints: native only (no wasm32), no hub build, freeze on stdio/gis/kernel pack+store/framework plugin crates,
no edits to `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (peer N2), hubs 7900–7909, private target `wp-g7w/target`.

Status: **done for this slice (18:50)** — the native two-user harness exists, is registered and runs end to end against
a real hub; **steps 1–3 (two users, one space, both rosters) are observed live through two wgpu shells**; three native
host defects that kept every real guest out of the native shell were fixed; the run now stops at a fourth, deeper
native-kernel defect (step 4) and, behind it, at two structural gaps (steps 5–6) that the freeze / W1 own. Nothing
after step 3 is claimed.

## 1. Inherited state (measured 17:45–18:01)

| claim | how measured | result |
|---|---|---|
| no wgpu collaboration run ever existed | g10 §D outcome 3 last row, g11 §C G7; `grep` over `🐚️Shell/🧪️tests/*` | no test builds two `ShellState`s, none opens a real guest on a hub binding; WG6's live law is one user, no document |
| staged `os-hub:build-dev` binary | `ls 🌎️hub/📦️packages/🦀️rust/dist/build-dev/` | **absent** (only a hidden subdir) — WG6's crashing binary is gone; the verb's zero-touch boot path has nothing to boot today |
| newest hub binary that boots | `.tmp-ticket/wp-h7/target/debug/os-hub` (17:13, current tree, H7 runs it on 7913) | copied to `wp-g7w/bin/os-hub` (sha256 `dfcf87a5c8e05174…`), booted on **7900** (§7) — boots, `publicSessionIssuance: true`, only `artifactAuthority` closed (`trusted-catalog-never-published-in-this-data-root`), `features.openPlan: false` |
| any on-disk catalog with a block/dag package | `trusted-catalog/current.json` of all 19 roots under `.🧬semio/🌐hub/` + W1 §4.2/§4.6 | **none**; catalog A (stdio,gis,note,draw,writer,puzzle) still in W1's run 7; H7 found no on-disk catalog boots against the current tree |
| completed native-hostable guests | `dist/component-dev/*.wasm` headers + JSON descriptors `🔌️plugin/…/dist/dev/🔌️plugin-modules/*/🔣️.json` | block (`e676aeb2…`) and dag (`c5049b92…`) component-dev wasm hashes == their descriptors' `wasmSha256`; dag's apps declare `artifactSchema: ""` (cannot bind any document), block declares `block.2d/3d/5d` → **block2d chosen** |
| native runtime for block2d | `bun ⌨️native-entrypoint/📦️modules/📜️script.ts publish block2d dev` (18:01) | `Published native block2d dev: 1 completed components` — `dist/runtime/native/dev/block2d/🔣️runtime.json` references the component-dev wasm + descriptor (no compilation) |

## 2. Harness design

Same shape as WG6's live journey, no browser: a `#[ignore]`d live law in
`🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` (already mounted by the shell target; the shell
file itself is untouched — peer N2), driven by a new renderer verb `hub-live-collaboration-check`.

- **Two contexts** = two real `ShellState`s in one process, each with its own native `DirectoryTransport`,
  its own `ArtifactHost` document backbone and its own guest instance on the shared native kernel thread
  (`KernelClient`), loaded from the staged native runtime by the product loader
  `program_bridge::load_wasm_plugins` (the same call `semio-wgpu-native` makes).
- **Two hub users** = two principals provisioned with `os-hub credential set` (Ada, Bo), each signed in
  through the shell's own hub lane (`os.openHub` → `hubSetAddress`/`hubAddConnection` → `hubSetEmail`/
  `hubSetPassword`/`hubSignIn`).
- **Connection loss** = actor B reaches the hub ONLY through an in-test loopback TCP relay
  (`SeverableRelay`); `sever()` shuts every live B socket (document + directory) down at the TCP level,
  `heal()` re-admits. A's connections are untouched.
- **Measured through shell state only**: hub-lane session phase/user ids, `hub_workspace.rows/members`
  (the retained workspace the a11y mirror projects), `session` (guest mounted), `sync_channel`/
  `presence_surface` (hub binding), `sync_status.remote` + `hub_documents` (the pill's projection),
  `presence_peers`, `history_entries` (edit ledger, read back from the guest via `read_history`).
- **Ledgered**: each of 12 steps records PASS/FAIL + what the shells showed, then the law asserts the whole
  ledger — one run is the per-step table even when step 6 fails.

## 3. Per-step observed table

Hub **7900** (`wp-g7w/bin/os-hub` = H7's 17:13 binary, fresh data root, two principals via `os-hub credential set`:
Ada `01a0d421-3e4e-…`, Bo `01a0d421-46c0-…`), native runtime `block2d` release, law
`shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document`. Last full run = the
registered verb itself (`bun ./📜️script.ts hub-live-collaboration-check`, 18:37, rc=1, `wp-g7w/generated/g7w-verb-collaboration.txt`).

| # | step | result | what the shells showed (last run) | first run that got here |
|---|---|---|---|---|
| 1 | both shells sign in as different hub users (B only through the severable relay) | **PASS** | A `signed-in` Ada, B `signed-in` Bo, both `error=None`; hub trace: 2× `server.auth.session.mint` ok + 2× `session.read` | run 3 (18:17) |
| 2 | A creates a space, seats B as author; B lists it with that role | **PASS** | A's `os.directory.upsert-member` relay → hub `server.directory.command upsert-member ok`; B's `hubRefreshSpaces` row `("g7w collaboration …", Some(Author))` | run 3 |
| 3 | both open the space; each roster lists both members | **PASS** | A and B rosters `[("Ada Lovelace", false), ("Bo Peep", false)]` — both members, **neither online** (online = document presence, step 7) | run 3 |
| 4 | both mount the block2d guest natively via `os.open-artifact` | **FAIL** | A: session `("block", "s.block.block2d@1/*#editor", 1)` — the release component compiled and instantiated on the native kernel, 4 retained surfaces rendered (`block2d-board`, `…artifact`, `…inspector`, `…history`), then `app catalogue fetch failed: kernel: shard produced no outcome for this turn` and every later event: `guest trapped: owned turn is mid-flight and cannot admit 1 more event(s) — resume it with no events until it settles`; B: `could not switch to block: kernel: shard produced no outcome for this turn` (shared kernel) | run 6 (18:29) |
| 5 | both bind the hub document (sync channel with the hub binding) | FAIL (blocked by 4) | A `presence_surface=Some("s.block.block2d@1/*#editor")` (default bindings computed) but `plugin document-backbone bind: kernel: shard produced no outcome` → no channel | run 6 |
| 6 | document socket Live on both | FAIL | never left `none`; **measured independently of 4**: `document_codec("block.2d") = None` in the renderer process (§5 B2) | run 3 |
| 7 | presence shows both (roster `online`) | FAIL (needs 6) | 0 presence peers, roster online flags false | — |
| 8 | A authors `addHandleKind` | FAIL (needs 4) | `Err("kernel: shard produced no outcome for this turn")`, ledger empty | — |
| 9 | B ingests A's edit | FAIL (needs 6) | — | — |
| 10 | B authors, A ingests | FAIL (needs 4, 6) | — | — |
| 11 | per-actor undo | FAIL (needs 4, 6) | `undo` → same kernel error | — |
| 12 | sever B's network, no freeze, heal, catch up | **partial** — the no-freeze half is observed, the catch-up half needs 6 | relay severed **2** live B sockets; B's local verb returned in **12.9 µs**; 3 s of frame pumps took **3.005 s** (no stall); B's space list went **`stale`** while severed and back to **`ready`** after heal; `relive=None` (never Live to begin with) | run 3 |

Unit of "observed": every cell is a field of the real `ShellState`s (hub lane session, `hub_workspace.rows/members`,
`session`, `presence_surface`, `sync_status`, `hub_documents`, `presence_peers`, `history_entries`), printed by the
law as `g7w-live step=…` and cross-checked with the hub's own trace sink (`g7w-hub-7900-trace.txt`).

## 4. Measured vs unverified

| claim | status |
|---|---|
| two native wgpu shells, two hub users, one space, both rosters, through each shell's own lanes over the real network | **observed** (steps 1–3, 6 runs + the registered verb) |
| B's network can be cut at the TCP level and healed while A is untouched; B's shell keeps working (µs-level local dispatch, frame pump unaffected) and its space list goes stale → ready | **observed** (step 12, first half) |
| the native runtime loader now loads a real 377 KB descriptor; the native kernel now compiles + instantiates a real 17.6 MB guest and renders its retained surfaces | **observed** (runs 5–9: session `("block", …, 1)`, 4 surfaces rendered) |
| empty-turn `ui_patches` no longer fault the native kernel | **observed** (run 5 error `invalid turn patch transport token` gone from run 6 on) |
| WG6's journey still green after the `withLiveHub` refactor | **observed** (`hub-live-journey-check`, 1 passed, `g7w-verb-journey.txt`) |
| document socket / presence / edit crossing / per-actor undo / reconnect catch-up on the wgpu path | **unverified — blocked** (§5 B1–B3); the law encodes them and fails at them today |
| `semio-wgpu-native --smoke` hangs the same way the law did before `drive` (renderer I/O is only pumped by the GPU present loop) | **inferred, not run** — same code path (`run_smoke` → `load_wasm_plugins` → `run_renderer_io`) as the observed hang in run 1 (`g7w-live-1-sample.txt`: test thread parked in `block_on(load_wasm_plugins)`, 0 % CPU for 8 min) |
| no wasm32 build, no hub build, no guest rebuild | held |

## 5. Blockers found (root causes, owners)

### Fixed in this slice (renderer crate only; none frozen, shell target untouched)

| # | defect (measured) | root cause | fix |
|---|---|---|---|
| F1 | law parked forever at 0 % CPU in `load_wasm_plugins` (run 1) | renderer native I/O slots are pumped only by `present_step_inner` (the GPU present loop); a headless caller has no pump | harness `drive()` pumps `pump_renderer_io_sessions` + `pump_worker_job_retirements` between polls, exactly what the present loop pumps. Product note: `run_smoke`/socket-grant probe share this gap (§4) |
| F2 | `native I/O populated read exceeds the mounted one-page consumer authority` (run 2) | `load_wasm_plugins`/`read_descriptor_manifest` used `ReadBytes`, which answers at most ONE 16 KiB page; every real descriptor is 300–400 KB, so **no native runtime could load any real plugin** | `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`: `read_native_json_pages` (consecutive `ReadPage`, bounded: manifest 1 MiB, descriptor `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`) + `close_native_json_pages`; borrowed pages feed the existing `NativeJsonPages` reader |
| F3 | same fault at `create_app` (run 4) | the kernel's `create_app` read the component with one `ReadBytes` and required `single_page()` — **no native shell could compile any real guest** | `🧊️renderer/🦀️.rs` `kernel_runtime::read_native_component`: paged read into the compiler's one contiguous input, bounded by `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES` (64 MiB, the bound the hub enforces). Consequence: the 88 MB **dev** component is refused by that bound → the harness stages the **release** component (17.6 MB) |
| F4 | `kernel: decode ui patch transport: invalid turn patch transport token` (run 5) | the plugin-host bridge (`🧵️shard` `to_actor_turn_result`) sends EMPTY `ui_patches` for a turn that painted nothing; the renderer decoded every turn as a token | `decode_actor_turn_result`: empty bytes = `UiTurnPatches::default()` |

### Open (not fixable inside this slice's constraints)

- **B1 — native kernel leaves the first guest turn mid-flight (step 4).** After the boot renders, the next turn
  (`refresh_app_catalogue` → `render_with_document(framework.section.catalogue)`) returns from
  `KernelThreadState::run_turn` with no outcome for the actor: the tick grants nothing (`decision.run` empty) and
  `turn_result` stays `None` (`🧊️renderer/🦀️.rs` `run_turn`, the `None => Err("kernel: shard produced no outcome for this
  turn")` arm). It is **not** the 5 s `RUN_TURN_OUTCOME_TIMEOUT`: raised to 120 s as an experiment (run 9, reverted) the
  same error came within seconds. Every later event is refused by the guest ("owned turn is mid-flight … resume it with
  no events until it settles") — the host never resumes an in-flight owned turn with an empty event batch before
  admitting new events. Same family as C7 root cause #4 (reserved/interactive tool turns are jobs the actor lane must
  drive) and memory "Wasm Pool Pump Starves Interactive Jobs". Owner: the renderer kernel-runtime (terra-kernel-loop)
  owner; next step: log the actor's turn status sequence around the catalogue turn (`MoreWork`/`Job` publication) and
  resume with `Payload::JobStep`/empty `Event::Wake` until `Idle` before submitting new events.
- **B2 — native document actor needs a LINKED codec (step 6), measured.** `store/sync` native actor's
  `start_connect_hub` reads `document_codec(schema)` for the `pack_schema_hash` before it even asks for an open plan,
  and `install_artifact_bootstrap` decodes the bootstrap with `codec.print_mirror`. The renderer process links no guest
  codec (`document_codec("block.2d") = None`, measured in every run), so for **every guest-owned kind** the actor loops
  `Backoff` forever and never dials. Fix design = the same as §8's browser actor (kind identity from the verified
  execution-target lease, bootstrap validation delegated to the mounted guest). Kernel `store` → **frozen**.
- **B3 — hub side (steps 5–7).** No ready trusted catalog exists anywhere today (`features.openPlan: false` on 7900;
  no on-disk root carries block/dag; W1 catalog A run 7 in flight), so the open-plan exchange would answer
  `CatalogUnavailable`, and without a catalog no document can be created (`POST /spaces/{id}/artifact-creations`).
  The wgpu shell also has **no artifact-creation door** at all (no caller of the creation routes in
  `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, no creation method on the Rust `DirectoryClient`) — the law opens a fresh document
  id and would get `NotFound` once B1/B2 are gone. Owners: W1 (catalog with block), shell owner (creation door; the shell
  target is peer N2's today).
- **B4 — sign-in deadline under load (flake, mitigated).** The hub lane's `DIRECTORY_COMMAND_DEADLINE_MS = 5000`
  vs a debug hub's password hash of 2.0–5.4 s (trace `durationUs` 2028665 / 5045927 / 5363640): run 4's sign-ins were
  minted on the hub but the shell had already given up. The law clicks Sign in a second time like a human would and
  records the first refusal (`retried-after=`); none was needed in the last three runs.

## 6. Files changed

Paths relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` unless absolute.

- `🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — the ledgered live law
  `two_live_wgpu_shells_collaborate_on_one_hub_document` (`#[ignore]`d, native-only) + `SeverableRelay`,
  `CollaborationLedger`, `drive`, `sign_in_live`, `shell_command`, `pump_pair`, `applied_edits`, `author_edit`,
  `online_members`; block2d kind constants. WG6's code in the file untouched.
- `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — F2 (paged runtime-manifest + descriptor reads).
- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — F3 (`kernel_runtime::read_native_component`, `create_app` uses it), F4
  (`decode_actor_turn_result` empty-owner arm + docstring). `RUN_TURN_OUTCOME_TIMEOUT` experiment reverted (5 s).
- `🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` — `withLiveHub` (N principals, env or staged-binary hub) +
  `runLiveLaw` shared by both live verbs; WG6's `HubLiveJourneyCheckScript` now uses them (re-run green); new
  `HubLiveCollaborationCheckScript` (`hub-live-collaboration-check`: `native-entrypoint` publish `block2d release`, two
  principals, the law).
- `🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` — target `hub-live-collaboration-check` (`cache: false`) after
  `hub-live-journey-check`.
- `/Users/ueli/Documents/semio/.vscode/launch.json`, `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc` — row
  `⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration` (`4_gate`, order `411.107575`) directly after WG6's
  `⚖️gate🔐️hub-auth🧊️wgpu-live-journey`, identical in both.
- Generated (product verb output, not source): `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/{dev,release}/block2d/`.

Build evidence: every change compiled in `cargo test -p semio-framework-os-renderer-wgpu --lib` (native; 546 crate
warnings present = type-checked; **0 warnings in any file above**). No wasm32 check (forbidden this slice); every new
Rust item is inside `cfg(not(target_arch = "wasm32"))` code (`kernel_runtime`, the native `load_wasm_plugins` family,
the native-gated law), so the browser build's source set is unchanged.

## 7. Processes started / stopped

- Hub hold `35430` / os-hub `35432` on **7900** (`wp-g7w/bin/os-hub`, data `/private/tmp/g7w-hub-data-7900`,
  `OS_HUB_CREDENTIAL_SIGN_IN=1`, trace sink `wp-g7w/generated/g7w-hub-7900-trace.txt`) — killed by pid 18:52, port free.
- Removed: `/private/tmp/g7w-hub-data-7900`, `wp-g7w/bin` (hub copy), `wp-g7w/target` (4 KB). Captures ≤ 100 KB each in
  `wp-g7w/generated/` (raw build noise trimmed to the test sections).
- To rerun: boot any credential-sign-in hub with the two principals (Ada/Bo, `os-hub credential set`), export
  `SEMIO_HUB_LIVE_ORIGIN/EMAIL/PASSWORD/PEER_EMAIL/PEER_PASSWORD`, then `bun nx run
  @semio-tech/framework-renderer-wgpu:hub-live-collaboration-check` (without the env it boots the staged
  `os-hub:build-dev` binary, which is absent today).
- Test runs (cargo `39248`, `51625`, `53153`, `55041`, `59069`, `61762`, run 7/8/9, the two verb runs): all exited;
  hung run-1 test binary `41033` killed by pid.
- No sub-agent, no worktree, no git-modifying command, no wasm32 build, no hub build.

## 8. § Browser document actor hub connect (after lift)

Coordinator scope addition (received 18:0x): design now, implement only after the freeze-lift message. Read:
`.tmp-ticket/📓️wp-h2.md` (socket contract), `.tmp-ticket/📓️wp-h4.md` (replica identity),
`📓️n2-wasm32-wgpu-artifact-open-relay.md` §6.1/§6.2, and the code below. Nothing in this section is landed.

### 8.1 Where the gap is (read, measured by source)

- The actor lives in the OS kernel crate `semio-framework-os-kernel`, module `sync`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`, mounted by `📦️packages/🦀️rust/🦀️.rs:289-291` behind
  `feature = "sync"`, excluded on wasip2). `mod wasm_actor` (`:3609`) has `WasmActor::connect` = `let _ = (&self.hub_base_url,
  &self.hub_space_id, &self.hub_surface);` (`:3680`) — the browser actor never dials. Everything AFTER the dial already
  exists in it: `on_binary`, `relay_operations` (Commands batches, outbox), the artifact bootstrap assembler, presence
  heartbeat send, `requeue_pending_batches`, and the reconnect edge (`WasmIncoming::Closed` → `connect()` again, `:4130`).
- `ArtifactHost::open` passes `credential`, `socket_grant_source` and the execution-target lease ONLY to the native
  `spawn_actor` (`:1342`); the wasm `spawn_actor(_pool, config, remote, cmd_rx, events)` (`:4073`) gets none of them.
- `HubSocketGrantSource::admit_document_socket` is synchronous and its only product impl is
  `#[cfg(not(target_arch = "wasm32"))] impl … for DirectoryClient<T>` (`📇️directory/🔌️client/🦀️.rs:1356`); the native
  actor runs it as a blocking job on the pool's `Io` lane. A browser isolate cannot block.
- Kind identity: exactly like the native actor (§5 B2), the wasm actor would need `document_codec(schema)` for
  `pack_schema_hash` and bootstrap decode — the browser renderer links no guest codec either.
- Contract to meet (H2): `POST /spaces/{s}/documents/{d}/open-plan` (Bearer) → `POST …/socket-grants` →
  upgrade `GET /scopes/{s}%2F{d}/document/ws?surface=<id>` with ONE `Sec-WebSocket-Protocol` value
  `semio.session.v1, <session token>`; server answers `semio.session.v1`; client-first `SocketHelloV1{schema,
  pack_schema_hash, resume_token, frontier}`; only `surface` in the query. Actor = the plan grant's `actorId`.
- H4: mutation/edit ids are minted by the guest Store with its wasi:random replica (guest side, W1's rebuilt guests);
  the actor only stamps HLC timestamps. The wasm actor seeds its HLC from `actor_seed(&config.actor)` — deterministic
  per user, so two tabs of one user tie on `(seed, counter)`.

### 8.2 Order at the lift

1. N2 §6.1 first, verbatim: in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` move
   `ureq = { version = "2", optional = true }` from `[dependencies]` to `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`.
   Then `CARGO_INCREMENTAL=0 cargo check -p semio-framework-os-kernel` (native) and, in ONE wasm-mutex hold,
   `cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --features sync`.
2. Then the hunks below as ONE compile-atomic set, checked the same two ways (+ `cargo test -p semio-framework-os-kernel
   --lib --features sync,ureq sync` natively, which H2/H4 keep at 61/61).

### 8.3 Planned hunks (kernel `store/sync` + directory client; one set)

| # | file / item | hunk |
|---|---|---|
| K1 | `📇️directory/🔌️client/🦀️.rs` `HubSocketGrantSource` | make admission a future on both targets: `fn admit_document_socket<'a>(&'a self, ctx: &'a OperationContext, …) -> Pin<Box<dyn Future<Output = Result<DocumentSocketAdmissionV1, DirectoryClientError>> + 'a>>` (no boxed-future alias exists in `semio_framework_async` today — checked; name one there if a second user appears) (target-neutral; the native impl wraps today's body unchanged, the native actor keeps submitting it to the `Io` lane and awaits the same oneshot). Add the wasm32 impl for `DirectoryClient<T>` over the client's own `async fn request_json` (`:897`), which the browser door already serves (the same open-plan → socket-grants → `DocumentSocketAuthorityV1::from_plan` sequence as `:1358ff`, no blocking). No second trait, no compat alias. |
| K2 | same file, new trait `DocumentSocketDialer` | `fn dial(&self, url: &str, protocols: [&str; 2]) -> Pin<Box<dyn Future<Output = Result<Box<dyn DocumentSocket>, TransportError>> + '_>>`; `DocumentSocket { send_binary, try_recv (one page, `dropped` counted), close }`. The kernel owns the trait; the renderer implements it over `🔌️socket-door` `SocketLane` (N2 §6.2: the page owns every socket — cookie, audit, page-mounted variant). |
| K3 | `🏪️store/🔄️sync/🦀️.rs` `ArtifactHost` | `set_document_socket_dialer(Arc<dyn DocumentSocketDialer>)` (wasm32 only); `open` passes `credential`, `socket_grant_source`, `document_socket_dialer`, `document_execution_target_lease` to the wasm `spawn_actor` too; `local_hub_ready` on wasm also requires the dialer. |
| K4 | `wasm_actor::WasmActor::connect` | the native `start_connect_hub`/`finish_connect_hub` sequence, async: credential + grant source + dialer present else `Backoff`; kind identity (K6); `admit_document_socket(…)` → verify `authority.hub_origin == hub_base_url`, scope, schema, pack hash, surface, lease; `url = hub_ws_url(origin, space, document, Some(surface))`; `dial(url, ["semio.session.v1", credential.capability()])` (the browser joins the two protocols into the one header value the hub splits on `", "`); `SocketHelloV1{wire_version:1, protocol_version:1, schema, pack_schema_hash, resume_token, frontier}`; `set_remote_state(Connecting → Live/Backoff)` emitting `ArtifactEvent::Status` exactly like native; authority deadline → re-admit. The `ws: Option<WebSocket>` field and its three closure vectors are deleted (one socket implementation: the door). |
| K5 | `wasm_actor` loop | a poll arm on the dialed socket (bounded page per turn, `dropped > 0` → fault + reconnect) replacing the `WasmIncoming::Binary` closure feed; reconnect with native's backoff law (500 ms doubling, 30 s cap) on `browser` timers; `Detach` sends `Bye` and closes. |
| K6 | both actors, kind identity (also cures §5 B2 natively) | `pack_schema_hash` = linked codec's when one is registered, otherwise the admitted lease's `artifact.pack_schema_hash` — and a hub binding WITHOUT a lease for an unlinked kind is refused at `open` (no silent Backoff). `install_artifact_bootstrap`: with no linked codec skip `print_mirror` and deliver the archive as `DocumentArchiveReplaced`; the mounted guest's `load_app_document_archive` is the validator (a refusal → `Conflict` + detach, as today's decode failure). |
| K7 | `wasm_actor::spawn_actor` HLC seed | seed from `🪪️identity::entropy_u64()` (browser `getrandom` js arm) instead of `actor_seed(&config.actor)`, matching H4's per-replica law so two tabs of one user never tie. |

Renderer/shell side (not kernel): R1 `🔌️socket-door` implements `DocumentSocketDialer`; R2 the browser boot calls
`document_host.set_document_socket_dialer(...)`; R3 BOTH shell targets call
`directory_client.document_execution_target_manifest(ctx, intent)` and `document_host.set_document_execution_target_lease(key,
lease)` before `open` when the kind has no linked codec, verifying `lease.component` against the mounted package's
descriptor `wasmSha256` (a mismatch refuses the open). R2/R3 are hunks in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` → **blocked on
N2** (peer inside that file); written here, not applied.

### 8.4 Proof plan after the lift

- Native: `cargo test -p semio-framework-os-kernel --lib --features sync,ureq sync` (61/61 + new laws for K6 on a mock
  lease and a mock async grant source — `🔄️sync/🧪️tests/🔬️unit/🦀️.rs` already has `MockHubSocketGrantSource` `:1393`).
- wasm32: `cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --features sync` and the renderer
  `--lib --target wasm32-unknown-unknown` (wasm mutex).
- Live: this slice's law gains the lease step (R3) and, with B1 fixed and a catalog carrying block (W1), steps 5–12
  become measurable natively; the browser twin is the same scenario through `🐍️wgr-live-hub-journey.mjs`-style probes.
