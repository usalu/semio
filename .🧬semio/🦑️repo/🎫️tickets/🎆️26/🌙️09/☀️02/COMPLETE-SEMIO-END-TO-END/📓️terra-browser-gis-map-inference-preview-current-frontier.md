# Terra Audit — Browser GIS Map Inference Preview Frontier

Date: 2026-09-05  
Scope: current-tree, read-only recheck of the GIS intent → browser owner → authenticated Hub executor → offer/approval route. No build, browser, Hub, model call, or process journey was run. “Present” therefore means source-integrated, not runtime-qualified.

## Decision

The older reports which said that no GIS inference action, Hub route, or browser port existed are obsolete. The current tree has a closed, authenticated browser-to-Hub proposal path and a frozen native deterministic GIS executor. It still does **not** supply a user-previewable Map proposal: the only renderer-visible offered value is `proposalHash`, yet the host panel renders an enabled **Approve** button on that state. This is the smallest independent gap after the Store-owned three-member commit seam: add an owner-private, typed **bounds preview projection** to the already-polled event page. Do not add an external model/provider or send generic proposal/command bytes to a plugin.

The existing service is deterministic bounds analysis, not an LLM or upstream model provider. That is an explicit current nonclaim, not a transport defect.

## Current trace

| Boundary | Current implementation | Result |
| --- | --- | --- |
| GIS user intent | `Gis2dCommand::ProposeBoundsRegion` maps `"proposeBoundsRegion"` to `Effect::RequestInferenceProposal { GisMapBoundsRegion }` at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:223-239,801-803`; the command emits no document mutation at `🎮️commands/💡️inference/🦀️.rs:24-28,43-58`. | Present and correctly host-owned. |
| Browser host admission | ShellHost turns that effect into `inference-open` then `inference-propose`, using the owning Hub scope and a new operation epoch at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3678-3695`. | Present; no plugin-supplied URL, bearer, provider, map pack, or approval bytes. |
| Browser transport | `🧵️backbone-worker.ts:2390-2721` owns one abortable port, requires a live writable verified execution-target lease before open, admits only the four exact Hub paths, response-bounds JSON, polls, cancels without optimistic terminal state, and approves by echoing the server hash. Its worker laws are at `:4157-4284`. | Present, source-only. |
| Shared schema/host UI | The TypeScript contract defines closed request/page/approval DTOs and reducer at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1393-1769`. The host panel is mounted at `🏛️ShellHost/🟦️.tsx:7824-7831` and enables approval when phase is `offered` plus a hash at `🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:155-199`. | Present, but has no proposal geometry. |
| Hub route/authentication | `POST jobs`, `GET events`, `POST cancel`, and `POST approval` are exact scoped routes at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6327-6394`. Each first constructs `InferenceRouteContextV1`; runtime re-authenticates the session, reads the original owner identity, and rechecks live Author scope before page/cancel/approval at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:558-636`. | Present; existing routes are owner-private. |
| Frozen provider | `HubInferenceRuntimeV1::infer` materializes only the verified active Map pack, runs `infer_gis_map_controlled`, then derives the sole `CreateRegion` and SHA-256 at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:237-271`. The GIS algorithm computes only finite lon/lat bounds and one region at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:59-94,168-171`. | Present deterministic provider; no external AI/model. |
| Approval/final publication | The route recomputes the typed command server-side and calls the narrow committer only after author/frozen-base checks at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:608-636`; startup currently injects `UnavailableGisMapApprovalCommitterV1` at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6637-6642`. | Correctly fail-closed until WGPU’s Store-owned fixed-three integration lands. This audit does not duplicate that durable-work review. |

## The actual P0 break: an offer has no preview

`InferenceJobLedgerV1::read` retains owner-private result/proposal bytes, but `read_gis_map_job_events` asks only `ledger.events` and returns `InferenceEventPageDtoV1` with lifecycle rows, progress and an optional hash (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:558-591`; DTO at `:402-417`). The sole browser page parser accepts only those fields (`🧬️schema/🟦️.ts:1715-1755`), and its status shape expressly excludes proposal bytes (`:1484-1519`). The panel consequently exposes only textual phase/progress and `Approve` (`🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:179-197`).

