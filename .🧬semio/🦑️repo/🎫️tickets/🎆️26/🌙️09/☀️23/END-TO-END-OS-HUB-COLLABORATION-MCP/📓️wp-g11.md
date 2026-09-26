# WP-G11 — AI Integration For Users Over The Semio MCP (Session 13)

Slice G11 · session 13 · 2026-09-26. Successor of G10 (`📓️wp-g10.md`; also `📓️wp-d1.md`, `📓️audit-s12-ai-mcp.md`).
Owns outcome 4 (semio MCP `semio-framework-os-mcp`, `mcp__semio__*`, never the repo MCP). Ports: hubs 8030–8039, serves
6530–6539. Private cargo target `.tmp-ticket/wp-g11/target`. Durable data + logs `.🧬semio/🌐hub/s13-g11-*`
(logs `.🧬semio/🌐hub/s13-g11-logs/`). Scripts `wp-g11/`. Every build/test niced (`nice -n 10`).

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | S4 user path on a CURRENT-tree hub: sign in → "Set up MCP client" → official `@modelcontextprotocol` SDK client connects, lists tools, edits a hub doc with approval in the shell → revoke; en 8/8 + de 8/8, timed | **BLOCKED on binaries (measured)**: en-1 on hub 8030 (C11's 19:07 os-hub) 3/8 — the last staged gateway (15:12) cannot bind the current hub (`hub directory stream is unavailable; authenticated snapshot was not activated`) and R9 moved revoke to `POST …/revoke` at 20:06 (no pre-20:06 hub serves the shell's Withdraw). Coordinator: runs on W3's rebuild (B3 on 7800 + current gateway). Battery scripted (`g11-battery.sh`, product `user-path-check`) |
| 2 | Hub quartet 19/19 (verify H11's S2/S11 hub fixes live), hub-agent-participant 17/17, durability, live-agent-loop en + de, agent-reply, client-e2e, MCP conformance TS + Rust (official SDK oracle); folder lane + hub lane, current tree | WAITING (same binaries; rule 25 forbids test builds until REBUILD START); quartet + participant in the battery; H11 gets rows 9/14/16/17 |
| 3 | After the all-package publish: every package over MCP against the hub (`capabilities_search`, `inference_list`/`run`/approve, `artifact_create` for every creatable kind, `action_invoke` + undo/redo, `ui_reveal`/`ui_focus`), en + de, approvals visible in the shell, per-package table | WAITING (W3 publish); product harness `plugin-coverage-check` (V1) |
| 4 | Security: revocation effective on the next request + closes sockets (H9 fence), rate limits, per-tool scopes, prompt-injection adversarial fixture green | probe written (`g11-security-probe.ts`: scopes matrix, audience cap, agent-session burst 429, revoke → next request + hub socket close); runs in the battery. Rate limits extended by (c) |
| a | Coordinator: land G10's `g10-preview-effect-refusal.py` | **SUPERSEDED, not applied** (P8 agent-lane refuses the same lane by name); MCP maps `interactive-job.agent-lane-uncarried` → `PLUGIN_UNAVAILABLE` + en/de remedy + law (in tree, compile check pending) |
| b | Coordinator: the 7 folder↔hub lane-parity reds → owned-child op groups ride the agent transaction | **PREPARED, dry-run clean** (`g11-child-carrier.py`, 90 hunks / 17 files: store `GroupMeta.group_id`, spr channel v18 + TS twin + hex fixtures, SDK preview/prepare/commit/undo, gateway `PreparedOps.children`, laws Rust SDK + MCP + channel round trips); waits for LC's P8 agent-lane to settle in `🔌️plugin/🦀️.rs` (LC's gate stopped 21:0x) |
| c | Coordinator (G13-P2-1): per-session tool-call budget in the hub rate limiter | **APPLIED 21:16, check running**: 5th class `agent-command` (schema enum + policy burst 120 / 250 ms), per agent SESSION, the document socket PACES an agent `Commands` frame ≤ 10 s before re-reading its authority (revocation still fences), refuses beyond by name; laws `an_agent_session_is_paced_for_exactly_its_refill_and_refused_beyond_its_patience`, `every_rate_limit_class_is_a_schema_member_and_its_policy_validates` (`g11-agent-command-budget.py`) |

### Session 13 Pids

| pid | what | started | stopped |
|---|---|---|---|
| 19267 | `g11-hub-build.sh os-hub-g11-1`: current-tree `os-hub` (19:12 tree) → `s13-g11-bin/os-hub-g11-1` (sha `34e08e69…`) | 19:12 | 19:41 done |
| 39134 (hub 39137) | hub hold `g11-hub.sh 8030 w2-catalog-b2 <C11 19:07 os-hub> hub-8030` → data `s13-g11-hub-8030`, state `s13-g11-hub-8030-state`, users user1/2/3 | 19:40 | 20:4x SIGTERM → `CHILD_EXIT code=0` (rule 22; root kept) |
| 41306 | `g11-check.sh semio-framework-os-mcp` (fault mapping) | 19:44 | 20:13 red on a peer's mid-landing `component-codec` (`genesis` removed from the impl before the trait) |
| 61834 | S4 en-1 (`g11-user-path.sh`) | 20:15 | 20:16, 3/8 (see item 1) |
| 64030 | os-mcp build (restage) | 20:18 | 20:40 stopped by me (rule 25: 21 min waiting on build-dir locks, no rustc) |
| 87336 | os-mcp check 2 | 20:43 | 21:05 stopped (same, then rule 26 → build-fleet-b) |
| 23814 | `g11-check.sh semio-hub -- --bins` (set c) in build-fleet-b | 21:16 | |
| 41751 | `s` react dev serve :6531 → hub 8030 (`g11-serve.sh`, rendezvous `s13-g11-bridge`, credentials `s13-g11-credentials`) | 19:44 | 20:4x (rule 22) |
| 41756 | `s` react dev serve :6530 local-only (same rendezvous) | 19:44 | 20:1x (rule 22 memory budget; restart for live-agent-loop later) |

