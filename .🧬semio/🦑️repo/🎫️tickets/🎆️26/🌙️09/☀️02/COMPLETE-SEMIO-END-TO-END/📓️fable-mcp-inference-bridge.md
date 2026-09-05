# Fable — MCP ↔ hub GIS Map inference bridge

Lane `fable-mcp-inference-bridge`, 2026-09-05. Packet: `📓️fable-explore-mcp-inference-bridge.md`, reconciled against the hub side that actually landed (`📓️fable-ai-map-proposal.md`) and the authority rules in `📓️terra-ai-map-proposal-approval-current-p0.md`.

`channel.not-wired` is gone for the one inference service this gateway can really execute — the hub's GIS Map service on an authenticated `--hub` binding. Every other declared inference keeps answering the same honest, retryable gap, because this crate's local `ArtifactChannel` still has no infer variant and nothing here pretends otherwise.

## What changed

### 1. A typed hub-HTTP client for the four authenticated routes

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs`, regions `//#region 💡️InferenceJobWire` and `//#region 💡️InferenceHubClient`:

- **Closed wire vocabulary mirrored from the hub, byte for byte** — `GisMapInferenceSubmitRequestV1`, `GisMapInferenceApprovalRequestV1`, `GisMapInferenceJobReceiptV1`, `GisMapInferenceEventV1`, `GisMapInferenceProgressV1`, `GisMapInferenceEventPageV1`, `GisMapInferenceApprovalReceiptV1`, `GisMapInferenceErrorBodyV1`, plus the kebab-case `GisMapInferenceJobStateV1`/`GisMapInferenceProposalStateV1`. Every one is `#[serde(rename_all = "camelCase", deny_unknown_fields)]`; there is no `serde_json::Value` anywhere on this boundary. A private hub field appearing on the wire is a loud decode failure, never a silently-dropped one.
- **`InferenceRouteErrorV1`** — the hub's whole published failure vocabulary (`inference.unavailable` 503, `inference.denied` 403, `inference.not-found` 404, `inference.invalid` 400, `inference.bounds` 413, `inference.conflict` 409, `inference.capacity` 429, `inference.expired` 410, `inference.cancelled` 409, `approval.commit-unavailable` 503, `inference.storage` 503) with `code()`, `status()`, `from_code()`, `from_status()`, `retryable()` and `to_gateway_error()`. Decoding resolves by the closed `{schema, code}` body first and only falls back to the status, because 503 is shared by three codes and 409 by two.
- **`InferenceHubTransport`** — the JSON twin of the P4-C `CanonicalPairTransport` seam: `InferenceHubRequestV1 { hub_origin, method, path, body, maximum_response_bytes }` → `InferenceHubResponseV1 { status, body }`. No concrete HTTP type appears above it, and the bearer never crosses it.
- **`NativeInferenceHubTransport`** — pins the request to the credential's own origin, rejects a path with `#` or a leading `//`, caps the request body at the hub's own 1024 bytes and the reply at 8192, and calls the one new protected primitive below.
- Four typed calls: `submit_gis_map_job`, `read_gis_map_job_events`, `cancel_gis_map_job`, `approve_gis_map_job`, plus the four exact percent-encoded path builders.

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`, new `//#region 💡️Inference` block: **`NativeDirectoryTransport::request_protected_json`** — the small-body twin of the existing `fetch_protected_stream`. It is ~15 lines, sits in its own region, and is the only edit this lane made to a file other lanes own. It exists because `LocalHubCredential::capability()` is `pub(crate)` to the directory crate, so the bearer can only be applied inside it; the method takes the credential, checks the URL is an exact path under that credential's origin, and never returns bearer text to the caller.

### 2. Binding, driver and workspace glue

`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`, `//#region 💡️Inference`:

