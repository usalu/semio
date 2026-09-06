# Closed Browser Actor: Current Catalog and Lease Frontier

## Decision

Bind a derived actor as a **package-owned, source-bound execution payload** in the trusted bundle, then project that payload unchanged through the open plan, receipt-free target lease, and a fourth protected target body.  It is not a raw `PackageHashes` member, not an open-plan selector, and not a client URL.

The first record should be a required tagged value rather than a nullable `Option`.  Both Rust and TypeScript then reject an omitted field, rather than silently treating an absent non-Wasm field as `None`.  This is a greenfield schema change: update every producer/fixture in one change; do not add a v1 compatibility default.

```text
verified raw component + verified descriptor + sealed codegen policy
  -> closed browser ESM bytes + BrowserActor record
  -> immutable trusted BundlePackage + generation
  -> server-selected plan / receipt-free lease
  -> authenticated current-selection `browser-actor` body
  -> private worker hash check and later contained activation
```

The present real actor builder is fixture-qualified, but this path does not yet exist in production.  Current catalog selection returns only raw component and descriptor; current worker status `renderer-unavailable` remains accurate.

## Current Exact Gaps

| Boundary | Current owner and behavior | Required bounded change |
| --- | --- | --- |
| Fresh artifact row | [`📇️registry/📜️script.ts:2207-2237`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts) defines `FreshCatalogCommitMarker`/receipt with only `raw`, `core`, and `descriptor`; `requireExactCatalogRow` at `:2771-2779` rejects a fourth directory. | Introduce the browser payload record and atomically publish it under the selected row before its marker.  Do not overload raw/core/descriptor hashes. |
| Existing producer | [`stdio` Rust script `:925-933`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts) is the current concrete row producer: it stages raw/core/descriptor, publishes marker last, then runs `createFreshCatalogBuildVerifier`.  GIS's script has no matching fresh-row producer. | The first real GIS actor needs a build-side coordinator; Hub must never invoke Bun/JCO while opening a document. |
| All-or-none coordinator | [`executeCatalogVerificationPlan`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2319) already supplies one `publish(verified)` callback only after every selected receipt verifies.  `CatalogCompleteScript` at `:3034-3069` supplies no callback, so it verifies the fleet but publishes no trusted deployment bundle. | Reuse that callback for a new profile-sealing operation.  It consumes the retained `CatalogArtifactReceipt.rawBytes`; it does not reopen a component path or codegen the entire catalog. |
| Bundle schema/loader | [`bundle.schema.json`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json) version 2 and `BundlePackage` ([`🦀️.rs:106-135`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:106)) contain component/descriptor only.  Loader `load_selected` at `:407-579` retains those two byte arrays; `VerifiedExecutionTargetAssets` at `:303-310` exposes exactly those two. | Add and verify actor metadata/bytes in the same immutable catalog generation and asset access result. |
| Plan/lease wire | [`directory schema Rust`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1362) and [`TypeScript`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:999) have no execution payload.  `lease_fields_from_plan_v1` / `leaseFieldsFromPlanV1` project only component/descriptor. | Add one mandatory tagged actor projection to both public values, parsers, validation, projection, and full-field equality. |
| Hub protected body | [`🚀️bin.rs:2424-2575`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2424) has exactly manifest/component/descriptor; `document_execution_target_selection` recomputes auth, descriptor, catalog generation and revision before returning bytes. | Add only `BrowserActor`; reuse the exact selection/fence function and add no receipt/path/digest/query body selector. |
| Worker | [`backbone-worker.ts:528-770`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:528) owns just component/descriptor, requests only three body kinds at `:601-603`, and mints no actor. | Extend the private lease with actor record/bytes; never post it through Shell/broker/plugin state.  Fetch/hash admission is separate from later worker containment and activation. |

## Canonical New Records

Use the same tagged representation in trusted bundle JSON, Rust `BundlePackage`, Rust public directory schema, and TypeScript.  It makes absence impossible to confuse with the non-Wasm state.

```text
BrowserActor =
  { kind: "none" }
| {
    kind: "closed-browser-actor",
    schema: "semio.os.closed-browser-actor.v1",
    path: "browser/closed-actor.mjs",          // trusted bundle only
    byteLength: u64 1..64MiB,
    sha256: lowercase nonzero SHA-256,
    blake3: lowercase nonzero BLAKE3,
    sourceComponentSha256: exact package.component.sha256,
    sourceDescriptorByteSha256: exact package.descriptor.sha256,
    policySha256: lowercase nonzero SHA-256,
    importInterfaces: sorted unique bounded canonical names
  }
```

`path` exists only in `BundlePackage`, is read through the existing contained-path/bounded-reader authority, and is discarded once `Arc<[u8]>` is retained.  The public plan and lease omit it:

```text
DocumentOpenBrowserActorV1 =
  { kind: "none" }
| {
    kind: "closed-browser-actor",
    schema, sha256, blake3, byteLength, policySha256, importInterfaces
  }
```

The loader must require the following relation before retaining any actor bytes:

* `rendererTarget == Wasm` iff the selected package record is `closed-browser-actor`; React and WGPU require `none`.
* actor source component/descriptor digests exactly equal the selected package rows; SHA-256, BLAKE3, and length exactly equal the bounded read actor bytes.
* `path` is unique across the selected closure, regular/non-symlink/contained, actor bytes count against a distinct 64 MiB selected-closure actor budget, and imports are structurally valid, canonical sorted/unique and policy-admitted.
* `trusted_profile_generation` includes every actor field, including source bindings, policy digest, imports and payload digests/length.  An actor-only rotation must invalidate an existing plan.

