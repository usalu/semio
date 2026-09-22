# HT16 — hub startup stall bound, wire frontier identity, and the hub suite number

Slice HT16 of ticket 26/09/18, session 8 (2026-09-22). Owns `🌎️hub/**` Rust this session.
Every number below comes from a capture in `🗑️generated/ht16-*`.

## 0. For the C8 slice's report reader — what changed on the wire

**C8 can drop its route-around for the welcome frontier's document id.**

Before this slice every `RuntimeFrontierSummary` the hub put on a document socket carried
`db_artifact_id`'s internal composite key — `v1:36:41:<space><document>`, e.g.
`v1:36:41:01a0c314-…artifact-0954e2…` — in `document_id`, because the db engine stamps its own key
into every `Frontier` it returns and the replication wire passed it through unchanged (C7 §1.4).

From this slice the hub projects it at its single frame-encoding door. Concretely, on a socket
opened for space `S`, document `D`:

| frame | field | now carries |
|---|---|---|
| `Welcome` | `server_frontier.document_id` | **`D`** |
| `Welcome` | `bootstrap.ArtifactBootstrap.baseline_frontier.document_id` | **`D`** |
| `Welcome` | `bootstrap.ArtifactBootstrap.required_tail_frontier.document_id` | **`D`** |
| `Ack` | `frontier.document_id` | **`D`** |
| `Commands` | `frontier.document_id` | **`D`** |
| `RebootstrapRequired` | `control.baseline_frontier.document_id` | **`D`** (already was — it comes from the directory's `ArtifactFrontier`, which is scope-keyed) |

Only the identity is projected; `head_edit_ordinal`, `head_edit_id`, `last_commit_seq` and
`chain_hash` are untouched.

**The ingress direction changed too, and C8's client must send `D`, not the key.** The db sync layer
compares an advertised frontier against its own key (`🔄️sync/🦀️.rs:1280`, `"database sync hello
frontier document mismatch"`), so the hub re-keys a client frontier onto the internal id before the
db sees it. A `SocketHelloV1`/`FrontierAdvertise` frontier that names **anything other than the
socket's own document** is now refused — `Error { code: "frontier-document-mismatch" }` on
`FrontierAdvertise`, and the same error frame followed by a socket close on the hello. A client that
echoes back the frontier the hub handed it is correct by construction; a client still hard-coding
the old `v1:…` key will be refused, which is the point.

`validateArtifactBootstrapIdentity` (`🏪️store/👷️worker/🟦️.ts:3884`) now compares equal values, so
whoever starts sending `ArtifactBootstrap` does not need C7's "project it in the same change" caveat
any more.

## 1. Item 1 — hub start under load: three wall-clock budgets replaced by stall bounds

### 1.1 What was measured, and by whom

CE2 §7.1 recorded hub 7621 dying `ArtifactAuthority(DeadlineExceeded)` on two consecutive starts
from the coordinator's own line while 17 rustc ran; the third start succeeded once the machine
quietened. M8 had recorded the same thing earlier on a warm 612 MB catalog, twice. Nothing about the
catalog differed between the failing and the succeeding start — only the machine's load.

### 1.2 The root: a bound that charges an operation for other people's work

`OperationContext` (`🌎️hub/🗿️artifact-authority/🦀️.rs`) had exactly one shape of bound, an absolute
wall-clock instant, and `checkpoint()` refused the moment `now_ms() >= deadline_ms`. That clock runs
while the operation is **not** running, so every millisecond the trusted-catalog load loses to the
other 80 things on the machine is charged against it. Three startup callers passed it 30 s:

| caller | file:line (before) | what it bounded |
|---|---|---|
| trusted-catalog load | `🏗️bootstrap/🦀️.rs:586` `TRUSTED_CATALOG_STARTUP_BUDGET_MS = 30_000` | verifying every package in the catalog closure |
| artifact-CAS coordinator handshake | `🏗️bootstrap/🦀️.rs:10241` `startup_now_ms + 30_000` | `configure_coordinator`, propagated with `?` — this one **aborted the boot** |
| launcher readiness poll | `🚀️local-bootstrap/🏃️execution/🟦️.ts:14` `LOCAL_READINESS_DEADLINE_MS = 30_000` | waiting for `/readyz` to admit the run |

### 1.3 The landed shape — `stall_bounded`, the `work_bounded` of HT15 §5.1 for this port

`🌎️hub/🗿️artifact-authority/🦀️.rs`

| change | what it does |
|---|---|
| new private `enum OperationBoundV1 { Deadline(u64), NoProgress { span_ms, last_checkpoint_ms: AtomicU64 } }` | names the two kinds of bound instead of assuming one |
| `OperationContext::new(deadline_ms, …)` | **contract unchanged** — still an absolute instant, still `const fn`; every client-facing operation keeps it |
| new `OperationContext::stall_bounded(span_ms, …) -> Result<Self, AuthorityError>` | no calendar; refuses only when `span_ms` passed with the operation reaching **no checkpoint at all**. `span_ms == 0` is `Err(InvalidLimits)` — a bound that can never fire is not a bound |
| `checkpoint()` | matches on the bound; a passing checkpoint **restamps** the no-progress span, so reaching here *is* the progress |
| new `AuthorityError::Stalled` | distinct from `DeadlineExceeded` on the wire and in the logs: nothing promised an answer by an instant |
| `deadline_ms()` → `Option<u64>` | a stall-bounded operation has no instant to hand to the two CAS lease horizons; they fall back to their own max TTL (`📇️directory/🦀️.rs:3004`, `:3103`) |

Nothing is loosened: the load is still bounded by `AuthorityLimits::maximum()`, by
`TRUSTED_COMPONENT_CLOSURE_MAX_BYTES` / `TRUSTED_DESCRIPTOR_CLOSURE_MAX_BYTES` /
`TRUSTED_BROWSER_ACTOR_CLOSURE_MAX_BYTES` / `TRUSTED_CATALOG_MAX_CODECS`, and by cancellation, which
is still decided **before** the stall verdict. The load checkpoints per package and per 64 KiB
hashed chunk (`🔏️trusted-catalog/🦀️.rs` `dual_hash`/`sha256`), so a slow machine keeps restamping
and a wedged read does not.

`🌎️hub/🏗️bootstrap/🦀️.rs`

* `TRUSTED_CATALOG_STARTUP_BUDGET_MS` → `TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS`, **same 30 000** —
  a no-progress span, not a lengthened timeout.
* `StartupArtifactAuthority::BudgetExceeded` → `::Stalled`; the readiness reason
  `trusted-catalog-load-exceeded-its-startup-budget` → `trusted-catalog-load-stalled-before-it-finished`.
* The artifact-CAS coordinator handshake takes the same `stall_bounded` context instead of its own
  second wall budget.
* `StartupCatalogControl::report` used to emit only the first and last unit, so a 30 s load printed
  nothing for 30 s and an operator could not tell a slow machine from a wedged one. In-flight units
  now emit too, rate-limited to `STARTUP_CATALOG_IN_FLIGHT_TRACE_MIN_GAP_MS = 1_000`, so the record
  count never scales with the catalog's.

`🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`

* `LOCAL_READINESS_DEADLINE_MS` → `LOCAL_READINESS_STALL_BOUND_MS`, **same 30 000**.
* `waitForReadiness` now bounds on the span with no change in an explicit observation —
  `${answered?} http=<status> <closed gates> bytes=<captured output length>` — instead of on total
  elapsed time, and its refusal names what the hub last said and for how long. The span is a third
  parameter (default the constant) so a law can state it in 200 ms.
* `waitForUiReadiness` (`📦️packages/🦀️rust/📜️script.ts`) had the same total-budget shape over the
  secure-local UI and took the same treatment.

### 1.4 Laws

| law | file | what it pins |
|---|---|---|
| `a_stall_bounded_context_refuses_only_when_it_reached_no_checkpoint_for_a_whole_span` | `🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs` | a checkpoint every 99 ms of a 100 ms span survives **49× the span**; one uninterrupted span with no checkpoint refuses, and refuses as `Stalled`, not `DeadlineExceeded`; cancellation still wins first; `span_ms == 0` is refused at construction |
| `only_a_calendar_bounded_context_publishes_an_absolute_deadline` | same | `new(...).deadline_ms() == Some(...)`, `stall_bounded(...).deadline_ms() == None` |
| `a_stalled_trusted_catalog_load_closes_the_gate_and_never_claims_the_catalog_is_unloadable` | `🧪️tests/🔬️bin-unit/🦀️.rs` (renamed from `…budget_overrun…`) | the stalled outcome carries no authority and publishes a reason distinct from every other closed reason |
| `readiness waits on the hub's own progress, never on a total wall-clock budget` | `🧪️tests/🧱️foundation-source/🟦️.ts` (new) | a hub that keeps changing what it reports is waited for well past the span and then admitted; a hub that repeats itself is refused **as a stall**, naming what it last said; a hub that exits is refused immediately |

## 2. Item 2 — C7 §1.4: the hub stamped its internal db key into every wire frontier

### 2.1 Root

The hub keys documents internally by `db_artifact_id(scope)` = `v1:<len>:<len>:<space><document>`
(`🏗️bootstrap/🦀️.rs:664`) because the db and fanout catalogs are flat and a bare document id is not
unique across spaces. `state.db.hello(db_id, …)` hands that key to the db engine, the engine stamps
it into the `Frontier` it returns, and `engine_frontier_to_wire` (`:4699`) passed it through
unchanged into `Welcome`, `Ack` and `Commands`. C7 measured `server_frontier.document_id =
v1:36:41:01a0c314-…artifact-0954e2…` where the client's own `documentId` belongs.

It has been invisible only because nothing checked it — the store worker assigns the welcome
frontier on the `None` bootstrap branch without validating it, and a reconnecting client echoes the
key straight back, so the round trip is self-consistently wrong. `validateArtifactBootstrapIdentity`
(`🏪️store/👷️worker/🟦️.ts:3884`) is the first reader that compares, and it would refuse.

### 2.2 The fix — one door, both directions

`🌎️hub/🏗️bootstrap/🦀️.rs`

| new item | what it is |
|---|---|
| `project_wire_frontier(frontier, document_id)` | stamps the document's own id onto one outbound frontier |
| `project_server_frame(frame, document_id)` | every frontier a frame carries, including **both** of an `ArtifactBootstrap`'s (`baseline_frontier`, `required_tail_frontier`), exhaustively matched so a new frame variant cannot silently skip it |
| `frame_carries_frontier(frame)` | so the projection clones only when it has something to rewrite |
| `wire_frontier_to_db(frontier, document_id, db_id) -> bool` | the exact inverse on ingress; **refuses** (returns `false`, leaves the frontier untouched) a frontier naming another document rather than overwriting an identity claim |
| `encode(frame, document_id)` | the hub's ONLY frame-encoding door now takes the document, so no call site can forget. All 11 call sites updated |

Ingress: the `SocketHelloV1` frontier is re-keyed before `state.db.hello`, and
`ClientFrame::FrontierAdvertise` before `db::sync::handle_frontier_advertise`. Both refuse a
mismatch with `Error { code: "frontier-document-mismatch" }`. This is load-bearing, not defensive:
`🔄️sync/🦀️.rs:1280` refuses a hello whose advertised frontier does not name its own document, so
without the inverse projection a client sending its honest `documentId` would be rejected by the db.

### 2.3 Law

`every_wire_frontier_the_hub_sends_names_the_document_not_its_internal_db_key`
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, new) pins the whole round trip: a `Welcome` carrying an
`ArtifactBootstrap` (all three frontiers), an `Ack`, a `Commands`, that only the identity moves and
the position does not, that a frontier-free frame is returned byte-identical, that an honest client
frontier is re-keyed onto the db id, and that a frontier naming another document is refused and left
exactly as the client sent it.

