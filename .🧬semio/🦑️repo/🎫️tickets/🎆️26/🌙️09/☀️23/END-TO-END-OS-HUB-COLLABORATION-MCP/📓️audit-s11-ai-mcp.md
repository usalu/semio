# Session 11 — Outcome 4 re-audit: AI integration over the semio MCP

Read-only, no builds/servers/edits (per session-11 rules). Repo `/Users/ueli/Documents/semio`. Scope: `semio-framework-os-mcp`,
`.mcp.json` server `semio`, tools `artifact_*`/`action_*`/`history_*`/`transaction_*`/`inference_*`/`ui_*`/`capabilities_*`,
`context_resolve`, `conversation_reply`, `job_*`. Never the `repo` MCP.

**Method.** Read in full: `📓️audit-semio-mcp.md` (09-23), `📓️peer-audit-g10.md` (09-24 ~12:45, G11's re-audit),
`📓️wp-g4.md`…`📓️wp-g10.md`, `📓️wp-gj2.md`, `wp-gj3.md`, `wp-m10b.md` (session 9/10/11 chain), and from
`.tmp-ticket-0918/`: `📓️g19-ai-user-experience-audit.md`, `📓️ac1-agent-reply-channel.md`,
`📓️ap1-shell-approval-and-live-snapshot.md`, `📓️a1-mcp-end-to-end.md`, `📓️a2-mcp-plugin-host-instance-open.md`,
`📓️m1-mcp-servers-start.md`, and headline sections of `ce1`/`ce2`/`ce3`/`gj1`/`wi1`. Cross-checked a sample of
highest-leverage claims directly against the current tree (`.mcp.json`, `GATEWAY_TOOL_NAMES`, `resources/subscribe`,
`.vscode/launch.json`, `open_folder`, `📜️script.ts`) with grep/git log, dated 2026-09-25.

**Headline: this ticket's own trajectory (09-18 → 09-25) is the single best evidence source for outcome 4.** It runs
phase-0 "MCP doesn't even connect" (09-18/19) → both `.mcp.json` P0 blockers fixed + `client-e2e` reaching 13/16 (A1,
09-19) → guest-open trap root-caused and fixed (A2) → agent free-text reply channel shipped and live-proven 9/9 (AC1,
09-22) → shell approval affordance shipped and live-proven on the **shell** lane specifically, not just elicitation
(AP1, 09-22) → wfc inference made to actually finish over MCP (WI1/GJ1/GJ2/GJ3, 09-22/23) → session 10's G4–G8 close
cancellation-everywhere, a generic inference-job model (G9), and `client-e2e` **38/38** repeatedly (G4–G8, five
independent re-runs). Session 11's own G10 slice (this session, port 8030–8039) had done nothing yet at read time
(`📓️wp-g10.md`, written 00:34, is a status-table skeleton, every item PENDING).

---

## 1. User journeys

### (1) Zero-touch MCP client connect to a running `s` session

**Status: WORKS-LIVE, with one still-open item.**

- **Zero-touch binary staging is real and current.** `📜️script.ts:60,388,625` imports and calls `ensureMcpBinary`
  before every `dev mcp stdio|http os` launch and in `setup` — **verified in the current tree this session**
  (`grep ensureMcpBinary 📜️script.ts` → hits at lines 60/388/625). This is GJ2's fix (09-23): content-hash gate,
  stages via `bun ./📜️script.ts build`, stderr progress, `.content-hash` stamp. Before it, the audit-semio-mcp.md
  baseline (09-23 12:36) measured the original defect directly: cold `initialize` stalled **>38 minutes** with no
  staged binary at all. That defect class is closed.
- **`.mcp.json` no longer binds a user's agent to the repo root.** `git log -p -1 -- .mcp.json` (commit
  `82c0bdf59a`, 2026-09-24 12:11) shows `--folder` changed from `"."` to `".🧬semio/🔗️space/os-mcp"` — this is
  exactly audit-semio-mcp.md gap 9 ("`.mcp.json` binds `--folder .` (repo root)… Fix: point `--folder` at their
  space directory"), now landed. **Verified live-current**: reading `.mcp.json` today shows the same
  `--folder .🧬semio/🔗️space/os-mcp` and `--scopes workspace.read,artifact.write,inference.execute,ui.observe,
  ui.control,conversation.write`. The directory does not exist on disk yet, but `open_folder`
  (`🌉️mcp/🏠️workspace/🦀️.rs:3332-3334`) calls `std::fs::create_dir_all(&path)` before opening, so the first
  connect self-provisions it — **zero-touch is preserved**, not broken by the path change. Not independently
  re-run live this session (no servers permitted); the code path is unambiguous.
- **Binary discoverability for Claude Code specifically** (`GATEWAY_TOOL_NAMES: [&str; 28]`, `🌉️mcp/🦀️.rs:282`,
  verified this session) is unchanged since GJ2/G4–G8: 28 tools, matches every wp-g* headline.
- **Still open (not re-verified live this session, no session-10/11 slice touched it):** P1 gap from
  audit-semio-mcp.md — a `--hub` agent still needs `--credential-file`/fd-3 wiring by hand; no in-product "copy my
  MCP config" flow exists (G19 gap 10, restated unchanged in peer-audit-g10.md Outcome-4 row "no production caller
  spawns `semio-os-mcp --hub` with a delegated credential" — still true as of 09-24 12:45, G4's own probe script is
  a **test harness**, not a product entry point).

