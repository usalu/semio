# WP-G12 — AI Integration For Users Over The Semio MCP (Session 14)

Slice G12 · session 14 · 2026-09-27 18:2x. Successor of G11 (`📓️wp-g11.md`, also `📓️wp-g10.md`, `📓️audit-s13-ai-mcp.md`,
`📓️acceptance-s13.md` §4). Owns outcome 4 (semio MCP `semio-framework-os-mcp`, `mcp__semio__*`, never the repo MCP).
Ports: hubs 8030–8039, serves 6530–6539. Private cargo target `.tmp-ticket/wp-g12/target`; native cargo only through
`fleet-mutex.sh native g12` in build-fleet-b. Durable data + logs `.🧬semio/🌐hub/s14-g12-*` (logs `s14-g12-logs/`).
Scripts `wp-g12/`.

## Session 14

### Session 14c

Successor agent, 2026-09-28 21:1x (spawned at 7800 READY on `s14-w4-catalog-p24`, WINDOW 3 OPEN). Gateway under test = the chain's
os-mcp build (20:59, 675 sources fresh at 21:12) copied to `.🧬semio/🌐hub/s14-g12-bin/semio-os-mcp-p24` (sha256 444125d1…), so
window-3 trains cannot make the harnesses re-stage it. Serve `s` React dev 6530 joined to 7800 (`wp-g12/g12-serve.ts`).

