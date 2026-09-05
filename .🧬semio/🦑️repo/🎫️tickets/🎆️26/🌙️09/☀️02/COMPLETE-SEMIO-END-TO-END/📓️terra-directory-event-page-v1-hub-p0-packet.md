# Authenticated Directory Event Page V1 — Hub P0 Packet

## Verdict

**RED: the shared `DirectoryEventPageV1` contract is implemented, but no authenticated hub endpoint produces it.** The router registers only the legacy `GET /directory/events` array endpoint; that handler accepts an optional unbounded-by-contract client `limit` (default `500`), reads before requiring a caller, performs visibility filtering without a receipt or post-read session revalidation, and returns a bare vector. It cannot safely drive the Home/Space cursor.

This packet is deliberately limited to `GET /directory/event-page/v1?after=<u64>`. Invite redemption and presence are excluded.

## Current, Source-Verified Boundary

| Boundary | Current source | Finding |
|---|---|---|
| Shared contract | [`🦀️.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:198), [`🟦️.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:132), [`🔣️.json`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json:173) | `DirectoryEventPageV1` requires exact camel-case canonical JSON, lowercase SHA-256 receipt, safe integers, strictly increasing visible event seqs, at most 128 events, 48 KiB/event, and 64 KiB/page. `parse_canonical_json` rejects noncanonical bytes. |
| Existing neutral/type proof | [`event-page-v1.json`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json:1), [`📜️script.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:47) | Existing AJV/Node SHA-256 and Rust-law registration prove a fabricated page, not hub authentication, scan, filtering, HTTP, or storage admission. No run was performed for this audit. |
| Legacy raw read | [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3832), [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3914) | `EventsQuery { since, limit }` and `get_directory_events` call `events_since(since, limit.unwrap_or(500))`, accept anonymous callers, and return only `visibility_filter_events`. It has neither receipt nor generation fence. |
| Router | [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5384) | `/directory/events` is registered; `/directory/event-page/v1` is absent. |
| Authentication seam | [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3448), [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3850) | `resolve_bearer_user` returns session id, user id, expiry, generation and retained capability. `caller_active` re-queries `authenticate_session` and compares id/user/generation. This is the correct starting point but no HTTP page uses it twice. |
| Visibility rule | [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3871), [`🚀️bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3888) | `event_visible` and `visibility_filter_events` already fail closed: a global event is private to its `user_id`; a space event requires current membership even for public spaces. They preserve output sequence order but do not define a raw cursor frontier. |
| Raw read/backend cap | [`🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:360), [`🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:469) | Existing `DIRECTORY_EVENT_READ_MAX` is 10,000. `events_since` does enforce dense ascending backend reads, but it is not the event-page cap. |
| Durable write seams | [`🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1679), [`🪶️sqlite/🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:453), [`🐘️postgres/🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1879), [`🌐️neo4j/🦀️.rs`](../../../../../../../../../🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1736) | Generic appends are transactional and fan out under `DirectoryService::write`, but none checks the 48 KiB public-event ceiling. SQLite also has the same persistence construction used by its reserved-checkpoint path at line 1525. |

### Decisive implications

1. `after` is a **raw-log cursor**, not a visible-event cursor. The existing fixture already allows a hole (`after=3`, event seq `5`, `through=5`), so hiding seq `4` must still advance `throughSeqInclusive` over it.
2. A 48 KiB event ceiling does **not** imply that 128 visible events fit in a 64 KiB page. The page builder must stop before the first *visible* row which would exceed the canonical page ceiling; it must not skip that row or set `through` beyond it.
3. Validating only `DirectoryService::append_and_publish_locked` is insufficient: direct backend `append_events` is contractual for rebuild/tests, and SQLite's reserved checkpoint path reaches `persist_event` directly. Admission belongs at the full `DirectoryEvent` persistence construction, before projection/commit, in each backend transaction.

## Smallest Coherent Implementation Packet

### 1. Establish one shared event-admission primitive

**Owner:** OS directory schema plus hub directory backends.

Add a public, schema-owned `validate_directory_event_page_event(&DirectoryEvent) -> Result<(), DirectoryEventPageErrorV1>` beside `DirectoryEventPageV1::validate` in [`🧰️framework/…/directory/🧬️schema/🦀️.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:247). It must share—not duplicate—the current per-event canonical JSON, control-character, 48 KiB, and safe-sequence checks. `DirectoryEventPageV1::validate` then invokes it for every event.

Invoke it after the backend has minted the complete `DirectoryEvent` (including dense `seq`, id, and recorded timestamp) and before projection or commit:

