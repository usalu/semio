# WP-G10 — Working AI Integration Over The Semio MCP (Session 11)

Slice G10 · session 11 · 2026-09-25. Ports: hubs 8030–8039, serves 6530–6539. Captures: `wp-g10/generated/`.
Owns outcome 4 (semio MCP `semio-framework-os-mcp`). Inherits G9 (`📓️wp-g9.md`).

## Status

| # | Item | Status |
|---|---|---|
| 1 | Landing: G9 commit binding (`wp-g9/g9-apply-commit-binding.py`), compile-atomic | DONE 00:52 — native + wasm32 green (`📓️landing.md`); Rust service law 6/6, TS/AJV oracle + inference-bridge-source-check green; schema mirror regenerated |
| 2 | Generic inference quartet live (hub gis + guest wfc, approval → commit) | **DONE LIVE.** Guest site 11/11 (folder, bitmap `pin-solution`). **Hub site 19/19** on my catalog-B hub :8030 (`s11-g10-logs/quartet-live-4.txt`, 15:4x): grid3d solve in the guest (result, no proposal), cancel, gis relayed to the hub, offer withdrawn, approve → commit → `head_seq 0→1`, agent B sees it. Getting there took four root fixes: the hub ledger's SQL bound, the MCP's per-document dialect, the hub runtime stack budget and the MCP pair-receipt join (Log 12:50–15:4x) |
| 3 | Suites: client-e2e, hub-agent-participant, live-agent-loop, MCP conformance | **client-e2e 38/38** (05:26); MCP TS 61/61; Rust quick 396/396, plus 128/128 after today's MCP changes (`workspace:: inference:: artifact::`); live-agent-loop 22/23 en + de (the red row is the frontend boot race); **hub-agent-participant 17/17 on 7800 catalog B** (15:25, re-measured after the sweep) |
| 4a | Zero-touch MCP client connect to running `s` session (re-prove) | LIVE (en+de): `.mcp.json` argv verbatim → offer → shell dials (rows 0, a); two zero-touch defects root-fixed (stamp lost on every build + dependency-blind hash → `sourcesRecord`; bridge offered only if `dev s` was up first → always offered) |
| 4b | Delegated agent principal edits shared hub doc, human sees it live | **Agent half LIVE, browser half BLOCKED.** Agent half on :8030 catalog B: principal `agent:<delegationId>`, `addBlock` SUCCEEDED, hub `head_seq 0→1`; the same wire is proven 21/21 by the durability gate. Human half: the note window's remote attach is refused by the shell with no hub request ("The document target changed"). The cause is catalog skew: the staged dev lane ≠ catalog B, and W2's 14:36 restage ended with 12 cores missing. The `s` Space-app path hits the index socket 404 (C10 F5). `g10-agent-edit-human-sees.ts` now also drives S15's `s` space route (`G10_OPEN=space`); it runs once the lane matches |
| 4c | Destructive verbs need approval visible in the shell (re-prove) | **LIVE on the real `s` host** (en 22/22, `live-agent-loop-5.txt`): (e1) Approve Once in the shell affordance (countdown 120), (e2) Deny → PERMISSION_DENIED channel=shell, (e3) silent-client elicitation timeout → APPROVAL_REQUIRED, (i18n) en; de 21/22 with (i18n) de PASS — its one red is a transient boot beacon (below) |
| 4d | React shell renders live agent tool-call transcript (en + de, Playwright; re-prove) | **LIVE en 22/22 on the `s` host** (note spawned through the command palette): rendezvous, shell dials, presence tone, running→ok rows, Cancel, ui_reveal/ui_focus, (f1)–(f8) chain visible in the live DOM; de 21/22 (`live-agent-loop-de-2.txt`) — every agent/transcript row PASS, `boot` red on a transient shell error beacon |
| 5 | Duplicate `semio://workspace/artifacts` resource entry | DONE (source + law): one owner per URI, dedupe mask deleted; new law green; live `resources/list` 0 duplicates (client-e2e row) |
| 6 | P0-2 (audit-s11-hub): `hub-edit-durability-check` on a catalog-A hub; catalog A carries H4? | **DONE: 21/21 on catalog B** (13:48, own hub :8031 from a credentialed clone, binary `1a5cf10…`): 3 lanes (2 agents via fresh MCP processes + 1 human socket), distinct mutation ids, relay, catch-up, and restart durability (`head_seq 3→3`). The gate now prints the hub's output tail and stops its hub on a boot failure |
| 7 | G-P1-1: launch rows for `hub-agent-participant-check`, `agent-reply-check` (+ every MCP gate run) | DONE (both files; 437/370 unique names) |
| 8 | G-P1-3 roster half: extension-contributed inference services in the roster; stdio descriptor request | STALE — live `inference_list` already lists 68 stdio gltf services + `cad-extension-aec-building` |
| 9 | G-P2-2: presence/roster marks an agent session as agent (declared principal kind on the wire), en + de | **Wire LIVE**: durability row 5p shows both agents in the human's roster as `:agent` principals (hub-stamped `principal_kind`, from the presence beat I added). The React PresenceBar badge (en/de) is unit-proven; its live browser proof is blocked with 4b |
| 10 | G-P2-3: in-product action → hub-bound MCP client config with delegated, revocable agent credential | **DONE LIVE en 8/8 + de 8/8** (`mcp-client-config-{en-3,de-1}.txt`, screenshot `g10-mcp-client-config-en.png`): shell sign-in → delegation → Set up MCP client → 0600 credential + config (no token) → Copy → a real MCP client from that exact entry is `agent:<delegation>` in the space → Withdraw removes the file and the hub refuses the old credential. Schema + law + AJV replay, pane/hook laws 90/90 |
| 11 | G-P1-4 spot-check `capabilities_search "draw rectangle"` | DONE: top hit `draw…addLayer` 20.1 vs 8.96, described; architect/wfc hits still have `description: ""` |
| 13 | U5 item 4: schema-first approval withdrawal on the gateway→shell wire (cancel/timeout/superseded); React + wgpu retire the affordance; law + live en/de | **DONE**: wire tag 11 `ApprovalWithdrawn{approvalId, reason: cancelled|timed_out|superseded}` (fixture rows first; Rust SSOT ×5 codec sites, TS twin, wgpu twin); gateway withdraws on cancel/countdown and moves a request to a newer shell (`superseded`, remaining time); law `🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json` replayed by Rust (real coordinator, 4 cases) + React hook (5) + wgpu (5); panel en/de laws; os-mcp policy/bridge/ui 92/92, agent-bridge-check 66/66, chat panel + approvals 35/35, wgpu agent 49/49. **Live** (c2) en + de PASS: cancel → affordance withdrawn, no countdown/decisions, localized reason |
| 12 | P1-6 (audit-s11-os-frontend): architect `setAdjacencyKind` duplicate capability id | NOT PRESENT: descriptor has it once; live catalog compiles, one hit; client-e2e "zero duplicate ids" |

## Session 12

Session 12 (2026-09-25 22:5x, preamble `📓️session-12-preamble.md`). Captures `.🧬semio/🌐hub/s12-g10-logs/`; durable data
`.🧬semio/🌐hub/s12-g10-*`. Ports: hubs 8030–8039, serves 6530–6539.

