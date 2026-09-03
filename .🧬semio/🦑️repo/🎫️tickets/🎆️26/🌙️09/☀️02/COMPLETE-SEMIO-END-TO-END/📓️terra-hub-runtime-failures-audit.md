# Hub Runtime Failures Audit

## Scope and evidence

Read-only audit of the active acceptance/plan and the current hub, DB-I/O, actor-return, and replication-wire sources. I also read `📓️sol-hub-security.md`, `📓️sol-document-descriptor.md`, `📓️sol-db-compile-gate.md`, and the coordinator-reconstructed `📓️sol-hub-six-compile-gate.md`. The last is supplemental evidence rather than an independently produced executor report.

One low-cost direct reproduction is current: Node `v24.15.0` running `node --experimental-strip-types 🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts` fails at line 57 with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` / “TypeScript parameter property is not supported in strip-only mode”. I did not claim a fresh Rust test pass: a prior narrow compilation attempt was stopped before execution to avoid competing Cargo work, and its output is not evidence.

## Findings

### H1 — `PayloadStorage::get` faults in the generic page lifecycle, not in Hub blob lowering

`🌎️hub/📦️packages/🦀️rust/📦️bin.rs` PUT succeeds, then `PayloadStorage::get` reaches `DbIoPageWriter::seal_retained_step` in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs`. During its phase 2, the retained writer changes one output page at a time from `Executing` to `TerminalResult` and returns `Yield`. The generic blocking driver immediately follows every yield with `task_owner.transition_pages(Executing, Queued)`. It therefore observes the first page in `TerminalResult`, not `Executing`, and faults.

The reported `stale generation: expected GenerationId(35), got GenerationId(35)` is misleading but informative: every page lease guard combines generation, operation, and phase, yet every mismatch becomes `DbError::StaleGeneration` and prints only the generation. Equal values rule out stale reuse; the hidden failed identity is the page lease’s phase (and the same guard also verifies its operation identity). The precise incompatible expectations are writer sealing `Executing → TerminalResult` versus the driver’s subsequent expected `Executing → Queued`.

This is backend-neutral DB-I/O ownership: it can affect SQLite, memory, and filesystem output operations using the retained writer. It is not a Hub handler, HTTP-byte-lowering, or SQLite SQL fix. Existing payload laws cover memory and filesystem but omit SQLite; that gap let the hub’s SQLite payload GET expose it.

### H2 — the apparent share/descriptor/private-directory failures are a post-authorization storage regression