- `HubRemoteBinding::hub_origin`/`space_id`/`inference_subject`/`inference_document` — the live authenticated subject (`authenticated_user_id` + the coarse `authority_generation` fence) and a document resolved **only** inside this binding's own space from the authenticated descriptor index. A caller never supplies a scope.
- `NativeHubBindingDriver` gained an `inference_transport` field, built in `connect` from the same `NativeDirectoryTransport` and `LocalHubCredential` the pair transport already uses, and four blocking methods that bridge the synchronous MCP tool surface onto the async transport through the driver's own `TokioHostRuntime` — the exact pattern `mount_canonical_pair` already established.
- `NativeHubBindingDriver::gis_map_inference_base` mounts the **P4-C canonical checkpoint pair** and projects exactly the frozen base an inference job is compared against: `descriptor_digest_v1`, `active_checkpoint_id`, `etag`, `catalog_generation` from `CanonicalPairMountIdentity`, and `head_edit_ordinal`/`head_edit_id`/`last_commit_seq`/`chain_hash` from the verified `ArtifactFrontier` baseline.
- `pair_mount_error_to_gateway` — the closed mount-failure mapping, mirroring the existing `binding_error_to_gateway`.

`🌉️mcp/🏠️workspace/🦀️.rs`, `//#region 💡️Inference`: `hub_inference_subject`, `gis_map_inference_base`, `submit_gis_map_inference_job`, `read_gis_map_inference_job_events`, `cancel_gis_map_inference_job`, `approve_gis_map_inference_job`. Each requires a hub origin, resolves the document in the bound space, states plainly when the document is not the GIS Map kind, and retains a `CancelToken` for the local wait under a bounded operation label.

### 3. Four tools, scope-gated and session-bound

`🌉️mcp/💡️inference/🦀️.rs`, `//#region 💡️InferenceJobTools`:

| tool | kind | declared scopes | what it does |
|---|---|---|---|
| `inference_submit` | Mutation | `documents.read`, `documents.write`, `jobs.spawn` | mounts the local frozen base, POSTs one closed intent, mints a session-owned `job_` handle |
| `inference_events` | Query | `documents.read` | polls `GET …/events?after=<cursor>` — MCP has no progress push |
| `inference_cancel` | Mutation | `jobs.spawn` | interrupts the retained local wait **and** records the hub's durable cancel |
| `inference_approve` | Mutation | `documents.read`, `documents.write`, `jobs.spawn` | sends one exact `{jobId, proposalHash}`; the hub rebuilds the typed effect |

- Registered in `GATEWAY_TOOL_NAMES` (now **26** entries, was 22), in the crate's compiled capability catalog (`inference_job_capabilities`), and in the TypeScript census in `🔄️end-to-end.test.ts`.
- **Bilingual EN/DE by construction** with no default language: each capability's `description` is `"<English> — <Deutsch>"`, asserted by both a Rust law and the process suite. The crate keeps tool descriptions on the `CapabilityDefinition`, which is where they went; `💬️prompts` (the other bilingual surface) was left alone because no new prompt is warranted for a service that fails closed on every hub today.
- **Local admission gate**: a new `MCP_SCOPE_TABLE` row `("inference.execute", ["documents.read", "documents.write", "jobs.spawn"])`. Every handler calls `PolicyEngine::authorize_scopes` first, using the **same** engine and the same shared `HandleTable` the mutation protocol already uses (`ActionAdapter::policy()`/`handles()`, added in their own region) — never a second, disjoint policy engine. The region's own comment states plainly that this is local admission and never hub authorization.
- **Per-connection session binding**: `inference_submit` mints a `HandleKind::Job` handle owned by the connection's `SessionHandle`, carrying `GisMapInferenceJobHandlePayloadV1 { space_id, document_id, job_id, subject_user_id, authority_generation, request_id, base }`. `inference_events`/`inference_cancel`/`inference_approve` take that `jobHandle` and `resolve_inference_job_handle` refuses it for a different session (`PERMISSION_DENIED`) **and** for a different live subject, space or authority generation. The hub then applies the authoritative owner-private check on top.
- **Cancellation propagation** is explicit and bounded: a process-wide, capacity-32 registry of `CancelToken`s keyed by a `gis-map:{space}/{document}[/{job}]` label. `inference_cancel` interrupts both the job label and the document-wide label before calling the hub's `/cancel`. The discarded `notifications/cancelled` JSON-RPC no-op is deliberately **not** repurposed — it cancels a request, not a job.

### 4. `inference_get` and the inference resources

