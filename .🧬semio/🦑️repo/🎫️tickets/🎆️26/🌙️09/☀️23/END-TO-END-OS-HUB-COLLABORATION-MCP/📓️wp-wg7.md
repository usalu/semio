# WP-WG7 — wgpu Shell (wasm32 Browser) Collaborates Over the Hub

Slice: WG7 (session 11). Ports: hubs 8050–8059 + inherited 7900, serves 6550–6559. Private cargo target:
`.tmp-ticket/wp-wg7/target`. Captures: `wp-wg7/generated/`. Inheritance: `.tmp-ticket-0918/📓️g7w-…md`, `📓️n2-…md`,
`📓️wg6-…md`, `📓️g8-…md`. Sibling: `📓️wp-wg8.md` (native B1/B2, creation door).

## Status

| # | Item | Status |
|---|------|--------|
| 1 | Landing: kernel `ureq` native-only + N2 69-hunk relay + its 2 test mounts, compile-atomic | **LANDED** 00:51 — native + wasm32 checks green (`📓️landing.md` row) |
| 1b | Relay laws (`document_relay_tests document_backbone_effect_tests plugin_install_tests`) native run | **17/17 passed** 00:59 (`wp-wg7/generated/l3-relay-laws.txt`) |
| 2 | Browser document actor hub `connect` (K1–K7 of g7w §8.3: async admission, dialer, host wiring, connect, socket loop, kind identity, per-replica seed) | **LANDED in kernel** 01:09 — native + wasm32 `--features sync` checks green; kernel `sync::` 64/64 (6 new fixture laws), `os_directory` 52/52 |
| 3 | Renderer side R1–R4 (door dialer, browser wiring, lease before open, remote uri) + browser package identity | **LANDED** — native `--lib` green 02:56; **wasm32 `--lib` green 03:07** (0 errors, covers every WG7 edit incl. the 02:5x kind-codec alignment); laws 56/56 native + vitest 64/64 |
| 4 | wasm32 browser wgpu shell: painted frame, hub sign-in, open hub document, second user sees the edit | **partial, measured** (catalog B, serve 6552, runs c2/c3): painted frame ✓×2, browser hub sign-in ✓×2, A authors a block ✓; attach reaches the lease and is refused with `schema-mismatch` — root cause the framework's declaration tree published an empty `io.artifactSchema` (fixed 15:2x, law 12/12). Re-run waits on the WG7 stdio,gis,note catalog + hub 8050 (chain 15:34) |
| 5 | Backlog (coordinator 00:5x): wgpu `AgentBridge` decodes agent reply (tag 10); approval overlay = React fields/countdown | tag 10 **already landed** (09-23 commit: decode arm + `AgentConversationEntry::AgentMessage`); overlay parity **LANDED** — React fixture law 21/21 ✓ (oracle), Rust `the_affordance_lines_match_the_shared_fixture_in_both_locales` ✓ (en + de), 56/56 in `b-approvals-laws.txt` (02:34) |
| 6 | Browser-reachable defects found by the E2E, fixed with laws | **LANDED**: chrome-as-active-window fault, Sync card never republished, panel actions dispatched in the panel, remote uri parity (React C1c), K8 frame ceiling, release serve font staging, declaration-tree surfaces stamp their document schema (C8.2), chrome mirror generation advances only on chrome change (a11y activations were dropped ~1/3 at idle) |
| 7 | R8 handoff: 2 Shell source laws pinning pre-relay gating | **LANDED** — updated to the relay truth, both pass (`r7-laws.txt`) |

## Session 12

