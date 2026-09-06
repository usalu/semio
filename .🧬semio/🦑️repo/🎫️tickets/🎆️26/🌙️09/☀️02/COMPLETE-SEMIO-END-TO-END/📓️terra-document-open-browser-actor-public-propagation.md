# Document-Open Browser-Actor Public Propagation

Read-only implementation packet, 2026-09-06. This covers mandatory identity/lease binding only. It expressly does not add an actor body route, Worker fetch, Worker activation, or containment claim.

## Exact public contract

The domain already owns the two required path-free types and their only legal projection:

* `DocumentOpenBrowserActorV1` is a plan identity with `none|closed-browser-actor`, no byte length, and source component/descriptor binding.
* `DocumentExecutionTargetBrowserActorV1` is the corresponding lease identity with the additional bounded `byteLength`.
* Rust [`DocumentOpenBrowserActorV1::to_lease`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs:123) and TypeScript [`documentBrowserActorLeaseFromPlanV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:76) already enforce the complete relation: `none` requires React/WGPU and no length; closed requires Wasm, exact package source digests, sorted admitted imports, and exactly one `1..=64 MiB` length.

Add one **required** field in both records (do not make either optional):

```text
DocumentOpenPlanV1.browserActor: DocumentOpenBrowserActorV1
DocumentExecutionTargetLeaseFieldsV1.browserActor: DocumentExecutionTargetBrowserActorV1
```

Place the fields next to the selected rendering data—after `surface` and before `grant` in the plan; after `descriptor` and before `artifact` in the lease. Neither form carries a private path, bytes, module URL, component/descriptor source file, provider handle, receipt, or socket grant.

## Shared Rust/TypeScript parser and projection delta

### Rust

1. Add `browser_actor` to [`DocumentOpenPlanV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1423). In `validate` ([1511](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1511)), call:

```rust
self.browser_actor.validate(
    DocumentBrowserActorSourceV1 {
        component_sha256: &self.package.component_sha256,
        descriptor_byte_sha256: &self.package.descriptor_byte_sha256,
    },
    renderer_name(self.surface.renderer_target),
)
```

`renderer_name` is a private total match over the three existing `DocumentOpenRendererTargetV1` variants ([1327](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1327)); do not stringify debug output or introduce a caller renderer string.

2. Add `browser_actor` to [`DocumentExecutionTargetLeaseFieldsV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1614), validating it by the same private source/renderer relation in `validate` ([1632](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1632). `same_lease_fields_v1` is already structural equality ([1724](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1724)); no parallel subset comparator is needed.

3. Change the sole shared projector at [`lease_fields_from_plan_v1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1701) to:

```rust
pub fn lease_fields_from_plan_v1(
    plan: &DocumentOpenPlanV1,
    component_byte_length: u64,
    descriptor_byte_length: u64,
    browser_actor_byte_length: Option<u64>,
) -> Result<DocumentExecutionTargetLeaseFieldsV1, DocumentOpenPlanErrorCodeV1>
```

It must call `plan.browser_actor.to_lease(...)` with the plan package hashes, the plan renderer, and only that `Option<u64>`; map the actor error to `Denied`. Returning `Result` is necessary because a closed plan with absent/zero/oversized length and a `none` plan with a supplied length are invalid projections, not representable leases.

### TypeScript

