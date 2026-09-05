# AI-Over-Map Authenticated Commit Frontier

Current-source, read-only audit on 2026-09-05. I reused the earlier AI/Map and durable-group reports, then rechecked the current Hub, GIS, and browser sources because the older reports predate the newly added Hub routes/runtime. No model request, build, server, browser, or process gate was run.

## Verdict

**RED for an end-user AI-over-Map edit; substantially safer than the older reports described.** The current code has an authenticated, document-scoped, server-materialized deterministic GIS proposal path and deliberately stops before publication. It does **not** call an external model/provider: the only execution is the frozen native `s.gis.gismap.inference` bounds algorithm.

No current request can bypass Map's three-member publication requirement because Hub startup installs `UnavailableGisMapApprovalCommitterV1`, which returns `approval.commit-unavailable` (HTTP 503). That fail-closed stop is correct. It is not a working approval/apply journey.

The smallest missing user journey is:

> An authenticated Author opens a trusted GIS Map document, requests bounds, sees an owner-private proposed region, cancels or approves it, and a second Author sees exactly one durable Map + drawing + value update but never the proposal/result bytes.

Today there is no browser/native caller or proposal-rendering DTO for the first half, and the last step is intentionally unavailable. No browser/native client source uses the Hub `inference/gis-map` or `inference-approval` contract; the GIS editor therefore cannot initiate, observe, cancel, inspect, or approve this HTTP job.

## Current protected path

| Boundary | Current source | Verified property | Limit / missing proof |
| --- | --- | --- | --- |
| Ingress | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6273-6342` | Four real authenticated-HTTP route handlers exist below `/spaces/{space}/documents/{document}/inference/gis-map/jobs`. The only request body is the closed `InferenceRequestV1`; approval accepts only `{ jobId, proposalHash }`. | There is no browser/WGPU transport or GIS editor action for them. |
| Session and space authority | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:430-435,475-505`; `🛂️authorization/🦀️.rs:7-30` | The Hub authenticates the bearer, derives the session-owned identity, and requires an active `Author` role in the exact space. The predicate binds user, session, authorization generation, `space_id`, and `document_id`; it races the directory read with the operation deadline/cancel control. | It returns a point-in-time check, not a publish-time capability. See the final-fence gap below. |
| Document/base binding | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:437-453,482-501`; `📇️catalog/🦀️.rs:294-386` | The Map pack comes only from `VerifiedRebootstrapSource::active_pair(scope)`, never from the caller. Descriptor scope/owner/package/version/component hash and one declared GIS service must all match the frozen binding; frontier and input digest enter the identity. | The only native route law starts without a trusted binding, so no successful configured profile is exercised. |
| Frozen executable | `🌎️hub/💡️inference/📇️catalog/🦀️.rs:156-200,209-284`; `🏃️runtime/🦀️.rs:237-271` | Binding admits one exact writable Map editor, native executable, catalog generation, package/component hashes, dialect, service and policy. The Hub invokes the literal GIS controlled executor with server-owned budgets and the materialized pack. | This is deterministic geographic bounds, not an external/model tool call and not browser rendering. |
| Privacy | `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:313-365,469-500`; `🏃️runtime/🦀️.rs:551-587` | Owner access compares user/session/auth-generation/space/document. The public receipt/event DTO exposes lifecycle/progress and a proposal hash, not base/result/proposal bytes. A same-space peer, cross-space user, wrong document, stale session or stale generation is rejected in the existing ledger law. | The owner-private `ledger.read()` has proposal bytes internally, but no HTTP DTO/route returns them; an editor cannot display the proposed geometry before approval. |
| Freshness before offer | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:522-548` | After execution, the Hub reacquires the per-document gate, rechecks live Author status, rematerializes the authority-verified base, and compares frozen binding/scope/frontier/pack digest before offering the proposal. | No successful route/process law tests this with a concurrently changed base or revocation. |

This prevents another space's Map context entering an accepted job: `exact_projection` rejects scope/descriptor/package mismatches at `🌎️hub/💡️inference/📇️catalog/🦀️.rs:294-324`, and every owner read begins with `identity_of(job_id, session_reader(session, path_scope))` at `🏃️runtime/🦀️.rs:565-573,591-601,616-629`. The current code has no evidence of a cross-space result leak.

