# Two-User GIS Inference Acceptance Packet

## Verdict

**RED — no server-owned inference capability exists yet.** This is a source-only audit on
2026-09-04; no build or runtime gate was run. The narrowest honest packet is a new hub-owned,
SQLite-backed `InferenceJobLedgerV1` and router which invokes the already deterministic GIS native
service through a new injected production port. It must not present MCP discovery, the process-global
native registry, or the plugin-host test runtime as an authenticated/durable inference path.

The packet proves two authenticated Authors in the same private space: Alice alone obtains her
private deterministic result and proposal, Alice approves it at its recorded frontier, and Bob sees
exactly the resulting ordinary document mutation through the existing document socket. Bob never
reads Alice's result or approves her private proposal. There is no network, paid model, or fake
authentication in this acceptance boundary.

## Current-tree evidence and reusable seams

| Concern | Current source evidence | Consequence for the packet |
| --- | --- | --- |
| Hub HTTP authority | [`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1908) authenticates a session or share for document sockets; [`issue_document_open_plan_inner`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1971) bounds bytes, checks the descriptor, revalidates authority, and resolves the selected catalog item. | Reuse its bounded-header/body, descriptor lookup, and before/after revalidation shape, but do **not** reuse the helper unchanged: it admits a share subject, while inference must require a live session with `SpaceRole::Author`. |
| Existing member decision | [`authorize_directory_command`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3551) gives document announcement only to an Author. [`resolve_bearer_user`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3333) resolves a live session. | Add one `authenticate_inference_author` beside the document helper: parse exactly one bounded bearer, authenticate session, read the scope role under a 2-second deadline, require `Author`, and reject Share/Invite/Spectator before a job id, slot, or executor call exists. |
| Real document transport and Bob's visibility | [`router`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5133) already registers the document socket. `HubState.fanout` is explicitly a thin relay of mutations that `db` has committed ([`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1376)). | The approval endpoint must submit the ordinary typed GIS mutation through the existing document commit path, then observe Bob's normal socket fan-out. Do not create a parallel “AI mutation broadcast.” |
| Directory event store | [`DirectoryEventBody`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:152) contains only users/spaces/members/documents/checkpoints/retention; [`DirectoryCommand`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:202) has no inference intent. | Do not smuggle private result bytes into a directory event. Add an inference-specific durable event/ledger and a private result projection; only the approved **document** mutation becomes collaborator-visible. |
| Existing async-operation pattern | `HubState` owns only `admin_operations` ([`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1341)); `AdminOperationRuntime` exposes atomic progress/cancel ([`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:4485)) and `AdminOperationCleanup` releases the slot/removes runtime ([`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:4513)). | Copy only the bounded-control/RAII shape. An inference job must have its own actor, ledger, capacity, visibility, idempotency and cancellation semantics; reusing an admin runtime would create wrong authority and audit scope. |
| Actual deterministic GIS executor | [`gis_map_inference_service`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:201) returns the native service. It rejects empty cancellation ids, zero/over-budget requests, incremental cache requests, malformed packs, deep/excessive work, and overlarge results at [`:220`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:220)-[`318`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:318). | This is the acceptance executor: no model account or outbound connection. It must receive a canonical snapshot selected by the server and produce a server-validated proposal, never receive a client-selected service/function pointer. |
| Native service abstraction | `ArtifactInferenceExecutionRequest` has policy/budgets/cancellation/payload/dependencies ([`plugin/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1291)); the registry is a process-global `OnceLock<RwLock<...>>` ([`:1384`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1384)) and `infer_artifact` merely calls its service ([`:1468`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1468)). | Reuse the request/result wire values as an **adapter payload**, but add a hub-local `InferenceExecutorV1` trait in `🌎️hub`; do not make global registration the hub's catalog, authority, lifecycle, or test seam. |
| Plugin-host router | [`ArtifactInferenceRouter`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6826) validates roster ownership and a revision/generation echo before exposing a guest result. | Its metadata-echo/freshness principle is useful, but it is a local plugin-host dispatch router, not a Hub REST/SQLite job service. `MockGuestRuntime` is test infrastructure and is forbidden as this acceptance executor. |
| MCP status | [`mcp/💡️inference/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1) explicitly states that execution is not reachable. It exposes only inert `InferenceJobPayload` ([`:180`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:180)) and terminates declared execution as `channel.not-wired`. | MCP stays out of P0. It may bind the hub routes in a later packet only after the server packet has real session/space/document authority. |
| Catalog/readiness | The hub manifest currently links stdio and plugin host but not GIS ([`Cargo.toml`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:39)). The only inference flag is currently false in hub readiness (see previous audit; router itself has no inference route). | P0 adds exactly one compiled GIS provider receipt/binding to the selected catalog. `readyz.features.inference` remains false unless the entire selected binding set validates atomically. No generated registry row is authority. |

## P0 production packet: server-authoritative deterministic GIS proposal

Implement this packet in the following order. It is deliberately SQLite-first and must fail closed
on other configured backends until their concrete ledgers exist; it must not claim PostgreSQL,
Neo4j, browser UI, WGPU, or MCP support.

1. **Schema and storage.** Add versioned `InferenceJob*V1` types under the hub's schema/directory
   domain, rather than extending generic directory events. A job's immutable accepted identity is:
   `job_id`, `request_id`, `request_digest`, requester `user_id`, `session_id`,
   `authorization_generation`, `DocumentScope`, descriptor digest, selected catalog generation,
   package hash, GIS artifact/inference schema and algorithm/policy versions, baseline frontier,
   input SHA-256, and a server `expires_at_ms`. Store bounded policy/input and private result/proposal
   bytes separately from shared directory events. Persist only state transitions
   `accepted → running → {succeeded | failed | cancelled}` and proposal
   `none → offered → {approved | stale | cancelled}`. All ids/digests are lower-hex fixed length;
   schema rejects unknown fields, duplicate keys, invalid enums, unsafe integers, oversize strings,
   and timestamps beyond the server cap.

2. **Authority and linearization.** Add `InferenceJobLedgerV1` plus a per-document actor/mutex to
   `HubState` adjacent to `document_open_plans` at
   [`📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1341). Under that
   actor, authenticate/revalidate Author A, fetch the exact descriptor, snapshot the frontier and
   selected catalog receipt, then establish request-id idempotency durably before reserving work.
   Same `(request_id, requester, identity digest)` returns the same receipt; same request id with
   any different authority/scope/catalog/frontier/input digest is `409`; a full ledger/slot is
   `503` and creates no row. Recheck session generation and role immediately before executor start
   and under the final transition/approval lock. Session revoke, member removal, scope delete,
   catalog generation change, expiry, deadline, and explicit cancel all win before publication.

