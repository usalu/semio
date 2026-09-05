# Directory Command Receipt Transport — lane `fable-directory-command-receipt`

Implements every slice of `📓️terra-directory-command-receipt-transport-p0.md` plus section 2
("Close the command/result wire and make retry exact") of `📓️terra-space-administration-ui-current-p0.md`.

The command lane is now a **bounded, request-idempotent receipt transport**: one sealed
`DirectoryCommandRequestV1` in, one canonical `DirectoryCommandReceiptV1` out, keyed durably by
`(authenticated user id, request id)`, with a closed error vocabulary and an honest secret-result
rule. A lost `202` can no longer mint a second invitation, and a retry can no longer replay a
one-shot capability that was never persisted.

## 1. Contract (schema-first, three twins)

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`

```
DirectoryCommandRequestV1  { schema, requestId (32-hex nonzero), command }
DirectoryCommandReceiptV1  { schema, requestId, commandSha256, outcome, events (≤4), result, receiptSha256 }
DirectoryCommandOutcomeV1  = accepted | previously-accepted | secret-undeliverable
DirectoryCommandResultV1   = { kind: "none" } | { kind: "invite", inviteToken }
DirectoryCommandErrorCodeV1 = unauthorized | forbidden | stale-session | request-conflict | invalid
                            | overloaded | too-large | capacity | closed | cancelled | transport
```

Constants (identical in all three twins): `DIRECTORY_COMMAND_REQUEST_MAX_BYTES = 8 KiB`,
`DIRECTORY_COMMAND_RECEIPT_MAX_BYTES = 64 KiB`, `DIRECTORY_COMMAND_RECEIPT_MAX_EVENTS = 4`,
`DIRECTORY_COMMAND_INVITE_TOKEN_MAX_BYTES = 256`, `DIRECTORY_COMMAND_REQUEST_ID_LEN = 32`.

- `receiptSha256` is SHA-256 over the declaration-ordered *unsigned* canonical JSON, exactly the
  `DirectoryEventPageV1` convention; `commandSha256` is SHA-256 over the canonical command JSON, so
  hub and every client derive it independently.
- Both parsers reject non-canonical byte order, unknown/snake-cased fields, control characters,
  out-of-order or duplicated event sequences, and any receipt whose declared `requestId`/
  `commandSha256` does not bind to the request that asked for it.
- **Redaction is a parse-time law**: a non-`accepted` outcome carrying events or a capability is
  rejected by both parsers, not merely avoided by the server.
- `mint_directory_command_request_id()` / `mintDirectoryCommandRequestId()` mint the correlation.
  It is an idempotency correlation, never a capability.

New Rust helpers: `directory_command_sha256`, `DirectoryCommandRequestV1::{new,canonical_json,
validate,parse_canonical_json}`, `DirectoryCommandReceiptV1::{seal,canonical_unsigned_json,
receipt_matches,validate,parse_canonical_json}`, `DirectoryCommandErrorCodeV1::{as_str,from_status,
is_transient}`. TypeScript twins: `sealDirectoryCommandRequestV1`, `directoryCommandRequestJson`,
`parseDirectoryCommandRequestV1`, `sealDirectoryCommandReceiptV1`, `parseDirectoryCommandReceiptV1`,
`directoryCommandSha256`, `directoryCommandErrorFromStatus`, `directoryCommandErrorIsTransient`.

The types landed first (within the first ~20 minutes of the lane) and the sibling
`fable-space-administration` lane already consumes them by name in `🧵️backbone-worker.ts`
(`postDirectoryAdministrationState`, `DirectoryCommandErrorCodeV1`) and in the WGPU shell.

## 2. Hub: durable idempotency owner

`🌎️hub/📇️directory/🦀️.rs`
- `model`: `DirectoryCommandResultKindV1`, `DirectoryCommandDispositionV1`,
  `NewDirectoryCommandReceipt`, `DirectoryCommandReceiptCompletion`, `DirectoryCommandReceiptRecord`,
  `DirectoryCommandClaimV1 { Claimed | Existing | Conflict }`.
- `HubDirectory` (backend-neutral, **not** a route-local map) gains
  `claim_or_read_directory_command_receipt`, `complete_directory_command_receipt`,
  `release_directory_command_receipt`.
- `validate_directory_command_claim` admits bounded actor id, 32-hex nonzero correlation and
  64-hex lowercase digest before any backend touch.
- `DirectoryService::execute_idempotent` is the pipeline: **write lock → atomic claim-or-read →
  `decide` → `append_events` → durable `complete_directory_command_receipt` → only then publish**.
  Publication therefore never precedes a durable receipt. A `decide` failure (before any durable
  write) releases the claim; a failure after the durable append deliberately leaves the row
  `Pending`, which resolves as `secret-undeliverable`.
- `replay_directory_command_receipt` seals the redacted receipt any later resolution returns:
  `Completed` + result kind `None` → `previously-accepted`; everything else (`Pending`, or a
  completed secret-bearing command) → `secret-undeliverable`. The durable row stores that receipt's
  digest — a bounded canonical receipt digest, never a capability plaintext.
- `directory_command_result_kind` derives the result class from the command *before* execution, so a
  `Pending` row is already meaningful after a crash.

Backends (all three implement the operation, atomically, keyed `(actor_user_id, request_id)`):
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` — new `hub_directory_command_receipt` table
  (composite primary key, CHECK-constrained disposition/result kind/lengths), claim-or-read inside a
  single transaction, `complete` as a `disposition = 'pending'`-guarded UPDATE (so one claim
  completes exactly once).
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` — same table, `SELECT … FOR UPDATE` claim and a
  `RETURNING` completion.
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` — `DirectoryCommandReceipt` node with a new
  `REQUIRE r.key IS UNIQUE` constraint.

