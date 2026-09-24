# WP-H7 — Hub Observability (G4) + checkpoint-publications Decision (G5)

Slice: H7 (session 10). Ports: 7910–7919. Private cargo target: `.tmp-ticket/wp-h7/target`. Captures: `wp-h7/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Structured hub event log (G4) | DONE except one leg. Live probe on the booted binary at `info`: directory command, directory socket open/close (shared `requestId`), document-socket refusal, boot, readiness, shutdown — **9/9 lines valid (Ajv) and declared**. Document-socket OPEN/CLOSE + presence live: wired into the two-client e2e, BLOCKED on a tree-consistent trusted catalog (W1), see Log |
| 2. checkpoint-publications decision (G5) | DECIDED: keep (product need). os client wiring BLOCKED (no guest canonical-pair export), see §G5 |
| 3. Hub test quick + long | DONE: quick **329/329** (10 skipped), long **338/338** (1 skipped, 2nd run; 1st run 337/338, see Log) |

## G4 — Hub observability

### Finding: the facility already existed
The G10 peer audit's "N8, still literally untouched / add `tracing`" was stale. `semio-framework-trace` has had a
zero-dependency structured observer since the 0918 ticket: `📝️record/🦀️.rs` (`TraceRecord` → one JSON line,
`TraceLevel`, `TraceOutcome`, `TraceSink` trait with `NullSink`/`StreamSink`/`CapturingSink`, `Tracer` with
per-event counters, `Span` with request ids, `SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK`), a language-agnostic
vocabulary (`🧫️fixtures/🛰️span-vocabulary`), and the hub already reported document/directory sockets, directory
commands, auth, rate limits, catalog load progress, boot, readiness and shutdown through it
(`HubState::span/note`, `GET /admin/api/observability`). No new crate, no `tracing`.

### Gaps closed
| Gap | Fix |
|---|---|
| No schema for the record line | `⏱️trace/📝️record/🧬️schema/🔣️.json` (draft-07, `additionalProperties:false`, level/outcome enums, event grammar, `requestId` pattern) + vectors `📝️record/🧫️fixtures/🔣️.json` (6 valid with exact lines, 7 invalid) |
| No file output | `FileSink` (append, survives restarts); `SEMIO_TRACE_SINK=file:<path>`; an unopenable path writes to stderr and reports `trace.sink` refused as its first record |
| Socket admission and close were two unrelated request ids | `Span::fork()`; document socket + both directory sockets: admission `ok` with `detail: upgrade` and the close `cancelled`/`closed` share `requestId` and principal |
| No presence records | `HubState::note_presence`: `server.presence.join` (peer becomes visible), `server.presence.expiry` (lease strips to identity), `server.presence.leave` (visible socket leaves) |
| Vocabulary drift: 10 emitted events undeclared (`server.shutdown`, `server.directory.backend`, `server.artifact.creation`, `server.auth.agent.*`) | vocabulary 13 → 23 events (Rust const + fixture, held equal by the existing law) |
| Two-client e2e never looked at the trace | asserts the whole first-hub trace (below), incl. `database=closed` on the `server.shutdown` record |

### Laws
- Rust (`semio-framework-trace`, `📝️record/🧪️tests/🔬️unit`): every vector renders to its exact line; schema properties == rendered keys and enums == Rust vocab; file sink appends across reopen; unopenable file sink reports refusal; fork shares id + identity.
- Hub bin-unit: `presence_join_expiry_and_leave_each_emit_one_record`.
- TS/Ajv (third-party oracle, `🌎️hub/🧪️tests/📝️trace-record/🟦️.ts`, in `os-hub-ts` vitest): valid vectors validate and read back as their record, invalid vectors refused, every declared event × outcome validates.
- Live (catalog-free): `.tmp-ticket/wp-h7/h7-trace-probe.ts <os-hub> <port>` boots the binary through the local-bootstrap pipe, drives a directory command, a directory socket open/close and a bogus-credential document socket, SIGTERMs it, and validates every stderr trace line with Ajv + vocabulary membership.
- Live (catalog): `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts` runs its hubs at `SEMIO_TRACE_LEVEL=info` (stderr sink); after the first hub's graceful exit every trace line must validate (Ajv), every event must be declared, one document socket's `upgrade`/`closed` pair must share `requestId`, and boot, catalog publication, directory command, presence join/expiry/leave and shutdown (`database=closed`) must appear. Its readiness wait now uses the shared progress-bound `waitForReadiness(…, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS)` instead of a fixed 180 s (a tree-consistent catalog takes ~6 min to verify on a debug hub, W1 §4.6); test cap 40 min.

## G5 — `checkpoint-publications`

**Decision: keep the route (product need), do not delete.** Evidence:
- Creation and GIS Map approval are the only server-side checkpoint producers. GIS Map approval refuses unless the
  active checkpoint's baseline equals the proposal base (`GisMapApprovalCheckpointPublisherV1Impl::current_matches_base`,
  `🌎️hub/🏗️bootstrap/🦀️.rs`); inference computes proposals against that baseline. After any human edit the baseline
  lags the head, and this route is the only path that advances it. Every cold open (`active-checkpoint/pair`) and MCP
  cold mount (`🌉️mcp/🏠️workspace/🔗️remote/🧩️pair`) also starts from it and replays the whole tail otherwise.
- Its callers are the hub's own process probes (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`
  `publishCheckpointPublicationProcessPairV1`, used by `checkpoint-publication-check`, the MCP cold-mount proof and the
  GIS Map proposal check, which seeds its initial active checkpoint through it). Deleting it would break those and
  leave GIS approval unusable after the first human edit.