3. **Bounded executor port.** Create a production `InferenceExecutorV1` whose call takes only a
   server-built immutable request and an `InferenceOperationControlV1` (deadline, atomic cancel,
   monotonic bounded progress). Its selected `GisMapExecutorV1` adapts only
   `gis_map_inference_service()` and verifies every returned metadata field and result byte bound.
   Its catalogue binding compares descriptor owner/package hash/schema/version against the linked
   GIS receipt before call. The executor has no network interface or credentials. This is an
   injected interface in tests; the test double implements the same production trait and is a
   blocking deterministic gate, not `cfg(test)` code in the route.

4. **Private result, deterministic proposal, and approval.** `succeeded` stores result bytes and
   their hash only in the requester's private job projection. Convert the GIS result deterministically
   into one typed, bounded `GisMapMutation` proposal containing the exact base frontier, candidate
   mutation bytes/hash, result hash, proposal expiry and requester identity. A public/member stream
   can expose only a coarse document-changed event after approval; it cannot expose job ids,
   inputs, diagnostics, result/proposal bytes, or progress. `POST .../approve` accepts only the
   requester A's live Author session, reloads the job under the same document actor, requires
   `offered`, unexpired matching descriptor/catalog/frontier and exact proposal hash, then commits
   through the existing document command path. Commit success and `approved` are one durable
   transaction/linearized critical section; all error/cancel/stale paths erase temporary result and
   create neither map mutation nor fanout.

5. **Routes.** Add strict JSON-only bounded routes beside the document routes in
   [`router`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5133):

   - `POST /spaces/{space_id}/documents/{id}/inference/jobs`
   - `GET /spaces/{space_id}/documents/{id}/inference/jobs/{job_id}`
   - `POST /spaces/{space_id}/documents/{id}/inference/jobs/{job_id}/cancel`
   - `POST /spaces/{space_id}/documents/{id}/inference/jobs/{job_id}/approve`

   Reject query strings, multiple/missing `Authorization` and `Content-Type`, bodies over the
   schema cap, Shares/Invites/Spectators, scope/descriptor mismatches, and unselected GIS services
   before allocation. `GET`/cancel/approve require the original requester and the same scope;
   B sees only the normal committed document socket frame. P0 sends progress only to A through a
   bounded authenticated job-status response (polling), avoiding a new raw directory stream.

## Minimal language-neutral fixture

Put one schema plus one canonical JSON fixture under a new hub-owned
`🌎️hub/🧪️fixtures/🧬️gis-inference-job-v1/` tree. The oracle must parse the JSON with AJV and
independently compute the deterministic proposal hash from a tiny canonical GIS snapshot; it cannot
import Rust codecs, use the service registry, or call a network model. The fixture's two session rows
are fixed `alice` and `bob`, both Author members of `space-gis`, with one private `doc-gis`, a fixed
descriptor/package hash/catalog generation/frontier, and a `DeterministicGisExecutorV1` script.