- SQLite: `SqliteDirectory::persist_event_with_identity` at [`🪶️sqlite/🦀️.rs:453`](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:453). Its enclosing transaction makes a post-insert validation failure non-durable; do not create a second nontransactional preflight.
- PostgreSQL: construct the full value immediately after `UPDATE … RETURNING seq` and validate it before its `INSERT` at [`🐘️postgres/🦀️.rs:1879`](../../../../../../../../../🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1879). A validation error aborts the transaction, including the head increment.
- Neo4j: construct/validate after the transactional counter increment and before `CREATE (:DirectoryEvent …)` at [`🌐️neo4j/🦀️.rs:1736`](../../../../../../../../../🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1736); error aborts `Txn` and counter mutation.

Keep the existing reserved-checkpoint flow on the SQLite persistence helper; its complete public event is therefore covered. This proposal does not change invite redemption or presence code.

Required result: an oversized event has no committed row, projection, sequence advancement after rollback, or broadcast. Do not lower the 48 KiB event limit to paper over the 64 KiB aggregate page cap.

### 2. Introduce an exact, private HTTP request/page builder

**Owner:** `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` near `EventsQuery` / `get_directory_events` (lines 3832–3917).

Add, with no alteration of the legacy endpoint:

```text
directory_event_page_request_admission(uri) -> Result<after: u64, StatusCode>
directory_event_page_session_binding_v1(AuthedUser) -> [u8; 32]
build_directory_event_page_v1(state, caller, after) -> Result<DirectoryEventPageV1, StatusCode>
get_directory_event_page_v1(OriginalUri, HeaderMap, State<HubState>) -> Result<DirectoryJson<DirectoryEventPageV1>, StatusCode>
```

`directory_event_page_request_admission` must accept exactly one canonical decimal `after` query field, no duplicate/unknown keys, no percent-decoding ambiguity, no leading zero except `0`, and `0..=DIRECTORY_WIRE_INTEGER_MAX`. It returns **400 with an empty body** before authentication or directory I/O.

`directory_event_page_session_binding_v1` must not hash a bearer string. Compute lowercase SHA-256 over this unambiguous preimage:

```text
"semio/hub/directory-event-page/session-binding/v1\\0"
u32be(byteLen(session_id)) || UTF-8(session_id)
u32be(byteLen(user_id))    || UTF-8(user_id)
u64be(authorization_generation)
i64be(expires_at)
```

This is a correlation receipt only; authorization remains the backend session lookup. The length-prefixing prevents concatenation ambiguity, and the domain prevents reuse as a socket, admin-cursor, or document-open digest. Use the existing `Sha256` pattern in `admin_page_cursor_hash` at [`🚀️bin.rs:4349`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4349), not an ad-hoc JSON hash.

`get_directory_event_page_v1` must:

1. Admit query; resolve bearer through `resolve_bearer_user`, else **401 empty body**.
2. Read **once** with `state.directory.events_since(after, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS)`. It must never use the legacy 10,000-row cap or `head_seq`; `after` is the raw frontier.
3. Re-resolve the same retained session capability after the read. Require exact id, user id, authorization generation, and recomputed binding equality. Failure, expiry, revocation, or lookup absence is **401 empty body**. A directory read failure is **500 empty body** under the existing `DirectoryError::Backend` mapping.
4. Apply the existing fail-closed membership filter to the scanned rows using the revalidated caller. Invisible rows consume raw cursor space but never appear in `events`.
5. Build the result in raw sequence order. Start `through=after`. Each invisible raw row may advance `through`. For each visible row, trial-encode the *candidate full page* using the actual shared canonical serializer/receipt calculation. If including it would exceed 64 KiB, stop before that row, keep `through` at the preceding raw sequence, set `hasMore=true`, and leave it for the next request. Never silently omit a visible row while advancing past it.
6. If all scanned rows were consumed, set `through` to the last raw seq (or `after` for an empty scan) and `hasMore = raw_len == 128`. This is a precise bounded-scan signal: a saturated raw scan may yield one harmless empty follow-up page, but cannot skip a row appended concurrently. If output capacity stopped the scan, `hasMore=true` irrespective of raw count.
7. Construct receipt with `schema="semio.directory.event-page.v1"`, binding digest, revalidated generation, raw `after`, calculated `through`, `hasMore`, and visible prefix; calculate `receiptSha256` from `canonical_unsigned_json`; call `validate` before `DirectoryJson` writes canonical bytes.

The final session revalidation linearizes the page against session revocation/generation change. Membership is evaluated from current reads after that revalidation and is never taken from a client claim. A membership change immediately after the final lookup is a normal post-linearization race; closing an existing scoped socket is a separate packet and is not claimed here.

### 3. Register the route and keep status/body semantics closed

Add exactly:

```text
.route("/directory/event-page/v1", get(get_directory_event_page_v1))
```

