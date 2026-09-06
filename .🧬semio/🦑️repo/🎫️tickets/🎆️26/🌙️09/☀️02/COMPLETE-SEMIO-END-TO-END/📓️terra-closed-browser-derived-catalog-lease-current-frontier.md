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

## 2026-09-06: Current Fixed Stdio/GIS Producer Revalidation

### Decision

The earlier coordinator gap has changed materially.  The real Hub producer is now
[`materializeTrustedStdioGisBundle`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:4808): it invokes
`produceFreshComponentV1` for exactly `semio-s-plugin-stdio` and
`semio-s-plugin-gis`, stages their captured component and descriptor bytes, then
publishes the fixed `local-stdio-gis-open-v1` generation.  The selected target is
the GIS Map editor (`wasm`); Stdio has no target.

Therefore the smallest coherent next slice is **one derived actor row for GIS and
an explicit `none` row for Stdio**, inside that producer's retained component
lease.  It is not a fresh-registry marker change, and Hub document opening must
not invoke Bun/JCO or reopen a package path.

This supersedes the older claim that no concrete GIS producer existed.  It does
not change the non-claim: no current production catalog contains an actor, no
current target route returns one, and the worker remains correctly
`renderer-unavailable`.

### Exact Mandatory Record

Use one required `browserActor` field in every catalog package, with no omitted
or nullable third state.  The current artifact-builder result at
[`browser-bundle/📜️script.ts:16`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:16)
has SHA-256, length, policy SHA-256, codegen-policy label, component SHA-256 and
canonical interfaces.  It does **not** currently produce BLAKE3.  Thus the first
catalog record must use SHA-256 only, just as the existing descriptor `BundleFile`
does; inventing a BLAKE3 field downstream would make the producer and loader
incoherent.

```text
BundleBrowserActorV1 =
  { kind: "none" }
| {
    kind: "closed-browser-actor",
    schema: "semio.os.closed-browser-actor.v1",
    codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1",
    path: "packages/gis/browser/closed-actor.mjs",  // bundle-private only
    byteLength: u64 in 1..67_108_864,
    sha256: nonzero lowercase SHA-256,
    sourceComponentSha256: exact package.component.sha256,
    sourceDescriptorByteSha256: exact package.descriptor.sha256,
    policySha256: nonzero lowercase SHA-256,
    importInterfaces: canonical strictly-sorted unique subset of the fixed
                      `pure`, `host-async`, and first-party Preview2 names
  }
```

The source component relation is the same SHA returned by
`buildClosedBrowserActorArtifactV1`; the source descriptor relation is attached
by the producer only after its descriptor snapshot has been verified.  Do not
bind `coreWasmSha256`: the catalog does not load the emitted core as an actor
input, and no existing source proves it equals the JCO embedded core closure.

`path` is catalog-private.  It must never occur in a plan, target manifest,
lease, worker message, Shell state, broker request, or a response URL.  The
public projections instead are:

```text
DocumentOpenBrowserActorV1 =
  { kind: "none" }
| {
    kind: "closed-browser-actor",
    schema, codegenPolicy, sha256,
    sourceComponentSha256, sourceDescriptorByteSha256,
    policySha256, importInterfaces
  }

DocumentExecutionTargetBrowserActorV1 =
  { kind: "none" }
| DocumentOpenBrowserActorV1 & { byteLength }
```

The plan follows the current component pattern and carries identity but no byte
length.  The receipt-free target manifest supplies the bounded length only after
the catalog has selected the actor.  Each parser must reject a `closed` record
whose source hashes differ from its package hashes, a noncanonical interface
list, zero/oversize actor length, and either target inversion: `wasm` requires
`closed-browser-actor`; `react` or `wgpu` requires `none`.

The `codegenPolicy` string must be exact, not a caller-selected label.  The
catalog loader may structurally admit only the current fixed interface vocabulary
emitted by the builder (`semio:framework/pure@1.0.0`,
`semio:framework/host-async@1.0.0`, and the exact
`browserWasiInterfaces` list at
[`🌐️wasi/🟦️.ts:8-13`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:8)).
`policySha256` is an immutable derivation/policy fence, not a browser sandbox
claim or an ambient policy input.

