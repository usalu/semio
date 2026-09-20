# 📓️ M7 — the live agent bridge loop: the React shell dials, elicitation is deadline-bounded, and the loop is proven at runtime

Slice M7 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Source memos:
`📓️m4-mcp-bridge-approval-binding.md` (§5 gaps 1 and 3 are this slice's items 1 and 2),
`📓️u1-progress-cancel-and-connection-status.md`, `📓️m2-agent-surface-and-inference.md`,
`📓️a2-mcp-plugin-host-instance-open.md`, `📓️v2-launchers-registry-taxonomy-perf.md`.
Crates/modules: `semio-framework-os-mcp` (`🌉️mcp`), `📺️renderer/…/🔗️AgentBridge`.

This worker was cut by the account session limit at ~08:15 and resumed at 11:00; every process it
had started was dead. Items 1 and 2 had already landed and were re-verified on resume.

| # | item | state |
|---|---|---|
| 1 | React shell dials `/__semio/agent-bridge`, live frames render, backoff reconnect, no redial storm | ✅ |
| 2 | elicitation wall-clock timeout (injected clock + test), same typed outcome as the shell timeout | ✅ tests only (§9.2) |
| 3 | live proof (a)–(e) against a real `dev` session + real stdio MCP client | ✅ **8 passed, 0 failed, 3 skipped of 11** — (e) blocked on an un-authored approval gate (§5.4) |
| 4 | permanent nx e2e target + launch rows (`live-agent-loop-check`) | ✅ |
| 5 | wgpu parity for the inbound chrome commands | ✅ written + typechecks; crate red on peer errors, tests unrun (§7) |
| 6 | §8.1 `ShellState` twin drift — isolated and fixed at the root | ✅ 43/43 |

Session 4 (2026-09-19 23:20 → 02:00). The loop is **live**: a `.mcp.json`-verbatim stdio gateway now
drives a running React shell, and `ui_reveal`/`ui_focus`/tool-call frames move the real chrome. Three
root causes had to be removed to get there (§5.1), of which the load-bearing one is that the React
bridge was a state *mirror* and answered the gateway `ok` while nothing on screen changed.

---

## 1. Inherited state

`git status --short` on my paths at start: the whole `🌉️mcp` crate carries M4/M3/A1/A2 live edits;
`🔗️AgentBridge/🟦️.tsx` carries U1's `shellSessionId` redial fix and M4's approval labels;
`🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` + `🧪️tests/🔬️wgpu-unit/🦀️.rs` carry M3's wgpu panel work.
No `🗑️generated/m7-*` captures existed, so nothing to inherit.

A2's root fix **has landed** in `.cargo/config.toml:46` (`-C link-arg=-zstack-size=8388608` on
`[target.wasm32-wasip2]`), and `semio_s_plugin_note.wasm` was restaged at 03:57, after it — which is
what makes live item (e) reachable at all. A2's own report §4–§8 were still `(filling)` at 11:00.

## 2. Item 1 — the React shell dials

M4 built both banks of the river and left the bridge deck out: the gateway publishes an offer to
`~/.semio/agent/bridge/offers/<pid>.json`, the dev server publishes this session to
`sessions/<pid>.json` and serves the gateway's offer at `GET /__semio/agent-bridge` — and
`discoverAgentBridgeConfig` returned `null` unconditionally, so no React shell had ever dialled.