- Named versions themselves (os Check In / Commit Checkpoint with a message) are already shared: they are events in
  the document log the hub persists and relays.

**os client wiring: BLOCKED, not done.** A client must upload the canonical pack+spr at the checked-in frontier.
No guest exposes its canonical pair to the host (worker `🏪️store/👷️worker/🟦️.ts` only holds the verified cold
bootstrap pair; no ABI export exists), so wiring needs a new guest ABI export + every guest rebuilt (W1-only) + the
worker upload. The better design (recommended follow-up): the hub materializes the checkpoint itself from the active
pair + its own WAL tail (`CheckpointRequest.operations`, native codec providers from the trusted catalog), and the
client's Check In only names the frontier. Route doc comment updated to record this.

## Evidence

| Command | Result | Capture |
|---|---|---|
| `cargo test -p semio-framework-trace --lib record` | **21/21** | `trace-test-1.txt` |
| `cargo check -p semio-framework-trace --target wasm32-wasip2` | EXIT 0 (1 pre-existing warning in `🧮️memory`) | console |
| `cargo check -p semio-hub --bin os-hub --tests` (after each edit set) | EXIT 0 | console |
| `bun nx run os-hub-ts:test` (incl. trace-record Ajv oracle) | 3 files, 15 passed, 2 skipped (gated e2e) | `hub-ts-test.txt` |
| `bun nx run os-hub-ts:typecheck` | EXIT 0 | `hub-ts-typecheck.txt` |
| `bun nx run @semio-tech/framework-trace-rs:test` | **54/54** | `trace-nx-test.txt` |
| `cargo build -p semio-hub --bin os-hub --features sqlite,postgres,neo4j,native-artifact-execution` (hub mutex) | EXIT 0 (9m36s) → `wp-h7/target/debug/os-hub` | `build-1.txt` |
| hub `test-quick --no-fail-fast` (mutex) | **329/329**, 10 skipped, EXIT 0 | `hub-quick-1.txt` |
| hub `test-long --no-fail-fast` (mutex) run 1 | 337/338: `inference::runtime::tests::quick::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` ("approval must remain retained before its cancellation phase") | `hub-long-1.txt` |
| that law alone ×5 + `presence_join_expiry_and_leave_each_emit_one_record` | 5/5 ok (load-sensitive timing law in untouched inference runtime); presence law ok | `targeted-1.txt` |
| hub `test-long --no-fail-fast` (mutex) run 2 | **338/338**, 1 skipped, EXIT 0 | `hub-long-2.txt` |
| two-client e2e sqlite, `hc1-boot` catalog | hub exit 1 at boot: `stdio.definition: decoded Stdio descriptor differs from native artifact semantics` (catalog older than tree) | `two-client-sqlite-1.txt` |
| `bun h7-trace-probe.ts wp-h7/bin/os-hub 7913` (no catalog: `not-ready` on `artifactAuthority` only) | create-space 202; directory socket 200/open/closed; document socket refused. **9 trace lines, 0 invalid**, `server.directory.socket` ok/`upgrade` and cancelled/`closed` share `r000000000001`; `server.document.socket` refused `credential`; `server.shutdown` ok `retained-sockets=0 database=closed` | `trace-probe-2.txt` |
| same, `w1-probe` catalog (13:10 tree) | not ready in 180 s; trace tail shows `CatalogLoading` ok ×3 then `GuestCodecExecuting` progress (debug run: 825 M/4 G fuel), the §4.6 W1 situation (catalog must come from the same tree as the binary) | `two-client-sqlite-2/3.txt`, `two-client-sqlite-4-debug.txt` |

