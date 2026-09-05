# Browser GIS WASM Rejection and Immutable Execution Lease — P0

## Verdict

**Current browser opening intentionally rejects the selected GIS Map WASM target before it can exchange a socket grant.** This is not a renderer defect alone. The browser compares the hub plan against a caller-supplied `InstalledDocumentExecutionTargetV1`, which contains no retained/verified component or descriptor bytes, catalog generation, grant, descriptor digest, expiry, or revalidation. It then explicitly requires `rendererTarget === "react"`.

The smallest honest P0 is therefore **verified installation plus an immutable, scoped execution-target lease**. It may end in a localized `renderer-unavailable` state for GIS WASM; it must not claim React or WGPU rendering. It begins only after the server has an actual current trusted `stdio+gis` generation. Current trusted-bundle evidence is source/neutral only, not materialization/candidate/process acceptance.

No browser, native, or process command was run for this audit.

## Exact current path and RED

| Boundary | Current source fact | Consequence |
| --- | --- | --- |
| Server selection | `DocumentOpenPlanV1` already projects descriptor digest, catalog generation, component SHA-256+BLAKE3, descriptor SHA-256, artifact, parent dialect, surface, grant, checkpoint, and revalidation ([TS schema](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:432>), [strict parser](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:529>)). The hub resolves against `openable_catalog` when issuing at [bin.rs:2091](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2091>) and rechecks catalog generation/selection at [2236–2241](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2236>). | A good short-lived selection receipt exists; it is not an installed executable. |
| Trusted profile | The staged `stdio+gis` profile source defines two packages, 28 native codec rows, and one read-only GIS Map viewer target. [Its report](./📓️sol-trusted-stdio-gis-bundle.md) records source/neutral evidence only. | There is no accepted fresh profile process from which a browser can safely acquire the GIS component. |
| Browser local expectation | [`InstalledDocumentExecutionTargetV1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:557>) holds only package/artifact/dialect/surface and is optional caller input in `PersistenceBinding` at [580–583](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:580>). | It is forgeable data, not byte ownership. It omits catalog generation, scope-local owner, descriptor digest, grant, revalidation, byte lengths, and buffers. |
| Browser rejection | [`documentOpenPlanAuthority`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:482>) compares plan values to that supplied structure, then unconditionally rejects `plan.surface.rendererTarget !== "react"` at [504](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:504>). `requestDocumentSocketAuthority` calls it before receipt exchange at [528–546](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:528>). | A real `wasm` GIS plan is denied locally even when the hub selected it. |
| Browser component loading | `PluginRuntime.loadPluginModule` and kernel activation follow a raw caller `moduleUrl`; the WGPU bridge does the same ([bridge](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:211>)). | No read verifies component SHA-256, component BLAKE3, descriptor byte SHA-256, selected generation, scope, or grant. |
| Hub assets | The hub router has plan/grant routes but no protected selected component/descriptor route; `HubState.openable_catalog` is only a selection authority ([bin.rs:1411](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1411>)). | The client cannot receive trusted selection-bound bytes even if its type were repaired. |
| Existing browser evidence | `browser-document-open-check` is wired in [hub script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3277>), but its own oracle asserts `rendererTarget === "react"` at [1755–1756](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1755>). | It cannot qualify GIS WASM. |
| Native parity gap | Native `DocumentSocketAuthorityV1` retains full plan data, but `matches_surface` compares only artifact/package/version/surface ([client](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:242>)); reconnect similarly constructs a partial expectation ([sync](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1998>)). | A shared fields relation is necessary; this browser P0 must define it once, not fork one weaker native predicate. |
| WGPU runtime | `load_wasm_plugins` scans a caller-selected filesystem root ([ProgramBridge](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:723>)); descriptor read can fall back to empty and `attach_backbone` returns an explicit retired error ([281–282](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281>)). | A lease must not be passed to this scanner or treated as WGPU map rendering. |

## P0 contract: fields first, private owner second

Replace (do not extend with aliases) `InstalledDocumentExecutionTargetV1` for hub documents with a schema-first `DocumentExecutionTargetLeaseFieldsV1`.

```text
schema/version
scope { spaceId, documentId }
catalog { generationId }
descriptor { sha256, byteLength }
package { pluginId, packageId, version,
          componentSha256, componentBlake3, descriptorByteSha256 }
component { sha256, blake3, byteLength }
artifact { kind, schema, packSchemaHash }
parentDialect { artifactKind, standard, subset }
surface { surfaceId, appId, windowKindId, role, rendererTarget }
grant { read, write, observe }
checkpoint? and revalidation (exact existing shapes)
```

Invariants:

- `component.sha256 == package.componentSha256`; `component.blake3 == package.componentBlake3`; `descriptor.sha256 == package.descriptorByteSha256`.
- Scope is the full `(spaceId, documentId)`; it shares the existing collision-safe runtime key, never merely `documentId`.
- Parent artifact kind equals artifact kind; the existing strict plan parser already enforces this relation.
- Surface role and grant relation remain exactly as current plan validation requires. GIS Map viewer has `write: false`; local mutation/outbox entry must reject before it queues work.
- The plan receipt, socket grant, session token, raw path, raw URL, and caller-selected package identity are **not** lease fields or constructors.

The public fields value is strict Rust/TS wire data. `DocumentExecutionTargetLease` is a private non-serializable owner:

- Browser: branded/frozen private constructor owns verified buffers and any private Blob/module URL; public callers receive a readonly fields view only.
- Native: crate-private constructor owns verified bytes/reader and the authenticated directory origin; it is not `Clone` and has no `FromValue`.
- Local `hubOrigin` comes from the credential-owning broker/client, not a hub JSON request. It must equal the protected transport origin used to fetch the bytes.
- One generated/equivalent `sameLeaseFieldsV1` compares every field above in both transports. No browser or native subset comparison is permitted.

## Smallest server addition: selection-bound asset port

Do not expose a generic package/hash/path download API. Add an internal exact-selection accessor to trusted catalog authority, conceptually:

```text
assets_for_current_selection(
  descriptor, requestedSurface, writable, currentGeneration
) -> VerifiedExecutionTargetAssets
```

It returns the selected verified descriptor/component only when package identity, version, all hashes, artifact, dialect, surface, grant and current generation agree. It is not a public package lookup.

Add protected, document-scoped asset reads alongside existing `open-plan` / `socket-grants`:

```text
POST /spaces/{space}/documents/{document}/execution-target/manifest
POST /spaces/{space}/documents/{document}/execution-target/component
POST /spaces/{space}/documents/{document}/execution-target/descriptor
```

Each accepts only the existing bounded `DocumentOpenIntentV1` (requested surface is preference, not authority), re-authenticates current membership/share binding, reloads descriptor, resolves current trusted selection, and obtains bytes only through that accessor. It returns either strict fields JSON or raw bytes. It never accepts package id, component digest, descriptor digest, catalog generation, local path, or receipt as a target selector.

The byte routes repeat current selection/authorization. Therefore a rotation or role change between manifest and component/descriptor reads yields a mismatch or denial; the client must discard all bytes. Use the trusted catalog’s current bounds—**64 MiB component and 4 MiB descriptor**—unchanged on both routes and clients.

The client receives the plan first, then manifest/assets, and performs final one-use plan → socket-grant exchange only after byte verification. The plan exchange remains the server’s final stale-generation fence. The asset port does not turn a plan receipt into a reusable download credential.

## Browser P0 slice

1. Add shared fields parser/schema + neutral fixtures under the existing directory contract. Derive `receiptFreeFields(plan)`; reject any manifest-field difference before retaining a byte.
2. Add a browser broker operation that can make only the three protected asset calls. It shares `state.docAbort` and the current deadline with [`requestDocumentSocketAuthority`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:513>).
3. Stream component/descriptors with explicit cap and progress; SHA-256 via Web Crypto and a browser-safe first-party BLAKE3 implementation. The only current TS BLAKE3 implementation is in the dev script, so move/reuse its algorithm in a normal runtime module; Web Crypto must not be claimed to provide BLAKE3.
4. Strictly parse raw descriptor bytes, require canonical re-encoding equality, package/version/hash self-consistency and exact equality to fields. JSON sibling manifests/raw URLs are not descriptor authority.
5. Only after both bytes verify, construct the private lease. Then permit `rendererTarget: "wasm"` through the plan comparison **only if this lease is supplied**. Route the verified GIS Map target to an explicit localized `renderer-unavailable` state; do not call `loadPluginModule`, `ActivationRegistry`, WGPU scanner, or `attach_backbone`.
6. Exchange the plan receipt and open the document socket only while the lease remains live. On cancellation, timeout, asset mismatch, plan expiry/stale response, session/membership revalidation change, descriptor/checkpoint change, catalog rotation, or socket rebootstrap, wipe buffers/revoke private URL and drop lease before any retry.

The browser must render explicit EN/DE `role="status"` for verification/progress and `role="alert"` for integrity/stale/renderer unavailable. It must not default locale or persist error status in the shared document.

## Neutral corpus and hostile matrix

Create `document-execution-target-lease-v1` fixture/schema, with AJV and an independent Node state machine plus a Rust fixture reader. One positive GIS Map viewer vector includes non-empty component/descriptor bytes and all fields. Required hostile rows mutate one field at a time:

- component SHA-256, BLAKE3, length, descriptor SHA-256/length;
- package/plugin/version, artifact/schema/pack hash, each dialect coordinate;
- scope space and document separately; catalog generation; each surface coordinate; each grant bit;
- descriptor canonical/trailing-byte/self-hash failure;
- a manifest from generation A plus component from B; stale plan after rotation;
- missing/oversized body, cancellation during manifest/component/descriptor/hash, deadline, and reconnect after lease invalidation;
- viewer write attempt; caller URL/path/module substitution.

Expected result is `unpublished` for every rejection: no activation registration, Blob URL, worker/open socket, retained byte buffer, or queued mutation.

## Exact gates

Existing gates are prerequisites, not proof of this packet:

```sh
bun nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --native
bun nx run os-hub:open-plan-check --skip-nx-cache
bun nx run os-hub:browser-document-open-check --skip-nx-cache
```

Register three new targets in the existing Hub `📜️script.ts`, project target, and launch **seed** (then regenerate launch output):

```sh
bun nx run os-hub:execution-target-lease-check --skip-nx-cache
bun nx run os-hub:execution-target-lease-check --skip-nx-cache -- --native
bun nx run os-hub:execution-target-lease-browser-check --skip-nx-cache
```

Suggested exact native FQNs after implementation:

- `artifact_authority::trusted_catalog::tests::selected_execution_target_assets_are_generation_and_digest_bound`
- `bin::tests::execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body`
- `os_directory::client::tests::execution_target_lease_compares_every_plan_and_verified_byte_field`

Suggested browser test names:

- `browser execution target lease verifies GIS wasm bytes before plan exchange`
- `browser execution target lease rejects every single-field substitution without publication`
- `browser GIS viewer exposes localized renderer-unavailable after verified lease`

A real process test follows only after the trusted-bundle native gate succeeds: authenticated viewer opens GIS Map, asset manifest/component/descriptor are fetched from the server-owned current generation, hashes verify, the one-use receipt exchanges, and the UI shows verified-but-renderer-unavailable. Rotation must invalidate generation A and require a fresh B lease. It proves no map renderer.

## Ownership and nonclaims

This P0 is owned by Hub artifact authority + OS directory/browser transport. It does not alter scoped socket revocation, Flow member opening, GIS mutation execution, raw plugin development installation, or WGPU rendering. It also does not claim that a browser can execute a WASI component or that native WGPU has a trustworthy loader; those are later consumers of the same verified lease.

