# GIS Execution-Target Lease Blueprint

## Verdict

**RED — a D1 `DocumentOpenPlanV1` can select the GIS map target, but no client has verified bytes from which it may mint an execution target.** Browser and native each currently trust a local URL/path or a caller-supplied `InstalledDocumentExecutionTargetV1`; neither binds byte acquisition, both component digests, the raw descriptor digest, plan generation, scope, grant, and lifecycle into one immutable local owner.

The first trusted `stdio+gis` bundle/profile remains a prerequisite. This packet begins only once that server-owned profile has a real GIS Map viewer selection. It establishes **verified installation plus plan/socket admission**. It deliberately does **not** claim a rendered WGPU map: native `attach_backbone` is explicitly retired and native WGPU still scans arbitrary directories.

This is a current-source, no-build audit. “Native” below means the directory/client and local verified installation boundary, not WGPU rendering.

## Current Source Trace

| Boundary | Current source evidence | Result |
| --- | --- | --- |
| Trusted server bytes | `VerifiedTrustedPackage` retains exact component and descriptor bytes after bounded SHA-256/BLAKE3 verification ([trusted-catalog](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:165), [370-490](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:370)). `VerifiedDocumentOpenSelectionV1` retains package/artifact/dialect/surface/grant ([258-266](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:258)). | Strong in-process source, but no protected component/descriptor route exists. |
| Plan issuance | `DocumentOpenPlanAuthorityV1::public_plan` projects all selected package hashes, artifact, parent dialect, surface, grant, checkpoint and revalidation ([hub bin](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:994), [1045-1071](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1045)). The strict shared TS parser has the same fields ([directory schema](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:430), [529-585](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:529)). | Source-backed D1 projection; it is a short-lived selection receipt, not an installed component. |
| Browser D1 comparison | `documentOpenPlanAuthority` compares the complete package digests, artifact, parent dialect and surface to an `installedTarget` ([backbone worker](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:482)). It then returns only schema, pack hash, parent dialect and surface ([507-510](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:507)). | Good comparison function, but its input is not a verified installation and it discards the lease identity after exchange. It also hard-rejects every non-React target at line 504. |
| Browser module loading | `fetchDescriptorManifest` derives a sibling `🔣️.json` from a caller URL and only checks JSON owner/apps ([kernel](../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:106)). `ActivationRegistry` stores `{pluginId,moduleUrl,caps}` and `activate` passes that raw URL to `ShardClient` ([1825-1948](../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1825)). `PluginRuntime.loadPluginModule` follows the same raw-url path ([PluginRuntime](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1024)). | No component fetch/hash, raw descriptor-byte SHA-256, catalog generation, scope or grant enforcement. No browser-runtime BLAKE3 implementation exists: the only TypeScript `blake3Hex` is in a dev package script. |
| Current browser target type | `InstalledDocumentExecutionTargetV1` contains package/artifact/dialect/surface only and is optional in a hub binding ([OS](../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:557)). `documentRuntimeKeyV1` does correctly distinguish `(spaceId,documentId)` ([564-578](../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:564)). | It is a forgeable data shape, not a scoped lease; it omits generation, scope, descriptor digest, grant, revalidation and verified byte ownership. |
| Native D1 comparison | `DirectoryClient::admit_document_socket` parses/validates the plan and moves all plan fields into `DocumentSocketAuthorityV1` ([client](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:756)). Its expectation and `matches_surface` omit component SHA-256/BLAKE3, descriptor SHA-256, catalog generation, parent dialect and grant ([242-305](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:242)). | The returned authority is richer than its local admission predicate. No installed-byte comparison exists. |
| Native reconnect | `ArtifactActor` recomputes only codec schema/pack hash and surface before reconnect/Hello ([store sync](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1998), [2040-2082](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2040)). | No immutable local selection survives/revalidates reconnect. |
| Native WGPU | `load_wasm_plugins` discovers the first `.wasm` below caller-selected directories and permits best-effort/empty manifests ([ProgramBridge](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:723)); `read_descriptor_manifest` is best effort and falls back to an empty manifest ([779-807](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:779). `attach_backbone` returns a retired error ([274-287](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:274)). | **RED.** Do not route a trusted lease through this scanner or represent verified installation as a rendered map. |

The existing hub routes are only `/open-plan` and `/socket-grants` ([hub bin](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5168)); no endpoint supplies selected trusted bytes. The catalog loader’s source limits are 64 MiB per component and 4 MiB per descriptor ([trusted-catalog](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:22)). They must become the client protocol’s exact maxima; do not create browser/native variants.

The current browser D1 fixture is intentionally synthetic and React-only: its schema pins `rendererTarget: react` ([fixture schema](../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧬️browser-document-open-v1.schema.json:132)). It is not evidence that the GIS `wasm` trusted target can load or render.

## P1 Boundary

Define one public *receipt-free fields value* and one private, non-serializable owner in the shared directory contract. Replace, rather than alias, `InstalledDocumentExecutionTargetV1` for hub documents.

```text
DocumentExecutionTargetLeaseFieldsV1 {
  schema: "semio.os.document-execution-target-lease/v1"
  version: 1
  scope: DocumentScope                      // space + document, never only documentId
  hubOrigin: canonical credential origin    // local transport origin, never plan input
  descriptorDigestV1
  catalog: { generationId }
  package: { pluginId, packageId, version,
             componentSha256, componentBlake3, descriptorByteSha256 }
  artifact: { kind, schema, packSchemaHash }
  parentDialect: { artifactKind, standard, subset }
  surface: { surfaceId, appId, windowKindId, role, rendererTarget }
  grant: { read:true, write:boolean, observe:true }
  checkpoint?: exact existing checkpoint
  revalidation: exact existing revalidation
  component: { byteLength, sha256, blake3 }
  descriptor: { byteLength, sha256 }
}
```

`component` and `descriptor` duplicate the plan hashes deliberately: they bind byte lengths and prevent a server byte response from silently using a different object. Require `component.sha256 == package.componentSha256`, `component.blake3 == package.componentBlake3`, and `descriptor.sha256 == package.descriptorByteSha256`. The server creates `AssetManifestV1` from its exact `VerifiedDocumentOpenSelectionV1` plus `VerifiedTrustedPackage`; the client parses it strictly and requires equality to the receipt-free plan projection before reading bytes.

`DocumentExecutionTargetLeaseV1` itself is a platform-private owner:

- **TypeScript:** private constructor/brand, fields frozen, its byte buffers and Blob/module URL unobservable outside the installer. A public `LeaseView` is a copy of `FieldsV1`, never a constructor.
- **Rust:** private fields with `pub(crate)` construction from the verified installer only; no `FromValue`, no `Clone` of byte owners or transport credential.
- Its identity key is `documentRuntimeKeyV1({kind:"hub",spaceId,documentId})` plus exact catalog generation and all fields. A same `documentId` in another space cannot share a lease.
- A plan receipt, socket grant, session token, raw component URL/path and app-selected descriptor are never fields or constructors. `hubOrigin` comes from the credential-owning browser/native transport and must equal the plan/socket authority origin; it is not supplied by hub JSON.

Use one pure Rust/TypeScript `leaseFieldsFromPlan` and `sameLeaseFields` relation generated from a neutral corpus. It must compare every field above, including all three hashes, dialect, all surface fields, both scope IDs, every grant bit, checkpoint and each revalidation generation. No browser/native hand-written subsets.

## Server-Owned Asset Read

Add a small protected asset-port alongside `DocumentOpenPlanAuthorityV1`, not a client-built path into `VerifiedTrustedCatalog.packages()`.

1. Add `DocumentExecutionTargetAssetIntentV1 { schema, version, requestedSurfaceId }`, bounded and strict. It expresses a preference only; it grants nothing.
2. Add three protected D1 routes under the exact document scope:

   ```text
   POST /spaces/{space}/documents/{document}/execution-target/manifest
   GET  /spaces/{space}/documents/{document}/execution-target/component
   GET  /spaces/{space}/documents/{document}/execution-target/descriptor
   ```

   Every route re-authenticates the session/share binding, loads the current descriptor, resolves the role-appropriate target with `VerifiedTrustedCatalog::resolve_document_open`, and obtains bytes through a new **private exact-selection accessor**. It must not expose `packages()` as a package-id lookup or accept package/hash/path from the client.
3. The manifest contains `FieldsV1`; component and descriptor bodies are raw bytes, bounded by the existing 64 MiB/4 MiB limits. On every body request the server recomputes the same selected `FieldsV1`; it denies if the selection/catalog/membership changed. The client compares the manifest with its parsed plan, then verifies the raw body against the manifest/plan hashes. No asset receipt is needed: the existing one-use plan receipt remains solely for socket exchange.
4. Keep the plan exchange as the final server fence. A plan that became stale while bytes were fetched is rejected by the existing ledger/current-authority check; the client closes the unbound lease. This avoids extending a one-use socket receipt into a multi-read download capability.

The required trusted-catalog API is a narrowly-scoped internal `assets_for_selection(&VerifiedDocumentOpenSelectionV1) -> VerifiedExecutionTargetAssets`, which returns bytes only when package, version, all package hashes, artifact, dialect, surface and grant agree. The existing public `VerifiedTrustedPackage::component_bytes`/`descriptor_bytes` accessors ([202-210](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:202)) are useful implementation storage but are not an HTTP selection policy.

## Browser Implementation Packet

### 1. Verify before publication

Add a source-owned `installDocumentExecutionTargetLease` under the OS directory/client boundary. It receives a parsed plan, an explicit `AbortSignal`, operation deadline and broker port; it does not take a URL or `InstalledDocumentExecutionTargetV1`.

1. Parse/validate the plan using existing `parseDocumentOpenPlanV1`; check cancellation and deadline.
2. Fetch the protected manifest and require exact `FieldsV1 == receipt-free(plan)` before any component is registered.
3. Read component and descriptor streams with a 64 KiB progress unit, byte-length preflight and running cap. Abort/cancel the reader on deadline, excess, non-OK, malformed stream or identity mismatch. Emit bounded progress `{stage:"manifest"|"component"|"descriptor"|"verify", completedBytes,totalBytes}` only; never emit bytes, paths, receipt or full hashes.
4. Compute SHA-256 and BLAKE3 over the exact component bytes, and SHA-256 over exact descriptor bytes. Web Crypto is an appropriate independent SHA-256 implementation, but it has **no BLAKE3**. Move the existing first-party BLAKE3 implementation out of the dev-only `🧑‍💻dev/.../📜️script.ts` into a shared browser-safe runtime module, then use it here and keep the script consumer as a normal import. Do not import a script package into production.
5. Strict-decode the raw descriptor pack, require canonical re-encoding equality and require package/plugin/version/component digest, descriptor digest, selected app/window/role/renderer, artifact, schema/pack hash and parent dialect equality. `decodePackValue` alone ([OS](../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:1148)) is not a typed PackageDescriptor admission boundary; add a shared strict descriptor parser rather than trusting a sibling JSON manifest.
6. Only then mint the private lease and register it in a generation-scoped registry. No failed/cancelled installation may call `ActivationRegistry.registerManifest`, create a Blob URL, retain a component buffer, or issue socket exchange.

### 2. Eliminate raw-url activation for this path

`loadPluginModule(pluginId,moduleUrl)` and `fetchDescriptorManifest` remain unsuitable for a lease. Add a separate internal `loadVerifiedPluginModule(lease, verifiedComponent, signal)` path which passes an opaque verified module source to the activation layer. The activation layer may internally create/revoke a Blob URL only after byte verification; neither ShellHost nor a plugin receives it, and raw `loadPluginModule` must not accept a D1 lease.

The GIS Map plan’s trusted target is `rendererTarget: wasm`. Remove the hard React-only assertion in `documentOpenPlanAuthority` only when the caller owns a verified lease and dispatches by the lease renderer target. Do **not** pretend the generic React `backbone-worker` can render it. The P1 browser completion point is:

```text
verified GIS component + verified descriptor + immutable read-only lease
  -> exact plan/socket equality and credential-free socket admission
  -> explicit “renderer unavailable”/unmounted execution state
```

It is not `openArtifact` rendering a GIS map. The existing WGPU bridge also uses raw `loadPluginModule` ([plugin bridge](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:211)); it must be converted later to consume the same lease, never an ad-hoc module URL.

### 3. Browser lifecycle and local capability gate

- Lease installation, plan exchange and WebSocket admission share one child `AbortController`; cancellation before/after each await wipes buffers and terminates without publication. A plan receipt is exchanged only after assets verify.
- The registry invalidates the owner on local generation replacement, plan expiry, server stale/denied response, socket close that requires a different reissued target, session/membership generation change, descriptor/checkpoint change, or cancellation. It closes the actor and revokes the private Blob URL before a new plan is requested. No silent catalog hot swap.
- A reconnect may reuse only the same live `FieldsV1`; it repeats the plan equality fence before a new exchange. A different generation or any identity field closes the old lease and returns `stale`, never downgrades an editor or upgrades a viewer.
- For the Map viewer target, `grant.write === false`. The lease must be required by the browser command/outbox entry point and reject mutation/action publication locally before any worker frame. Server authorization remains independent. `grant.observe` similarly gates presence/subscription. Do not infer capability from a descriptor or an app role.

## Native Implementation Packet

1. Add the same shared fields schema plus a native private `VerifiedDocumentExecutionTargetLease`. Extend `DirectoryClient` with an authenticated, bounded asset-port method using the existing credential/origin and `OperationContext`; it owns response buffers and uses the existing Rust `Sha256`/`blake3` incremental 64 KiB checkpoint style ([trusted catalog](../../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:907)).
2. Replace `DocumentSocketExpectationV1`/`DocumentSocketSurfaceExpectationV1` as the **D1 local comparison input** with a single lease reference. At pre-plan, post-plan, post-exchange and pre-Hello, compare full lease fields rather than only schema/pack hash/surface. `DocumentSocketAuthorityV1` may continue to be the receipt-free wire result, but must expose/compare the full immutable fields relation.
3. Store the lease in `ArtifactActor`, not only `hub_surface`/`document_socket_surface`; its reconnect closure requires the current owner and invalidates it on mismatch. The local read-only gate rejects outbox commands before `flush_outbox` and does not schedule write retries.
4. Add a native `VerifiedProgramSource` owned by the lease for a future renderer. It is intentionally not fed to `load_wasm_plugins` and does not call `ProgramBridge::attach_backbone`. This proves native installation and D1 admission only. A later WGPU packet may accept `VerifiedProgramSource` after it replaces directory scanning and repairs the event-driven backbone contract.

## Accessibility and Redaction

Use the existing ShellHost bootstrap status pattern: it already accepts an explicit `"en" | "de"` locale and renders `role="status"` / `role="alert"` ([host bootstrap](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:81)). Add an execution-target status discriminant and pass the host’s explicit UI locale—no ambient/default language and no persisted document language.

| Code | EN | DE |
| --- | --- | --- |
| `verifying` | “Verifying document component…” | “Dokumentkomponente wird überprüft…” |
| `integrity-failed` | “The document component could not be verified. Reopen the document.” | “Die Dokumentkomponente konnte nicht verifiziert werden. Öffnen Sie das Dokument erneut.” |
| `stale` | “The document target changed. Reopen the document.” | “Das Dokumentziel wurde geändert. Öffnen Sie das Dokument erneut.” |
| `cancelled` | “Opening the document was cancelled.” | “Das Öffnen des Dokuments wurde abgebrochen.” |
| `renderer-unavailable` | “The verified document component is ready, but this renderer is unavailable.” | “Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.” |

These codes are the complete UI payload. Logs may retain an internal reason, but no UI/telemetry label includes an origin, URL/path, receipt/grant, user identifier, or digest.

## Neutral Corpus and Registered Proof

Create `document-execution-target-lease-v1` beside the existing directory fixtures with JSON Schema, a first-party canonical frame encoder, AJV, Buffer/DataView framing, Node/Web Crypto SHA-256, the shared first-party BLAKE3 and Rust parsing. Pin exact bytes and byte lengths, not just parsed objects.

Required positives and hostile rows:

1. GIS Map viewer: read/observe true, write false, exact full fields and component/descriptor bytes; one verified lease is minted.
2. One-field substitutions for both scope IDs, origin, descriptor digest, catalog generation, package identity/version, three package digests, artifact/schema/pack hash, all dialect fields, all surface fields, every grant bit, checkpoint and revalidation fields; all deny before registration or socket exchange.
3. Component SHA-256-only and BLAKE3-only substitution; descriptor SHA substitution; body length mismatch, `max+1`, truncated/extra stream, malformed/noncanonical descriptor, app/role/surface/dialect mismatch, and raw URL/sibling JSON substitution. Each leaves zero installed target/Blob URL/native program source.
4. Missing/changed surface, mixed server bundle, plan generation turnover, descriptor checkpoint change, session/membership revocation, wrong space with same document ID, plan expiry, cancellation/deadline at manifest/read/hash/decode/pre-publication/exchange/Hello, and reconnect yielding a changed selection. Each closes prior ownership and creates no mutation.
5. Viewer mutation/action/presence-write attempts: local rejection and zero outbound command frames; server-side write denial remains a separate assertion.
6. EN/DE status and assertive error semantics, with strings free of receipts/URLs/digests.

Register exact-one, dependency-ordered targets through the owning `📜️script.ts` and launch seed (then generate launch metadata):

| Target | Proof |
| --- | --- |
| `os-hub:execution-target-lease-check` | Neutral schema/oracle, hub selection-only asset route, frozen trusted GIS fixture, and source one-of selectors. |
| `os-hub:execution-target-lease-native-check` | Exact Rust DirectoryClient asset verification, four equality fences, cancellation/close, read-only outbox rejection and reconnect invalidation. It must not start WGPU. |
| `os-hub:execution-target-lease-browser-check` | Browser broker fixture proves no raw `loadPluginModule`/`fetchDescriptorManifest` call, all bytes checked before registry publication, plan exchange after verification, localized status, and no command when viewer-only. |
| Launch-seed local trusted GIS open | Process test starts the server-owned `stdio+gis` profile, gets a real viewer plan, verifies assets through the protected route, exchanges once, then restarts to a new generation. It must prove old lease invalidation and keep WGPU rendering explicitly absent. |

Retain `open-plan-server-check`, `browser-document-open-check` and `native-document-open-check` as D1 transport regressions. They do not prove component installation today and must not be re-labelled as this P1 proof.

## Ordered Handoff

1. Complete the server-owned fresh `stdio+gis` profile/readiness packet; without it any GIS bytes remain synthetic.
2. Add shared runtime BLAKE3 and strict raw PackageDescriptor admission; then add shared lease fields/corpus.
3. Add the hub’s selected-only asset port and exact trusted-catalog accessor.
4. Implement browser/native installers and four equality fences, with renderer dispatch explicitly stopping before WGPU.
5. Separately replace browser WGPU raw URL loading and native directory scanning with `VerifiedProgramSource`, then implement the event-driven `attach_backbone` successor. Only that later packet can claim GIS map rendering.

## Explicit Nonclaims

- No current browser or native process has executed this lease path.
- No byte-serving route, shared runtime BLAKE3, strict browser descriptor parser, verified-module source, or native lease exists in current source.
- No WGPU rendering, GIS map UI, document member open, collaboration mutation, inference action or plugin hot rotation is accepted here.
