# M5b — semio MCP protocol conformance

Slice: `📓️g7-mcp-agent-and-collaboration-audit.md` §5 + §6 items 6, 7, 8, 9, 11; `📓️g10-goal-gap-reaudit.md` §D Outcome 4 rows O4-7, O4-8, O4-9, O4-10, O4-12.
Crate: `semio-framework-os-mcp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`).
Started 2026-09-20. No inherited M5b report or `🗑️generated/m5b-*` capture existed — this slice started from zero.
Peers R2 / M5a / M6 edit the same crate; every hunk below is confined to protocol / resources / notifications / pagination / inference-routing code.

## 1. Baseline measured (before any edit)

| claim (G7 §5/§6) | verified on the tree at slice start |
|---|---|
| `resources/subscribe`/`unsubscribe` are unconditional `Ok(())` no-ops | TRUE — `🧠️context/🦀️.rs:272-278`, both `Ok(())` for any URI, while `server_capabilities()` (`🧭️protocol/🦀️.rs:863`) advertises `"resources": {"listChanged": true, "subscribe": true}` |
| `NOTIFICATION_RESOURCES_UPDATED` has zero non-test call sites | TRUE — the constant existed at `🧭️protocol/🦀️.rs:283`; nothing in production published it |
| no `notifications/progress` support at all | TRUE — zero occurrences of `progressToken` crate-wide; `METHOD_NOTIFICATIONS_CANCELLED` dispatched to `DispatchOutcome::NoResponse` (`🧭️protocol/🦀️.rs:993`) |
| no pagination on the list methods | TRUE — `handle_tools_list`/`handle_resources_list`/`handle_resources_templates_list`/`handle_prompts_list` took no `&JsonRpcRequest` at all, so a `cursor` param was structurally unreachable |
| duplicate `semio://workspace/artifacts` in `resources/list` (G7 §6 P2.11) | **ALREADY FIXED by a peer** — `🧠️context/🦀️.rs:216-218` retains by first-seen URI over a `BTreeSet`. M5b adds the regression tests (Rust + client-e2e) that keep it fixed; no code change was needed |
| `inference_submit/events/cancel/approve` hard-wired to the GIS service id | TRUE — `GisMapInferenceSubmitRequestV1::new` stamped `GIS_MAP_INFERENCE_SERVICE_ID` unconditionally and `validate()` required that exact id |
| "every tool declares an `outputSchema`" (G7 §5, stated as already real) | **FALSE** — six tools declared none: `action_cancel`, `transaction_begin`, `transaction_commit`, `transaction_rollback`, `history_undo`, `history_redo` (`🦀️.rs`, `build_tool_registry`). G7 over-reported this one |

## 2. Item 1 — `resources/subscribe` → real `notifications/resources/updated`

New facet `📣️notify` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🦀️.rs`, mounted at `📦️packages/🦀️rust/🦀️.rs`), deliberately NOT glob-re-exported at the crate root (its `publish`/`paginate`/`Page` would collide with the flat facet namespace).

- `NotificationSink` / `NotificationSlot` — one lane per connection, filled by whichever transport owns the socket. No sink bound is an ordinary tier, not an error.
- `ResourceSubscriptions` — per-connection subscribe set; `ResourceSubscriptions::registered()` registers a `Weak` with the process-wide `ResourceUpdateBroker`, so a closed connection is pruned on the next fan-out instead of accumulating.
- `McpServer` now owns one (`🧭️protocol/🦀️.rs`, `subscriptions` field + `publishing_notifications_into` + `subscriptions()`), `handle_resources_subscribe`/`unsubscribe` record into it.
- `WorkspaceResourceRegistry::subscribe`/`unsubscribe` became real **validators** (`🧠️context/🦀️.rs`): `Ok(())` only for a URI this registry lists or a well-formed instance of its declared templates (`semio://capability/{id}` that exists, `semio://artifact/{id}[/history|/validation|/inference]`); anything else is a loud `NOT_FOUND` instead of the old silent forever-wait.
- The one publish site is `McpServer::handle_tools_call` (`🧭️protocol/🦀️.rs`), calling `notify::broadcast_tool_result_changes`. Which tool changes which resource is a **declared table** (`TOOL_RESOURCE_EFFECTS` in `📣️notify/🦀️.rs`) naming the JSON path its artifact id lives at — never inferred from result shape. `artifact_create`/`artifact_open` additionally emit `notifications/resources/list_changed` plus updates for the two `semio://workspace…` projections.
- An artifact change fans out to `semio://artifact/{id}` and its `history`, `validation`, `inference` sub-resources (`notify::artifact_resource_uris`).

## 3. Item 2 — MCP-native progress + cancellation

- `_meta.progressToken` on `tools/call` enters a thread-local `ProgressScope` held across the whole call (`🧭️protocol/🦀️.rs`, `progress_binding` + `enter_progress_scope`). The guard restores the binding it replaced, so a nested call can never silence its caller.
- `JobRegistry::begin_with_id` binds every freshly-minted job to the active scope (`🖥️ui/🦀️.rs`) — every producer in the crate is covered by construction; none has to opt in.
- `JobRegistry::report_progress` and the terminal transition publish `notifications/progress` `{progressToken, progress, total: 1.0, message?}`, ending with one final `1.0` row on success, then release the binding so a long-lived stdio process does not accumulate one entry per job.
- `notifications/cancelled` is no longer a documented no-op: `handle_notifications_cancelled` (`🧭️protocol/🦀️.rs`) maps `params.requestId` onto `job_registry().request_cancel` for every job minted under that request — the very path `job_cancel` and U1/M2 use. It still answers nothing, per JSON-RPC.
- Transports: `StdioNotificationSink` rides the single-owner `StdioLines` channel M4 built (`🚚️transport/🦀️.rs`), so a notification emitted inside a tool call reaches the client mid-call; `HttpEventPublisher` now implements `NotificationSink` and is filled on `HttpTransport::start`, pushing onto the same resumable log the `GET` stream replays. Both wired in `run_stdio`/`run_http` (`🦀️.rs`).

