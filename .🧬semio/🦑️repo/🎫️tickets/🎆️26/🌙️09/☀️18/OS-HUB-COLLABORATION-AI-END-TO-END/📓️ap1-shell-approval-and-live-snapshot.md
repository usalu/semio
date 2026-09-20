# 📓️ AP1 — shell approval affordance + the live snapshot law

Slice AP1 of ticket 26/09/18. Spec: `📓️m5a-mcp-catalog-agent-usability.md` §8.12–§8.15,
`📓️m4-mcp-bridge-approval-binding.md`, `📓️lb1-live-bridge-action-routing.md` §10–§11,
`📓️m7-live-agent-bridge-loop.md`, `📓️u1-progress-cancel-and-connection-status.md`.

Slice: close `@semio-tech/framework-os-mcp-rs:live-agent-loop-check` except `(f8)` export (WR3 owns
the media OUT port).

## Status

**Done.** The gate moved from **14 pass / 4 fail / 1 skip of 19** to **16 / 2 / 1**; the two
remaining failures are `(f8)` (WR3) and `(e3)` (R2). Section 5 carries every number.

## 1. Inherited state — the gate reproduced before anything was touched

M5a §8.13 reported **15 pass / 3 fail / 1 skip of 19** on `:6080`. Reproduced first, unchanged tree:

```
S_OS_MCP_LIVE_SHELL_URL=http://127.0.0.1:6080 S_OS_MCP_LIVE_PLUGIN=note \
S_AGENT_BRIDGE_DIR=<ticket>/🗑️generated/m7-rendezvous \
bun ./📜️script.ts live-agent-loop-check      # in 🌉️mcp/📦️packages/🦀️rust
```