### Catalog and Generation Changes

Current `bundle.schema.json` makes component and descriptor the only payloads
([`:94-122`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json:94)).
`BundlePackage` and retained assets have the same two-payload shape
([`trusted-catalog/🦀️.rs:104-157`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:104),
[`303-320`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:303)).

Make the field required in JSON and Rust.  This is a greenfield simultaneous
format change: do not add an omitted-field fallback.  If its semantic bundle
format version is advanced, advance all producers/fixtures/loader checks in one
change; do not make a dual v2/v3 reader.  Retaining schema version `2` is only
coherent if every in-tree bundle is regenerated atomically and that version has
not been treated as a stable external contract.

The loader insertion point is after current descriptor decode/validation and
before provider preview at
[`trusted-catalog/🦀️.rs:464-480`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:464).
For a closed row it must:

1. resolve the actor with the existing contained-path fence, reject a reused
   component/descriptor/actor resolved path, and bounded-read it;
2. verify byte length and SHA-256 before retaining an `Arc<[u8]>`; account it
   against a separate actor closure cap, rather than silently borrowing the
   descriptor cap;
3. validate source-component/source-descriptor equality and the fixed policy
   label/interface grammar before any native provider is previewed or any codec
   is registered;
4. retain metadata in `VerifiedTrustedPackage`, attach the public identity to
   `VerifiedDocumentOpenSelectionV1`, and return the private actor bytes only
   through `VerifiedExecutionTargetAssets`.

The existing provider registration is already after the package loop, so this
ordering preserves all-or-none native-codec publication.  Its progress count is
currently `3 * packages + 1` at
[`trusted-catalog/🦀️.rs:425`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:425);
make the actor stage explicit rather than reporting a four-file load as three.

`trusted_profile_generation` presently frames only package identity, component,
descriptor, dependencies/codecs, and one open target
([`trusted-catalog/🦀️.rs:694-779`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:694)).
Frame an explicit actor tag for *every selected package*, followed for `closed`
by schema, policy label, private path, byte length in a fixed integer field,
actor SHA, both source SHA values, policy SHA, and a counted sorted interface
sequence.  Mirror the exact order in
[`trustedBootstrapProfileEncoding`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:4487).
An actor-only output or policy rotation must change the generation and stale an
issued plan.

### Fixed Producer Ownership

The materializer's current lease callback only derives a component SHA
([`📜️script.ts:4831`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:4831)).
`produceFreshComponentV1` deliberately exposes a one-use copied
`FreshComponentLeaseV1` ([`describe/📜️script.ts:49-56`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:49),
[`342-404`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:342)).
Use that exact owner:

* for `stdio`, consume only to derive the existing component digest and return
  `browserActor: { kind: "none" }`;
* for `gis`, consume once to call the closed actor builder, immediately stage
  its returned bytes at the fixed actor path with exclusive create/fsync, and
  retain only its metadata in the bundle row; and
* if codegen, staging, cancellation, descriptor/source binding, policy binding,
  or final generation fence fails, retire the actor staging file and the whole
  uncommitted generation.  Do not publish a component/descriptor-only GIS row
  after an actor derivation was selected for that profile.

The builder's private `policyCanonical` and `bytes` must remain inside this
producer handoff; only the sealed policy SHA and derived actor record are put
in the bundle.  `FreshComponentReceiptV1` stays raw/core/descriptor-only.

### Simultaneous Census

