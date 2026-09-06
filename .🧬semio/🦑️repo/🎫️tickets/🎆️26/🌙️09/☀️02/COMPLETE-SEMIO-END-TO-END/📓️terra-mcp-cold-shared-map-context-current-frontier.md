# Cold Shared Map Context to MCP: Current Frontier

## Outcome

The smallest independent Home slice is an authenticated, read-only MCP resource for the **currently active canonical checkpoint pair** of one scoped document. It is not a Map renderer, local fixture, inferred summary, Store mount, journal writer, or durable approval committer.

The exact current URI should be:

```
semio://workspace/scopes/{percent-encoded-space-id}/{percent-encoded-document-id}/checkpoint
```

It gives an MCP client the verified `Pack` and `SPR` bytes plus the exact scope, descriptor digest, active checkpoint identity, ETag, authority/catalog generations, and frontier from which a later selected GIS actor may make a semantic projection. The resource is a frozen checkpoint, not a live mutation stream or a statement that a Map actor is running.

This avoids WG's Store-owned three-member admission, WAL witness, recovery, and durable committer work entirely. The existing MCP crate deliberately treats plugin document bytes as host-opaque, and its Cargo package does not depend on `semio-s-plugin-gis`.

## Current path and the missing link

| Boundary | Current authoritative behavior | Gap |
| --- | --- | --- |
| Authenticated descriptor discovery | `HubRemoteBinding` owns a ready, membership-scoped `AuthorizedDescriptorSnapshot`; `workspace/artifacts` lists `scope`, `descriptorDigestV1`, and the scoped descriptor resource. | Correctly exposes metadata only. |
| Canonical body acquisition | The fixed protected route is `GET /spaces/{space}/documents/{document}/active-checkpoint/pair`. The Hub rechecks member authorization before body, before each record, and before terminal framing. | Already present. |
| Transport integrity | The MCP remote pair decoder requires exact media type, ETag, scope, descriptor digest, pair record order, Pack/SPR hashes, aggregate hash, terminal frame, and `<= 4 MiB` total verified payload. | Already present. |
| Retention/revocation | `CanonicalPairActor` owns/wipes cached `PairBytes`, has a mount id, and `finish_mount_return` rechecks binding generation, authority generation, session/snapshot, descriptor digest, and active actor mount. | No body projection API is exposed to the workspace. |
| MCP resource surface | Hub workspace resources list only each `.../descriptor` URI. Generic `semio://artifact/{id}` intentionally returns retryable unavailable in Hub mode. | Add the scoped `checkpoint` resource; do not overload generic artifact ids, which lack a space discriminator. |

Source anchors:

