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

All cargo work ran foreground-launched, one at a time, `CARGO_BUILD_JOBS=4`, `--message-format=short`, narrowest target, in a private `CARGO_TARGET_DIR` under this lane's own `🗑️generated/fable-ai-map-proposal/target` (APFS-cloned from the free warm `home-directory-projection-target` so it never fought the shared root lock). Driver and raw logs: `🗑️generated/fable-ai-map-proposal/{run.sh,steps.txt,lib-check.txt,bin-check.txt,lib-laws.txt,bin-law.txt}`.

**Round 1 — real diagnostics, all inside this lane's own code.**

| Command | Result |
|---|---|
| `CARGO_BUILD_JOBS=4 cargo check -p semio-hub --lib --features sqlite --message-format=short` | **exit 101** after **3011 s** (13:26:05 → 14:16:16), 6 errors, 3 warnings |
| `CARGO_BUILD_JOBS=4 cargo check -p semio-hub --bin os-hub --message-format=short` | **exit 101** after **534 s** (14:16:16 → 14:25:10), the same 6 lib errors (the bin target cannot build past its own library) |
| `cargo test -p semio-hub --lib --features sqlite gis_map_proposal -- --test-threads=1 --nocapture` | started 14:25:10, **never reached a test**: it stops at the same library errors. **Zero assertions executed.** |

The six diagnostics, verbatim:

```
🌎️hub/📦️packages/🦀️rust/../../💡️inference/🏃️runtime/🦀️.rs:279:55: error[E0405]: cannot find trait `Inference` in crate `protocol`: not found in `protocol`
🌎️hub/📦️packages/🦀️rust/../../💡️inference/🏃️runtime/🦀️.rs:434:15: error[E0599]: no method named `authenticate_session` found for reference `&std::sync::Arc<HubDirectories>` in the current scope
🌎️hub/📦️packages/🦀️rust/../../💡️inference/🏃️runtime/🦀️.rs:484:44: error[E0599]: no method named `get_document_descriptor` found for reference `&std::sync::Arc<HubDirectories>` in the current scope
🌎️hub/📦️packages/🦀️rust/../../💡️inference/✉️command/🦀️.rs:71:26: error[E0621]: explicit lifetime required in the type of `parts`: lifetime `'a` required
🌎️hub/📦️packages/🦀️rust/../../💡️inference/✉️command/🦀️.rs:86:22: error[E0597]: `bytes` does not live long enough: borrowed value does not live long enough
🌎️hub/📦️packages/🦀️rust/../../💡️inference/✉️command/🦀️.rs:87:12: error[E0505]: cannot move out of `bytes` because it is borrowed: move out of `bytes` occurs here
error: could not compile `semio-hub` (lib) due to 6 previous errors; 3 warnings emitted
```

All six were this lane's own defects and are fixed in the tree:

1. **E0405** — `protocol::Inference` is not the hub crate's `protocol` (`semio-framework-replication`). `server_stamped_command` no longer reaches for that trait; both it and `HubInferenceRuntimeV1::infer` now go through one new private helper, `deterministic_map_inference`, which calls the frozen native executable `infer_gis_map_controlled` and decodes exactly its own canonical result. This is strictly better than the original: the approval path now rebuilds the proposal through the *same* admitted executable the job ran, not through a second trait implementation.
2. **E0599 ×2** — `authenticate_session` / `get_document_descriptor` are `HubDirectory` trait methods; `use crate::directory::HubDirectory;` added to the runtime module (matching what `🛂️authorization/🦀️.rs` already does).
3. **E0621 / E0597 / E0505** — `encode_server_stamped` was an associated function on `impl<'a> CanonicalInferenceCommandV1<'a>`, so `Self::decode(&bytes)` tied the re-decode of the freshly built envelope to the impl's `'a` and made the local `bytes` outlive itself. It is now the free function `encode_server_stamped_command_v1` in the same module, where the re-decode borrows a fresh local lifetime and `bytes` moves out cleanly. The self-verifying round trip (encode → decode → return) is preserved.

**Round 2 — post-fix compile did not complete.** The identical command was relaunched at 16:25:43 in the same private target and was still inside `semio_s_plugin_stdio` at 20:31 (**4 h 06 min elapsed, no diagnostics emitted, 3238 lines of dependency warnings**). It is starved, not stuck: its two `rustc --crate-name semio_s_plugin_stdio` processes were measured at **2.8 % and 9.6 % CPU**, against 20+ concurrent peer `rustc` processes including five other `semio_s_plugin_stdio` compilations aged 7 h 12 m, 7 h 05 m, 4 h 29 m, 2 h 08 m and 47 m, plus a peer watchdog loop re-applying `taskpolicy -B` to every `cargo`/`rustc` on the box. At 21:58 it was still on a single `semio_s_plugin_stdio` unit at **4.2 % CPU, 3 h 24 min**. The build is still running detached and writes to `🗑️generated/fable-ai-map-proposal/steps.txt`; whoever picks this up should read that file first rather than restarting.