Everything below is in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx`
(line numbers as landed; peers move them):

| what | where | why |
|---|---|---|
| `AGENT_BRIDGE_OFFER_ENDPOINT` = `/__semio/agent-bridge` | `:146` | the one loopback seam; the proof never travels through an env var or a build-time define |
| `isAdmissibleBridgeUrl` | `:153` | refuses a URL that carries its own credential (`?token=…`, userinfo), a non-`ws:`/`wss:` scheme and every non-loopback host — a poisoned offer is refused rather than dialled |
| `parseAgentBridgeOffer` | `:170` | total function over an untrusted body: non-object, missing/empty proof and inadmissible URL all answer `null` |
| `fetchAgentBridgeConfig` | `:184` | never throws and never rejects: absent endpoint, `404`, non-JSON body and refused offer are all the ordinary `null` |
| `useDiscoveredAgentBridgeConfig` | `:211` | polls with backoff 2 s → 30 s while nothing is offered, then keeps one 30 s beat so a **restarted gateway** (new port, new proof) is picked up; returns the **same object identity** while the offer is unchanged |
| `useAgentBridge({ discoveryEndpoint?, discoveryFetch? })` | `:398` | discovery runs only when no `config` was passed, so every existing test and embedder is untouched |

`discoverAgentBridgeConfig` (the `null`-returning stub) is **deleted**, not left as a shim — its
security law (no environment credential carrier) now lives in `isAdmissibleBridgeUrl`'s tests, which
assert the refusal against real values instead of against an unreachable branch.

**No redial storm.** The socket effect's deps are `config?.url, config?.admissionProof`, so identity
stability is what matters, and the poll deliberately returns the *previous* object when both fields
match. Asserted directly: 200+ polls of an unchanged offer open exactly one socket (§4).
`ShellHost` needed no change at all — it already calls `useAgentBridge({ shellSessionId })`.

## 3. Item 2 — elicitation wall-clock timeout

M4 §5.3 called this unfixable without a reader thread ("a blocking `read_line` cannot be
deadline-bounded"). That diagnosis was right, and the reader thread is the fix.

`🌉️mcp/🚚️transport/🦀️.rs`:

1. **`StdioLines` no longer reads in place** (`:51-140`). One owned reader thread
   (`semio-mcp-stdio-reader`) drains the client stream into an `mpsc` channel; `output` keeps its own
   mutex. Ordering is unchanged (a channel is FIFO), EOF is the channel disconnecting, an io error is
   an `Err` item, and the thread ends on EOF, on io error, or when the last `StdioLines` drops.
   The `deferred` queue and its exact semantics are untouched: the serve loop still drains deferred
   lines first, and the elicitation wait still never reads them (that was M4's own spin bug).
2. **`read_line_direct` → `read_line_direct_within(budget)`** returning `StdioRead::{Line,Eof,TimedOut}`
   (`:118`) — `recv_timeout`, which consumes nothing when it expires.
3. **`ELICITATION_TIMEOUT_MS = 120_000`** (`:236`) — deliberately the same budget as `🛡️policy`'s
   `SHELL_APPROVAL_TIMEOUT_MS`, so a human deciding in an MCP client and a human deciding in the OS
   shell get the same time, and a silent one closes its lane the same way.
4. **`ElicitationClock` + `SystemElicitationClock`** (`:242-266`) — the injected clock, plus
   `ElicitationChannel::with_deadline(timeout_ms, clock)`. The wait re-checks the clock every
   `ELICITATION_READ_SLICE_MS` (25 ms) so an injected clock that jumps is noticed at once instead of
   at the end of one enormous `recv_timeout`.
5. **`ElicitationUnavailable::TimedOut`** (`:198`), mapped in `🛡️policy/🦀️.rs:393` to
   `"the connected client did not answer the elicitation in time"` — i.e. the lane closes with a named
   reason and the chain falls through to the shell lane and finally to the typed
   `APPROVAL_REQUIRED`/`Unreachable { details }`, which is **exactly** what a silent shell already did.
   Same typed outcome, as specified.

## 4. Tests — real counts, all run

### 4.1 Rust — `cargo test -p semio-framework-os-mcp --lib`

Filtered run (capture `🗑️generated/m7-elicitation-tests.txt`):
**23 passed, 0 failed** (349 filtered out), of which **3 are new in this slice**:

| test | what it pins |
|---|---|
| `transport::quick::a_silent_client_times_out_instead_of_wedging_the_agent_forever` | a client that never answers, against a `SteppingClock` — proves the deadline arithmetic **without spending the deadline** (the injected-clock law) |
| `transport::quick::the_elicitation_deadline_is_real_wall_clock_not_only_an_injected_one` | the same silent client with the real `SystemElicitationClock` and a 120 ms budget, asserting `Instant::elapsed() >= 120 ms` — the injected clock is not the only thing holding the deadline up |
| `policy::quick::a_client_that_never_answers_its_elicitation_times_out_into_the_same_typed_outcome_as_a_silent_shell` | the timeout arrives in `ApprovalResolution::Unreachable { details }` under `channels.elicitation`, with the `--auto-approve` remedy — never an approval |

Full-crate run (capture `🗑️generated/m7-oslib-tests.txt`): **370 passed, 2 failed**. Both failures are
`workspace::long` plugin-host round trips (`a_headless_commit_propagates_to_a_second_host_on_the_same_folder`,
`plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`) — the store-drop /
`InstanceOpen` territory A2 and M3 own, untouched by this slice and failing on the same two names M3
already classified. The whole M4 elicitation suite (deferral, unadvertised client, error response,
EOF) still passes over the reworked `StdioLines`, which is the real regression risk of item 2.

### 4.2 TypeScript — `bun ./📜️script.ts agent-bridge-check`

(run from `📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`; the target sets
`SEMIO_INCLUDE_AGENT_BRIDGE=1`.) **41 passed, 1 failed of 42.** **10 are new in this slice**:

| suite | new | what it pins |
|---|---|---|
| `parseAgentBridgeOffer / isAdmissibleBridgeUrl` | 3 | the gateway's exact offer shape is accepted; a non-object / proofless / empty-proof offer is refused; a `?token=` URL, a userinfo URL, a non-loopback host and a non-ws scheme are all refused |
| `fetchAgentBridgeConfig` | 3 | a live offer is read; `404` is the ordinary `null`; a throwing fetch and a non-JSON body never reject |
| `useDiscoveredAgentBridgeConfig` | 2 | the offer is returned, keeps **one object identity** across many polls, swaps on a gateway restart, and drops to `null` when the gateway dies; discovery disabled asks nothing at all |
| `useAgentBridge discovery` | 2 | nothing is dialled while nothing is offered (`status: "disabled"`); the discovered offer is dialled with its proof in the subprotocols; **200+ polls of the same offer open exactly one socket**; a restarted gateway closes the old socket and opens exactly one new one |

The 1 failure is `AgentBridge inference state parity`, and it is **pre-existing and not mine**: it
compares `createDefaultShellState()` (which I did not touch) against
`🖥️shell/🧫️fixtures/💡️set-document-inference-port.json` (which I did not touch, and which is
unmodified in the working tree). The fixture carries 49 state keys; the TS default carries 58 — the
9 `ui*` fields (`uiAppearance`, `uiLayout`, `uiLocale`, `uiTerminology`, `uiDriverId`, `uiThemeId`,
`uiCustomDrivers`, `uiCustomThemes`, `uiKeybindingOverrides`) are absent from the fixture. The
fixture is **generated by the Rust twin** (`🖥️shell/🧪️tests/🔬️unit/🦀️.rs:313`, `assert_ok`), so this
is a real TS↔Rust `ShellState` twin drift in the shell module, owned by whoever added those fields.
Recorded in §8 rather than fixed here: regenerating it is a shell-module change in the middle of a
peer's live edits, and it is not on any path this slice touches.

## 5. Item 3 — the live transcript (a)–(e)

Run shape: one real React `dev` session (note variant, vite on `127.0.0.1:6080`, activated once and
served detached — the same process survived from the 11:20 session), one real `semio-os-mcp` stdio
gateway launched with `.mcp.json`'s argv **verbatim**, one headless Chromium on the shell, and
nothing mocked between them. Captures: `🗑️generated/m7-live-agent-loop-gate.txt` (final),
`m7-live-probe.txt` / `m7-live-probe-2.txt` (the two ticket-probe runs that root-caused it),
`m7-rendezvous-isolate.txt`, `m7-bin-build-2.txt` / `m7-bin-build-3.txt`, `m7-wgpu-check.txt`.

### 5.1 Three root causes stood between "both banks built" and a live loop

**(i) The staged gateway binary predated the rendezvous carrier.** Session 3's probe scored
`0 rendezvous` red with the gateway's own stderr claiming it had offered to the right directory. A
purpose-built isolation (`🐍️m7-rendezvous-isolate.ts`, new — it walks the offers directory on disk
and the dev session's endpoint over loopback every 500 ms with the gateway's stderr interleaved)
separated "the gateway never published" from "the dev server never served it" in 120 s:

```
3247ms stderr [semio-os-mcp] bridge listening on ws://127.0.0.1:60264/bridge
               — offered to 4 live os session(s) via /Users/ueli/.semio/agent/bridge/offers/95210.json