| # | Item | Status |
|---|------|--------|
| S12-1 | Item 4 to done: wasm32 browser A authors on a hub document, B attaches and sees it, B edits, A sees it (en + de) | **collaboration proven on 8050** (runs s12c 10/13, s12e steps 1–12 ✓: sign-in ×2, attach ×2, B sees A ~1 s, A sees B ~2–4 s, pills `Persisted`); de chrome was English (no host-locale door) → fixed in source (🗣️HostLocale, law green), renderer rebuild in the `wasmshort` lane; then 8050 re-run + 7800 B2 run |
| S12-1b | Coordinator add-on (audit P2-2): ONE kernel connection-shortage primitive (suspend/resume, refuse long offline) for native + wasm32 + React TS twin, fixture law, 5–20 s outage proof en + de | primitive landed, laws green; **15 s cut proven live** (s12e 10–12: worker answers in 2–26 ms, offline edit `Pending (1)`, relink 1.5 s after the cut, B receives the offline edit). Kernel defect measured: a cut longer than ~31.5 s expired (retry scheduled past the bound) → patch set `wp-wg7/s12-link-expiry-patch.py` prepared + verified in isolation (rustc 2/2, bun twin 52/52 steps, Ajv ✓); applies after rule 20 lifts |
| S12-2 | Presence in the wasm32 wgpu shell (roster, peer cursors) interoperating with React | roster name parity **landed** (the wgpu heartbeat sent the APP ID as the person's label; now React's `presenceClientIdentity` rule, law 13/13); peer cursors not started (wgpu sends no `views`, paints none) |
| S12-3 | Agent reply + approval overlay parity live in the wasm32 shell | **gap found + source landed**: the wasm32 shell had NO bridge discovery (nothing called `semioWgpuSetAgentBridgeConfig`, the wgpu serve mounted no rendezvous) — shared offer leaf + page watcher + `host-agent-bridge` transport message + worker hook + serve plugin; laws 68/68 + 47/47, tsc 0. Live run pending (renderer build) |
| S12-4 | P2-12 native/wgpu accessibility (keyboard traversal, AccessKit tree for chrome) — measured | **browser half measured** (`wg7-a11y-probe.mjs`, `wg7-tab-trace.mjs`): mirror 37 nodes / 36 focusable, but **0 Tab stops — Tab from the canvas into the mirror faulted the whole shell** (transport refused node id 0, the contract's first minted id) → fixed in source (TS transport + Worker credits share one rule, fixture `transportCredits`, schema; TS 61/61, Rust law pending run); 2 unnamed `application` surfaces; native AccessKit waits on rule 20 (Cargo.lock) |
| S12-5 | wasm32 `--lib` checks of every touched crate through the wasm mutex | kernel `sync` + renderer green (02:20, 09:48); renderer re-check for the locale door running (`wasmshort`, 10:49) |

### Session 12 log

- 10:5x–11:0x **S12-4 browser keyboard traversal, measured** (serve 6552, release of 10:16): the mirror holds 37 nodes (36 focusable) in en and
  de, yet **Tab never reaches it** — 80 presses alternate `<body>` ↔ canvas (`generated/a11y-probe.json`). Trace (`wp-wg7/wg7-tab-trace.mjs`,
  `generated/focus-bounce2.mjs`): Tab moves focus from the canvas to the first mirror element, the note window's root `note.play.composite`
  (role `application`, **node id 0**); its `focus` listener enqueues `accessibility-focus`, and `BrowserFrameTransport.enqueueLossless`
  refused the address (`nodeId < 1` → `fail("lossless-overflow")`) → `onFault` → the mirror is disposed and the frame transport closed: **a
  keyboard user tabbing into the app killed the browser shell** (`end: mirror false, active BODY`). Node id 0 is legitimate — the contract's
  `UiNodeIdAllocator` mints 0 first — and liveness (a retired generation, a key mismatch) is already the renderer's own `Ok(false)` in
  `handle_accessibility_event`. Fix (renderer + TS, not guest-linked): the transport and the Worker admit an address by CREDITS only
  (bounded, non-empty, control-free ids; non-negative safe integers); the Worker's private `invalid_accessibility_id` became the shared
  `input_wire::accessibility_address_within_credits`; fixture `🧫️fixtures/♿️wgpu-accessibility-interaction` gained `transportCredits`
  (3 admitted rows incl. node 0 and generation 0, 5 refused rows) and its schema the `transportCredits`/`creditRow` definitions. Laws: TS
  `📨️browser-frame-transport` + `♿️wgpu-accessibility-interaction` **61/61** (`generated/s12-vitest-transport-2.txt`), Rust
  `accessibility_addresses_are_admitted_by_credits_and_node_zero_is_an_address` (input-wire laws) — run pending (fleet load 50 rustc).
  Also measured: the two note window roots are unnamed `application` nodes (a reader hears "application" with no name); the de page's chrome
  labels are English (locale door, below).

- 10:1x–10:3x **runs `s12d` / `s12e`** (8050, renderer release of 10:16, serve 6552 pid **63164**, A en-US / B de-DE, fresh door note
  `artifact-6f85879dc45ac30cb4dbfe2c007a6da5`; `generated/collab-s12e-*`): steps 1–12 ✓ — painted ×2, sign-in ×2, attach ×2, roster
  `User One · User Two` ✓, **B sees A's block, A sees B's block** without reloads, frames name the document ×2, **15 s outage: A's frame worker keeps
  answering (13 / 26 ms), the offline edit is admitted and shown `Pending (1)`, A relinks 1.5 s after the cut and B receives the offline edit**. Reds
  (12b/12c/13/14) and their causes:
  - probe: the Sync card's retained keys are surface-scoped (`s-sync-status/framework.sync.link.<code>`), the probe matched a bare prefix, so it
    never saw the link line and "ensured" the card by toggling it — closing it (12b screenshot: card closed, pill `Remote: backoff`). Fixed:
    link code from the scoped key, card opened only when its dock switch is not `checked`, card digest in every outage record, medium cut
    `WG7_MEDIUM_CUT_MS` (default 20 s, the top of the required 5–20 s range);
  - **de-DE chrome was English**: the shell resolved `locale resolved=en reason=preferences` — the page's `navigator.language` read reached the
    frame Worker (`message.locale`) and was spent on one boot error message; nothing handed it to the renderer. Fixed (renderer + TS only):
    region 🗣️HostLocale in `🧊️renderer/🦀️.rs` (`host_locale_from_tag` = React's `normalizeUiLocale`, `set_host_locale`, `host_locale`,
    wasm `semioWgpuSetHostLocale`), the shell's ONE `resolve_locale_id` = React `ShellHost`'s `locks?.locale ?? stored ?? detectShellLocale(navigator.language)`
    term for term (both load paths), the frame Worker forwards `message.locale` with the other environment doors, the embeddable door reads its
    page's `navigator.language`. Law `the_host_language_is_the_fallback_after_a_lock_and_the_stored_preference`; native renderer laws
    **40/40** (`ui_prefs_themes_i18n_tests`, `document_relay_tests` incl. the footer law never run before, `appearance_tour_and_footer_pill_tests`;
    `generated/s12-renderer-locale-laws.txt`);
  - **kernel link expiry (A's 45 s cut expired)**: `retry_at` ignored the bound — after the capped attempt at since+31.5 s the next was due at
    since+61.5 s while `Tick` expired at since+60 s, so every cut longer than ~31.5 s expired although the hub was back. Rule 20 freezes the kernel,
    so the fix is a ticket-local patch set `wp-wg7/s12-link-expiry-patch.py` (dry run by default, `--apply` after the freeze): no retry past the
    bound (`retry_at = min(now + backoff, since + bound)`, the last attempt runs AT the bound), only a failed attempt at/after the bound expires, `Tick`
    expires at the ceiling bound + one capped backoff (an attempt that never answers), `expires_at_ms` = ceiling; kernel region + TS twin + schema text
    + fixture (2 new vectors: `a-cut-shorter-than-the-bound-relinks-at-the-bound`, `an-attempt-that-never-answers-expires-at-the-ceiling`; 52 steps)
    + Rust deadline law. Verified in isolation: the patched kernel region compiled standalone with `rustc --test` 2/2 over the patched fixture, the
    patched TS twin walks all 52 steps, Ajv admits the patched fixture (`generated/link-patch-check/`, dry-run diff `generated/s12-link-expiry-dryrun.diff`).
- 10:4x frame-worker regeneration was blocked for everyone: a peer's new import `📇️directory/🔌️client/🚪️socket-close/🟦️.ts` (+ its `🔣️.json`) in
  `🛍️products/💻️os/🟦️.ts` was not owned by the taxonomy's WGPU browser profile ("WGPU browser import is not schema-owned"). Added both paths to
  `sourceModulePaths` + `inputPatterns` (sorted); frame-worker regenerated, `check-frame-worker` fresh.
- 10:49 renderer rebuild queued in the `wasmshort` lane (`s12-short-lane.sh`, detached, wrapper **76702**, log `generated/s12-short-lane-locale.txt`).

- 22:55 start. Nothing running (every process died ~19:30). State recovered from disk: the last session-11 step that ran was the
  served-note materialization at 18:05 (`s11-wg7-logs/n-catalog-module.txt`: served `wasmSha256 ac4b77f7…` == catalog-n note), serve 6552
  restarted 18:05; no E2E run happened after it. Hub 8050 data root (`s11-wg7-hub-8050`, generation `f6d193e1…`, space
  `01a0d91e-03c1-71fd-ba2e-28ee3db6c9e9`, note `artifact-ffe16f4e05776efdde9a094f3796af77`) and its signed binary survive. W2 holds the wasm mutex.
- 23:00 **schema-mismatch root cause re-checked**: the session-11 fix (declaration-tree `editor_surface`/`viewer_surface` stamp
  `E::DOCUMENT_SCHEMA`, 15:2x) is in the tree and in catalog-n's note (`ac4b77f7…`, published 16:58 after the fix); the served note is
  that component. The NEXT wall on the same path is the one WG8 named: a fresh door artifact answers `Welcome { bootstrap: None }`, and
  on wasm32 `seed_document_genesis` found no `ComponentDocumentCodec` → no genesis → the guest would author under its app id. Also found:
  the browser `load_app_document_pack` called `loadAppArtifactPack` WITHOUT awaiting its promise (a genesis load would race the actor open
  and lose its error).
- 23:0x–23:12 **browser jco `ComponentDocumentCodec` (one mechanism with native)**, no ABI change (the guest already exports `codec`):
  - leaf `🎭️actor/📮️shard-client/🧬️component-codec/🟦️.ts` (zero-import: `ShardCodecRequest`/`ShardCodecAnswer`/`ActorCodecExports`,
    `ACTOR_CODEC_REFUSAL`, `actorCodecAnswer` — a guest `plugin-error` answers `{fault}`, never a thrown `[object Object]`), its schema
    `🧬️schema/🔣️.json` + language-agnostic fixture `🧫️fixtures/🔣️.json` (7 cases);
  - bridge generator (`pluginComponentBridgeSource`) destructures `codec` and inlines `actorCodecAnswer` as the `codec` arm; shard worker
    (`shardWorkerSource`) `case "codec"` (activation-exact, refuses while a turn of the actor is in flight, beats like a job step);
  - `ShardClient.codec(actorId, request)`; wgpu plugin handle `codec` on a live instance's actor through `submitActorWork` (serialized with
    its turns), fault → display message; JS bridge `codecPackSchemaHash`/`codecGenesis`/`codecPrintMirror`;
  - Rust `ProgramBridge` (wasm32): `browser_component_codec` (thread-local JS handle per schema, `OnceLock` fingerprint) registered in
    `create_app` for the app's document schema (`app_document_schema`, now target-neutral) — the twin of native `create_app`'s
    `OwnedComponentDocumentCodec`; `load_app_document_pack` awaits `loadAppArtifactPack`.
  - frame-worker regenerated; the new leaf is owned by the taxonomy's WGPU browser profile (`🔣️taxonomy.json` `sourceModulePaths` +
    `inputPatterns`) and the renderer package's `nativeSources`. Full WGPU package projection renders (6 nodes, 134 inputs); `check-frame-worker` fresh.
  - **Laws:** vitest `ShardComponentCodecLane` 3/3 (fixture ⊨ schema via Ajv; every fixture case identical in the leaf arm AND the generated
    bridge imported as a real ES module beside a stand-in jco component; generated worker answers every case with a result + beat, refuses an
    overlapping turn and a stale activation) + `ShardWorkerCancelJobReply` 3/3 (`generated/s12-vitest-codec-1.txt`). Whole actor package
    275/283 — the 8 failures are pre-existing field/fixture drift in lifetime/return/settlement laws (`actorsPastFirstTurn`, `onLiveness`,
    `contendedWitnessStaysTerminal`…), none on the codec lane (`s12-vitest-actor-all.txt`). tsc over the 5 changed TS files: 0 errors in them
    (1 in a peer's `backbone-envelope-io` test). Native `cargo check -p semio-framework-os-renderer-wgpu --lib` green (126 pre-existing warnings).
  - Note for the coordinator: my first frame-worker generation went through `nx run …:generate-frame-worker`, whose `dependsOn` ran
    `@semio-tech/plugin-registry:generate` (W2-owned). Its output was byte-identical (no tracked registry / launch.json change); since then I
    call the package script directly.
- 00:3x **third-party oracle for the browser codec** (`wp-wg7/s12-codec-oracle.mjs`): the SERVED note component, run by jco on V8 (the
  browser shell's runtime), answers `codec.pack-schema-hash("note.document")` = `e415edbf…` = the catalog-n pin wasmtime computed at
  publish, and `codec.genesis("note.document", "artifact-ffe16f4e…")` = pack `3dedb3b6…` + spr `4f7ba296…` — byte-identical to the
  genesis pair the hub (wasmtime) stored in its artifact CAS for that document. **4/4** — two independent component runtimes, one answer.
- 23:12 wasm mutex hold queued (wrapper **99708**, `wp-wg7/s12-wasm-hold.sh`, log `.🧬semio/🌐hub/s12-wg7-logs/wasm-hold.txt`): renderer wasm32
  `--lib` check, then `wasm-release`. Served-module glue refreshed (`wp-wg7/s12-module-glue.ts`: note bridge + shard worker from the current
  generators; the diff is exactly the codec arm + case; the served component bytes are untouched).
- 23:19 **hub 8050 restarted on its existing data root** (hold **6043**, os-hub **6048**, runId `57f8aea3…`, state `s12-wg7-hub-8050-state`), readyz 200.
- 23:20 **renderer wasm32 `--lib` check green** (hold 99708, 4 m 48 s, 0 errors; no warning in the codec code) — S12-5 for this edit.
  The `wasm-release` build that followed was **preempted by the coordinator at 23:51** (29 min in, recompiling release-profile plugin
  crates) so W2's catalog B2 could run; re-queue after W2's B2 publish hold ends.
- 23:55–00:20 **S12-1b (audit P2-2) — one link-shortage primitive for every shell.** Before: React (`suspendDocumentBrowserActorLink`,
  60 s bound, 401/403/404/410 → revoked) and the two Rust actors (native: own `(b*2).min(30_000)`, wasm32: `next_backoff_ms`, neither
  with any long-offline bound) were three rules. Now:
  - schema `🏪️store/🔄️sync/🧬️schema/document-link-shortage/🔣️.json` (policy, `accessRefusedStatuses`, en/de texts, states, events,
    observations) + fixture `🏪️store/🧫️fixtures/document-link-shortage-v1/🔣️.json` (5 vectors, 29 steps: short shortage resumes in place;
    backoff doubles to its cap and an unlinked open expires; long offline expires and never relinks; refusal revokes);
  - kernel region `🔖️DocumentLinkShortage` (`🏪️store/🔄️sync/🦀️.rs`, target-neutral): `DocumentLink {Linked, Unlinked{since,backoff,retry_at},
    Expired, Revoked}`, `DocumentLinkEvent`, `DocumentLinkStatus` (+ en/de `text`), `DOCUMENT_LINK_SHORTAGE_POLICY` (500 ms → 30 s, bound 60 s),
    `DOCUMENT_LINK_ACCESS_REFUSED_STATUSES`, `document_admission_refuses_access`, `document_link_terminal_message`;
  - **native actor** and **wasm32 actor** drive it (`schedule_reconnect` = `Failed`, `set_remote_state(Live)` = `Restored`, a tick per drive/loop,
    admission refusal = `Refused`, an expired/revoked link closes its socket, reports `Detached` and emits the coded conflict; no schema change to
    `RemoteState`/`ArtifactSyncStatus`); the native connect future now says WHY it failed (`DocumentConnectFailure::{Refused, Short}`);
  - **wgpu shell** (both targets): `sync_link` follows the actor (backoff → reconnecting, live → linked, coded conflict → terminal + retire the
    document), the Sync card speaks `framework.sync.link.<status>` in en/de;
  - **React**: TS twin in `🛍️products/💻️os/🟦️.ts` (`DOCUMENT_LINK_SHORTAGE_POLICY`, `documentLinkTransition`, texts, refused statuses); the worker's
    suspend/resume/retire now walks `state.link` through it (bound + refusal + status code from the twin; the dangling `@see` to a non-existent
    `link-shortage-v1` fixture is gone).
  - Laws: Rust `document_link_shortage_tests` 6/6 (fixture, policy, texts, refusal classification, terminal message, deadlines); TS
    `DocumentLinkShortage` 3/3 (Ajv schema ⊨ fixture, same vectors, texts == React's execution-target texts); kernel `os_store::sync` 73/73;
    renderer relay laws 12/12 incl. `a_short_link_is_spoken_and_a_terminal_link_ends_until_the_next_open`; framework-os vitest 463/463; tsc 0.
    Native checks: kernel `--lib --tests --features sync,ureq` green, renderer `--lib --tests` green (no new warnings).
- 00:2x **S12-2 roster parity**: `advance_presence_preview_step` sent `label = session.app.id` (React users saw `s.note.note@1/*#editor` as
  the wgpu person's name). Now `shell_presence_label(identity, actor)` = React's `presenceClientIdentity` (display name when signed in,
  else `Guest <last 4 of actor>`); law `a_presence_heartbeat_names_the_signed_in_person` (relay laws 13/13). Peer cursors: the wgpu heartbeat
  sends no `views` and nothing paints peers' `views[].pointer` — open.
- 00:2x–00:31 **S12-3 wiring** — measured gap: in the wasm32 shell nothing ever called `semioWgpuSetAgentBridgeConfig`, and the wgpu serve
  mounted no `semioAgentBridgeRendezvousVitePlugin`, so the bridge stayed `Disabled` forever (React discovers the offer by polling
  `/__semio/agent-bridge`). Landed:
  - React-free leaf `🔗️AgentBridge/🛰️offer/🟦️.ts` (moved out of the React hook file: `AgentBridgeConfig`, `AGENT_BRIDGE_OFFER_ENDPOINT`,
    `isAdmissibleBridgeUrl`, `parseAgentBridgeOffer`, `fetchAgentBridgeConfig`, `bridgeProtocols`, discovery intervals; new
    `nextBridgeDiscoveryIntervalMs`, `sameAgentBridgeOffer`, `watchAgentBridgeOffer`); React's hook uses the same schedule helpers; the
    rendezvous vite plugin reads the one endpoint constant (its duplicate literal deleted) and takes an explicit `shellKind`;
  - wgpu: `BrowserFrameHostAgentBridge` message + `BrowserFrameTransport.setHostAgentBridge`, the frame Worker hands it to the renderer's
    `semioWgpuSetAgentBridgeConfig`, `🚀️browser-boot` and `🎬️renderer-boot` run `watchAgentBridgeOffer`, `createWgpuBrowserConfig` mounts the
    rendezvous plugin (`shellKind: "wgpu-web"`); the leaf is owned by the taxonomy's WGPU browser profile; browser-boot + frame-worker regenerated.
  - Laws: `agent-bridge-check` 68/68 (+2: watcher publishes appear/change/vanish once each on React's schedule; a poisoned offer is no offer),
    `📨️browser-frame-transport` 47/47 (+1: offer and withdrawal cross on the current lifecycle); tsc 0 over the 10 touched files.
- 00:32 coordinator slotted WG7's ONE wasm hold: queued (mutex wrapper **52905**, `s12-wasm-hold.sh`: kernel `sync` + renderer wasm32
  `--lib` checks, then renderer `wasm-release`; exits and releases on any failure), after W2's B2 hold and R8/WG8's short tickets.
- 02:20–02:29 **the one wasm hold ran** (wrapper 70298, `s12-wg7-logs/wasm-hold-2.txt`): kernel `sync` wasm32 check rc=0, renderer wasm32
  `--lib` rc=0 (fresh from a peer's identical check after my edits), renderer **`wasm-release` rc=0** (8 m 12 s) — contains the codec twin,
  link primitive, bridge discovery, roster badge and the browser ephemeral cache (01:5x). S12-5 green for kernel + renderer.
- 01:5x **wasm32 app presence twin** (WG8 landed the native half: board views/cursors + native `AppFrame::Ephemeral` cache; the wasm32
  heartbeat still sent `(None, None)`): `browser_ephemeral` in `🌉️ProgramBridge` reads the JS bridge's `ephemeralSnapshot` (the shared
  `AppChannelClient.ephemeral()`) after every action/command, decodes the interaction, caches per instance; `ephemeral_snapshot` and
  `ProgramEphemeralSnapshot` are target-neutral; the shell heartbeat's wasm32 stub is gone. TS: `WgpuPluginHandle.ephemeralSnapshot` +
  `wgpuEphemeralSnapshot` + JS bridge field; law in `🧩️package-integration` 26/26. Native renderer `--lib --tests` check green (03:5x).
- 03:4x resumed after the usage reset. Serve **6552** restarted (pid **36759**, nice 0, private rendezvous: `/__semio/agent-bridge` → 404
  "no gateway", session record in `s12-wg7-bridge/sessions`); frame-worker + browser-boot regenerated (the ephemeral TS postdated them).
- 03:4x **run `s12a`** (hub 8050, fresh door artifact `artifact-efc52861…`, A en-US / B de-DE, `generated/collab-s12a-*`): painted ✓×2,
  **browser hub sign-in ✓×2**, attach ✓×2 up to the hub (`open-plan` 200 → `socket-grants` 200 → `/document/ws` upgrade; the hub admits the
  browser principals — `server.document.socket ok upgrade`, `presence.join`), **document frames name the hub document id on BOTH browsers
  (step 9 ✓×2: 15 / 10 sent frames carry `artifact-efc52861…`)**, A and B each author locally (Add Text ✓×2). ✗: neither browser ever
  shows `Live` (pill `Remote: detached`), no roster peer, no edit crosses; every browser socket is closed BY THE CLIENT after exactly
  ~30.0 s (hub `server.document.socket cancelled closed durationUs≈30 000 000`) and re-admitted — the plan/authority lifetime rule
  (native has the same `invalidate_socket_authority`), so the 30 s cycle is not the blocker; not reaching `Live` is. 8/13.
  Next: decode the frames (payload capture added to the probe; decoder `wp-wg7/s12-decode-frames.ts`).
- 04:0x **run `s12b`** (same setup, frame payloads captured, decoded with the product codecs by `wp-wg7/s12-decode-frames.ts`): the WIRE works —
  `SocketHelloV1{schema note.document}` → `Welcome{bootstrap Tail, frontier artifact-efc52861… head 2}` → `Commands` carrying **A's and B's
  Add Text envelopes from run s12a, `document_id = artifact-efc52861…`, actors `hub.v1.8180…` (A) and `hub.v1.851b…` (B)** → `Session` →
  `Presence`; reconnects resume (`Welcome{bootstrap None}`). So **item 1's core is proven on the wire: a fresh door artifact, the browser
  seeds the component genesis (jco codec), authors under the hub document's identity, and the hub accepts both users' edits.** Still 8/13:
  the shells never showed any of it.
- 04:1x **root cause (renderer):** `FrameFinishPhase::Deferred` set `cursor.pump_sync` only under `#[cfg(not(target_arch = "wasm32"))]`, so on
  wasm32 `ShellState::pump_sync_events` never ran: every actor event (`Status`, `Presence`, `RemoteMutations`, `Conflict`) sat in the
  channel, the pill stayed `detached`, and the browser's directory lane (`pump_directory_events`: identity poll, space administration,
  creation door, **agent bridge pump**) never ran either. Fix: one target-neutral cadence `SHELL_SYNC_PUMP_INTERVAL_MS` (100 ms). Source law
  `the_sync_pump_cadence_is_not_gated_off_the_browser_build` (relay laws **14/14**). (A peer's in-flight `continue_resolved_open` move error
  briefly broke the renderer test build; the peer fixed it themselves within 2 min.) Needs one renderer `wasm-release` — asked the coordinator.
- 05:59 release attempt 3 **rc=0** (renderer with the sync pump fix). 08:4x resumed after the second usage reset; serve 6552 restarted on the
  new build (pid **6913**), second fresh door note on 8050 **`artifact-a414777f4653467f7c62ce3a2b62086c`**.
- 08:5x **run `s12c` (8050, wasm32, A en-US / B de-DE): 10/13 — COLLABORATION WORKS IN THE BROWSER WGPU SHELL**: painted ✓×2, browser hub
  sign-in ✓×2 (first attempt each), A authors (Add Text) → **B sees A's block in ~1 s** without a reload, B authors → **A sees B's block in ~4 s**,
  both pills `Persisted`, document frames name the hub document ✓×2 (`generated/collab-s12c-*`, screenshots `collab-s12c-05-b-after-edit.png`,
  `-07-a-after-b-edit.png`). The 3 reds were observation defects of the shell, now fixed in source:
  - **stale sync pill**: the footer sync leaf's label (and accessible name) is baked into the dock when it is rebuilt, and `Status` events
    never rebuilt it, so the pill read `Remote: detached` for minutes after the actor was live (steps 3 ✗×2). Fix: `relabel_sync_tab` after every
    pumped change (+ the terminal-fault path);
  - **roster not accessible**: the footer's presence chip (and the signed-in hub chip) are painted with no hit target, so they never reached
    the accessibility tree (step 4 ✗; React's `#s-presence-peers` is a labelled status). Fix: `footer_status_chips` → `role: status`, polite,
    non-focusable nodes carrying the painted text. Law `the_footer_speaks_its_live_sync_pill_and_names_its_status_chips`.
  Probe now reads the roster from `s-presence-peers`. Renderer wasm32 `--lib` check green through the `wasmshort` lane (09:48, 6 m 05 s);
  release queued in the same lane (`wp-wg7/s12-short-lane.sh`, log `s12-wg7-logs/short-lane-1.txt`). Native laws of this change: my first
  attempt sat ~30 min idle in a cargo lock wait beside two peers' idle renderer test cargos (15422, 19429) — stopped mine (15209), told the
  coordinator; re-run after the release.
- 04:25–05:0x rebuild under the coordinator's no-mutex exception (niced, detached): attempt 1 (pid 48887) stopped at the wasm32 check —
  the pump timestamp FIELD `last_sync_pump_ms` was also native-only (fixed: field + native initializer un-gated, native check green);
  attempt 2 (pid 52604) stopped at the wasm32 check — the browser worker's own `AppInteractionState` initializer lacked the field (fixed);
  attempt 3 (pid **56863**): kernel + renderer wasm32 `--lib` checks **rc=0** (04:44), `wasm-release` compiling (release-profile stdio crates
  recompiling at nice 15, load ~41). From rule 19 on: wasm32 work through `fleet-mutex.sh wasmshort wg7`, check and release split.
- 04:2x **7800/B2 prepared**: seeded space **`01a0db87-d093-7912-ba75-6f74e372ad44`** ("WG7 wasm32 collaboration", user1 owner, user2 author) and
  fresh door note **`artifact-aae0fa4e344fbb4c5460ce0efa53270e`** (`s12-wg7-logs/seed-7800-*.txt`); B2's own note materialized byte-exact
  (`7a957ce7…` == catalog) into the separate root `.🧬semio/🌐hub/s12-wg7-catalog-modules-b2` (glue refreshed: the shared release shard
  worker there predates the codec case); **codec oracle 4/4 on B2** (jco genesis == 7800's wasmtime CAS genesis, pack hash == pin
  `e415edbf…`, same pin as catalog-n). Ticket scripts parametrized: `wg7-catalog-module.ts [durableName]`, `serve/wg7-vite.config.ts`
  (`WG7_MODULE_ROOT`), `s12-serve-6552.sh [port] [moduleRootName]` (private rendezvous per port), `s12-module-glue.ts --root=`.
- 01:0x–01:24 **roster agent parity** (S12-2/S12-3): the wgpu `PresencePeerRow` had no principal kind, so a wgpu roster could not show a
  delegated semio-MCP agent AS an agent (React: badge `peer-agent-badge:<actor>`, "AI agent"/"KI-Agent" in the accessible name). Now
  `PresencePeerRow.is_agent` (`semio-framework-ui`), mapped from the hub-admitted `principal_kind`; the tree roster adds the badge node, the
  footer chip reads `Label (AI agent)` / `(KI-Agent)`; `presence_agent_label` = React's `ui.presence.kind.agent`. Law
  `an_agent_row_carries_the_agent_badge_in_both_tongues_and_a_person_never_does` (ui presence 10/10); renderer relay + footer laws 25/25;
  native checks green (ui `--features wgpu-engine --lib --tests`, renderer `--lib --tests`).
- 01:24 rule 18: every build now under `nice -n 15`; the queued hold script (read at lock time) edited to nice its three builds.
- 00:53 rule 17 (no BG_NICE): the queued wrapper 52905 ran at nice +5 → re-queued the same hold at nice 0 (`setopt no_bg_nice`; wrapper
  **70298**, log `s12-wg7-logs/wasm-hold-2.txt`; now behind T12's ticket), stopped 52905 by pid (its TERM trap dropped the ticket but kept
  looping — the session-11 shape — so `-9`). Hub 8050 (hold 6043 / os-hub 6048) already runs at nice 0 (setsid Popen), not restarted.
- 00:33 fresh door-created note on hub 8050 for the genesis proof: **`artifact-efc52861f595b319ae596144f5eadde3`** ("WG7 S12 genesis",
  kind `s.note.note`, space `01a0d91e-…`, `s12-wg7-logs/seed-s12-document.txt`). Serve launcher `wp-wg7/s12-serve-6552.sh` pins the agent-bridge
  rendezvous to the slice-private `.🧬semio/🌐hub/s12-wg7-bridge` (`S_AGENT_BRIDGE_DIR`) — with S12-3 every wgpu serve registers in the
  rendezvous, and a default-dir serve could attract other sessions' semio-MCP gateways.
- Probe `wg7-browser-collab.mjs` extended: A `en-US`, B `de-DE` (chrome follows `navigator.language`), steps 7–8 (B authors, A sees), step 9
  (the page's document-socket frames sent by each browser name the hub document id — the genesis identity on the wire).

## Scope split with WG8 (coordinator 00:5x)

- **WG8** owns native B1 (kernel turn), B2 (native document actor codec for guest-owned kinds), the artifact-creation
  door, and — proposed here — **runs the 12-step `hub-live-collaboration-check`** (it is the native law; every failing
  step 4–12 is behind B1/B2/B3).
- **WG7** owns the landing, the browser actor `connect` (kernel `wasm_actor` + directory-client browser admission), the
  renderer's browser wiring, and the browser E2E.
- Shared code in `🏪️store/🔄️sync/🦀️.rs`: WG7 adds target-neutral connect helpers (expectation, authority check,
  hello, kind-identity resolution, per-replica seed) in a new `//#region 🔖️DocumentSocketConnect` and edits only the
  `wasm_actor` module + `ArtifactHost` browser wiring. The native actor's `start_connect_hub`/`finish_connect_hub`
  stay WG8's; WG8 may switch them to the shared helpers (same semantics, one body).

## Log

- 00:37 kernel `ureq` move → native check green; wasm32 `--features sync` found one more break (native-gated
  `ArtifactId` import used by target-neutral `envelopes_from_history_edit`) → un-gated → green 00:39.
- 00:41 N2 relay: true dry run on a scratch copy of the 5 files (all 69 hunks clean), applied unchanged; native
  renderer check green 00:44; wasm32 renderer check green 00:51 (6 min); kernel wasip2 check green 00:52.
- 00:53 relay law build started (pid 67566, private target dir).
- 01:04 K1 landed: `HubSocketGrantSource::admit_document_socket` is one admission body in three legs
  (`document_admission_intent` / `_exchange` / `_finish`); native keeps the blocking `protected_post` on its `Lane::Io`
  job, the browser impl is a future over the transport's async `http` (`protected_post_async`). Kernel native + wasm32 green.
- 01:09 K2–K7 landed in `🏪️store/🔄️sync/🦀️.rs`: new `//#region 🔖️DocumentSocketConnect` (pub, target-neutral:
  `document_pack_schema_hash`, `document_socket_expectation`, `DocumentSocketBinding` + `document_socket_authority_admits`,
  `document_socket_hello`, `document_socket_protocols`, `replica_hlc_seed`), `//#region 🔖️DocumentSocketDoor` (browser:
  `DocumentSocket`, `DocumentSocketDialer`, `DocumentSocketPoll`), `ArtifactHost::set_document_socket_dialer` +
  `local_hub_ready` requiring it in a browser, and the browser actor rewritten (no `web_sys::WebSocket`; admission →
  authority check → dial → hello on open → native `on_hub_frame` semantics incl. socket-actor filter, `Session`
  confirmation, `Live` after catch-up, `Status` events, 500 ms→30 s backoff; bootstrap without a linked codec is bound to
  the lease's pack identity and validated by the mounted guest). Native actor untouched (WG8).
- 01:13 kernel laws: fixture `🏪️store/🧫️fixtures/document-socket-connect-v1/🔣️.json` + runner
  `🔄️sync/🧪️tests/🔬️document-socket-connect/🦀️.rs` (6 laws) — `sync::` 64/64, `os_directory` 52/52.
- 01:22–01:38 R1–R3 in the renderer (`wp-wg7/r1-r3-browser-document-wiring.py`, anchored): `🔌️socket-door` browser
  `DoorDocumentSocketDialer`/`DoorDocumentSocket` implement the kernel's `DocumentSocketDialer`/`DocumentSocket` over the
  page-owned duplex door (binary frames; `dropped > 0` → `Lost`, closed → `Closed`); the browser shell installs it on its
  `ArtifactHost`; hub sign-in now lends credential + grant source to the document host on BOTH targets;
  `SHELL_DOCUMENT_TRANSPORTS` browser = `{folder: false, hub: true}`; `open_document` → new
  `bind_document_execution_target` (both targets): a hub binding of a codec-less kind fetches the hub's
  `execution-target/manifest` lease for the exact scope + requested surface and binds it only when
  `document_execution_target_admitted` (plugin, package, component SHA-256, schema, surface all equal the mounted
  program's). `ProgramBridgeEntry` gains `component_sha256` (native: the runtime module's `wasm_sha256`; browser: the
  bridge's new `packageIdentity()` read off the served descriptor, so a JS program finally has a `package_id`).
- 01:4x TS: kernel `fetchPackageDescriptor` (identity = descriptor `packageId` + `hashes.wasmSha256`, refused when
  absent); plugin-bridge handle carries `packageId`/`componentSha256`, `WgpuJsBridge.packageIdentity()`. Laws: vitest
  `🫀️plugin-load-progress` "reads a package descriptor's identity…" ✓, `🧩️package-integration` "answers the mounted
  package identity…" ✓ (`r-vitest-bridge-identity.txt`). The same suite's 2 frame-worker render laws FAIL on a peer's
  in-flight edit: `📇️directory/🧬️schema/🟦️.ts` imports `./📌️document-check-in-v1/🟦️.ts`, which the WGPU browser
  import profile does not own ("WGPU browser import is not schema-owned") — blocks regenerating `🎞️frame-worker/🤖️generated`
  (H9's check-in slice owns that import).
- Rust law: `📂️wgpu-document-relay` updated (browser = hub-capable; `NONE` host keeps the refusal law) + new
  `an_execution_target_lease_binds_only_the_mounted_package`.
- 02:05 G-P2-1 overlay parity (`wp-wg7/b1-approval-affordance-parity.py`): the shared fixture
  `🤖️AgentApprovals/🧫️fixtures/🛡️summary/🔣️.json` gains `affordance` rows (the exact lines React's
  `AgentApprovalAffordance` shows, en + de, at 42 s / 0 s / no wait); wgpu element gains `approval_row_lines` (verb/role
  subject, capability beside a title, description, "Applies to", change, requester, risk, countdown/expired/pending) +
  labels, `approval_row_line_count` counts exactly those lines; the shell overlay paints them with the live countdown
  from `approval_seconds_remaining(timeout, requested_at_ms, now)`. Laws: React (oracle side) renders the fixture's `en`
  lines for all 4 rows — vitest `🤖️AgentApprovals` 21/21 ✓; Rust `the_affordance_lines_match_the_shared_fixture_in_both_locales`
  queued.
- AgentBridge tag 10 (G-P2-1 first half) was already done before this session (`🔗️AgentBridge` wgpu decode arm at
  `GatewayToShell::AgentReply`, `AgentConversationEntry::AgentMessage`, committed 2026-09-23); nothing to add.
- 02:10 R4 (`wp-wg7/r4-remote-backbone-uri.py`): the wgpu sync card's `remote://host/space/document` had React's pre-C1c
  bug (everything after the host became the SPACE id, the document id was the local `plugin-instance`) — two browsers
  attaching one uri could never share a document. Now `parse_remote_backbone_uri` (twin of `💻️os/🟦️.ts`
  `parseRemoteBackboneUri`) names host/space/document, `attach_sync_backbone` binds that document with the session app's
  canonical surface; a partial remote uri is refused. Law `a_remote_backbone_uri_names_host_space_and_document`.
  The `/spaces/{id}` route's space-index binding (and its two constants + `space_index_dialect`) is un-gated for the
  browser build (it was native-only; `open_document` is target-neutral since the relay landing).
- 02:18 frame-worker bundle regenerated with the bridge identity (`generate-frame-worker`; `check-frame-worker` fresh). The
  peer schema-import fault seen at 01:50 is gone; vitest `🧩️package-integration 🫀️plugin-load-progress
  🔬️wgpu-extension-dispatch` **64/64** (`r-vitest-bridge-identity.txt` superseded).
- 02:20 guest request for W2 written: `.tmp-ticket/wp-w1/requests/wg7.txt` (note in the post-landing catalog + the wgpu
  browser activation of the SAME component, SHA equality required by the lease check).
- 02:34 native laws after R4 + approvals parity: `agent_approvals document_relay_tests agent_bridge` **56/56**
  (`b-approvals-laws.txt`), incl. the 3 new relay laws and the fixture-driven affordance law.
- 02:38 browser E2E seed on hub 7800 (W2's canonical, catalog A) with `wp-wg7/wg7-seed.ts` (C10's sealed-command seed,
  copied): space `01a0d5fe-e6ef-78cb-b32f-8619d42315a2` ("wg7 browser collab 023721", user1 owner, user2 seated
  **author**), note document `artifact-d5f22f9f2a8272e23e9d32a3e210aac4` (`note.document`, ready in ~11 s). The browser
  attach uri is `127.0.0.1:7800/01a0d5fe-e6ef-78cb-b32f-8619d42315a2/artifact-d5f22f9f2a8272e23e9d32a3e210aac4`.
- 02:5x WG8 integrated its B2 kind registry into WG7's helper (`document_pack_schema_hash` asks
  `document_kind_codec` — linked, else the mounted component's codec — before the lease). Aligned the two remaining
  WG7 sites: the browser actor's `install_artifact_bootstrap` and the shell's `bind_document_execution_target` both
  resolve `document_kind_codec` (the lease is fetched only when NO codec resolves). Kernel native + renderer native
  `--lib` green 02:56. Browser follow-up (not done): a jco-backed `ComponentDocumentCodec` registered by the browser shell
  would make the mounted guest itself the kind identity (no component-SHA equality with the catalog needed).
- 03:1x network outage cut the turn (coordinator notice). State re-measured 03:25: the queued wasm32 renderer check had
  already run to completion at 03:07 — `r-renderer-wasm32-check.txt`: `Finished`, 0 errors, only peers' dead-code
  warnings. No WG7 process was lost besides the wait loops.
- 03:26 renderer browser wasm build queued (`@semio-tech/framework-renderer-wgpu:wasm` through the wasm mutex, detached pid
  72068, capture `w-renderer-wasm-build.txt`) so the browser E2E can start the moment W2 stages note for wgpu.
- 03:30 measured the identity mismatch the lease check will meet: note dev module (W2 03:06) `wasmSha256 f26fb83c…`,
  release module (09-24) `acfaf580…`, catalog A note `component.sha256 70ba2d15…` — three different builds. Decision:
  the browser E2E serves the **release** variant (`serve note release` + renderer `wasm-release`), and W2 is asked to make
  the note release materialization and the note catalog package one component (request addendum). The dev serve against
  a release catalog is refused by design (`document-execution-target.component-mismatch`, localized notice). Renderer
  build switched to `wasm-release` (pid 73535; my own dev-build wrapper 72068 killed by pid — its TERM trap removed the
  ticket but kept looping, same shape N2 recorded).
- Design note for the coordinator: a dev browser shell collaborating on a release-catalog document needs either the
  hub's closed browser actor mounted as the program (React's `activateBrowserActor` twin — needs a binary host-io door op,
  the directory door is JSON-text-only) or a jco-backed `ComponentDocumentCodec` (needs a `codec` arm in the materialized
  `🌉️bridge.js` + shard worker + ShardClient + JS bridge — an all-plugin restage, W2's). Not started: both collide with
  W2's in-flight restage.
- 03:31 K8 (`wp-wg7/k8-frame-ceiling.py`, apply|revert): the page door refuses any frame over 48 KiB, so one large edit
  made the browser actor's send fail → disconnect → requeue → resend → the same refusal, a reconnect loop that never
  delivers. `DocumentSocket::max_frame_bytes` (door: `SOCKET_DOOR_SEND_MAX_BYTES`); `relay_operations` halves a
  `Commands` batch until its frame fits and rolls back + reports (`CommandOutcome::Rejected`) the one envelope that never
  can. Kernel native green; the code is browser-only, so its proof is the wasm32 check in WG7's next mutex hold
  (`wp-wg7/wg7-wasm-hold.sh`, pid 75488: kernel + renderer wasm32 checks → revert K8 on any error → renderer
  `wasm-release`), capture `w-hold.txt`.
- 03:4x K8 restructured before its hold: the batching is a target-neutral `commands_frames_within(max_frame, first_batch_id,
  local, wire) -> CommandsFramePlan { frames, oversized }` (region `DocumentSocketConnect`), the browser actor only
  sends its frames and refuses its `oversized`. Native law `a_relay_is_split_into_frames_within_the_socket_ceiling`
  (5 envelopes, ceiling ≈ 2 frames: consecutive batch ids from 40, order kept, the 6 KB one is `oversized`; unbounded =
  one batch) — kernel `sync::` **66/66** (`k8-kernel-sync-tests.txt`). wasm32 proof still in the queued hold.
- 05:2x resumed after the usage cut. The 04:03 hold (`w-hold.txt`) ran to completion: kernel `--features sync` wasm32 check
  green (0.29 s — already fresh from a peer's identical check after K8), renderer wasm32 `--lib` green (1 m 23 s), no revert;
  **`wasm-release` renderer built** 04:18 (22.0 MB `_bg.wasm`, includes K1–K8 + R1–R4 + approvals parity). **K8 LANDED**.
- W2's answer (request file, 03:5x): the catalog package is the wasm-RELEASE component, `activate … wgpu dev` stages the
  wasm-DEV one — SHAs differ by construction. Measured 05:25: `serve note release` refuses to start
  (`Missing prepared WGPU artifact …/dist/release/🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts` — the release module root
  is the stale 09-24 staging). So the release lane is not servable today, and the dev lane cannot pass the lease check.
  Next: dev serve (wasm-dev renderer queued, pid 2551) for the measurable half — painted frame, browser hub sign-in through
  the page door, attach → the lease refusal named on screen — while the lease-compatible path is decided (below).
- 05:32 renderer `wasm` (dev) built (2 m 25 s under the mutex). 05:38–05:39 `activate-note-wgpu-dev` under the mutex: nx
  rebuilt note `component-dev` incrementally (18 s, current tree) + re-materialized note dev (browser bridge + descriptor)
  + prepared/activated `dist/runtime/wgpu/dev/note` (`w-activate-note-wgpu-dev.txt`). Single-plugin, not a restage of
  W2's set; noted for W2 in the request file.
- 05:40 **serve up**: `serve note dev --port 6550`, pid **10452** (`wp-wg7/serve-6550.log`), `WGPU browser ready`.
- 05:39 correction for the coordinator's 05:5x notice: the note `component-dev` rebuild at 05:39 WAS this slice's
  `activate-note-wgpu-dev`, and it ran **inside** the wasm mutex (wrapper pid 9548, `fleet-mutex.sh wasm WG7`, capture
  `w-activate-note-wgpu-dev.txt` 05:38:xx→05:39:22). The serve (pid 10452) started after it, at 05:40.
- 05:4x–05:57 browser runs against hub 7800 (`wg7-browser-collab.mjs`, captures `generated/collab-r{1,2}-*`):
  **step 1 painted frame PASS ×2** (64 distinct colours), **step 2 browser hub sign-in PASS for user1** (run 2: `200 POST
  /auth/sessions` + `200 GET /auth/sessions/me` through the page's `directory-http` door; the space rows appear in the
  workspace) — the one hop WG6/WGr never observed. user2 flaked on the known stale-enabled "Add a hub" (WGr's defect).
  Then activating the footer `s-sync-status` **killed the renderer**: `worker-frame-failed: action window instance
  framework.hub has no declared kind`. Root cause: focusing a control of the `/hub` overlay made `framework.hub` the
  `active_window_id` (`retained_surface_is_panel` knew only anchored panels, the overlay leaf has no anchor), and the
  next plugin action resolved its window kind from it. Fix: `retained_surface_is_panel` also answers every
  shell-owned leaf; law `the_hub_workspace_overlay_is_never_the_active_plugin_window` — relay laws **9/9**
  (`r5-relay-laws.txt`). Also measured: with the sync panel opened first, the hub pill did not open the workspace (40 s);
  hub-first works in 2 s — not yet root-caused (probably the same active-window capture), the probe opens the hub first.
- 06:01 wasm-dev renderer rebuild queued with the fix (pid 20198); the probe gained bounded retries for the stale-enabled
  Add / Sign in buttons.
- 06:1x run 3 (`collab-r3-*`, renderer with the active-window fix): **both users sign in through the browser**
  (user1 +42 s, user2 +83 s: `200 POST /auth/sessions`, `200 GET /auth/sessions/me`), no renderer fault any more; the attach
  stopped at the Sync card: activating Remote reached the shell (`Activate … framework.sync.remote`) but the card never
  showed its path input. Root cause: `dispatch_action` returned straight from `handle_sync_action`, so no card verb ever
  republished the mounted Sync panel (the hub workspace republishes via its own `refresh_ui`). Fix: every `framework.sync`
  verb is followed by `republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID)` (the host-owned-panel successor
  path Settings/Theme use). Law `every_sync_card_verb_republishes_the_sync_panel` (selectRemote + setSyncDraft each
  advance the panel revision; the card carries the path input + typed uri) — relay laws **10/10** (`r6-relay-laws.txt`).
- 06:3x run 4 (`collab-r4-*`, renderer with the Sync-card fix): user2 signed in, opened the Sync card, typed
  `127.0.0.1:7800/<space>/<document>`, pressed Attach; the shell fetched the hub's lease
  (`200 POST …/documents/artifact-d5f2…/execution-target/manifest`) and refused the open:
  `frame deferred action failed: document-execution-target.component-mismatch` — the served dev note (`f26fb83c…` lineage)
  is not the catalog's release note (`70ba2d15…`). This is the lease check doing its job; the E2E continues once W2's new
  catalog is up and a note activation from the SAME component is served. user1's sign-in flaked in this run (submit
  activated, no POST) — probe now retries sign-in until a session is minted (≤ 4 attempts).
- Coordinator 06:3x soft freeze acknowledged: none of WG7's changes touch the guest ABI, pack schemas or codec hashes.
- Edit step prep: note's editor declares `addBlock` (and undo/redo); the wgpu command dock lists only shell categories in the
  probe (`wg7-command-probe.mjs`, `generated/command-probe.json`) — the edit will be authored through the note surface
  or a keybinding once the document is live.
- 06:46 W2 rebuilt note `component-release` for catalog B (current tree, `991e840b…`). WG7 queued
  `activate-note-wgpu-release` through the wasm mutex (materialize-release from that component + release renderer + release
  runtime `dist/runtime/wgpu/release/note`), so the browser can serve the SAME note bytes catalog B publishes. Capture
  `w-activate-note-wgpu-release.txt`.
- 07:31–07:49 `activate-note-wgpu-release` in the wasm mutex (17 m, `w-activate-note-wgpu-release.txt`): rebuilt note
  component-release (**70640810…**, not W2's 06:45 **991e840b…** — the tree moved between the two builds; equal bytes need
  one build shared by catalog + served module, request refined for W2 07:55) + release renderer + release runtime.
- 07:5x **release serve precondition fixed** (it could never start: `Missing prepared WGPU artifact …/dist/release/
  🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts.bin`). Root cause: only the legacy dev-runner path
  (`ensureGuestSlimTypstFontsAsset`, dev root only) ever placed the font; the `support-<profile>` targets staged preview2
  shims + shard worker but not the font the serve and the plugin workers' typst path read. Fix (one body):
  `🌐️browser-bundle/🏗️materialization` `ensureGuestSlimTypstFontsAt(moduleRoot, repoRoot)` (+ `guestSlimTypstFontSeed`);
  `SupportScript` stages the font for its profile (refuses when the `semio-framework-os-infinite:fonts` seed is absent);
  the dev-runner path calls the same body (keeps its font-tool build fallback); `support-dev|release` nx targets depend on
  `semio-framework-os-infinite:fonts` and declare the font output. Laws: cache-contract `⚡️cache-contracts` asserts the
  three support outputs + the fonts dependency (graph verified by `nx show project`); the pre-existing
  `🧊️browser-serving` fixture served the font from a `fonts` root the server never mounts — fixture/schema/test now put it
  at `modules/🪞️vendor/…`, and that law now **PASSES** ("WGPU serves completed dev/release artifacts … PASS"). `repo:test`
  still stops later at a peer's `testWgpuBootInputs` (1 !== 4, `🧊️wgpu-browser-boot-cache-inputs`) — not WG7's.
- 07:58 **release serve up**: `serve note release --port 6551`, pid **92356** (`serve-6551.log`).
- 08:0x run 5 against the RELEASE serve (`collab-r5-*`): painted ✓×2, sign-in ✓×2 (first attempt each), both users typed the
  remote uri and pressed Attach, both reached the hub lease (`200 …/execution-target/manifest`) and both were refused
  `document-execution-target.component-mismatch` (catalog A note `70ba2d15…` vs served release note `70640810…`). Every
  browser hop up to the document socket is now measured through the product's own UI; the socket itself waits on a served
  note that IS the catalog's note (W2 request). Dev serve 6550 (pid 10452) stopped; release serve 6551 (pid 92356) kept.
- 08:1x prep for the edit step (local document, release serve): the wgpu shell offers no command path to note's app
  actions (the search palette `mod+p` and the Command dock list shell commands only; `wg7-search-probe.mjs`,
  `wg7-command-probe.mjs`) — a parity gap vs React's `action.<id>` controls, noted, not fixed here. A real authored edit IS
  reachable from the canvas: click the composite, `Meta+a` (selectAll), `Meta+d` (duplicateSelection) → history row
  "Duplicate Selection" (`wg7-edit-probe.mjs`, `generated/edit-probe.json`). The collaboration probe will use it once the
  hub document is live (it needs ≥ 1 block in the hub document).
- 08:1x the "hub pill does nothing after the Sync panel was opened" observation reproduced 2/3, then passed (1 s) once
  (`wg7-order-probe.mjs`): a race, not deterministic — mirror events carry the chrome `window_generation` they were
  published with, and the chrome republishes while a footer panel is open; not root-caused, recorded.
- 10:2x resumed (usage reset). **R8 handoff closed**: the two renderer-wgpu Shell source laws R8 routed to WG7
  (`no_chrome_maintenance_lane_arms_itself_without_pressure`, `the_footer_pills_are_not_gated_off_the_browser_build`)
  pinned the pre-relay gating (`#[cfg(not(wasm32))]` presence arm, `#[cfg(wasm32)] … Detached` pill arm) that the N2 relay
  landing removed on purpose. Updated to pin the new truth: the presence arm carries no target gate and still requires a
  sync channel; the pill is one cfg-free body whose "no status yet" answer is `Remote(Detached)`. Both **pass**; with
  WG7's own laws `chrome_maintenance appearance_tour footer_pills document_relay_tests agent_approvals` **39/39**
  (`r7-laws.txt`, 10:24).
- 10:2x W2 state: batch-B releases done (block, writer, draw, puzzle, wfc), hub built, **catalog B publish failed rc=1**
  at `os-hub:trusted-catalog-bootstrap` (08:34, W2's to diagnose); hub 7800 still on catalog A. WG7's browser E2E waits on
  it (the served release note must be the catalog's note).
- 10:2x edit path found and one more browser-reachable defect fixed: note's Artifact panel offers "Add Text"
  (`note-play-blocks.add.text`, the `addBlock` verb; `wg7-panel-probe.mjs`), but activating it faulted
  `action window instance framework.panel.artifact has no declared kind`: every retained body stamps its own surface id as
  `windowId` (`publish_retained_action`), and the plugin dispatch took a PANEL id for a window instance. Fix: pure
  `action_window_instance_id(requested, panel_leaves, view_window, active_window, first_kind)` — a panel-origin action runs
  in the active window (React's rule, memory "panel actions dispatch in the active window"), and its `windowId` argument is
  not forwarded to the guest. Law `a_panel_action_is_dispatched_in_the_active_window`; relay + R8 laws **25/25**
  (`r8-laws.txt`). The collaboration probe's edit is now "Add Text" and B's witness is a new block row in its own
  Artifact panel. Release renderer rebuild queued (10:28, `w-renderer-wasm-release-2.txt`).
- 10:3x **Add Text authors a real block in the browser shell** (release renderer 10:32 with the panel-dispatch fix):
  `wgpu-shell dispatch action=addBlock scope=full`, the Artifact panel's list gains `note-play-block:text-bbee316f…-0|Text`
  (the id carries the store's replica namespace, H4). Probe witness keys updated to `note-play-block:*`.
- 10:41 W2's catalog B publish failed again (rc=1, 10:39): block's `codec.pack-schema-hash(kit.catalog)` — "artifact codec
  schema is owned by no app of this bundle". W2's/WG8's; WG7 told W2 (request file) that a B without block unblocks the
  browser E2E. WG7 waits on the hub 7800 restart (runId change) with the release serve 6551 up.
- 11:12 W2's third catalog B publish failed too (rc=1 11:11, bootstrap JSON parse EOF in the space-artifact-creation status step); still waiting.
- 11:44 W2 published **catalog B** (`.🧬semio/🌐hub/w2-catalog-b`, generation `e8167ce8…`, rc=0 on the 4th attempt) and hub
  7800 restarted (runId `d4bcc105…`). Catalog B's note component `4567a670…` ≠ the served release note `70640810…` (built
  07:49; the bootstrap builds fresh) — equality by rebuilding is not reachable while the tree moves. WG7's answer, all
  ticket-local: `wp-wg7/wg7-catalog-module.ts` materializes the catalog's OWN note component with the product's
  materialization steps (transpile, descriptor probe, `finalizePluginDescriptor`, bridge) into `wp-wg7/modules/release/`
  (support dirs copied from the release root); `wp-wg7/serve/wg7-serve.ts` serves the product wgpu config with only
  `moduleRoot` replaced. No shared staging is touched. Queued in the wasm mutex (11:5x, `w-catalog-module.txt`).
- 11:55 hub 7800's restart came with a fresh data root, so the E2E was re-seeded on catalog B: space
  `01a0d7fc-fb25-7f88-b9d8-9fd69491925d` ("wg7 browser collab 115433", user1 owner, user2 author), note document
  `artifact-7c32abe8a49e74da909d3017ac6cc690` (`generated/seed-b-*.txt`).
- 12:27 my 11:54 mutex ticket (wrapper 57936) vanished without running (WG8's queued ticket too) while W2 rotated holds; re-queued (12:27).
- 12:26 `wg7-catalog-module.ts` ran in the wasm mutex (43 s): served note `wasmSha256 4567a670…` **== catalog B note
  component** (`w-catalog-module.txt`). 12:27 ticket-local serve **6552** up (pid **2313**, `serve-6552.log`; 6551 pid 92356
  stopped). 12:2x run `b1` (`collab-b1-*`): painted ✓×2, then sign-in failed for both (no `POST /auth/sessions` at all):
  measured 12:37 — **hub 7800 is down** (`readyz` 000, nothing listening). Cause per W2's report: the disk hit 96 % and an
  external prune deleted `wp-w2/generated/` (hub 7800's data root + hold state lived there since its restart). Waiting for
  W2 to bring 7800 back (the space/document seeded at 11:55 died with that data root; re-seed after).
- 12:39–12:45 W2 restarted hub 7800 on catalog B with its data root at `.🧬semio/🌐hub/w2-hub-7800-b` (ready 12:45:06).
- 12:4x coordinator revised preamble rule 15: durable data lives under `.🧬semio/🌐hub/s11-<slice>-*`. Measured the prune's
  effect on WG7: `wp-wg7/generated/` survived, but the prune deleted the **`.core.wasm` files and the typst font `.bin`** inside
  the ticket-local catalog module root (dir mtimes 12:29), so run `b1` also had no loadable note. Moved the root to
  **`.🧬semio/🌐hub/s11-wg7-catalog-modules/release/`** (`wg7-catalog-module.ts` + `serve/wg7-vite.config.ts` repointed), serve and seed
  logs to `.🧬semio/🌐hub/s11-wg7-logs/`. Font restaged with the product helper `ensureGuestSlimTypstFontsAt`. Serve 6552 restarted (pid **29033**).
- 12:47 re-seed on the restarted 7800 (`s11-wg7-logs/seed-c-*.txt`): space **`01a0d82d-8c88-707e-a0e4-c19518553623`** ("WG7 Browser
  Collaboration", user1 owner, user2 author), note document **`artifact-f42c6c013028c961498c18a4074fe2b0`** (ready in ~7 s). Two seed
  fixes: `/directory/spaces` now answers `{access, space}` rows (the seed reads `row.space`), and catalog B names the note creation
  kind **`s.note.note`** (schema `note.document`).
- 12:55 run `c1` (`collab-c1-*`): painted ✓×2, then every step failed. Screenshot: "Plugin unavailable: note … create_app promise failed:
  Failed to execute 'compile' on 'WebAssembly': HTTP status code is not ok" — the pruned `.core.wasm` (above). Not a shell defect.
- 13:06 `wg7-catalog-module.ts` re-queued in the wasm mutex (wrapper pid **44856**, log `s11-wg7-logs/catalog-module-c.txt`), behind W2's
  component-release hold (41766).
- 13:24 catalog-module rebuilt in the mutex (served `wasmSha256 4567a670…` == catalog B again; `.core.wasm` back). Serve 6552 restarted (pid **67896**).
- 13:2x run `c2` (`collab-c2-*`): painted ✓×2, **browser hub sign-in ✓×2** (first attempt, `POST /auth/sessions` 200), Add Text ✓ (A authors a
  `Text` block); attach ✗×2 — the Sync card's Attach stayed **disabled**: the E2E set the path input before the accessibility mirror
  had the textbox live, so the shell never received the uri. Fixed in `wg7-browser-collab.mjs` (`typeInto` retries until the mirror
  input exists and, for the attach path, until the projection's `valueText` equals the uri; `awaitEnabled` waits for Attach to enable).
- 13:3x run `c3` (`collab-c3-*`): B typed the uri, Attach **activated**, the shell fetched the lease (`200 POST …/execution-target/manifest`)
  and refused it: **`document-execution-target.schema-mismatch`** (B console `frame deferred action failed`). A's sign-in missed that
  run (the hub overlay's `ui-doc begin refused … InterruptedClose` churn, the known pill race).
- **Root cause (framework, not the shell):** the attach passes the session app's `io.artifact_schema` as the document schema, and
  the note editor publishes `io.artifactSchema: ""` (lease: `note.document`). Census of the release plugin modules: **~40 apps publish an
  empty `io.artifactSchema`** — the note editor and viewer, every declaration-tree viewer, wfc, dag, puzzle 2d/5d, vcs, reasoning, …
  `PluginBuilder::editor`/`viewer` stamp `E::DOCUMENT_SCHEMA` into an empty `io.artifact_schema` (C8.2 schema-first), but the
  declaration tree's `editor_surface`/`viewer_surface` (`.declare_artifact(…)`, the path note uses) never did. Side effects of the
  gap: `artifact_kind_choices` (Rust + TS) skips apps with an empty io schema, so note offered no local creation choice, and the React
  attach fell back to the breadcrumb (`semio.note`).
- 15:2x **fix landed** (`🔌️plugin/🦀️.rs` `editor_surface`/`viewer_surface` take `mut def` and stamp `E::`/`V::DOCUMENT_SCHEMA` when empty,
  docstrings updated; two codemod typos in those docstrings fixed). Law `every_declared_surface_names_the_schema_it_opens`
  (app-declarations fixture: every projected editor + viewer names its subset's schema). `cargo check -p semio-framework-plugin --lib --tests`
  rc=0 (warnings present), laws **12/12** (`declarations::` + `schema_stamping`, `s11-wg7-logs/plugin-declarations-laws.txt`). No law
  pinned an empty io schema (grep). Soft freeze respected: manifest string only, no guest ABI / pack schema / codec hash change; the hub
  never reads app io. W2 told (`wp-w2/requests/wg7.txt`).
- 15:34 the E2E needs a catalog whose note carries the fix: `wp-wg7/wg7-catalog-n.sh` (detached, pid 42680, `s11-wg7-logs/n-chain.txt`)
  builds os-hub (hub mutex), publishes **stdio,gis,note** into `.🧬semio/🌐hub/s11-wg7-catalog-n` (wasm mutex), materializes that note for
  serve 6552, and boots **hub 8050** on a fresh copy (`s11-wg7-hub-8050`, credentials for user1/user2, W2's hold script, state in
  `s11-wg7-hub-8050-state`). Hub 7800 is untouched.
- 15:3x **A's lost sign-in in c3 diagnosed**: the pill's mirror `Activate` reached the shell (`start_dispatch enter`) and nothing followed —
  `handle_accessibility_event` refuses a chrome event whose `window_generation` is not `presented_input_epoch`, and that epoch advances on
  EVERY presented frame. Measured at idle on 6552 (`wg7-chrome-generation-probe.mjs`, `generated/chrome-generation-probe.json`): **28 chrome
  generations in 15 s with ONE distinct node content**, and the ARIA mirror was a generation behind in **20/60 samples** — so a screen-reader
  or mirror activation was silently dropped about a third of the time, and the mirror's focus restore (keyed on the generation) broke every frame.
- 15:4x **fix landed** (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`): new `presented_chrome_accessibility_generation` — the epoch at which the chrome's
  projection last CHANGED; `acknowledge_presented_input` republishes the chrome accessibility (and advances the generation) only when the
  projection differs; the chrome gate compares against it. Laws: the old `a_delayed_chrome_mirror_address_cannot_activate_after_its_presented_epoch_retires`
  (which pinned the churn) is replaced by `a_mirror_address_stays_live_across_frames_that_present_the_same_chrome` and
  `a_delayed_chrome_mirror_address_cannot_activate_after_the_chrome_changes` (the safety property kept: a changed chrome retires old addresses);
  chrome targets in the theme-editor and palette laws address the new generation. Native `--lib --tests` check rc=0 (594 warnings);
  **nextest 61/61** (`shell_shortcuts_palette_tests`, `theme_editor_and_accessibility_tests`, `presented_input_authority_tests`,
  `wgpu_introspection`; `s11-wg7-logs/renderer-a11y-laws-nextest.txt`). Plain `cargo test` in one process fails 17 of these with "candidate
  could not be sealed" — the known process-global leak (R1: nextest is the runner), not this change. Renderer wasm-release queued in the
  wasm mutex (wrapper 55434, `s11-wg7-logs/renderer-wasm-release-a11y.txt`); it is also the wasm32 compile of this change.
- 15:5x language-neutral contract: `🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json` `presentedChrome.generationRule` (control +
  first / same-chrome / changed-chrome rects; note extended) with its schema (`🧬️schema/…/🔣️.json`, `$defs.rect`); both Rust laws now read
  the rule; vitest `♿️wgpu-accessibility-interaction` **13/13** (Ajv schema validation of the fixture + rule sanity) and nextest **61/61** again.
- 16:1x the WG7 chain's hub build finished (15:39, 1m17s); its publish ticket waits in the wasm FIFO behind W2's `restage2` hold (15:23,
  `materialize-dev` over 51 projects, parallel 2), then the renderer wasm-release (55434). WG8 note: its open item 4 (wgpu
  `os.create-space-artifact` kind choices) depends on `artifact_kind_choices`, which skipped every app with an empty `io.artifactSchema`
  — the 15:2x stamp makes note/wfc/dag/… choosable.
- 16:58 **WG7 catalog published** (33m46s): `.🧬semio/🌐hub/s11-wg7-catalog-n`, profile `local-stdio-gis-note-open-v1`, generation
  `f6d193e1…`, note component `ac4b77f7…`, open targets `s.note.note@1/*#editor|#viewer` → `note.document`.
- 17:09 **hub 8050 up** (`wg7-hub-8050.sh`, split out of the chain; hold **28307**, os-hub **28313**, data `s11-wg7-hub-8050`, signed binary copy
  `s11-wg7-bin/os-hub-8050`; readyz 200). Seed (`s11-wg7-logs/seed-n-*.txt`): space **`01a0d91e-03c1-71fd-ba2e-28ee3db6c9e9`** (user1 owner,
  user2 author), note **`artifact-ffe16f4e05776efdde9a094f3796af77`**.
- 17:11 renderer wasm-release built with the a11y fix (11m44s, EXIT=0 — also the wasm32 compile of it). Serve 6552 restarted (pid **30112**).
  **Runtime proof:** `wg7-chrome-generation-probe.mjs` at idle now measures **1 chrome generation in 15 s, mirror behind 0/60** (was 28 and 20/60).
- The served-note step (catalog `ac4b77f7…` note into the durable module root) waits in the wasm FIFO behind W2's restage2 hold (825).

## Files changed (WG7)

Kernel `semio-framework-os-kernel`:
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — `ureq` native-only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` — K1 admission in three shared legs, browser
  `HubSocketGrantSource` impl (`DocumentSocketAdmissionFuture`), `protected_post_target/_answer/_async`, pub `decode_lower_hex_32`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` — `ArtifactId` import, regions `DocumentSocketConnect`
  (+ K8 `commands_frames_within`) and `DocumentSocketDoor`, `ArtifactHost` dialer wiring, browser actor rewrite.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-socket-connect/🦀️.rs` (new, WG8 extended it),
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/document-socket-connect-v1/🔣️.json` (new).
Renderer `semio-framework-os-renderer-wgpu` (paths under `📺️renderer/🧑‍🎨engine/`):
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — N2 relay hunks; browser transports; dialer install; sign-in lends the
  hub to the document host on both targets; `bind_document_execution_target` + `document_execution_target_admitted`;
  `parse_remote_backbone_uri`; space-index route un-gated; `retained_surface_is_panel` covers shell leaves; Sync card
  republish; `action_window_instance_id`; approval overlay paints `approval_row_lines`.
- `🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — N2 bridge hunks; `component_sha256`; `ProgramPackageIdentityV1`.
- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — N2 relay hunks (aliases, one pump).
- `🎯️targets/🧊️wgpu/🔌️socket-door/🦀️.rs` — `DoorDocumentSocketDialer`/`DoorDocumentSocket`.
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` — N2 relay hunk.
- `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` — `packageId`/`componentSha256`, `packageIdentity()`;
  `🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js` regenerated.
- `🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` — `approval_row_lines` + labels; fixture
  `🧱️elements/🤖️AgentApprovals/🧫️fixtures/🛡️summary/🔣️.json` (`affordance` rows).
- Laws: `🧱️elements/🐚️Shell/🧪️tests/📂️wgpu-document-relay/🦀️.rs`, `🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs`
  (relay), `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` (from_wasm arity),
  `🧱️elements/🐚️Shell/🧪️tests/💓️chrome-maintenance-pressure/🦀️.rs`, `🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs`,
  `🧱️elements/🤖️AgentApprovals/🧪️tests/🔬️wgpu-unit/🦀️.rs`, `🧱️elements/🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx`,
  `🧪️tests/🧩️package-integration/🟦️.ts`, `🧪️tests/🫀️plugin-load-progress/🟦️.ts`.
Kernel TS: `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — `fetchPackageDescriptor`.
Renderer a11y: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`presented_chrome_accessibility_generation`), laws `🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`,
`🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs`, fixture `🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json`, schema
`🧬️schema/♿️wgpu-accessibility-interaction/🔣️.json`, vitest `🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`; probe `wp-wg7/wg7-chrome-generation-probe.mjs`.
Plugin framework `semio-framework-plugin`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`editor_surface`/`viewer_surface` stamp the
document schema), `🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs` (law `every_declared_surface_names_the_schema_it_opens`).
Plugin web / tooling: `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` (`ensureGuestSlimTypstFontsAt`,
`guestSlimTypstFontSeed`), `…/🚀️commands/🟦️.ts` (support stages fonts), `🔌️plugin/🏗️build/📦️materialization/🟦️.ts`,
`🔌️plugin/📦️packages/🟦️typescript/📋️project.json` (support → fonts); caching laws `🦑️repo/…/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`,
`…/🧪️tests/🧊️browser-serving/🟦️.ts`, `…/🧫️fixtures/🧊️browser-serving/🔣️.json` + `📐️schema/🔣️.json`.
Ticket (`wp-wg7/`): `k1-directory-admission.py`, `k2-k7-browser-actor.py`, `k4-wasm-actor.rs.txt`, `k8-frame-ceiling.py`,
`r1-r3-browser-document-wiring.py`, `r4-remote-backbone-uri.py`, `b1-approval-affordance-parity.py`, `wg7-wasm-hold.sh`,
`wg7-seed.ts`, `wg7-browser-collab.mjs` + probes (`wg7-keys-probe`, `wg7-command-probe`, `wg7-search-probe`,
`wg7-order-probe`, `wg7-edit-probe`, `wg7-panel-probe`, `wg7-addtext-probe`), `wg7-catalog-module.ts`, `serve/wg7-serve.ts`,
`serve/wg7-vite.config.ts`, `wg7-catalog-n.sh`; `wp-w1/requests/wg7.txt`, `wp-w2/requests/wg7.txt`.

## Processes (WG7)

- Serve `note release (catalog-B note)` on **6552**: pid **29033** (running since 12:47; log `.🧬semio/🌐hub/s11-wg7-logs/serve-6552.log`;
  module root `.🧬semio/🌐hub/s11-wg7-catalog-modules/release/`). Earlier serves: 6550 pid 10452, 6551 pid 92356, 6552 pid 2313 — all stopped by me.
- Inherited hub 7900 (hold 74207 / os-hub 74210): untouched (not used; the E2E runs on W2's 7800).
- Serve 6552 pid **30112** (17:11; earlier 67896, stopped). Hub **8050**: hold 28307 / os-hub 28313 (`s11-wg7-hub-8050-state/pids.txt`).
- Chain parent 42680 stopped by me at 17:08 (the hub step moved to `wg7-hub-8050.sh`); its queued served-note mutex wrapper **21478** runs on its turn.
- Mutex wrappers of this slice: 44856 (done 13:24). Killed earlier (mine): 72068, 73535.

## From WG8 (17:4x) — genesis-on-open and the relay test case

- **Genesis-on-open (native, landed, WG8 gate run 12 → 18 green):** a fresh door-created artifact answers the document socket
  `Welcome { bootstrap: None }` (the hub db holds no command yet; the genesis lives only in the artifact store), so a guest that
  never loaded the document authors envelopes under its **app id** (`s.block.block2d@1/*#editor`) and the document actor refuses
  every edit as `document backbone scope mismatch`. Fix in the target-neutral `open_document` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`):
  a **hub-bound** open first loads `store_sync::os_store::component_document_genesis(schema, document_id)` into the guest — the
  same `codec.genesis` export and zero-history check the hub's trusted catalog seeds the artifact from. Local ids are skipped (the
  component refuses a genesis for a non-server-minted id). On wasm32 it applies once a `ComponentDocumentCodec` is registered for
  the kind (native registers `OwnedComponentDocumentCodec` in `create_app`); the browser shell needs a jco-backed twin of that
  registration (the store trait now has `genesis`) or the identity arrives some other way — please check the browser path with a
  fresh door artifact: the editor's first `Commands` envelope must carry `documentId = artifact-…`.
- **T12 contract rows (13):** `🐚️Shell/🧪️tests/📂️wgpu-document-relay` renamed to `🔀️wgpu-document-relay` (📂️ is one of the
  taxonomy's generic emoji identities, so the case name and all 11 test bodies in it counted as non-canonical); `#[path]` and the
  file's docstring emoji updated. Layout probe on the whole Shell tree: 0 findings (old name reproduces 12); relay laws 11/11.