### Session 13 Log

- 19:0x started; read AGENTS.md, preambles 13 + 12, `📓️wp-g10.md`, W2 Hub Handoff, H9 (revocation fence in the tree,
  Qb/Qc hub fixes), H10. Nothing of mine runs. 7800 = B2 (hub 54029). Load 41, 111 GiB free.
- 19:12–19:41 current-tree `os-hub` built (`s13-g11-bin/os-hub-g11-1`, kept); coordinator then pointed at C11's 19:07 binary
  (proven to boot B2): hub **8030** runs that copy on a fresh B2 clone (`g11-hub.sh`), **ready 19:53:59** (13.7 min at load ~95).
- 19:4x coordinator add-ons: (a) land G10's `g10-preview-effect-refusal.py`; (b) the 7 folder↔hub lane-parity reds are G11's;
  (c) P2 per-session tool-call budget. **(a) superseded, not applied**: P8's `p8-agent-lane` (being landed by LC, in the tree since
  19:07, `🔌️plugin/🦀️.rs` `preview_addressed_action`) already refuses a whole-document load (and downloads, file requests,
  extension calls, tasks, owned children) as `interactive-job.agent-lane-uncarried`; G10's hunk would sit after that check as dead
  code. Coordinator agreed. MCP side done instead: `map_fault` maps `interactive-job.agent-lane-uncarried` → non-retryable
  `PLUGIN_UNAVAILABLE` with `details {faultCode, remedy {en, de}}` + law
  `a_verb_whose_lane_an_agent_cannot_carry_is_refused_by_name_with_an_en_and_de_remedy` (check running).
- 20:0x (b) design agreed with the coordinator: owned-child op groups ride the agent transaction (spr channel
  `AppFrame::Emit.child_ops` + `AppCommand::TransactionPrepare.prepared_child_ops`, CHANNEL_VERSION 18, TS twin + fixtures, SDK
  preview/prepare/commit through `dispatch_emit_group` with the transaction id as the group id, gateway `PreparedOps.children`).
  Prepared as one patch set (`wp-g11/g11-child-carrier.py`) because LC's `p8-land.py` is still writing/gating `🔌️plugin/🦀️.rs`
  (a failing gate restores its backups, which would erase a concurrent edit); lands after LC's P8 run, before REBUILD START.
- 20:1x rule 22 (swap 21.6/22.5 GB): serve :6530 stopped (not needed this hour); hub 8030 + serve :6531 kept for S4.
- 20:15 **S4 en-1 on hub 8030: 3/8** (`s13-g11-logs/user-path-en-1.txt`): rows 0–2 PASS (fresh space + note by hub authorities in
  35 s, clean-profile sign-in lands in the space, "Set up MCP client" writes a 0600 credential + one stdio entry), then the
  official SDK client's connect failed `Connection closed`. Reproduced by hand: the pinned 15:12 gateway (`s13-g11-bin/semio-os-mcp-1512`,
  the last staged one; the dist is stale since `nx.json` changed) retries 6× `hub binding is not settled yet (hub directory stream is
  unavailable; authenticated snapshot was not activated)` and exits 1 against the 19:07 hub. Also measured: R9 changed the delegation
  revoke to `POST /auth/agent-delegations/{id}/revoke` in hub + shell TS at 20:00–20:06, so the S4 Withdraw row needs a hub built after it.
- 20:18–20:40 current-tree os-mcp build: cargo sat 21 min at 0 % with no rustc child (build-dir lock convoy) → stopped (rule 25).
  20:4x coordinator: no separate os-mcp/os-hub build now (tree mid LD wire change); live items run on W3's rebuild (B3 on 7800 first).
- 20:4x V1 productized my harnesses: `@semio-tech/framework-os-mcp-rs:user-path-check` (rows 0–8 incl. the connected-client refusal)
  and `:plugin-coverage-check`. The battery uses them; new harnesses (security probe, quartet) go to the product next (rule 21).
- 21:0x LC's `p8-land.py` stopped after its agent-lane gate (architect check exit -15, 2190 s): it restored action-bus, retained-command
  and the architect law but left `🔌️plugin/🦀️.rs` (changed after landing by a peer). My set (b) waits for LC; asked LC to sequence.
- 21:16 set (c) applied (4 files); `cargo check -p semio-hub --lib --tests --bins` in build-fleet-b running (rule 26).