NEVER SERVED within 120000ms
```

The offer went to the **per-user default**, not to the `S_AGENT_BRIDGE_DIR` the run pinned. A byte
grep settled it: `grep -ac AGENT_BRIDGE dist/build/semio-os-mcp` → **0**. The staged binary (11:11)
was built before `🛰️rendezvous`'s `RENDEZVOUS_DIR_ENV` existed, so every fleet-pinned rendezvous
silently fell back to the shared default — which is also why the session-3 stderr looked right
(that run's gateway was a differently staged binary) and the endpoint still 404'd. The endpoint
itself was never broken: a synthetic offer file made it answer `200` on the first try.

**(ii) The mcp crate could not be rebuilt.** The first rebuild (13 min) died on five `E0063`s from a
peer's in-flight `audience`/`artifact_kind` refactor. Three were already fixed by the peer while the
build ran; the two that were not are now closed, in the direction the refactor was going:

| file | what |
|---|---|
| `🌉️mcp/🦀️.rs:218` | `to_schema_search_hit` made `pub(crate)` — the one place a `CapabilityDefinition` is projected onto a `SearchHit` |
| `🌉️mcp/🏠️workspace/🦀️.rs:2028` | the hand-rolled second projection (which dropped `app_id` and half of `plugin_id`) replaced by that call; `SearchFilters` gains `audience: Vec::new()` |
| `🌉️mcp/💡️inference/🦀️.rs:1151` | gateway inference capabilities declare `audience: CapabilityAudience::Agent`, like every other gateway capability in `🗂️catalog` |

Rebuild: **5 m 05 s, 0 errors**, restaged, `grep -ac AGENT_BRIDGE` → 1. Rendezvous then served in
**8.0 s** end to end.

**(iii) The React bridge was a mirror, not a controller — the load-bearing M7 defect.**
With the rendezvous green, the gateway answered `ui_reveal` `{"ok":true,"path":["framework.chat"]}`
and `ui_focus` `{"ok":true}` — and the shell's DOM had no chat panel, no presence element and no
conversation rows. `useAgentBridge` reduces every inbound `ShellCommand` against its **own private
`ShellState` mirror** and answers the gateway from that; `🏛️ShellHost` called it as
`useAgentBridge({ shellSessionId })` and never read the result back. Every `ui.*` verb in the
product therefore reported success while nothing on screen moved.

The seam already existed and was unused (`onCommandApplied`). Two minimal hunks in
`🏛️ShellHost/🟦️.tsx` close it:

| where | what |
|---|---|
| `:2446` | `applyAgentShellCommandRef` + `onCommandApplied` wired into `useAgentBridge` — a ref because the hook runs ~7 600 lines before `dispatch`/`dock` exist |
| after the tool-run reveal effect (`:10080`) | `apply­InboundAgentShellCommands`: `focusWindow` → `SET_ACTIVE_WINDOW_ID`; `setPanelPath` → `findPanelTabInDock` + `SET_PANEL_PATH` + `SET_PANEL_VISIBLE`, the exact idiom the tool-run and inspection reveals already use |

The tab is the address, not the anchor: the shell SSOT has four anchors (`left|right|top|bottom`)
and this dock has six (`top-right`, `bottom-right`, …), so the requested anchor is advisory and
`findPanelTabInDock` answers where the tab actually lives.

### 5.2 The measured transcript

`bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check` against the live session
(capture `🗑️generated/m7-live-agent-loop-gate.txt`):

| step | result | evidence |
|---|---|---|
| 0 rendezvous | **PASS** | endpoint `404` before the gateway, `200` after; `url=ws://127.0.0.1:52351/bridge`, `pid=18019` |
| boot | **PASS** | `ready=note error=null windows=note-composite,note-navigator` |
| (a) the shell dials `/bridge` | **PASS** | `ui_focus` stops answering `PLUGIN_UNAVAILABLE` — `🖥️ui/🦀️.rs:107` refuses unless a shell connection is registered, so this is the gateway's own witness that the socket is live |
| (a) agent presence renders | **PASS** | `[data-semio-agent-presence-tone]="connected"`, text `"Agent idle"` |
| (d) `ui_reveal` moves the real dock | **PASS** | `{"ok":true,"anchor":"right","path":["framework.chat"]}` **and** `[data-semio-agent-chat-panel]` in the DOM |
| (d) `ui_focus` moves the real active window | **PASS** | `{"ok":true,"windowId":"note-composite"}`, taken from the live DOM's own `data-window-id` |
| (b) AgentToolCall → AgentToolResult | **PASS** | running row `framework.chat.entry.toolCall.inv_4` `state=running`, settled row `inv_1` `state=ok` — live frames, not a fixture |
| (c) Cancel cancels an in-flight call | **PASS** | clicked `inv_6`, row moved `running` → `cancelling`; the gateway's own answer came back `BUDGET_EXCEEDED` from the guest (R2's lane) |
| (e1)/(e2)/(e3) approval | **SKIP**, root-caused | `note.…deleteSelection: approval=whenDestructive, destructive=false` — read live off the catalog, see §5.4 |