1. Add the same required fields to [`DocumentOpenPlanV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1002) and [`DocumentExecutionTargetLeaseFieldsV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1191).
2. Make `parseDocumentOpenPlanV1` ([1099](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1099)) require `browserActor` and call `parseDocumentOpenBrowserActorV1` only after parsing package and surface.
3. Make `parseDocumentExecutionTargetLeaseFieldsV1` ([1217](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1217)) require and parse `browserActor` with `parseDocumentExecutionTargetBrowserActorV1` after parsed package/surface. `sameLeaseFieldsV1` ([1320](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1320) must call `sameDocumentBrowserActorV1` in addition to its existing full-field relation.
4. Change [`leaseFieldsFromPlanV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1299) to accept `Readonly<{ component: number; descriptor: number; browserActor?: number }>` and put `documentBrowserActorLeaseFromPlanV1(plan.browserActor, packageSource, plan.surface.rendererTarget, byteLengths.browserActor)` in the canonical parsed output. Its existing parse-on-return gives the same fail-closed behavior as Rust's `Result`.

This deliberately allows schema-level projectors to receive a number while requiring the only production owner to derive it from verified retained bytes below. The projector does not grant body access or make a caller-selected length authoritative.

## Hub owner and route propagation

The only trusted values are already available at the two existing protected selections.

| Owner | Current anchor | Mandatory field source |
|---|---|---|
| Open-plan retained authority | [`DocumentOpenPlanAuthorityV1`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1147), `public_plan` [1198](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1198) | Add `DocumentOpenBrowserActorV1`; validate it against the authority package/surface; project it unchanged only in `public_plan`. |
| Open-plan route assembly | [`issue_document_open_plan_inner`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2254), authority literal [2309](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2309) | Copy only `selected.browser_actor` from the already verified catalog selection. Neither intent nor headers gain an actor selector. |
| Protected target manifest | [`document_execution_target_selection`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2429), fields literal [2487](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2487) | Derive `browser_actor_byte_length` only as `assets.browser_actor.as_ref().map(|bytes| u64::try_from(bytes.len()))`; call `assets.selection.browser_actor.to_lease` using the selected package/surface. `assets_for_current_selection` has already made `Some(bytes)` possible only for exact closed WASM identity ([catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:380)). Any `none+Some`, closed+`None`, bad cast, or validation failure maps to `ComponentUnavailable`; return no manifest. |
| Existing target response router | [`DocumentExecutionTargetAssetV1`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2425), match [2580](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2580) | Leave this enum and its three routes unchanged. A manifest identity/length is not authority to fetch the actor. |

Use the selected `assets` in the target-manifest route, not a plan echo or direct request JSON. This preserves the existing per-body re-authentication/revision fence and avoids granting a closed actor a body route before containment exists.

## Native and browser consumers

### Native client

[`DocumentSocketAuthorityV1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:392) must retain `DocumentOpenBrowserActorV1`, copied only from the parsed plan in the current authority literal ([1170](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1170). Change its `lease_fields` to take `browser_actor_byte_length: Option<u64>` and return `Result`; `matches_lease_fields` derives that option from the candidate lease actor variant and returns `false` on a failed projection. Migrate the two existing shared-projector calls:

* socket expectation comparison at [1143](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1143);
* neutral fixture/cross-check at [2314](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:2314) and [2352](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:2352).

No native client actor loader is introduced by this change.

### Browser worker

The worker has exactly one suitable relation: `receiptFreeFields` ([582](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:582), used for manifest equality at [741](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:741) and installed-target authority at [786](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:786)). Feed it:

```ts
browserActor: fields.browserActor.kind === "closed-browser-actor"
  ? fields.browserActor.byteLength
  : undefined
```

for a manifest, and the equivalent exact option from an installed target. The parser and shared equality then reject every identity/length substitution. Do not add `browser-actor` to `DocumentExecutionTargetAssetV1`, `executionTargetAssetPath`, the allowlist regex ([528-605](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:528), or `DocumentExecutionTargetLease` buffers. The worker continues to fetch and retain only manifest/component/descriptor; a closed identity is metadata until the later containment owner exists.

## Full constructor and corpus census

All currently in-scope direct constructors are below; no compatibility default is permissible because all DTOs deny unknown/missing fields.

| Site | Exact update |
|---|---|
| Hub route and test authority | `DocumentOpenPlanAuthorityV1` literals at [2309](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2309), [8146](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8146), and selected reassignment in [8280](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8280): preserve the catalog-selected actor, including when test target is `none`. |
| Hub manifest direct literal | [2487](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2487): use only verified `assets` actor body length. |
| Hub React mock catalog | [618](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:618), selections [8214](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8214): keep `None` with no actor bytes; it proves `none` works without creating a fake closure. |
| Directory native client | Authority literal [1170](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1170), test authority [2355](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:2355), and the shared projector calls listed above. |
| Store sync local lease fixture | [`fixture_execution_target_lease`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:3400): it is WGPU, so add `DocumentExecutionTargetBrowserActorV1::None`. |
| Neutral Rust plan fixture | [document-open-plan](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json:66): add `browserActor:{kind:"none"}` to both React catalog rows and `validPlan`; add absence, closed-on-React, `none`-on-Wasm, source component/descriptor mismatch, policy/hash/interface mutation vectors. |
| Browser generic open fixture | [browser-document-open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json:15): both installed target and React plan carry `none`; update the full-field hostile matrices to include it. |
| Browser execution-target fixture | [execution-target lease](/Users/ueli/Documents/semio/🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json:16): its Wasm plan gets a closed identity and manifest gets the same identity plus bounded length. Add each of schema, policy, four digests, each interface row/order, `kind`, and `byteLength` to the existing `manifest-field` denial rows. This fixture must not add an actor body or route. |
| Test-only casts | [inference harness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4160): remove/replace the partial `as unknown as DocumentExecutionTargetLeaseFieldsV1` builder with a complete `none` field so it cannot bypass the new parser relation. |

## Minimal acceptance laws

1. Rust and TS decode the same closed Wasm plan and lease, and both reject every plan identity mutation plus each lease identity/length mutation; React/WGPU `none` remains accepted only with no lease length.
2. A Hub open-plan response exposes precisely the verified catalog identity; a forged request `browserActor`, closed-vs-none mismatch, or actor source hash mismatch receives no plan/socket grant.
3. The manifest derives its length from retained catalog actor bytes. `none+body`, closed-without-body, wrong length, and any closed identity mismatch deny before an asset response. The three existing asset paths remain the complete route allowlist.
4. Browser worker rejects a plan/manifest actor mismatch before component/descriptor body requests are retained; it makes no actor body request. Existing closed-Wasm fixture proves `sameLeaseFieldsV1` includes all actor fields; existing React fixture proves `none` passes.
5. Native `DocumentSocketAuthorityV1` and store-sync WGPU fixture reject an actor-only lease substitution through the same shared relation.

No build was launched and no product source was modified by this audit.