This is deliberately an execution payload distinct from `DocumentOpenPackageV1.component*`: the raw WebAssembly component remains bound to descriptor parsing while the ESM has its own bytes and dual digests.

## Smallest Coherent Implementation Order

1. **Schema and immutable loader — primary implementation slice.**
   Change [`trusted-catalog bundle schema`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json), `Bundle*`/validation/generation/loading at [`trusted-catalog/🦀️.rs`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs), and feature-gated [`test-support`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🏗️test-support/🦀️.rs).  The latter currently writes a synthetic component for a `wasm` target and explicitly says it never executes it; give that test profile a separately synthetic, correctly digested actor solely to test catalog/body identity.  It is not evidence of JCO generation or browser execution.
2. **Public authority projection.**  Add mandatory `browserActor` tagged values to `DocumentOpenPlanV1`, `DocumentExecutionTargetLeaseFieldsV1`, their Rust validators/projection/equality and TypeScript interfaces/parsers/equality.  Add it to `VerifiedDocumentOpenSelectionV1` so plan issuance at [`Hub `issue_document_open_plan_inner`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2247) has identity without looking up bytes.  Add it to private `DocumentOpenPlanAuthorityV1`/`public_plan` ([`🚀️bin.rs:1147-1227`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1147)) so exchange/final-fence equality cannot drop it.
3. **Hub target route.**  Extend `DocumentExecutionTargetAssetV1`, router registration at [`🚀️bin.rs:6386-6388`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6386), `VerifiedExecutionTargetAssets`, and the same `document_execution_target_selection` result with actor.  For `browser-actor`, require a Wasm actor before returning a bounded `application/javascript; charset=utf-8`, `Content-Length`, `Cache-Control: no-store` body.  For all other renderer targets reject it before a body.  Preserve the current final subject/revision revalidation immediately before all four replies.
4. **Private worker admission only.**  Extend [`backbone-worker.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts) asset union/path regex, exact manifest equality, bounded stream reader and `DocumentExecutionTargetLease`.  It must SHA-256 and BLAKE3-check actor bytes against the verified manifest, preserve raw component/descriptor descriptor checks, and wipe/revoke all three on every terminal path.  Do not enable ordinary activation here: it depends on the separately unfinished dedicated-worker deadline/termination and aggregate argument/core-memory admission.
5. **Build-side producer after that schema is proven.**  Add a profile-sealing coordinator to [`📇️registry/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts), using `executeCatalogVerificationPlan(..., { publish })`.  It derives only each profile's selected Wasm package from its already-retained verified `rawBytes`, calls `buildClosedBrowserActorArtifactV1`, immediately copies/hashes the returned bytes into its owned staging row, and publishes the bundle/marker last.  The Hub consumes that sealed result later through `OS_HUB_TRUSTED_CATALOG_BUNDLE`; it does not invoke a TypeScript builder or accept a component path.

Current `buildClosedBrowserActorArtifactV1` ([`browser-bundle script :15-220`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:15)) currently exposes `componentSha256`, actor SHA-256, byte length, imports and a policy label.  Before step 5 it needs the sealed `policySha256` already being developed and actor BLAKE3 in its consumed output.  The producer, not an untrusted caller, attaches the already-verified descriptor-byte SHA-256.  This preserves the acyclic dependency: raw receipt and descriptor receipt exist before actor derivation; the final marker/bundle binds the result afterwards.

## Required First Laws

1. **Neutral schema vector** under the existing document execution-target lease fixture: valid Wasm actor; valid React/WGPU `none`; missing `browserActor`; Wasm/none and non-Wasm/actor inversions; actor SHA/BLAKE3/length/policy/source binding/import ordering mutations.  Rust and TypeScript parsers must agree.
2. **Trusted loader native law** based on `VerifiedGisMapTestProfileV1`: valid actor returns exact three retained byte owners; missing actor for its Wasm target, escaped/reused path, source component/descriptor mismatch, noncanonical imports, payload digest/length mismatch, and actor-only generation rotation deny before codec registry publication.
3. **Hub native route extension** of `execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`: exact actor content type/no-store/body success; unauthenticated/foreign scope/surface/query/actor mutation/revision/role/catalog rotation deny before actor bytes.  The existing test already supplies the exact route/fence harness.
4. **Worker process law**: one actual Hub-served closed actor is digest-checked privately; plan/manifest actor mismatch, actor body tamper, non-Wasm actor body, cancellation between actor chunks and close/revoke each leave no lease/Blob URL/bytes.  A separate later law proves worker termination of a noncooperative actor; this route law must not claim it.
5. **Producer law**: take a verified raw GIS receipt, mutate raw/descriptor/policy/import/output after respective capture points, require no browser row/marker/bundle publication; equal source receipt produces the same actor record.  This is where the current Blob child pre-dispatch mutation laws contribute, but they are not catalog publication evidence by themselves.

## Current Non-claims

There is no current trusted bundle producer for real GIS/stdio deployment: `CatalogCompleteScript` verifies fresh rows and intentionally stops before `publish`; `configured_artifact_authority` only loads the path/profile supplied through Hub startup.  The only current Hub profile builder is feature-gated test support and uses a synthetic component.  Therefore neither existing actor-builder receipts nor target component/descriptor route receipts establish catalog-derived browser-actor delivery yet.

Likewise, the actor record solves source/policy/payload identity and authenticated private byte handoff.  It does not certify the generated code as a sandbox.  Keep the distinct worker-containment frontier for non-cooperative deadline/termination and aggregate resource admission.
