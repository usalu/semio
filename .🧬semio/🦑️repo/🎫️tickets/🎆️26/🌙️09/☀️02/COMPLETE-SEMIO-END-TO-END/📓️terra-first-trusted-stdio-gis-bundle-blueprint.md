# First Server-Owned Trusted Stdio + GIS Bundle

## Verdict

**RED — the hub can validate a hand-authored fixture today, but no fresh, server-owned two-package artifact exists that can make a real GIS document-open target ready.** The smallest honest packet is a restart-scoped bootstrap which emits and verifies an immutable `stdio+gis` bundle, loads its complete native codec closure, and exposes exactly one `s.gis.gismap` viewer selection. It stops at hub plan issuance/readiness. Browser and native execution-target consumption remain separately RED.

This is a current-source audit only. No build, native hub run, browser run, or signature verification was performed.

## Current Evidence

| Boundary | Current evidence | Classification |
| --- | --- | --- |
| Linked native closure | [`native-openable-provider`](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24) links `stdio` and `gis`; GIS produces exactly Map and Terrain receipts. | Source-backed, not a fresh bundle. |
| Receipt identity | GIS binds plugin/package/version, schema, extension and protocol SHA-256 before creating a codec at [`native-codecs`](../../../../../../✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:31). Stdio's receipt has no package-version field at [`registry`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:852). | GIS exact; stdio version substitution RED. |
| Bundle validation | Loader reads bounded component and descriptor bytes, verifies component SHA-256+BLAKE3 and descriptor SHA-256, validates every linked factory and registers only after full preflight at [`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:348). | Strong generic primitive; no real bundle. |
| Package/role/target binding | `validate_descriptor` requires package id, plugin/version, role and component SHA-256 equality at [`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:819); target validation requires an isolated descriptor app, exact dialect/window/role and `wasm` renderer at [lines 538–560](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:538). | Source-backed. |
| Current GIS descriptor | The committed packed descriptor is 51,116 bytes, below the loader's 4 MiB descriptor bound; its rendered owner JSON is 214,795 bytes and is not a load input. The current rendered top-level descriptor projection has no `packageId` and its top-level manifest has no artifact kinds. | Cannot enter `validate_descriptor`; do not use owner output as trust input. |
| Descriptor source topology | GIS declares both artifact definitions at [`plugin()`](../../../../../../✏️s/🔌️plugins/🌍️gis/🦀️.rs:26), while `gisterrain` is deliberately a composed child, not a discoverable `ArtifactKindSpec` ([lines 48–53](../../../../../../✏️s/🔌️plugins/🌍️gis/🦀️.rs:48)). The builder starts a manifest with an empty `artifact_kinds` array ([`Plugin::new`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25011)); artifact definitions are not copied into that array by `try_build` ([lines 644–727](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:644)). | Real descriptor/codec cardinality mismatch. |
| Fresh producer | Stdio has a controlled isolated-root producer at [`catalog-root`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:548), but it currently writes its owner descriptor and runs global registry generation ([lines 616–625](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:616)). GIS `describe` writes directly to its owner root ([`script.ts:17–23`](../../../../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:17)). | No server-safe two-package producer. |
| Hub configuration | The hub only accepts the bundle/profile as the paired process environment variables in [`configured_artifact_authority`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380). Ordinary `os-hub:dev` supplies neither ([`launch.json`](../../../../../../.vscode/launch.json:5097)). | No zero-touch server bootstrap. |
| Readiness | Startup constructs the linked provider, loads the optional catalog, and enables `openPlan` only when it has an open target ([`bin.rs`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5319)). The sole runtime-shaped law builds `abc` plus a synthetic stdio descriptor and injects it into test state ([`bin.rs`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5576)). | Fixture-only, stdio-only. |
| Client execution | Browser module loading takes a plugin id/module URL rather than a verified plan. Native WGPU scans a local module directory and can skip invalid/missing artifacts; `attach_backbone` returns a retired error ([`ProgramBridge`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281), [723](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:723)). | RED; not in this P0. |

