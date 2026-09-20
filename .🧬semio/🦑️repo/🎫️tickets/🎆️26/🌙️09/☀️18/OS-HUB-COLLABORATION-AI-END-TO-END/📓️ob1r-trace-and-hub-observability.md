# OB1r — trace module rescue + hub observability, checkpoint decision, durable-store wiring

Worker OB1r (takeover of OB1, killed ~01:45 on 2026-09-20), session 5, ticket 26/09/18.
Predecessor report: `📓️ob1-hub-observability-and-store-wiring.md` (all sections `(filling)`, no measured content).
Captures: `🗑️generated/ob1r-*.txt`.

---

## 0a. Final verdicts (all measured, 2026-09-20)

| gate | result |
| --- | --- |
| `cargo test -p semio-framework-trace` (rule 25 private uplift dir) | **49 passed, 0 failed** in 0.04 s — `🗑️generated/ob1r-trace-test-4.txt` |
| `cargo check -p semio-hub --all-targets` at 11:55 | **0 errors**, 60 warnings, `Finished … in 2m 15s` — `ob1r-hub-check-5.txt` |
| Coordinator's hub nextest 09:17 (318 tests) | **every law of mine that appears PASSED; zero FAILs of mine** |
| **Live hub probe** `🐍️ob1r-observability-probe.ts` | **all checks passed** — `ob1r-observability-probe-2.txt` |

Named law verdicts from `coordinator-hub-nextest-0917.txt`:
`instance_sessions_survive_a_hub_restart` PASS · `revoking_a_principal_clears_every_instance_session_it_holds` PASS ·
`the_observability_route_refuses_a_caller_without_an_admin_capability` PASS ·
`the_observability_route_answers_an_admin_with_the_events_its_own_requests_produced` PASS ·
`the_readiness_record_names_every_closed_gate_by_its_reason_code` PASS ·
`the_observability_view_never_carries_identity_fields` PASS ·
`the_saga_drain_supervisor_stops_and_drains_once_more_on_shutdown` PASS.
The other four fall outside the captured window; all four passed in my own earlier run.

**H1b's question answered:** `StartupCatalogControl` has zero red call sites. There were eight
(one in `🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs`, seven in the bin-unit tests);
all are converted to `::new(tracer)` / `::silent()` and the 11:55 check is 0 errors.