## 3. Item 3 — G16 open hub-backend findings acted on

G16's ranked list, each item with what this slice did.

| # | G16 item | HT16 |
|---|---|---|
| 1 | run the hub suite to a final number | **done — §4** |
| 2 | TC2 §9 catalog-carried genesis | **not taken.** It is TC3e's lane this session and the fix is the 4-step landing order TC2 already wrote; duplicating it would collide with a live slice |
| 3 | live Postgres/Neo4j | **descoped by the user** (Docker), untouched |
| 4 | C3 §3.4 presence-roster asymmetry | **already fixed by PR1 in source**; the live re-verification the coordinator asked for is **blocked** — see §5.1 |
| 5 | browser-actor activation | not hub backend |
| 6 | real reverse proxy | a runtime exercise, no code to change |
| 7 | register a saga or delete `HubSagas` | **named, not a defect — see 3.2** |
| 8 | two stale README sections | **done — 3.1** |
| 9 | `checkpoint-publications` dead-or-undiscovered | **answered — 3.3** |
| 10 | Wave 3 decision | a ticket-owner decision, not a hub defect |

### 3.1 `🌎️hub/README.md` — the observability claim was false (G16 §h, coordinator addition (a))

Two sentences claimed the opposite of the shipped code:

* `:333` "There is **no metrics endpoint and no structured request/WebSocket tracing** — the hub has
  no `tracing`/`prometheus`/`opentelemetry` dependency."