The ledger’s bytes must **not** simply be copied to a generic browser value or plugin `viewState`: the canonical proposal is a general `GisMapMutation` JSON value and the approval command/inverse are more privileged data. The exact safe output is already known: the provider produces only `CreateRegion` with an `inference-<jobId>` feature and a five-point lon/lat rectangle (`💡️inferences/🦀️.rs:59-94`). Its independent fixture contains both the canonical mutation and expected extrema at `🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json:2-19`.

Therefore the current UI lets a user approve a hash without seeing what will be added. It is not an acceptable preview/approval journey, even though it is correctly non-auto-applying.

## Smallest independent implementation packet

Extend the existing owner-private **events** response rather than creating a fifth inference endpoint or a generic proposal fetch.

1. In `🌎️hub/💡️inference/🏃️runtime/🦀️.rs`, add a private projection helper used only by `read_gis_map_job_events`. After the existing authentication, owner identity read, live Author check, and fresh-base comparison have succeeded, read the same owner-private ledger row. Verify `sha256(row.proposal) == page.proposal_hash`; decode the canonical proposal through the existing `directory::os_pack::json::from_json_str` / GIS `FromValue` seam; require exactly `GisMapMutation::CreateRegion`, id `inference-<jobId>`, `kind == "inference-bounds"`, and the exact closed five-point finite rectangle inside lon ±180/lat ±90. Project only:

   ```text
   { schema: "semio.hub.gis-map-inference-preview/v1",
     jobId, proposalHash, regionId,
     ring: [[lonMin,latMin],[lonMax,latMin],[lonMax,latMax],[lonMin,latMax],[lonMin,latMin]] }
   ```

   `preview` is absent unless the page is current, `state == succeeded`, `proposalState == offered`, `cancelRequested == false`, and its hash is present. A stale/cancelled/failed/approved response has no preview. Any decode, hash, geometry, or bound failure is `inference.conflict`/`inference.invalid`, never a partially projected response.

2. Make this DTO an optional `preview` member of the already strict event-page schema in both `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`. Keep its byte budget inside `GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES` (16 KiB); it is five finite pairs and bounded identifiers, not a raw `proposal`, `result`, base pack, command, inverse, receipt, bearer, or provider detail. The current worker poll automatically receives it, so no new broker capability/path is needed.

3. Add optional preview to the ephemeral `GisMapInferencePortStatusV1` and retain it only from an exact current offered page. Clear it on every terminal/lease failure/close. `InferencePortPanel` renders the projected region identity and lon/lat extent before the Approve button; it must not write the projection into plugin state or send it into a plugin. A later host-owned Map-overlay adapter can consume this narrow DTO, but is not needed to close the immediate blind-approval defect.

4. Retain the existing approval body unchanged: `{ jobId, proposalHash }`. Approval still recomputes and validates the server-stamped command from the current verified base at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:274-299`; the preview is an inspection projection, never approval authority.

### Separate browser scope correction (P1, do with the preview slice)

The browser’s document/session ownership is already keyed by `documentRuntimeKeyV1` so equal document ids in different spaces do not share lifecycle state (`🏛️ShellHost/🟦️.tsx:1651-1655,4020-4024`). Inference alone regresses to `documentId`: its reducer map and current-operation field are named/keyed `portByDocumentId`/`operationDocumentId` (`🐚️Shell/🟦️.tsx:590-600,1006-1018`), its message handler admits an epoch-matching status without checking the current runtime scope (`🏛️ShellHost/🟦️.tsx:1705-1708`), and the initial effect owner is the first matching plugin/instance in the whole session map (`:3683`).

Because capacity is one and the worker checks scope internally, this is not presently a demonstrated cross-space approval bypass. It is nevertheless the wrong browser ownership model and can render or clear the wrong same-id document as soon as the same plugin instance has more than one scoped session. Key the UI state and action cleanup by `documentRuntimeKeyV1({ kind: "hub", ...scope })`, retain expected scope beside the operation epoch, require an exact scope-key match before accepting a worker status, and fail closed when effect origin cannot identify one exact session. Do not parse actor URIs or infer a scope from a peer/proposal.

## Existing fixtures, gates, and first executable laws

Reuse, do not fork, these sources:

| Existing source | Reuse |
| --- | --- |
| `🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/{🧬️.schema.json,🔣️.json}` | Add `preview` expected projection, cross-owner denials, and malformed/hash/ring hostile rows. It already pins the exact canonical proposal/hash and expected lon/lat bounds. |
| `🧰️framework/🛍️products/💻️os/🧫️fixtures/💡️gis-map-inference-port-v1/{🧬️.schema.json,🔣️.json}` | Extend offered-page and renderer status vectors with the optional preview; preserve the existing no-raw-proposal nonclaim. |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4043-4123,5055-5086` | Extend `gis-map-proposal-check`; it already cross-checks the GIS fixture, but its present process mode explicitly does not run a trusted-profile journey. |
| `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:119-243` | Extend `gis-map-inference-port-check --browser`; its present banner expressly makes no real Hub/renderer/two-user claim. |