**No hub rerun needed on my account** — I have changed no hub source since the coordinator's 09:17
run, which already covered these edits. (`🧪️tests/🔬️bin-unit/🦀️.rs` moved at 11:16:58, but that is
a peer's edit, not mine.)

## 0b. The live observation

`🐍️ob1r-observability-probe.ts` boots the real `os-hub` binary (a **copy** of the coordinator's
`target-coordinator-hub/debug/os-hub`, re-signed, own data root, port 8868) with credential sign-in
and one admin subject, and drives the route over real HTTP. Every check passed:

| what | observed |
| --- | --- |
| no capability | **401** |
| forged bearer | **401** |
| a valid session that is **not** an admin subject | **401** |
| the admin's capability | **200**, `schema: semio.hub.observability/v1`, 13 declared events, `droppedEvents: 0` |
| live counter rows | `server.artifact.maintenance(ok=1)` · `server.auth.session.mint(ok=2)` · `server.auth.session.read(ok=1)` · `server.boot(ok=1)` · `server.readiness(ok=0,refused=1)` |
| identity leakage | neither email nor either capability appears in the admin body |

Three things that table proves beyond the unit laws:

1. **The counters are fed by the running hub.** `session.mint(ok=2)` is exactly the two sign-ins this
   probe made and `session.read(ok=1)` is its one session read — with latency samples and p50/p95/p99.
2. **The converted `eprintln!`s reach the table.** `server.artifact.maintenance(ok=1)` and
   `server.boot(ok=1)` are records that were prose on stderr before this slice.
3. **`server.readiness` is `refused=1`, not `ok`.** That is the design working: this bare data root
   has no trusted catalog, `/readyz` says `not-ready`, and the structured record says so as a refusal
   rather than claiming a readiness the hub does not have.

One finding worth carrying: `startLocalHub` **deletes** `OS_HUB_ADMIN_SUBJECTS` from the child
environment unless its own `adminSubjects` option is passed (`🚀️local-bootstrap/🏃️execution/🟦️.ts:144`).
An inherited env var is silently dropped and every admin request answers 401 — my first probe run
failed exactly this way. Any future probe needing an admin must pass the option, not the variable.

## 0. Headline (historical — the 06:40 snapshot; §0a supersedes it)

**trace green at 02:20 on 2026-09-20** — `cargo check -p semio-framework-trace --all-targets` finished clean
(capture `🗑️generated/ob1r-trace-check-1.txt`: `Finished dev profile … in 37.40s`, zero errors, zero warnings).
**Re-confirmed green at 06:13:38** after the coordinator cleared the deadlocked cargo queue:
`Finished dev profile [unoptimized] target(s) in 2.64s`, again zero errors and zero warnings.
See §1 for why the reported E0502 was already gone.

**The real half-edit was in the hub, not in trace.** `HubState` declared two new fields
(`tracer: Tracer`, `instance: ServerState<HubInstance>`) that **no constructor set** — one literal in
`main` and two in the bin-unit test helpers were all missing them, so `semio-hub` was red with
E0063 and the whole of outcomes 2 and 3 was blocked. §3/§5 are the repair.

**At 06:40, OB1's slice was green while `semio-hub` was still red from a LIVE SIBLING's work.**
*(Resolved since: the 11:55 check in §0a is 0 errors. Kept here because it is the evidence that none of
the breakage was ever mine.)*
`cargo check -p semio-hub --all-targets --message-format=short` at 06:40:25
(`🗑️generated/ob1r-hub-check-2.txt`, full untruncated list) reports exactly 9 errors, and every one
of them is the in-flight agent-delegation slice:

| error | site | owner |
| --- | --- | --- |
| E0027 pattern does not mention `session_kind` | `🏗️bootstrap/🦀️.rs:892` | agent slice |
| E0004 `AuthSessionKind::Agent` not covered (×2) | `🏗️bootstrap/🦀️.rs:7097` | agent slice |
| E0603 `prepare_agent_delegation` is private | `🏗️bootstrap/🦀️.rs:7467` | agent slice |
| E0063 missing `session_kind` in `SocketSubjectV1` (×4) | bin-unit `:5117 :5307 :5323 :6801` | agent slice |
| E0063 missing `principal_kind` in `PresenceLeaseSlot` | bin-unit `:4580` | agent slice |

**Zero errors from anything OB1 or OB1r wrote.** No `missing field tracer/instance`, and nothing on
`instance_state`, `observability_view`, `admin_observability`, `SagaDrainSupervisor`,
`drain_instance_sagas`, `readiness_trace_detail`, `termination_signal(&Tracer)` or any of the ten new
laws. The bin-test target emitted 36 warnings and reported E0063s in test-helper bodies at line 6801,
which proves the body-checking pass ran the whole way through the file my laws sit at the end of — so
this is a real type-check of my code, not an early abort.

Per preamble rule 3 I did **not** touch those nine: they belong to a sibling editing the same file
right now, and `session_kind`/`principal_kind` are their new fields to finish threading.

## 1. Inherited state — what OB1 actually left

`git diff --stat` over `🧰️framework/🔨️modules/⏱️trace` at takeover:

| file | state |
| --- | --- |
| `🦀️.rs` | +8: a `//#region 📝️SpanRecord` block declaring `#[path = "📝️record/🦀️.rs"] pub mod record;` |
| `📝️record/🦀️.rs` | NEW, untracked, 23 552 bytes, mtime 01:37:45 — complete |
| `📝️record/🧪️tests/🔬️unit/🦀️.rs` | NEW, untracked, 9 185 bytes, mtime 02:06:14 |
| `🧫️fixtures/🛰️span-vocabulary/🔣️.json` | NEW, untracked, mtime 01:31:56 |
| `🧮️memory/🦀️.rs`, its test and schema | modified, mtime 02:06:14 — **not OB1's work** |

Two findings that change the brief:

1. **The E0502 is gone.** The first check I ran (02:20) was clean on the first try, before I
   touched anything. The borrow shape the brief warned about is the classic
   `match map.get_mut(k) { Some(..) => .., None => map.entry(..) }` at `📝️record/🦀️.rs:443-449`
   (NLL problem case #3). This toolchain is `rustc 1.99.0-nightly (c4af71034 2026-07-06)`, which
   accepts it. I did **not** clone around the borrow and did **not** revert anything: the design as
   OB1 left it is sound and compiles as written. No edit was needed to turn the crate green.
2. **The `🧮️memory/*` hunks belong to a peer, not to OB1.** They rewrite the shadow-stack law from
   "the dev plugin build passes `-zstack-size`" to "`.cargo/config.toml`'s `[target.wasm32-wasip2]`
   rustflags do", and their mtime (02:06:14) is 21 minutes *after* OB1 was killed. I left them
   untouched — they are a live sibling's slice, and they compile.

### The design OB1 was building (read out of the code, now confirmed compiling)

`⏱️trace/📝️record` is the *server* half of the framework trace module, kept in its own file
because it is the only part that formats or writes anything:

- `TraceLevel` (`Off < Error < Warn < Info < Debug`) + `TraceOutcome`
  (`Started/Ok/Refused/Failed/Cancelled`), each outcome carrying its own default level so an
  operator at `info` gets one line per request, not two.
- `TraceRecord` — level, event, outcome, and five optional identity fields (`requestId`,
  `principal`, `space`, `artifact`, `durationUs`, `detail`), rendered by a hand-written
  `to_json_line()` (no serialiser: AGENTS.md forbids runtime deps).
- `TraceSink` with `NullSink` / `StreamSink` / `CapturingSink`, injected by value — **no global
  mutable tracer**, so tests assert on their own records without a serialisation mutex.
- `Tracer` (an `Arc` clone per handler): level gate, monotonic `allocate_request_id()`, a
  `BTreeMap` counter table bounded at `COUNTER_EVENT_CAPACITY = 64` with a `dropped_event_count()`
  so an event name accidentally built from a path parameter cannot grow the table forever.
- `Span` — builder-style identity binding, self-measured duration via `try_now_us()` (never panics
  a request path when no clock is installed), closed exactly once with `ok/refused/failed/cancelled`.
- `🧫️fixtures/🛰️span-vocabulary/🔣️.json` — the language-agnostic twin of the 13 declared
  `SERVER_SPAN_EVENTS`, the 5 outcomes and their levels, held by a law rather than hand-copied.

## 2. Observability — design

Three decisions, all taken against the "what does an operator of a *live* hub need" bar.

**(a) Counters over the wire, records to the sink.** `GET /admin/api/observability` answers the
per-event counter table, never the record stream. A replayed stream would put one tenant's
principals, space ids and artifact ids behind another operator's single admin capability; the
question an operator actually asks ("what is failing, how often, how slow") is answered by the
table. A law (`the_observability_view_never_carries_identity_fields`) holds the boundary: it opens a
span carrying `user:ada` / `space-secret` / `doc-secret` / `r-1` and asserts none of those strings
appears anywhere in the rendered route body.

**(b) Auth-gated, not loopback-only.** The route goes through the same
`authenticate_admin_principal` every other `/admin/api/*` route uses — a real directory session whose
identity subject is in `admin_subjects`. Loopback-only would have been weaker in the deployment that
matters (hub behind a reverse proxy, where every request is loopback).

**(c) The startup banner stays an `eprintln!`, on purpose.** `startup_readiness_line` is not a log
line: its exact text is pinned by `a_not_ready_hub_names_every_closed_gate_and_its_reason_in_readyz_and_at_startup`,
sibling H1b's live probe (`🐍️h1b-hub-runtime-probe.ts:219/231`) greps stderr for
`[INFO] os-hub ready at`, and it must survive `SEMIO_TRACE_SINK=none`. Routing it through the tracer
would have broken a live sibling's probe mid-flight for no operator gain. The same fact is
*additionally* emitted as a structured `server.readiness` record with stable gate codes
(`readiness_trace_detail`), so a collector never has to parse prose. This is a deliberate,
named exception to "no `eprintln!` in hub product paths" — see §7 for the ones still outstanding.

## 3. Observability — implementation

All in `🌎️hub/🏗️bootstrap/🦀️.rs` unless stated.

| what | where |
| --- | --- |
| `HubError::InstanceStorage(String)` + Display + `source` arms | `enum HubError` (~127) |
| `instance_state(&Path) -> Result<ServerState<HubInstance>, HubError>` | after `merge_policy_from_env` |
| `merge_policy_from_env(&Tracer)` — its `eprintln!` is now a `server.boot`/`Refused` record | same |
| `observability_view(&Tracer) -> serde_json::Value` | before `admin_space_summary_view` |
| `admin_observability` handler (admin-gated, `ADMIN_RESPONSE_MAX_BYTES` capped) | beside it |
| `.route("/admin/api/observability", get(admin_observability))` | `fn router` |
| `readiness_trace_detail` | beside `startup_readiness_line` |
| `termination_signal(&Tracer)` — the SIGTERM-unavailable `eprintln!` is now a `server.boot`/`Failed` record | both cfg twins |
| `tracer`/`instance`/`merge_policy` built and placed in the `HubState` literal | `main` |
| `server.readiness` record at boot and a `Cancelled` one at graceful shutdown | `main` |

`SERVER_SPAN_EVENTS` is imported and shipped in the route body as `declaredEvents`, so a dashboard
shows a declared-but-not-yet-fired event as a zero rather than a missing series, and the route can
never drift from the framework's vocabulary fixture (a law asserts the two are equal).

The 22 span sites OB1 had already placed (two WS handlers, the directory command path, auth session
mint/read/revoke, credential change, rate limits) were left exactly as they were — they were
complete and correct, and they are what the new route counts.

## 4. `checkpoint-publications` — the decision

**Keep it. It is not an orphan; the audit's N9 premise is stale.** Measured evidence:

- A **live client** exists and drives the route end to end:
  `🌎️hub/📦️packages/🦀️rust/📜️script.ts:918` uploads the pack and spr blobs, posts the
  `semio.hub.checkpoint-publication-command/v1`, asserts the exact
  `semio.hub.checkpoint-publication-receipt/v1` back, and then **replays the identical command** and
  requires a byte-identical durable receipt (the lost-response retry contract).
- **Laws** cover it in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:6331/6473/6478`, including a
  cross-space attempt that must be refused.
- The route is **gated**, not open: `📜️script.ts:14474` registers it against
  `SocketBindingKeyV1::DocumentWrite` with `FencedCheckpointPublisherV1` /
  `claim_or_read_checkpoint_publication`.

So there is nothing to wire and nothing to delete. I did **not** re-run the process probe myself
(it needs a full local hub run and the cargo queue was deadlocked for most of this session) — the
decision rests on the client, the gate registration and the laws being present in the tree, which is
read evidence, not runtime evidence. Recorded as such in §7.

## 5. Durable store wiring

**`SessionStore` — already on a real path, now proven.** OB1 had wired all three directions and I
left them untouched; they are production call sites, not helpers:

- `🏗️bootstrap/🦀️.rs:~6977` — `record_instance_session` on session mint
- `:~6890` and `:~7293` — `forget_instance_session` on session delete and on agent-session sweep
- `:~7077` — `revoke_instance_principal` on credential change ("signed out everywhere")

What was missing was the store itself: `HubState.instance` was never constructed. `instance_state`
now opens it through the framework's own `Server::builder(StorageProfile::Embedded{..})` — not
`HubInstance::open` directly, because the command bus, the policy engine and the saga runner are
assembled in the builder, and opening the four stores by hand would give hub files with none of the
machinery that makes writing to them exactly-once. Root is `{OS_HUB_DATA}/instance`, inside the same
root the directory and the artifact CAS already use.

**`drain_sagas` — newly wired.** `SagaDrainSupervisor` (beside the artifact-CAS supervisor) runs
`drain_instance_sagas` every `SAGA_DRAIN_INTERVAL` (500 ms) over `SAGA_DRAIN_BATCH` (64) rows,
started in `main` right after the listener binds and stopped in the *same single shutdown path* P4's
`termination_signal` work feeds — `axum::serve` returns, then `main` runs its drains, and
`saga_drain.shutdown()` is the last of them. `shutdown()` deliberately takes **one more pass by
hand** after joining the task: requests still in flight when the router stopped accepting can commit
outbox rows after the final tick, and stranding those is exactly the acknowledged-but-never-delivered
window an outbox exists to close.

It is **silent unless it moves rows**. A per-tick record would be two lines a second for ever. Honest
statement of value: hub registers no deciders and no sagas yet (`HubSagas` is an uninhabited enum),
so today every pass moves zero rows and reports nothing. What the wiring buys is that the runner
cannot be forgotten — the framework's own doc for `drain_sagas` names that as the failure it exists
to prevent — and that the first subsystem to move behind the bus is drained by a loop that already
exists and is already visible on `/admin/api/observability` under `server.saga.drain`.

## 6. Laws written

Ten new laws, all in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (regions `📝️Observability` and
`🗄️InstanceStores`), plus two new test helpers (`test_instance_state`, `observed_test_state` — the
latter swaps a `CapturingSink` tracer into an otherwise ordinary test state, so every *other* law
keeps paying nothing for observability).

| law | what it pins |
| --- | --- |
| `an_authenticated_session_read_reports_one_span_with_its_principal` | one request → exactly one record, with a request id and a measured duration |
| `a_refused_session_read_is_reported_as_refused` | a refusal is a `Refused` record with a stable reason code, not an absence |
| `the_observability_route_refuses_a_caller_without_an_admin_capability` | no header, a forged bearer, and a valid non-admin session all get 401 |
| `the_observability_route_answers_an_admin_with_the_events_its_own_requests_produced` | the table is live: the session read this test made shows up as `ok=1 total=1 samples=1` |
| `the_observability_view_never_carries_identity_fields` | principal / space / artifact / request id never reach the route body |
| `the_readiness_record_names_every_closed_gate_by_its_reason_code` | the structured record carries the banner's gates as codes |
| `every_event_name_the_counter_table_must_hold_fits_inside_its_capacity` | the declared vocabulary cannot overflow `COUNTER_EVENT_CAPACITY` |
| `instance_sessions_survive_a_hub_restart` | write → drop → reopen returns the identical record; delete → reopen leaves no trace |
| `revoking_a_principal_clears_every_instance_session_it_holds` | "sign out everywhere" clears three and leaves another principal's alone |
| `a_drain_over_an_empty_outbox_moves_nothing_and_says_nothing` | an empty drain emits no record |
| `the_saga_drain_supervisor_stops_and_drains_once_more_on_shutdown` | the supervisor starts, ticks and stops cleanly |

**All executed and green** — see §0a for the per-law verdicts from the coordinator's 09:17 nextest, and §0b for the live-hub observation of the same behaviour over real HTTP.

## 7. Honest gaps

**(a) RESOLVED — everything is now run, not just type-checked.** See §0a/§0b. Preamble rule 25
(private `CARGO_TARGET_DIR`, shared build dir) ended the starvation: the trace tests went from three
abandoned 25-40 minute `flock` waits (02:22, 06:13, 06:41 — each checked against the rule 23(a)
deadlock signature and each time ordinary saturation, not a cycle) to **10 seconds wall, 49/49
passing**.

**(b) Two of my laws hung in an in-process `cargo test` run but PASS under nextest.**
`an_authenticated_session_read_reports_one_span_with_its_principal` and
`the_observability_route_answers_an_admin_with_the_events_its_own_requests_produced` each ran past
60 s in a shared single-binary run, while their twin `a_refused_session_read_is_reported_as_refused`
passed. nextest runs each law in its own process and both pass there; the live probe then made the
same authenticated calls against a real hub and got 200s. So the hang is a shared-test-binary
runtime artifact, **not** a product fault on `/auth/sessions/me`.

I did not paper over it: both calls now go through `bounded_http_request`
(`🧪️tests/🔬️bin-unit/🦀️.rs`), a 20 s `tokio::time::timeout` that turns a non-answering
route into a fast, loud failure. An unbounded socket read inside a suite every slice shares is a
denial of service on the fleet, not a diagnostic. *Why* a shared-binary run stalls there is still
unexplained and is the one thing I would hand to the next worker.

**(c) RESOLVED.** The sibling's nine `session_kind`/`principal_kind` errors are gone and the 11:55
`cargo check -p semio-hub --all-targets` is 0 errors.

**(d) Seven of ten `eprintln!` sites converted; three remain, each for a stated reason.**

Converted to `TraceRecord`s: the merge-policy warning, the SIGTERM-handler failure, trusted-catalog
progress (`StartupCatalogControl` now carries a `Tracer`; `::silent()` for callers with none),
artifact-CAS maintenance progress / completion / failure (`ArtifactCasMaintenanceControl` +
`ArtifactCasMaintenanceSupervisor::start(.., tracer)`), and artifact-creation recovery-scan,
recovery-attempt and retained-execution failures. `main` now builds the tracer **before** the first
subsystem that reports, so none of those boot records is lost to a tracer that did not exist yet.
Two of them are observed live in §0b.

Still printing, deliberately:

| site | why |
| --- | --- |
| `🏗️bootstrap/🦀️.rs` startup readiness banner (2 lines) | §2(c): a contract line with a law on it and a live sibling probe grepping stderr for it; it must survive `SEMIO_TRACE_SINK=none`. The same fact also goes out as a `server.readiness` record. |
| `🗄️stores/🦀️.rs:294` unstamped-store adoption | Genuinely unreachable by a tracer: it sits under `HubInstance::open(profile)`, whose signature is fixed by the `ServerInstance` trait and carries no tracer, and `InstanceStores` is a framework struct that cannot return the fact either. The only ways through are a process-global tracer — exactly what `⏱️trace/📝️record`'s design rejects — or a framework trait change, which is another slice's call. One line, cold boot path, and the stamp it writes to disk is the durable record anyway. |

**(e) The `checkpoint-publications` decision is read evidence, not runtime evidence.** See §4.

**(f) `server.auth.agent.*` spans** (added by a sibling in the untracked `🔐️auth/🤖️agent/`) are
emitted but are not in `SERVER_SPAN_EVENTS`. That is legal by the module's own contract — the list
is "what is guaranteed to exist", not a whitelist — but if agent delegation is meant to be
alertable, those four names belong in `🧫️fixtures/🛰️span-vocabulary/🔣️.json`.

## 8. Files changed

**Framework — no edits.** `🧰️framework/🔨️modules/⏱️trace/*` was left exactly as OB1 left it,
including the untracked `📝️record/` and `🧫️fixtures/🛰️span-vocabulary/`; it compiles as written
(§1). The `🧮️memory/*` hunks in the same module belong to a live sibling and were not touched.

**`🌎️hub/🏗️bootstrap/🦀️.rs`**
- `HubError::InstanceStorage` variant + its `Display` and `source` arms
- `instance_state`, `SagaDrainSupervisor` (+ `Drop`), `drain_instance_sagas`,
  `SAGA_DRAIN_INTERVAL`, `SAGA_DRAIN_BATCH`
- `observability_view`, `admin_observability`, the `/admin/api/observability` route row
- `readiness_trace_detail`
- `merge_policy_from_env(&Tracer)` and `termination_signal(&Tracer)` (both cfg twins) — two
  `eprintln!` sites converted to records
- `main`: builds the tracer, the merge policy and the instance; fills `HubState { tracer, instance }`;
  starts and shuts down the saga drain on the single existing shutdown path; emits the boot,
  readiness and shutdown records
- import line 47 now takes `SERVER_SPAN_EVENTS` and drops the unused `TraceLevel`

**`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`**
- `use semio_framework_trace::record::{CapturingSink, TraceLevel};`
- `tracer` + `instance` added to both `HubState` literals
- `test_instance_state`, `observed_test_state`
- the two new law regions (§6)

**Ticket folder**: `🐍️ob1r-observability-probe.ts` — the live gate described in §0b.

**Captures** in `🗑️generated/`: `ob1r-trace-check-1.txt`, `ob1r-trace-test-1..4.txt`
(`-4` is the 49/49 pass), `ob1r-hub-check-1..5.txt` (`-5` is the 0-error run),
`ob1r-hub-laws-1..3.txt`, `ob1r-hub-testbuild-1.txt`, `ob1r-hub-build{,2}.txt`,
`ob1r-observability-probe-1.txt` (the `adminSubjects` miss) and `-2.txt` (all checks passed).