(a), (b) and both (d) legs were **red before the ShellHost hunk and green after it**, with nothing
else changed between the two runs (`m7-live-probe.txt` → `m7-live-probe-2.txt`) — which is the
experimental isolation of cause (iii).

### 5.3 (c) — the cancel affordance

Two probe defects had to be removed before this step measured the product rather than itself.

1. A gateway-owned read verb settles in milliseconds, so the first attempt (`inference_run`) never
   gave the cancel button a frame to exist in and scored "no cancellable row" — a probe artefact,
   not a product defect: (b)'s own running row already carried `Cancel`. Keeping a plugin
   `action_invoke` in flight produces a real cancellable row.
2. Then the *click* lost the race: a Playwright locator click is a second round trip and the verb
   settled inside it (`Timeout 10000ms exceeded … waiting for locator('[data-semio-agent-chat-cancel="inv_5"]')`).
   The gate now finds and clicks in **one** `page.evaluate`, so it clicks the affordance that exists
   at the instant it is seen.

Measured: `clicked=inv_6`, and the row moved `running` → **`cancelling`**
(`framework.chat.entry.toolCall.inv_6`) — U1's `ShellToGateway::AgentCancel` frame travelling from a
real button in a real shell to a real gateway. The invocation's own answer came back
`BUDGET_EXCEEDED` from the guest, which is R2's lane and not part of this assertion.