| Boundary | Exact current source | Required change |
| --- | --- | --- |
| Bundle schema and loader | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json:94-122`, `🦀️.rs:104-157, 345-580, 694-779` | Required union, bounded actor read/retain, source/policy/interface validation, generation framing, selection/assets projection. |
| Loader native fixtures | `trusted-catalog/🦀️.rs:1286-2090`; `🏗️test-support/🦀️.rs:1-156`; `🧪️fixtures/👥️two-package/🔣️.json` | Add GIS `closed` and zero-target Stdio/base `none`; test-support actor bytes are synthetic identity-only and must retain its existing no-execution disclaimer. |
| Real fixed producer | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4487-4568, 4808-4908, 4978-4992` and `🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` | One GIS builder handoff from the lease; fixed private actor file; actor fields in summary/generation, materialized bundle, rotation, fixture and source gate. |
| Shared plan/lease codecs | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:999-1320`, `🦀️.rs:1444-1749`, exported by `💻️os/🟦️.ts:558-565` | Add `browserActor` to both plan and lease, strict parse/validate/projection/full equality; update Rust DTO consumers rather than a compatibility default. |
| Neutral/public test DTOs | `💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json`; `🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json`; `directory/🔌️client/🦀️.rs:411,2305`; `🏪️store/🔄️sync/🦀️.rs:3400` | Put `none` in the current React-neutral plan fixture and `closed` in the Wasm execution-target corpus; update every explicit Rust literal. |
| Hub authoritative projection | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:381-401, 1147-1227, 2290-2345, 2417-2519` | Carry the selection actor into `DocumentOpenPlanAuthorityV1`, `public_plan`, revalidation equality and manifest fields.  The existing two selection comparisons at `:2406-2410` and `:3018-3021` become actor-safe automatically only if the selection derives `Eq` over the new actor identity. |
| Hub body route—deferred | `🚀️bin.rs:2424-2575, 6386-6388`; relay fixture `🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1` | Do **not** add `browser-actor` to body routes, relay allowlists, or the body corpus in this identity slice.  It can be added later through the same reauthenticated/final-fenced selector, with `text/javascript; charset=utf-8`, no-store and exact length, only after worker aggregate-memory/deadline containment is qualified. |
| Browser recognizer—deferred bytes | `💻️os/🧵️backbone-worker.ts:537-870, 4686-4920` | Update only parsers/projection equality to tolerate and bind actor metadata.  Do not add a fourth fetch, Blob URL, dynamic import, or `DocumentExecutionTargetLease` byte owner yet: current actor+component+descriptor aggregate admission and non-cooperative worker termination are not qualified. |
| Native type propagation | `directory/🔌️client/🦀️.rs`, `🏪️store/🔄️sync/🦀️.rs`, `📺️renderer/.../🧊️wgpu/🦀️.rs`, and `🌎️hub/💡️inference/📇️catalog/🦀️.rs` | Make their values structurally carry `none`/metadata; they must not mint, fetch, or execute browser actor bytes. |

### First Executable Laws

1. **Schema/loader neutral plus native.**  A valid fixed GIS closed row and
   Stdio `none` load; missing field, Wasm/none, non-Wasm/closed, reused/escaped
   actor path, source component/descriptor substitution, byte/SHA mismatch,
   policy-label/SHA/interface-order mutation each reject before native codec
   registration.  The existing `assets_for_current_selection` law is the right
   byte-ownership assertion; it must still make no execution claim.
2. **Generation.**  Extend
   `trusted_profile_generation_binds_zero_target_package_and_every_codec_row`
   so every closed actor field and a GIS-only actor rotation changes generation;
   changing Stdio's explicit `none` is impossible by shape.
3. **Producer.**  From the one copied GIS lease, mutate component,
   descriptor, policy/result bytes and actor stage identity after their capture
   points.  Each must leave no published generation/current pointer.  An equal
   captured source must reproduce the same row bytes and generation.
4. **Plan/lease parity.**  Rust and TypeScript agree on a React `none` vector,
   a Wasm `closed` vector, no `path`, no policy canonical bytes, and every
   actor field substitution.  `sameLeaseFieldsV1` and Rust equality must fail
   actor-only substitutions.
5. **No premature actor delivery.**  Extend the worker lease corpus with
   closed metadata but retain exactly the current three asset paths and
   `renderer-unavailable` result.  This proves the actor is not silently
   fetched/activated while containment remains unfinished.

### Current Non-claim

The browser artifact builder has source/fixture qualification, but it is only
referenced by its own fixture script today.  This audit establishes a
catalog/plan identity and producer direction, not a worker sandbox,
browser execution, GIS renderer, body delivery, or durable Map mutation path.
