# AI-Over-MAP Live Journey Audit

Current-source audit, 2026-09-05. This rechecks the live tree after the private ledger, WAL witness, authorization predicate, GIS codec receipt, and trusted-catalog work. No build, server, browser, or socket process was run for this audit.

## Verdict

**RED — there is no OS-to-hub MAP inference journey.** The first decisive break is the OS action boundary: the GIS Map editor has no inference request, job observation, cancellation, result, or approval action. Even a forged client cannot take the next step: the hub exposes no inference route, owns no job runtime or ledger in `HubState`, has no inference wire frame, and reports `features.inference: false`.

The tree does contain useful, bounded internal prerequisites. They are not a user journey, a model/provider integration, a document mutation, or a two-user result:

| Current piece | Current evidence | Deliberate limit |
| --- | --- | --- |
| GIS Map calculation | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:195-292` declares `s.gis.gismap.inference` and runs deterministic bounded lon/lat calculation with checkpoint callbacks. | It is an in-process deterministic algorithm, not a remote/model provider or tool loop. |
| Hub identity and authorization | `🌎️hub/💡️inference/📇️catalog/🦀️.rs:47-76` constructs an identity from verified catalog, descriptor, session, scope and frontier; `🛂️authorization/🦀️.rs:7-30` rechecks live Author membership. | Both are unused by `🚀️bin.rs`; the authorization comment correctly says its successful read is not a retained submission grant. |
| Private job ledger | `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:11-48,121-321` has bounded idempotent SQLite lifecycle/outbox tables, private bytes, cancellation, expiry, and an approval-reconciliation witness. | It is a separate SQLite sidecar, has no `HubState` owner/route/worker/fanout, and does not itself submit a document command. |
| Exact approval identity | `🧬️schema/✅️approval/🦀️.rs:7-24` accepts only `{jobId,proposalHash}`; `sqlite/🦀️.rs:219-287` binds a canonical command to a committed-WAL witness. | No HTTP/socket approval handler calls it, and no normal `ArtifactHandle::submit` path reconciles it. |
| Normal collaborative mutation | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2818-2892` admits a regular socket command, fsync-submits it, then fans out `ServerFrame::Commands`. | It has no inference job/proposal binding and no inference event source. |

## End-to-end trace

### 1. OS user intent — first decisive RED

`Gis2dCommand` has exactly fourteen rows at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/.../✏️editor/🦀️.rs:219-247`; `command_from_action` maps the same closed set at `:756-800`. The retained factory's `GIS2D_RETAINED_TOOL_IDS` and publication contracts repeat only those actions at `:259-365`. None is an inference request, status read, cancellation, result inspection, or typed approval.

Thus neither the native editor nor the browser string `{action,args}` bridge can send a MAP inference intent. No EN/DE accessible pending/progress/error/approval state exists. This is stronger than a missing button: there is no typed UI command or app-owned job factory to authorize one.

### 2. Authenticated, scoped hub transport — independently RED

The only document endpoints are ordinary status/open-plan/socket endpoints (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5138-5174`). `HubState` has directory, document DB, trusted-open catalog and normal socket fanout, but no inference ledger/operation registry/control (`:1344-1379`). `hub_readiness` hard-codes `inference: false` (`:1691-1733`).

The document wire cannot carry a request or job stream: `ClientFrame` is Hello/Commands/Frontier/Preview/Presence/Credit/Bye only and `ServerFrame` has no inference state (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:45-56,517-567`). Reusing `Preview` would be wrong: it is ephemeral, unaudited, not durably replayable, and normal peer-visible rather than owner-private.

`check_live_inference_author` is the right narrow predicate for a future route: it binds session id, user id, authorization generation and `DocumentScope`, races the directory read against cancellation/deadline, and only admits `Author` (`🌎️hub/💡️inference/🛂️authorization/🦀️.rs:7-30`). It supplies neither a submission lock nor a role policy for Admin; the route must revalidate under its subject-to-document operation gate. An Admin role therefore has no current inference privilege or audit/read policy.

### 3. Selection, execution, and cancellation — foundation only

`identity_from_verified_catalog` correctly requires the Map descriptor, `semio:gis` package/version/hash, a single declared GIS service, exact scope and frontier (`🌎️hub/💡️inference/📇️catalog/🦀️.rs:17-76`). It is not wired to the hub's `openable_catalog` or a selected open plan; it needs the retained `VerifiedTrustedCatalog`, whereas `HubState` currently only exposes an erased `DocumentOpenCatalogAuthorityV1` (`🚀️bin.rs:360-389,1344-1352`).

The GIS artifact's service metadata and controlled deterministic computation are real (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:195-292,357-365`). It decodes a canonical snapshot supplied to it, reports checkpoint work, rejects cancellation/budget/cache violations, and outputs a canonical result. It does **not** select a model, invoke a provider/tool, own a durable job, or receive an authenticated scope. The local framework `ArtifactInferenceRouter` is likewise a plugin-runtime router (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:6826-6921`), not a hub authority; no hub source references it.

MCP is explicitly discovery-only: `inference_get` returns `channel.not-wired` for a declared service (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:155-162,300-315`). It is not an alternate MAP execution path.

### 4. Durable result, approval, and collaboration — RED

The private ledger is useful for this future route: `accept` is a transactionally idempotent request winner (`🪶️sqlite/🦀️.rs:129-150`), state transitions bind the full identity and expire stale work (`:152-188`), and the approval outbox binds canonical command bytes, job, proposal and actor (`:219-251`). `reconcile_committed_approval` will only mark the outbox committed after an exact WAL witness (`:270-287`). Those are direct-ledger APIs, not integrated effects.

There is no caller that:

