# MCP Cold Shared Map Context

## Outcome

The source and neutral/schema boundary for an authenticated, cold canonical checkpoint resource is implemented. Native and direct-process qualification remain separate and pending.

The resource is exactly:

`semio://workspace/scopes/{percent-encoded-space-id}/{percent-encoded-document-id}/checkpoint`

It exposes the already-verified canonical Pack/SPR pair as bounded JSON with exact scope, descriptor digest, active checkpoint, ETag, authority/catalog generation, frontier, byte lengths, SHA-256 digests, and base64. It does not decode GIS, mount Store, mutate, commit, render, or claim a selected GIS actor is active.

## Implemented boundary

- The remote pair actor has a crate-private synchronous projection callback over borrowed retained bytes. It requires the exact active mount/cache identity and performs the binding, authority, ready-snapshot, descriptor, mount-id, and actor checks both before projection and again before returning the owned result.
- The binding state lock is released before the actor lock is acquired. Invalidation retains the existing actor-before-state order.
- The codec enforces the existing 4 MiB raw pair ceiling and a 6 MiB MCP text ceiling. Checked base64 lengths and a 16 KiB metadata allowance are validated before either base64 allocation.
- `HeadlessWorkspace` lists descriptor and checkpoint resources for every authenticated Hub document. It rejects another space before invoking the driver or pair transport.
- `WorkspaceResourceRegistry` routes only the scoped workspace prefix to a bound workspace.
- The GIS inference selector now requires both the exact kind `s.gis.gismap` and exact schema `gis.map`; substituting either is rejected.

## Neutral evidence

Registered command:

`@semio-tech/framework-os-mcp-rs:canonical-checkpoint-resource-check`

Direct Nx receipt on 2026-09-06:

```text
canonical-checkpoint-resource-oracle: AJV=1 parts=2 hostile=12 lifecycle=4 selector=3
canonical-checkpoint-resource-source: uri=scope-exact retained-pair=private final-fence=2 raw-limit=4MiB text-limit=6MiB GIS-selector=exact
NX Successfully ran target canonical-checkpoint-resource-check for project @semio-tech/framework-os-mcp-rs
```

The neutral fixture is validated by AJV. Node's independent SHA-256/base64 projection matches both expected pair parts. Hostiles cover same-document/other-space, descriptor/checkpoint/ETag/frontier substitution, Pack/SPR length/hash/base64 substitution, the output ceiling, cancellation/deadline, and revoke/descriptor refresh before publication.

Rust syntax was parsed successfully with the pinned nightly rustfmt over the four touched Rust modules. This is not a compile or runtime verdict.

## Native registration and pending state

Registered native command:

`@semio-tech/framework-os-mcp-rs:canonical-checkpoint-resource-native-check`

It selects exactly:

1. `authenticated_hub_checkpoint_resource_projects_exact_verified_pair_and_never_crosses_scope`
2. `gis_map_inference_selector_requires_the_exact_kind_and_schema_pair`

The first law mounts the canonical-pair corpus, projects exact independently pinned Pack/SPR bytes and hashes, rejects the same document in another space before transport, and proves both revoke and descriptor refresh after projection but before the final fence return no result and wipe the retained original bytes.

The same checkpoint law is also registered beside `mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority` in `runNativeDocumentAdmissionLaws`.

Native execution is pending because the retained catalog gate exclusively owns `public-member-open-sol-target` in Cargo process 32171/rustc 33170. No second Cargo process was started.

## Launch evidence

The source and native commands are registered in the project and launch seed. Plugin-registry generation and immediate freshness check passed:

```text
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages)
.vscode/launch.json regenerated
plugin registry generated catalog and launch bytes are fresh.
```

Generated launch entries begin at `.vscode/launch.json:6580` and `.vscode/launch.json:6588`. The native entry uses the ticket-owned exact artifact directory, the coordinated `public-member-open-sol-target`, one Cargo job, and the long build/orchestration/command budgets.

## Direct-process frontier

The direct process law is not implemented or claimed yet. The current public Hub surface can upload content-addressed blobs and read the active pair, but it has no authenticated command that reserves CAS ownership and publishes an `ArtifactCheckpoint`. The only complete `reserve_artifact_cas -> stage -> publish_reserved_artifact_checkpoint` producers are crate-private Hub test helpers. The existing MCP process journey creates `os.agent.probe`, opens a live probe Store, and performs `artifact_snapshot`; it neither selects a GIS Map descriptor nor publishes an active canonical pair.

Consequently, extending that journey today would either fabricate a checkpoint, inject backend state, add a test-only production control route, or reuse the prohibited probe fixture. None qualifies the requested real Hub CAS/directory publication. The clean next boundary is an existing first-party checkpoint producer making the normal publication API reachable to the process harness; then the credential-FD MCP child can prove `resources/list`, `resources/read`, cross-space denial, independent digests/base64/frontier, and revoke/descriptor-refresh no-body behavior without changing this resource design.
