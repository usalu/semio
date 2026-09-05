# Terra All-Plugin Catalog Activation Blueprint

Date: 2026-09-04  
Scope: current-tree, source-only audit of the path from a receipt-verified `stdio` D0 provider to an honestly complete installed plugin/artifact catalog. No product source, test source, configuration, generated asset, Cargo target, or runtime gate was changed or run for this report. Source presence is not runtime acceptance.

## Decisive boundary

**RED: the repository can make one statically linked provider a bounded D0 authority, but it cannot truthfully activate the installed catalog as a whole.** The immediate dependency-ordered packet is a **single static `stdio` provider plus one verified JSON viewer target**, not “all plugins.” It must retain the full `stdio` native-codec closure required by the descriptor, publish only an immutable receipt-derived bundle/profile, and expose only the one target that has passed all joins. Everything else remains inventory or unavailable with a typed reason.

The current hub is deliberately fail-closed: `linked_native_codec_bindings()` returns `Vec::new()` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393-395`. Startup supplies that empty vector to the trusted loader at lines 5248-5254 and derives D0 feature readiness only from a configured catalog with at least one target at lines 5280-5283. A bundle requiring a codec therefore cannot succeed today. The hub Cargo manifest also does not link the stdio crate (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:31-51`).

`ready` is not a D0 success claim: `hub_readiness` excludes `open_plan_ready` from `required_ready` while reporting the feature separately (`📦️bin.rs:1725-1743`). Consumer code must inspect `features.openPlan`; a catalog packet must not market a general ready response as catalog activation.

## Current chain and its proof limit