## Files changed
- `🧰️framework/🔨️modules/⏱️trace/📝️record/🦀️.rs` — vocabulary 23, `FileSink`, `file:` sink, `TRACE_SINK_EVENT`, `Span::fork`
- `🧰️framework/🔨️modules/⏱️trace/📝️record/🧬️schema/🔣️.json` (new), `📝️record/🧫️fixtures/🔣️.json` (new)
- `🧰️framework/🔨️modules/⏱️trace/📝️record/🧪️tests/🔬️unit/🦀️.rs` — 5 laws
- `🧰️framework/🔨️modules/⏱️trace/🧫️fixtures/🛰️span-vocabulary/🔣️.json` — 23 events
- `🌎️hub/🏗️bootstrap/🦀️.rs` — presence records, socket span fork, route doc (G5)
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — presence trace law
- `🌎️hub/🧪️tests/📝️trace-record/🟦️.ts` (new), `🌎️hub/🧪️tests/🎚️config/🟦️.ts` — include it
- `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts` — trace sink + Ajv assertions
- `🌎️hub/README.md` — observability paragraph
- `.tmp-ticket/wp-h7/h7-trace-probe.ts` — live catalog-free probe (ticket input)

## Log
- build 1 (hub mutex, pid 85419) → `build-1.txt`.
- 17:3x: e2e cannot boot any on-disk catalog against the current tree (stale `hc1-boot`, mismatched `w1-probe`). Asked W1
  for a tree-consistent catalog path (`wp-w1/requests/h7.txt`); W1's catalog A run 7 holds the wasm mutex since 16:37.
- The trace sink paid for itself during that diagnosis: with the file sink the e2e's failure message now appends the
  trace tail, which is how the `GuestCodecExecuting` stall was read.
- Follow-up filed as a task chip: hub-materialized checkpoint publication on Check In (G5).
- Binary kept at `.tmp-ticket/wp-h7/bin/os-hub` (sha256 dfcf87a5…), private target deleted. Ports used: 7911 (e2e), 7913 (probe); no process left running.
- **Remaining leg (document-socket open/close + presence live):** once W1 publishes a catalog from the same tree as the
  hub binary, run `OS_HUB_TRUSTED_CATALOG_SOURCE=<catalog> OS_HUB_BINARY=<matching os-hub> HUB_TWO_CLIENT_PORT=7911 bun nx run os-hub-ts:two-client-e2e sqlite`;
  the receipt/console prints `traceEventOutcomes` and `documentSocketSpan`. Request recorded in `wp-w1/requests/h7.txt`.