`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`
- `DirectoryCommandResponse`, its ad-hoc `ToValue`, and `command_result_value` (which returned a bare
  `DslValue` carrying the plaintext token) are **deleted**.
- `post_directory_commands` now takes raw `Bytes`, enforces the 8 KiB ceiling before decoding, parses
  exactly one canonical `DirectoryCommandRequestV1`, runs `resolve_bearer_user` **and**
  `authorize_directory_command` *before* any stored completion is consulted, and returns
  `DirectoryJson<DirectoryCommandReceiptV1>` (validated before it is written).
- `DefaultBodyLimit::max(DIRECTORY_COMMAND_REQUEST_MAX_BYTES)` is layered on that route only.
- The membership socket fence was factored into `directory_command_socket_binding` +
  `pause_directory_command_membership_fence`, shared by the pre-existing
  `execute_directory_command_fenced` (still used by the admin-intent path, which owns its own
  `AdminIntentV1` idempotency) and the new `execute_directory_command_receipt_fenced`.
- `409 request-conflict` for an equal key with an unequal digest; every denial has an empty body.

## 3. Clients (closed boundary, no `unknown`, no open `DslValue`)

TypeScript — `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `DirectoryCommandResult = { events; result?: unknown }` is **deleted**; `DirectoryCommandError`
  (a closed `code`, never server text) replaces it.
- `DirectoryClient.command(request, options)` posts `directoryCommandRequestJson(request)`, maps every
  non-2xx through `directoryCommandErrorFromStatus`, counts raw UTF-8 bytes *before* JSON decoding,
  and parses only a request-bound canonical receipt. `options.signal` is honoured before and after
  the wait.
- `BackboneWorkerRequest` gains `directory-command-cancel`; `BackboneWorkerResponse` replaces
  `directory-command-result` with typed `directory-command-receipt` and closed
  `directory-command-failed`.

Rust — `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `CommandOutcome { events, result: Option<DslValue> }` is **deleted**; `CanonicalDirectoryCommandReceiptV1`
  (exact response bytes + typed receipt) replaces it.
- New bounded `request_bytes_limited` caps raw bytes before decoding and never places a response body
  into a UI-facing error; `DirectoryClient::command` returns `Result<_, DirectoryCommandErrorCodeV1>`.
- The test `FakeTransport` now records request bodies (it previously discarded them), which is what
  makes the byte-identical-retry law checkable.

## 4. Browser transport owner

`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`
- `QueuedDirectoryCommand` is replaced by `DirectoryCommandTransportOperationV1`
  (sealed request, `AbortController`, `sessionEpoch`, `workerEpoch`, `settled`) held in a
  request-id-keyed map plus a FIFO, with `DIRECTORY_COMMAND_TRANSPORT_CAPACITY = 64`.