| Row | Setup/action | Required observation |
| --- | --- | --- |
| `success-two-user` | A submits a valid GIS request; executor reports `1/2`, `2/2`, returns the known result; A approves the known proposal; B has an already-authenticated document socket. | One executor call; A alone can read terminal private result/proposal; exactly one normal document mutation and one B socket commit; no inference bytes in B/status/directory event output. |
| `outsider-denied-before-ledger` | C has a valid session but no membership. | 403/404 per non-disclosure policy, zero accepted jobs, executor calls, progress, proposal, and fanout. |
| `share-and-spectator-denied` | A share capability and a Spectator session submit the same body. | Both fail before id/request allocation; proves the open-plan share helper was not reused as write authority. |
| `duplicate-request-single-winner` | Two concurrent A submissions share a request id and exact body. | Same receipt/job id, exactly one durable accepted event and executor call. Altering one identity/input field with that request id is conflict and still one call. |
| `cancel-before-start-no-partial` | Blocking deterministic executor waits after admission; A cancels before release. | Terminal `cancelled`, no result/proposal/document mutation/fanout; release cannot resurrect it. |
| `membership-revoked-before-completion` | Remove A's Author membership while the executor is held. | A completion is not stored/published; no proposal or mutation. This uses the same session/role revalidation shape as the socket grant boundary. |
| `frontier-changed-before-approval` | A gets a proposal; another legitimate document command advances the frontier. | Approval becomes `stale`, persists no document mutation, and B sees only the unrelated mutation. |
| `capacity-and-boundary` | Fill the fixed job capacity, submit oversized/deep input, and make executor return oversized/mismatched metadata. | 503/400/failed terminal respectively; no partial private/public row, no external call, no fanout. |
| `cross-user-private-read` | B guesses A's job/proposal id and correct hashes. | B receives no private job/progress/result/proposal, cannot cancel/approve, and does not learn whether guessed id exists. |

The fixture has nine rows because they cover the independently meaningful normal, authority,
idempotency, cancellation, revocation, stale-frontier, capacity/bounds, and privacy transitions.
No provider row may contain a URL, API key, environment lookup, time-dependent random output, or
network permission.

## Exact registered acceptance gate

Add `gis-inference-job-check` to the existing hub script, not an ad-hoc script:
[`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2968)
already shows the required independent-oracle, `cargo --list` exact-one selector, exact-run and
all-feature-check pattern. Register it in the existing script router at
[`📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3311) and its
normal Nx project/launch generation path. The eventual command is:

```sh
bun nx run os-hub:gis-inference-job-check --skip-nx-cache
```

The script must, in order:

1. execute the JSON Schema + independent Bun deterministic-GIS oracle (all nine rows);
2. list and require exactly one FQN for each focused real-router law, then exact-run it with
   `--test-threads=1`;
3. run two real bearer sessions against `router(test_state)` backed by SQLite and the production
   `InferenceExecutorV1` test implementation, using a semaphore only to hold the actual executor;
4. run `cargo check --manifest-path Cargo.toml --all-features --bin os-hub` separately;
5. assert zero outbound provider calls by construction: the only test executor is the deterministic
   local GIS implementation/trait object and its test recorder count is one only in the positive row.

Focused suffixes should be exact-one selected, not substring-filtered:

- `gis_inference_two_author_success_is_private_until_approved_document_commit`
- `gis_inference_rejects_share_spectator_and_cross_user_before_ledger_or_executor`
- `gis_inference_request_id_is_single_winner_and_identity_conflicts_fail_closed`
- `gis_inference_cancel_revocation_and_late_completion_publish_no_partial_result`
- `gis_inference_frontier_catalog_and_descriptor_mismatch_make_proposal_stale`
- `gis_inference_bounds_capacity_and_executor_echo_mismatch_fail_without_publication`

Add a launch registration only after the target is real; it invokes the one Nx gate above and must
not masquerade as browser/MCP/WGPU acceptance. P0's nonclaims are explicit: no LLM/model provider,
no MCP `artifact-infer`, no public/share inference, no cross-backend claim, no browser GIS command,
and no recovery/retry policy beyond a terminal/read-only query of the SQLite-backed job.

## Sol handoff order

1. Hub schema/SQLite ledger and its neutral fixture/oracle.
2. Session-Author guard plus per-document actor, bounded state machine and routes.
3. Catalog-bound deterministic GIS executor adapter and atomic proposal/ordinary-document commit.
4. Exact real-router two-user laws, then script/Nx/launch registration.

Do not start at the MCP facade: its current `channel.not-wired` response is correct. Do not register
GIS globally or trust a generated registry row as proof that the selected hub catalogue can execute
it. The source currently uses a changing normalization tree; revalidate the linked GIS receipt,
descriptor package hash and the exact hub manifest immediately before implementation.