1. derives the snapshot/input from the committed document at the selected frontier;
2. invokes the selected GIS service under a server-owned cancellation control;
3. persists progress to an event stream visible to the request owner after reconnect;
4. converts the result to the existing typed `GisMapMutation::CreateRegion`; or
5. calls ordinary `submit_commands` and then reconciles its committed WAL witness before document fanout.

The local fixture contains a `CreateRegion` proposal, but its script explicitly says it proves no hub catalog activation or approved inference acceptance (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:71-121`). A Map mutation also re-derives content-addressed drawing/value child handles on every feature change (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:64-105,161-170`), so an approved mutation must wait for the retained atomic parent+existing-child publication route. Treating an outbox phase as collaborative visibility would bypass that outstanding composition invariant.

Normal document peers can observe only a successfully committed generic mutation via `ServerFrame::Commands` after `ArtifactHandle::submit` (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2801-2838`). There is currently no private owner-only job stream, no collaborator-visible approved inference event, and no reconnect cursor for either.

## Smallest honest P0

Split the work deliberately; a single "AI mutation" packet would conceal two unresolved authorities.

### P0-A — executable private deterministic MAP proposal job

This is the smallest vertical slice that can be run after trusted GIS installation and public member open. It produces no document mutation and no general model claim.

1. Add one closed request/read/cancel route family and server DTOs under `🌎️hub/💡️inference/🧬️schema`, plus a private `HubState` runtime owner at `🚀️bin.rs:1344`. The client may send only `InferenceRequestV1`; the hub derives scope, authenticated session/generation, descriptor, catalog generation, canonical Map snapshot/input and frontier. It must retain a concrete `Arc<VerifiedTrustedCatalog>` beside the erased open catalog so that the existing `identity_from_verified_catalog` helper is usable without client-supplied package facts.
2. On issuance and immediately before run, serialize per exact `(spaceId,documentId)` operation. Recheck `check_live_inference_author`, current document frontier, trusted GIS Map selection and the deadline. Then call ledger `accept`/`start`; execute only `gis_map_inference_service()` selected by those server facts. Persist bounded `accepted/running/progress/succeeded|failed|cancelled` events in the owner-private ledger and make request idempotency `(requestId, identity digest)`.
3. Add owner-private `read` and `cancel` endpoints or a separate authenticated job socket; neither may use `ServerFrame::Preview`. A reconnect cursor must replay monotonic durable events, recheck session/generation/scope at each page, and send no result/proposal to peers. Cancellation/deadline must be terminal and retire the operation; provider failures cannot leave an offered proposal.
4. Add a GIS editor action only once the route exists. It sends the closed intent, holds a local cancellable presentation operation, and renders EN/DE labels for queued/running/cancelled/failed/succeeded. The user-visible result remains private and read-only in P0-A.

This is a deterministic GIS calculation; it is **not** an LLM/model/tool-provider implementation and must not be marketed as one.

### P0-B — later explicit approval and collaborative document effect

Do not start this before atomic parent+existing-child publication is available. It must re-read the offered job under the same owner/session/scope and exact proposal hash, construct the sole typed `GisMapMutation::CreateRegion` from the server-produced proposal (never client command bytes), submit it through the ordinary document authority, obtain the committed WAL witness, then reconcile the outbox. Only that successful commit may emit a document `Commands` fanout. A viewer/other space/admin cannot approve or observe private proposal bytes by implication. Reconnect observes the normal committed tail, not a duplicated job event.

## Required law packet

Existing foundation command, not re-run here:

```sh
bun nx run os-hub:gis-inference-ledger-check
```

It explicitly labels itself “no route/GIS-approval acceptance” in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4249-4283`; retain it as a prerequisite only.

Add a separate `gis-map-private-job-check` target in that project’s `📋️project.json` and `📜️script.ts`, with a neutral `🌎️hub/🧪️fixtures/🗺️gis-map-private-job-v1` corpus and exact native route laws. Minimum rows:

| Law | Required assertion |
| --- | --- |
| Happy path | One authenticated Author in a selected GIS Map document obtains `accepted → running → succeeded`; recorded identity includes exact scope, session generation, descriptor/catalog/package hashes and frontier; output equals the GIS independent fixture. |
| Privacy | A member in the same space, a user in another space, and an Admin without an explicit policy cannot read/cancel/approve the job or proposal. |
| Freshness | Revoked/stale session, changed descriptor/catalog generation, changed frontier, wrong Map kind/service, and duplicate request id with altered identity all fail before execution. |
| Termination | Cancel-before-run, cancel-during-checkpoint, deadline, provider fault, and retry/restart each leave one terminal event and no offered proposal after a failed computation. |
| Reconnect | The original Author receives ordered durable events exactly once across reconnect; a peer sees none. |
| P0-B guard | Before approval no mutation/fanout; after a deliberately injected document-commit failure the outbox remains reconcilable and no peer observes a mutation; only an exact WAL witness turns it approved. |

The browser/process proof must start a real local hub with a server-owned trusted GIS profile, create a Map in an Author-owned space, issue the typed UI intent, receive and cancel/reconnect the private job view, and assert that a second authenticated socket sees no result. A separate future two-user process proof adds approval plus the normal document tail after atomic composition publication. No such registered end-to-end process target exists today.

## Nonclaims

- No authenticated inference route, live job worker, durable progress stream, UI action, browser/native runtime proof, or two-user MAP effect exists in current source.
- The private ledger/WAL/catalog/authorization tests are genuine targeted foundation tests, but not an integrated server effect.
- GIS controlled inference is deterministic local computation, not a model/provider/tool integration.
- The existing document socket only proves generic committed command delivery; it does not prove a proposal/approval path.
- Admin policy is absent rather than implicitly inherited from directory administration.
