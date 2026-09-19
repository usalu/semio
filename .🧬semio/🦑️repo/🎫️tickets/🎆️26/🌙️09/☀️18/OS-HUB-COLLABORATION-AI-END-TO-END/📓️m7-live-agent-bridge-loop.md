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
| 2 | elicitation wall-clock timeout (injected clock + test), same typed outcome as the shell timeout | ✅ |
| 3 | live proof (a)–(e) against a real `dev` session + real stdio MCP client | 🟡 |
| 4 | permanent nx e2e target + registry-generated launch row | 🟡 |
| 5 | wgpu parity for the approval + cancel affordances | 🟡 |

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

(filling)

## 6. Item 4 — the permanent nx e2e target

(filling)

## 7. Item 5 — wgpu parity

(filling)

## 8. Honest gaps

1. **`AgentBridge inference state parity` is red before and after this slice** — a TS↔Rust
   `ShellState` twin drift in `🖥️shell` (9 `ui*` fields missing from the generated fixture), argued
   but not experimentally isolated: I did not run the suite at `HEAD` without my edits, because the
   files the assertion reads are both unmodified in the working tree.

(filling)

## 9. Files changed

Rust (`semio-framework-os-mcp`): `🚚️transport/🦀️.rs`, `🚚️transport/🧪️tests/🔬️quick/🦀️.rs`,
`🚚️transport/🧪️tests/🔬️long/🦀️.rs`, `🛡️policy/🦀️.rs`, `🛡️policy/🧪️tests/🔬️quick/🦀️.rs`.

TypeScript: `📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts`.

Ticket: this report, `🗑️generated/m7-check-1.txt`, `m7-elicitation-tests.txt`, `m7-oslib-tests.txt`,
`m7-bin-build.txt`.
