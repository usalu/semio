# WP-G12 — AI Integration For Users Over The Semio MCP (Session 14)

Slice G12 · session 14 · 2026-09-27 18:2x. Successor of G11 (`📓️wp-g11.md`, also `📓️wp-g10.md`, `📓️audit-s13-ai-mcp.md`,
`📓️acceptance-s13.md` §4). Owns outcome 4 (semio MCP `semio-framework-os-mcp`, `mcp__semio__*`, never the repo MCP).
Ports: hubs 8030–8039, serves 6530–6539. Private cargo target `.tmp-ticket/wp-g12/target`; native cargo only through
`fleet-mutex.sh native g12` in build-fleet-b. Durable data + logs `.🧬semio/🌐hub/s14-g12-*` (logs `s14-g12-logs/`).
Scripts `wp-g12/`.

## Session 14

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