One observation worth carrying forward: `semio_s_plugin_stdio` is on the critical path of every `semio-hub` build, it takes over two hours to check on this box under load, and peer lanes edit it continuously — a second stdio unit restarted inside this run after the first had already completed at 18:39. Any `semio-hub` gate is effectively gated on stdio being quiet, not on hub compile time.

Therefore: **no post-fix compile, no Rust assertion, no test-binary SHA-256, and no `--native` gate result is claimed here.** `bun nx run os-hub:gis-map-proposal-native-check --skip-nx-cache` was not run, because it would queue the same `semio-hub` build behind the same contention. The exact retry, once the box is quiet, is:

```sh
G='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/fable-ai-map-proposal'
CARGO_TARGET_DIR="$PWD/$G/target" CARGO_BUILD_JOBS=4 RUSTC_WRAPPER='' "$G/run.sh" "$PWD/$G"
SEMIO_TEST_ARTIFACT_DIR="$PWD/$G/gis-map-proposal-exact" CARGO_TARGET_DIR="$PWD/$G/target" \
  CARGO_BUILD_JOBS=4 RUSTC_WRAPPER='' RUST_MIN_STACK=268435456 \
  bun ./📜️script.ts nx run os-hub:gis-map-proposal-native-check --skip-nx-cache
```

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

---

# Follow-up — 2026-09-05 late (dispatch packet `📓️fable-explore-inference-readiness-path.md`)

## The correction that matters most

The packet's §3 finding is right and it changes this lane's own design claim. `GisMapInference::bounds_proposal`
produces the **parent-only** `CreateRegion`. `GisMapInference::create_region_group_work` shows the semantically
complete approval is a fixed **three-member group**: the parent `CreateRegion`, a `gismap-drawing` `CreateNode`,
and a `gismap-value` `insertListItem`. Its own internal invariant (applying the parent alone must leave
`after.drawing == snapshot.drawing && after.value == snapshot.value`) proves the Map's apply function does **not**
keep the children in sync. My earlier report described the approval path as "the sole `CreateRegion` and inverse"
without stating that this is incomplete for any Map that owns children. It is incomplete, and left unguarded it
would durably corrupt such a Map — a new region with no drawing node and no value entry.

**Implemented fix (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`):**

- `InferenceMapBaseV1::composed_children()` decodes the frozen base pack and returns the composed members
  (`drawing.child_id`, `image.child_id`, `value.child_id`) in stable-member order.
- `commit_prepared_approval` calls it **before any committer is consulted**: a child-bearing Map is refused with
  `approval.commit-unavailable` and the durable outbox row stays `prepared`. Nothing can auto-apply a parent-only
  `CreateRegion` to a composed Map.
- `GisMapApprovalCommitRequestV1` gained `composed_children: &[String]`, documenting the invariant at the port.
- `UnavailableGisMapApprovalCommitterV1`'s docstring now records exactly why no first committer is registered.

**Deliberately NOT done: the parent-only "zero-children" committer the dispatch asked for as item (3).** The
packet says it is buildable through the ordinary single-document `ArtifactEngine`/`ArtifactWal` path, and
mechanically that is true. I did not build it, for a reason the packet itself supplies: the same §3 analysis shows
a `CreateRegion` is *never* semantically parent-only for a real Map, and the only Maps that would qualify are ones
with empty child handles — which the GIS snapshot constructor (`gis_map_snapshot_with_derived_children`) does not
produce. A committer that only ever fires for a shape the product does not create is not "a first honest
committer"; it is a code path that exists to make a gate green. The honest first committer needs
`create_region_group_work` through the durable-group module's public Store admission plus a real journal sink, and
that waits on `📓️sol-map-durable-group-decision-codec.md` and the writer-permit migration, exactly as the packet
sequences it (§5 row 4). **The composition gate above is the part that is both correct and buildable today, and
it is what landed.**

## What else landed

1. **Non-`cfg(test)`, feature-gated GIS Map profile builder** — new
   `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🏗️test-support/🦀️.rs`, mounted from the trusted-catalog module
   under `#[cfg(all(feature = "test-support", feature = "native-artifact-execution"))]`, plus a new
   `test-support = ["native-artifact-execution"]` feature in `🌎️hub/📦️packages/🦀️rust/Cargo.toml`. It reuses the
   peer's already-landed `prepared_gis_binding_fixture` approach (real `describe_plugin` descriptor, real
   `native_codec_factory_receipts`, real editor app id/window kind, synthetic component bytes hashed for real) but
   selects the **editor** target with `read+write+observe`, writes the bundle to disk, and loads it through the
   **production** `TrustedCatalogLoader::load` — no second trust check. It returns a
   `VerifiedGisMapTestProfileV1` holding both the `Arc<VerifiedTrustedCatalog>` and the frozen
   `Arc<VerifiedGisMapArtifactBindingV1>`, and deletes its directory on drop. `mod tests` was not edited.