## The First Real Target

The selected target is exactly the GIS map viewer:

```text
package:  gis / semio:gis / workspace package version
artifact: s.gis.gismap / gis.map / SHA-256(protocol)
dialect:  s.gis.gismap / 1 / *
surface:  s.gis.gismap@1/*#viewer
role:     viewer
renderer: wasm
grant:    read + observe, never write
```

GIS must nevertheless contribute **both** private codec rows (`gismap`, `gisterrain`), because [`native_codec_factory_receipts`](../../../../../../✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs:95) is intentionally a closed two-receipt factory closure. Stdio must contribute its complete 26-row closure. Thus the profile has two packages, 28 registered codec rows, and one public open target; it is not a two-codec catalog.

`gisterrain` must remain absent from the public discoverability/open-target projection. It is an owned child codec, not an independently openable document. The existing loader equality at [`validate_descriptor`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:833) incorrectly equates discoverable manifest kinds with the complete native codec closure. The correct invariant is:

```text
descriptor manifest artifactKinds ⊆ package nativeCodecs
every public openTarget ∈ descriptor apps ∩ package nativeCodecs
every nativeCodec ∈ exact statically-linked package receipt closure
```

The descriptor must expose the one real `gismap` `ArtifactKindSpec`, sourced from [`gismap::artifact_kind`](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:176), without inventing a `gisterrain` discovery kind. The loader must no longer require equality of the two cardinalities. This preserves the actual child topology rather than making a package record tell a false UI story.

## P0 Canonical Sources and Ordered Changes

### 1. Make fresh descriptor output represent its real authority

1. Extend the GIS plugin assembly at [`✏️s/🔌️plugins/🌍️gis/🦀️.rs`](../../../../../../✏️s/🔌️plugins/🌍️gis/🦀️.rs:26) to add the existing `gismap::artifact_kind()` as the one discoverability contribution. Do not add a terrain `ArtifactKindSpec`.
2. Keep `.artifact(gismap)` and `.artifact(gisterrain)` as the full runtime definition authority. Require a fresh emitted `PackageDescriptor` to have exact `packageId: "semio:gis"`, `role: plugin`, isolated execution, correct Cargo version and component SHA-256, one discoverable map kind, and the actual map viewer app/window/dialect.
3. Change [`validate_descriptor`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:819) from cardinality equality to the subset invariant above. Retain duplicate, identity and schema mismatch rejection. Add an explicit hostile rule that a terrain *open target* is denied because no descriptor surface authorizes it.
4. Add `package_version` to [`NativeCodecFactoryReceipt`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:852), derive it from the compiled package contract, and require it in [`preview_stdio_bindings`](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:48). GIS already makes that comparison at [lines 52–74](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:52).
5. Rename/re-scope the stale `NATIVE_OPENABLE_CATALOG_PROVIDER_V1_ID` and receipt-count constants ([lines 8–11](../../../../../../🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:8)) so their identity represents the linked stdio+GIS provider set, rather than a stdio-only subset.

### 2. Define a closed, server-owned bootstrap profile

