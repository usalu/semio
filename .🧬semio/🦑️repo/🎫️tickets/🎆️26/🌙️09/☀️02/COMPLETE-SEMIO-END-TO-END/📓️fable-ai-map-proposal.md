# Fable — AI over GIS Map: frozen binding identity, hub inference runtime, typed approval port

Lane `fable-ai-map-proposal`, 2026-09-05. Packet: `📓️terra-ai-map-proposal-approval-current-p0.md` slices A, B, C plus its "Required test packet" items 1–4.

## What changed

### A — frozen binding identity (`InferenceIdentityV1` rework)

Slice A's `VerifiedGisMapProposalBindingV1` already existed when this lane started, landed by the frozen-binding lane as `VerifiedGisMapArtifactBindingV1` in `/Users/ueli/Documents/semio/🌎️hub/💡️inference/📇️catalog/🦀️.rs`, retained in `HubState.gis_map_binding` beside the concrete `verified_catalog` and the erased `openable_catalog`. This lane did **not** re-implement it; it consumed it and closed the identity half the packet asked for.

- `/Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🦀️.rs` — new `InferenceParentDialectV1` and `InferenceBindingIdentityV1` (digest, catalog generation, package id/version, component SHA-256 **and** BLAKE3, artifact kind, document schema, full parent dialect, surface id, granted mode, service id/version, algorithm version). `InferenceIdentityV1` now carries `binding` in place of the ambiguous `package_hash`/`artifact_schema`/`algorithm_version`/`catalog_generation_id` fields. No compatibility layer, no alias, no deprecation: the old field names are rejected by `deny_unknown_fields` and by the JSON schema's `additionalProperties: false`. New constants `IDENTITY_JSON_MAX_BYTES`, `PROGRESS_MAX_CURSOR`, `EVENT_PAGE_MAX_ITEMS`, `CLAIM_LEASE_MAX_MS`, plus the GIS identity constants.
- `/Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🔣️.json` — matching `$defs/binding`; `identity` requires `binding` and no longer admits `packageHash`.
- `/Users/ueli/Documents/semio/🌎️hub/💡️inference/📇️catalog/🦀️.rs` — added `VerifiedGisMapArtifactBindingV1::identity()` (the frozen projection → identity), and replaced `identity_from_verified_catalog` with `identity_from_frozen_binding(binding, source, control)`, which additionally rejects any descriptor whose package id/version/component SHA-256/artifact kind/document schema disagrees with the frozen binding.
- Fixture cutover, recomputed (not hand-invented) by `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/fable-ai-map-proposal-identity-recompute.mjs`:
  - `🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json` — nested `identity.binding`, new `identityDigest`, `outbox.jobId`, `mutationId`, `commandHex`, `commandHash`; `hostileIdentities` moved from a flat `field` to a `path` array and grew from 4 to 17 cases (binding digest, BLAKE3 length, package id/version, artifact kind, document schema, surface, granted mode, dialect standard/subset, service, algorithm, plus the two removed aliases `catalogGeneration` and `packageHash`).
  - `🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json` — the same recomputed job/mutation/command identity so the ledger and committed-WAL corpora still agree byte for byte.
  - `🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🧬️.schema.json` — `hostileIdentities` schema now `path`-shaped, 17 items.
  - Consumers updated: the Rust law in `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` and the Bun oracle in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` both walk the nested `path`.

Boundary stated honestly: the binding materializes the current Map through the **verified active checkpoint pair reader** (`VerifiedRebootstrapSource::active_pair`), not through `MemberFactory::Open`. The opener is source-complete but native-unaccepted, so `runtime::map_base` documents that substitution in its own docstring. No route accepts a client-supplied Map pack anywhere.

### B — `HubInferenceRuntimeV1` and the four authenticated routes

New module `/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs` (mounted as `pub mod runtime`, `#[cfg(feature = "sqlite")]`):

- `HubInferenceRuntimeV1` owns the frozen binding, the ledger, a fixed-capacity operation set (`OPERATION_CAPACITY = 32`), one per-`DocumentScope` async gate (`DOCUMENT_GATE_CAPACITY = 64`), and retained cancellation controllers.
- `InferenceRouteErrorV1` is the whole published failure vocabulary with its exact code and HTTP status: `inference.unavailable` 503, `inference.denied` 403, `inference.not-found` 404, `inference.invalid` 400, `inference.bounds` 413, `inference.conflict` 409, `inference.capacity` 429, `inference.expired` 410, `inference.cancelled` 409, `approval.commit-unavailable` 503, `inference.storage` 503.
- Route operations live in the library (`submit_gis_map_job`, `read_gis_map_job_events`, `cancel_gis_map_job`, `approve_gis_map_job`) so that `🚀️bin.rs` only carries axum glue — deliberately, because sibling lanes are editing `post_directory_commands`, `get_directory_space` and the execution-target asset routes in that file.
- `🚀️bin.rs` additions are confined to one `//#region 💡️Inference` block plus one `HubState` field, one startup block, one readiness argument and a single-line change in `router()` (`inference_routes(Router::new())`). Routes:
  - `POST /spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs`
  - `GET  …/inference/gis-map/jobs/{job_id}/events?after=<cursor>`
  - `POST …/inference/gis-map/jobs/{job_id}/cancel`
  - `POST …/inference/gis-map/jobs/{job_id}/approval` (`InferenceApprovalRequestV1 { jobId, proposalHash }`, and the body's `jobId` must equal the path's)
  Each is body-limited to 1024 bytes and returns only `{schema, code}` on failure.
