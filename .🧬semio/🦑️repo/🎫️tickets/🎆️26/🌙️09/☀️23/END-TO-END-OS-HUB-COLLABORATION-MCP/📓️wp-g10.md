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