- [`workspace/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs) lines 1376-1388 reject Hub body reads; 1661-1720 lists/reads authenticated descriptor-only resources; 1723-1770 publishes only descriptor resources.
- [`remote/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs) lines 520-533 owns the adjacent scoped URI parser; 810-824 is the synchronous native mount bridge; 965-982 proves the existing exact mount-to-frozen-base pattern.
- [`pair/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs) lines 16, 230-305, 507-665, and 885-978 own the 4 MiB limit, wipe-on-drop pair bytes, mount fences, and byte-level verification.
- [`hub bin.rs`](../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs) lines 2843-2929 is the protected canonical-pair route. It sets `private, no-store` and revalidates authorization throughout body production.

## Exact API/owner seam

Keep `PairBytes` private and do not refetch a raw URL from the MCP workspace. Add the following crate-private projection seam in the remote pair/binding boundary:

```rust
pub(crate) fn project_mounted_canonical_pair<T>(
    &self,
    mount: &CanonicalPairMount,
    wall_now_ms: i64,
    project: impl FnOnce(
        &CanonicalPairMountIdentity,
        &ArtifactFrontier,
        &[u8], // Pack
        &[u8], // SPR
    ) -> Result<T, CanonicalPairMountError>,
) -> Result<T, CanonicalPairMountError>;
```

`NativeHubBindingDriver` should add one named `read_canonical_checkpoint(...)`/`project_canonical_checkpoint(...)` operation that first builds its own `OperationContext` exactly as `gis_map_inference_base` does, calls `mount_canonical_pair`, and invokes this projection. `HeadlessWorkspace::read_resource` calls only that driver operation after parsing the scoped URI and resolving its exact `DocumentScope` from the binding's ready snapshot.

The projection must:

1. Require `mount.identity.scope == requested_scope`, an unexpired ready snapshot in the binding's one configured space, and exact current descriptor digest, binding generation, authority generation, mount id, and cached pair identity.
2. Borrow `PairBytes` only while the private actor owns it; the callback is synchronous/non-reentrant and may return a bounded owned JSON string only. It must not `await`, expose a `PairBytes`, a `CanonicalPairMount` constructor, or a raw cache handle.
3. Perform the same final binding/snapshot/actor fence as `finish_mount_return` after projection and before publishing `ResourceContent`. Do not hold the binding state lock while taking the pair actor lock; existing invalidation takes the actor lock before changing state.
4. Convert cancellation, deadline, body limit, stale completion, and revoked state through the existing `pair_mount_error_to_gateway` mapping. The returned JSON is authorized at that final fence; revocation after the response has been linearized cannot retract a response already delivered.
5. Let the pair cache still wipe its original Pack/SPR on invalidation/eviction. The separate resource response copy is intentionally visible to the authenticated MCP caller and must be bounded; it is not a cache escape.

The resource codec should be schema-first, e.g. `semio.mcp.canonical-checkpoint-resource/v1`, with exact canonical fields:

```json
{
  "schema": "semio.mcp.canonical-checkpoint-resource/v1",
  "scope": { "spaceId": "…", "documentId": "…" },
  "descriptorDigestV1": "<64 lowercase hex>",
  "activeCheckpointId": "<64 lowercase hex>",
  "etag": "\"<64 lowercase hex>\"",
  "authorityGeneration": 1,
  "catalogGeneration": null,
  "frontier": {
    "documentId": "…",
    "headEditOrdinal": 0,
    "headEditId": "…",
    "lastCommitSeq": 0,
    "chainHash": "<64 lowercase hex>"
  },
  "pack": { "byteLength": 1, "sha256": "<64 lowercase hex>", "base64": "…" },
  "spr": { "byteLength": 1, "sha256": "<64 lowercase hex>", "base64": "…" }
}
```

Use the same full lower-case hex and `ArtifactFrontier` vocabulary already used by the pair decoder and directory schema, not a new Map revision type. Recompute the two output SHA-256 values during the bounded projection or carry the verified header values through a private typed value and assert their equality before serialization. The JSON response has two byte representations, but `Pack` and `SPR` need separate base64 fields; the existing folder-only resource currently emits Pack alone and is not an adequate codec.

`HUB_PAIR_MAX_VERIFIED_BYTES` caps raw bytes at 4 MiB. Before base64 allocation, use checked arithmetic for each `4 * ceil(n / 3)` and for the complete response length; a 4 MiB raw pair needs at most 5,592,408 base64 characters before metadata. Set and test one explicit MCP resource text ceiling above that calculated maximum (6 MiB is adequate) and reject before allocating any output over it.

## Minimal Home edits

1. In `remote/🦀️.rs`, add sibling `checkpoint_resource_uri` and `parse_checkpoint_resource_uri` next to the descriptor helpers, plus the driver-owned mount/projection operation above.
2. In `workspace/🦀️.rs`, add a `read_canonical_checkpoint_resource(scope, uri)` branch before generic `semio://artifact/`, and add one `Resource` per authorized document in Hub mode. The displayed resource must use the exact scoped URI, not document-id-only `semio://artifact/{id}`.
3. In `context/🦀️.rs`, extend `WorkspaceResourceRegistry::is_workspace_uri` to recognize only the `semio://workspace/scopes/` prefix. Otherwise MCP dispatch rejects the new resource before `HeadlessWorkspace` sees it.
4. Add `workspace/🧫️fixtures/🔐️canonical-checkpoint-resource/{🧬️.schema.json,🔣️.json}`. It may reuse the pair fixture's actual Pack/SPR framing as input, but its expected resource JSON must be independently decoded/checked, not merely compare an implementation-generated string.
5. Correct the independently visible GIS selector bug at `workspace/🦀️.rs:1922`: reject unless **both** artifact schema and kind exactly match. Current `schema != expected && kind != expected` admits a descriptor with either one substituted. Use `||` for mismatch. This is a pre-existing inference-scope gate correction and should also be enforced by any Map-specific future projection; the generic byte resource itself remains artifact-generic.

Do **not** add a GIS `ArtifactPack` dependency to the MCP package, a local `Store`, a `DocumentOpenPlan` issue/exchange, a socket grant, or a generic caller-controlled URL. `ArtifactOpen` is a local channel/socket lifecycle and does not prove a cold checkpoint read; `artifact_snapshot` remains based on generic id/probe semantics and should not be overloaded.

## Laws required before claiming the slice

### Neutral/schema law

Fixture cases: valid small GIS Map pair; percent-encoded scope; same document id in another space; replaced descriptor digest; replaced active checkpoint id/ETag/frontier; Pack/Spr length/hash/base64 mismatch; total response over the calculated base64 ceiling; cancellation/deadline; and revocation/descriptor refresh between mount and final publish. A semantic Map fixture is only acceptable if its bytes were first accepted as a canonical pair; no local JSON Map stand-in.

### Native MCP law

Extend the existing authenticated workspace fixture at `workspace/🦀️.rs:2070-2140` and the pair test transport at `remote/pair/🦀️.rs:1003+` with a uniquely named law such as:

`authenticated_hub_checkpoint_resource_projects_exact_verified_pair_and_never_crosses_scope`

It must prove that the resource's decoded Pack/SPR match the verified pair input and listed hashes, the response scope/frontier/descriptor are exact, a same-document-other-space URI is rejected before transport/body allocation, and revoke/descriptor change at the final fence yields no `ResourceContent`. Keep the existing pair cancellation/expiry/wipe checks; add a projection-specific assertion that an invalidated cache wipes its original payload after the result copy is gone.

Register this exact law alongside `mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority` in `runNativeDocumentAdmissionLaws` in `hub/📜️script.ts:7647-7657`; do not hide it under an all-tests target.

### Direct process law

Add a sibling of `proveMcpWorkspaceProcess` in `hub/📜️script.ts:1203-1290`, or extend it only after separating response counts. It must:

1. Start a real Hub and create a private space for the MCP credential holder.
2. Announce a selected GIS Map descriptor and publish a **real Hub CAS/directory checkpoint** through the normal `reserve_artifact_cas → stage → publish_reserved_artifact_checkpoint` authority path. `hub bin.rs:7247-7281` demonstrates the owned Hub publication sequence, but its `test.artifact` payload is not sufficient as the process oracle.
3. Start the credential-FD MCP child; `resources/list` must include the exact scoped `checkpoint` URI; `resources/read` must decode the neutral checkpoint codec and independently recompute Pack/SPR SHA-256/base64, scope, descriptor digest, active checkpoint, ETag, and frontier.
4. Prove denied cross-space/same-document reads and a membership revoke or descriptor-generation change that occurs before final projection returns no body. Keep stdout below the existing 32 KiB direct-child cap by publishing a genuinely small canonical GIS Map checkpoint.

The current process helper at `hub/📜️script.ts:1077-1114` only announces `os.agent.probe`; `proveMcpWorkspaceProcess` then exercises `artifact_open` and `artifact_snapshot` at 1236-1269. It has no active canonical pair, so it cannot qualify this feature. Reusing the existing `canonical_pair` Hub route alone is likewise insufficient: the process law must prove the authenticated MCP resource endpoint and its own codec.

## Nonclaims and next boundary

This makes cold, authorized shared Map context observable through MCP. It does not interpret a GIS snapshot, call an external model, render, mutate, publish a three-member decision, persist, undo, or synchronize a collaborator. A future semantic projection must run inside the selected verified GIS actor after Worker containment and use the same pair identity; it must not bypass this authority with a local fixture or direct component bytes.