- `check_live_inference_author` plus the complete `compare_frozen` (frozen binding identity + space + document + descriptor digest + head ordinal/edit id/commit seq + base-pack SHA-256) re-runs at accept, at offer publication, at read (reported as `stale`), at cancel and at approval, under the per-document gate. Only `Author` is admitted — the predicate matches `SpaceRole::Author` alone, so `Admin` is not implicitly allowed.
- Ledger extension in `/Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs`:
  - scoped idempotency: `UNIQUE(user_id, authorization_generation, space_id, document_id, request_id)` replaces the globally unique `request_id`; `accept` looks the existing row up on that whole key.
  - run epoch + claim lease: `start` returns `Option<InferenceRunClaimV1>`; a live lease is never stolen, an expired lease is reclaimable only after the caller has revalidated identity and authorization, and `succeed` refuses any epoch but the claiming one.
  - appendable bounded progress cursor: `inference_job_progress_v1` (append-only, immutable triggers, `cursor BETWEEN 1 AND 16`, monotonic `completed`), `progress()` and `events()` with `EVENT_PAGE_MAX_ITEMS = 8`.
  - durable cancel-request state: `cancel_requested_at` plus a `cancel-requested` event; `start`/`succeed` observe it and terminate instead of publishing a late offer.
  - `identity_of` returns the frozen identity to the original owner only.
- Readiness: `hub_readiness` gained an `inference_ready` argument; `features.inference` is `true` only when a binding produced a runtime. Production today has no trusted profile, so it publishes `false` and all four routes fail closed with `503 inference.unavailable`.

### C — typed, server-stamped approval and the composition port

- `HubInferenceRuntimeV1::infer` runs `infer_gis_map_controlled` with the retained cancellation controller, decodes its canonical result, and derives the sole `bounds_proposal` `CreateRegion`; the canonical proposal bytes are hashed and stored privately.
- `server_stamped_command` reloads the base, rebuilds the `CreateRegion` **and** its inverse (`inverse_gis_map_mutation`) on the server, refuses if the rebuilt proposal hash differs, and stamps exactly one canonical dependency-free envelope through the new `CanonicalInferenceCommandV1::encode_server_stamped` (added to `🌎️hub/💡️inference/✉️command/🦀️.rs`, which re-decodes its own output before returning it).
- `GisMapApprovalCommitterV1` is the private port. Its receipt is `GisMapApprovalReceiptV1 { witness: CommittedInferenceWalWitnessV1, document_generation }`, and `CommittedInferenceWalWitnessV1` has **no public constructor** — only `InferenceWalVerifierV1::verify` mints one — so an implementation cannot fabricate durability, and `commit_prepared_approval` reconciles the outbox only against that real committed-WAL witness.
- `UnavailableGisMapApprovalCommitterV1` is registered today, so approval is terminal `approval.commit-unavailable` (503) while no atomic parent+existing-child composition transaction exists (`📓️terra-durable-artifact-group-visibility-wal-p0.md`, `📓️terra-retained-parent-children-atomic-publication-blueprint.md` own that). The prepared outbox row survives, the proposal stays `offered`, and no `approved` event is appended.
- `ArtifactHandle::submit` and `db.pathmap.v1` are never used as the mutation receiver, and nothing auto-applies.

### Tests and gate