- Capacity rejection answers the **newest** intent with a terminal `capacity` result and never drops
  the oldest; `closeDirectory` bumps the worker epoch and answers every live operation with `closed`.
- `directory-command-cancel` aborts the HTTP wait and answers `cancelled` (indeterminate) — the id is
  retained, never silently reissued.
- Only `overloaded`/`transport` retry, re-sending the byte-identical sealed request; `401/403/409/413`
  and any validated server rejection are terminal. An operation whose session or worker epoch was
  replaced is suppressed rather than delivered or auto-replayed.
- `settleDirectoryCommand` retires an operation exactly once.
- `directoryRejectionStatus` was kept (the sibling administration lane's page path still uses it).

## 5. React / WGPU handoff (no pane)

`…/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `DirectoryCommandResultSlotV1`, `DIRECTORY_COMMAND_RESULT_SLOTS = 64`,
  `retainDirectoryCommandResult`, `mintDirectoryCommandRequestId` (all exported, and re-exported
  through the React barrel `…/🎯️targets/⚛️react/🟦️.tsx`).
- The message handler fills the retained request-id-keyed slot **first** and folds events only for an
  `accepted` receipt; a failure logs the closed code only. The replay path now mints a 32-hex
  correlation instead of an `actionId-timestamp-random` string.

`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `Vec<DirectoryCommand>` is replaced by an owned `NativeDirectoryCommandQueueV1`
  (`admit`/`admit_first`/`head`/`settle`/`result`/`pending`) holding sealed
  `DirectoryCommandRequestV1`s plus a bounded transient result slot
  (`NativeDirectoryCommandResultV1::{Receipt,Failed}`). Extracting the policy into an owned type is
  what makes it provable without a renderer, a surface, or a live hub.
- `flush_pending_directory_commands` stops at the first transient failure and keeps the head's exact
  bytes; a terminal auth/conflict/validation failure produces a result and lets the queue proceed.
- `directory_command_ctx` creates one `directory_cancel` child **with a finite 5 s deadline**
  (`DIRECTORY_COMMAND_DEADLINE_MS`), closing the "no deadline yet" gap the packet called out.
  The sibling administration lane already uses this context for its own page/command turns.

## 6. Tests and gate

Neutral vectors (byte-identical in both trees, and the gate asserts that):
- `🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/{🔣️.json,🧬️.schema.json}`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧾️command-receipt-v1.json`

4 request vectors (including an equal-id/different-digest conflict pair), 5 receipt vectors
(live invite delivery, redacted retry, one- and two-event acceptances, previously-accepted),
8 rejected requests, 12 rejected receipts (snake-case, unknown field, non-canonical order, tampered
receipt digest, substituted command digest, redaction violation, over-limit token, out-of-order and
duplicate events, >4 events, unknown outcome, 64 KiB+1), plus the transport status/transient/terminal
tables and six named transport traces.

Gate `os-hub:directory-command-receipt-check {source|native|process}`, registered in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` + `📋️project.json` + `.vscode/🧩️launch.seed.jsonc`
(launch output regenerated with the plugin-registry generator), mirroring
`directory-event-page-v1-check`.

- **source**: AJV (2020, strict) over the published fixture schema **and** an independent Node
  `createHash("sha256")` recomputation of every canonical request/receipt digest and byte length,
  with the repository's own TypeScript parser exercised as a third implementation (never as the
  oracle), plus the hub/os fixture byte-equality check.
- **native**: four hub laws in `🚀️bin.rs`, extending the existing route laws rather than mocks.
- **process**: two real authenticated sessions against a real SQLite hub.

Other suites extended: the schema Rust fixture law
(`directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles`), the Rust
client law (`directory_command_parses_only_a_bounded_canonical_receipt_and_never_echoes_server_text`),
the WGPU queue law
(`native_directory_command_queue_retains_a_transient_head_and_proceeds_past_a_terminal_failure`),
four new browser directory-lane cases, and one ShellHost slot case.

