# Descriptor Emitter Dependency Closure

The descriptor's canonical type already has a standalone owner. `semio-framework-manifest-plugin` defines `PackageDescriptor`, `CanonicalDescriptorValue` and `ASSEMBLY_FAILED_PLUGIN_ID`. The framework facade reexports them unchanged. The emitter also obtains retained-turn and budget types from that facade. The actor crate owns the retained turn types and the kernel identity crate owns `Budget`. These imports can point to their owners without altering the descriptor schema.

Changing those imports in the emitter alone does not prune the normal Cargo closure. A direct walk of the current manifests with the repository TOML parser found 355 distinct package names reachable through unconditional ordinary dependency declarations. Removing either emitter-to-facade edge individually still reaches all 355 names. A walk omitting both facades at every depth leaves 237 names before supplying replacement dependencies. That count describes a hypothetical graph subtraction and does not forecast the final crate or unit count. The walk excludes target-specific, optional, development and build dependencies. The current sealed successful descriptor-emitter receipt independently confirms 584 compiler units, including both facades, two full DSL units, two replication units, two replication-history units, and the OS store engine. The audited dependencies therefore occur on the actual normal emitter build path.

The strongest remaining edges are:

| Dependency | Paths that survive a direct emitter import change |
| --- | --- |
| Framework facade | Guest contract, interpreter runtime, plugin ABI, actor input and actor-input wire |
| OS kernel facade | Guest contract, interpreter runtime, interpreter imports and plugin ABI |
| Full DSL | Job and Pack reach the DSL-record crate, which has an unconditional full DSL dependency |
| Replication | Manifest plugin needs the replication `SchemaId`; guest contract reaches replication through kernel JSON and the kernel. The kernel uses replication command and history surfaces directly. |

The emitter's Pack encoder and decoder come from the OS Pack bridge, which already forwards through the canonical one-field record body. The interpreter's entropy and original operation authority have narrower existing owners in the OS identity and VCS entity-identity crates. Those substitutions alone would still leave the interpreter runtime's channel admission and the shared ABI's actual IO and `ArtifactStoreSnapshot` contracts. The shared ABI bundles those store-aware contracts with the wire data used by descriptor execution. A meaningful next closure cut would require separating that wire subset from the store-aware ABI and selecting only the canonical Pack, authority and channel owners. Excluding full DSL and replication after that requires separate work on the owner edges in the table.

No package imports, manifests, lockfiles, descriptor bytes, or source freshness settings were changed. No native build or Cargo resolution ran. The direct manifest graph and retained compiler evidence establish why a local facade substitution would leave the failing replication-history input in custody.