| # | Item | Status |
|---|---|---|
| 1 | Battery on 7800/p24, chain gateway (`battery-p24-14c-1/summary.txt`) | quartet **19/19 PASS** · security **6/6 PASS** (S2 read audience now green) · durability **23/23 PASS** (own hub 8032, p24 binary + catalog) · user path en **9/9 PASS** (destructive approval, withdrawal) · user path de 6/9 (row 4 shell-instance race → host fix, item 4) · participant 17/19 (link backoff, item 3; 19/19 on re-run 3) · refused-relay probe correct · untrusted-content red (item 2) · hub coverage 0/36 (transport, item 3) |
| 2 | Untrusted-content law: folder-lane csv create timed out at 240 s | **ROOT CAUSE (host)**: the isolated compile worker (`compile-component`) wrote stdio's 330 MB `.cwasm` in ~4.5 min, then sat > 15 min in `CodeMemory::drop` → macOS `__deregister_frame` (linear scan per frame = quadratic). Fix in-tree: `compile_component_isolated` uses `Engine::precompile_component` (never publishes executable code) + law `an_isolated_compile_writes_the_precompiled_entry_its_host_engine_loads`; plugin-host check rc 0, laws 10/10 (`gate-precompile-1.txt`). Plant (csv set-cell) still needs item 5 |
| 3 | Hub-lane transport (coverage "cannot be matched…", participant link backoff) | H13's refill + my live catalog landed by L1 (mini-train 21:59); my live-catalog script left 1 os-mcp E0308 (generic borrow rewrite) → fixed in-tree 22:18 + script fixed; gate `p24b` queued (check + laws + gateway) → rerun coverage + participant |
| 4 | User path de row 4: agent edit before the shell published the note's instance | **Host fix in-tree**: `🐚️channel` waits ≤ `SHELL_INSTANCE_WAIT_MS` (10 s) for the plugin's instance before refusing by name + law `a_command_waits_a_bounded_time_for_the_shell_to_publish_its_plugins_instance`; proof in gate `p24b` |
| 5 | Revision-bound verbs unreachable for agents (coordinator decisions 21:5x/22:0x/22:4x) | **PREPARED + LANDED BY L1**: `wp-g12/g12-revision-binding.py` (36 files: schema-first `ArgFormat::DocumentRevision`/`TargetRevision` optional+hidden; SDK agent-lane `fill_agent_revisions` + `agent_target_revision` hooks; stdio declarations + zip/pdf/xlsx/epw target fills; gateway guard `revision.guard-required`; laws manifest/csv/zip×2/gateway×2). Scratch apply + re-run idempotent; live tree now "applied" (L1, T1). Compile proof = L1's T1 train |
| 6 | Untrusted envelopes | **6/6 PASS** (gateway p24, `s14-g12-logs/untrusted-md-2.txt`): headless plant = md `textEdit` (csv set-cell is revision-bound and a blank csv has no cell), planting gateway auto-approves (landed, test-only) |
| 7 | User path en/de | en **9/9 PASS** (p24b, private rendezvous `S_AGENT_BRIDGE_DIR`, `battery-p24b-4/`); de 6/9 3 runs out of 3: directory bootstrap FAULT ("Verzeichnisaktualisierung angehalten") in de → S18 (coordinator relayed). Cross-wiring root cause: a gateway's bridge offer reaches every live os session (item 8) |
| 8 | Scoped bridge offers (security; coordinator 23:2x: G12 owns) | **LANDED 01:34, green** (os-mcp check rc 0, shell_channel 20/20, rendezvous 8/8, AgentBridge vitest 82/82, os tsc 0, boot Home):  rendezvous v2 — `BridgeOffer.scope` (`Hub{hubOrigin,spaceId}`/`Local`), `principal` = the adopted agent (offer published after the delegation exchange: `start_stdio_bridge` + `publish_stdio_bridge_offer`); dev server `liveAgentBridgeOfferFor(scope)` = `selectAgentBridgeOfferV1` (hub offer only for the shell on its hub+space whose human's own `GET /auth/agent-delegations` lists its agent; local offer any shell; newest); React `offerScope` (ShellHost: hub origin, open space, own delegations); schema def `AgentBridgeOfferRecordV2`; fixture `🛰️offer-answers` `select` (6 records, 9 cases incl. "another human's shell never receives it"); laws TS (Ajv oracle) + Rust record twin. os tsc: 0 in my files (2 foreign, 🏪️store/👷️worker 23:43); boot probe 6530 → Home, 0 faults. wgpu page → WG11 (relayed) |
| 9 | User path row 4 on a fresh hub (8031, fresh users): both locales red — the delegated agent's edits took the attached shell's route, and nothing opens the note in the human's shell | **FIXED (host)**: `SessionChannelBinding::for_principal` — a delegated agent is its own headless hub participant (the shell still carries approvals/conversation/UI); law `a_delegated_agent_never_routes_artifact_verbs_through_the_attached_shell` PASS (gate `t1e`); live proof after the hub refresh |

#### Session 14c Pids

| pid | what | started | stopped |
|---|---|---|---|
| 16985 | `s` React dev serve 6530 → 7800 (`s14-g12-logs/serve-6530.log`) | 21:15 | |
| 17157 | `g12-battery.sh … p24-14c-1` → `s14-g12-logs/battery-p24-14c-1/` | 21:15 | 21:51 done (coverage child killed by me 21:48: every open broken by the transport) |
| 19983 | orphaned `semio-os-mcp-p24 compile-component` (stuck in `__deregister_frame`) | 21:21 | 21:57 killed by me |
| 29000 | `g12-gate-host.sh precompile-1` (native lane) → `gate-precompile-1.txt` | 21:41 | 22:15 done (host green; os-mcp red = live-catalog E0308, copied binary stale → deleted) |
| 54078 | `g12-gate-mcp.sh p24b …` (native lane, build first) → `gate-p24b.txt` | 22:19 | 22:42 reboot (build rc 0 22:32, os-mcp check rc 0 22:36, laws not reached) |
| 16985 → | serve 6530 | 21:15 | 22:42 reboot |
| 62814 | `g12-battery.sh … p24b-1` (participant, coverage, user path de/en; gateway p24b) | 22:33 | 22:42 reboot (participant done; coverage in its creation phase) |

#### Session 14c Log

- 21:1x read preamble 14/14b/14c + rule 24, fleet-14 tail (wave-B relays: set-cell/set-node need `revision`), this report. Nothing
  in flight from 14b (g12-live-catalog.py stays prepared for L1's T4). Battery extended: `wp-g12/g12-refused-relay-probe.ts` (G11/H13
  probe, credentials from env, note kind by dialect), untrusted-content law (real binary), durability on port 8032.
- 21:16–21:51 battery `p24-14c-1` (gateway = chain build 20:59): participant 17/19 (rows 12/12b: "hub did not acknowledge … link
  backoff (retry in 16000 ms)"), quartet 19/19, security 6/6, refused-relay probe: (a) read delegation → PERMISSION_DENIED
  `viewer.read-only` + en/de remedy, hub head stays 0; (b0) edit delegation SUCCEEDED `relay:acknowledged` head 0→1; (b1) after
  revoke (204) → PERMISSION_DENIED "hub session is unauthorized", head stays 1. Untrusted-content: beforeAll timed out (240 s) creating
  the csv kind (item 2). Coverage-hub: 36 kinds offered, 34 created (2d/3d.generation: hub genesis trap "ordered-map root must be
  explicitly retired before drop" — same in W4's open-plan probe → W4/S19), every open PLUGIN_UNAVAILABLE "cannot be matched to one of
  this workspace's 23 registered plugins" → stopped at row 5 (transport, item 3). Durability 23/23. User path en 9/9 (17.2 s to an
  approved agent edit), de 6/9 (item 4).
- 21:25–21:34 participant re-runs with gateway stderr now captured (`S_OS_MCP_PARTICIPANT_OUT`, evidence in the record; row 8 shows the
  link state): run 2 red again (same backoff), run 3 **19/19**. `wp-g12/g12-link-probe.ts` (env creds, participant argv): 3/3 green
  (open 2 s, link live); the failing spaces had NO document-socket upgrade in 7800's capture → client-side failure before the hub =
  the byte bucket (H13). Probe on the coverage space: open 23 s, link "backoff (retry in 8000 ms)".
- 22:19–22:36 gate `p24b` (build first, early lane stamp): gateway build rc 0 22:32 → `semio-os-mcp-p24b`; os-mcp `check --lib --tests`
  rc 0 22:36 (proves the 🐚️channel wait + the 1-line live-catalog fix). CAVEAT (measured from L1's `w3-backup/` stamps vs the SDK
  unit's `invoked.timestamp` 22:26:27): p24b links T1a **plus L1's partial T1** (lb2-p6/p1/p7, t14-f9, f9-carriers, t14-g12 seed, class
  fix written 22:19–22:26) — not a train-green state; rebuild after L1 reports T1 green.
- 22:34 **participant 19/19 PASS** on p24b (`battery-p24b-1/participant.txt`; link live at open, relay acknowledged, 12b/12c green) =
  the transport fix measured live. Coverage-hub was in its creation phase when the machine rebooted (~22:42).
- 22:4x machine reboot → every process died (serve 6530, battery p24b-1, gate p24b laws). Torn-write check: my tree edits (🐚️channel,
  plugin host, 1-line workspace fix, participant/untrusted/user-path TS) all compiled/ran after their last write; none torn.
- 22:5x revision set finished (gateway guard + laws + stdio target fills) and relayed to L1; L1 applied it in T1 (dry run now "applied");
  P9's note: the agent-lane anchor now ends at `command_from_action(…).await?;` (robust to P9's rewrite of the preview call).
- 23:0x user path p24b: en 7/9 then 9/9 once the serve + gate share a private `S_AGENT_BRIDGE_DIR` (the default rendezvous offered my gateway
  to 5 live os sessions of other slices; a foreign shell attached → "no open instance of `note`"); de 6/9 ×3 = directory FAULT in de (S18).
  Harness: user path writes the MCP client's gateway stderr (`gateway-stderr-<locale>.txt`); tsc rc 0.
- 23:4x–00:1x scoped offers written (item 8) + delegated-agent headless routing (item 9); os tsc 0 (00:0x, rc 0, 108 s) + boot probe
  6530 → Home 0 faults; revoked/expired delegation scope law (`scopes` rows, `agentBridgeOfferScopeFromDelegationsV1`) added on the
  coordinator's ask; hub-side revocation laws confirmed for WG11 (hub `an_agent_delegation_mints_a_session_that_works_until_it_is_revoked`,
  `revoking_a_delegation_closes_the_agents_open_document_socket_and_roster_row`). Local folder coverage (p24) started 00:12: animate +
  architect created + mutated, then the 00:31 panic killed it (not resumed: rule 25, hubs down).
- 00:1x usage cut + kernel panics (22:42, 00:31, 00:41, 00:48) + 01:14 external sweep (rule 26: private target now
  `.🧬semio/🌐hub/s14-g12-target`). 01:3x resume: every scoped-offer/routing edit intact (fixture 6 records / 9 cases / 3 scopes,
  TS law, Rust record twin, `for_principal` + law); none of my queued proofs had run (gates t1c, vitest). Gate `t1d` (check → laws, no
  build) launched 01:3x.
- 01:28–01:34 gate `t1d`/`t1e` (native lane, target `.🧬semio/🌐hub/s14-g12-target`): check rc 0; `shell_channel` 20/20,
  `rendezvous` 8/8; revision-guard law red at its own setup (raw scope name) → fixed, 2/2; guard now evaluated only for an authorized
  caller. `descriptor_authority_generation` (my live-catalog law) RED on the tree = `🌍️gis/🔣️.json did not decode: missing field
  label` — T1 renamed artifactKinds `name`→`label`, descriptor regen pending (L1/T14); re-run after regen. AgentBridge vitest 82/82
  (`SEMIO_INCLUDE_AGENT_BRIDGE=1`).
- 21:5x relays: L1/H13 transport refill = gate of the hub lane (coordinator: early mini-train, landed 21:59 with my live catalog);
  LB2 revision-bound verbs → coordinator decision: G12 writes it (item 5).

### Session 14b

Successor agent, 2026-09-28 12:0x (predecessor cut ~20:45 by the usage limit; app restart killed every process). Chain launched
12:02:46 → GUEST FREEZE ON. Hub 7800 on B3 (hold 45800 / hub 45803).

| # | Item | Status |
|---|---|---|
| 1 | Reconcile predecessor's in-flight edits + compile the host note-id fix; law for two fresh agent sessions | **DONE**: nothing half-applied (all in `5bcb2da`); os-mcp compile-green 12:45 (`gate-ingress-3.txt`) after fixing the Codex media-export E0004s; law = `🤝️hub-edit-durability` row 4r, live 23/23 (agent-2 cursor 1 = head 1); participant 12b/12c live green on 7800 |
| 1b | H13 relay: read agent's edit answered SUCCEEDED on `plugin:note` | **FIXED (host), laws green**: edits refused before the guest runs (`hub_edit_refusal`), no success without the hub's ack (`hub.relay-unacknowledged`); live proof needs a hub with the role cap + channel-19 guests → after ALL |
| 2 | Guest half (`g12-authoring-seed.py`) dry-run clean on the live tree for T14 | **DONE**: 130 files (129 literal files seen), only problem = F9 not landed; T14's overlay bug (absolute `/.🧬semio` skip) fixed + loud counts; T14 overlay = 129/129 applied |
| 3 | MCP scope gate green | **DONE**: security S1 PASS live (7800, ingress-1); artifact scope laws in quick (green) |
| 4 | Live AI user path on 7800 (B3 now) | participant 19/19, quartet 19/19, security 5/6 (S2 = B3 hub binary lacks the read cap). User path en/de, hub coverage, delegation audiences → after ALL (the tree is channel 19 since the Codex bump; B3 guests are 18) |
| 5 | After ALL: every package over MCP | waiting (W4 chain); gateway for ALL = `s14-g12-bin/semio-os-mcp-ingress-3` |
| 6 | Harness productization (rule 17) | durability gate zero-touch + acceptance record (R10: add `mcp-hub-edit-durability`, requires []); `mcp-plugin-coverage-hub` in R10's plan |
| 7 | Channel pin 19 drift (coordinator decision) | **DONE**: `channel-version generate` 11 consumers; 7 derived left → H13/H14 |
| 8 | H13 root cause (b): hub workspace catalog compiled once at `open_hub` | **PREPARED for window 3** (native lane 11 deep at 13:3x → no green proof possible before publish rc): `wp-g12/g12-live-catalog.py` dry run clean on the live tree, scratch apply + re-run = "nothing to do". Apply in window 3 + `g12-gate.sh live-catalog-1` |

#### Session 14b Pids

| pid | what | started | stopped |
|---|---|---|---|
| 47152 | `g12-gate.sh ingress-2` (native lane) → `s14-g12-logs/gate-ingress-2.txt` | 12:09 | 12:13 killed by me after CHECK red (it held the lane into quick) |
| 52304 | `g12-gate.sh ingress-3` (native lane, queued behind wg11/h14/h13) → `gate-ingress-3.txt` | 12:14 | 12:45 done |
| 53538 | `g12-battery.sh … b3-14b-1` (7800, gateway ingress-1) → `battery-b3-14b-1/` | 12:16 | 12:19 done |
| 58663 | `hub-edit-durability-check` run 1 (own hub 8030) | 12:21 | 12:24 exited (hub stopped by the gate) |
| 10106 | `s` React dev serve 6530 (local-only) | 12:56 | 12:58 stopped by me (process group) |
| 15211, 39981 | `plugin-coverage-check --hub 7800` runs b3-2, b3-3 | 12:59, 13:05 | 13:03, 13:18 exited |
| 75458 | private hub 8031 (`wp-g12/g12-hub.sh`, H11/H13 P0 binary `os-hub-h11-1619`, B3 generation clone) | 13:2x | 13:2x stopped by me, root deleted |

#### Session 14b Log

- 12:0x read preamble 14 (+14b), AGENTS.md, fleet-14 tail, this report. Reconcile: every predecessor edit is in the 09-27 21:54
  auto-commit `5bcb2da23da` (26 os-mcp files); NO os-mcp file changed overnight (`git diff --stat 5bcb2da23da 3b2f1181d27` empty).
  Its last gate `gate-ingress-1.txt` finished AFTER the cut (lane 21:04→21:09): check rc 0, quick 479/479 (35 skipped), gateway
  build rc 0 → binary `s14-g12-bin/semio-os-mcp-ingress-1`. The "last step" (rule-17 flag mapping in the os-mcp-rs `📜️script.ts`:
  `HARNESS_FLAGS` + `harnessEnvironment` on 7 live verbs, `runOwnedCommand(…, { env })`) is complete, no half-applied hunk.
  Re-gate `ingress-2` launched 12:09 because the overnight peer edits (~1 870 files) touch os-mcp's dependency closure.
- 12:12 **gate ingress-2 RED** (`gate-ingress-2.txt`): the Codex peer's 04:30 store edit (`📡️spr/🧵️channel`, CHANNEL_VERSION 18→19)
  added `AppCommand::{SubmitMediaExport,PollMediaExport,CancelMediaExport,TakeMediaExportChunk}` +
  `AppFrame::{MediaExportSubmitted,MediaExportStatus,MediaExportChunk}` → 3× E0004 in `🏠️workspace/🦀️.rs`
  (`app_command_seq_mut`, `app_frame_reply_seq`, `app_frame_tag`). Fixed (arms added, exhaustive kept). Same gap in
  `semio-framework-os-run` (`🏃️run/🦀️.rs` ~386 + ~2189, frozen, not compiled by me) → RELAY W4 12:15.
  Consequence: a gateway built from the live tree speaks channel 19; B3's guests speak 18 → live runs on 7800 B3 use
  `semio-os-mcp-ingress-1` (built 21:09 from channel 18 + the host fixes); the ingress-3 build is for ALL.
- 12:14 item 1 law (language-agnostic, self-booting hub): `🤝️hub-edit-durability` gains row `4r` per agent lane — its fresh
  session's first `expectedRevision.cursor` must equal the hub's `head_seq` before that lane (fixture expectation
  `freshSessionStartsAtHead`); with lanes agent-1, agent-2 this is the two-fresh-sessions law. Plus: the os-mcp-rs
  `📜️script.ts` `▶️`/`🪞️` docstrings the harness block had split from `DevScript`/`SchemaMirrorScript` reattached.
  `tsc -p wp-g12/tsconfig-g12.json` rc 0 (17 s). Not run live yet (needs a hub binary + catalog root; next).
- 12:16–12:19 **battery b3-14b-1 on 7800 (gateway ingress-1 = host note-id fix + terminal-link refusal)**:
  participant **19/19 PASS** (incl. 12b fresh session starts at the hub head, 12c its edit lands head +1 — red 18/19 on the
  16:29 staged gateway yesterday) · quartet **19/19 PASS** · security **5/6**: S1, S3, **S4a PASS** (revoke → next edit
  PERMISSION_DENIED at +528 ms, reverted; red yesterday), S4b PASS; S2 red = the B3 hub binary acknowledges a read
  delegation's edit (head 1→2) — hub-side read-audience ceiling (H13's P0 binary; 7800 is still B3).