Also repaired in passing (a peer rename had left it dangling and it blocked the whole React suite
from loading): `🔬️index.test.ts`'s `boardSessionSchema` import now points at
`…/🌉️wasm/🧪️fixtures/🧬️.schema.json` (the file that exists) instead of `🔣️.schema.json`.

## 7. Verification — exact commands and results

### Green

| Command | Result |
|---|---|
| `bun ./📜️script.ts directory-command-receipt-check source` (in `🌎️hub/📦️packages/🦀️rust`) | `directory-command-receipt-check: checks=90 phase=source` — **90 assertions**, re-run green three times (13:32, 13:52, 14:01) including after the GIS-probe call-site conversion |
| `bun ./📜️script.ts test quick` (in `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`) | `Test Files 3 passed (3) / Tests 258 passed (258)`, 8.27 s; re-run at 14:26 after the last edit: `Tests 267 passed (267)`, 16.60 s |
| `bun ./📜️script.ts test long -t "directory lane"` (same package) | `Tests 6 passed | 252 skipped`, 899 ms — the four new transport laws plus the two pre-existing lane cases |
| `bun ./📜️script.ts test long -t "mints a 32-hex"` (in `…/🎯️targets/⚛️react`) | `Test Files 1 passed | 9 skipped / Tests 1 passed | 568 skipped`, 39.11 s |
| `bun ./📜️script.ts typecheck` (in `…/🎯️targets/⚛️react`) | The first run surfaced exactly **one** error belonging to this lane — `🧵️backbone-worker.ts(3842,132): error TS2552: Cannot find name 'DirectoryCommandResultV1'`, a missing type import in the new in-source test block. **Fixed**; the confirming re-run (captured at `🗑️generated/fable-directory-command-receipt/react-typecheck.txt`, `tsc-exit=2`) contains **68 `error TS` lines and zero occurrences of any symbol this lane introduced** (`grep -ciE 'DirectoryCommandRequestV1|DirectoryCommandReceiptV1|DirectoryCommandResultV1|DirectoryCommandOutcomeV1|DirectoryCommandErrorCodeV1|retainDirectoryCommandResult|mintDirectoryCommandRequestId|DirectoryCommandTransportOperationV1|sealDirectoryCommand'` → `0`). The remaining 68 are pre-existing and belong elsewhere: `FetchTimeoutResponse.body`, `SharedArrayBuffer`/`Bun` typings, `DocumentExecutionTargetLeaseFieldsV1` (execution-target lane), `🛂️SpaceAdministration/🟦️.tsx` (administration lane), and `📇️directory/🟦️.ts` view re-exports (`DirectorySpaceListEntryV1`, `MemberSpaceViewV1`, `PublicSpaceViewV1`, `PublicDocumentCatalogEntryV1`) dropped by the `DirectorySpaceDetailV1` retirement. This target is therefore already red independently of this work. |

The browser transport cases were briefly gated to `long` while the `quick` wall-clock budget was
being exceeded; once the box quietened the whole `quick` suite ran in 8.27 s, so the gate was removed
and all four run at `quick`. The earlier budget kills were peer load, not these cases.

### Not run / blocked

- **No `cargo` invocation completed at all in this lane.** Not the plain
  `cargo check -p semio-hub --bin os-hub --all-features --tests`, not the four hub laws, not the
  process gate, not the schema/client/WGPU Rust laws. An earlier note in this report claimed the
  hub check "ran to completion at 13:38 with no error line" — that was a misreading: the exit code
  I saw belonged to the *waiter* shell, not to cargo, and the cargo job was still queued on the lock.
  It was later killed while still queued. **Every Rust claim below is therefore "written, not
  built".** The hub laws and the process gate both need `runExactCargoLaws` to build the `os-hub`
  test binary. From ~13:40 onward the shared target
  directory has been held continuously by peers. At 14:30 the queue was:
  `cargo test -p semio-s-plugin-process --lib` holding `target/debug/.cargo-lock` (pid 5360, 57 min
  elapsed, live `rustc` child at 7% CPU — a legitimate peer build, not a stuck orphan), with
  `semio-s-plugin-lowpoly`, `-procedural`, `-puzzle`, `-norm`, `semio-hub --lib` and a
  `cargo check --quiet --workspace` all queued ahead of mine, ~33 `cargo check` processes total. My
  own `cargo check -p semio-hub --bin os-hub --all-features --tests` sat on
  `Blocking waiting for file lock on build directory` for over 40 minutes. The four hub laws and the
  process gate are therefore **written and registered but not executed** — see the nonclaims.
  Two attempts were made (13:23 and 13:49); **both were killed while still queued on the lock**,
  after ~75 and ~60 minutes respectively. The retained output
  (`🗑️generated/fable-directory-command-receipt/hub-check.txt`) contains exactly one line —
  `Blocking waiting for file lock on build directory` — which is the whole evidence trail.