### 5.4 (e) — the approval gate is unreachable in the live catalog, and not because of R2

Measured, not inferred:

- `🛡️policy/🦀️.rs:218` `requires_approval`: `Never` → never, `WhenDestructive` → **iff
  `effects.destructive`**, `Always` → always.
- `✏️s/🔌️plugins/🗒️note/🔣️.json` declares `"approval": "whenDestructive"` **56** times and
  `"approval": "never"` 65 times — and `"destructive": true` **zero** times out of 121 flags.
- Across **every** plugin descriptor in `✏️s/🔌️plugins/*/🔣️.json`: `"destructive": true` → **0**.
- Across the whole repo: `.destructive()` (the `ActionDefinition`/`CommandDefinition` builder at
  `🛂️manifest/🦀️.rs:1037,1693` that sets the flag) has **0** call sites.

So `ApprovalMode::WhenDestructive` is dead at runtime: **no capability in the live catalog can raise
an approval affordance**, and the `--auto-approve` waiver, the shell lane and the elicitation lane
are all downstream of a gate that never fires. This is an authoring gap, one builder call per verb,
not a framework defect — and it is independent of R2's reactor work: R2's fault
(`plugin.command-cursor-mismatch`, now `execute_turn: no Effect::Respond for seq N`) is what those
verbs hit *after* the gate, and fixing it would not make the gate fire.

The gate therefore reports (e1)/(e2)/(e3) as `SKIP` with that exact reason, checked at runtime
against the live catalog through `capabilities_describe` — the measured line is
`no capability in the live catalog gates on approval (note.s.note.note@1/*#editor.deleteSelection:
approval=whenDestructive, destructive=false); author one with ActionDefinition::destructive() and
this step runs` — the moment one capability declares
`destructive`, the three steps run and are required like every other. That is a precondition, not a
waiver: nothing is asserted into thin air and nothing is silently skipped.