- 12:2x H13 relay (P0 hub 8010, capture `wp-h13/generated/p0-refused-relay-2.txt`): a read agent's addBlock answered
  SUCCEEDED on the gateway-local `plugin:note` session (no binding → genesis document), hub head 0. Fix (os-mcp only):
  `RoutingArtifactChannel` now carries the hub authority (`hub: Option<Arc<HubRemoteBinding>>`) and refuses every edit
  (`PureCommand`/`TransactionPrepare`/`TransactionRedo`) BEFORE its guest runs via `hub_edit_refusal`: terminal link →
  its code; hub role Spectator → `viewer.read-only` (PERMISSION_DENIED + en/de remedy); authority not ready →
  `plugin.unavailable` (retryable); no single bound hub document of the plugin, or one without a document actor →
  `hub.edit-unbound` (PRECONDITION_FAILED). No write reports success without the hub's acknowledgement: invoke AND saga
  commit revert + answer `hub.relay-unacknowledged` (retryable PLUGIN_UNAVAILABLE) when the relay is not acknowledged
  (the `relay-pending` success-with-warning is gone), refused relays keep their code; a hub-bound commit whose guest
  relayed nothing is unacknowledged (`commits_with_nothing_relayed`). Laws:
  `a_hub_session_edit_is_refused_before_its_guest_runs_unless_an_author_holds_a_live_document` (workspace),
  `a_commit_the_hub_did_not_acknowledge_is_reverted_and_never_reported_as_a_write` +
  `a_hub_edit_that_cannot_reach_the_hub_is_refused_by_name` (dispatch; mock `force_relay`). All in gate ingress-3.