| Boundary | Current source evidence | Classification |
| --- | --- | --- |
| Isolated stdio component receipt | `@semio-tech/stdio-plugin:catalog-root` creates an empty-root build, raw/core/descriptor receipt and marker, then checks registry generation (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:448-545`). | Source-present build handoff; no hub import or runtime observation. |
| Complete registry fresh-artifact audit | `catalog-complete` strictly reads every raw/core/descriptor triplet and commit marker from an explicit fresh root (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2601-2704,2708-2745`). | Strong prerequisite for build bytes, but it produces neither a hub trusted bundle nor a native codec binding nor an app/surface activation. |
| Static codec implementations | Stdio has 26 private factory functions and IDs including `stdio.native.json.v1` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:861-922`). | Compile-time capability only. It is intentionally not authority until schema rows opt in. |
| Receipt admission | `native_codec_factory_receipts()` requires source-authorized executable rows, exact runtime/descriptor bijection, package ID, schema, extension and nonzero pack hash (`📇️registry/🦀️.rs:981-1035`). Current law asserts six codec rows but **zero** executable registrations and rejects factory promotion (`:1109-1141`). | Correct fail-closed boundary; no currently admissible `stdio` receipt vector. |
| Hub loader | The trusted loader bounds/canonicalizes the selected dependency closure; checks component SHA-256+BLAKE3, descriptor byte SHA-256, package identity, native-codec bijection and target-to-codec relation before registering codecs (`🌎️hub/🗿️artifact-authority/🗂️📇️trusted-catalog/🦀️.rs:321-477`). | Source-present loader, currently unreachable from a real native provider. |
| Browser D1 consumer | The worker refuses an absent `installedTarget`, requests a plan/receipt via the broker, and compares full package hashes, artifact and React surface before exchange (`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:481-547,1659-1662`). | Source-present D1 consumer; no catalog discovery supplies `installedTarget`, and no runtime was run. |
| Native/WGPU consumer | Native sync asks the directory source for an admission and checks schema/hash/surface before WebSocket use (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1977-2064`). The WGPU shell derives a local app/window/surface before opening (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:540-603`). | Source-present transport guard, but not package-hash-complete and not a working WGPU render path. |
| MCP consumer | Hub readiness declares `mcp_workspace: false`; MCP workspace uses a hard-coded probe surface and its artifact module documents generic probe-only creation (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1702-1743`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:350-365`; `🗿️artifact/🦀️.rs:14-18`). | RED for catalog-owned headless open/execute. |

The generated registry is useful inventory: a row carries package identity, role, dependencies, activation and raw/core/descriptor SHA-256 values (for example `🤖️generated/🔣️plugins.json:1-26`). It does not contain the hub bundle’s BLAKE3 component identity, codec factory receipt, profile closure, or verified app/window/surface selection. It must never be consumed as a ready-to-open catalog.

## Exact current REDs

### 1. No native provider can get the first bundle through startup

The immediate blocker is not a missing route: it is the empty hub provider function. The first packet must link the stdio crate, obtain `native_codec_factory_receipts()` once, instantiate and recheck every authorized receipt, and project it to `NativeCodecBinding` before `configured_artifact_authority` is invoked. It must reject receipt failure, a foreign plugin/package, duplicate factory/descriptor/`(kind,schema)` identity, zero hash, or a factory output different from its receipt.

Do not make one factory visible while carrying only one codec in the trust record. `validate_descriptor` requires the descriptor’s artifact-kind count to equal `nativeCodecs`, and the loader rejects undeclared or unconsumed bindings (`trusted-catalog/🦀️.rs:401-466,761-800`). Consequently the first stdio profile may expose **one JSON viewer target**, but the selected stdio package record and provider must carry the full explicitly authorized stdio codec closure. Until the source schemas authorize those receipts, the honest outcome remains no D0 provider.

### 2. Build receipt, trusted bundle, and native executable are three separate authorities

`catalog-root` gives an isolated raw/core/descriptor triplet and commit marker. The hub bundle model has only a component and descriptor file record; the loader verifies the component dual digest but only checks that the decoded descriptor’s core-Wasm metadata is nonzero (`trusted-catalog/🦀️.rs:349-371,761-773`). It does not read a core-module file or compare it to the descriptor’s core hash. The bundle also has no relationship to a Rust factory other than the in-process binding match.

The all-catalog packet therefore needs an immutable **activation-generation input**, generated only from fresh verified receipts:

```text
CatalogActivationGenerationV1
  generationDigest
  profiles[] -> exact dependency roots
  packages[] -> plugin/package/version, raw component {length, sha256, blake3},
                core {length, sha256}, descriptor {length, byteSha256, selfHash},
                exact dependency and artifact-kind closure
  nativeReceipts[] -> providerId, factoryId, plugin/package, kind/schema,
                      packSchemaHash, runtime-capability id
  surfaces[] -> package/artifact plus descriptor-derived appId, windowKindId,
                role, renderer target, surfaceId
```

The file form supplied to the hub must be bounded, canonical and rooted in one immutable generation directory. `core` is required because the fresh registry verifier checks it and a package descriptor names it; otherwise a full-package equality claim is incomplete. The native function pointer stays private to the provider; it must not cross a wire or appear in the catalog response.

### 3. The hub currently trusts bundle-authored surfaces rather than proving descriptor ownership

`BundleOpenTarget` requires nonempty `surfaceId`, `appId` and `windowKindId` and a matching codec (`trusted-catalog/🦀️.rs:652-665`). `validate_descriptor`, however, verifies package/dependency/artifact-kind identity only (`:761-800`); it does not prove that the target’s app, window, role, renderer and surface occur in the decoded descriptor. The loader then places those bundle values directly into `VerifiedDocumentOpenSelectionV1` (`:408-449`).

This is a material all-catalog authorization gap. Make the descriptor carry a canonical open-surface declaration (or a descriptor-owned mapping from application/window/role to surface and renderer) and validate every target against it. Do not hand-write the tuple in a bundle generator. The WGPU helper demonstrates the needed local relation—plugin/package/version, manifest app membership and exact window—but it presently derives only local strings (`Shell/…/🦀️.rs:540-565`). The activation generator must use the same descriptor source and reject zero, duplicate, unknown, conflicting or non-renderable app/window/role tuples.

### 4. Consumers do not yet share one generation-backed installed-target resolver

The browser’s `InstalledDocumentExecutionTargetV1` does carry full package hashes (`🧰️framework/🛍️products/💻️os/🟦️.ts:557-581`) and rejects a server plan that differs (`backbone-worker.ts:481-506`), which is the correct final comparison. But `PersistenceBinding` accepts that target as an optional caller-provided value; this audit found no hub catalog query or generation-verified resolver that constructs it. A marketplace/generated row is not an acceptable substitute.

Native does less: `DocumentSocketSurfaceExpectationV1` includes plugin/package/version and surface data but no component or descriptor hashes (`📇️directory/🔌️client/🦀️.rs:324-348`), and `DirectoryClient::admit_document_socket` compares that surface plus schema/hash, not a locally installed component/descriptor digest (`:807-861`). Native/WGPU must receive one local `InstalledCatalogSelectionV1` that includes and checks **both** component hashes and descriptor byte hash before calling `set_document_socket_surface`. A package/version match alone is not immutable-package equality.

MCP is intentionally outside this packet: its only hub document surface is the probe and it cannot make a catalog-owned generic plugin document or command. Do not flip `mcp_workspace` or expose catalog resources as part of server D0 readiness.

### 5. WGPU selection is guarded but activation is still knowingly unavailable

The WGPU shell correctly claims the surface before opening and closes the host if attachment fails, but every real native path calls `ProgramBridgeEntry::attach_backbone` (`🐚️Shell/…/🦀️.rs:3653-3660,3703-3711`). The underlying WGPU exchange returns the explicit retired-v12 error (`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281-282,521-526`). It cannot be credited as a rendering/mutation consumer of the first provider, much less every catalog entry.

## Smallest dependency-ordered implementation plan

### P0 — one honest stdio D0 provider

Owner: hub + stdio only. Keep the existing `catalog-root` as the sole build receipt producer. Authorize the one JSON codec in source schema with its exact `stdio.native.json.v1` receipt only when every receipt field is valid. Since the current loader’s selected package closure requires all declared artifact kinds, either (a) authorize and bind the complete stdio receipt set before launching P0, or (b) publish an actually smaller descriptor/package whose artifact-kind manifest itself contains only JSON. Do not advertise a 26-artifact `stdio` package while binding only JSON.

Build one activation generation from the receipt, include raw/core/descriptor bytes and a descriptor-derived JSON viewer surface, create `NativeOpenableCatalogProviderV1`, and make `linked_native_codec_bindings()` return only its immutable vector. On every error: no authority, no codec visible, no `openPlan`, and no plan route success. This is the only packet that may set D0 readiness nonempty.

Bounded P0 acceptance: **the hub starts with one freshly verified stdio generation and authorizes exactly one JSON viewer plan/one-use socket receipt.** It does not claim that a browser or WGPU renders JSON, an MCP command runs, or another installed package is available.

### P1 — descriptor-owned activation generation and availability taxonomy

Owner: plugin registry + descriptor schema. Add the descriptor-owned surface declaration and the activation-generation schema above. Every generated registry entry must classify itself as exactly one of:

- `openable`: full receipt, provider, codec, descriptor-derived surface and consumer support;
- `installed-non-document`: host/extension with no standalone document promise; or
- `unavailable`: bounded typed reason such as `missing-descriptor`, `missing-provider`, `unsupported-renderer`, or `missing-asset`.

No inventory row becomes `openable` from source folder discovery, a marketplace row, a Wasm filename, or a generated hash. Make `catalog-complete` a required input but retain its current receipt proof as a prerequisite rather than pretending it has made an activation generation.

### P2 — atomic all-provider publication

Owner: hub artifact authority + every linked native provider. Build the entire selected profile and all provider vectors off-lock. Then acquire the existing artifact assembly barrier only after all closure, core/raw/descriptor, factory and surface checks pass. Preflight every codec before registration, as the current loader does (`trusted-catalog/🦀️.rs:473-476`), and publish the catalog authority, `openable_catalog` and readiness as one logical generation.

Current registries are process-global/additive, while hub state is initialized once (`📦️bin.rs:5248-5291`). Therefore P2 must be **restart-only**. Do not add a live profile reload: safe replacement would require versioned codec deregistration plus a catalog/connection fence that does not exist. A failed candidate leaves the prior process generation intact; a new process either publishes the whole candidate or remains `openPlan=false`.

### P3 — consumer-specific activation, after P0/P2 only

- Browser: resolve `installedTarget` from the verified activation generation, require full package equality, await module/app/open acknowledgement, and close late work on cancellation.
- Native/WGPU: extend the native local selection to carry all package hashes, repair the retired event-backbone exchange, then prove one provider’s document attaches, mutates, disconnects and reconnects without a secret leaving the socket header path.
- MCP: separately implement a descriptor-owned headless D0 open/command ABI. It must receive an exact selection and capability from the catalog authority, not use the probe mechanism or a generic registry guess.

P3 deliberately does not block P0’s server-only result, but it blocks any user-facing “all plugins/artifacts open” acceptance.

## Neutral fixtures and hostile oracle

Add a language-neutral `catalog-activation-generation-v1` fixture family. The oracle must be an independent Bun/AJV/WebCrypto implementation; it may read the fixture but may not import Rust factory code, hub loader code, generated registry renderer, or a development cache.

Positive vectors:

1. one receipt-derived stdio JSON provider and exact viewer surface;
2. a two-package dependency closure with a non-document extension; and
3. an all-profile inventory whose non-openable entries give explicit reasons rather than open targets.

Mandatory deny vectors:

- missing/extra/duplicate provider receipt; unknown factory; foreign package; changed package ID; zero/noncanonical schema hash; factory output mismatch;
- raw/core swap; changed raw SHA-256 or BLAKE3; changed core SHA; changed descriptor byte/self hash; stale commit marker; changed file during bounded read; path escape, symlink, duplicate path and byte/count closure overflow;
- profile root absent, incomplete dependency, cycle, descriptor package/version/dependency/artifact-kind mismatch;
- target whose app/window/role/renderer/surface is absent from the descriptor, mismatches its artifact schema, is duplicate, or is not supported by its claimed consumer;
- generated/inventory row with no fresh receipt; cache-only module; descriptor-less package; browser target supplied from a different generation; native target with a matching version but changed component/descriptor hash;
- provider preflight collision or any late error: assert no codec, authority, target or readiness bit from the candidate becomes observable; and
- cancellation/deadline during build, bundle read, preflight, plan issue, plan exchange and consumer connect: no receipt/grant/port/module/session remains usable.

The hub-side law must exercise the production `TrustedCatalogLoader`, provider and startup/readiness path. A copied map helper, compiled-but-unlaunched provider, or a fixture that seeds `HubState.openable_catalog` directly is insufficient.

## Registered gates to retain and the missing final gate

Existing registrations are useful but non-substitutable:

- `bun nx run @semio-tech/stdio-plugin:catalog-root -- --build-root <absolute-empty-root>` — fresh stdio component receipt;
- `bun nx run @semio-tech/plugin-registry:check-generated` — generated registry/launch freshness only;
- `bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root <absolute-fresh-root>` — all-row raw/core/descriptor receipt audit;
- `bun nx run os-hub:open-plan-check --skip-nx-cache` — neutral D0 oracle plus exact Rust laws (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2750-2792`);
- `bun nx run os-hub:browser-document-open-check --skip-nx-cache` and `bun nx run os-hub:native-document-open-check --skip-nx-cache` — consumer-specific D1 gates, not catalog activation (`📜️script.ts:2828-2884`).