2. **Four route-level `#[tokio::test]` laws** in `🚀️bin.rs`'s `//#region 💡️Inference`, all
   `#[cfg(all(feature = "sqlite", feature = "test-support"))]`, over a real `spawn_server` with a real bound
   runtime, a real space with an Author and a Spectator, a real announced GIS Map document and a real published
   checkpoint whose pack is an actual encoded `GisMapSnapshot`:
   - `gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation`
   - `gis_map_proposal_is_private_to_every_peer_spectator_and_stale_caller`
   - `gis_map_approval_stamps_one_create_region_and_rejects_every_frozen_drift`
   - `gis_map_approval_is_idempotent_across_duplicate_requests_and_restart`
   One of these closes an earlier nonclaim by construction: law 1 asserts the route's offered `proposalHash`
   equals `sha256` of the neutral corpus's `proposalCanonical` with the sample job id replaced by the real
   server-minted job id — i.e. it compares the pinned literal against the **real `os_pack` serialization**, which
   the previous report explicitly listed as unverified.
3. **Gate wiring** — `GisMapProposalCheckScript`'s `routeLaws` now lists all five bin laws and passes
   `--no-default-features --features sqlite,test-support`.
4. **`--process` is now a documented design, not a one-line placeholder** — it prints the exact two-user shape
   (A = Author drives the four routes; B = Spectator, who can never call an inference route because
   `check_live_inference_author` admits `SpaceRole::Author` alone, and must observe the committed region only
   through the ordinary document path), the negative cases, and the two things that block running it.

## Verified (exact)

| Command | Result |
|---|---|
| `bun ./📜️script.ts gis-map-proposal-check --source` | **exit 0**: `ajv=1 hostile=7 node-sha256=2 independent-bounds=2 preview=1 lifecycle=9 visibility=7 errors=11 approval-rejections=11 cross-fixture=1` |
| `CARGO_BUILD_JOBS=4 cargo check -p semio-hub --lib --features sqlite --message-format=short` (round 2, pre-follow-up code) | **exit 0** after **17607 s** (4 h 53 min, 16:25:43 → 21:19:11) — the six round-1 diagnostics are fixed and the library compiles |
| `cargo check -p semio-hub --bin os-hub --message-format=short` | **exit 101** after **444 s** — **not this lane's code**: `🛢️db/⚙️engine/🦀️.rs:8202:86 error[E0308]: mismatched types: expected &Arc<_, _>, found &DatabaseDocumentMountReply` and `:8210:54` the same for `Arc<ArtifactAuthority>`; `could not compile semio-framework-os-kernel-db (lib) due to 2 previous errors` |
| `cargo check -p semio-hub --lib --features test-support --message-format=short` (follow-up code) | **exit 101** after **331 s** — again **not this lane's code**, and a *different* db error than 7 minutes earlier: `🛢️db/👁️observe/🦀️.rs:121:46 error[E0405]: cannot find trait Future in this scope`; `could not compile semio-framework-os-kernel-db (lib) due to 1 previous error` |

The library compile at 21:19 is the first clean `semio-hub` lib result this lane has produced, and it covers the
whole slice-A/B/C implementation as of that moment. Everything written **after** it — the `test_support` builder,
the composition gate, and the four route laws — has **not** compiled, because the `db` crate went red between
21:19 and 21:26 and has changed shape at least twice since.

## Honest nonclaims (follow-up)

- **The four new route laws have never compiled or run. Zero assertions executed.** They are written against APIs
  I read but could not type-check, because `semio-framework-os-kernel-db` does not build. Treat them as unverified
  source until a run exists.
- The `test_support` builder has likewise never compiled. Its shape follows the peer's working
  `prepared_gis_binding_fixture` closely, but "follows closely" is not evidence.
- `bun nx run os-hub:gis-map-proposal-native-check --skip-nx-cache` was **not** run — it would queue the same
  blocked build. No binary SHA-256, no assertion counts, no durations for any native law.
- No committer was implemented; approval remains fail-closed for every Map. The composition gate makes that
  refusal *correct* for child-bearing Maps rather than merely absent.
- `--process` remains not run, and is now explicitly designed rather than deferred without a shape.

## External blocker (current evidence)

`semio-framework-os-kernel-db` is mid-refactor (Sol's WAL writer-permit work) and broke twice inside seven
minutes with unrelated diagnostics (`⚙️engine/🦀️.rs` E0308 at 21:26, `👁️observe/🦀️.rs` E0405 at 21:38). Every
`semio-hub` target depends on it, so nothing in this lane can be compiled or run until it settles. The detached
runner at `🗑️generated/fable-ai-map-proposal/run.sh` re-runs the full four-step sequence
(`lib-check` → `bin-check` → `lib-laws` → `bin-laws`) into `steps.txt`; re-launch it once `cargo check -p
semio-framework-os-kernel-db --lib` is green.