## 4. Item 3 — cursor pagination + duplicate resource dedupe

- `tools/list`, `resources/list`, `resources/templates/list`, `prompts/list` all take the request now and read `cursor` (`🧭️protocol/🦀️.rs`, `requested_offset` + `paged_result`).
- Ordering is stable (tools sorted by name, the rest in registry declaration order). Cursor is opaque (`semio.page.<zero-padded offset>`); one this server did not mint is `INVALID_PARAMS`; a cursor past the end is a legal empty final page so a walk always terminates; `nextCursor` appears only when a next page exists. Default page 100, so every existing client still sees one page.
- Dedupe: already landed by a peer (§1); M5b's Rust test `resources_list_never_repeats_a_uri` and the client-e2e step `os: resources/list pagination + unique URIs` are the regression guards.

## 5. Item 4 — `inference_submit/events/cancel/approve` descriptor routing

- New declared table `HUB_INFERENCE_ROUTES` + `hub_inference_route_for` + `resolve_hub_inference_route` (`💡️inference/🦀️.rs`), and `HeadlessWorkspace::hub_inference_document_descriptor` (`🏠️workspace/🦀️.rs`) which reads the document's **own** `artifact_kind`/`artifact_schema` from the bound hub view.
- All four handlers now resolve the route from that descriptor before anything leaves the process. `GisMapInferenceSubmitRequestV1::new` takes the resolved `service_id` (no longer stamps the constant) and `validate()` accepts exactly the service ids the route table declares.
- A document whose kind has no hub-backed row is refused **locally** with a `NOT_FOUND` naming the kind, the hub-backed services this gateway can reach, that kind's own plugin-declared inference services, and `inference_run` as the route which serves them — instead of a hub round trip that answered `PRECONDITION_FAILED` with no descriptor named.
- `events`/`cancel`/`approve` re-resolve the route from the job handle's document and report `serviceId` in their structured output.
- The four capability titles/descriptions were de-GIS-ified accordingly (and `inference_events`' "MCP has no progress push, so poll this cursor" is now false and was corrected).

## 6. Item 5 — `outputSchema` / `structuredContent`

- G7's claim that every tool declares an `outputSchema` was wrong: six did not. All six now declare one (`🦀️.rs`): `action_cancel`, `transaction_begin`, `transaction_commit`, `transaction_rollback`, `history_undo`, `history_redo`.
- **Honest limit:** those six use the registry's existing `capability_generic_output_schema(<capability id>)` envelope (`{"type":"object"}` + its own `$id`) — the same envelope every plugin capability already uses, so the single-source-of-truth convention holds and no schema-mirror regeneration was needed. It proves "is an object", not the typed `SagaReport`/`UndoRedoReport` shape. Typing those six is a named follow-up (it needs new `🧬️schema` exports plus the `🔣️.json` mirror).
- Two Rust oracles (`📣️notify/🧪️tests/🔬️quick/🦀️.rs`): one asserts every tool declares an `outputSchema` that COMPILES with the repo's owned `semio_framework_schema::OwnedJsonSchemaValidator`, the other drives every tool through real `tools/call` (with a declared representative-argument table for the four that need arguments) and validates each successful result's `structuredContent` against that tool's own declared schema.

## 7. Tests and transcript

(filling — the scoped `cargo test -p semio-framework-os-mcp --lib notify::quick` run and the `client-e2e` transcript go here with real counts; nothing in this section is claimed until its output is captured to `🗑️generated/m5b-*.txt`)

## 8. Honest gaps

1. **stdio cancellation is serialized.** The stdio serve loop dispatches one request at a time, so a `notifications/cancelled` sent *while* a blocking tool call runs is buffered by `StdioLines`' reader thread and acted on when that call returns. Concurrent cancellation of a blocking call is live only on HTTP. Stated in the README rather than papered over; making stdio dispatch concurrently is a transport restructure outside this slice.
2. **Six tools' `outputSchema` is the generic object envelope** (§6) — real, compiled and validated, but not a typed shape.
3. **Resource updates are published from the tool-result table, not from the store.** An artifact changed by something other than a `tools/call` on this connection (a peer's edit arriving through the workspace, a hub replication) does not yet publish `resources/updated`. The broker is process-wide and ready for such a caller; no store-side call site exists yet.
4. **Hub inference routing table has exactly one row.** Routing is now descriptor-driven and table-extensible, but the only hub transport that exists is the GIS Map quartet, so a second row cannot be proven today.
5. Everything here is in the working tree, not merged (the standing INF-2 risk).

## 9. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🦀️.rs` (new facet)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📣️notify/🧪️tests/🔬️quick/🦀️.rs` (new tests)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs` (mount `notify`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs` (subscriptions, notification slot, pagination, progress scope, cancelled handler, tool-result broadcast)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧠️context/🦀️.rs` (subscribe/unsubscribe validators)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs` (stdio + HTTP notification sinks)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs` (job→progress binding and push)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs` (hub route table + four handlers + capability text)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` (`hub_inference_document_descriptor`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` (six `output_schema` declarations, `run_stdio`/`run_http` notification slots)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` (client-e2e: pagination walk, subscribe→mutate→updated, progress token, cancelled probe)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` (Notifications + Pagination sections, descriptor-routed inference quartet, unique-URI note)