First laws, in order:

1. **Hub preview projection is exact and owner-private.** A succeeded/offered fixture row yields exactly the five-corner DTO with the fixture hash; same-space peer, other-space author, viewer, stale session generation, and anonymous user receive only the existing typed denial, with no proposal/body bytes.
2. **Hub does not preview uncurrent data.** Cancelled, stale frontier/base, changed proposal hash, malformed mutation, non-CreateRegion, wrong region id/kind, non-finite/out-of-range/reordered ring, and `max-preview-byte + 1` produce no preview and no approval widening.
3. **Browser drops a mismatched scoped page and clears the preview.** Same `documentId` in two spaces plus a stale/mismatched-scope status cannot overwrite the retained port or make its buttons operate on the other runtime. Terminal, close, lease loss, and identity change erase projection state.
4. **Browser renders inspection before approval.** The current offered fixture shows its bounds and only echoes the Hub hash on click; raw proposal JSON, command/inverse/base/result, socket receipt, URL, bearer, and plugin viewState remain absent.
5. **Later real process journey (after the Store committer).** With a verified GIS target, SQLite Hub and two authenticated Authors: open the actual GIS editor, invoke `proposeBoundsRegion`, observe a hash-bound preview, prove the peer cannot read it, approve once, then reopen/replay and prove the Map parent plus drawing/value triple appears only after the genuine committed receipt. This audit did not run it.

## Qualification status

The source-only browser path is substantially implemented; the user-preview path and any authenticated trusted-profile browser/Hub process receipt are not. The fixed-three Store publication remains a downstream prerequisite for successful approval, not a reason to defer the independent preview projection. No external model provider, WGPU rendering result, or real user journey is claimed here.

---

## Addendum — Current Typed Preview, Approval, and Scope Review (2026-09-05)

This supersedes the opening finding that the current implementation lacks a typed preview. The current tree now has that projection. This read-only review did not run a build, Hub, browser, or process journey; all positive findings below are source/fixture findings only.

### Authority result

No forged-preview-to-publication bypass is apparent in the current source.