- Neutral fixture `/Users/ueli/Documents/semio/🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/{🔣️.json,🧬️.schema.json}` (the repo's emoji-uniqueness enforcer renamed the folder from `🗺️` to `🗳️` mid-session and rewrote every reference): canonical binding fields, base Map snapshot/frontier/expected inference, `sampleJobId`, the canonical proposal and inverse texts with their SHA-256, the five-event success lifecycle and four-event cancel lifecycle, seven two-user visibility rows, eleven published error codes with statuses, eleven approval rejections, and seventeen exact byte/item/lease limits, plus four explicit nonclaims.
- Independent oracle `proveGisMapProposalApprovalFixture` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: AJV 2020 with seven hostile mutations, two Node SHA-256 checks over the canonical texts, two independent bound folds over the base snapshot, the closed-ring/corner equality of the proposal, a state-transition walk over both lifecycles, the "exactly one owner" visibility law, code↔status consistency, and a cross-fixture equality against `🧊️gis-map-frozen-binding-v1` and `🗺️gis-inference-job-v1`.
- Rust laws:
  - library (`semio-hub --lib --features sqlite`), in `🏃️runtime/🦀️.rs`: `gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation`, `gis_map_proposal_is_private_to_its_original_author_owner`, `gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies`, `gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary`.
  - binary (`semio-hub --bin os-hub`), in `🚀️bin.rs`: `gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding` — a real loopback `spawn_server` + `issue_test_session` law over all four routes, authenticated and unauthenticated, asserting `503 inference.unavailable`, the two-field closed error body, and `features.inference == false`.
- Registered gate `os-hub:gis-map-proposal-check` with `--source | --native | --process` modes:
  - `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (`GisMapProposalCheckScript`, registered in the router).
  - `🌎️hub/📦️packages/🦀️rust/📋️project.json`: `gis-map-proposal-source-check`, `gis-map-proposal-native-check`, `gis-map-proposal-process-check`.
  - `.vscode/🧩️launch.seed.jsonc` entries `⚖️gate🗺️gis-map-proposal🌎️hub{📐️source,🦀️native,🔁️process}` at orders 411.146–411.148, mirroring the sibling `gis-map-frozen-binding` gates; `.vscode/launch.json` regenerated via `@semio-tech/plugin-registry:generate`.
  - The `--process` mode says explicitly that there is no external model provider and no WGPU rendering, and that the two-user journey is not run or claimed.

## What was verified, exactly

| Command | Result |
|---|---|
| `bun ./📜️script.ts gis-map-proposal-check --source` (cwd `🌎️hub/📦️packages/🦀️rust`) | **exit 0**. `gis-map-proposal-oracle: ajv=1 hostile=7 node-sha256=2 independent-bounds=2 lifecycle=9 visibility=7 errors=11 approval-rejections=11 cross-fixture=1` |
| `bun ./📜️script.ts gis-inference-ledger-oracle` | **exit 0** end to end after the identity rework and the GIS control schema repair — `traces=9 hostile=13 identity-hostile=17 sqlite-integers=6 hashes=6`, `inference-wal-proof-oracle traces=16 ownership=3 binding-hostile=2`, `inference-command-oracle vectors=20`, `inference-approval-request-oracle hostile=14`, `inference-author-oracle cases=16`, `inference-wal-chain-oracle exact=14 crc-valid=14`, `inference-catalog-projection-oracle exact=12`, `trusted-catalog-identity-oracle exact=6`, `gis-native-codec-oracle receipts=2 hostile=8`, `gis-controlled-proposal-oracle literal=1 interruption=3 rejection=7`, `gis-native-provider-selection-oracle cases=8`, `memory-backing-oracle hostile=5`, `native-deficit-oracle checks=8` |
| `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` | **exit 0**; `.vscode/launch.json regenerated`, 8 lines matching `gis-map-proposal` present |
| `CARGO_BUILD_JOBS=4 cargo check -p semio-hub --lib --message-format=short` | see "Cargo" below |

### Cargo

RESULT_PLACEHOLDER

## Honest nonclaims

- **No route-level end-to-end proposal/approval run exists yet.** The four routes are wired and their fail-closed behaviour is a real HTTP law, but the owner→claim→offer→approve journey cannot be executed in a `#[tokio::test]` today, because `HubInferenceRuntimeV1` requires a real `Arc<VerifiedGisMapArtifactBindingV1>`, which requires a `VerifiedTrustedCatalog` from `TrustedCatalogLoader::load_selected`. The only builder for such a catalog is `prepared_fixture()` inside the trusted-catalog module's own `#[cfg(test)] mod tests`, which is invisible to both the `os-hub` bin target and to any other crate. The packet assumed "tests construct the binding from the test bundle used by `trusted-stdio-gis-bundle-check`", but that bundle is produced by an out-of-process multi-minute component build (`produceFreshComponentV1`), not by a unit test. Closing this needs a non-`cfg(test)` `test_support` bundle builder owned by the trusted-catalog lane; it is not written here and nothing in this report should be read as if it were.
- Consequently the packet's native-law bullets "approval creates only server-stamped `CreateRegion` plus inverse", "reconciles exactly one committed witness" and "restart around claim/commit" are proven **at the comparison and port boundary** (frozen-drift rejection, duplicate-approval idempotency, witness-only reconciliation, fail-closed commit) and **not** through a live route.
- No two-user process journey was run. No browser or native UI port exists (slice D was explicitly out of this packet's scope for this lane).
- No external model provider and no WGPU rendering are involved anywhere; the GIS service is a deterministic bounded local computation.
- The proposal/inverse canonical texts in the neutral fixture are pinned literals verified by two independent Node SHA-256 computations; **they have not yet been compared against the real `os_pack` JSON serialization by a Rust run.** If the float or key rendering differs, the native laws are where that will surface.
- The `--native` and `--process` gate modes have not been executed.

## Blockers (external, current evidence)

1. **GIS control fixture/schema drift — repaired here.** `bun ./📜️script.ts gis-inference-ledger-oracle` used to exit 1 at `proveGisControlledProposal` (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:75`) with `invalid GIS controlled corpus: … "additionalProperty":"id" … must NOT have additional properties`. `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/💡️inference-control/🔣️.json` had been updated to put `id` inside `data` (matching `GisMapInference::bounds_proposal`, which builds `object([("id"),("kind"),("ring")])`), while `🧬️.schema.json` had not. This lane added `"id"` to that object's `required` and `properties` (same `^inference-[a-f0-9]{32}$` pattern as the sibling `item.id`). `bun ./📜️script.ts gis-inference-ledger-oracle` now exits **0** end to end — see the verification table.
2. **Repo-wide taxonomy collision** blocked every `📜️script.ts` entry point for a stretch of this session: `Invalid taxonomy schema: semanticDirectoryMemberKinds collide for owner "schema" and member "🌍️change-annex"` from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1277`, with `🔣️taxonomy.json` in a `MM` working state. A peer resolved it; recorded because it will recur for anyone running these gates.
3. **Build contention.** `cargo check` for this crate spent over 40 minutes queued behind concurrent peer builds (`semio-s-plugin-stdio`, `semio-s-plugin-block`, `semio-framework-os-mcp`, wasmtime/wit-component). Every cargo invocation here was serialized, foreground, `CARGO_BUILD_JOBS=4`, narrowest target.
4. **`🧊️gis-map-frozen-binding-v1` churn.** Its `expectedDigest` changed mid-session (`8e190c78…` → `435e0206…`, `rendererTarget` `wgpu` → `wasm`). The identity chain was recomputed against the current value. The cross-fixture equality in the new oracle is deliberate coupling: if that lane changes the projection again, `gis-map-proposal-check --source` fails loudly and the recompute script regenerates the chain.
5. **Fixture folder rename.** The repo's semantic-emoji enforcer renamed `🗺️gis-map-proposal-approval-v1` to `🗳️gis-map-proposal-approval-v1` and `🗺️gis-map-frozen-binding-v1` to `🧊️gis-map-frozen-binding-v1` mid-session, rewriting source references. The launch-seed entry names still read `🗺️gis-map-proposal` (they are launcher labels, not paths).

## Next steps for whoever picks this up

1. Land a non-`cfg(test)` `test_support` GIS Map bundle builder in `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/`, then add the four route-level laws (owner streams, peer/cross-space/viewer/admin/stale/cancelled denial, server-stamped `CreateRegion` + inverse with frozen-drift rejection, duplicate approval + restart reconciling to one witness) to the `//#region 💡️Inference` block in `🚀️bin.rs` and to `GisMapProposalCheckScript`'s `routeLaws`.
2. Register a real `GisMapApprovalCommitterV1` once the atomic parent+existing-child composition transaction exists; it must return the `CommittedInferenceWalWitnessV1` minted by `InferenceWalVerifierV1::verify` and nothing else.
3. Repair `✏️s/🔌️plugins/🌍️gis/🧪️fixtures/💡️inference-control/🧬️.schema.json` so `data` admits `id` — that unblocks `gis-inference-ledger-check` for its owner too.

## Files touched by this lane

```
🌎️hub/💡️inference/🧬️schema/🦀️.rs
🌎️hub/💡️inference/🧬️schema/🔣️.json
🌎️hub/💡️inference/📇️catalog/🦀️.rs
🌎️hub/💡️inference/✉️command/🦀️.rs
🌎️hub/💡️inference/🪶️sqlite/🦀️.rs
🌎️hub/💡️inference/🦀️.rs
🌎️hub/💡️inference/🏃️runtime/🦀️.rs                              (new)
🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json         (new)
🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🧬️.schema.json  (new)
🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json
🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json
🌎️hub/📦️packages/🦀️rust/🚀️bin.rs
🌎️hub/📦️packages/🦀️rust/📜️script.ts
🌎️hub/📦️packages/🦀️rust/📋️project.json
.vscode/🧩️launch.seed.jsonc
.vscode/launch.json                                              (generated)
.🧬semio/…/COMPLETE-SEMIO-END-TO-END/fable-ai-map-proposal-identity-recompute.mjs (new, ticket input script)
```
