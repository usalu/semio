# Actor Transport Boundary

The next transport inventory inspects current TypeScript declarations, including named forwarding exports that the earlier top-level declaration-only inventory omitted. The actual private shard-client module exports 91 names. InboundMessage, OutboundMessage, PendingEntry, ShardInstanceOwner and ShardSlot are all exported in the current source. The previous suggestion that they were nonexported was incorrect; they remain implementation and test surfaces that need deliberate interface ownership.

There are 74 external private references across 40 files in the five-target inventory (shard-client 52, retained source 4, component codec 2, liveness 1, wire-turn 15). The retained source four references have since been adopted to its defining public facet. These counts are literal source references, not whole semantic dependency counts.

The final whole neutral receiver after five facet adoptions reports 26 Actor-target findings across 13 source-target pairs. Their remaining five targets are shard-client, wire-turn, bootstrap test supply, component codec and liveness. Root package scheduler resolution previously had a broad test alias to the shard implementation, so new subpath consumers must resolve from actual package exports or explicit defining facade files. That test alias also needs replacement when the shard contract is separated.

The clean next implementation should define the production transport/owned-result port separately from test dependency and backend-slot access. Existing return, codec, liveness and segmented-download owners should expose their own exact facets instead of being forwarded through the broad shard-client barrel. All current consumers and tests must adopt those owners directly; the complete class surface should not be copied wholesale into a new public barrel. Begin with a closed language-neutral contract and a failing actual package/export/consumer law, then qualify the real transport behavior and dependent clients.

Current exported forwarded names:

- `ACTOR_RETURN_RESULT_MAXIMUM_BYTES` → "../📤️return/🟦️.ts"
- `decodeActorReturnDrive` → "../📤️return/🟦️.ts"
- `decodeActorReturnResult` → "../📤️return/🟦️.ts"
- `encodeActorReturnDrive` → "../📤️return/🟦️.ts"
- `encodeActorReturnResult` → "../📤️return/🟦️.ts"
- `ActorReturnControl` → "../📤️return/🟦️.ts"
- `ActorReturnDrive` → "../📤️return/🟦️.ts"
- `ActorReturnFault` → "../📤️return/🟦️.ts"
- `ActorReturnIdentity` → "../📤️return/🟦️.ts"
- `ActorReturnOrigin` → "../📤️return/🟦️.ts"
- `ActorReturnPageReceipt` → "../📤️return/🟦️.ts"
- `ActorReturnResult` → "../📤️return/🟦️.ts"
- `admitSegmentedDownloadChunk` → "./📤️segmented-download/🟦️.ts"
- `admitSegmentedDownloadOperationId` → "./📤️segmented-download/🟦️.ts"
- `SEGMENTED_DOWNLOAD_CONTRACT` → "./📤️segmented-download/🟦️.ts"
- `SEGMENTED_DOWNLOAD_REFUSAL` → "./📤️segmented-download/🟦️.ts"
- `ACTOR_CODEC_REFUSAL` → "./🧬️component-codec/🟦️.ts"
- `actorCodecAnswer` → "./🧬️component-codec/🟦️.ts"
- `ActorCodecExports` → "./🧬️component-codec/🟦️.ts"
- `ShardCodecAnswer` → "./🧬️component-codec/🟦️.ts"
- `ShardCodecDocumentPair` → "./🧬️component-codec/🟦️.ts"
- `ShardCodecRequest` → "./🧬️component-codec/🟦️.ts"
- `SHARD_LIVENESS_POLICY` → "./🫀️liveness/🟦️.ts"

Root has made no transport implementation changes during this inventory. The original native fleet continues shared concrete receiving and typed reconstruction.

## Codec and Liveness Defining Contract

The first implementation cut separates two existing leaf owners from the broad shard barrel. The closed public API corpus now declares ./codec (the existing two runtime operations, four first-party types, and the defining request schema) and ./liveness (the original frozen policy and its defining schema). Both schemas remain at their sole semantic owner. The actual installed package resolver refuses both new entries before implementation. That is an actual missing-entry probe; a repository test execution remains pending while the current supported source receiver is in build discovery. The exact hand-authored contract is retained in actor-codec-liveness-contract-inputs.json.

Actor GUI current5 and General producer52243 are confirmed missing by their tool handles and a current process census; their logs contain only discovery progress and no terminal/test acceptance. The code/caller reports preserve that limitation. No bypass of immutable graph authorization or unvalidated graph-reuse flag is used.

The explicit ./codec and ./liveness facades and exact manifest entries are now authored. The existing product-denial corpus requires 16 defining owners and 10 actual production/public bundle entries. Four current external references in three OS clients were adopted directly, including the one schema import selecting actorShardLivenessSchema while retaining its local binding. Same-owner Actor imports remain at their semantic owners. No new concrete transport API or wildcard barrel is published. Current client roster:

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts (1 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️component-codec-reply/🟦️.ts (1 imports)

Full current strict/behavior/API/product-denial qualification remains pending. The supported DSL source verification exhausted its real 180 second original deadline in project discovery before collecting a test; a read-only fleet audit is being arranged for the shared graph bottleneck. No direct unapproved Nx or unvalidated graph reuse substitutes for that receiver.