adjacent to `/directory/events` in [`🚀️bin.rs:5391`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5391). It is authenticated; unlike legacy `/directory/events`, it must never return an anonymous empty page that could be mistaken for a receipt.

| Situation | Response |
|---|---|
| malformed, duplicate, unknown, noncanonical, overflow query | `400`, empty body |
| no/invalid/expired/revoked bearer; post-read session id/user/generation/binding mismatch | `401`, empty body |
| directory backend failure | `500`, empty body |
| successful bounded scan | `200`, `application/json`, exact canonical `DirectoryEventPageV1` bytes |

Do not add an error JSON shape, a legacy `since` alias, caller-selected `limit`, a public-event fallback, or a client-supplied binding/generation.

## Acceptance Matrix

### Schema-first neutral corpus

Add a distinct fixture family under `🌎️hub/🧪️fixtures/📇️directory/event-page-route-v1/{🧬️.schema.json,🔣️.json}` and a Bun/AJV/SHA-256 oracle in the existing hub [`📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:1). Keep the existing shared contract fixture unchanged.

Rows must pin:

1. `raw-holes`: raw `[11 hidden,12 visible,13 hidden,14 visible]`, `after=10`, `through=14`, visible `[12,14]`, receipt/binding exact.
2. `128-hidden`: 128 raw invisible rows, no events, `through=after+128`, `hasMore=true`; proves raw rather than visible scanning.
3. `page-byte-prefix`: two individually admissible visible events where the second would take full canonical output beyond 64 KiB; first response stops before it and the next response returns it. This guards the aggregate-cap bug.
4. empty, one-row, exact-48-KiB accepted, and 48-KiB-plus-one rejected-before-durability.
5. malformed `after`, duplicate `after`, unknown query key, unsafe integer, missing bearer, revoked-after-read, changed authorization generation-after-read, receipt substitution, raw invisible identity leakage, and backend failure.

The oracle must calculate the length-prefixed session-binding preimage and SHA-256 independently using Node `createHash`; it must not import the endpoint helper. It also must pin all status/body pairs above.

### Native exact laws

Add to `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` tests, reusing `test_state`, `issue_test_session`, `spawn_server`, and `raw_http_request` at lines 6004, 6227, and 6292. Suggested exact law names:

1. `directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility` — actual SQLite state, 128 raw scan, member versus outsider, exact bytes parsed by `DirectoryEventPageV1::parse_canonical_json`; assert no hidden actor/body text appears.
2. `directory_event_page_v1_route_revalidates_session_generation_after_read_before_response` — a test-only read fence pauses after `events_since`; revoke/rotate the session; resume; require 401 and empty body, never a partial page.
3. `directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq` — exact page then continuation; assert every visible sequence appears once and only once across the two receipts.
4. `directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j` — backend-gated implementation laws at the three persistence seams: 48 KiB exact commits/projection succeeds; plus-one errors; head, events, projection, and `DirectoryService` broadcast remain unchanged. This is a backend law, not a mocked source check.
5. `directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body` — all 400/401 paths have zero bytes and the directory event read counter remains zero for bad query.

Use a bounded test-only `DirectoryEventPageSource` / read fence rather than sleeps. It must own no production branch or compatibility switch.

### Process gate and registration

Extend the hub [`📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:2455) with one `directory-event-page-v1-check` command offering `source`, `native`, and `process` phases, following `scoped-directory-socket-check`'s phase pattern at lines 2464–2518:

- **source:** AJV/Node oracle and exact source route/admission markers only;
- **native:** the five selected Rust laws with `runExactCargoLaws`, `--bin os-hub`, `--all-features`, `--exact --test-threads=1`;
- **process:** build/spawn the real SQLite `os-hub` binary, issue two normal test sessions through existing local authentication, use real HTTP to issue commands/events and query the page. Assert raw holes, 128 scan, canonical receipt, post-read revocation 401/empty, continuation, and restart persistence. Register the launch/Nx entry by editing its source seed rather than any generated launch file.

The existing shared command remains the type gate:

```text
bun nx run @semio-tech/framework-os-kernel:directory-event-page-contract-check -- --native
```

The proposed hub command is intentionally separate because it proves server session/visibility/durability behavior that this current command cannot.

## Honest Nonclaims

- No native, process, browser, or SQLite command was run for this audit.
- The current shared contract/oracle does not prove a live HTTP route.
- This packet does not claim invite redemption linearization, presence expiry, scoped WebSocket 4401 behavior, or browser retained event-page ownership. It supplies the authoritative HTTP page they can consume.
- It does not make the legacy `/directory/events` route safe; that route remains distinct until explicitly removed in a separately coordinated breaking change.