## Cancellation and effect boundary

There is real local cancellation machinery: `InferenceOperationControlV1` has deadline, atomic cancellation, and bounded checkpoints (`🌎️hub/💡️inference/🦀️.rs:31-67`); the runtime retains a controller per running job (`🏃️runtime/🦀️.rs:206-229`); and the GIS executor calls the control at each reported work checkpoint (`:237-264`). The ledger law `gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation` proves an injected cancel prevents a late offer (`:708-738`).

That is not yet an HTTP cancellation proof. `submit_gis_map_job` calls synchronous `runtime.infer(...)` inline at `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:516`, with no worker hand-off/yield between checkpoints. The cancel endpoint can only run concurrently if the server schedules another worker; the targeted tests can use a current-thread Tokio runtime and there is no held-checkpoint HTTP law. Do not claim that a user pressing Cancel interrupts a live HTTP request until a controlled executor/process test demonstrates it.

There is a second, narrower cancel gap after approval preparation. `approve_gis_map_job` persists a prepared outbox row then awaits commit (`:616-646`). `request_cancel` first records `cancel-requested` (`🪶️sqlite/🦀️.rs:295-310`), but `cancel` returns `Conflict` if that prepared row exists (`:369-383`). The current unavailable committer leaves precisely that prepared row. Thus an owner cannot cleanly withdraw a 503 approval attempt, and a later recovery worker must not treat that prepared row as a free-standing authorization to publish.

## Approval/publication firewall and the future trap

The approval request carries no effect, actor, scope, command, module URL, or Map bytes (`🌎️hub/💡️inference/🧬️schema/✅️approval/🦀️.rs:7-24`). Hub reconstructs `CreateRegion` plus inverse from the verified current base and verifies the stored proposal hash before producing canonical server-stamped bytes (`🏃️runtime/🦀️.rs:274-300,616-638`). The private committer trait expressly excludes `ArtifactHandle::submit` and `db.pathmap.v1`, and ledger reconciliation requires an exact committed-WAL witness (`:115-137,329-356`; `🪶️sqlite/🦀️.rs:449-466`). Those are the correct firewalls.