### (2) Discovery, open/create artifacts, prepare/invoke, snapshots, undo/redo, transactions

**Status: WORKS-LIVE for the mutation chain and history; PARTIAL for search-quality/localization.**

- **The full mutation chain is live-proven, repeatedly, not just once.** `wp-g4.md` §3.1 stdio probe transcript
  (dated 09-24, live): `artifact_create` → `artifact_open` → `action_prepare addBlock` → `action_invoke SUCCEEDED`
  → `artifact_snapshot` (spr 223→718) → `history_undo` → `history_redo` → `transaction begin→commit` (snapshot
  moved) → `begin→rollback` (unchanged). **`client-e2e` 38/38** independently re-run green by G4, G5, G6, G7, G8 (five
  separate reports, each its own capture file, 09-24). This is peer-audit-g10.md's own top finding: "the single
  largest change since G10" — the previous ticket-round's blocking defect (retained-command-owner mismatch, A2 §6.1)
  is fixed, author unclear across the fleet but the fix itself independently re-confirmed 5×.
- **The undo/redo revision-stamp defect found by CE1 (09-20/21) is fixed and stayed fixed.** `history_undo` used to
  leave `RevisionStamp.head_edit_id` unchanged (an optimistic-concurrency hazard); one-line fix at
  `🌉️mcp/🏠️workspace/🦀️.rs:1793` per CE1, not contradicted by any later report.
- **`capabilities_search` differentiates hits** — standing since before this ticket, re-confirmed indirectly by
  every wp-g4–g10 stdio probe reusing the same catalog machinery.