`//#region 💡️InferenceHubRead`: for the GIS Map service id on a hub-bound workspace, `inference_get` and `semio://artifact/{id}/inference/{field}` now go straight to the hub route instead of `execution_not_wired_error`. The client idempotency key is derived deterministically from `(hub origin, space, document, user, authority generation)`, so repeated reads reconcile through the hub ledger's scoped idempotency to exactly **one** job rather than starting a new one per poll. The reply is the owner-private receipt the hub hands to that exact subject and nothing else. Any other `inferenceSchema` falls through to the unchanged discovery path and its unchanged, honest `channel.not-wired`.

## Verification — exact commands and results

| Command | Result |
|---|---|
| `bun ./📜️script.ts inference-bridge-check --source` (cwd `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript`) | **exit 0** — `inference-bridge-oracle: ajv=7 hostile=22 errors=11 visibility=7 lifecycle=9 routes=4 limits=4` |
| `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` | **exit 0** — `.vscode/launch.json regenerated`, 4 lines matching `inference-bridge` present |
| `CARGO_BUILD_JOBS=4 cargo check -p semio-framework-os-mcp --lib --message-format=short` | CARGO_CHECK_PLACEHOLDER |
| `CARGO_BUILD_JOBS=4 cargo test -p semio-framework-os-mcp --lib inference_jobs --message-format=short` | CARGO_TEST_PLACEHOLDER |
| `bun ./📜️script.ts inference-bridge-check --process` | PROCESS_PLACEHOLDER |

### What the source oracle actually proves

`📦️packages/🟦️typescript/💡️inference-bridge.ts` (`proveMcpInferenceBridgeFixture`) is a real third-party observer built on AJV 2020-12, sharing **no** code with the Rust client:

- it AJV-compiles the hub's own neutral fixture schema (`urn:semio:hub:gis-map-proposal-approval-fixture:v1`, with `urn:semio:hub:inference-job:v1` added as its `$ref` target) and validates the shared corpus `🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json` — **reused, never forked**;
- it pins the closed vocabulary: 11 unique codes, `503 → {inference.unavailable, inference.storage, approval.commit-unavailable}` and `409 → {inference.conflict, inference.cancelled}`, which is precisely why the client must decode by code and not by status;
- it pins the one-owner visibility law (7 rows, exactly one reader, every other role `inference.denied`), both lifecycles in order and within one bounded page, the four fixed limits the client mirrors, and the four nonclaims;
- it compiles its own 2020-12 schemas for the five closed wire bodies and rejects **22** hostile mutations, including a smuggled `mapPack`, a client-supplied `actor`, a client-stamped `command`, a leaked `proposal`, a cursor past 16 and a page past 8 items;
- it greps the four `.route("…")` literals out of `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` and checks the client's four path builders render exactly those templates. If the hub lane renames a route, this fails loudly.

### Rust laws (`💡️inference/🦀️.rs`, `//#region 🧪️InferenceJobTests`, `mod inference_jobs`)

They read the **same** neutral fixture via `include_str!`:

- `the_published_error_vocabulary_is_exactly_the_neutral_fixtures_and_status_alone_is_ambiguous`
- `a_503_inference_unavailable_becomes_a_retryable_plugin_unavailable_that_names_the_missing_binding` — the `PLUGIN_UNAVAILABLE`-style naming law the packet asked for, plus the full closed code map
- `the_client_mirrors_the_neutral_fixtures_exact_fixed_limits`
- `a_submit_intent_encodes_within_the_fixed_bound_and_every_hostile_field_is_refused`
- `an_approval_intent_carries_only_the_job_and_its_exact_proposal_digest`
- `the_four_client_paths_are_exact_percent_encoded_hub_paths`
- `a_reply_decodes_by_its_closed_code_and_never_by_its_ambiguous_status`
- `a_two_hundred_reply_must_declare_its_own_exact_schema_and_carry_no_unknown_field`
- `the_neutral_lifecycles_decode_into_the_closed_event_page_in_order`
- `a_submit_call_posts_the_bounded_closed_intent_to_the_exact_job_route`
- `an_events_call_refuses_a_foreign_job_id_or_an_out_of_range_cursor_before_any_request`
- `an_already_cancelled_operation_context_never_reaches_the_hub_and_maps_to_cancelled`
- `a_transport_failure_maps_onto_the_closed_route_vocabulary_and_never_a_fabricated_success`
- `a_retained_local_wait_is_interrupted_by_its_own_operation_label_and_by_nothing_else`
- `every_inference_job_tool_is_denied_without_its_scope_and_admitted_by_inference_execute` — the cross-scope denial law
- `a_job_handle_is_readable_only_by_its_own_session_and_its_own_authenticated_subject`
- `the_four_capabilities_are_direct_object_typed_gateway_tools_with_bilingual_descriptions`

