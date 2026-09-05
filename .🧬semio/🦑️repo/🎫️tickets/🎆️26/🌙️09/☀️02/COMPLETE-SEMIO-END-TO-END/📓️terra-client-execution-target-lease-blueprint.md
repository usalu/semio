# Shared Client Execution-Target Lease Blueprint

## Current Verdict

**RED: browser and native do not share an immutable complete execution-target lease.** The hub catalog generation already commits the requested identity tuple, but neither client retains every committed field through plan exchange, reconnect, and app opening. Browser checks more than native; the native path can proceed from a local manifest/surface expectation without immutable component or descriptor identity. The current public plan also omits \`parentDialect\`.

This is a current-source, no-build audit.

## Authoritative Tuple and Present Losses

| Field | Hub source | Browser today | Native today |
| --- | --- | --- | --- |
| Scope | Issuer binds space and document in \`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1970-2055\` | Plan scope equals the intent at \`🧵️backbone-worker.ts:481-485\` | Scope is passed to DirectoryClient and rechecked at \`🏪️store/🔄️sync/🦀️.rs:2045-2058\`. |
| Catalog generation | SHA-256 commits the sorted full target tuple at \`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:563-644\` | Parsed but neither installed target nor active authority retains or compares it. | Returned in DocumentSocketAuthorityV1 but absent from the expectation and equality relation. |
| Package IDs/version | Trusted selection at \`trusted-catalog/🦀️.rs:456-475\` | Compared. | Compared. |
| componentSha256, componentBlake3, descriptorByteSha256 | Same selection | Compared at \`🧵️backbone-worker.ts:486-501\`. | Present in the response but absent from DocumentSocketSurfaceExpectationV1; matches_surface compares none (\`📇️directory/🔌️client/🦀️.rs:244-304\`). |
| Artifact kind/schema/pack hash | Same selection | Compared, including configured pack hash. | Schema/pack hash are compared; kind is only a local surface field. |
| parentDialect | Selection and generation bind it at \`trusted-catalog/🦀️.rs:258-266,606-644\`; private hub authority retains it at \`🚀️bin.rs:993-1040\`. | **Absent from plan and installed target.** | **Absent from plan, authority, and expectation.** |
| Surface tuple | Same selection | Complete equality and React target required. | Manifest-derived equality with no verified-catalog backing. |
| Grant | Role-derived and catalog-bound | Parsed then discarded from BrowserDocumentSocketAuthorityV1. | Returned, but no local capability gate consumes it. |

The current Rust and TypeScript DocumentOpenPlanV1 schemas carry scope, catalog, package, artifact, surface, grant, and revalidation but not parent dialect (\`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:686-701\`, \`🧬️schema/🟦️.ts:426-440\`). The hub checks parent dialect privately on exchange and socket validation (\`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2158-2164,2606-2612\`), but a public Flow member-open consumer cannot receive it.

Browser InstalledDocumentExecutionTargetV1 contains package, artifact, and surface only (\`🧰️framework/🛍️products/💻️os/🟦️.ts:557-581\`). Its worker validates that partial target then collapses the result to receipt, schema, pack hash, and surface ID (\`🧵️backbone-worker.ts:439-545\`). Native has no corresponding type: PersistenceBinding::Hub and ArtifactActorConfig carry base URL, space, optional surface, document and schema only (\`🏪️store/🔄️sync/🦀️.rs:82-111\`).

## Smallest Shared Schema

Add one public, non-secret immutable value to both directory schema files, not a browser-only type or a native manifest extension:

    DocumentExecutionTargetLeaseV1 {
      schema: "semio.os.document-execution-target-lease/v1"
      version: 1
      scope: DocumentScope
      catalog: { generationId }
      package: { pluginId, packageId, version,
                 componentSha256, componentBlake3, descriptorByteSha256 }
      artifact: { kind, schema, packSchemaHash }
      parentDialect: { artifactKind, standard, subset }
      surface: { surfaceId, appId, windowKindId, role, rendererTarget }
      grant: { read: true, write: boolean, observe: true }
    }

Reuse the existing DocumentOpen package/artifact/catalog/surface/grant nested types. Add DocumentOpenParentDialectV1 to \`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts}\`, with strict three-field parsing and the hub's existing nonempty, bounded, non-control, trim-free semantics. Add it to DocumentOpenPlanV1, or public Flow member open cannot prove its parent is the catalog-selected target.

The lease is a locally verified installed selection bound to one requested scope. It is neither a plan receipt, socket grant, session token, nor component bytes. The hub retains authority for membership, receipt one-use, expiry, checkpoint and revalidation.

Define one pure cross-language \`matches_open_plan\` / \`matchesOpenPlan\` relation from a neutral fixture. It compares every field above before exchange and after plan receipt. Do not maintain separate hand-written browser/native comparisons.

## Exact Implementation Owners

1. **Contract:** extend the two directory schemas and \`document-open-plan-v1\` fixture. \`DocumentOpenPlanAuthorityV1::public_plan\` at \`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1045-1060\` must project parent dialect.
2. **Browser install:** replace InstalledDocumentExecutionTargetV1 in \`🧰️framework/🛍️products/💻️os/🟦️.ts\` with the lease, no compatibility alias. Only local verified component/descriptor installation may construct it. ShellHost must receive it from the selected installed app, not derive it from canonicalSurfaceId (\`🏛️ShellHost/🟦️.tsx:3442-3458\`).
3. **Browser transport:** hub PersistenceBinding requires the lease. \`documentOpenPlanAuthority\` and BrowserDocumentSocketAuthorityV1 at \`🧵️backbone-worker.ts:439-545\` retain the validated lease through close rather than reducing it to schema/hash/surface.
4. **Native install:** add the lease to Rust, then to PersistenceBinding::Hub, ArtifactActorConfig and the preclaimed selection in \`🏪️store/🔄️sync/🦀️.rs:1060-1168\`. WGPU may create it only from a verified native catalog, never a ProgramBridgeEntry manifest.
5. **Native transport:** DirectoryClient::admit_document_socket at \`📇️directory/🔌️client/🦀️.rs:748-826\` takes the lease as its comparison input. A private surface is derived for URL/Hello only after complete lease-plan equality.
6. **Flow consumer:** receives exact parent dialect, scope and selected target after plan/socket acceptance; it never receives a bundle path or raw plan receipt.

## Lifecycle and Invalidation

The local verified catalog publishes a generation-scoped lease set atomically. Browser retains a lease only in its ArtifactState; native retains it in the actor's claimed selection. Neither session storage, URL/query, plugin arguments, nor local storage may construct trust from this value.

Before every plan request and again after parsing it, both transports verify the lease is live in the local generation. They verify again after receipt exchange and before WebSocket/SocketHello. A local catalog replacement cancels every actor holding its old generation and returns stale-target; it must not silently retry with another target. A hub generation mismatch, membership revocation, descriptor/checkpoint change, expiry, or cancellation discards the active authority. Reconnect may reuse only the same still-live local lease; an editor cannot silently downgrade to viewer.

Grant gates local behavior (write false prevents outbound mutations, observe false prevents observation/presence) but is never a credential. Server checks remain independent.

## Neutral Corpus and Exact Laws

Create \`document-execution-target-lease-v1\` schema/fixture with an AJV plus byte-framing oracle. It must reject one-field substitutions for both scope IDs, catalog generation, all three package digests, IDs/version, artifact kind/schema/pack hash, all parent-dialect fields, every surface field, and all grant bits. Include unknown/missing keys, invalid hash casing/length, control/trim/overlength text, parent-kind mismatch, editor/write mismatch, viewer/write mismatch, local-generation turnover after a valid plan, cancellation before exchange, cancellation after exchange before socket, and stale reconnect.

Register two exact, non-mocked laws on the same corpus:

- **Browser:** an installed verified Flow package constructs the lease; ShellHost passes it to the worker; complete equality is required before BFF request, after plan and before SocketHello. Every hostile case has zero app publication; absent or stale lease has zero HTTP/WS effects.
- **Native:** a verified native Flow catalog constructs the same lease; WGPU preclaims it once; ArtifactHost and DirectoryClient execute all three equality fences. Turnover/cancellation returns the owned selection and closes the actor without stale codec use.

Current browser-document-open-check and native-document-open-check are insufficient because generation, parent dialect and local grant retention are absent, and native does not compare immutable package digests. Register the new exact commands in the source-owned hub script, then project target and launch seed before generating launch metadata.