Add a new **schema version 2** trusted bundle record in [`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:55), not an optional legacy field:

```text
TrustedCatalogBootstrapProfileV1
  profileId: "local-stdio-gis-open-v1"
  selectedClosure: [
    { pluginId:"stdio", packageId:"semio:stdio", version },
    { pluginId:"gis",   packageId:"semio:gis",   version }
  ]             // canonical plugin-id order
  selectedClosureSha256: hex SHA-256 of canonical complete closure projection
  openTarget: full GIS-map viewer projection above
```

`selectedClosure` must equal the selected roots plus their resolved dependency closure, in canonical order, and `bundle.packages` must equal that closure for this bootstrap profile. A third unused record, a missing stdio package, a profile with an extra root, or a dependency that changes the closure is rejected. This is the bounded meaning of “stdio+gis”, and it makes the mixed-bundle hostile meaningful.

Use one catalog generation for all selected package identities and bytes, not only `open_targets`. Current [`document_open_catalog_generation`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:606) hashes only target rows; a changed zero-target stdio component/descriptor can leave the generation unchanged. Replace it with a canonical profile-generation encoding containing, in order, every selected package identity, role, component SHA-256, component BLAKE3, descriptor SHA-256, dependency identities, all codec rows, and the one target plus its full parent dialect/grant. The document-open plan’s `catalog.generationId` must bind this full profile generation; its current target fields are retained, not substituted.

There is no bundle signature or key-id field today. The P0 is therefore **local-machine server trust**, established by the server creating and retaining its own root with restrictive permissions—not a signed downloadable marketplace artifact. A signed/remote bundle is deliberately out of scope; it needs a separately designed key-id/signature verification boundary before byte reads.

### 3. Build, verify, then atomically publish only the two rows

Create a single hub-owned script command in [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:586), registered through its existing `project.json` router, for example `trusted-stdio-gis-bootstrap`.

It owns an empty private run root and performs this order:

1. Create non-symlink private `staging/<nonce>` and two separate package work/target roots. Reject a caller-supplied build root and reject the repository `target`/dev cache. Reuse the good isolated-root and cancellation discipline of stdio [`catalog-root`](../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:533), but extract the generic component/core/descriptor emission procedure from [`describe`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:52), rather than importing a package script that mutates owner files.
2. Build the stdio and GIS WASI components separately; derive each output filename with `pluginWasmArtifactPath`, extract the core with JCO, and emit the descriptor into that package’s staging directory. The procedure must never call GIS `describe` or stdio `catalog-root` as they currently write owner descriptors/global registry state.
3. Independently parse the descriptor, inspect the component/core with JCO/WIT, compute component SHA-256+BLAKE3 and descriptor SHA-256, and compare each against the staged bundle row. Verify the static provider’s 26 stdio and 2 GIS receipt closures against those descriptor records before publication.
4. Write the v2 bundle JSON only after both packages have produced exact receipts. Do **not** call the current `TrustedCatalogLoader::load` from the materializer: it registers codecs in the process-global assembly at the end of a successful load. The materializer remains side-effect free beyond its private files; the candidate hub process is the one and only process that calls the loader against this generation with the linked provider and its startup deadline.
5. Atomically rename the complete staged root to `profiles/local-stdio-gis-open-v1/<generation>/`. Start the candidate against that immutable path, and update the server-owned `current` pointer/metadata only after the candidate has verified bytes/factories and reached readiness. Cancellation, a deadline, or any failed byte/factory check removes staging and leaves `current` untouched.
6. Spawn the hub with bundle/profile paths obtained only from this materializer, not client launch input. The ordinary launch seed changes first, then generated [`launch.json`](../../../../../../.vscode/launch.json:5097) is regenerated. Existing secure negative smoke must continue to delete trusted variables ([`script.ts`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:618)).

The hub continues to consume a path/profile pair at [`configured_artifact_authority`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380), but the dev/bootstrap command is the sole process which supplies them. A direct arbitrary environment override must not become a production client configuration surface.

### 4. Restart-only rotation and readiness

The current loader registers codecs in a process-global assembly after verification ([`trusted-catalog`](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:497)); it has no unregister/live-reload protocol. P0 rotation is therefore restart-only:

1. Materialize and validate a new immutable generation offline.
2. Start a candidate hub process with that generation. It becomes ready only after catalog load succeeds and `open_target_count() == 1`; that is the existing readiness boundary at [`bin.rs`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5354).
3. Retire the old process only after the candidate exposes `/readyz`; otherwise retain the old current generation. Plans/receipts from the prior profile-generation are rejected after reconnect, then reissued by the new hub.

No live codec replacement, client-supplied generation, catalogue row mutation, or profile fallback is part of P0.

## Schema-First Neutral Corpus

Add one corpus under the hub trusted-catalog package, with JSON Schema, a Bun/AJV + Buffer/DataView framing oracle, Node SHA-256, the first-party BLAKE3 implementation, and the Rust consumer. It must frame exact raw bytes and canonical records; generated descriptor rows are inputs, never trusted fixtures.

| Case | Expected result |
| --- | --- |
| Fresh stdio 26 + GIS Map/Terrain 2 closure, only GIS Map viewer target | accepted, 28 codecs, one target, generation exact |
| Stale generation after changing only stdio component/descriptor bytes | denied/recomputed generation differs |
| SHA-256 or BLAKE3 component substitution | denied before provider/factory selection |
| Descriptor SHA-256 substitution, package-id/version/role mismatch | denied before target resolution |
| Stdio receipt version mismatch | denied by new stdio receipt binding |
| GIS Map/Terrain missing, duplicate or factory/schema/pack-hash mismatch | denied; neither codec is registered |
| Missing GIS viewer surface/window/dialect/isolated execution or renderer != `wasm` | denied; no fallback target |
| Terrain placed in `openTargets` | denied as undiscoverable/no descriptor app |
| Extra package/root/dependency or profile closure differing from exactly stdio+GIS | denied as mixed bundle |
| `..`, symlink/path collision, over-limit component/descriptor, duplicate file path | denied before retention |
| Cancellation at build, copy, hash, descriptor, provider, and pre-publication verification stages | staging removed, previous current generation unchanged |
| Restart with old current and failed new staging | old process/catalog remains usable; no partial generation |

The neutral oracle must use the first-party BLAKE3 implementation for BLAKE3 framing; WebCrypto supplies SHA-256 only.

## Required Registered Proofs

1. **Source/neutral** — `bun nx run os-hub:trusted-stdio-gis-bootstrap-check`: exact corpus and exact-one selector. It builds no ambient descriptor and proves profile framing, fresh output receipt handoff, hashes, closure, cancellation cleanup and rotation metadata.
2. **Native hub** — a new exact `--bin os-hub` law beside [`native_openable_stdio_provider_is_the_only_atomic_readiness_transition`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5665). It must materialize actual fresh stdio+GIS bytes, start through the real configured startup path, assert `/readyz` 200 with open-plan enabled, issue an authenticated GIS-map-viewer plan, and assert package identities, both component hashes, descriptor hash, parent dialect, surface, read-only grant and full-profile generation. It must also prove all 28 codecs are registered atomically or none are.
3. **Process** — a `launch.json` seed-derived local bootstrap target starts the server-owned materializer and hub, creates/uses a seeded GIS descriptor, waits for `/readyz`, issues the one plan, then restarts on a newly materialized generation. The same target proves stale old-generation plan denial and failed replacement leaves the former `current` generation bootable. It must not invoke browser rendering, WGPU, external networking, Docker or client-selected paths.
4. **Existing D1 suite** — retain the current exact parent-dialect issuer/exchange/socket tests. Extend their neutral plan fixture only after the full-profile generation is authoritative; do not treat plan parsing as component installation proof.

## Explicit Nonclaims / Next Handoff

- No detached or browser/native client is yet allowed to fetch/execute the plan component. Browser `loadPluginModule(pluginId, moduleUrl)` and native directory scanning do not enforce the plan’s component SHA-256/BLAKE3, descriptor SHA-256, parent dialect, surface, grant or generation.
- No WGPU rendering is established; `attach_backbone` is currently an explicit retired error.
- No signed remote bundle, package marketplace, all-plugin catalog, Flow member open, GIS inference approval, document mutation, or client hot rotation is established.

After this P0, hand off to the existing execution-target lease packet: one immutable plan-derived lease with those exact identity fields and cancellation/generation invalidation for both browser and native. That is the first point at which the single server-selected GIS map viewer may become a real client execution target.