* `:696` "**No metrics, no request tracing.** `/readyz` and stdout are the whole observability
  surface."

What the code does: `HubState.tracer` is a first-party `semio_framework_trace::Tracer` configured
from `SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK`; `HubState::span` opens a request-id-bound span and 17
call sites use it, including the document WebSocket handler (`server.document.socket`), the
directory backend faults and the boot; `GET /admin/api/observability` (`🏗️bootstrap/🦀️.rs:9423`,
behind `authenticate_admin_principal`) returns `{schema: "semio.hub.observability/v1", level,
droppedEvents, declaredEvents, rows:[{event, started, ok, refused, failed, cancelled, total,
samples, p50Us, p95Us, p99Us}]}`. Only the third clause of the old sentence was true — there is no
third-party dependency and no Prometheus exposition format.

Landed: the route is now a row in the "Health endpoints" table, the paragraph under it states what
the observer is, what opens a span on it, and that the missing thing is a **scrape endpoint in
someone else's format**, not tracing; the gap-list bullet says the same in one line.

### 3.2 `HubSagas` (G16 #7) — named, and a decision rather than a defect

`🌎️hub/🗄️stores/🦀️.rs:103/120/129` hold three uninhabited enums (`HubDeciders`, `HubSagas`,
`HubResolvers`), each already carrying a 🕳️ docstring that says exactly why it is empty ("while its
command handling still lives in `🏗️bootstrap`", "while nothing drains its outbox into further
commands", and — importantly — "an empty ladder is not an open door: `ResolverChain` falls back to
the anonymous principal"). This is not a *silently* empty CQRS layer, so G16's "delete it or say so
plainly" is already half-satisfied; the other half, deleting the wiring, would remove the seam the
hub is explicitly migrating toward and change the framework's generic `ServerInstance` contract,
which is G16 #10's Wave-3 decision and not a hub-backend defect. **No code changed.** Whoever takes
the Wave-3 decision takes this one with it.

### 3.3 `POST …/checkpoint-publications` (G16 #9) — undiscovered, not dead

Re-established by grepping the whole tree (excluding this ticket's notes) for the route path, for
`CheckpointPublicationCommandV1` and for `semio.hub.checkpoint-publication-command/v1`: the only
producers anywhere are `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` and
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`'s own process probe. No shell, no wgpu/native client, no MCP
gateway, no React host. The route is complete, fenced, authenticated as a document-write subject and
law-covered; nothing has been written yet that needs it, and deleting it would delete the only
authority path by which a non-hub process can publish a checkpoint at all.

Landed: that answer now lives in `post_checkpoint_publication`'s own docstring
(`🏗️bootstrap/🦀️.rs`), with the date and the exact greps, so Wave 3's module split does not inherit
the ambiguity a third time.

### 3.4 A red the hub's own TS gate was carrying (found while running it)

`os-hub:foundation-source-check` was **failing before this slice touched anything**, for two
independent reasons, both now fixed:

1. `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` did not list `hubDevPostgresBinaryPath` /
   `HUB_DEV_POSTGRES_BINARY_TARGET`, which `📜️script.ts` has imported since the PostgreSQL dev
   binary landed. Confirmed pre-existing against `HEAD~2`.
2. The heavy law `package, target, input and launch registrations bind only the moved owners` parses
   every owner's TypeScript, the router's, `📋️project.json`, `nx.json` and `.vscode/launch.json`;
   it measured **21.7 s at load 62** against bun's **5 s default** per-test timeout, which was
   chosen for nothing in particular. It now carries an explicit named budget (600 s — ~30× the
   measured run, a wedge bound, not a verdict on speed), as does the new readiness law.

`bun test ./🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` at default harness settings: **12 pass / 0
fail, 270 expect() calls, 18.4 s** (`🗑️generated/ht16-ts-foundation-4.txt`).

## 4. Item 4 — the full hub suite in a private target dir, and `os-hub:test`

### 4.1 The suite — **330 / 330, 0 reds, 0 leaky**

`📜️coordinator-hub-run.sh` copied to `📜️ht16-hub-run.sh` (never run in place), same build + suite,
`CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-ht16`, run as `ht16a`:

```
BUILD_START 11:40:53 → Finished dev in 10m 28s → BIN_DONE 11:51:21
NEXTEST_START 11:51:21 → NEXTEST_DONE 12:02:49
Starting 330 tests across 2 binaries
Summary [  69.124s] 330 tests run: 330 passed, 0 skipped
```

Captures: `🗑️generated/ht16-hub-nextest-full-ht16a.txt` (full),
`🗑️generated/ht16-hub-nextest-latest-ht16a.txt` (filtered), `🗑️generated/ht16-hub-run-1.txt`.

| | baseline s7i (02:01) | ht16a (12:02) |
|---|---|---|
| tests | 327 | **330** (+3: the two stall-bound laws and the wire-frontier law) |
| passed | 327 | **330** |
| failed | 0 | **0** |
| `LEAK [` lines | 3 | **0** (`grep -c LEAK` over the full capture) |
| wall | 31 s | 69.1 s (load 74–96 during the run) |

**No red to root-cause.** The three previously-leaky sub-second laws did not leak this time, which
is consistent with HT15 §6's reading of them as nextest's 100 ms `leak-timeout` measured against a
loaded machine rather than a teardown defect — this run was slower overall (69 s vs 31 s) yet leaked
nothing, so the leak verdict tracks the per-process pipe close, not the suite's speed. Nothing was
changed for it.

Slowest five, for whoever tunes the level budgets:
`…artifact_chunk_cas_opaque_continuation_converges_after_page_overflow_cancel_and_resume` 65.2 s;
`…genesis_accepted_only_recovery_has_no_prepared_or_public_side_effects` 30.1 s;
`…admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row` 21.0 s;
`…artifact_chunk_cas_neutral_fixture_matches_repository_sha256` 20.9 s;
`…the_rust_decoders_and_the_json_schema_admit_exactly_the_same_wire` 17.0 s.

### 4.2 Type-check, with warnings as proof of expansion

`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-ht16 cargo check -p semio-hub --all-targets
--keep-going`, twice:

| run | result | capture |
|---|---|---|
| 11:16 → (aborted) | **blocked by a peer**: `semio-framework-plugin` (lib) did not compile — 6 errors in FP10's in-flight `Emit { tasks }` / `TaskSlot.reserved` edit, none of them in a file this slice touched | `🗑️generated/ht16-check-1.txt` |
| 11:26 → 11:35 (8 m 52 s) | **Finished, 0 errors, 348 warnings**, lib + `bin "os-hub"` + both test targets expanded | `🗑️generated/ht16-check-2.txt` |
| 12:07 → 12:10 (3 m 12 s), after clearing my own two `unused_qualifications` | **Finished, 0 errors, 345 warnings** | `🗑️generated/ht16-check-3.txt` |

### 4.3 `os-hub:test` — G14's "never measured" item, measured

`bun ./📜️script.ts test` (the verb behind the `os-hub:test` nx target) from
`🌎️hub/📦️packages/🦀️rust`, once, at load 74, same private target dir
(`🗑️generated/ht16-os-hub-test.txt`):

```
Nextest run ID … with nextest profile: fundamental
Starting 301 tests across 2 binaries (29 tests skipped)
Summary [   8.128s] 301 tests run: 301 passed, 29 skipped
bun ./📜️script.ts test  117.13s user 21.64s system 76% cpu 3:02.07 total
```

**It finishes inside its level budget: 8.128 s against `TEST_LEVEL_BUDGET_MS.fundamental = 15_000`,
with 46 % of the budget to spare, on a machine at load 74.** The 3 m 02 s total is the
`cargo nextest list --list-type binaries-only` build/metadata phase, which is charged to
`buildBudgetMs()` — and `BUILD_BUDGET_MS = 0`, i.e. the build phase is deliberately unbounded. So
the level budget is a bound on execution only, and the hub's fundamental level fits it.

The verb skips 29 tests (`--skip long:: --skip exhaustive::`) and runs 301 of the 330; the other 29
are the `long`/`exhaustive` levels that `os-hub:test-long` / `:test-exhaustive` own.

Incidental: the verb prints `[DEBUG] Nextest artifacts retained at …` from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` (`nextestArtifactLocation`). Preamble
rule 10 forbids leftover `[DEBUG]` output; that file is the repo library, not `🌎️hub`, so it is
named here rather than edited.

## 5. Honest gaps

### 5.1 🚨 Every published trusted-catalog root in this repository is unloadable by a hub built from the current tree

This is the biggest thing this slice found, it blocks four other slices, and it is **not mine to
fix**.

Measured: HT16 booted its own hub on **7681**, on its OWN copy of `jc1-boot`
(`.🧬semio/🌐hub/ht16-boot`, never shared), executing `target-ht16/debug/os-hub` (built 11:51 today).
The hub **exited before binding** (`🗑️generated/ht16-hub-7681.txt`):

```
Error: ArtifactAuthority(Catalog("unknown field `openTarget`, expected one of `id`,
`selectedClosure`, `selectedClosureSha256`, `openTargets`, `generationId` at line 1 column 312"))
```

Root: commit `50c97b2051` (**2026-09-21 14:38**) changed
`TrustedBundleProfileV1::open_target: TrustedBundleProfileOpenTargetV1` to
`open_targets: Vec<TrustedBundleProfileOpenTargetV1>` (TC3b's "a generation carries one creatable
kind per entry"). `TrustedBundleProfileV1` is `#[serde(deny_unknown_fields)]`, so a bundle written
before that commit is refused outright. Every data root on disk predates it:

| root | `trusted-catalog/current.json` | profile field |
|---|---|---|
| `gm1-boot` | 09-20 20:20 | `openTarget` |
| `hs1-boot` | 09-20 21:53 | `openTarget` |
| `jc1-boot` | 09-21 05:49 | `openTarget` |
| `rb1-prod` | 09-21 11:10 | `openTarget` |
| `s10-boot`, `c8-boot`, `tc3c/d/e-boot` | — | no `current.json` at all |

Hand-editing the bundle is **not** a fix: `trusted_profile_generation` now frames a COUNT ahead of
the open-target rows, so the generation id of a one-target profile is no longer the stored
`8086b61f336e08b5…`, and the loader validates `profile.generation_id` against the recomputed one.
The only honest repair is a **republish from the current tree** — TC3e's lane, through the wasm
mutex. Nothing was patched here; the unusable copy was deleted.

**Consequence for peers.** A hub started today on an existing root binds only if it is executing a
binary from before 14:38 on 09-21. 7621 (C8/CE3, `target-jc1` built 03:06 on 09-21) still runs
because it is an old binary; it will not survive a restart on a rebuilt one.

### 5.2 What is NOT observed at runtime

* **The wire-frontier fix (§2) is law-proven and suite-proven, not observed on a live socket.** The
  suite's socket laws bind a real hub in-process and drive `handle_ws`, so the projection executes
  there; but reading the byte off an external socket needs a hub with a loadable catalog, which
  §5.1 says does not exist today. `🐍️c7-welcome-socket-probe.ts` is the probe to run the moment
  TC3e republishes one — point it at the new origin and read `server_frontier.document_id`.
* **The stall bound (§1) is law-proven, and its fast-exit half is observed.** `waitForReadiness`
  refused the 7681 boot in under a second with `hub exited before readiness` rather than waiting out
  a span, which is the branch the live boot exercised. The interesting branch — a slow-but-advancing
  catalog load finishing past 30 s — could not be observed for the same reason as above.
* **PR1's presence fixes are still not verified live (coordinator addition (b)).** The probe
  `🐍️pr1-presence-socket-probe.ts` needs a document socket, which needs `open-plan`, which needs a
  loaded catalog. Blocked by §5.1, not by anything in PR1's own work. The two verdicts to watch when
  a hub with a fresh catalog exists are still PR1 §6's: `symmetric-roster-for-the-whole-window` PASS
  **including the first sample**, and `no-decay-while-the-socket-is-open` PASS with `PR1_BEAT_MS=0`.

### 5.3 Smaller, named

* **A non-capturing launcher run has no pre-bind progress signal.** `waitForReadiness`'s observation
  includes `run.output().length`, which is only populated for `capture: true` runs; with
  `stdio: "inherit"` the launcher cannot read the hub's boot trace, so between spawn and port-bind
  its observation is constant and the stall bound degrades to "30 s with nothing observable". That
  is still strictly better than a 30 s total budget (a hub that binds and then reports changing
  gates is now waited for indefinitely), but it is not a full stall bound. Closing it properly means
  a progress frame on the local-bootstrap pipe, which is a protocol change this slice did not make.
* **`AuthorityError::Stalled` maps to `504 GATEWAY_TIMEOUT`** alongside `DeadlineExceeded` in
  `checkpoint_publication_error_status`, and to `RebootstrapError::DeadlineExceeded` in
  `🛰️lag-rebootstrap`. No production route constructs a stall-bounded context yet — both startup
  callers are pre-bind — so neither mapping has ever fired; they exist so the variant is total.
* **`🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` is still not an exhaustive declaration gate.**
  Adding `hubDevPostgresBinaryPath` made it green, but several exported names in
  `🚀️local-bootstrap/🏃️execution/🟦️.ts` remain unlisted and nothing fails; the fixture lists what
  the router imports, not what the owner exports. Stated, not changed.
* **`os-hub:test` prints `[DEBUG]`** from the repo library — §4.3.

## 6. Files changed

Rust (`🌎️hub`, owned by this slice this session):

* `🗿️artifact-authority/🦀️.rs` — `OperationBoundV1`, `OperationContext::stall_bounded`, the
  two-armed `checkpoint`, `AuthorityError::Stalled` + its `Display`, `deadline_ms() -> Option<u64>`.
* `🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs` — two new laws; one `deadline_ms()` call site.
* `📇️directory/🦀️.rs` — the two CAS lease horizons take the optional deadline (`:3004`, `:3103`).
* `🛰️lag-rebootstrap/🦀️.rs` — `Stalled` arm in `map_authority`.
* `🏗️bootstrap/🦀️.rs` — `TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS`, `StartupArtifactAuthority::Stalled`,
  the new closed reason, both startup contexts stall-bounded, `StartupCatalogControl`'s rate-limited
  in-flight progress record, `project_wire_frontier` / `project_server_frame` /
  `frame_carries_frontier` / `wire_frontier_to_db`, `encode(frame, document_id)` and its 11 call
  sites, the two inbound re-keyings, `post_checkpoint_publication`'s docstring.
* `🧪️tests/🔬️bin-unit/🦀️.rs` — the renamed stalled-gate law, the new wire-frontier law.

TypeScript:

* `🚀️local-bootstrap/🏃️execution/🟦️.ts` — `LOCAL_READINESS_STALL_BOUND_MS`,
  `localHubReadinessObservation`, the stall-bounded `waitForReadiness` with an injectable span.
* `📦️packages/🦀️rust/📜️script.ts` — the import rename; `waitForUiReadiness` stall-bounded.
* `🧪️tests/🧱️foundation-source/🟦️.ts` — the new readiness law; explicit budgets on it and on the
  heavy registration law.
* `🧫️fixtures/🧱️foundation-source/🔣️.json` — the constant rename plus the two missing
  PostgreSQL-dev-binary names.

Docs:

* `🌎️hub/README.md` — the observability route row, the corrected paragraph, the corrected gap
  bullet.

Ticket folder (new, permanent for this ticket):

* `📜️ht16-hub-run.sh` — HT16's own copy of the coordinator's hub build+suite line.
* `📜️ht16-hub-hold.sh` — holds a hub on 7681 against HT16's own catalog copy.

Captures, all under `🗑️generated/ht16-*`: `check-1/2/3`, `hub-run-1`, `hub-nextest-full-ht16a`,
`hub-nextest-latest-ht16a`, `os-hub-test`, `ts-foundation-1..4`, `hub-7681`, `hub-hold-wrapper`,
`hub-pid`.

Infrastructure left running: **none.** The 7681 hub exited on its own (§5.1) and its data-root copy
`.🧬semio/🌐hub/ht16-boot` was deleted. 7621, 7641 and 7651 were never touched.