- **PARTIAL / stale claim worth flagging: verb descriptions non-empty + localized, raw-input-event exclusion
  (G19 gap 8 / peer-audit-g10 gap G8).** G19 (09-22) measured every capability's `description: ""` live. No
  session-10 or session-11 report (G4 through G10, this session's own skeleton) claims to have touched MCP
  capability description text at all — **peer-audit-g10.md marks this explicitly "NOBODY" in session 10**, and
  this session's read confirms nothing has changed it since. **Still open, unowned.**
- **Capability-audit (destructive-verb declarations) is measurably better but not zero.** CE1 (09-20) measured 111
  findings → CE2/G10-era measured 29 findings/1 diagnostic, unchanged across CE2→CE3→peer-audit-g10 (09-22 through
  09-24). No session-10/11 report claims further progress. **Still 29 findings open** (14 plugins need
  `effects.destructive`/`audience` metadata + re-describe) — peer-audit-g10 gap not superseded.

### (3) Approvals for destructive verbs, visible to the human

**Status: WORKS-LIVE on both channels (elicitation AND the shell), with two named residual gaps.**

- **Elicitation-channel approval (Approve Once / Deny / session) is live-proven**: CE2 (09-21/22)
  `live-agent-loop-check` 21/21 including (e1)/(e2)/(e3) against a live `s` shell on `:6196`.
- **AP1 (09-22) specifically drove the SHELL lane** (a client with no elicitation capability, which no earlier gate
  had ever exercised) and made the shell's own approval affordance real, not thin: before, the chat panel printed
  the gateway's raw JSON blob with no verb/target/who-asked/countdown. After: `capabilityTitle`, `description`,
  `artifactKind`, `timeoutMs` (a duration, not a deadline, so browser/gateway clock skew cannot show an
  already-expired countdown) — shared fixture + two parsers (React `AgentApprovals/🟦️.tsx`, wgpu
  `AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs`), one live gate result: `live-agent-loop-check` moved from **14 pass/4
  fail/1 skip of 19 → 16/2/1**, with `(e1)` now measured **`affordance=dialog countdown=120`** and `(e2)`
  **`channel:"shell", PERMISSION_DENIED`** over the real shell, not the elicitation fallback.
- **Cancellation (a close cousin of approval-visibility) is live-proven repeatedly and fast**: in-flight cancel
  3 ms (G4), 10 ms (G5), 4–17 ms (G6), 1 ms (G7, for `artifact_create` mid-cold-compile), each with a dedicated
  language-agnostic law (`binding-cancellation-law.json`, `compile-cancellation-law.json`) plus Rust replay.
- **Gap, restated and re-confirmed unowned (peer-audit-g10 Outcome-4 row, not touched since):** the
  destructive-capability **approval** *accept* path specifically (as opposed to cancellation) was not re-driven in
  session 10 — the closest re-proof is AP1's (e1)/(e2), which is 09-22, three days stale relative to this session
  but not contradicted.
- **Named residual gaps from AP1 itself, not closed since:** two approval surfaces (`AgentApprovals` modal +
  `AgentChatPanel` inline) are not unified — the modal's veil blocks the inline group until dismissed (AP1 §6 item
  5); the wgpu approval overlay does not yet paint the richer fields/countdown, only the four legacy summary rows
  (AP1 §6 item 6). No later report (through session 11's skeleton) claims either is fixed.

### (4) Agent as a distinct principal in a shared hub space, human sees edits live

**Status: PARTIAL, RUNTIME up to the wire, blocked on one hub-side ledger defect that is fixed in source but not staged.**

- **Live-proven, this session's own predecessor slice (G4, 09-24, `wp-g4.md` §4):** a human signs in, delegates an
  agent, holds a live document socket on a note. The agent, over `semio-os-mcp --hub` with a delegated credential,
  opens the **same** document and commits `addBlock` → SUCCEEDED. **The human's live socket receives the agent's
  `Commands` frame** (1 envelope, actor `hub.v1.51fb…` — distinguishable from the human's own actor id). This is the
  concrete, measured answer to "does a human see an agent's edit land live" — yes, at the socket-relay layer.
- **Red, root-caused, fix landed but not staged (the actual current blocker):** `GET /spaces/{s}/documents/{d}`
  stays at `head_seq=1 commit_seq=1` after 4+ accepted agent edits — a fresh joiner's catch-up replays only 1 frame.
  Root cause (H4, cross-referenced by G4 and peer-audit-g10): the guest mints a **constant** edit id per process (no
  per-session entropy: `edit-e73113e9c5251c7e` in all 5 of G4's probe runs, hours apart, fresh processes), so the
  hub's dedup-by-command-id treats every agent edit as an idempotent replay — Accepted, nothing written. H4 fixed
  this in source (wasi:random entropy, replica-scoped edit/op ids, hub refuses colliding-content resends) and it is
  a law now, but **it needs the same W1 guest rebuild that Outcomes 1 and 3 are also blocked on** — per
  peer-audit-g10 §E, this is "the single highest-leverage item left in the whole ticket," shared across three
  outcomes. **Not verified fixed this session** — no wp-g* report after G4/H4 claims the rebuild landed or that the
  ledger now advances.
- **No presence beat from the agent's actor** — the human saw an empty presence roster during G4's run (a named,
  separate small gap).
- **First-class "AI agent principal" concept (roster kind, UI treatment) still does not exist** — the actor id is
  distinguishable at the wire level (proven), but there is no product-level principal concept beyond that
  (peer-audit-g10 Outcome-4 row, unowned in session 10, not touched in the session-11 skeleton either).

### (5) Inference services — generic quartet, `inference_run`, progress/cancel

**Status: WORKS-LIVE for `inference_run` (guest lane, wfc) and cancellation; TESTS-ONLY / PENDING for the generalized
hub quartet (`inference_submit`/`events`/`cancel`/`approve` beyond GIS).**

- **`inference_run` over MCP genuinely finishes now**, after a real multi-slice throughput chase: GJ3 (09-23) fixed
  a starved-not-runaway pool-pump bug; WI1/GJ2 got the wfc solve from "never answers within 240–600 s" to a real
  answer; G5 (09-24) got a specific genesis solve from ">580 s, unfinished" to **38.4 s warm** (2.07× a native
  18.5 s reference, narrowly missing a <2× target); G6 removed idle-pool CPU entirely (8%→0.00 s) and got the
  factor to **1.29–1.33×** CPU; G7 made `artifact_create` itself a cancellable job and removed the epoch-keeper
  overhead, landing at **1.28–1.30× process CPU**. Every one of these re-ran `client-e2e` **38/38** and the stdio
  probe **17/17** as regression gates — this is unusually well-verified, not a single claim.
- **Binding/compile/create are all now cancellable jobs with real progress**, not opaque waits: `artifact_create`
  reports `resolving→reading→hashing→loading-compiled-code→compiling-component`, and a cancel mid-cold-compile
  (which used to run to completion in the background) is now answered in ~1 ms and the isolated compile worker is
  actually killed (G7 §1, law `compile-cancellation-law.json`, 1/1, live stdio capture).
- **The generic inference-job quartet (G9, dated 09-24, this session's direct predecessor) is DESIGNED and
  IMPLEMENTED IN SOURCE, not yet live-proven.** G9's own status table: items 1–6 DONE (design, schema-first wire,
  MCP quartet generalized to guest+hub sites, hub publishes `features.inferenceServices`, wfc `pin-solution` commit
  action native-tested); items 7–10 (frozen-crate hunk application, guest rebuild, live stdio+hub proof,
  client-e2e/hub-agent-participant re-run) are **PREPARED but PENDING**, waiting on the coordinator's freeze lift
  and a W1 guest rebuild that carries the new wfc contract. **This session's own G10 slice inherits exactly this
  as item 1 ("Landing: G9 commit binding… IN PROGRESS") and item 2 ("Generic inference quartet live… PENDING")** —
  confirmed from `wp-g10.md`, written at the start of this session, before this audit began; not yet advanced.
- **Hub-side quartet generalization beyond GIS is the literal open item**: peer-audit-g10's own row
  "`inference_submit/…` hardcoded to gis — UNCHANGED" is precisely what G9 addresses in source; it is real
  progress but not yet a live fact.
- **stdio's 68 real geometric inference algorithms and the cad-extension's inference remain unreachable** —
  G19 (09-22) root-caused both (missing catalog descriptor for the whole `stdio` plugin; extension-contributed
  inferences never reach the roster builder). No later report claims either is fixed; not contradicted, still open.

### (6) Human sees the agent's tool-call transcript / conversation in the shell

**Status: WORKS-LIVE, proven with a 9/9 gate and a screenshot, on the React shell only.**

- **AC1 (09-22) closed this cleanly and is the best-evidenced claim in this whole audit.** New bridge frame
  `GatewayToShell::AgentReply` (tag 10), a 28th tool `conversation_reply` gated by a new `conversation.write` scope,
  an `AgentConversationEntry::agentMessage` kind, one render branch in `AgentChatPanel`. Live gate
  `agent-reply-check`, **9 passed / 0 failed / 0 skipped of 9**, capture `ac1-agent-reply-gate.txt`, plus a
  screenshot (`ac1-agent-reply-panel.png`) showing tool-call rows, an AGENT row, and a YOU row in the same
  transcript. The reverse direction (human types, agent is notified) also pushes
  `notifications/resources/updated` on `semio://ui/agent-messages` rather than requiring the agent to poll — also
  measured live in the same gate (step 5).
- **`GATEWAY_TOOL_NAMES` is still 28 as of this session's own tree read** (`🌉️mcp/🦀️.rs:282`, verified
  2026-09-25), consistent with AC1's "twenty-eight" and every wp-g4–g10 report — this specific claim has not gone
  stale.
- **Named, unclosed gap: the wgpu shell renders no reply at all.** AC1 explicitly did not touch the third codec
  twin (`AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs:334`); a reply frame there decodes to `BridgeFrameFault::UnknownTag(10)`
  (not a crash, but silently dropped). No session-10/11 report touches this file for this purpose (WG6/N2's edits
  to the neighboring wgpu Shell file are for hub sign-in/relay, not agent chat). **Still open, unowned by any named
  slice.**
- **Other AC1-named gaps, unconfirmed changed:** reply is a broadcast projection with no retention (a shell that
  connects late never sees prior turns); nothing server-side nudges the agent to reply (by design, no model
  provider in this crate); truncation past 2048 bytes is silent to the agent. None contradicted by later reports.

### (7) Docs + launch rows

**Status: PARTIAL — real functionality, undiscoverable through the sanctioned devflow for two important gates.**

- **README is substantively current as of AC1 (09-22)**: corrected destructive-verb counts (measured 29
  findings/128 `destructive:true` occurrences across 27 plugins vs 4309 `false`), a real worked `--hub
  --credential-file` example, `conversation_reply` documented, 28-tool count. Not re-verified this session for
  currency against G9's inference-quartet rename (`HubInference*` types, route-parameterised hub calls) — a
  reasonable follow-up, not confirmed stale.
- **Verified this session, a real gap:** `.vscode/launch.json` has rows for `🛠️dev🌉️os-mcp🧵️stdio`, `🌐️http`,
  `🤝️client-e2e`, `📦️build-release`, `🚚️publish`, and gates `⚖️gate🌉️os-mcp🚨️capability-audit`,
  `🤖️live-agent-loop`, `🤝️hub-edit-durability` — **but NOT** `hub-agent-participant-check` or `agent-reply-check`,
  even though both exist as real, working nx targets (`🌉️mcp/📦️packages/🦀️rust/📋️project.json:195,215`) that
  G4–G8 and AC1 all run repeatedly and cite as green gates. AC1's own report claims it registered
  `⚖️gate🌉️os-mcp💬️agent-reply` at launch order `411.107585` — **that row is not in the current
  `.vscode/launch.json`** (grepped `agent-reply`/`hub-agent-participant` against the live file, 09-25: zero hits in
  `launch.json`, zero hits in `.vscode/🧩️launch.seed.jsonc`). Either the row was dropped by the 09-24
  "Reorganize framework product modules" commit (`82c0bdf59a`) or never actually landed. **This directly violates
  AGENTS.md's own standing rule** ("All devs are using `launch.json` and never use the cli… You MUST register all
  executable commands there"), for two of the ticket's most load-bearing live gates.
- **`inference-bridge-check`** (referenced in G9 as `inference-bridge-check --source`) was not found as a target in
  the os-mcp Rust package's `project.json` in this session's grep — it likely lives under the separate
  `💡️inference-bridge` module; not tracked down further under the time-box. Flag as unverified, not as a gap.

### (8) MCP protocol conformance — resources/subscribe, prompts, elicitation

**Status: WORKS-LIVE for prompts and elicitation; PARTIAL/verified-in-source-only for resources/subscribe's actual
push (mechanism real, no live capture of a client receiving a push this session or any read report).**

- **`resources/subscribe` is real, not a false advertisement — verified directly in the tree this session.**
  `🌉️mcp/🧭️protocol/🦀️.rs:295-296` (`METHOD_RESOURCES_SUBSCRIBE`/`_UNSUBSCRIBE`), `:881` (`"resources": {
  "listChanged": true, "subscribe": true }`), `:927-932` (`ResourceSubscriptions` held per-connection and
  registered with a process-wide broker, doc comment: *"a promise this server keeps rather than advertises"*),
  `:1240-1262` (`handle_resources_subscribe`/`_unsubscribe` real handlers). This **corrects** the original
  `audit-semio-mcp.md` (09-23) claim that this was OPEN/`M5 not started` — peer-audit-g10 flagged this exact
  staleness on 09-24, and this session's own tree read confirms the registry code is real. **However**: no report
  read for this audit (including this session's grep) shows a live capture of `notifications/resources/updated`
  actually reaching a subscribed client for an MCP resource (as opposed to AC1's bridge-side
  `agent_messages_changed()` push, which is a different, already-proven mechanism). **Mechanism verified in
  source; live client-visible push unverified.**
- **Prompts: real, bilingual, unchanged and unproblematic.** 5 prompts (`explore_workspace`, `safe_mutation`,
  `inspect_artifact`, `drive_the_ui`, `undo_last_change`), `en`/`de` via a `locale` argument
  (`GATEWAY_PROMPT_NAMES`, `💬️prompts/🦀️.rs`) — consistent across every report from audit-semio-mcp.md through
  this session.
- **Elicitation: real, and specifically proven to have the RIGHT fallback semantics.** `🧭️protocol/🦀️.rs:860,870`
  tracks whether the connected client advertised `elicitation` capability; `ApprovalCoordinator::resolve` offers
  elicitation first and falls back to the shell affordance only for a non-eliciting client (AP1 §4, live-proven,
  09-22). `ELICITATION_TIMEOUT_MS = 120_000` (also verified: AP1 measured a 120 s countdown live). This is a
  materially better answer than a generic "elicitation exists" — the fallback boundary itself was found to be
  previously untested and is now covered.
- **One protocol-adjacent gap restated from the original audit, not contradicted:** duplicate
  `semio://workspace/artifacts` resource-list entry — cosmetic, unowned since the 0918 ticket (peer-audit-g10 gap
  G9).

---

## 2. Stale-claim corrections applied in this pass

1. **`resources/subscribe` "OPEN, M5 not started"** (original `audit-semio-mcp.md`, 09-23) is **stale** — the
   mechanism is real in the current tree (§1.8 above). Correcting per peer-audit-g10's own flag, independently
   re-verified this session by direct grep.
2. **"`.mcp.json` binds `--folder .` (repo root)"** (original `audit-semio-mcp.md` gap 9, 09-23) is **stale** — a
   09-24 commit changed it to a dedicated space directory that self-provisions on connect (§1.1). This is a genuine
   fix, not previously flagged by peer-audit-g10 (which read the tree at 09-24 12:45, likely before or concurrent
   with the 12:11 commit).
3. **"binary not present / build extremely slow" P0** (original `audit-semio-mcp.md` gap 1, 09-23) is **stale** as
   a standing blocker — `ensureMcpBinary` zero-touch staging (GJ2, 09-23) is verified still wired in the current
   `📜️script.ts` this session.
4. **The mutation chain "blocked on retained-owner defect"** (peer-audit-g10's own read of the original audit) is
   **confirmed fixed and independently re-verified 5× more** since peer-audit-g10 was written (G6, G7, G8 each
   re-ran `client-e2e` 38/38 after peer-audit-g10's 09-24 12:45 cutoff).
5. **Approval-affordance status**: peer-audit-g10 (09-24 12:45) says "the approval/elicitation accept path
   specifically was not re-tested" in session 10 — technically still true for session 10, but this undersells the
   real state: AP1 (09-22, in the *prior* ticket, read directly for this audit) already live-proved both the
   elicitation-channel accept **and** the shell-channel accept with a real countdown UI. Not stale, just worth
   surfacing since peer-audit-g10 did not cite AP1 by name in that row.

---

## 3. Gaps, ranked

### P0

**G-P0-1 — Hub ledger does not advance after an agent's relayed commit (blocks journey 4's actual bar).**
- Evidence: `wp-g4.md` §4, §8 item 5; root-caused by H4, cross-referenced in peer-audit-g10 §E as the single
  highest-leverage shared blocker across Outcomes 1/3/4.
- Acceptance: a fresh human joiner's Welcome catch-up on a hub document replays every one of N agent-committed
  edits (`head_seq`/`commit_seq` advance past 1); H4's per-session edit-id entropy fix is staged in a guest build
  the MCP gateway actually loads; `wp-g4.md`'s own `g4-agent-edit-human-sees.ts` repro goes from 6/7 to 7/7.
- Owner: **W2** (owns the guest rebuild/catalog publish this depends on) to land the rebuild; **H9** to re-verify
  the hub-side ledger advance once it's staged; **G10** to re-run the live proof script.

**G-P0-2 — Generic inference quartet (G9) is prepared but not applied/staged/live-proven.**
- Evidence: `wp-g9.md` status table items 7–10 PENDING; `wp-g10.md` (this session) items 1–2 inherit exactly this,
  still IN PROGRESS/PENDING at read time.
- Acceptance: `wp-g9/g9-apply-commit-binding.py` lands compile-atomic; wfc guest carries `pin-solution`; hub
  publishes `features.inferenceServices` live; `wp-g9/g9-quartet-live.ts` runs green over stdio + a real hub;
  `client-e2e`/`hub-agent-participant-check` re-run green on the new tree.
- Owner: **G10** (this session's own slice — already scoped to it).

### P1

**G-P1-1 — `hub-agent-participant-check` and `agent-reply-check` are not registered in `.vscode/launch.json` (or
the seed), violating AGENTS.md's own launch-row rule, for two of the ticket's most-cited live gates.**
- Evidence: verified this session by direct grep against the live `launch.json` and seed file (zero hits for
  either gate name); both exist as real nx targets (`🌉️mcp/📦️packages/🦀️rust/📋️project.json:195,215`); AC1
  claims it added the agent-reply row at order `411.107585` — not present now.
- Acceptance: both gates have a `.vscode/launch.json` row (and seed entry) in the existing `4_gate` group /
  ordering convention, discoverable without the CLI.
- Owner: **G10** (small, self-contained, no wasm/hub dependency).

**G-P1-2 — Capability-audit findings stuck at 29 findings / 1 diagnostic across three sessions (CE2 → CE3 →
peer-audit-g10 → this session), no session-10/11 slice claims further progress.**
- Evidence: `wp-g*.md` series never mentions `capability-audit-check`'s count changing after CE2/CE3; G19 gap 8.
- Acceptance: `capability-audit-check` finding count decreases, ideally to 0, across the named ~14 plugins
  (architect, energy, gis, layout, lowpoly, mathematical, norm, playbook, sequence, trinity, vcs, wfc, writer,
  demonstrator).
- Owner: **NEW** (no current slice owns plugin-descriptor `effects`/`audience` metadata edits at scale;
  peer-audit-g10 also lists this unowned).

**G-P1-3 — `stdio`'s 68 real geometric inference algorithms and the cad-extension's inference remain entirely
unreachable via `inference_list`/`capabilities_search`.**
- Evidence: G19 §1c (09-22), root-caused (missing catalog descriptor for the whole plugin; extension-contributed
  inferences never reach the roster builder); not contradicted by any later report through this session.
- Acceptance: `inference_list` includes `stdio`'s services once it has a committed descriptor (a separate,
  larger plugin-descriptor gap — see W2/catalog ownership); the roster builder walks extension contributions so
  `cad-extension-aec-building`'s service appears too.
- Owner: **W2** (stdio descriptor is a catalog-completeness problem) for the descriptor half; **G10** for the
  roster-builder extension-walk half (small, self-contained in `💡️inference/🦀️.rs`).

**G-P1-4 — Verb descriptions are still empty/non-differentiated for `capabilities_search`, and raw input events
are not excluded from the default result set (G19 gap 8a / peer-audit-g10 gap G8).**
- Evidence: G19 (09-22) measured every capability's `description: ""` live; peer-audit-g10 explicitly marks this
  "NOBODY" in session 10; this session's read found no contradicting report.
- Acceptance: `capabilities_search "draw rectangle"` returns a differentiated, non-tied top hit; raw pointer/
  engagement events excluded from default results.
- Owner: **NEW**.

### P2

**G-P2-1 — wgpu shell renders no agent reply at all (drops tag 10 silently) and its approval overlay lacks the
richer AP1 fields/countdown.**
- Evidence: AC1 §7 item 1 (unowned hand-off, explicitly not done because the wgpu Shell file was mid-edit by a
  parity ticket); AP1 §6 item 6 (same wgpu-lag pattern, different surface). No session-10/11 report touches either.
- Acceptance: wgpu `AgentBridge` decodes tag 10 into an `AgentConversationEntry`; wgpu `AgentApprovals` overlay
  paints capability title/description/target/countdown, matching the React bank's fixture-driven parser.
- Owner: **S15** (frontend/renderer parity is that slice's natural home) — sequence after any wgpu-Shell-editing
  ticket currently in flight, per AC1's own note that the file is hot.

**G-P2-2 — No first-class "AI agent principal" concept (roster kind, UI treatment) beyond wire-level actor-id
distinguishability.**
- Evidence: peer-audit-g10 Outcome-4 row, unowned in session 10; G4's live run only proves actor ids differ, not
  that any UI treats them specially.
- Acceptance: a human's roster/presence UI visibly marks an agent-originated session as an agent, not merely a
  differently-colored human.
- Owner: **C10** (collaboration/presence UI) or **NEW**.

**G-P2-3 — No production, in-product flow to spawn `semio-os-mcp --hub` with a delegated credential; the only
example is a ticket-owned test script.**
- Evidence: G19 gap 10 / peer-audit-g10 Outcome-4 row, unchanged.
- Acceptance: an in-product "copy my hub-bound MCP config" action (off e.g. an AgentDelegations panel) produces a
  working `.mcp.json`-shaped entry a real client can paste in.
- Owner: **NEW** (frontend + credential-delegation UI, cross-cutting S15/H9).

**G-P2-4 — Two approval UI surfaces (modal `AgentApprovals` + inline `AgentChatPanel`) are not unified; the modal's
veil blocks the inline decision until dismissed.**
- Evidence: AP1 §6 item 5, not contradicted since.
- Acceptance: one live affordance per approval, reachable without an extra dismissal.
- Owner: **S15**.

**G-P2-5 — Duplicate `semio://workspace/artifacts` resource-list entry (cosmetic).**
- Evidence: standing since the 0918 ticket, restated by peer-audit-g10 gap G9, not touched since.
- Owner: **G10** (one-line fix, whenever convenient).

---

## 4. What this session's own G10 slice should read first

`wp-g9.md` (full, esp. §4's frozen-crate hunk) and `wp-g10.md`'s own skeleton — this session's slice is already
correctly scoped to G-P0-2 (item 1: landing G9's commit binding; item 2: generic quartet live) and already lists
4a/4b/4c/4d (zero-touch connect, delegated-agent-edit-visible, destructive-verb approval visible, React transcript
Playwright-verified) as its own pending items — items 4a/4c/4d are **already substantially proven** by GJ2/AP1/AC1
respectively (§1 above); the highest-leverage remaining original work for this slice is 4b (blocked on G-P0-1,
shared with H9/W2) and closing G-P1-1 (the missing launch rows), which is cheap and self-contained.