- The Rust schema law, the Rust client law and the WGPU queue law are likewise **written but never
  compiled**; they live in `semio-framework-os-kernel` / the renderer crate, which the same lock
  contention blocked. A peer reported `semio-framework-os-kernel --lib` green at 13:21, but that is
  `--lib` only and not this lane's observation, so it says nothing about the three new `#[test]`s or
  about `--all-features`.
- Nothing else. `bun ./📜️script.ts typecheck` for the React renderer package **did** run (twice,
  ~17 min each under load) and is covered under Green below.

### External blockers observed (not this lane)

- `bun ./📜️script.ts directory-event-page-v1-check source` fails with
  `error: directory event page source oracle admitted removed fence 3`. Its mutation oracle does
  `postgres.replace("validate_directory_event_page_event(&full)", "Ok(())")`, and `String.replace`
  substitutes only the first occurrence — but `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` now has
  **two** such call sites (lines 1799 and 2017), so the mutant still closes the fence. That is the
  Sol event-page lane's own gate and its own postgres call site; this lane touched neither.
- `🔬️index.test.ts` could not load at all
  (`Failed to resolve import "…/🌉️wasm/🧪️fixtures/🔣️.schema.json"`); a peer had renamed the file to
  `🧬️.schema.json`. Repaired here because it blocked the whole React suite.

## 7b. File inventory

Changed:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs` (re-exports)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc` and the generated `.vscode/launch.json`

Created:
- `🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/🔣️.json`
- `🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/🧬️.schema.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧾️command-receipt-v1.json` (byte-identical mirror)
- this report

Generated output lives only under
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/fable-directory-command-receipt/`
(`hub-check.txt`). No other lane's generated folder was touched; `📋️master-plan.md` and
`✅️acceptance-matrix.md` were not edited.

## 8. Honest nonclaims

- **No replayable invite-token delivery.** There is no server-side confidential receipt vault, and
  `InviteRecord` still stores only `secret_digest`. A retry of a completed `create-invite` returns
  `secret-undeliverable`: it proves no duplicate was minted, and it cannot recover the link. The
  future author-admin page must show issued-invite metadata and offer revoke/new-invite.
- Cancelling an HTTP wait does **not** cancel a command already past its server linearization point.
  The retained operation keeps its `requestId` and becomes indeterminate; it may be explicitly
  resolved, never silently reissued under a fresh id.
- This packet implements **no** administration page or pane, no Home retained action, no invite
  redemption change, no scoped-4401 change, and no persistent offline command queue.
- The 64 KiB receipt ceiling cannot be reached from client input through the route (an 8 KiB request
  can only produce a far smaller receipt); the ceiling is proved at the parser level from neutral
  vectors and asserted structurally at the route.
- Native `cargo` never compiles `#[cfg(target_arch = "wasm32")]` code. The directory schema and
  client are consumed by wasm plugins; **no wasm target was built in this lane**, so the wasm side of
  the schema/client change is unverified here.
- **The strongest nonclaim: none of this lane's Rust has ever been compiled.** The schema twin, the
  three backend implementations, `DirectoryService::execute_idempotent`, the rewritten hub route, the
  bounded native client, the WGPU queue, and all six new Rust `#[test]`s are unbuilt. The TypeScript
  half is fully exercised (258→267 passing unit tests, 90 gate assertions, a clean lane-scoped
  typecheck); the Rust half is reviewed source only. The first person with the target-directory lock
  should run, in order:
  `cargo check -p semio-hub --bin os-hub --all-features --tests --message-format=short`,
  `bun ./📜️script.ts directory-command-receipt-check native`, then `… process`.