The handler order in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` is definitive:

1. `resolve_auth` sends `ServerFrame::Error { code: "unauthorized" }` for denied callers.
2. The durable descriptor lookup sends `document-not-announced` when absent and `directory` on lookup fault.
3. Schema or pack-hash disagreement sends `schema-hash-mismatch`.
4. Only then `ensure_document`, `db.hello`, `take_welcome`, and bootstrap work run; all their faults send `ServerFrame::Error { code: "storage", .. }` before any `Welcome`.

That means the reported focused pattern—share 3/4, descriptor 2/3, private realtime 0/1 with a non-`Welcome` first server frame—does not demonstrate a share-auth, descriptor, or privacy-policy regression:

- In `share_token_is_scoped_read_only_and_revocable`, tokenless and cross-space cases must still receive `unauthorized`; the scoped, valid share holder must receive `Welcome` and only then `Session`. Its non-`Welcome` is the shared storage path. Do not weaken this assertion to accept `storage`.
- In `document_open_rejects_missing_or_conflicting_descriptor_before_db_creation`, missing must stay `document-not-announced`, a conflicting hash must stay `schema-hash-mismatch`, and both must leave the DB document absent. The third, valid announced hello must stay `Welcome`; it alone enters the broken DB path.
- In `directory_ws_isolates_private_realtime_activity_and_global_identity`, the private author’s first valid document hello must be `Welcome` then `Session`. Its current failure happens before the subsequent presence and isolation oracles, so privacy is not yet re-validated either way.

The exact underlying storage message on each WebSocket has not been freshly captured in this audit; the handler proves the frame code is `storage`, while the prior blob reproduction supplies the equal-generation diagnostic. Keep those two evidence levels distinct.

### H3 — aggregate Hub build cannot start Rust because native TypeScript cannot strip a parameter property

`🌎️hub/📦️packages/🦀️rust/📜️script.ts:22` runs `bun nx run os-hub-admin:build` before `runCargo` at line 23. The admin Vite target’s native TypeScript path reaches `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:57`:

```ts
constructor(readonly bytes: Uint8Array, maximum: number) {
```

Node’s strip-only loader rejects parameter properties before Rust is invoked. The local standards-compliant repair is to declare `readonly bytes: Uint8Array;` on `ReturnReader`, accept `bytes: Uint8Array` as an ordinary constructor parameter, and assign `this.bytes = bytes` in the constructor. Do not mask this by changing loader/configuration behavior; the source must remain executable by the native strip-only path.

### M1 — rejection-message encoding has no response-byte bound or cooperative cancellation/yield point

`encode_messages` in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` eagerly allocates a `DslValue::Array` for every `MutationMessage` and serializes it to a new `Vec<u8>`. `MutationMessage` has public unbounded `message: String` and `target: Vec<String>` fields (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`), while `ApplyOutcome::Rejected.messages` is an unrestricted `Vec<u8>` and replication `write_bytes`/`read_bytes` has no field ceiling. Thus a rejected mutation can produce an unbounded outbound `Ack`, with neither cancellation nor a yield while materializing/serializing it.

The first-party JSON parity itself is good: `mutation_message_payload_matches_language_neutral_fixture` parses the emitted bytes, compares the language-neutral fixture value, and round-trips through `FromValue`. Preserve that oracle. The missing constraint is a resource/transport invariant, not a JSON-shape defect.

### M2 — blob GET error mapping and copying are not uniformly bounded/cancellable at the HTTP seam

`db_io_pages_into_http_bytes` correctly caps the post-GET page owner at `HUB_BLOB_MAX_BYTES` and closes pages one step at a time with a yield on both success and over-limit paths. Source review found no direct retained-page leak in those normal paths.

Two actionable gaps remain in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`:

- `get_blob` maps `DbError::NotFound` to 404 but maps every other storage failure to 500, bypassing its own `db_error_status`; `LimitExceeded` and `Unavailable` therefore lose their 400/503 classification.
- The page-to-`Vec` fragment loop has no cancellation signal or cooperative yield before it starts retirement. It is presently constrained by storage’s 496 KiB read ceiling, but the Hub’s advertised 1 MiB cap is larger and the function accepts arbitrary `DbIoPages`; the resource contract should be explicit rather than dependent on current backend limits.

## Dependency-ordered Sol packets

### P0-A — DB retained-writer lifecycle repair (independent of artifact bootstrap and framework-codec)

**Own only:** `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs`, plus the SQLite storage law test surface under `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs` if that module owns SQLite construction.

Make generic task phase transitions compatible with a writer that has progressively sealed terminal result pages, or defer those per-page terminal transitions until the driver can atomically publish them. Do not change a phase/operation mismatch into a false success. Improve the error taxonomy/diagnostic so it reports expected/actual generation, operation, and phase; equal generations must be visibly distinguishable from actual stale reuse.

Add a direct non-empty `PayloadStorage::put → get` law for SQLite beside the existing memory/FS laws, with page-boundary payloads (1 byte, exactly one page, one-page-plus-one) and missing hash. Oracle: returned bytes, content hash, `contains`, `len`, and a before/after DB-I/O ledger witness proving all result pages are retired. Include cancellation while a retained output writer is sealing and assert it also returns the ledger to baseline. Then run:

```sh
RUST_MIN_STACK=16777216 cargo test --manifest-path '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml' --all-features sqlite_storage_satisfies_payload_storage_laws -- --exact
RUST_MIN_STACK=16777216 bun nx run os-hub:test-quick -- blob_put_get_head_round_trip
```

There is no current named Nx project for the DB Rust package; the manifest-scoped command is the exact owner test until a properly routed repository target exists. The Hub test is the integration oracle; the SQLite law is the minimal root-cause oracle.

### P0-B — native-strip-only actor-return source repair (independent of artifact bootstrap, framework-codec, and P0-A)

**Own only:** `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts` and its existing actor TypeScript test surface/fixture only if needed.

Replace the parameter property with an explicit field and constructor assignment without changing ReturnReader byte parsing behavior. Add a regression that executes the actual Node strip-only entry/import successfully, while retaining the existing actor-return fixture vectors as behavior oracle. Gates, in order:

```sh
bun nx run @semio-tech/framework-actor:test-quick
bun nx run os-hub-admin:build
bun nx run os-hub:build
```

The final command establishes that Cargo is reached only after the admin build succeeds.

### P1 — restore Hub handshake/blob acceptance after P0-A (depends on P0-A; independent of artifact bootstrap/framework-codec)

**Own only:** `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` test region, unless P0-A exposes a correct production change that needs no Hub code modification.

Do not alter expected successful frames. Add precise first-frame assertions for every branch: valid share holder and valid descriptor are `Welcome`; missing descriptor is `document-not-announced`; hash/schema conflict is `schema-hash-mismatch`; tokenless, cross-space, and revoked share are `unauthorized`; any intentionally injected DB fault is `storage`. In the private-directory test, require `Welcome`, then `Session`, before testing presence/event isolation. Re-run the focused tests and only then report fresh counts.

Fixture/oracles: retain `🧪️fixtures/🧬️hub-boundaries/🔣️.json` for blob bytes/hash; use the existing live WebSocket encoder/decoder and `next_server_frame` as the language-neutral wire oracle. The direct `PayloadStorage` SQLite law in P0-A is the separate storage implementation oracle.

### P2 — bound/cancel outbound rejection data (framework-codec coordination required; no artifact-bootstrap dependency)

**Hub-owned seam:** `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`. **Shared contract seam:** `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` and `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`.

Agree one protocol-level maximum for rejected-message bytes/count and enforce it before allocation/copy on decode and before outbound frame construction. Hub encoding must preserve valid canonical JSON under truncation/rejection; it must never splice a JSON byte prefix. Make the work resumable/cancellable at a bounded message/page interval. Keep the exact fixture JSON semantic equality and `FromValue` round-trip, then add max-minus-one/max/max-plus-one and cancellation tests. This lane can develop in parallel with P0-A/P0-B, but its final API/codec decision must coordinate with the framework-codec owner.

### P3 — tighten Hub blob response seam after P0-A (depends on P0-A for end-to-end test; independent of artifact bootstrap/framework-codec)

**Own only:** `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` and its existing blob tests.

Route every `get_blob` DB error through `db_error_status`, add an explicit bounded copy/retirement control that yields and observes request cancellation, and assert page retirement on success, limit error, cancellation, and storage fault. Preserve bytes/hash/HEAD oracle from `hub-boundaries` and retain an exact status oracle for 404, 400-limit, and 503-unavailable.

## Completion evidence needed

P0-A and P0-B may run concurrently. P1/P3 need P0-A’s real payload GET success; aggregate `os-hub:build` additionally needs P0-B. P2 has no artifact-bootstrap dependency but must not race a framework-codec contract change. No fresh passing claim should be made for the share, descriptor, private-directory, blob, or aggregate Hub gates until these ordered checks have executed successfully.