However, `GisMapApprovalCommitRequestV1` carries only a parent-style canonical command byte slice (`🏃️runtime/🦀️.rs:87-98`). It does **not** carry a non-forgeable typed Map group plan. The actual Map unit of work is `GisMapCreateRegionGroupWorkV1`: one parent mutation/inverse, one `gismap-drawing` child mutation/inverse, and one `gismap-value` child mutation/inverse (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45-55,92-164`). Its source rejects an image-bearing composition and derives/proves the two child effects from the exact snapshot.

Therefore the current unavailable configuration is safe, but simply replacing it with a committer that applies the canonical parent `CreateRegion` would create the bypass this task asks about. The durable triple implementation remains deliberately out of scope here; `📓️terra-map-durable-group-current-frontier.md` is the current journal/visibility audit. The narrow integration rule is:

1. At approval, reconstruct `GisMapCreateRegionGroupWorkV1` server-side from the same freshly materialized Map base and job id.
2. Give the private composition committer that typed work plus the frozen identity/base frontier; keep the canonical command only as an audited/idempotent receipt digest, never as the authority to apply one member.
3. Immediately before its irreversible anchor/visibility decision, recheck active session generation, `Author` membership, unchanged descriptor/catalog/base frontier, and cancellation. The existing trait supplies neither the session/generation nor an operation control, and Hub's last authorization check is before `map_base`, `prepare_approval`, and the awaited committer call (`🏃️runtime/🦀️.rs:626-638`).
4. Return a witness only after all three members are durable/visible together. Only then reconcile the private outbox and allow ordinary document fanout.

This is a future-wiring risk, not a present unauthorised publication: startup always injects `UnavailableGisMapApprovalCommitterV1` at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6577-6593`, and the native law confirms that this leaves the offer unapproved and emits no witness (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:765-806`).

## Smallest executable handoff

The next vertical slice should be one **owner-private proposal review + final-fenced triple approval**, not a generic AI action or an external model integration:

1. Start a local Hub with a real trusted GIS Map selection, real descriptor/active pair, and an injected test-only three-member committer. Add a private read DTO that reveals the canonical proposed region only to the same live owner; do not add it to document socket/presence/fanout frames.
2. Move the controlled GIS work to a cancellable execution turn (or introduce an internal executor port that can be held at a checkpoint); retain current identity/base rechecks at offer.
3. Make the real committer construct/accept exactly the server-derived typed triple, take a final authorization/revision/cancel fence, and return the existing exact witness. The client-side approval remains only `{jobId,proposalHash}`.
4. Then add the closed GIS Shell action and authenticated browser worker transport. The Map editor should render owner-private proposed geometry and status; a second user only receives the normal post-commit document delta. No WGPU/rendering or model-provider claim belongs to this server slice.

## First failing acceptance laws and reusable fixtures/gates

| Proposed first law | Exact assertion | Reuse | Why it fails/is absent today |
| --- | --- | --- | --- |
| `gis_map_authenticated_route_owner_scope_and_private_preview` | A configured trusted Map profile lets Author A submit and read the proposed region; same-space Author B, cross-space Author C, viewer, stale generation and wrong document all receive no job bytes. | `🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1`, `🗺️gis-inference-job-v1`, `🗳️gis-map-proposal-approval-v1`, live-Author fixture. | Existing `gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding` only proves 503/no disclosure (`🚀️bin.rs:7309-7341`); there is no happy-profile route or owner preview route. |
| `gis_map_http_cancel_interrupts_held_executor_without_offer` | Hold the executor at a checkpoint, send the authenticated cancel request, observe bounded terminal cancellation, zero proposal and no retained operation; repeat on a current-thread test runtime. | `🗳️gis-map-proposal-approval-v1` cancel lifecycle and the existing operation controller. | The current inline synchronous call has no deliberately interleavable HTTP test seam. |
| `gis_map_approval_final_fence_rejects_revocation_frontier_and_cancel` | After an approval reaches the pre-commit fence, revoke/change role, alter base frontier, or cancel; each yields no group anchor/witness/normal fanout. | Author fixture plus a barrier committer and authority-pair fixture. | The current committer request cannot revalidate session generation/control, and current code checks authorization before the awaited commit. |
| `gis_map_approved_region_is_exact_parent_drawing_value_group` | Approval emits all and only Map, `gismap-drawing`, `gismap-value`, preserves/rejects image according to the GIS group law, and gives one witness; a parent-only command is rejected. | `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🧩️map-create-region-group` and `GisMapCreateRegionGroupWorkV1` native law. | Production committer is unavailable; the Hub request shape still permits only a parent canonical command. |
| `gis_map_two_author_process_private_then_one_visible_delta` | A requests/reviews/approves, B gets no job/proposal result but sees exactly one normal committed document update; retry/restart reconciles one witness only. | Existing Hub proposal fixture plus the durable-group receipt corpus when its owner lands it. | No browser caller, configured trusted-profile process gate, or triple transaction exists. |

Existing targets are prerequisites only, and were not run:

```sh
bun nx run os-hub:gis-map-proposal-check --process
bun nx run @semio-tech/gis-plugin:map-create-region-group-check
bun nx run @semio-tech/gis-plugin:map-create-region-group-native-check
```

The Hub script itself says `gis-map-proposal-check --process` does **not** run the two-user journey because it still needs a trusted profile and atomic composition transaction (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:5061-5094`). Its current native laws prove owner ledger privacy and the unavailable-committer fail-closed path, not successful map approval. The GIS source gate itself explicitly makes no durable-publication claim (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:110-187`).

## Nonclaims

- No external AI/model provider, tool execution, browser request, WGPU rendering, or end-to-end process test was found or run.
- No successful trusted-profile job route or committed three-member Map edit is claimed.
- The current code does not leak a different space's Map/result through the reviewed endpoints; it also does not yet expose an owner-readable proposal suitable for UI review.
- This audit does not redo the WAL/group-journal design; it relies on the existing durable-group frontier and only identifies the necessary Hub-to-group integration boundary.