**14 passed / 4 failed / 1 skipped of 19** (`🗑️generated/ap1-gate-baseline.txt`). The four:
`(f3)`, `(f8)`, `(e3)`, and `(b)` — `(b)`'s running row settled before the poll saw it (`running=null
settled=state:"ok"`), a probe race, not a regression; M5a saw it pass.

Two facts the baseline settles that the two source memos disagreed about:

1. **The route is the SHELL route, not the headless one.** `context_resolve.channel = "shell"`
   (`🗑️generated/ap1-shell-route-probe.txt`), and every revision stamp reads
   `note:s.note.note@1/*#editor:1` — `${pluginId}:${appId}:${instanceId}`, which only
   `🏛️ShellHost`'s `agentArtifactRouteRef` mints. LB1 §10.3's 6/9/4 was `:6070`, whose Home never
   boots an editor; `:6080` boots one, so the same code answers differently. §4 makes that
   explicit instead of leaving it to which serve is up.
2. **`:6080` needs `S_AGENT_BRIDGE_DIR`.** Its vite rendezvous plugin was started against
   `🗑️generated/m7-rendezvous`, so a gateway publishing into the per-user default is invisible to
   it and `/__semio/agent-bridge` answers 404. With the variable set, step 0 passes.

## 2. `(f3)` — the head does not move, and the verb is the reason

`(f3)` asserts `status == SUCCEEDED && revisionAfter != revisionBefore`. Measured:

```
FAIL (f3) status=SUCCEEDED before={cursor:"0",headEditId:""} after={cursor:"0",headEditId:""}
```

The gate drives the **whole (f) chain with the destructive verb it found for (e)** —
`note…editor.deleteSelection` — with `input: {}`. That verb's own handler
(`✏️s/🔌️plugins/🗒️note/…/🎮️commands/🚫️delete-selection/🦀️.rs:20`) is:

```rust
if ctx.selected_block_ids.is_empty() { return Ok(Emit::default()); }
```

Nothing is selected in a freshly created document, so it emits nothing, the store has nothing to
apply, and the head correctly stays put. **`(f3)` could never have passed as written** — it was not
measuring the shell route, it was measuring a no-op.

Driving an unconditional mutation through the same gateway (`🐍️ap1-shell-route-probe.ts`,
`addBlock` with `{"kind":"text","x":40,"y":40}`) proves the route itself is sound:

```
action_invoke :: revisionBefore {cursor "0", headEditId ""}
                 revisionAfter  {cursor "1", headEditId "apply"}   status SUCCEEDED
```

**measured.** So the shell route dispatches, the store applies, and the history read reflects it.

## 3. The real defect the probe found: `artifact_snapshot` is frozen at create time

The same probe run, on either side of that head move, answered a **byte-identical document**
(`packBytes=515 sprBytes=223` both times, identical `packBase64`) — across a mutation the same call
chain had just reported as applied. Root cause, read off the code rather than guessed:

- `🏠️workspace/🦀️.rs` `create_plugin_artifact` seeds the artifact with **one** `ReadArtifact` over
  the session channel and writes those bytes into the `--folder` event log.
- `artifact_snapshot` → `read_artifact_resource` → **`read_artifact_bytes`**, which knew only two
  sources: this crate's own in-memory `ProbeStore`, and that persisted folder row. It never asked
  the session's channel.

So on the shell route every snapshot of a session-owned artifact answers the document **as it was
when the agent opened it**, forever. This is the third of the brief's three hypotheses — *"from the
headless workspace instead of the session's channel"* — confirmed, not inferred.

### The law landed

`🏠️workspace/🦀️.rs`: `read_artifact_bytes` gained a middle lane between the probe store and the
folder row, and `read_session_artifact_bytes` implements it — when this session resolved `shell`
**and** the artifact carries a `PluginArtifactBinding` this gateway minted, the bytes are re-read
from the owner over `AppCommand::ReadArtifact`. A headless session (`Ok(None)`) still reads the
folder row, which is still the truth there. A shell that refuses answers **by name** as
`SIDE_EFFECT_REJECTED` carrying the shell's own fault text — never a silent fall back to the frozen
row, which is the stale answer the lane exists to kill.

It fixes `artifact_export` in the same hunk: that verb also fed `read_artifact_bytes`' frozen pack
into the guest's `LoadDocument`.

### The witness, and a trap worth naming

After the fix the probe still showed an unchanged `packBase64` — and that is CORRECT. A note is
event-sourced: `pack` is the genesis container and every edit lands in the **`spr` sidecar**. The
live read moved `sprBytes` **223 → 612** while `packBytes` stayed 515
(`🗑️generated/ap1-shell-route-probe5.txt`), and `artifact_open`'s `sizeBytes` (pack + spr) moved
**738 → 1127** in the same run. A witness that watches only the pack calls a real edit "no change",
which is the mistake the first draft of `(f4)` made and why the gate's assertion is now
`packBytes:sprBytes:packBase64`.

An honest gap in the tool itself, found on the way: `artifact_snapshot` publishes `packBase64` but
NOT `sprBase64`, so an agent can see that the document changed (`sprBytes`) without being able to
read what changed. Naming it here; it is a one-field addition to
`artifact_snapshot_output_schema` and out of this slice's hunks.

## 4. `(e3)` / the shell approval affordance — why no affordance was ever rendered

M5a §8.13 recorded *"the SHELL approval lane does not render a gateway-initiated approval at all
yet"*. Measured here, that is **not** what is wrong. Both `(e1)` and `(e2)` pass with
`channel=elicitation affordance=false`: `ApprovalCoordinator::resolve` offers **elicitation first**
(`🛡️policy/🦀️.rs`), and the gate's own client advertises `capabilities.elicitation`, so the
client's human settles every approval and the shell is never asked. Forcing the shell lane by
staying silent does not work either — `ELICITATION_TIMEOUT_MS` is **120 s**, and M5a waited 30 s for
an affordance that could not be published until that deadline expired.

The lane a real client actually takes — **a client with no elicitation capability** — was never
exercised by any step of this gate. §5 adds it.

What the shell renders on that lane was genuinely thin, and that part of M5a's finding stands: the
chat panel printed `entry.summary` **raw**, i.e. the gateway's JSON blob, and neither surface showed
who asked, what the verb does, what it applies to, or how long the human has.

### Landed

The frame's wire shape is unchanged (tag 3 carries one string); **that string's payload is the
schema**, and it now has one shared fixture and two parsers asserted against it:

| bank | parser | test |
| --- | --- | --- |
| React | `🤖️AgentApprovals/🟦️.tsx` `parseApprovalSummary` | `🧪️tests/🧩️component/🟦️.tsx` |
| wgpu | `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` `parse_approval_summary` | `🧪️tests/🔬️wgpu-unit/🦀️.rs` |

Fixture: `🤖️AgentApprovals/🧫️fixtures/🛡️summary/🔣️.json` — a rich row, a plain-text row, a
**legacy row without the new fields** (so an older producer still parses), and a non-positive
timeout that must refuse to count.

Producer: `🛡️policy/🦀️.rs` `ApprovalRequest::shell_summary(timeout_ms)` now carries
`capabilityTitle`, `description` (the verb's own published sentence — the one the agent found it
by), `artifactKind`, and `timeoutMs`. **`timeoutMs` is a duration, never a deadline**: the shell
counts from the frame's arrival, so a browser clock that disagrees with the gateway's cannot make an
approval look already-expired.

Surfaces: `💬️AgentChatPanel` renders verb / description / target / change summary / requested-by and
a `role="status" aria-live="polite"` countdown (`data-semio-agent-chat-approval-countdown`); the
decision group keeps its `role="group"` + `aria-label` and its three `Button`s, so it stays keyboard
and screen-reader reachable, and every row is `min-w-0 break-words` inside the existing
`flex-wrap` column, which is what makes it survive phone width. `🤖️AgentApprovals`' dialog gained
the same fields and the same countdown.

## 5. Measured

### 5.1 The gate — `@semio-tech/framework-os-mcp-rs:live-agent-loop-check`

Same one line, same serve, before and after (`🗑️generated/ap1-gate-baseline.txt`,
`🗑️generated/ap1-gate-final.txt`):

| | baseline (this tree, untouched) | after |
| --- | --- | --- |
| total | **14 pass / 4 fail / 1 skip of 19** | **16 pass / 2 fail / 1 skip of 19** |
| `(b)` running row | FAIL (`running=null`) | **PASS** — observed, not polled |
| `(f3)` head moves | FAIL (`cursor 0 → 0`) | **PASS** — `cursor 0 → 1`, `headEditId "apply"` |
| `(f4)` snapshot shows it | PASS on `packBytes > 0` alone | **PASS on a real witness** — `515/223 → 515/612` |
| `(e1)` Approve Once | PASS over `elicitation`, `affordance=false` | **PASS over the SHELL** — `affordance=dialog countdown=120` |
| `(e2)` Deny | PASS over `elicitation` | **PASS over the SHELL** — `channel:"shell"`, `PERMISSION_DENIED` |
| `(f8)` export | FAIL | FAIL — **WR3's**, excluded from this slice by the brief |
| `(e3)` silent-client timeout | FAIL | FAIL — **R2's**, unchanged (§6) |
| `(f5)` | SKIP | SKIP — unchanged (§6) |

The two remaining failures are both named owners outside this slice, so the slice's own bar —
everything except `(f8)` — is met.

### 5.2 Tests

| what | result | capture |
| --- | --- | --- |
| `cargo check -p semio-framework-os-mcp --all-targets` | **rc=0**, 0 errors, 87 warnings (warnings are the proof expansion ran) | `ap1-cargo-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --all-targets` | 1 error, **not in any file this slice touched** — a peer's `🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d` reads `InputState::pending_actions`, now private. 185 warnings emitted from the same `lib test` target, so this slice's `🤖️AgentApprovals` wgpu edits did type-check | `ap1-wgpu-check.txt` |
| `🤖️AgentApprovals` + `💬️AgentChatPanel` component suites | **19 passed / 19**, exit 0 | `ap1-vitest-approvals.txt` |
| `bun ./📜️script.ts build` (gateway binary) | rc=0, 1 m 44 s | `ap1-mcp-build.txt` |

`🤖️AgentApprovals`' component suite **was in no runner's include list** — the exact blind gate the
react vitest config's own comment warns about. It is now registered next to
`💬️AgentChatPanel`'s, which is why its four new fixture rows actually run.

### 5.3 Gate determinism (brief item 3)

Three things now decide which shell the gate is talking about, instead of whichever serve was up:

1. **Refused before step 0** — the serve must answer `SHELL_URL`, and its transformed `🏛️ShellHost`
   must contain `agentArtifactRouteRef` (LB1 §11.2 precondition 2, now enforced rather than
   documented). This is what made `:6070` and `:6080` produce two different verdicts.
2. **Step 0 asserts ownership** — `/__semio/agent-bridge` serves the newest LIVE offer, so a dead
   gateway's file or a peer's running one is served in preference to nothing. Step 0 now reads the
   offer in full before starting the gateway and passes only once the `pid` has CHANGED, and its
   failure text names the remedy (`S_AGENT_BRIDGE_DIR`).
3. **The two flaky reads are observed, not polled** — `(b)`'s transient running row is collected by
   an in-page `MutationObserver` installed before the call, and `(e)` decides only an approval id
   that was NOT on screen before its own invocation (step `(c)` cancels its call while the gateway
   still waits for a human, so its row outlives it and was being clicked instead — that is what
   produced `countdown=0` and a hung `APPROVAL_REQUIRED` mid-slice).

**The recipe that reproduces every number above:**

```
cd "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" &&   S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:6080" S_OS_MCP_LIVE_PLUGIN=note   S_AGENT_BRIDGE_DIR="<ticket>/🗑️generated/m7-rendezvous"   bun ./📜️script.ts live-agent-loop-check
```

`:6080` is the live `note` React serve (vite pid 49007), started against that rendezvous directory —
which is why the per-user default makes it answer 404. Do not kill it; do not kill `:6070` (S3's).

## 6. Honest gaps

1. **`(e3)` — R2's, untouched.** A silent elicitation-capable client with its own empty rendezvous
   still dies with `INTERNAL: typed command frames require an owner-qualified manifest command key
   before deserialization`, **before** any approval lane is reached. It is a headless-channel
   dispatch fault, not an approval fault: the same client on the shell route approves and denies
   correctly (`(e1)`/`(e2)`).
2. **`(f8)` — WR3's,** as the brief assigns. The refusal is precise: `note` exposes no media OUT
   port on `PluginWasmHandle`. Rerun the gate when WR3 reports it landed.
3. **`(f5)` still SKIPs.** The gateway's artifact id and the shell's live document are two
   identities; the shell's DOM carries its own, so the id the agent invented is not findable. The
   live-read law makes the BYTES the same document — it does not unify the ids. Naming them is a
   follow-up, and it is what would turn `(f5)` green.
4. **The approval summary is single-locale.** `capability.description` comes off the catalog already
   resolved for the session's locale, so the wire carries one language and the shell localises only
   its own chrome (labels, countdown, decisions — all en+de). Carrying both locales needs the
   catalog to keep the plugin's `LocalizedLabel` pair, which is A3's regeneration territory.
5. **Two approval surfaces, one modal.** `🤖️AgentApprovals` is a modal dialog and `💬️AgentChatPanel`
   renders the same decision inline. While the dialog is up its veil owns every pointer event, so
   the inline group cannot be clicked until the human dismisses the dialog — measured, with
   Playwright naming the overlay. Both are real affordances and the transcript row keeps the record,
   but a human reading the chat sees buttons that need a dismissal first. Unifying them (one live
   affordance per approval) needs the dialog's open state lifted into `🏛️ShellHost`, which is a
   peer-hot file; not taken here.
6. **wgpu parity debt.** The wgpu bank has the parser, the countdown helper and the shared-fixture
   test, so it can never drift from the React parser. It does NOT yet PAINT the new fields or the
   countdown — `🐚️Shell`'s wgpu approval overlay still draws capability/diff/risk/requested-by only
   (`approvals::summary_row_count` counts those four). That is the one piece of this slice the wgpu
   host has not caught up with.
7. **The renderer wgpu crate does not compile its test target** because of a peer's
   `InputState::pending_actions` change (§5.2). The wgpu unit test added here therefore type-checks
   but has not been RUN.

## 7. Files changed

Rust (`semio-framework-os-mcp`):
- `🏠️workspace/🦀️.rs` — `read_artifact_bytes` gained the session lane; new `read_session_artifact_bytes`.
- `🛡️policy/🦀️.rs` — `ApprovalRequest{capability_description, artifact_kind}`; `shell_summary(timeout_ms)` carries `capabilityTitle`/`description`/`artifactKind`/`timeoutMs`.
- `🔀️dispatch/🦀️.rs` — `settle_approval` fills the two new fields off the live `CapabilityDefinition`.
- `🛡️policy/🧪️tests/🔬️quick/🦀️.rs` — the shell-lane test asserts all four new summary fields.
- `🧪️tests/🤖️live-agent-loop/🟦️.ts` — preconditions, offer-ownership, the (f)-chain mutation verb, the (f4) witness, the non-eliciting client, the new-approval targeting, the running-row observer.

TypeScript / React:
- `🤖️AgentApprovals/🟦️.tsx` — the richer `ParsedApprovalSummary`, `approvalSecondsRemaining`, dialog rows show title/description/target + countdown, decision buttons carry `framework.approvals.<decision>.<id>`.
- `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` — the twin parser and countdown helper.
- `🤖️AgentApprovals/🧫️fixtures/🛡️summary/🔣️.json` — **new**, the shared corpus both banks assert.
- `🤖️AgentApprovals/🧪️tests/🧩️component/🟦️.tsx`, `🧪️tests/🔬️wgpu-unit/🦀️.rs` — fixture-driven laws on both banks.
- `💬️AgentChatPanel/🟦️.tsx` — the parsed approval row + `useApprovalCountdown` live region.
- `🔗️AgentBridge/🟦️.tsx` — four new en+de labels.
- `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — registers the `🤖️AgentApprovals` component suite.

Ticket: `🐍️ap1-shell-route-probe.ts`, `🗑️generated/ap1-*.txt`, this report.