| # | Item | Status |
|---|---|---|
| S1 | 4b browser half: delegated agent edits a shared hub doc over the semio MCP, signed-in human sees it live in the React `s` shell (Space index → note → agent's block), en + de; PresenceBar agent badge | **DONE LIVE on 7800 B2 (05:0x)**: **en 8/8** (`agent-edit-en-13.txt`), **de 8/8** (`agent-edit-de-1.txt`, screenshot `g10-agent-edit-human-sees-{en,de}.png`): human signs in in the `s` shell → Home → Space index lists the hub-created note → opens it live → agent `agent:<delegation>` opens + commits `addBlock` → the human's roster shows **"Claude G10 en (AI agent, Editing)" / "Claude G10 de (KI-Agent, Bearbeitet)"** (`data-presence-kind=agent`, G-P2-2 live) → the human's canvas gains the agent's block `data-ink-block-id=text-…-0` (+ navigator row) 2–16 ms after the Commands frame → hub `head_seq 0→1`. Fixed on the way: S9 (agent commit relay acknowledgement) |
| S2 | live-agent-loop 23/23 en + de on the current tree; client-e2e; MCP conformance TS + Rust; hub quartet; durability; hub-agent-participant (on 7800 B2) | **Local half DONE on the final tree (09:0x–10:3x)**: live-agent-loop **en 26/26, de 26/26** (`live-agent-loop-{en-10,de-3}.txt`), agent-reply **9/9**, client-e2e **39/39** (`client-e2e-8.txt`, restage 16), MCP TS **70/70** (`mcp-ts-7.txt`), Rust quick **462/462** (`mcp-rust-quick-4.txt`; run 5 on the final tree in the log). **Hub half**: hub-agent-participant **17/17** on 7800 B2 (04:07); quartet on 7800 best **17/19** — the two reds are hub-side (7800 answers agents' execution-target requests `503 deadline-exceeded` after a commit; gis approval answered `503 inference.unavailable` while the ledger advanced), reported; the gateway defects it exposed are fixed (S11). Durability gate: running on its own B2 hub (log) |
| S3 | All plugins over MCP: per-package coverage of `capabilities_search`/`inference_list`/`artifact_create`/`action_invoke` (description authoring moved to slice D1 by the coordinator) | **DONE 01:4x, measured (run 4, folder lane, current tree + S7/S8/fault mapping, `--auto-approve all`)**: 35 packages, 1548 plugin capabilities, 74 declared inferences (stdio 67, wfc 5, cad 1, gis 1); `artifact_create` **71/71** `s.*` kinds; `action_invoke` **64/71** kinds. The 7 reds: cad, flow, sequence, space home (P8), and s.home, s.space, playbook procedural (no mutation of their own kind). Table: "### S3 per-package coverage". Fixed on the way: S7, S8, fault mapping |
| S4 | User path, zero-touch, timed, en + de: sign in → Set up MCP client → real client connects, lists tools, edits a hub doc with approval in the shell → revoke | **en 7/8 on 7800 (09:2x, `user-path-7800-en-6.txt`, fresh page load)**: clean profile signs in (15.9 s) → "Set up MCP client" → the **official `@modelcontextprotocol/sdk` Client** over `StdioClientTransport` from exactly the shown entry connects in 2.3 s (28 tools) → edits the hub note → destructive `deleteSelection` → shell approval "Approve Once" → **19.5 s from first page load to the approved agent edit** (the shell-attach grace holds live). **Blocked on U5 (reported twice)**: (a) "Set up MCP client" stays `installing` in 2 of 3 runs (en-5, de-1) although the credential file is written — `🔗️HubConnection` drops the install result when the operation owner is no longer current and has no terminal phase for it; (b) after the approval the agent pane is unbound and the overlay reopened from the palette says "Not signed in to a hub" above the user's own spaces, so the revoke row is unreachable. de: same step-2 stall. Revoke itself: G-P2-3 (session 11, 8/8 en + de) |
| S5 | Remaining `audit-s11-ai-mcp.md` items in scope (+ the session-12 audits' items routed to G10) | **DONE 10:4x, each with evidence**: G-P0-1 → 4b 8/8 en + de (S1); G-P0-2 → quartet 17/19, the two reds hub-side (S2/S11); G-P1-1 launch rows and G-P2-5 duplicate resource → session 11 rows 7/5; G-P1-3 roster half → session 11 row 8 (stale, live lists stdio + cad-extension); G-P2-2 agent principal → 4b roster "Claude G10 en (AI agent, Editing)"; G-P2-3 in-product MCP config → S4 (official SDK client from the shown entry). Routed by `audit-s12-os-frontend.md`: "approval affordance lingers after a cancel" → closed live, live-agent-loop row (c2) en-10/de-3 "Withdrawn — the agent's request was cancelled", no countdown, no decisions; "architect `setAdjacencyKind` duplicate capability id" → closed, headless `capabilities_search "set adjacency kind"` returns `…#editor.setAdjacencyKind` once, 0 duplicate ids in 50 hits (`🗑️generated/s12/adj-probe.py`). Routed by `audit-s12-build-convergence.md`: "orphaned `dev mcp stdio` build" → already in tree (`SEMIO_BUILD_OWNER_PID`: the staging build cancels itself when its owner is gone). Not G10's: G-P1-2/G-P1-4 (descriptions → D1), G-P2-1/G-P2-4 (S15) |
| S6 | Coordinator (audit-s12 G12-P1-2): prompt-injection surface | **DONE 00:2x (local + shell lanes)**: schema-first envelope `semio.mcp.untrusted-content/v1` (`UntrustedContentV1` + `UntrustedProvenanceV1`: document, kind, space, revision `contentSha256`/`headEditId`/`commitSeq`, authors `local-principal`/`space-writers`, fixed en/de notice) is the ONLY carrier of document bytes: `artifact_snapshot`, `semio://artifact/{id}`, `artifact_export`, hub `…/checkpoint`, hub `semio://workspace` space entry. `initialize.instructions` + tool/resource descriptions state it (en — de). Law fixture `🗿️artifact/🧫️fixtures/🧷️untrusted-content-law.json` (canary, carriers, observers, 7 hostile envelopes): Rust quick law green, TS process law **6/6** (AJV + `node:crypto`), live agent loop **en 26/26, de 26/26** with (g1) canary only inside `untrusted` (deflate-inflated), (g2) the destructive follow-up it demands → shell affordance → Deny → `PERMISSION_DENIED`, (g3) block survives. Hub lanes (checkpoint/space entry) re-run on 7800 B2 |
| S7 | Coordinator: headless lane answered INTERNAL (`owner-mutation payload did not decode … Truncated(0)`) for every verb whose preview has no op (selection verbs with nothing selected, effect-only verbs such as note `setFixtureJson`) | **DONE 00:3x**: `🔀️dispatch` invoke answers a committed no-change (`SUCCEEDED`, `warnings:[no-change: …]`, revision unchanged, no undo token, audit `no_change`) and never opens a guest transaction over an empty op list; a saga skips such members. Law `actions::quick::an_action_whose_preview_produced_no_operation_commits_nothing_and_says_so` (28/28 `actions::`); client-e2e row **39/39** (`client-e2e-4.txt`). Guest half (typed refusal for `Effect::LoadDocument` previews) prepared, not applied: `wp-g10/g10-preview-effect-refusal.py`, request in `wp-w1/requests/g10.txt` |
| S8 | S3 finding, MCP-owned: the headless lane opened ONE guest per plugin (its first editor app), so every verb of a multi-app plugin's other apps was refused (`action app owner s.block.block3d@1/*#editor does not match s.block.block2d@1/*#editor`) and `artifact_create` of those kinds persisted the FIRST app's genesis document under the other kind's schema | **DONE 00:4x**: routing keys are `AppRoute{plugin, app}` (channels, instance slots, session artifacts, backbone relay); `artifact_create`/`artifact_export` open the kind's own app; plugin-scope verbs run on the plugin's default route. Laws: `every_app_of_a_multi_app_plugin_routes_to_its_own_instance_slot` + the routing integration law rebuilt on the REAL note/cad descriptors (it had routed synthetic app ids) 7/7; `workspace:: actions:: artifact::` 100/101 before the integration-law rebuild, the one red was that law. Live: coverage run 3 (below) |
| S9 | Found by 4b on 7800 (API-only reproducer `wp-g10/g10-agent-relay-probe.ts`): a hub-bound agent's `action_invoke` answered `SUCCEEDED` while the hub head never moved (1 of 3 rounds, 3 of 6 browser runs under load ~50): the guest committed, the envelopes sat in the document actor whose link was not live, and the gateway process could exit before any relay | **DONE 05:0x**: a commit that relayed envelopes waits (≤ `HUB_RELAY_ACK_WAIT_MS` 10 s) for a status the document actor reports AFTER the relay (live link, 0 pending, acknowledged head) and answers `postconditions: ["edit:…", "relay:acknowledged"]` or `warnings: ["relay-pending: link …, N pending; last fault: …"]`; `artifact_open.sessionDocument.sync` reports `{remote, pendingMutations, acknowledged, lastFault}` (actor `Status`/`Conflict` events, e.g. an expired link). Law `a_hub_commit_is_acknowledged_only_by_a_live_status_reported_after_its_relay`; probe 4/4 `relay:acknowledged` + head 1; 4b en/de 8/8. Root of the unacknowledged runs (hub admission `open-plan`+`socket-grants` over the actor's 5 s budget under load, then the 60 s link expiry) is now visible in `lastFault`, not yet proven |
| S10 | Coordinator (from U5): pid 88810, the coordinator's old `dev mcp stdio os`, accepts TCP on its bridge port and never answers; the shell redials forever | **Gateway half DONE 05:4x, shell half DONE 09:0x, live-proven**. Root cause (measured on 88810: exactly 64 `CLOSED` sockets): `start_bridge_only` kept the `Handback` terminal policy of the full HTTP transport, so every finished non-upgrade request parked its socket, and after `HTTP_CONNECTION_CAPACITY` (64) `accept_one` never ran again. Gateway: bridge-only listener closes terminal sockets; every accepted connection is answered within `BRIDGE_OPENING_DEADLINE_MS` (10 s) — at capacity a typed `503` `semio.mcp.transport-refusal/v1`, a `Hello` of another version a typed `Refused{version, gatewayVersion}` frame (new tag 12, fixtures `0c000100`/`0c010100` replayed by Rust, TS and wgpu codecs); the offer file is withdrawn when the listener's run completes; law `transport::long::a_bridge_only_listener_answers_every_connection_it_accepts` (70 connections, v0/v2 refused, silent socket closed). Shell (React + wgpu twin, one fixture `🔗️AgentBridge/🧫️fixtures/🤝️handshake/🔣️.json`): 8 s handshake deadline from the dial, at most 3 unanswered dials per offer, a typed refusal or a foreign `Welcome` ends the offer at once; statuses `unavailable`/`incompatible` render ONE localized footer notice (en/de, names both versions); a different offer restarts the ladder. Laws: React `useAgentBridge handshake` 8/8 (bridge suite 80/80), AgentPresence 15/15 (suite was in no include list — registered). Live on `:6530`: silent stand-in (88810's wire behaviour) → exactly 3 dials, notice at 28.7 s (en) / 32.7 s (de), 0 console errors, no dial after; v2 stand-in → 1 dial, typed refusal, "incompatible" notice en + de, 0 console messages. See log 08:4x–09:0x |
| S11 | Found by the quartet on 7800: a hub-bound `inference_approve` answered retryable `PLUGIN_UNAVAILABLE` while the hub ledger had advanced (the agent is told to retry an edit that landed); every hub tool failed at once in the window after any directory event; failed authority refreshes were retried back-to-back against a busy hub; the refusal named nothing | **DONE 10:3x (os-mcp)**: (1) `HubRemoteBinding` announces every state/authority/catalog transition on a Condvar; hub-bound entry points (`settled_hub_binding`, component resolve, `open_hub`) wait ≤ `HUB_AUTHORITY_SETTLE_WAIT_MS` (10 s = one refresh turn's deadline) for a refresh in flight, then their gate still fails closed. (2) `approve_hub_job` checks its base precondition before the relay; after the hub receipt it answers success with named `warnings` (`undo-unavailable`, `events-unavailable`). (3) The refusal carries `phase` + `lastFault`; `HubBindingError::Unavailable` keeps the HTTP status and the hub's typed refusal code (that is how `503 deadline-exceeded` was found). (4) Failed refreshes back off 100 ms → 5 s (cancel-aware). Laws `a_call_during_an_authority_refresh_waits_for_it_and_fails_closed_only_after_the_wait`, `failed_authority_refreshes_back_off_and_saturate`; remote + inference **73/73** (`settle-law-2.log`) |

### S3 per-package coverage

Harness `wp-g10/g10-plugin-coverage.ts` (a fresh `--folder` space, staged binary, `--no-bridge`): per package, every
capability via paginated `capabilities_search {owner}` + `capabilities_describe`, `inference_list`, `artifact_create` of every
installed kind of the package, then prepare + invoke of the first verb of that kind whose required args it can supply.
Run 3 (01:0x, after S7 + S8; `coverage-3/coverage-rows.jsonl`, `coverage-table.md`):

| package | capabilities | mutations | destructive | empty description | mutations declaring no args | inferences | artifact_create ok/kinds | action_invoke ok/created |
|---|---|---|---|---|---|---|---|---|
| animate | 14 | 11 | 5 | 14 | 7 | 0 | 1/1 | 1/1 |
| architect | 25 | 13 | 3 | 20 | 4 | 0 | 1/1 | 1/1 |
| block | 37 | 27 | 11 | 37 | 27 | 0 | 3/3 | 3/3 |
| cad | 26 | 14 | 2 | 0 | 13 | 1 | 1/1 | 0/1 |
| dag | 11 | 9 | 2 | 11 | 8 | 0 | 1/1 | 1/1 |
| demonstrator | 166 | 84 | 21 | 139 | 69 | 0 | 1/1 | 1/1 |
| draw | 15 | 14 | 5 | 0 | 2 | 0 | 1/1 | 1/1 |
| energy | 28 | 19 | 3 | 23 | 0 | 0 | 1/1 | 0/1 |
| fem | 60 | 56 | 4 | 60 | 7 | 0 | 2/2 | 0/2 |
| flow | 24 | 18 | 4 | 20 | 17 | 0 | 1/1 | 0/1 |
| forms | 20 | 18 | 6 | 0 | 15 | 0 | 1/1 | 1/1 |
| gis | 34 | 10 | 3 | 22 | 3 | 1 | 2/2 | 2/2 |
| home | 0 | 0 | 0 | 0 | 0 | 0 | 1/1 | 0/1 |
| imperative | 14 | 8 | 2 | 10 | 7 | 0 | 1/1 | 1/1 |
| layout | 18 | 8 | 1 | 0 | 7 | 0 | 1/1 | 1/1 |
| lowpoly | 56 | 31 | 3 | 47 | 20 | 0 | 1/1 | 1/1 |
| mathematical | 7 | 6 | 2 | 7 | 2 | 0 | 1/1 | 1/1 |
| norm | 90 | 30 | 30 | 60 | 0 | 0 | 15/15 | 0/15 |
| note | 20 | 18 | 4 | 0 | 15 | 0 | 1/1 | 1/1 |
| playbook | 8 | 7 | 2 | 8 | 6 | 0 | 2/2 | 1/2 |
| procedural | 72 | 26 | 7 | 59 | 23 | 0 | 2/2 | 2/2 |
| process | 24 | 18 | 5 | 20 | 15 | 0 | 1/1 | 1/1 |
| puzzle | 153 | 71 | 12 | 153 | 61 | 0 | 3/3 | 3/3 |
| raster | 20 | 10 | 2 | 0 | 9 | 0 | 1/1 | 1/1 |
| reasoning | 5 | 4 | 2 | 5 | 4 | 0 | 1/1 | 1/1 |
| remodel | 31 | 28 | 10 | 31 | 12 | 0 | 1/1 | 1/1 |
| sequence | 16 | 12 | 3 | 16 | 10 | 0 | 1/1 | 0/1 |
| shooting | 39 | 25 | 2 | 35 | 22 | 0 | 1/1 | 1/1 |
| sourcing | 7 | 6 | 3 | 7 | 4 | 0 | 1/1 | 1/1 |
| space | 73 | 28 | 5 | 68 | 24 | 0 | 4/4 | 1/4 |
| stdio | 291 | 18 | 9 | 18 | 0 | 67 | 6/7 | 0/6 |
| trinity | 20 | 15 | 5 | 20 | 5 | 0 | 2/2 | 1/2 |
| vcs | 6 | 5 | 1 | 6 | 4 | 0 | 0/1 | 0/0 |
| wfc | 108 | 92 | 17 | 103 | 36 | 5 | 5/5 | 5/5 |
| writer | 10 | 9 | 4 | 10 | 5 | 0 | 1/1 | 1/1 |

Run 4 (01:2x–01:4x, `coverage-4/`; `--auto-approve all` so destructive verbs run too, required args synthesised by type,
900 s create budget):

| package | capabilities | mutations | destructive | empty description | mutations declaring no args | inferences | artifact_create ok/kinds | action_invoke ok/created |
|---|---|---|---|---|---|---|---|---|
| animate | 14 | 11 | 5 | 14 | 7 | 0 | 1/1 | 1/1 |
| architect | 25 | 13 | 3 | 20 | 4 | 0 | 1/1 | 1/1 |
| block | 37 | 27 | 11 | 37 | 27 | 0 | 3/3 | 3/3 |
| cad | 26 | 14 | 2 | 0 | 13 | 1 | 1/1 | 0/1 |
| dag | 11 | 9 | 2 | 11 | 8 | 0 | 1/1 | 1/1 |
| demonstrator | 166 | 84 | 21 | 139 | 69 | 0 | 1/1 | 1/1 |
| draw | 15 | 14 | 5 | 0 | 2 | 0 | 1/1 | 1/1 |
| energy | 28 | 19 | 3 | 23 | 0 | 0 | 1/1 | 1/1 |
| fem | 60 | 56 | 4 | 60 | 7 | 0 | 2/2 | 2/2 |
| flow | 24 | 18 | 4 | 20 | 17 | 0 | 1/1 | 0/1 |
| forms | 20 | 18 | 6 | 0 | 15 | 0 | 1/1 | 1/1 |
| gis | 34 | 10 | 3 | 22 | 3 | 1 | 2/2 | 2/2 |
| home | 0 | 0 | 0 | 0 | 0 | 0 | 1/1 | 0/1 |
| imperative | 14 | 8 | 2 | 10 | 7 | 0 | 1/1 | 1/1 |
| layout | 18 | 8 | 1 | 0 | 7 | 0 | 1/1 | 1/1 |
| lowpoly | 56 | 31 | 3 | 47 | 20 | 0 | 1/1 | 1/1 |
| mathematical | 7 | 6 | 2 | 7 | 2 | 0 | 1/1 | 1/1 |
| norm | 90 | 30 | 30 | 60 | 0 | 0 | 15/15 | 15/15 |
| note | 20 | 18 | 4 | 0 | 15 | 0 | 1/1 | 1/1 |
| playbook | 8 | 7 | 2 | 8 | 6 | 0 | 2/2 | 1/2 |
| procedural | 72 | 26 | 7 | 59 | 23 | 0 | 2/2 | 2/2 |
| process | 24 | 18 | 5 | 20 | 15 | 0 | 1/1 | 1/1 |
| puzzle | 153 | 71 | 12 | 153 | 61 | 0 | 3/3 | 3/3 |
| raster | 20 | 10 | 2 | 0 | 9 | 0 | 1/1 | 1/1 |
| reasoning | 5 | 4 | 2 | 5 | 4 | 0 | 1/1 | 1/1 |
| remodel | 31 | 28 | 10 | 31 | 12 | 0 | 1/1 | 1/1 |
| sequence | 16 | 12 | 3 | 16 | 10 | 0 | 1/1 | 0/1 |
| shooting | 39 | 25 | 2 | 35 | 22 | 0 | 1/1 | 1/1 |
| sourcing | 7 | 6 | 3 | 7 | 4 | 0 | 1/1 | 1/1 |
| space | 73 | 28 | 5 | 68 | 24 | 0 | 4/4 | 2/4 |
| stdio | 291 | 18 | 9 | 18 | 0 | 67 | 7/7 | 7/7 |
| trinity | 20 | 15 | 5 | 20 | 5 | 0 | 2/2 | 2/2 |
| vcs | 6 | 5 | 1 | 6 | 4 | 0 | 1/1 | 1/1 |
| wfc | 108 | 92 | 17 | 103 | 36 | 5 | 5/5 | 5/5 |
| writer | 10 | 9 | 4 | 10 | 5 | 0 | 1/1 | 1/1 |

Run 4's 7 red kinds: cad, flow, sequence, space home = the P8 guest faults below (space home also has `set-cell`
"typed command has no exact controller/owner", same class); `s.home`, `s.space`, `s.playbook.procedural` declare no
mutation of their own kind (nothing to invoke; not a defect).

Invoke failures by cause (run 3, before run 4's harness changes):

| cause | kinds | owner |
|---|---|---|
| harness could not supply required free-form args (no default/enum) | energy, fem 2d/3d, stdio html/json/md/tsv/txt/xml, trinity jack, space home/space | harness (run 4 synthesises by type) |
| every mutation of the kind is destructive (harness skipped destructive) | norm ×15, s.home, s.space, playbook procedural | harness (run 4 uses `--auto-approve all`) |
| `interactive-job.not-ui-safe` (`BatchOnlyPendingRewrite`): the verb is published to agents but its guest refuses the agent lane | cad importCadFile, architect runReport/importProgram, space importSpace | **blocked: BatchOnly (P8)**: coordinator decision 01:2x, the commands are dead for humans too and get MIGRATED (not unpublished) by P8; meanwhile the MCP answers PLUGIN_UNAVAILABLE instead of INTERNAL |
| cad preview: "the artifact view is not bound to a public command operation" | cad duplicateObject/addNode/patchSelection | **blocked: P8 lane parity** (coordinator 05:1x: the SDK's `preview_addressed_action` runs `A::handle` directly while the shell lane runs the retained tool-job work; P8 fixes the preview, no gateway patch) |
| flow: "Flow retained routes execute only through their exact app-owned job factory" | flow addWidget/patch/rename/disconnect | **blocked: P8 lane parity** |
| sequence: `sequence-content-child-dialect-required` | sequence addStep/reorganize/connect | **blocked: P8 lane parity** |
| `s.home.session-identity-required` | space home createStudio | P8 |
| `mutation.target-missing` for a default id | space renameArtifact/touchArtifact | correct refusal; MCP now answers PRECONDITION_FAILED instead of INTERNAL |
| client timeout 240 s on a cold create | stdio csv (7 stdio kinds share one 200 MB component), vcs | measured again in run 4 with 900 s |

Also measured: note `patchBlocks/deleteBlock/moveBlock/duplicateBlock` declare no args (coordinator routed to T12); writer
`setText` output cap 64 op bytes (≈38 chars of text) → the MCP now answers INPUT_INVALID (`interactive-job.preview-output`)
instead of INTERNAL; empty descriptions are D1's.

**Fault mapping (01:1x, `🔀️dispatch` `map_fault`)**: `interactive-job.not-ui-safe` → non-retryable PLUGIN_UNAVAILABLE,
`interactive-job.preview-output` → INPUT_INVALID, `transaction.member-rejected` → PRECONDITION_FAILED; every other unknown
code stays INTERNAL (law `every_fault_code_maps_to_the_right_gateway_error_code` extended; `actions::` 28/28).

### Session 12 Pids

| pid | what | started | stopped |
|---|---|---|---|
| 94695 (→ bun 94698, vite 94700) | `s` react dev serve :6530 local-only, private rendezvous `.🧬semio/🌐hub/s12-g10-bridge` (`g10-serve.sh s 6530`) | 23:04 | 08:58 (stale offer endpoint) |
| 19583 | `s` react dev serve :6530 local-only, same rendezvous (restart: the node-side offer endpoint still answered the pre-U5 untyped body) | 08:58 | running |
| 42075 | `s` react dev serve :6531 bound to hub 7800 (`g10-serve.sh s 6531 http://127.0.0.1:7800`) | 04:1x | running |
| 19662 / 20447 | bridge stand-ins `g10-bridge-double.py` silent / v2-refusal (offer in my rendezvous only) | 08:58 / 09:01 | 09:01 / 09:02 |
| 15420 | wgpu twin unit laws (`g10-wgpu-bridge-unit.sh`, private target) | 08:55 | running |
| 25626 | os-mcp restage (`g10-mcp-build.sh`, `mcp-build-10.txt`) | 09:03 | running |
| 45457 / 47633 / 68937 / 94060 | coverage harness runs 1–4 (run 1 stopped for the rendezvous incident, run 2 stopped as baseline) | 00:24–01:4x | all ended |
| 2602 | hub hold → os-hub :8030 (catalog-B clone `s11-g10-hub-8030-b`, binary `s11-g10-bin/os-hub-stack` 98dea583…), state `.🧬semio/🌐hub/s12-g10-state-8030` — rehearsal of S1/S4 while 7800 is down | 01:4x | 04:0x (7800 up) |

### Session 12 Log

- 22:53 resumed. Nothing of mine runs. 7800 down until W2 publishes catalog B2. Staged `semio-os-mcp` fresh (662 sources unchanged since
  19:36). The Space index guest link (`fold-directory-events` selects the one folded space when the index `space_id` is empty) is in
  the tree since 17:55, so restage4 (18:56) carries it.
- 22:58 MCP TS suite **61/61** (`mcp-ts-1.txt`, 63 s). 23:0x client-e2e **38/38** (`client-e2e-1.txt`, 256 s).
- 23:04 `s` serve :6530 local-only (pid 94695, `serve-s-6530.txt`: `[fresh] 60 staged components match their sources`).
  `g10-serve.sh` now keeps the bridge rendezvous and credentials under `.🧬semio/🌐hub/s12-g10-{bridge,credentials}`: the
  session-11 dirs `wp-g10/{bridge,credentials}` were tracked paths (`git check-ignore` says so), wrong for a 0600 credential.
- 23:06 os-mcp Rust quick **453 passed, 34 skipped** (`mcp-rust-quick-1.txt`, private target).
- 23:0x live-agent-loop run 1 red (10/22): my env named the host (`S_OS_MCP_LIVE_PLUGIN=s`), so the gate found no `s` mutation
  and picked dag's destructive verb. The gate's contract is `PLUGIN=note SPAWN=note` on the `s` host (script fixed).
  Run 2 **en 23/23** (`live-agent-loop-en-2.txt`), **de 23/23** (`live-agent-loop-de-1.txt`): `boot` green in both locales
  (S15's boot fix holds), (c2) withdrawal, (e1) countdown 120, (e2) `PERMISSION_DENIED channel=shell`, (i18n) both locales.
- 23:1x coordinator: description authoring → slice D1; new item S6 (prompt-injection envelope, G12-P1-2).
- 23:2x–00:2x **S6 landed** (design + files in the S6 row; files in "Files Changed (G10, session 12)" below). Measured:
  `cargo check -p semio-framework-os-mcp --lib --tests` green after each edit; laws `artifact:: schema::` + the checkpoint law
  18/18 (`untrusted-law-1.txt`); schema mirror regenerated (72 exports, strict AJV resolves all) and `--check` green;
  `canonical-checkpoint-resource-check` green (oracle now verifies the envelope, `untrusted=1`); TS process law 6/6
  (`untrusted-ts-4.txt`); MCP TS suite **69/69** (`mcp-ts-2.txt`); Rust quick **456/456** (`mcp-rust-quick-2.txt`);
  client-e2e **38/38** (`client-e2e-3.txt`; run 2 was 37/38 with an intermittent wfc guest `inference_run`
  `SIDE_EFFECT_REJECTED` at progress 0.35, green on rerun, not reproduced since, nothing in S6 touches inference);
  live agent loop **en 26/26** (`live-agent-loop-en-7.txt`), **de 26/26** (`live-agent-loop-de-2.txt`).
  On the way (measured, recorded for S3): (a) note's `patchBlocks`/`deleteBlock`/`duplicateBlock`/`moveBlock` declare NO args,
  so their MCP input schema is `{}` with `additionalProperties:false`: an agent cannot set a block's text or delete a block
  by id at all; (b) writer `setText`'s declared output cap is 64 op bytes (a text of ~38 chars), so the law's canary is 37
  chars; (c) the headless (`--folder`) lane answers `INTERNAL "owner-mutation payload did not decode … Truncated(0)"` for
  any effect-only command (note `setFixtureJson`); the shell lane applies it. The note text lives in deflate streams inside
  pack/spr, so the live gate inflates every stream in the envelope to find the canary.
  A peer's `🏪️store/🔄️sync` edit at 23:5x made the staged MCP stale: my live-gate run 5's gateway restaged it (>240 s
  initialize timeout, gate red), its orphan finished the build and exited on EOF; rerun green.
- 00:2x coordinator: S7 (headless effect-only INTERNAL) → G10; note args → T12. S3 coverage harness `wp-g10/g10-plugin-coverage.ts`.
  **Incident (mine):** coverage run 1 (00:24–00:27) spawned its gateway without `--no-bridge`, so it offered itself in the
  per-user rendezvous and a peer's live `dev s` session dialled it (`context_resolve` → `channel=shell`); 50 `artifact_create`
  calls went to that peer's shell and were refused (`refused ReadArtifact`), no document was changed. Stopped (pids 45457/45459,
  mine) at 00:27; every harness now passes `--no-bridge`. The peer's chat panel may show those refused tool calls.
- 00:2x–00:3x S7 landed (row S7). MCP binary restaged (88 s). (Earlier log lines that say 00:4x–00:5x for S7 meant 00:2x–00:3x.)
- 00:3x–00:5x S8 landed (row S8), found by coverage run 2 (baseline, pre-S7/S8 binary, 21 packages measured before I stopped it).
  Rule 17 (BG_NICE): my detached processes start through `g10-detach.py` (python `Popen`, new session), not zsh `&`, and run at
  nice 0 (measured: serve 94695/94700 `NI 0`).
- 00:5x client-e2e run 5: 38/39, `inference_run` wfc bitmap `SIDE_EFFECT_REJECTED` again (2 of 5 runs, both while another heavy
  run of mine shared the machine). The same call standalone succeeded twice (110 s and 73 s under load, `infer.ts` probe). The
  client-e2e row now prints the error's code, message and details in full, so the next red names its cause.
- 01:1x–01:4x coverage runs 3 + 4 (S3 row, table below); local suites re-measured after S6–S8 (S2 row).
- 03:4x official MCP SDK 1.30.0 (S4's real client) validates `structuredContent` against `outputSchema` even for `isError`
  results, so every typed tool error failed client-side. Root fix: `schema::admit_tool_errors` publishes every tool output
  schema as `anyOf[success, typed tool error]` with `$defs` kept at the root (first attempt nested `$defs` and broke every
  `#/$defs` ref → SDK `tools/list` failed). Law `a_tool_output_schema_admits_its_success_shape_and_the_typed_tool_error`
  (`tool-error-law-1.txt`); MCP TS **70/70** (`mcp-ts-5.txt`, includes the SDK typed-error row).
- 03:59 7800 READY on B2. 04:07 hub-agent-participant **17/17** (`hub-agent-participant-7800-b2.txt`). Hub hold 2602 stopped.
- 04:1x–04:4x 4b runs en-1…12: driver fixes (Commands-frame baseline taken BEFORE the commit — a baseline after it had
  miscounted "1→1"; `[data-ink-block-id]` witness; space-open retry; roster/head waits 90 s) and the S9 finding.
- 05:0x S9 landed (`relay-law-1.txt`, restage 8); 4b **en 8/8, de 8/8** (S1 row); relay probe 4/4.
- 05:1x–05:4x S4 on 7800 runs 1–4 (S4 row). The intermittent "no OS shell attached" was a race: the approval coordinator
  looked for a shell before the shell had dialled the new gateway's offer. Fix: `SHELL_ATTACH_GRACE_MS` 35 s, only while a
  live os session exists, cancel-aware; law `a_shell_that_attaches_within_the_grace_is_asked` (`grace-law-1.txt`, policy +
  bridge 60/60). Not yet re-run live (needs the restage below).
- 05:3x coordinator (from U5): bridge-handshake defect, S10 row. Gateway half + law `bridge-law-1.txt` by 05:4x. My restage 9
  (05:41) rotated `dist/build.stage-dUd3Xu.previous` away before the instruction to reproduce with an old-build copy, and
  88810 runs from that deleted inode, so no old-build copy exists; the live repro uses stand-ins with 88810's exact wire
  behaviour (listens, never accepts) and a v2 gateway (typed refusal). 88810 untouched.
- 08:40 resumed after the usage-limit stop. Rule 20 (guest freeze) noted: everything below is os-mcp + renderer; the wgpu
  renderer crate (`semio-framework-os-renderer-wgpu`) is linked by no guest (only a ticket probe depends on it).
- 08:4x–08:5x S10 shell half (React hook, AgentPresence, chat panel, footer notice, wgpu twin, fixture). React laws: bridge
  suite **80/80** (`vitest-bridge-2.log`), AgentPresence + chat panel **29/29** (`vitest-presence-1.log`); `typecheck` of the
  React renderer clean, `--listFiles` shows every edited file.
- 08:58 serve :6530 restarted (mine): its node-side `/__semio/agent-bridge` still answered the pre-U5 `{"error":…}` body,
  which the current shell never parses as an offer. Now `{"schema":"semio.os.agent-bridge-offer/v1","offered":false}`.
- 08:58–09:02 S10 live (`🗑️generated/s12/bridge-live-*.json` + `.png`, stand-in logs `double-*.jsonl`):
  silent stand-in, en: dials at 1.4 s / 10.3 s / 20.3 s, each retired at +8.0 s, notice "The AI client's bridge does not
  answer; restart the AI client to connect again" at 28.7 s, nothing dialled in the remaining 41 s, `consoleErrors 0`
  (Chrome prints one browser-native *warning* per retired dial, "WebSocket is closed before the connection is established",
  3 in total). de: same shape, notice "Die Brücke des KI-Clients antwortet nicht; …" at 32.7 s, 3 dials, 0 errors.
  v2 stand-in: 1 dial each for en and de, the shell's `Hello` carried `bridgeVersion 1`, the stand-in answered `0c000200`,
  notice "The AI client uses bridge version 2, this shell version 1; update the older one" / "Der KI-Client nutzt
  Brückenversion 2, diese Oberfläche Version 1; aktualisiere die ältere", 0 console messages of any kind.
- 09:03 os-mcp restage 10 (51 s). Real new gateway, live (`hold-new.jsonl`): 70 finished HTTP requests on its bridge port all
  answered, 0 `CLOSED` sockets held, the 71st answered in 7 ms; the shell on :6530 dialled its offer once and was welcomed
  (frame tag 0), no notice, 0 console messages. Closing its stdin removed the offer file (`offerLeft:false`); a SIGTERM-killed
  gateway's leftover file is ignored by the supervisor's liveness check (`offered:false`).
- 09:0x suites on restage 10 + fresh :6530: MCP TS **70/70** (`mcp-ts-6.txt`), client-e2e **39/39** (`client-e2e-7.txt`),
  live-agent-loop en 25/26 → row `0 rendezvous` still read `pid` from the offer answer, which U5's typed
  `semio.os.agent-bridge-offer/v1` no longer carries; the live-agent-loop and agent-reply gates now read the answer with the
  shell's own `parseAgentBridgeOffer`/`sameAgentBridgeOffer` (a new gateway = new url + proof). Then live-agent-loop **en 26/26**
  (`live-agent-loop-en-10.txt`), **de 26/26** (`live-agent-loop-de-3.txt`), agent-reply **9/9** (`agent-reply-1.txt`), Rust
  quick **462/462**, 35 skipped (`mcp-rust-quick-4.txt`).
- 09:2x S4 on 7800 (serve :6531, fresh page loads): en-5 2/3 and de-1 2/3 — "Set up MCP client" stays `installing` although
  the credential file is written (09:23:13, 09:26:40): `🔗️HubConnection` `installAgentMcpClient` drops the result when
  `operationOwnerCurrent(owner)` is false and has no terminal phase for it. en-6 **7/8**: sign-in 15.9 s, setup, official SDK
  client connects in 2.3 s (fresh binary), edits the note, destructive request → shell approval "Approve Once" at 30.3 s,
  **19.5 s from first page load to the approved agent edit** (the S4 shell-attach grace holds live); red: revoke — after the
  approval the agent pane is unbound (space "", path /hub) and the overlay reopened from the palette says "Not signed in to a
  hub" above the user's own space list, so no delegation row. Reported to the coordinator for U5 (both, with captures).
- 09:3x quartet on 7800 run 1 **17/19** (`quartet-7800-1.txt`): row 14 `inference_approve` answered retryable
  `PLUGIN_UNAVAILABLE` "descriptor index is refreshing" while row 16 shows the hub ledger advanced 0→1 — the agent was told to
  retry an approval that had landed; row 17 agent B read headSeq −1. Root: every directory event in the bound space
  `invalidate`s the authority and every hub call in the refresh window failed at once. Fixes (os-mcp): `HubRemoteBinding`
  announces each state/authority/catalog transition on a Condvar and hub-bound entry points wait for a refresh in flight
  (`HUB_AUTHORITY_SETTLE_WAIT_MS` = one refresh turn's deadline, 10 s) before their gate fails closed; `approve_hub_job` checks
  its base precondition BEFORE the relay and, once the hub receipt exists, reports success with named `warnings`
  (`undo-unavailable`/`events-unavailable`) instead of an error; the refusal now names `phase` and `lastFault`. Law
  `a_call_during_an_authority_refresh_waits_for_it_and_fails_closed_only_after_the_wait` (remote 14/14, `settle-law-1.log`).
  Run 2 (restage 11): approval answers success (no retry signal), but `job` null — the binding was still refreshing after the
  10 s wait and 20 s later; restage 12 carries the diagnostics to name why.

### Session 12 Files Changed

| path | change |
|---|---|
| `🌉️mcp/🧬️schema/🦀️.rs` (+ regenerated `🔣️.json`, `🟦️.ts`) | `UNTRUSTED_CONTENT_SCHEMA`/`NOTICE`, typed provenance, `untrusted_content()` builder, shapes `UntrustedContentV1`/`UntrustedProvenanceV1` (exports 70 → 72); snapshot/export output shapes carry `untrusted` |
| `🌉️mcp/🏠️workspace/🦀️.rs` | `untrusted_artifact_provenance`, `artifact_body` (folder + hub bodies), hub `semio://workspace` space entry enveloped, resource descriptions |
| `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` | hub checkpoint resource: `pack`/`spr` keep `byteLength`+`sha256`, bytes move into `untrusted` |
| `🌉️mcp/🏠️workspace/🧬️schema/🔣️.json`, `🧫️fixtures/🔐️canonical-checkpoint-resource/🔣️.json`, pair unit law | checkpoint contract + fixture + law follow the envelope (cross-document `$ref` to `UntrustedContentV1`) |
| `🌉️mcp/🗿️artifact/🦀️.rs` + quick law | export enveloped; en/de descriptions; law `document_authored_content_reaches_an_agent_only_inside_the_untrusted_envelope` |
| `🌉️mcp/🗿️artifact/🧬️schema/🔣️.json`, `🧫️fixtures/🧷️untrusted-content-law.json` (new) | `UntrustedContentLawV1` + the law fixture |
| `🌉️mcp/🧪️tests/🧷️untrusted-content/🟦️.ts` (new) | process law over the real binary (AJV, `node:crypto`) |
| `🌉️mcp/🧭️protocol/🦀️.rs` | `SERVER_INSTRUCTIONS` in both `initialize` results |
| `🌉️mcp/🧠️context/🦀️.rs` | artifact resource template description + mime |
| `🌉️mcp/🟦️.ts` (client-e2e) | reads content from the envelope |
| `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` | envelope reads; (g1)–(g3) prompt-injection rows; `decide` takes a capability + input |
| `🌉️mcp/📦️packages/🦀️rust/📜️script.ts` | checkpoint oracle verifies the envelope; `workspaceContract` registers the os.mcp component |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (2 readers) | hub checkpoint gates read bytes from `untrusted.content` |
| `🌉️mcp/README.md` | "Document content is untrusted data" |
| `🌉️mcp/🔀️dispatch/🦀️.rs` + quick laws | S7 no-change invocation (`PreparedOps::is_empty`, `NO_CHANGE_WARNING`, saga skips empty members), mock `force_empty_preview`; fault mapping `interactive-job.not-ui-safe`/`preview-output`, `transaction.member-rejected` |
| `🌉️mcp/🏠️workspace/🦀️.rs` + quick laws | S8 `AppRoute` routing (channels, slots, session artifacts, backbone relay, create/export per app); routing integration law rebuilt on real descriptors; new multi-app law |
| `🌉️mcp/🟦️.ts` (client-e2e) | no-change row; `inference_run` row prints the full error |
| ticket `wp-g10/g10-serve.sh`, `g10-live-agent-loop.sh`, `g10-mcp-rust-quick.sh`, `g10-plugin-coverage.ts`, `g10-user-path.ts`, `g10-preview-effect-refusal.py` | s12 paths, gate env, private-target niced quick run, S3 harness, S4 user path, prepared guest patch |
| `🌉️mcp/🚚️transport/🦀️.rs` + long law | S10: bridge-only listener closes terminal sockets, opening deadline, typed `503` capacity refusal, `Refused{version}` on a foreign `Hello`, run completion |
| `🌉️mcp/🧵️bridge/🦀️.rs`, `🟦️.ts`, `🧫️fixtures/📨️frames.json` | S10: `GatewayToShell::Refused` (tag 12) + `BridgeRefusal`, fixture rows `0c000100`/`0c010100` |
| `🌉️mcp/🦀️.rs` | S10: the stdio bridge offer is withdrawn when the listener's run completes |
| `🌉️mcp/🛡️policy/🦀️.rs` + law | S4 race: `SHELL_ATTACH_GRACE_MS` while a live os session exists |
| `🌉️mcp/🧬️schema/🦀️.rs`, `🧭️protocol/🦀️.rs` + law | every tool output schema admits the typed tool error (`anyOf`, `$defs` at the root) — official SDK client |
| `🌉️mcp/🏠️workspace/🦀️.rs`, `🔀️dispatch/🦀️.rs`, `🗿️artifact/🦀️.rs` + law | S9 relay acknowledgement (`HubRelay`, `relay:acknowledged` / `relay-pending`, `sessionDocument.sync`) |
| `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` + unit laws, `🏠️workspace/🦀️.rs` | S11: settle Condvar + `await_settled`, `settled_hub_binding`, named refusal (`phase`, `lastFault`, HTTP status + code), refresh backoff |
| `🌉️mcp/💡️inference/🦀️.rs` | S11: hub approval checks preconditions before the relay and never errors after the receipt |
| `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`, `🧪️tests/💬️agent-reply/🟦️.ts` | rendezvous row reads the typed offer answer through the shell's own parser |
| `📺️renderer/…/🔗️AgentBridge/🟦️.tsx` | S10 shell: handshake deadline from the dial, ≤ 3 unanswered dials per offer, `refused`/foreign `welcome` end the offer, `unavailable`/`incompatible` + `versionMismatch`, en/de labels, `BRIDGE_VERSION` in `hello` |
| `📺️renderer/…/🔗️AgentBridge/🧫️fixtures/🤝️handshake/🔣️.json` (new) + React laws | the language-neutral handshake scenarios (React + wgpu replay them) |
| `📺️renderer/…/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` + wgpu-unit | wgpu twin: tag 12, `Unavailable`/`Incompatible(mismatch)`, dialer deadline + bounded unanswered dials; backoff law rewritten for the bounded ladder |
| `📺️renderer/…/🚦️AgentPresence/🟦️.tsx` + wgpu twin + both test suites | `blocked` tone, localized unavailable/incompatible texts (versions named) |
| `📺️renderer/…/💬️AgentChatPanel/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx` | mismatch passed through; one footer notice while the offer is unavailable/incompatible |
| `📺️renderer/…/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the AgentPresence suite (it ran nowhere) |
| ticket `wp-g10/g10-bridge-{hold.py,double.py,live.ts}`, `g10-wgpu-bridge-unit.sh`, `g10-mcp-build.sh`, `g10-exec-target-probe.ts`, `g10-durability.sh`, `g10-user-path.ts`, `g10-quartet-live.ts` | S10 live proof, wgpu laws, restage, hub probe, durability run, S4 pane trace, s12 capture dir |

## Pids

| pid | what | started | stopped |
|---|---|---|---|
| 77159 | note react dev serve :6530 local-only, private bridge dir `wp-g10/bridge` (`g10-serve-note.sh`) | 01:05 | 01:47 (restart for the new vite plugin) |
| 9172 | same serve :6530, restarted | 01:47 | 05:37 |
| 9500 / 9505 | hub hold (`g10-hub-hold.ts`) / os-hub :8030 (W2's binary copy `wp-g10/bin/os-hub`, data `wp-g10/hub-8030` = catalog A + user1/user2) | 01:49 | 01:56 (hold's 300 s readiness stall bound fired at load ~60) |
| 40569 / 40570 | same hub :8030, hold with a 30 min stall bound | 02:31 | 05:37 (catalog-A guests are refused by the current tree) |
| 53404 | note react dev serve :6531 bound to hub :8030 (`S_HUB_URL`), private bridge + credentials dirs | 02:45 | 05:22 |
| 55419 | `s` react dev serve :6532 bound to hub :8030 (`g10-serve.sh`) | 02:46 | 05:37 |
| 9117 | `s` react dev serve :6530 local-only, after W2's `activate-s-react-dev` (`g10-serve.sh s 6530`) | 05:37 | 06:10 |
| 58874 | `s` react dev serve :6530 local-only (approval-withdrawal live proof) | 07:15 | 07:29 |
| 59867 | `s` react dev serve :6530 bound to hub 7800 (catalog B) | 11:58 | 12:23 |
| 79030 | `note` react dev serve :6531 bound to hub 7800 | 12:14 | 12:23 |
| 96492 | `note` react dev serve :6531 bound to my hub 8030 | 12:23 | gone by 12:33 (external) |
| 99401 / 99402 | hub hold / os-hub :8030, restart on the same root | 12:26 | gone by 12:33 (external; data root deleted) |
| 16657 / 16662 | hub :8030 on a catalog-B clone under `generated/` | 12:35 | 12:44 (moved to `.🧬semio/🌐hub/s11-g10-*`) |
| 26272 / 26278 | hub :8030 (`s11-g10-hub-8030-b`, binary `1a5cf10…`) | 12:44 | 13:20 |
| 63430 / 63433 | hub :8030 on `os-hub-ledger` | 13:20 | 13:30 (SIGABRT stack overflow on approve) |
| 87757 / 87764 | hub :8030 on `os-hub-stack` | 13:49 | 15:43 |
| 18261, 22706 | note serve :6531 (hub not yet ready / restart) | 12:34 | 12:44 |
| 31928 | note serve :6531 → hub 8030 (log `s11-g10-logs/serve-note-6531.txt`) | 12:50 | 15:43 |
| 80631 / 80632 | hub hold / os-hub :8030 on my copy of catalog B (`generated/hub-8030-b`, binary = shared-target os-hub sha 1a5cf10…, W2's catalog-B build) | 12:18 | 12:23 killed externally |

## Log

- 00:42 landing applied (dry run clean); 00:47 native green; 00:52 wasm32 green; os-mcp service law 6/6 (`mcp-targeted-1.txt`).
- 01:09 client-e2e run 1 died on the schema mirror (`HubInferenceApprovalRequestV1` export pending since G9) → `schema-mirror`
  regenerated; `inference-bridge-source-check` green (oracle + TS/AJV service law). client-e2e run 2: 16 PASS, 1 FAIL — the
  `artifact_create` guest instantiation refused by H9's new `codec.replay-envelopes` export (current-tree host, pre-01:13 guest).
- 01:15 hub-edit-durability run 1: row 0 (own hub on catalog-A copy, W2 binary) + row 1 PASS, then `invalid intent`: the gate
  assumed a pre-authored space. Gate now creates its own space (zero-touch) and gained an agent-presence row (5p).
- 01:50 os-mcp TS suites: 3 reds — (1) G9's `inference_submit` description broke the one-` — ` en/de convention, (2) a law that
  pinned the pre-G9 "folder workspace has no inference" semantics, (3) tier-2 cross-talk: test gateways offered their bridge into
  the per-user rendezvous and a peer's live `dev s` dialled them. All three root-fixed (description; law updated to the generic
  model; vitest `env.S_AGENT_BRIDGE_DIR` = per-run temp dir). 02:30: **60/60**.
- 01:54 live-agent-loop run 1: the `.mcp.json` gateway spent >240 s of `initialize` re-staging a binary nx had built 1 min
  earlier. Root cause: the staged dir is published atomically, so ensureMcpBinary's `.content-hash` stamp was deleted by every
  build, and the hash only covered `🌉️mcp/**` (blind to kernel/plugin-host changes such as H9's ABI). Fix: `buildCargoArtifacts`
  `sourcesRecord` stages Cargo's own dep-info closure (`semio.cargo.binary-sources/v1`: builtAtMs + 661 sources) beside the
  binary in the same publication; `ensureMcpBinary` stat-checks it (74 ms warm). Fixture cases (dep-info ×4, freshness ×3) +
  restage-after-dependency-change law.
- 02:1x zero-touch: a stdio gateway only offered its bridge when a `dev s` session was already live at launch (opening the MCP
  client before the shell needed a reconnect). Now it always offers (unless `--no-bridge`); refusal texts + README updated.
- os-mcp Rust `quick::` 396/396 (`mcp-lib-quick-1.txt`), after fixing H9's stale refusal-text expectation (one line, test only).
- 02:5x G-P2-3 live: run 1–2 were probe defects (sign-in form selector; the `s` host needs its space route settled before the
  pane binds). Run 3 (en) and de: **8/8** each.
- 02:5x coordinator: an orphaned `dev mcp stdio os` (ppid 1) held a `cargo build` in a build-dir flock cycle for 59 min. Product
  defect, root-fixed: `buildCargoArtifacts` watches `SEMIO_BUILD_OWNER_PID` and cancels (process-group SIGTERM → SIGKILL) the
  moment its owner is gone; `ensureMcpBinary`'s staging passes the gateway's pid. Measured: build with a dead owner cancels in
  2 s (`owner-watch-1.txt`), no cargo left; law `a staging build lives exactly as long as the process it was started for`;
  os-mcp TS 61/61. Bun's `process.ppid` is live after a parent SIGKILL (measured: 62492 → 1).
- 03:1x cut by a network outage; 03:25 resumed. Orphan sweep: none of mine (only my recorded serves/hub, all intended).
- 03:28 hub-agent-participant made zero-touch (default origin = the `dev s` hub 8787; creates its own space + note through the
  hub's authorities unless `OS_MCP_HUB_SPACE`). On hub 8030: 15/17, rows 11/12 = the guest-ABI refusal. resources/subscribe live
  push is covered by client-e2e's own rows (`resources/subscribe` → `notifications/resources/updated after the commit`), which
  were green in G8's 38/38 — the audit's "unverified live" is stale; rerun blocked by the same ABI refusal.
- 03:33–03:51 quartet live on hub 8030 (catalog A, current-tree os-mcp; `wp-g10/g10-quartet-live.ts`, G9's script re-pointed to a
  fresh space, staged binary, `ready.artifactId`): 5/18. wfc half: no wfc in catalog A (expected; W2). **gis half found a real
  defect**: `inference_submit` → hub 413 `inference.bounds`. Root cause: hub.inference `INPUT_MAX_BYTES` = 64 KiB, but every map
  the hub creates packs to 80,801 B (measured by `artifact_snapshot`), so no freshly created map was ever inferable on the hub.
  Root-fixed 05:23 (1 MiB; schema const, JSON `InferenceLimitsV1`, TS twin, neutral fixture, and the fixture-pinning law now
  also asserts the kind's default map fits); `cargo check -p semio-hub --lib --tests` green; the MCP's Bounds explanation
  (which blamed "the 1024-byte intent") now names the declared bounds. Hub rebuild requested (`wp-w1/requests/g10.txt`).
  `gis-inference-ledger-oracle` is red at `gis-native-provider-selection` ("exact GIS package") — unrelated to this change
  (the inference/proposal oracles before it pass: `gis-controlled-proposal-oracle bounds=1`); not mine, noted for W2/hub.
- 05:20 resumed after the usage limit. Stopped serve 6531 (pid 53404 tree). No orphans of mine.
- 05:26 client-e2e **38/38** (W2's stage chains restaged every guest from the current tree, so the ABI refusal is gone locally).
- 05:28 guest-site quartet live over stdio MCP (folder lane, staged wfc): **11/11**.
- 05:28 live-agent-loop run 3 on serve 6530: (e3) silent-client elicitation timeout now **PASS** (fresh staged note), but the (f)/(e1)/(e2) shell route
  refused (`no open instance` of `note` in the attached shell): W2's `plugin-registry:generate` (05:28–05:32) rewrote the generated
  registry under my live serve. Environment, not product: rerun on a serve started after W2's `activate-s-react-dev`.
- 05:37 stopped serves 6530/6532 and hub 8030 (catalog-A guests are refused by the current tree); `s` serve 6530 local-only after W2's
  `activate-s-react-dev` (60/60 fresh). live-agent-loop on the real `s` host (`S_OS_MCP_LIVE_SPAWN=note`): **en 22/22**, de 21/22.
- de `boot` red, isolated with `g10-boot-probe.ts` (en never shows it): on a `de` boot the `s` host turns ready, then its error beacon
  flips on for ~1 s and off. First cause: `reportRefreshFault` reported a refresh that landed on a RETIRED instance (`[DEBUG] … no
  actor for instance 1`) as a window fault → fixed (retired = drop, like `dropForRetiredInstance`; react typecheck 0 errors). Probe 2
  then shows the same race through another error, `plugin-ui.intake-rejected:intake:actor-activation.revoked`. Underlying: a
  locale-dependent re-open of the Home instance during boot (`switchToPluginApp`'s identity depends on `uiLocale`). Frontend-owned
  (renderer/`🏛️ShellHost`, S15/R8), not chased further here; `boot-probe-de-{1,2}.txt` are the evidence. `PluginRuntime` still
  throws user-visible `[DEBUG] …` strings (e.g. `requireActorId`); also frontend-owned.
- 06:10 coordinator: note's wasm-dev was rebuilt outside the mutex at 05:39. Not by a command of mine that I can identify: my probes
  in 05:24–05:48 were client-e2e (builds only os-mcp), the guest-site quartet, live-agent-loop (os-mcp AOT-compiles the staged
  component with wasmtime into `~/.semio/cache/wasmtime`; it never runs cargo on a guest) and a `serve s` that printed `[fresh] 60
  staged components`. The one candidate is a dev serve's source watcher (my note serve on 6530 ran 01:47–05:37 across W2's restage).
  All my serves are stopped now (06:10); none runs until W2's hub handoff.
- 06:19 hub law run under the hub mutex (private target): `gis_map_proposal_fixture_pins_…` (now also asserting the default map fits
  `INPUT_MAX_BYTES`) **ok**; all `inference::` hub lib tests **27/27** (`hub-test-{bound,inference}-1.txt`).
- 06:2x prepared for W2's handoff: `g10-agent-edit-human-sees.ts` (4b + G-P2-2 in the `s` shell, en/de: Home → space → note open live;
  delegated agent opens + commits; roster row `data-presence-kind=agent` with the localized badge; ledger grows live; hub head_seq
  advances) and `g10-after-w2.sh` (hub-agent-participant + quartet on 7800, durability on an own hub from a credentialed copy).
  Waiting on W2 (component-release ×34 → os-hub build → publish `--packages all`).
- 07:0x–07:29 U5 item 4 (coordinator): approval withdrawal. See status row 13. Live gate (en 22/23, de 22/23): the new `(c2)` row
  PASS in both; the one red is `boot` — the same transient boot error beacon as before, now also in `en` (intermittent):
  after `ready=s` the Home instance is retired and re-created during boot, and a refresh/intake still in flight on the old
  instance surfaces as the shell error for ~1 s (`[DEBUG] … no actor for instance 1`, then `plugin-ui.intake-rejected:…
  actor-activation.revoked`, then bare `actor-activation.revoked`; probes `boot-probe-*`). I fixed the one site that is
  unambiguous (`reportRefreshFault` drops a refresh on an instance already MARKED retired). I did not mark
  `actor-activation.revoked` as a retirement: a shard loss yields the same failure (`🔌️plugin-runtime` law), so doing so would
  hide crashes. Root cause (why boot re-creates the Home instance) is frontend-owned (S15/R8).
- Not done live: `timed_out` (needs the 120 s countdown) and `superseded` (needs a second shell connection); both are proven
  by the Rust law against the real coordinator + React + wgpu replays.
- 11:58 resumed: catalog B live on 7800 (W2). Its binary was built 11:14, after my 05:23 hub bound change.
- 11:59 hub-agent-participant on 7800 (catalog B): **17/17** (`hub-agent-participant-7800.txt`).
- 12:0x 4b en runs 1–2 failed in the shell, not in the agent path: user1 now holds 10 spaces on 7800 (peers + 3 of mine), and
  the `s` Home surface faults at ≥ 9 author rows (`retained surface render fault … s-home-main nodes: 129, max_nodes: 128`,
  `space-app-probe-1.txt`) — U5's open framework defect (TableWindowKit windowing). I archived my 3 (archive keeps them listed)
  and run the human as user2 (4 spaces). Hub 7800's user1 Home is broken for every slice until that defect is fixed.
- 12:13 4b en run 3 (user2): the `s` Space app lists no documents and its Create dialog says "Artifact kinds are unavailable.
  Reopen the space" (`space-app-probe-2.txt`). Root cause, measured: the shell opens the space INDEX's scoped directory stream
  with `POST /directory/spaces/{space}/documents/index/socket-grants`, and the hub's `issue_scoped_directory_socket_grant`
  (`🌎️hub/🏗️bootstrap/🦀️.rs`) answers 404 because `index` has no document descriptor (the shell's own comment at
  `🏛️ShellHost/🟦️.tsx:6876` names the same fact for open-plan). So on catalog B no human can list or create documents in the
  `s` Space app. Owner: collaboration/frontend (C10/S15) + hub (H9); not changed here. 4b proceeds on the other product path a
  human uses to open a hub document: a `note` window + the footer sync card's remote attach (`host/space/document`), as C3/C8/C10.
- 12:15 hub 7800 went down (hold 54886 and hub 54888 both gone, no CHILD_EXIT line, `wp-w2/generated/bin/` emptied at 12:16 — looks like an external sweep; not me, I never touched those pids). Continuing on my own catalog-B hub :8030.
- 12:23–12:25: my hub 8030 (hold + hub) was killed externally with no exit line, together with my running probe; at 12:25 EVERY hub port 7800/7900/8000–8070 answers 000. A fleet-wide kill of hub processes (not mine). Restarted mine once on the same root.
- 12:29–12:30 every ticket `generated/` folder was deleted externally (`wp-g10`, `wp-w2`, `wp-c10`, `wp-h9` and the ticket-root `🗑️generated`; W2 logs the coordinator freeing the disk from 96 % to 137 GiB at that time). **All G10 captures named above before 12:30 are gone**; the results in this report are the measured values recorded when each run finished. The hub data roots went with them (W2's 7800 root included), so 7800 cannot come back on its old root. From 12:35 every hub proof runs on my own hub :8030 from an APFS clone (`cp -c`) of `.🧬semio/🌐hub/w2-catalog-b` with the shared-target os-hub sha `1a5cf10…` (the binary W2 validated catalog B with).
- 12:44 coordinator rule 15 (revised): durable data under `.🧬semio/🌐hub/s11-<slice>-*`, `generated/` is expendable (the cleanup also killed processes holding files there). My hub root, binary copy, hold state and every long-lived serve log moved to `.🧬semio/🌐hub/s11-g10-{hub-8030-b,bin,state-8030-b,logs}`; hub 8030 restarted on it (hold 26272). The staged `semio-os-mcp` had also been deleted (12:16); restaged 12:44 (`bun ./📜️script.ts build`, 71 s, rc 0).
- 12:50–12:57 hub quartet run 1 on :8030 (catalog B, `g10-quartet-live.ts` adapted: wfc half = `3d.wfcgrid3d`, whose solve has no commit binding; gis half = submit → withdraw one offer → approve another): **8/19**, two root causes, both mine to fix:
  1. **Hub ledger bound drift.** My 05:2x change raised `INPUT_MAX_BYTES` to 1 MiB, but the ledger's SQL still said `CHECK(length(input)<=65536)`. A map input above 64 KiB then failed the SQL CHECK, and `storage()` reported that as `503 inference.storage` ("temporarily unavailable", `retryable: true`). The SQLite ledger was empty (0 jobs), which confirms it. Fix (`🌎️hub/💡️inference/🪶️sqlite/🦀️.rs`): `ledger_schema()` builds the SQL from the declared bounds (`INPUT/RESULT/PROPOSAL/IDENTITY_JSON_MAX_BYTES`, `PROGRESS_MAX_CURSOR`, `command::COMMAND_MAX_BYTES`), and the 9 Rust `8192` literals now use `COMMAND_MAX_BYTES`. New law `a_ledger_stores_every_input_its_declared_bound_admits` (65 537 B and exactly 1 MiB). `cargo test -p semio-hub --lib -- inference::sqlite` **6/6** (`s11-g10-logs/hub-ledger-bound.txt`); `os-hub` built rc 0 (sha `ed814135…`). A hub root created by an older binary keeps the old table (`CREATE TABLE IF NOT EXISTS`). It needs a fresh `inference/gis-map-jobs.sqlite3` (no migration). Hub 7800 (binary `1a5cf10…`) still has the 64 KiB CHECK.
  2. **The MCP published the wrong artifact kind for hub documents.** The hub `semio://artifact/{id}/schema` resource published `descriptor.artifact_kind` (the manifest id, `3d.wfcgrid3d`) as `artifactKind`, where the dialect coordinate (`s.wfc.grid3d`) belongs. So `inference_submit` answered NOT_FOUND on every hub kind whose manifest id differs from its dialect (all catalog-B kinds except gis and note), and `artifact_open`/`capabilities_search` matched no verbs for those kinds. Separately, `inference_list` matched on the pack schema, so a hub gis map listed no service. Fix: `AuthorizedCatalogSnapshot.dialect_kinds` keeps each document's `lease.parent_dialect.artifact_kind` (the dialect the hub indexed at genesis; `selections` is per package). The schema resource publishes it, and `declared_inferences_for_artifact` uses the same key as `select_inference_service`. The quick law is sharpened: descriptor kind `note.document` vs lease dialect `s.gis.gismap`. `cargo test -p semio-framework-os-mcp --lib -- workspace:: inference:: artifact::` **127/127** (`s11-g10-logs/mcp-dialect-check.txt`). No wasm32 check was run (no cfg-gated code touched).
- 12:50–13:05 durability run 1 died: the gate's own hub (:8031) never answered `/readyz` for the 600 s stall bound. Four catalog hubs were booting at once (W2 7800, S15 8040, mine 8030, the gate's 8031). The gate threw from `bootHub` with no hub output. Fix: `bootHub` now prints the last 4 KiB of hub output (channel key redacted) and stops its hub on a boot failure.
- 12:5x–13:1x **4b** en runs 1–2 on :8030 (note serve :6531 → remote attach). The agent half is green: rows 0, 2, 4, 6 (principal `agent:<delegationId>`, `addBlock` SUCCEEDED, hub `head_seq` 0→1). The shell half is red. `g10-remote-attach-probe.ts` measured it step by step: sign-in 200, then Attach. **No hub request follows**; the shell raises "Document restore failed: document authority cancelled" and "The document target changed. Reopen the document.", then drops the hub session ("Shared access is unavailable…", footer back to `signed out`). The cause is catalog skew: the serve's staged note guest (`c632a2ea…`, `[stale] source-changed`) ≠ catalog B's note, the same as C10 `c10gp14n`, and the cleanup pruned 15 dev-lane cores. The browser half of 4b and the live roster badge wait for W2's restage from the catalog-B tree. The `s` Space-app path is still blocked by the index socket 404 (C10 F5).
- 13:20 hub :8030 restarted on the ledger-fixed `os-hub` (`s11-g10-bin/os-hub-ledger`, sha `ed814135…`) with a fresh inference ledger file (my root, 0 rows). MCP binary restaged with the dialect fix.
- 13:26–13:31 hub quartet run 2 on :8030 (ledger-fixed hub + dialect-fixed MCP): rows 0–13 PASS. The live ledger table now reads `CHECK(length(input)<=1048576)`. `inference_list` names `s.wfc.grid3d.solve` and `s.gis.gismap.inference`. The grid3d solve runs in the guest (accepted → succeeded, 4 progress rows, result `{assignments, satisfiable}`, proposalState `none`). A second solve is cancelled. Approving a job without a proposal is refused (PRECONDITION_FAILED). gis is relayed to the hub (`site=hub`), streams 8 progress fractions to an offered proposal, and one offer is withdrawn (`offered → cancelled`). **Row 14 `inference_approve` killed the hub**: `SIGABRT`, "thread 'tokio-rt-worker' has overflowed its stack" (`~/Library/Logs/DiagnosticReports/os-hub-ledger-2026-09-25-133041.ips`). The faulting stack is `inflate ← pack decode_document ← durable_group verify_inverse ← …MapAssembly advance ← drive_turn ← commit_retained ← commit_prepared_approval ← approve_gis_map_job ← post_inference_gis_map_job_approval ←` about 100 axum/hyper frames on a tokio worker. The durable-store turn runs inline under the whole HTTP middleware stack, and in the unoptimized build it outgrows tokio's 2 MiB default. Fix (`🌎️hub/🏗️bootstrap/🦀️.rs`): `main` builds the runtime itself with `thread_stack_size(HUB_RUNTIME_THREAD_STACK_BYTES = 8 MiB)`, the budget a process main thread gets, and the former body is `serve()`. `os-hub` rebuilt under the hub mutex, rc 0 (sha `98dea583…`). Before this fix, every hub-site approval crashed the whole hub for every user.
- 13:26–13:48 **P0-2 `hub-edit-durability-check` on catalog B: 21/21** (`s11-g10-logs/hub-edit-durability-2.txt`). The gate ran its own hub :8031 from the credentialed clone `s11-g10-hub-src-b`, with the catalog-B binary `1a5cf10…`. Results: agent-1 and agent-2 (fresh MCP processes) and human-1 each commit, and each edit is relayed and advances `head_seq` (0→1→2→3). Row **5p** shows both agents in the human's roster **as agent principals** (`H4 durability agent 1:agent`). Three distinct mutation ids; a late joiner's catch-up carries all 3; `head_seq` is kept across a hub restart (3→3), and a fresh joiner after the restart catches up on all 3. Honest note: runs 1–2 of this gate ran outside the hub fleet mutex (the gate boots a hub; it compiles nothing of the hub). Later runs go through the mutex.
- 13:49 hub :8030 restarted on `s11-g10-bin/os-hub-stack` (sha `98dea583…`): ledger fix + stack budget.
- 15:20 resumed (usage limit). My hub :8030 (stack-fixed binary) and the note serve :6531 were still up. W2's restage (14:36) ended with `verify` failing: 12 plugins still have no dev `.core.wasm` (`w2-logs/restage.txt`, `consistent=-12`), so the `s`/note browser lanes still do not match catalog B.
- 15:23–15:25 **hub quartet run 3 on :8030: 18/19** (`s11-g10-logs/quartet-live-3.txt`, transcript `g10-quartet-live-transcript.json`). **Approve → commit now works end to end**: gis offer → `inference_approve` (state `succeeded/approved`, commit carries `baseBinding`) → replaying the approval is refused → hub ledger `head_seq 0→1`, `commit_seq 0→1` → **agent B, a second MCP client**, sees `headSeq 0→1` in its own authenticated descriptor view. The quartet's row 17 now reads the descriptor resource, because a hub document has no MCP history resource. 7 progress notifications. The one red row was the second grid3d solve, submitted while the first was still mounting the document: it **failed on its own** with `PLUGIN_UNAVAILABLE "a canonical pair receipt is already in flight"`. The MCP's canonical-pair actor refused every concurrent reader of a loading scope unless both carried the same exact expected identity, so two jobs on one hub document raced. Fix (`🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs`): every concurrent reader joins the one loading receipt of its scope. `PairCompletion::Published` now carries the published identity; a reader with an expectation checks it against that identity (`StaleCompletion` if it differs), and a reader without one takes what was published. `InFlight` is deleted. New law `every_reader_of_a_loading_scope_joins_its_one_receipt_and_learns_what_it_published`.
- 15:23–15:25 **`hub-agent-participant-check` on canonical 7800 (catalog B, runId `345ceda4…`): 17/17**, run under the hub mutex (`s11-g10-logs/hub-agent-participant-7800.txt`). The lost 11:59 capture is re-measured.
- **Warning for W2/coordinator:** hub 7800 runs binary `1a5cf10…`, which has both hub defects fixed above. A gis `inference_approve` on 7800 **aborts the whole hub** (stack overflow), and a map input above 64 KiB is refused as `inference.storage`. So I ran no approval against 7800. W2's next hub restart needs an `os-hub` from the current tree **and a fresh `inference/gis-map-jobs.sqlite3`** (the old table keeps its 64 KiB CHECK).

- 15:38 MCP laws after the pair change: `cargo test -p semio-framework-os-mcp --lib -- workspace:: inference:: artifact::` **128/128** (`s11-g10-logs/mcp-pair-check.txt`). MCP binary restaged (61 s).
- 15:4x **hub quartet run 4 on :8030: 19/19** (`s11-g10-logs/quartet-live-4.txt`): grid3d guest solve → result; a second concurrent solve is cancelled cleanly; approval without a proposal is refused; gis relayed to the hub, one offer withdrawn, another approved → commit (`succeeded/approved`), the replay is refused, `head_seq 0→1`, agent B sees `headSeq 0→1`, 9 progress notifications.
- 15:43 stopped my hub :8030 (hold 87757) and note serve :6531 (31928). Deleted `wp-g10/target`, `wp-g10/generated`, the gate's durability data copies, the superseded binary `os-hub-ledger`, and empty or stale ticket dirs (`bin`, `bridge`, `credentials`, `state-8030`). Kept for the 4b rerun: `.🧬semio/🌐hub/s11-g10-{hub-8030-b,hub-src-b,bin/os-hub,bin/os-hub-stack,logs}`.

## Remaining (4b browser half + live roster badge, blocked on a dev lane that matches catalog B)

1. When W2's Hub Handoff reports a lane that matches catalog B, restart my hub. Run `python3 g10-detach.py <state>/hold.txt bun g10-hub-hold.ts 8030 .🧬semio/🌐hub/s11-g10-hub-8030-b .🧬semio/🌐hub/s11-g10-bin/os-hub-stack <state>`, or use 7800 once it runs a current-tree binary. Then start `g10-serve.sh s 6530 <hub>` and run `G10_OPEN=space bun g10-agent-edit-human-sees.ts http://127.0.0.1:6530 <hub> en` and then `de`.
2. W2: restart 7800 on a current-tree `os-hub` with a fresh `inference/gis-map-jobs.sqlite3`. Until then, a hub-site gis approval aborts 7800.

## Files Changed (G10)

| path | change |
|---|---|
| G9 landing set (11 files, `📓️landing.md` row G10) | manifest/plugin `InferenceCommitBinding`, TS projection, wfc ×5 contracts, MCP `declared_commit_action` + 2 test literals |
| `🌉️mcp/🧠️context/🦀️.rs`, `🏠️workspace/🦀️.rs` (`list_resources`) + 2 tests | one owner per resource URI (duplicate `semio://workspace/artifacts` gone at the source) + uniqueness law |
| `🌉️mcp/🏠️workspace/🦀️.rs` (`beat_agent_presence`, `agent_presence_moment/peer`) + quick test | hub-bound agent session beats presence per connection (hub stamps label + principal kind) |
| `🌉️mcp/🦀️.rs` (`attach_stdio_bridge`), `🖥️ui/🦀️.rs` (refusal texts), README | stdio gateway always offers its bridge (zero-touch regardless of start order) |
| `🌉️mcp/🟦️.ts` (`ensureMcpBinary`, `mcpBinaryFreshness`), `📦️packages/🦀️rust/📜️script.ts`, `🎚️config/🧱️binary-gate.json`, `🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts` | staged-binary freshness = Cargo dep-info sources record; staging build dies with its owner |
| `🦑️repo/…/📦️artifacts/🏗️native-build/🟦️.ts` | `sourcesRecord` (dep-info closure, `semio.cargo.binary-sources/v1`), `SEMIO_BUILD_OWNER_PID` watchdog |
| `🌉️mcp/💡️inference/🦀️.rs` | `inference_submit` en/de description; Bounds explanation names the declared bounds |
| `🌉️mcp/🧬️schema/{🔣️.json,🟦️.ts}` | schema mirror regenerated (G9's `HubInferenceApprovalRequestV1`) |
| `🌉️mcp/🧪️tests/🎚️config/🟦️.ts`, `💡️inference-bridge/🟦️.ts` | per-run bridge rendezvous for test gateways; folder-lane inference law follows the generic model |
| `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` | `S_OS_MCP_LIVE_LOCALE`, `(i18n)` and `(c2)` rows |
| `🌉️mcp/🧪️tests/🤝️hub-edit-durability/🟦️.ts` + fixture | own space (zero-touch), agent-presence row `5p` |
| `🌉️mcp/🧪️tests/🤖️hub-agent-participant/🟦️.ts` | own space + note, default origin = `dev s` hub 8787 |
| `🌉️mcp/🧵️bridge/{🦀️.rs,🟦️.ts,🧫️fixtures/📨️frames.json}` + quick test | `ApprovalWithdrawn` (tag 11) |
| `🌉️mcp/🛡️policy/🦀️.rs` + quick test + `🧫️fixtures/🪦️approval-withdrawal.json` (new) | withdraw on cancel/timeout/superseded; law |
| `📺️renderer/…/🔗️AgentBridge/🟦️.tsx` + component test; `🤖️AgentApprovals/🟦️.tsx`; `💬️AgentChatPanel/🟦️.tsx` + test | withdrawn state + en/de labels |
| `📺️renderer/…/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` + wgpu-unit; `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (2 match arms) | wgpu twin + withdrawn chip/state |
| `📺️renderer/…/🏛️ShellHost/🟦️.tsx` | agent credential install port; `reportRefreshFault` drops retired-instance refreshes |
| `📇️directory/🤖️delegations/🟦️.ts` + `🧫️fixtures/🔌️mcp-client-config.json` (new), `📇️directory/🧬️schema/🔣️.json` | MCP client config contract (6 defs) |
| `📺️renderer/…/🔗️HubConnection/{🟦️.tsx,🏛️workspace/🟦️.tsx}`, `🤖️AgentDelegations/{🟦️.tsx,tests,story}` | Set up MCP client / Copy, en + de |
| `🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` | dev-host credential install endpoint (same-origin, 0600/0700, DELETE on withdraw) |
| `🌎️hub/💡️inference/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts}`, `🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json`, runtime unit test | inference input bound 1 MiB + default-map law |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🌉️os-mcp🤝️hub-agent-participant`, `💬️agent-reply`, `🤖️live-agent-loop🌐️de` |
| `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` + unit test | ledger SQL built from the declared bounds (`ledger_schema()`), `COMMAND_MAX_BYTES` everywhere; law `a_ledger_stores_every_input_its_declared_bound_admits` |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | explicit runtime with `HUB_RUNTIME_THREAD_STACK_BYTES` (8 MiB); former `main` body is `serve()` |
| `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`, `🌉️mcp/🏠️workspace/🦀️.rs` + quick test | per-document `dialect_kinds` in the catalog snapshot; the hub schema resource publishes the dialect as `artifactKind` |
| `🌉️mcp/💡️inference/🦀️.rs` | `declared_inferences_for_artifact` keys by artifact kind (same key as submit) |
| `🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs` + unit test | concurrent readers join the loading pair receipt; `InFlight` deleted |
| `🌉️mcp/🧪️tests/🤝️hub-edit-durability/🟦️.ts` | boot failure prints the hub output tail and stops the hub |
| ticket `wp-g10/` | scripts listed in the log (new: `g10-remote-attach-probe.ts`, `g10-hub-ledger-bound.sh`, `g10-hub-build.sh`, `g10-mcp-dialect-check.sh`, `g10-mcp-pair-check.sh`, `g10-participant.sh`); captures in `.🧬semio/🌐hub/s11-g10-logs/` |

## Honest Gaps

- **4b browser half and the live React roster badge are not proven.** The agent side and the wire-level roster (`:agent`, durability 5p) are live. The shell side is blocked by catalog/lane skew and the Space index 404; neither is G10-owned.
- The hub quartet ran on my own hub :8030. That hub has the catalog-B guests and the current-tree `os-hub` with the two hub fixes. Canonical 7800 still runs `1a5cf10…`, so no approval ran there. Row 15's replay refusal cites the refreshed authority generation (`PERMISSION_DENIED`), not "already approved"; it is refused either way.
- Hub runtime stack budget: proven live, with no dedicated law. The stack overflow only shows under the full HTTP stack of an unoptimized build.
- No wasm32 check for today's MCP changes (no cfg-gated code touched). Durability runs 1–2 ran outside the hub mutex.
- The `boot` row of live-agent-loop is intermittently red in both locales: a frontend boot race (the Home instance is retired and
  re-created during boot), owned by S15/R8.
- `timed_out`/`superseded` withdrawals are law-proven, not live.
- G-P1-4: architect/wfc capabilities still carry `description: ""` (not reworked, per coordinator).
- `gis-inference-ledger-oracle` red at `gis-native-provider-selection` (not mine).
- Every capture cited before 12:30 was deleted by the external cleanup. The numbers are the values recorded when each run finished.