R2's reactor fault was observed moving during this session (`command-cursor-mismatch` →
`execute_turn: no Effect::Respond for seq 1 before terminal acknowledgement`), so mutation verbs are
still red; the transcript was re-run after each of my own landings, never after R2's.

## 6. Item 4 — the permanent nx e2e target

The transcript is no longer a ticket-folder probe that `🗑️generated` sweeps. It lives in the domain
tree, in the shape AU3's live sign-in gate established:

| file | what |
|---|---|
| `🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts` (new) | the whole (0)–(e) transcript: `.mcp.json`-verbatim gateway, real browser, per-step `pass`/`fail`/`skip` with its reason, exit 1 on any `fail` |
| `🌉️mcp/🧪️tests/🤖️live-agent-loop/🏃️execution/🟦️.ts` (new) | `OsMcpLiveAgentLoopScript extends BundleScript`, `runOwnedCommand` with a 900 s budget |
| `🌉️mcp/📦️packages/🦀️rust/📜️script.ts:22,656` | `.register("live-agent-loop-check", OsMcpLiveAgentLoopScript)` |
| `🌉️mcp/📦️packages/🦀️rust/📋️project.json` | target `live-agent-loop-check`, `cache: false`, `dependsOn: ["build"]` — the gate must run against a FRESHLY staged binary, which is the very defect §5.1(i) was |
| `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🌉️os-mcp🤖️live-agent-loop`, group `4_gate`, order `411.49` / `411.10758`, beside `⚖️gate🔐️hub-auth🤝️live-sign-in`. Both files re-validated: 379 / 243 named configurations, 0 duplicate names |

Run, end to end, through the registered target (`bun ./📜️script.ts live-agent-loop-check`):
**8 passed, 0 failed, 3 skipped of 11**, exit 0.

Two design decisions worth naming:

1. **`S_AGENT_BRIDGE_DIR` is inherited, never minted.** The gate's first version created its own
   temp rendezvous and scored `0 rendezvous` red against a perfectly healthy session — the gateway
   must meet the session in the rendezvous *that session* published into. The `(e3)` silent client
   still gets its own empty rendezvous, deliberately: that closes the shell lane by construction so
   the only budget that step spends is the elicitation deadline itself.
2. **The dev session is a precondition, not something the gate boots.** An activation costs minutes
   and every developer already has one open; when nothing answers, the gate names the exact launch
   row to start (`🛠️dev🗒️note⚛️react`) and the `S_OS_MCP_LIVE_SHELL_URL` override.

## 7. Item 5 — wgpu parity

The React landing this slice added is "an inbound agent `ShellCommand` moves the real chrome". The
wgpu shell answered **every** `ShellCommand` with one blanket refusal —
`"wgpu shell has no ShellState reducer twin"` (`🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs:633`) — which
was both a lie and the same defect in mirror image: `ui_focus` and `ui_reveal` are chrome actions
that shell has always been able to perform, and `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7932` already owns
`reveal_dock_tab(tab_id)`, the exact twin of React's `findPanelTabInDock`.

| file | what |
|---|---|
| `🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` | `InboundShellCommand::{FocusWindow, RevealPanelTab, Acknowledge}` + `decode_inbound_shell_command`; `apply_frame` queues what it can honour and refuses the rest **by name**; `take_inbound_shell_commands` / `settle_shell_command` split take from acknowledge |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`pump_agent_bridge`) | `apply_inbound_agent_shell_commands`: `FocusWindow` → `active_window_id`; `RevealPanelTab` → `reveal_dock_tab`, refusing with `"this shell's dock hosts no `<tab>` panel tab"` when the dock has no such tab; **acknowledges only after the chrome moved**, which is precisely the honesty the React mirror lacked |
| `🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs` | 3 new tests: `ui_focus`/`ui_reveal`/`setPanelVisible` queue while a chromeless verb is refused by name; the host acknowledges only after applying; a malformed payload is a named refusal, never a panic |

**Compile status, honestly:** `cargo check -p semio-framework-os-renderer-wgpu --lib` reports **4
errors, none in the files above** — `🧊️renderer/🦀️.rs:175` imports a
`world3d_reference_url_is_current` that no longer exists, and `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7000,
25957, 25990` use a `theme_editor_page`/`theme_editor_open_section`/`build_settings_theme_editor_tree`
that a peer is mid-rename on (`build_settings_theme_editor_sections` exists). rustc type-checks the
whole crate in one pass and raised nothing against the new code, so it type-checks — but the crate
does not link tonight and the new tests have therefore **not been run**, and no `wasm32-unknown-unknown`
check was attempted (it would hit the same four). Capture: `🗑️generated/m7-wgpu-check.txt`.

