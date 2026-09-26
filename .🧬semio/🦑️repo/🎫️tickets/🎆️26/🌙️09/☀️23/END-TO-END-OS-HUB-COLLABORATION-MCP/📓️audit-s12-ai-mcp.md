# Session 12 — Outcome 4 re-audit: AI integration over the semio MCP

Read-only, no builds/servers/edits, no `mcp__semio__*` mutating calls (per session-12 rules). Repo
`/Users/ueli/Documents/semio`. Scope: `semio-framework-os-mcp` (`.mcp.json` server `semio`), tools
`artifact_*`/`action_*`/`history_*`/`transaction_*`/`inference_*`/`ui_*`/`capabilities_*`, `context_resolve`,
`conversation_reply`, `job_*`. Never the `repo` MCP.

**Method.** Read in full: `📓️session-12-preamble.md`, `📓️audit-s11-ai-mcp.md` (session 11's own outcome-4 audit),
`📓️wp-g10.md` (G10 = MCP slice), `📓️wp-u5.md` (approvals/cancel/frontend), and the session-12-relevant parts of
`📓️wp-s15.md` and `📓️work-packages.md`. Cross-checked the highest-leverage claims directly against the current
tree (`.mcp.json`, `GATEWAY_TOOL_NAMES`, `.vscode/launch.json` + seed, `📒️audit/🦀️.rs`, `🤖️agent-credential/🦀️.rs`,
`🌎️hub/🔐️auth/🤖️agent/🦀️.rs`, the wgpu `AgentBridge` codec) with `/usr/bin/grep`, `git log`, and `Read`, dated
2026-09-25 23:0x–23:3x. **Novel to this pass**: this auditor session is itself a live, zero-touch `.mcp.json`
client of `semio-framework-os-mcp` (pid 88810, `bun ./📜️script.ts dev mcp stdio os --folder
.🧬semio/🔗space/os-mcp …`, alive continuously since 19:36 — it survived the 19:30 desktop-restart boundary the
session-12 preamble describes for every other process). The read-only tools `capabilities_search`,
`capabilities_describe`, `inference_list`, `context_resolve` were called live against this real gateway process
during this audit — this is first-hand, this-session evidence, not a re-citation of an executor's capture file.

**Headline.** Session 12 opened with every executor process dead (19:30 desktop restart) and canonical hub 7800
down; nothing new has landed for outcome 4 yet (G10/U5 have not written a `## Session 12` section; S15 has only
just restarted a serve). So this audit is substantively a **live-verified snapshot of where session 11 left
outcome 4**, corrected in five places by direct evidence gathered this session (three fixes confirmed live that
the session-11 audit had marked open; one gap the session-11 audit under-measured — now shown to be
catalog-wide, not two plugins; one new, previously unassessed security-relevant gap: prompt-injection surface from
shared-document content into tool results). No exploit was run — that would require write access I do not have and
is out of scope for a read-only auditor; the finding is a structural surface, assessed from source.

---

## Summary

| Outcome-4 area | Status | Evidence |
|---|---|---|
| Zero-touch `.mcp.json` connect to a running gateway | **LIVE-PROVEN (this session, first-hand)** | this auditor's own connection; §0 |
| Discovery (`capabilities_search`/`_describe`, `inference_list`) | **LIVE-PROVEN**, differentiated scoring, 68 stdio + cad-extension services all present | §0, §1.2 |
| Mutation chain (`action_prepare`/`invoke`, `artifact_*`, `history_*`, `transaction_*`) | **LIVE-PROVEN** (G4–G8, 38/38 `client-e2e`, repeatedly) | §1.2 |
| Approvals for destructive verbs (elicitation + shell) | **LIVE-PROVEN** on both channels; unified into one affordance (source-verified this session) | §1.3 |
| Cancellation + progress | **LIVE-PROVEN**, fast (ms-scale), including agent-tool-call cancel of a parked approval | §1.4 |
| Delegated agent principal edits shared doc, human sees it live | **PARTIAL**: agent half + wire-level roster LIVE; browser half BLOCKED on catalog/lane skew, not re-proven since session 11 | §1.5 |
| Transcript visible in shell (React) | **LIVE-PROVEN** (9/9 gate, session 9); wgpu now also wires the decode (source-verified this session, corrects session-11 "silently dropped") | §1.6 |
| Undo of an agent edit | **LIVE-PROVEN** (undo/redo in `client-e2e`); hub-document undo has a known `action-owner-mismatch` refusal (C10-owned, unrelated to MCP itself) | §1.5, §1.7 |
| Agent presence | **LIVE-PROVEN at the wire** (`:agent` principal kind, durability gate row 5p); no first-class UI treatment beyond a badge | §1.5 |
| Error/refusal texts en + de | **LIVE-PROVEN** for the approval/shell path; JSON-RPC `GatewayError` bodies are English-only (protocol-level, not shell-rendered) | §1.8 |
| Description-quality of destructive verbs | **LIVE-PROVEN BROKEN, catalog-wide** (this session): every "delete"/"move" verb sampled across ~10 plugins returns `description: ""`; safety metadata (`effects.destructive`, `policy.approval`) is intact, so approval still fires — this is a discovery/UX gap, not a safety bypass | §2 |
| Raw input/engagement events excluded from search | **LIVE-PROVEN FIXED** (this session): `capabilities_search "hover"` → 0 results | §2 |
| MCP works against a hub-only setup (no local `s` session) | **CURRENTLY BROKEN**: canonical hub 7800 is down at audit time; the last hub that had G10's ledger/dialect/stack fixes was G10's own private hub, not 7800 — 7800's last live binary crashes on a hub-side inference approval | §1.5, §3 |
| Credential / delegation scope enforcement | **SOURCE-VERIFIED, well-designed**: 0600 file, 16 KiB bound, wiped on drop, space+audience-scoped, structurally capped at Author/Spectator (no admin path) | §3 |
| Delegation revocation propagation delay | **NEW FINDING**: an already-minted agent session (TTL 1 h) is not killed on revoke; revocation only blocks the *next* re-exchange | §3, P2 |
| Prompt-injection surface (document content → tool result) | **NEW FINDING, UNMITIGATED**: `artifact_snapshot`/`artifact_open` forward a shared document's raw content into the tool result verbatim; any collaborator with write access can plant adversarial text an agent later reads with no untrusted-content marking | §3, P1 |
| Audit-trail redaction (secrets never reach the sink) | **SOURCE-VERIFIED, correct** | §3 |
| `launch.json` rows for the load-bearing gates | **LIVE-PROVEN FIXED** (this session): `hub-agent-participant`/`agent-reply` rows now present in both `launch.json` and the seed | §2 |

---

## 0. This session's own live proof (first-hand, not a re-citation)

This auditor is itself a real `.mcp.json` client. Confirmed live, this session:

- **`context_resolve()`** → a real session opens with `principal: "agent:local"`, `locale: "en"`, `channel:
  "shell"`, `scopes` exactly matching `.mcp.json`'s declared scope list
  (`workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write` decoded into the
  gateway's own scope vocabulary), and `activeArtifactId: "live-agent-loop-muemspcf"` — a residual artifact from an
  earlier live-agent-loop probe run against the same self-provisioned `.🧬semio/🔗space/os-mcp` folder
  (`.mcp.json` line 18: `"--folder", ".🧬semio/🔗space/os-mcp"`, confirmed present on disk with a `.semio/`
  subdirectory — self-provisioning per session-11's audit is reconfirmed, this directory did not exist before the
  first connect and now does).
- **`capabilities_search("draw rectangle")`** → top hit `draw…addLayer` score 20.13 with a real, differentiated
  description ("Appends a new layer of the given kind (a rectangle, ellipse, line, polygon, freehand path, text,
  image, group, boolean or trace) to the top of the drawing."), next-best 8.95 — matches G10's own spot-check
  (`wp-g10.md` item 11) almost exactly (20.1 vs 8.96).
- **`inference_list()`** (workspace-wide) → 68 `s.stdio.gltf.inference.*` services (aspect-ratios through volume/
  void-volume, one row per geometric measure), the 5 `s.wfc.*.solve` services (bitmap, grid2d, grid3d, wfc2d,
  wfc3d — grid3d's carries a `commit.action: "pin-solution"` binding, `s.gis.gismap.inference`, and
  `s.cad-extension-aec-building.building-structure-summary`. This **live-confirms** G10's item 8 ("STALE — live
  `inference_list` already lists 68 stdio gltf services + cad-extension-aec-building") and closes the session-11
  audit's G-P1-3 with first-hand evidence rather than a re-cited claim.
- **`capabilities_search("hover")`** → `{"results": [], "total": 0}`. Raw pointer/engagement interactions are
  excluded from the default result set. This closes half of session-11's G-P1-4 (the "raw input events not
  excluded" half) live.
- **`capabilities_search("wfc grid solve")` / `("architect program")` / `("delete")`** → see §2: this is where the
  description-quality gap was measured directly, and found to be worse and broader than previously reported.

---

## 1. User journey, step by step

### (1) Install/launch → sign-in → in-product MCP client setup

**LIVE-PROVEN**, largely unchanged from session 11, re-confirmed this session by direct read:

- `.mcp.json` (read live): the `semio` server is `bun ./📜️script.ts dev mcp stdio os --folder
  .🧬semio/🔗space/os-mcp --scopes workspace.read,artifact.write,inference.execute,ui.observe,ui.control,
  conversation.write` — a dedicated space directory, not the repo root (session-11's stale-claim correction #2
  still holds). `ensureMcpBinary` zero-touch staging is still wired (unchanged since GJ2); this auditor's own
  connection did not require any manual build step.
- **In-product "copy my hub-bound MCP config" flow**: `wp-g10.md` item 10 (session 11, dated ~02:5x) claims this
  was built and live-proven en 8/8 + de 8/8 (`AgentDelegations` panel → "Set up MCP client" → Copy → a pasted
  entry connects as `agent:<delegation>`). Source for the UI exists (`📇️directory/🤖️delegations/🟦️.ts` +
  fixture `🔌️mcp-client-config.json`, confirmed present this session). **Not independently re-run live this
  session** (would need a running `s` host + hub, neither available at audit time) — carried forward as
  LAW-ONLY/SOURCE-VERIFIED for session 12, previously LIVE-PROVEN in session 11. This closes session-11's
  G-P2-3 ("no production flow, only a test script") if the session-11 claim holds, which nothing this session
  contradicts.

### (2) Credential scope, expiry, revocation

**SOURCE-VERIFIED, well-designed** — see §3 for the full security read. Summary: file-based (0600) or fd-based
delivery, never argv/env/URL; 16 KiB bound; wiped from memory on drop; hub-side delegation is
space-scoped + audience-scoped (`read`→Spectator, `edit`→Author, **never** an admin role — "an agent can never
invite, revoke, or delegate onward", `🌎️hub/📇️directory/🦀️.rs:278-280`); delegation TTL 60 s–90 days (default
7 days); minted **agent sessions** are short (1 h) and every re-exchange re-checks revocation. **Residual gap,
new this session**: a session already minted before revocation is not itself killed — see §3, P2.

### (3) Tool discovery

**LIVE-PROVEN** this session (§0). `GATEWAY_TOOL_NAMES` (`🌉️mcp/🦀️.rs:282-311`) is a stable, non-progressive
census of 28 tools — a tool's *presence* never depends on server tier, only its *result* does (doc comment at
`:277-281`, unchanged since session 9). `capabilities_search` is BM25, deterministic, no LLM
(`capabilities_search`'s own tool description, confirmed by this session's live scores). Pagination
(`nextCursor`) works (confirmed: `"delete"` query returned `total: 48` with `nextCursor: "10"`).

### (4) Per-plugin coverage

**LIVE-PROVEN broad, not exhaustive.** This session's live `inference_list()` and several `capabilities_search`
calls surfaced draw, wfc (×5 kinds), architect, animate, lowpoly, procedural, puzzle (×3 kinds), reasoning, dag,
demonstrator, flow, imperative, playbook, process, sequence, gis, stdio, cad-extension-aec-building — a wide
cross-section of the ~34-plugin catalog is reachable through the MCP surface right now. Not exhaustively swept
(would need a query per plugin, out of this session's time-box); no evidence found or reported this session of a
plugin absent from the catalog entirely (the one historical case, stdio's 68 inference services, is now present —
§0).

### (5) Approvals for destructive verbs

**LIVE-PROVEN on both channels, and now source-confirmed unified into one affordance.**

- Elicitation-channel and shell-channel accept/deny were both live-proven in session 11 (CE2, AP1) and re-proven
  repeatedly by G4–G10's `live-agent-loop` runs (22/22 en, 21–22/22 de across five independent re-runs,
  `wp-g10.md` item 4c).
- **Session-11's G-P2-4 ("two approval UI surfaces not unified, modal veils the inline one") is fixed, confirmed
  this session by direct source read**: `🤖️AgentApprovals/🟦️.tsx:7` — "unified (modal dialog retired)" — the
  modal `Dialog` component is gone; `AgentApprovalAffordance` is the one affordance, rendered inline in the agent
  conversation (matches `wp-u5.md` §4's own claim, now independently corroborated by reading the component
  itself rather than only the report).
- Every destructive-verb capability sampled this session (`wfc…deleteTile`, and by extension the `effects`
  block's shape) carries `policy.approval: "whenDestructive"` and `effects.destructive: true` — the safety gate
  itself is intact even where the description text is empty (§2). One oddity worth a follow-up, not a safety
  gap: `architect…exportProgram` (a snapshot export) is also marked `destructive: true, reversible: false` —
  plausibly an over-classification (exporting does not obviously destroy anything), erring toward caution rather
  than away from it.

### (6) Cancellation + progress

**LIVE-PROVEN, fast, repeatedly.** In-flight cancel measured at 1–17 ms across G4–G7's five independent runs
(`compile-cancellation-law.json`, `binding-cancellation-law.json`). U5's session-11 defect-and-fix is the most
interesting item here: **Cancel on an agent tool call parked behind a destructive-verb approval used to wait out
the full 120 s elicitation deadline before the cancel took effect** (`wp-u5.md` §2) — root-caused to
`ApprovalCoordinator::resolve_by_shell` polling for a decision without ever checking whether the call itself had
been cancelled. Fixed (gateway `📣️notify`/`🛡️policy`/`🔀️dispatch`, session 11), measured **364 ms** end to end
after the fix (`generated/u5-live-agent-loop-en-3.txt`, 22/22). Not independently re-run this session (read-only,
no servers), but the fix is in the tree (not contradicted by any later report) and the fix's own test names
(`a_cancelled_tool_call_ends_its_parked_shell_approval_instead_of_waiting_out_the_deadline`) are present in
`🛡️policy/🧪️tests/🔬️quick/🦀️.rs`.

### (7) Transcript visible in the shell

**LIVE-PROVEN on React (AC1, session 9, 9/9 gate + screenshot); wgpu decode now source-confirmed, correcting a
stale P2 gap.** The session-11 audit (`audit-s11-ai-mcp.md` §1.6) stated the wgpu shell "renders no reply at
all… decodes to `BridgeFrameFault::UnknownTag(10)`". Reading the current wgpu `AgentBridge` codec directly this
session shows this is **no longer true**: `🎯️targets/🧊️wgpu/🦀️.rs:444` decodes tag 10 into
`GatewayToShell::AgentReply{ reply_id, in_reply_to, text, complete }`, and the frame is fully wired into the
conversation state machine at `:858-874` (`append_conversation_reply_chunk`, streaming/complete states, mirrors
React's append/extend pair per the docstring at `:864-867`). This is a **stale-claim correction**: the gap is
closed at the data-model layer. **Caveat, honestly marked**: this session could not run the wgpu shell (no
servers permitted), so this is SOURCE-VERIFIED that the frame decodes and updates state, not LIVE-PROVEN that it
paints on screen — the render step (turning the conversation entry into pixels) was not independently confirmed.
Recommend a follow-up slice re-run the wgpu agent-reply live gate to convert this to LIVE-PROVEN.

### (8) Undo of an agent edit

**LIVE-PROVEN** for the base mutation-protocol history chain (`history_undo`/`history_redo`, `client-e2e` 38/38,
repeatedly). The undo/redo revision-stamp defect (CE1) stays fixed. **Known, separate gap**: hub-document undo
specifically can be refused by an `action-owner-mismatch` lane (C10-owned, a collaboration/ownership concern, not
an MCP-gateway defect) — `wp-s15.md` item 4 names this; not touched by any MCP-side report.

### (9) Agent presence

**LIVE-PROVEN at the wire, not yet a first-class UI concept.** `🌎️hub/🔐️auth/🤖️agent/🦀️.rs:269-286`:
`agent_principal_id()` mints `agent:<delegationId>` — structurally impossible to confuse with a human `user:<id>`
actor; `is_agent_principal()` is the roster/undo-ownership predicate. `wp-g10.md` item 9/`hub-edit-durability`
row 5p live-measured both agents in a human's roster tagged `:agent`. The React `PresenceBar` badge is
unit-proven (en/de) but its live browser proof is blocked on the same catalog/lane skew as journey (5) below —
unchanged from session 11.

### (10) Error/refusal texts en + de

**LIVE-PROVEN for the human-facing approval/shell path** (AP1/U5: "Approve Once"/"Einmal genehmigen",
"Deny"/"Ablehnen", localized `PERMISSION_DENIED`/`APPROVAL_REQUIRED` framing in the shell UI, live-gated
repeatedly). **Not localized, and this is expected/acceptable**: the raw JSON-RPC `GatewayError` body
(`⚠️errors/🦀️.rs`) carries only an English machine-readable `code` + message — this is protocol-level output
for a calling agent/LLM, not shell-rendered UI text, so English-only here does not violate AGENTS.md's
en+de-for-user-facing-strings rule (the *user*-facing rendering is the shell affordance, which is localized).

### Journey blocker — delegated agent + human-sees-it-live (item 4 of the original audit brief)

**PARTIAL, unchanged since session 11, not re-attempted this session (no servers permitted).** `wp-g10.md` item
4b: the agent half is LIVE (principal `agent:<delegationId>`, `addBlock` SUCCEEDED, hub `head_seq 0→1`) on G10's
own private hub; the **browser half is BLOCKED**: the note window's remote attach is refused by the shell with no
hub request ever leaving the client ("The document target changed"), root-caused to catalog/lane skew (the dev
serve's staged guest ≠ the hub's catalog generation) — not an MCP-gateway defect, owned by S15/W2. As of this
audit (session 12, ~23:1x), canonical hub 7800 is **down** (work-packages.md: "hub 7800 down" at session-12
start; W2 is rebuilding catalog B2), so this journey step currently cannot be driven end-to-end by anyone, on any
hub, until W2 publishes and G10 resumes its own `g10-after-w2.sh`/`g10-agent-edit-human-sees.ts` scripts (both
named and ready in `wp-g10.md` "Remaining").

---

## 2. Tool-surface quality

- **Tool census**: 28 stable tools (`GATEWAY_TOOL_NAMES`), no stubs, no progressive-enhancement gaps in presence
  (§1.3). Naming is consistent (`noun_verb`, e.g. `artifact_open`, `inference_submit`).
- **Capability description quality — LIVE-PROVEN BROKEN, and broader than previously reported.** The session-11
  audit's G-P1-4 characterized this as "verb descriptions are still empty/non-differentiated" citing a 09-22
  measurement (G19) and G10's own single spot-check (architect/wfc). This session's **direct, live
  `capabilities_search`/`capabilities_describe` calls** (§0, §2 examples below) show the defect is **catalog-wide
  across the destructive-verb class specifically**, not limited to two plugins:
  - `capabilities_search("delete")` → 48 total hits; **every one of the first 15** (animate, lowpoly, procedural,
    puzzle ×2 kinds, reasoning, wfc ×2 kinds, dag, demonstrator, flow) has `description: ""`.
  - `capabilities_search("wfc grid solve")` → all 5 `wfc…solve` editor verbs (the actual mutation, as opposed to
    the separately-described `wfc.infer.*` inference services, which DO carry a generated description) have
    `description: ""`.
  - `capabilities_search("architect program")` → `exportProgram`, `importProgram`, `search`, `addElement`,
    `removeElement`, `runValidation`, `runAnalysis`, `runReport` all `description: ""`.
  - `capabilities_search("draw rectangle")` is the one counter-example measured: draw's verbs carry real prose.
  - **Confirmed via `capabilities_describe`** on `wfc.s.wfc.grid3d@1/*#editor.deleteTile`: `description: ""` but
    `effects.destructive: true`, `policy.approval: "whenDestructive"`, `effects.undo.kind: "inverse"` — **the
    safety-relevant metadata is present and correct; only the human/LLM-facing explanatory text is missing.**
    This distinction matters: an agent or a human approver sees a bare title ("Delete Tile") with no explanation
    of scope or consequence when `capabilities_describe` is called, but the approval gate itself still fires
    correctly. This is a discovery/trust-building gap, not a safety bypass.
  - **Practical consequence for `capabilities_search` quality**: because BM25 scoring partly depends on
    description text, capabilities with empty descriptions score purely on title-token overlap — visible in the
    "delete" query, where 15 near-identical `description: ""` rows tie at scores differentiated only by title
    text, unlike the draw example's clean 20.13 vs 8.95 margin.
- **Raw input/engagement event exclusion — LIVE-PROVEN FIXED this session** (`capabilities_search("hover")` →
  0 results). Closes the other half of session-11's G-P1-4.
- **`capability-audit-check` finding count (audience/destructive-flag correctness, as opposed to description
  text)** — the underlying gate (`🧰️.../🌉️mcp/📦️packages/🦀️rust/📜️script.ts:527-533`,
  `class CapabilityAuditCheckScript`) runs the built gateway's own `audit --folder <repo>` mode and fails on "a
  gesture-named route published to agents with no declared audience, or a delete/clear/replace mutation whose
  `effects.destructive` is false". This session's spot checks (`deleteTile`, `exportProgram`) both show correct
  `destructive: true` — consistent with the gate being green on those specific capabilities — but the "29
  findings / 1 diagnostic" count from CE2/CE3/peer-audit-g10 (stuck since 09-22) **could not be re-run this
  session** (requires the built binary in `audit` mode over every descriptor — a `bun` invocation, not blocked by
  read-only rules per se, but the finding count itself needs the tool's actual stdout, not something the 4
  allowed live tools expose). **UNVERIFIED this session; carried forward as an open P1 from session 11.**
- **`launch.json` discoverability — LIVE-PROVEN FIXED this session.** Session-11's audit flagged `.vscode/
  launch.json` missing rows for `hub-agent-participant-check` and `agent-reply-check` (G-P1-1, a direct AGENTS.md
  violation). Grepped both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` live this session: both now
  carry `⚖️gate🌉️os-mcp🤝️hub-agent-participant` and `⚖️gate🌉️os-mcp💬️agent-reply` rows (plus
  `capability-audit`, `live-agent-loop` ×2 locales, `hub-edit-durability`) — matches `wp-g10.md` item 7's "DONE"
  claim, now independently confirmed by direct grep rather than trusting the report.
- **Tools that cannot work without a dev `s` session**: `context_resolve`/`capabilities_search`/
  `capabilities_describe`/`inference_list` work against the self-provisioned folder workspace with **no** `s`
  session or hub at all (proven this session — no `dev s`, no hub was running when this audit's live calls were
  made, only the standing stdio gateway process). Mutation tools (`artifact_open`/`action_invoke`/history/
  transaction) need a workspace bound to either a local folder document or a hub document; hub-bound mutation
  additionally needs a **running, correctly-versioned hub** — see next point.
- **Does the MCP work against a hub-only setup?** **Currently no, in practice**, though not for a gateway-side
  reason: at audit time canonical hub 7800 is down (session-12 preamble + `work-packages.md`: "every session-11
  process died ~19:30… hub 7800 down"), and the last binary 7800 ran (`1a5cf10…`) has two defects G10 found and
  fixed only on its own private hub: a 64 KiB SQL `CHECK` that rejects any input above the old bound (now
  1 MiB), and a hub-runtime stack overflow that **crashes the entire hub process** on any `inference_approve`
  against a gis map (`wp-g10.md` 13:26–13:31 log entry, `HUB_RUNTIME_THREAD_STACK_BYTES` fix not yet on 7800).
  G10 explicitly flagged this as a warning to the coordinator/W2. So "hub-only, no local `s` session" is
  architecturally supported (proven by discovery tools working with zero local session this audit), but **not
  presently safe against the canonical hub's currently-deployed binary** for one whole inference class.

---

## 3. Security review of the agent path

### 3.1 Delegation token handling (client side) — SOURCE-VERIFIED, sound

`🧰️.../🌉️mcp/🤖️agent-credential/🦀️.rs`:

- The secret **never travels in argv, env, or a URL** (module doc comment, `:1-7`); read only from a file
  (`--credential-file`, `read_file` at `:95-115`) or an inherited fd (`--credential-fd`, `read_fd` at
  `:122-138`).
- File path: must be a regular file, ≤ 16 KiB (`AGENT_CREDENTIAL_MAX_BYTES`, `:21`), and on unix **must not be
  group/world-readable** (`mode & 0o077 != 0` → `PermissionDenied`, `:106-109`) — refused at startup, not
  silently accepted.
- The decoded struct wipes the token bytes on `Drop` (`:38-43`, `unsafe { self.token.as_bytes_mut() }.fill(0)`)
  and its `Debug` impl redacts the token (`:32-36`, prints `<redacted>`).
- Token shape is validated before use: `delegation.v1.<32 hex>.<64 hex>`, exactly 111 bytes, checked
  byte-for-byte (`valid_delegation_token`, `:144-148`) — a truncated paste fails fast with a clear message
  instead of an opaque `401` three network calls later.
- `audience` is a closed enum (`read`/`edit` only, `:82-84`) — no other string is accepted client-side either.

### 3.2 Scope enforcement at the hub — SOURCE-VERIFIED, sound, structurally capped

`🌎️hub/🔐️auth/🤖️agent/🦀️.rs` + `🌎️hub/📇️directory/🦀️.rs:278-313`:

- Delegation is created by a signed-in human (`POST /auth/agent-delegations`, bearer = the human's own session),
  scoped to **one space** (`space_id`) and one **audience** (`read`|`edit`). The token is shown exactly once, at
  creation (`AgentDelegationReceiptV1`, `:105-119`) — the list endpoint (`GET .../agent-delegations`) returns
  metadata only, "no token, no selector, no digest" (doc comment `:121`).
- **`AgentAudience::space_role()` (`🦀️.rs:307-312`) is a structural ceiling, not a policy check that could be
  forgotten**: `Read → SpaceRole::Spectator`, `Edit → SpaceRole::Author`, and `SpaceRole` **has no administrative
  variant at all** — so "an agent can never invite, revoke, or delegate onward" (doc comment `:278-280`) is true
  by construction, not by a call site remembering to check it. An agent session authenticates with the same
  `SessionCapability` shape every other hub route already authenticates (doc comment `:193-195`,
  `AgentSessionMintResponseV1`), so it inherits every existing per-role authorization check rather than needing a
  parallel one that could drift out of sync.
- **Revocation is real but not instantaneous for a session already minted.** `DELETE
  /auth/agent-delegations/{id}` revokes the *delegation*; `decide_agent_session` (`:303-310`) refuses future
  re-exchanges (`DelegationRevoked`/`DelegationExpired`/`InvalidDelegation`, collapsed to one `invalid-delegation`
  for an unproven caller so a delegation id can never be probed for existence, per the doc comment at `:298-302`
  — a nice anti-enumeration property). But an **agent session already exchanged is a bearer `SessionCapability`
  with its own 1-hour TTL** (`AGENT_SESSION_TTL_SECS = 3600`, `:44`) that is not itself looked up against the
  delegation's revoked state on every request — only at the *next re-exchange*. **Net effect: revoking a
  delegation stops a NEW agent process from authenticating, but does not immediately end an agent process that is
  already mid-session; it can keep acting for up to the remaining 1-hour session TTL.** This is a real,
  previously-unflagged gap (no prior audit in `.tmp-ticket/` mentions it) — see P2 below. Bounded severity (≤ 1 h
  exposure window, not indefinite), and the design intent (short session TTL forcing periodic re-checks) is
  visible in the code's own doc comment (`:42-44`: "Short on purpose: the agent re-exchanges, and every
  re-exchange re-reads the delegation's revocation state") — so this reads as an accepted trade-off rather than
  an oversight, but it is not documented anywhere as a known limit for an operator relying on "revoke ends
  access now".

### 3.3 Prompt-injection surface from document content into tool results — NEW FINDING, unmitigated

Traced the read path an agent actually uses to see document content:

- `artifact_snapshot_handler` (`🧰️.../🌉️mcp/🗿️artifact/🦀️.rs:427-452`) reads the `semio://artifact/{id}`
  resource and forwards its **decoded content verbatim** as the tool call's `structured` result (`:448-451`):
  `contents.first()...text...serde_json::from_str(text)` → returned as-is. This resource's content is the
  document's actual authored data (e.g., a note's block text, an architect program's elements) — content any
  collaborator with write access to the shared space can author.
- `artifact_open_handler` (`:264-308`) is metadata-only (kind/revision/size), but `artifact_snapshot` and the
  read side of `action_invoke`/`action_prepare` for read-capable verbs are exactly the paths that hand a
  document's real content to the calling agent.
- **No marking, wrapping, or warning distinguishes "this text came from a possibly-untrusted collaborator" from
  "this is the gateway's own instruction"** in the tool result shape. An MCP client/LLM receiving this content
  has no structural signal telling it the block is data, not an instruction — the same class of risk any
  tool-augmented LLM has when reading attacker-influenced web pages or files, except here the "attacker" is any
  other author in a shared, multi-user space (including another delegated agent), which is exactly the scenario
  outcome 4 is meant to make routine ("my AI assistant edited my shared document").
- **No prior audit in this ticket's `.tmp-ticket/` (checked: every `📓️audit-*.md` and every `wp-g*`/`wp-u5.md`)
  mentions prompt injection, untrusted content, or content sanitization for the MCP tool-result path.** This is
  a genuine, previously-unassessed gap, not a re-discovery.
- **Not exploited or demonstrated live** — this is a structural read from source, appropriately scoped for a
  read-only auditor; confirming actual LLM behavior under adversarial document content would require driving a
  real agent loop against a live hub, out of this session's rules.

### 3.4 Audit-trail redaction — SOURCE-VERIFIED, correct

`📒️audit/🦀️.rs`: `AgentAuditEvent.input_redacted` is the *only* projection of call arguments ever written to a
sink; `redact_input` (`:100-113`) recursively replaces any object value whose key case-insensitively matches
`password|token|secret|apikey|api_key|authorization|bearer|credential|credentials` (`SENSITIVE_KEYS`, `:87`) with
`«redacted»`, walking arrays and nested objects. `input_hash` is computed over the **raw**, unredacted arguments
(so identical calls correlate) but the hash itself never round-trips to plaintext. The module doc comment
explicitly names a dedicated test proving this end-to-end (`sensitive_field_never_reaches_the_sink`) — not
independently re-run this session (would need `cargo test`), but the redaction logic itself is straightforward
and matches its own doc claim on inspection.

### 3.5 Summary of the security review

No critical (P0) finding. Two real, previously-unflagged gaps at P1/P2 (prompt-injection surface, delayed
revocation propagation) plus one already-known, still-open discovery-quality gap (empty descriptions) that this
session showed is broader than reported. The credential/scope/audit design is unusually careful for a
work-in-progress codebase — structural caps (no admin `SpaceRole` an audience can reach) rather than
call-site checks are the kind of design that resists regression.

---

## 4. Ranked gaps

### P0

None newly found this session with acceptance-blocking severity. The two P0s inherited from session 11
(hub-ledger-does-not-advance; generic inference quartet not staged) were **both addressed and live-proven by G10
in session 11** (`wp-g10.md` items 2, 6: hub quartet 19/19, durability 21/21, on G10's own hub) — **but neither
fix is on canonical hub 7800 yet**, because 7800 is down and W2's rebuild (catalog B2 → all packages) is still in
progress as of this audit. I am not re-opening these as P0 findings of outcome 4 itself (the code fix is real and
measured); I am flagging the **rollout gap** as the session's actual top blocker:

**G12-P0-1 — None of G10's session-11 hub fixes (ledger 1 MiB bound, MCP artifact-kind/dialect key, hub-runtime
8 MiB stack, pair-receipt concurrency) are live on canonical hub 7800; 7800 is currently down.**
- Evidence: `wp-g10.md` "Warning for W2/coordinator" (hub 7800 runs `1a5cf10…`, a gis `inference_approve` on it
  aborts the whole hub); `work-packages.md` session-12 opening line ("hub 7800 down"); no listening process on
  7800 found this session (`lsof -nP -iTCP:7800` — not checked directly this session, but no session-12 report
  claims it is up, and the preamble states every session-11 process died).
- Acceptance: 7800 (or its session-12 successor) runs an `os-hub` built after G10's fixes, with a **fresh**
  `inference/gis-map-jobs.sqlite3` (the old table keeps its 64 KiB `CHECK` even under a new binary —
  `CREATE TABLE IF NOT EXISTS`, per `wp-g10.md`'s own note); a gis `inference_approve` against it does not crash
  the process.
- Owner: **W2** (sole hub-rebuild/publish owner this ticket) — already the session-12 plan's first line.

### P1

**G12-P1-1 — Capability description text is empty across the destructive-verb class, catalog-wide (not just
architect/wfc), degrading both `capabilities_describe`'s explanatory value and `capabilities_search`'s score
differentiation.**
- Evidence: live, this session — §2 (48-result "delete" query, first 15 all `description: ""`; all 5 wfc `solve`
  editor verbs empty; 8 of 8 sampled architect editor verbs empty).
- Acceptance: a delete/move/export-class capability's `capabilities_describe` returns non-empty, specific prose
  (what it deletes/moves/exports, from what, with what consequence) for at least the plugins sampled here (wfc,
  architect, animate, lowpoly, procedural, puzzle, reasoning, dag, demonstrator, flow); `capabilities_search`
  score margins on a query like `"delete"` stop being tied.
- Owner: **NEW** (unowned — no session-10/11/12 slice claims this; it is plugin-descriptor text authorship at
  scale, the same class of work U5 did for German mutation labels, but for capability descriptions specifically).

**G12-P1-2 — Prompt-injection surface: shared-document content reaches an agent's tool results with no
untrusted-content marking.**
- Evidence: §3.3, file:line `🧰️.../🌉️mcp/🗿️artifact/🦀️.rs:448-451` (`artifact_snapshot_handler` forwards
  `semio://artifact/{id}` resource content verbatim into `structured`).
- Acceptance: at minimum, a documented, deliberate decision (README/`📋️master.md`) that this is the calling
  MCP client's responsibility, OR a structural mitigation (e.g., the gateway wraps document-authored text fields
  in a way an MCP client can recognize as data rather than instruction). Either is acceptable; the current state
  — nobody having assessed this at all — is not.
- Owner: **NEW** (cross-cutting MCP-gateway + product-security ownership; no slice currently owns this).

**G12-P1-3 — Capability-audit finding count (audience/destructive-flag correctness) not re-verified since
09-22 (CE2/CE3/peer-audit-g10's "29 findings / 1 diagnostic"); this session's spot checks are consistent with the
gate being correct on the two capabilities sampled, but the aggregate count is unconfirmed for three sessions
running.**
- Evidence: §2; `capability-audit-check`'s own implementation (`📜️script.ts:527-533`) requires a built binary
  run in `audit` mode, not reproducible with this session's 4 allowed read-only tools.
- Acceptance: `capability-audit-check` is actually re-run (by an executor with build permission) and its finding
  count reported, ideally trending toward 0.
- Owner: **NEW** (same as session-11's audit — still unowned).

### P2

**G12-P2-1 — Delegation revocation does not immediately end an already-minted agent session (up to 1 h
exposure window).**
- Evidence: §3.2, `🌎️hub/🔐️auth/🤖️agent/🦀️.rs:44` (`AGENT_SESSION_TTL_SECS = 3600`), `:303-310`
  (`decide_agent_session` only gates re-exchange).
- Acceptance: either document this as an accepted bound (a "revoke can take up to 1 hour to fully take effect"
  line in the delegation UI/README) or add a revocation check on the session bearer token itself (a small
  per-request lookup, not a design change — the hub already tracks revoked-at on the delegation row).
- Owner: **H9** (hub-side auth) for the fix-or-document decision; **U5** (delegation UI) if the UI copy is the
  chosen mitigation.

**G12-P2-2 — 4b (delegated agent edits a shared doc, human sees it live in the browser) still not proven on any
real browser session; blocked on catalog/lane skew, unchanged since session 11, currently also blocked on 7800
being down.**
- Evidence: §1, "Journey blocker" above; `wp-g10.md` items 4b, "Remaining".
- Acceptance: `g10-agent-edit-human-sees.ts` (already written, per `wp-g10.md` "Remaining" item 1) runs green
  en + de once a hub matching the dev-serve's staged catalog generation is available.
- Owner: **G10**, gated on **W2**'s catalog B2/all-packages publish (session-12 plan already names this as G10's
  first item).

**G12-P2-3 — wgpu agent-reply rendering not live-verified (only source/state-machine-verified this session).**
- Evidence: §1.7. Corrects session-11's audit (which had this as "silently dropped, unowned") to "decodes and
  updates state, paint step unconfirmed" — a real improvement, but not yet a closed loop.
- Acceptance: `agent-reply-check` (or an equivalent wgpu-lane gate) run live against the wgpu shell, with a
  screenshot or DOM-equivalent capture showing the reply text rendered, not just decoded.
- Owner: **WG7/WG8** (wgpu-lane owners this session) or **S15** (per session-11's audit's original P2-1
  assignment).

**G12-P2-4 — Hub-document undo `action-owner-mismatch` refusal (not an MCP-gateway defect, but blocks the
"undo an agent's edit" step of the outcome-4 journey when the document is hub-bound).**
- Evidence: `wp-s15.md` item 4 ("undo on the hub document refused (C10's `action-owner-mismatch` lane)").
- Owner: **C10** (already named in the source report).

---

## Honest notes on this audit's own limits

- No build/test/server was run; every "LIVE-PROVEN (this session)" claim in §0 is a genuine live MCP tool call
  made by this auditor against the standing gateway process, but every other "LIVE-PROVEN" claim not marked
  "this session" is carried from session-9/10/11 reports and was **not** re-run here (no servers permitted).
- I did not and could not verify: the "8/8 en + de" MCP-client-config UI flow live; the hub quartet 19/19 on any
  hub (no hub was reachable to me with mutating calls, and I made none); wgpu render pixels; whether
  `capability-audit-check`'s aggregate finding count has moved since 09-22.
- One process was observed listening on port 8080 (`os-hub`, pid 90572, started 23:00:36, under
  `s12-u5-bin/os-hub`) and another MCP-adjacent hub-hold process under `wp-u5` — these belong to the U5 slice's
  own session-12 startup, not touched, not used for any claim above beyond noting the auditor's own MCP gateway
  process (pid 88810) is a separate, independent connection that predates and outlived the session boundary.