Add a permanent `os-hub:catalog-activation-check`, registered in the hub `📜️script.ts`, its Nx project and `.vscode/launch.json` alongside the document-open gates. It must:

1. require exact-one FQN selection for every named Rust law before exact-running it;
2. run the neutral generation oracle and no-generated-bypass checks;
3. build an isolated fresh P0 input, start a real hub with the bundle/profile and observe `openPlan=false → true` only after full provider/loader success;
4. prove the first issue/exchange/socket operation uses the exact provider selection; and
5. run the current all-feature hub qualification separately, reporting any unrelated feature-graph failure rather than silently changing to a subset.

Only after P1–P3 can a final `all-plugin-catalog-activation-check` enumerate every registry item and validate its declared availability category across browser, native/WGPU and MCP. It must not encode a missing provider as success or skip a missing surface.

## Explicit nonclaims

This packet must not claim a signed marketplace, live catalog refresh, generic MCP plugin commands, WGPU rendering, browser module availability, mutation/collaboration success, or that a Rust codec pointer was compiled from the exact component bytes without a later build-attestation design. It also must not claim that the existing registry’s raw/core/descriptor checks make every descriptor-backed row openable.

The handoff is therefore: **implement P0 as the sole server D0 readiness packet, then P1/P2 as catalog-wide authority, and only then separately accept browser, WGPU and MCP activation.**