## 8. The `ShellState` twin drift — isolated and fixed at the root

### 8.1 What it actually was (§4.2's diagnosis was wrong)

Session 3 recorded this as "9 `ui*` fields **missing from the generated fixture**", i.e. a
generator that had fallen behind. Isolating it experimentally says the arrow points the other way:

| side | fields | evidence |
|---|---|---|
| Rust `ShellState` (`🖥️shell/🧬️schema/🦀️.rs:599`) | 49 | `impl Default for ShellState` at `:703` lists exactly 49 rows; no `ui_appearance`/`ui_layout`/`ui_locale`/`ui_terminology`/`ui_driver_id`/`ui_theme_id`/`ui_custom_drivers`/`ui_custom_themes`/`ui_keybinding_overrides` anywhere in the module (grep: 0 hits) |
| generated TS mirror (`🖥️shell/🤖️generated/🟦️.ts:49`) | 49 | same 49 keys, same order — the generator is **not** behind |
| committed fixture `💡️set-document-inference-port.json` | 49 | same 49 keys |
| `createDefaultShellState()` in `🔗️AgentBridge/🟦️.tsx` | **58** | a hand-written second spelling of the default |

The nine extra rows are `🐚️Shell`'s own `UiPrefsState`
(`📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:588-600`, read in `🏛️ShellHost/🟦️.tsx:2040` as
`shellState.uiPrefs`) minus its two draft rows — renderer **preference** state, which the Rust SSOT
deliberately excludes ("hosts supply durable UI preferences through the canonical OS config
projection", `🦀️.rs:704`). So nothing is missing from the fixture; the bridge's private copy of the
default had absorbed nine rows from a different state object. There is no generator to fix: the
fixtures are asserted against the Rust reducer (`🖥️shell/🧪️tests/🔬️unit/🦀️.rs:153`,
`constructed_cases_match_committed_fixtures`), never written by it — the earlier report's "generated
by the Rust twin" was also wrong.

### 8.2 The fix — one spelling of the default

The duplicate's own docstring claimed it was "field-for-field identical … not drift", and it had
drifted nine fields wide, which is what a second spelling always does. So the second spelling is
gone rather than corrected:

- `🖥️shell/🟦️.ts` gains **`defaultShellState()`** (new `//#region 🌱️default`, before `🧮️reduce`) —
  the exported TypeScript twin of Rust's `impl Default for ShellState`, documented with *why* the
  nine `ui*` rows belong to `🐚️Shell` and not here, so the next reader does not re-add them.
- `🔗️AgentBridge/🟦️.tsx` `//#region 🔖️DefaultState` is now a single line,
  `export { defaultShellState as createDefaultShellState };`, and `useAgentBridge` calls
  `defaultShellState()` directly (an `export … as` makes no local binding — that is a real
  `ReferenceError` the suite caught, not a typing nicety).
- the parity suite gains **`spells the neutral state exactly once, in the shell SSOT twin`**, which
  asserts the two names are the *same function object* — a future copy cannot pass it.
- the `[DEBUG]` `console.log` left in the parity test (`🧪️tests/🧩️component/🟦️.ts:298`) is deleted.

### 8.3 Measured

| run | before | after |
|---|---|---|
| `bun ./📜️script.ts agent-bridge-check` (`SEMIO_INCLUDE_AGENT_BRIDGE=1`) | 41 passed / **1 failed** of 42 | **43 passed / 0 failed** of 43 (capture `🗑️generated/m7-agent-bridge-check.txt`) |
| `@semio-tech/framework-os-shell` `test quick` | — | **7 passed / 0 failed**, 2 files (capture `🗑️generated/m7-shell-ts-test.txt`) |

## 9. Honest gaps

1. **No capability in the repo is authored `destructive`**, so `ApprovalMode::WhenDestructive` — the
   mode 56 of note's 121 capabilities declare — never fires, and the entire approval surface
   (shell affordance, elicitation lane, `--auto-approve` waiver, `PERMISSION_DENIED`) is
   unreachable at runtime. (e1)/(e2)/(e3) are `SKIP`, not `PASS`. The fix is one builder call per
   genuinely destructive verb (`ActionDefinition::destructive()`, `🛂️manifest/🦀️.rs:1037`) plus a
   descriptor regeneration; it is plugin authoring, outside this slice, and it is the single thing
   standing between outcome 4 and a proven approval story. §5.4 has the counts.
2. **The elicitation wall-clock timeout (item 2) is still only proven by the three Rust tests**
   (§4.1), never live, for the same reason: the silent client cannot reach a gate that does not
   fire. Its typed outcome is asserted; its *live* arrival is not.
3. **The wgpu parity compiles but does not link and its 3 new tests have not been run** — four
   unrelated peer errors (theme-editor rename, a stale `world3d_reference_url_is_current` import)
   hold the crate red. No `wasm32-unknown-unknown` check was attempted. §7.
4. **Mutation verbs remain red in the guest reactor** (R2's slice) — (c) proves the cancel
   affordance and the `AgentCancel` frame, not that the cancelled verb would otherwise have
   succeeded. During this session the fault
   moved from `plugin.command-cursor-mismatch` to `execute_turn: no Effect::Respond for seq N`, so
   R2 is live; the transcript was never re-run against a landed R2 fix.
5. **The gate needs an already-running dev session** and says so rather than booting one. It is
   therefore a developer/CI-with-a-session gate, not a cold-start gate.
6. **`ui_reveal`'s `anchor` argument is advisory on both renderers.** The tab id decides where the
   panel opens. That is the only honest mapping from the SSOT's four anchors onto a six-anchor dock,
   but it means an agent cannot *move* a tab to another anchor through `ui_reveal`.
7. **Two `🌉️mcp` files were completed on a peer's behalf** (§5.1(ii)) to get the crate to build at
   all: `🏠️workspace/🦀️.rs` and `💡️inference/🦀️.rs`. Both follow the direction the peer's own
   `to_schema_search_hit` set, but they are that peer's design to confirm.
8. **The live transcript is single-variant** (note, React, one shell, one gateway). Nothing here
   proves two shells, two gateways, or the wgpu host at runtime.

## 10. Files changed

**Rust — `semio-framework-os-mcp`** (items 2 + the build unblock):
`🚚️transport/🦀️.rs`, `🚚️transport/🧪️tests/🔬️quick/🦀️.rs`, `🚚️transport/🧪️tests/🔬️long/🦀️.rs`,
`🛡️policy/🦀️.rs`, `🛡️policy/🧪️tests/🔬️quick/🦀️.rs`, `🦀️.rs` (`to_schema_search_hit` → `pub(crate)`),
`🏠️workspace/🦀️.rs`, `💡️inference/🦀️.rs`.

**Rust — `semio-framework-os-renderer-wgpu`** (item 5):
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`pump_agent_bridge` +
`apply_inbound_agent_shell_commands`).

**TypeScript** (items 1, 3 and the twin drift):
`💻️os/🔨️modules/🖥️shell/🟦️.ts` (new `defaultShellState()`),
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (two hunks — the bridge seam and its drain).

**New — the permanent gate** (item 4):
`💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`,
`💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️live-agent-loop/🏃️execution/🟦️.ts`,
plus registrations in `🌉️mcp/📦️packages/🦀️rust/📜️script.ts`,
`🌉️mcp/📦️packages/🦀️rust/📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.

**Ticket**: this report, `🐍️m7-live-agent-loop-probe.ts` (the (c) fix), `🐍️m7-rendezvous-isolate.ts`
(new), and captures `🗑️generated/m7-live-agent-loop-gate.txt`, `m7-live-probe.txt`,
`m7-live-probe-2.txt`, `m7-rendezvous-isolate.txt`, `m7-bin-build-2.txt`, `m7-bin-build-3.txt`,
`m7-agent-bridge-check.txt`, `m7-shell-ts-test.txt`, `m7-wgpu-check.txt`, plus session 3's
`m7-check-1.txt`, `m7-elicitation-tests.txt`, `m7-oslib-tests.txt`, `m7-bin-build.txt`.