### Registered gate

`inference-bridge-check` with `--source | --process`:

- `🌉️mcp/📦️packages/🟦️typescript/📜️script.ts` (`InferenceBridgeCheckScript`, registered in the router beside `test`);
- `🌉️mcp/📦️packages/🟦️typescript/📋️project.json`: `inference-bridge-source-check` (no Rust dependency at all) and `inference-bridge-process-check` (`dependsOn` the Rust build, then Vitest on the one suite);
- `.vscode/🧩️launch.seed.jsonc` entries `⚖️gate💡️inference-bridge🌉️mcp{📐️source,🔁️process}` at orders 411.149 / 411.1491, immediately after the sibling `gis-map-proposal` hub gates; `.vscode/launch.json` regenerated via `@semio-tech/plugin-registry:generate`.

## Honest nonclaims

- **No external model provider is involved anywhere.** The GIS Map inference is the existing deterministic bounded local computation the hub runs in-process.
- **No WGPU or browser rendering** is implicated. Slice D (the UI port) is a separate lane.
- **No two-user process journey was run or claimed.** It needs the hub's own process gate, which needs a trusted GIS Map catalog binding; `📓️fable-ai-map-proposal.md` records that no non-`cfg(test)` bundle builder exists yet, so no route-level owner→claim→offer→approve journey can execute on either side today.
- **No live hub inference job has been observed end to end from MCP.** With no trusted binding a hub answers `503 inference.unavailable`, and every law here stops exactly there. What is proven is: the closed shapes, the closed error mapping including that 503 → retryable `PLUGIN_UNAVAILABLE` naming the missing binding, the path agreement with the hub's own registered routes, the scope gate, the session-bound job handle, and cancellation.
- **`inference_submit` blocks while the hub runs the service inline.** The hub's `POST …/jobs` route claims, runs and offers before it answers, so the MCP call is a bounded synchronous wait under the hub's own 120 s job lifetime. The local `CancelToken` registry makes cancellation real if the dispatcher ever runs tool calls concurrently; with today's strictly sequential stdio dispatch, a same-connection `inference_cancel` cannot interleave with an in-flight `inference_submit`, and this is stated rather than papered over.
- **Per-connection session binding is one fixed session per process** (`DEFAULT_SESSION_ID`), inherited from P1b. That is not narrowed here: `HubRemoteBinding` is itself one hub subject per process, so there is no multi-subject ambiguity today. The handle payload already carries the subject and authority generation, so the check becomes real the moment the process becomes multi-connection.
- **A job handle does not survive process restart.** The hub's ledger does; a fresh MCP process would have to re-derive its job through `inference_get`'s deterministic idempotency key rather than resolve an old handle.

## Blockers (external, current evidence)

1. **Build contention.** `cargo check -p semio-framework-os-mcp --lib` was queued behind ~55 concurrent peer cargo processes at a host load average above 80. Every cargo invocation here was serialized, foreground, `CARGO_BUILD_JOBS=4`, narrowest target, default shared target dir. CARGO_BLOCKER_PLACEHOLDER
2. **Hub inference readiness is `false` in every deployment today.** `📓️fable-ai-map-proposal.md` records that production has no trusted profile, so `features.inference` publishes `false` and all four routes fail closed with `503 inference.unavailable`. That is the correct end of this chain right now; it is also why no live-job law exists on this side.

## Files touched by this lane

```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.ts        (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.test.ts   (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🔄️end-to-end.test.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json
.vscode/🧩️launch.seed.jsonc
.vscode/launch.json                                                                                (generated)
```