- 12:24–12:52 `hub-edit-durability` live (own hub, B3 binary + root copy, gateway ingress-1): run 1 on 8030 with password sign-in
  22/23 (row 4r GREEN for both agents: agent-2 first cursor 1 = head 1; row 10 red: post-restart catch-up 0/3 — did not recur);
  then zero-touch (free port, pipe-issued human, no creds env) runs 2 + 3: **23/23**, acceptance `mcp-hub-edit-durability` PASS en/de
  (`durability-2.txt`, `durability-3.txt`). Run copies deleted. R10 relay (coordinator): made self-provisioning.
- 12:45 **gate ingress-3**: check rc 0, build rc 0 (`semio-os-mcp-ingress-3`), quick 475/477 — red
  `authenticated_hub_discovery_uses_retained_selection_and_never_installed_fallback` + `remote::…hydrates_exact_selected_descriptor…`
  = the hub lease corpus fixture still at channel 18 vs CHANNEL_VERSION 19 (derived fixture, owner H13). My 3 new laws + the
  terminal-link law run from the built binary: 4/4 ok. SendMessage main "os-mcp green at 12:45".
- 12:4x `installed_artifact_kinds` for a hub workspace now reads the hub-selected descriptors (was the repo registry → export
  answered "INTERNAL repo root not found" for every hub kind in the predecessor's hub coverage run). Compiled in ingress-3.
- 12:50 channel pin: the Codex peer bumped `🔖️channel-version.json` to 19 without the generator → `channel-version check` 20
  findings. os-mcp's 2 fixed by hand (hub-live-catalog-check rc 0), then (coordinator decision) `channel-version generate`:
  11 written, 7 derived left (H13 ×4, H14 ×3). tsc of the changed modules + importers clean (6 unrelated pre-existing errors in
  hub `📜️script.ts`). Serve `s` React dev on 6530 (pid 10106, `wp-g12/g12-serve.ts` = S18's `ensureDevServe`) → Home, console
  error-free, stopped 12:58.
- 12:3x T14 relay: `g12-authoring-seed.py` skipped every dir under an absolute `/.🧬semio` → fixed (component pruning, loud
  counts: 0 literals or changed+applied ≠ seen fails).
- 12:59–13:18 hub-lane coverage on 7800/B3 (gateway ingress-1, `coverage-hub-b3-2`, `-3`): B3 offers 19 creatable kinds.
  Run 2: 10 of 19 creations "refused" (all 19 POSTed at once, no detail kept) → harness now creates in windows of
  `CREATE_WINDOW` = 4 and keeps the hub's status + answer for a refused creation; run 3: **19/19 created**. Mutate+undo+redo
  green for 2d.block (addHandleKind) and 2d.drawing (addLayer); puzzle2d's verbs are shell-only job previews (P9);
  at the 4th doc (wfc2d) the gateway's hub binding answers "hub directory is temporarily unavailable (transport)" and never
  recovers (later opens: "cannot be matched to one of this workspace's 4 registered plugins") — deterministic in both runs →
  RELAY H13 (🔗️remote; the transport error's detail is dropped). Harness now also writes the gateway's stderr
  (`coverage-hub-gateway-stderr.txt`; it held no diagnostics). Export = "repo root not found" in ingress-1 (fixed in ingress-3).
  Acceptance `mcp-plugin-coverage-hub` FAIL 0/19 (measured). The predecessor's stdio csv `set-cell SUCCEEDED head 0→0`
  (16:29 gateway) is exactly the no-ack success the 1b fix now refuses.
- 13:2x delegation-audience proof attempt: private P0 hub 8031 (B3 generation, users from my env files via stdin) + G11's
  refused-relay probe with gateway ingress-3 → the channel-19 gateway refuses the channel-18 hub's execution-target leases at
  start ("execution-target response failed validation") — expected after the pin alignment; proof waits for 7800 on ALL.
  Hub stopped, root deleted. **Ready for ALL** (fire on the coordinator's announcement, gateway = the chain's os-mcp build or
  `semio-os-mcp-ingress-3`): `zsh wp-g12/g12-battery.sh http://127.0.0.1:7800 <7800 state dir> all-1 <serve>` with
  `G12_HUB_BINARY`/`G12_HUB_ROOT` = 7800's ALL binary/root (participant, quartet, security, hub coverage, durability, user
  path en + de); serve = `bun wp-g12/g12-serve.ts 6530 http://127.0.0.1:7800`; then H13's refused-relay probe for the spectator.
- 13:3x coordinator resume (one item): H13's root cause for the hub-lane coverage failure — (a) the gateway's HttpPool byte
  bucket never refills (kernel `📇️directory/🔌️client`, H13 prepares it for window 3); (b) mine: `open_hub` compiled the capability
  catalog ONCE from the startup snapshot and `RoutingArtifactChannel` held that same `Arc<Catalog>`. Prepared (not landed —
  the native lane had 11 waiters, the overlay lane 13, so no green proof before W4's publish rc): `wp-g12/g12-live-catalog.py
  [--dry-run|--write] [--root]` — `WorkspaceCatalog` (`fixed` for a folder; `following(hub)` recompiles when the selected
  package set `plugin_id@descriptor_byte_sha256` changes, reuses it otherwise; `authoritative()` fails closed while the
  authority refreshes = discovery, `current()` keeps the last compiled one = routing); `HeadlessWorkspace.catalog` +
  `RoutingArtifactChannel.catalog` hold it, every read goes through `current()`, hub `discovery_catalog` = `authoritative()`
  (drops the per-tools/list recompile), the authenticated hub fixture follows its binding like `open_hub`; law
  `a_hub_workspace_catalog_follows_a_new_descriptor_authority_generation` (gis → gis + note after a new selection; unchanged set
  = same Arc; invalidated stream → discovery Err, routing keeps the last catalog). Live dry run: 2 files, 0 problems; scratch
  copy apply → re-run "nothing to do". Window 3: `python3 wp-g12/g12-live-catalog.py --write` → `zsh wp-g12/g12-gate.sh
  live-catalog-1` → coverage-hub on ALL with the fresh gateway (needs H13's refill fix too for > ~80 MiB of components).
- Finding (not fixed): plugin argument faults with plugin-specific codes (`snapshot-edit.argument-missing`, …) map to INTERNAL
  in `map_fault` — needs a declared fault category from the guest, not a gateway code list.

### Session 14 (predecessor)

| # | Item | Status |
|---|---|---|
| 0 | Reconcile G11's in-flight tree edits (scope fix, fault mapping, dial cause, security harness) | G11's staged os-mcp set is complete; its only red (quick law `workspace_backed_tools_degrade…`, fixture principal had no scopes after the scope fix) fixed in the test (`fixture_server_granting`); gate queued (`gate-ingress-1.txt`) |
| 1 | os-mcp note instance mints identical block ids per fresh agent session | **ROOT CAUSE MEASURED (host)**: the gateway seeds its guest from the hub's checkpoint pair and dropped every `DocumentArchiveReplaced`/`DocumentBackbone` its document actor delivered (tail + live edits) → every fresh session acts on the checkpoint (probe: session B `expectedRevision.cursor 0` with hub head 1). Host fix written (tail/live ingress into the guest, open waits for the live link, snapshot reads the live guest), laws written; gate queued. Guest hardening (note ids author-unique for concurrent writers) → window-3 patch (next) |
| 2 | G11's MCP scope fix compiled + gate green | **LIVE GREEN**: security-check S1 on 7800 with the staged 16:29 gateway (carries the fix) — 11 tools × 2 grants PASS (`s14-g12-logs/security-1.txt`, 19:0x). Compile + quick law re-run in gate `gate-ingress-1` (queued) |
| 3 | Live AI user path on 7800 (quartet, participant, coverage gap per kind, delegation audience, destructive approval, untrusted envelopes, en + de) | pending |
| 4 | After 7800 on ALL: every package's creatable kinds over MCP (create/open/mutate/undo/redo/export) | waiting (W4 chain) |
| 5 | G11's `g11-agent-probe-child-groups.py` + `g11-channel-version-targets.py` | child-groups: NOT TS — it edits `🔌️plugin/🦀️.rs` (SDK, `artifact-app-testing` harness) = frozen path → window 3, dry run clean 18:5x. channel-version targets: spec sent to R10 (RELAY 18:5x, rule 17: R10 adds target + generated rows) |
| 6 | Outcome-4 acceptance as one permanent harness (rule 17: extend the existing verbs; R10 adds targets/rows) | **IN PROGRESS**: outcome 4 = goal-gate step `ai-over-mcp` (+ participant in `collaboration-wgpu-and-agents`) of R10's `acceptance goal`. Done (TS, tsc clean `wp-g12/tsconfig-g12.json`): env-only credentials (removed the hard-coded local passwords from 5 harnesses; `requiredHarnessEnv`/`hubCredentialFromEnv`/`isAcceptancePreconditionMissing` in `🌉️mcp/🟦️.ts`; missing → `blocked` record en + de, measured), rule-17 flags `--hub/--serve/--locale/--hub-admin-capability` on the 6 live verbs (`harnessEnvironment`, refuses unknown flags without echoing values), participant publishes its own record + rows 12b/12c. Next: hub-lane coverage mode (item 4) + spec to R10 |

### Session 14 Pids

| pid | what | started | stopped |
|---|---|---|---|
| 61560 | `g12-gate.sh ingress-1` (native lane: os-mcp check → quick → gateway copy) | 18:50 | |
| 46950 | plugin-coverage-check (folder lane, staged gateway) `coverage-1.txt` | 19:22 | 19:43 killed by me: architect create took 589 s at load ~100 (component compile contention), 2/35 plugins in 21 min |

### Session 14 Log

- 18:2x started; read AGENTS.md, preambles 14/13/12, fleet-14 roster + log (no CHAIN LAUNCHED yet → no freeze yet), `📓️wp-g11.md`,
  `📓️audit-s13-ai-mcp.md`, `📓️audit-semio-mcp.md`, `📓️acceptance-s13.md` §4, fleet-13 log 14:00→16:0x. Nothing of G11's runs.
  G11's last captures: scope gate `s13-g11-logs/gate-scopes-2.txt` (16:30: check rc 0, quick 302/303 — 1 red
  `workspace_backed_tools_degrade_to_a_retryable_plugin_unavailable_without_a_binding`, restage rc 0 → staged gateway 16:29 carries the
  scope fix); battery `battery-b3-3` (16:00, gateway WITHOUT the scope fix): S4 en/de 8/9 (row 8 red: already-connected client still
  edits after withdraw), quartet 19/19, participant 17/17, security 3/6 (S1 scopes, S2 read-audience edit, S4a revoke-next-request),
  coverage 71/71 created / 41/71 mutated.
- 18:3x reproduced WG9's collision on 7800 with `wp-g12/g12-two-session-ids-probe.ts` (from G11's): `ids-probe-1..3.txt` +
  dumps `wp-g12/generated/ids-3/`. Session A: open → prepare `expectedRevision.cursor 0` → invoke SUCCEEDED (hub head 0→1) →
  second prepare cursor 1 (own commit seen). Session B (fresh gateway, same note): `revisionBefore.cursor 0`, `headEditId ""`
  while the hub head is 1 → B never saw A's op. `artifact_snapshot` after A's own edit answered the checkpoint bytes (299/242,
  identical to genesis) with provenance `commitSeq 1`. Root: `🏠️workspace/🦀️.rs` seeds the guest once from
  `read_hub_canonical_pair` (last checkpoint) and `watch_hub_document` ignored `ArtifactEvent::DocumentBackbone` /
  `DocumentArchiveReplaced` (the actor's bootstrap tail + every other writer's edits) → nothing past the checkpoint ever reached
  the guest; note's ids are `text-<fnv(doc.id+command)>-<block count>`, so every fresh session re-minted `…-0`.
- 18:4x host fix (os-mcp only, freeze-open; H13 edits `🔗️remote/` in parallel, regions split via RELAY): `HubRelay` queues the
  actor's `HubInbound::{Archive,Backbone}` in order; `plan_hub_inbound` (newest baseline supersedes; reseeds only when it
  differs from the seeded pair); `RoutingArtifactChannel::{exchange,activate}` take the queue before every command except
  `TransactionCommit`/`Rollback`/`Infer` (`carries_hub_inbound`), reseed if needed, then
  `PluginArtifactChannel::receive_document_backbone` (the shells' `Event::Message{Backbone}` turn, driven to idle; guest
  `AppFrame::Error` → typed fault; refused batch → the rest go back to the queue); egress to the actor now forwards only
  `Mutations` (guest `Ack`s dropped as the wgpu shell does; genesis/member refused by name); `artifact_open` waits ≤ 10 s for
  the live link (catch-up queued); a bound hub document's `semio://artifact/{id}` reads the live guest, not the checkpoint.
  Laws: `a_hub_documents_tail_and_baselines_reach_the_guest_in_order_and_a_new_baseline_reseeds_it`,
  `a_relay_queues_deliveries_in_order_and_withholds_them_while_a_transaction_finishes`. Gate `wp-g12/g12-gate.sh ingress-1`
  (check → quick → gateway binary copy `s14-g12-bin/semio-os-mcp-ingress-1`) queued in the native lane 18:5x (6th).
- 18:5x coordinator: preamble rule 17 (R10 harness contract) and rule 16 (RELAY) noted; P9 now owns the puzzle/writer agent-lane
  preview + jack precondition coverage reds; H13 fixing `🔗️remote validate_snapshot` (agent capped role) — no overlap.
- 19:0x **item 2 live**: `security-check` on 7800 (staged gateway 16:29 = G11's scope fix): S1 PASS (11 tools × 2 grants), S3 PASS,
  S4b PASS; S2 red = hub-side read-audience ceiling (H13, 7800 still B3 binary); **S4a red = MCP-side**: after revoke (204 in 17 ms)
  the connected agent's next `action_invoke` answered SUCCEEDED at +10288 ms (committed in its own guest, relay never
  acknowledged; same as S4 user-path row 8). Root: the gateway treated a terminal document link (`access-revoked`) as "not yet
  acknowledged". Fix (os-mcp, in gate ingress-1): `HubRelay` records a terminal link code (`document_link_terminal_code`:
  access-revoked / link-expired) from the actor's `Conflict`; `await_acknowledged` answers at once with `refused`; the invoke
  path undoes the local transaction and answers `PERMISSION_DENIED` (`map_fault`: access-revoked → PERMISSION_DENIED,
  link-expired → PLUGIN_UNAVAILABLE); a new edit (`PureCommand`/`TransactionPrepare`/`TransactionRedo`) on a terminal link is
  refused before it reaches the guest. Laws `a_terminal_link_answers_a_waiting_commit_at_once_with_the_refusal`,
  `a_terminal_hub_link_refuses_the_agent_by_name`.
- 19:0x harness: `hub-agent-participant` gains rows 12b (a fresh agent session starts from the hub head: its first prepare's
  `expectedRevision.cursor` ≥ the first session's commit) + 12c (its edit lands as a new hub edit, head +1) — the permanent form
  of the ids probe. tsc clean for my files (`wp-g12/tsconfig-g12.json`; the one tsc error is orchestration 396, not mine).
- 19:1x item 1 guest part (window 3, agreed with T14 via main: ONE primitive = F9 `store::content_id`): `wp-g12/g12-authoring-seed.py
  [--dry-run|--write] [--root <tree>]` — SDK `AppOperationContext.authoring_seed` + `ArtifactOwnedToolJobRequest.authoring_seed`
  + `VcsArtifactApp::authoring_seed(actor)` = content_id("authoring-seed", actor ␟ store HLC ticked) on both admission lanes;
  125 `AppOperationContext {}` literals; note retained scope += seed. Live dry run: 126 files, only problem = F9 not landed yet;
  scratch-tree apply + re-run = "nothing to do" (idempotent), diff reviewed. T14 applies it after F9 in its overlay + 105-crate proof.
- 19:2x coverage harness (`🧪️tests/🧩️plugin-coverage/🟦️.ts`): destructive verbs are tried after non-destructive ones (throwaway
  folder, auto-approve; row flag `invokeDestructive`), and a kind whose package declares NO mutation for it (s.home,
  s.playbook.procedural — verified via `mcp__semio__capabilities_search`: playbook's 7 verbs all target s.playbook.playbook;
  demonstrator's own kind has only destructive replace verbs) is `no-mutation-declared`, counted apart in en + de, never rounded
  into the mutated count. Run 1 stopped (above); re-run when load allows.
- 20:0x MCP TS suite on the staged gateway (`s14-g12-logs/mcp-ts-1.txt`, killed at its 300 s budget under load ~110): e2e tier 1/2 red =
  G11's scope fix (the tier tests spawned a scope-less gateway) → tests now grant the tools' scopes; rerun `mcp-ts-2.txt`: e2e 12/12 green;
  hygiene 2 red = `spawnRawMcp` 10 s first-line wait at load ~110 (timing, not code); untrusted-content red = its headless canary plant
  (`writer.setText`) is a P9 preview-unsupported verb → plant switched to `stdio csv set-cell {row 1, column Canary, value}` (verb
  declared, agent-lane green in coverage); rerun `mcp-ts-3.txt` timed out creating the csv kind at load ~110 (240 s) → re-run later.
- 20:0x P9 (direct message): its window-3 SDK set replaces `interactive-job.preview-unsupported` by `interactive-job.preview-budget` and
  jack's `patchNodes` answers `app.command.targets-required` → mapped now (PLUGIN_UNAVAILABLE + remedy; INPUT_INVALID), law extended;
  the old code's arm is deleted when P9 lands (current guests still emit it).
- 20:1x item 6: env-only credentials + rule-17 flags (row above). Proofs (7800, staged gateway 16:29):
  `hub-agent-participant-check --hub` with env → **18/19, red 12b exactly as designed** ("second session's first revision cursor 0,
  first session committed at cursor 1" — the collision defect, red without my host fix; `participant-staged-1.txt`); without env →
  `mcp-hub-agent-participant BLOCKED — precondition missing: OS_MCP_HUB_EMAIL is not set` (en + de), same for `mcp-security`;
  `security-check --password x` refused without echoing the value. Private creds: `.🧬semio/🌐hub/s14-g12-credentials/user{1,2}.env`
  (0600, gitignored). Battery `wp-g12/g12-battery.sh <hub> <state> <tag> [serve]` sources them.