| Boundary | Current guard | Why a forged preview cannot authorize a Map commit |
| --- | --- | --- |
| Hub owner page | `read_gis_map_job_events` authenticates the session, resolves the job with a session-and-scope reader, rechecks live inference Author authority, and holds the document gate before looking at the offered row at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:626-641`. | `gis_map_inference_preview` first matches the stored proposal SHA-256, then accepts only the one canonical `CreateRegion`/`inference-<jobId>`/five-corner finite rectangle projection at `:431-476`. It exposes neither proposal, command, inverse, base pack, receipt, nor identity. |
| Hub approval | `approve_gis_map_job` rejects a path/body job mismatch, re-authenticates and re-authorizes the original owner in the exact scope, verifies the stored proposal hash, rechecks the frozen base, and regenerates the command server-side at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:688-709`. | Approval never consumes the browser preview. It accepts only `{jobId, proposalHash}` and independently reconstructs the mutation/inverse before handing a server-stamped command to the committer. A visual substitution cannot select another job, proposal, or command. |
| Browser wire and reducer | `parseGisMapInferencePreviewV1` requires the v1 schema, bounded hex IDs, exact `inference-<jobId>` id, finite in-range closed rectangle; page/status parsers require preview job/hash equality with their enclosing status/page at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1699-1732,1807-1817`. | A syntactically valid rectangle with another job or proposal hash is rejected before it reaches the reducer. The reducer and worker both require the preview identity to equal the currently offered job/hash before emitting approval at `🧵️backbone-worker.ts:2614-2641`. |
| Worker private transport | One port requires a live, writable execution-target lease for the exact scope and emits only four fixed scoped endpoints under the worker's own broker at `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2424-2447,2490-2503`. | The plugin receives neither a bearer nor a path, proposal, command, preview, or approval body. The worker constructs approval from retained status, not caller data. |
| React ownership | The Shell retains `{operationEpoch,runtimeKey,scope}` and accepts a worker status only when epoch, derived runtime key, space, and document all agree at `🏛️ShellHost/🟦️.tsx:1630-1633,1707-1712`; UI actions demand the same owner at `:1993-2013`. The intent side resolves exactly one scoped document session at `:3687-3709`. | Equal document IDs from different spaces cannot replace the active operation or redirect its approve/cancel action. Preview state is host-only, keyed by runtime key, and the mounted panel receives only bounded status at `🐚️Shell/🟦️.tsx:1003-1018` and `🏛️ShellHost/🟦️.tsx:7846-7853`. |

The panel is also fail-closed for blind approval: it renders a preview if supplied, but exposes **Approve** only for `offered`, non-cancelled status whose preview job/hash equal the status job/hash (`🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:162-203`). This is appropriate UI gating, while the Hub remains the actual authority.

### One concrete current source defect

`OracleCorpus` omits the schema-required `otherJobId`, yet the new hostile-preview loop reads `fixture.otherJobId`:

- declaration: `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:37-53`
- offending use: `:147-155`
- actual fixture/schema field: `🧰️framework/🛍️products/💻️os/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json:76` and `🧬️.schema.json:20,239`.

This is a TypeScript static type error (`Property 'otherJobId' does not exist on type 'OracleCorpus'`) if that script is typechecked. The minimal correction is to add `otherJobId: string` to the local closed corpus type. It does not weaken parsing or add a compatibility path.

### Evidence boundary: what is and is not proved

The new tests are useful source-level laws, but none is a real authenticated Hub-to-browser journey:

- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts:124-252` validates the neutral corpus against an independent reducer and calls the worker Vitest suite only with `--browser`; the file explicitly says it never runs a real Hub or renderer.
- The worker suite at `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4088-4294` uses a fake already-authenticated broker, a stubbed `fetch`, and a locally forged test lease. It establishes fixed-path/lease/status behavior, not Hub authentication or browser origin/proof handoff.
- `👥️scoped-presence.test.tsx:110-126` renders `InferencePortPanel` directly and proves the button is hidden without a supplied preview. It does not mount `FrameworkOsShell`, drive the worker `onmessage` handler, or start a Hub. Its paired source oracle checks the new inference scope strings rather than exercising that callback (`🎯️targets/⚛️react/📜️script.ts:149-156`).
- The present Hub binary law is `gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding` only (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7374-7410`). It proves safe absence, not a successful preview response under a loaded trusted profile.

The first runtime acceptance after the Store committer is available should therefore use two authenticated users and two scopes sharing a document id: open the actual GIS editor in scope A, offer and view its preview, attempt page/approval from peer and scope B (both must disclose no preview/proposal), inject or replay a valid-looking B status toward A (must not alter A UI or send approval), then approve A and observe exactly one three-member committed publication. This remains unrun.

### Minimal missing adversarial laws

1. In `🧵️backbone-worker.ts`’s existing `gis map inference port` suite, return an otherwise valid offered page whose preview has another valid job id or proposal hash. Assert parsing terminates the port and no `/approval` request is sent. Existing coverage rejects malformed geometry but not this valid-looking cross-job substitution.
2. In `👥️scoped-presence.test.tsx`, add a fixture-driven ShellHost-level adapter/law for two same-document scopes, old epoch A and active epoch B. Feed the exact encoded `inference-port-status` messages and assert only B is displayed/actionable; close A and assert B remains. A direct panel render does not establish the handler fence at `ShellHost/🟦️.tsx:1707-1712`.
3. Add a trusted-profile Hub route law that reaches `read_gis_map_job_events`, proves an owner gets exactly the bounded DTO, and proves same-space peer/cross-space/expired session get typed denial with no `preview`, `proposal`, or raw proposal bytes. The existing lib projection law cannot cover its authentication/route composition.

### Exact Hub native selector

The new runtime projection unit is:

```text
inference::runtime::tests::gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary
```

The accepted `runExactCargoLaws` suffix is:

```text
gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary
```

It belongs to package `semio-hub`, target `{ kind: "lib", name: "semio_hub" }`, with `cargoArgs: ["--features", "sqlite"]`, as currently registered at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5069-5083`. It is **not** in the `os-hub` binary target: that group contains only the no-trusted-binding route law. Consequently, a root binary-only native run does not execute the preview projection law; it needs the lib group above (or the registered `gis-map-proposal-check --native` which runs both groups).
