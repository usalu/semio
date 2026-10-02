# Independent Schema Source Audit

Read-only bounded source review completed 2026-10-02T11:05:32Z. Production, Git, worktrees, compilers and tests were untouched. Shared sources can change concurrently; no authorship is assigned. The portable 15-law/144-assertion receipt is inherited evidence, not a run by this reviewer. No native pass or full architecture correctness is asserted.

## Concrete Contract Mismatch: Artifact Single Replacement

`🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/🧱️neutrality/🔣️.json:711` defines `artifact-single-replace`: initial descriptor `a` has Old leaves, the single candidate has New leaves, `exports` starts empty, and expected publication succeeds with both descriptor and facet mirrors containing New leaves. The corpus contract at line2190 explicitly retains single replacement semantics. This preservation law must not be weakened or removed.

Actual global public operations cannot produce this authored lifecycle. First `register_artifact_schema_descriptor(old)` calls `register_descriptor` at `📇️registry/🦀️.rs:519`, inserts the old descriptor and publishes all four old facet mirrors at line523. Then `register_artifact_schema_descriptor(new)` calls `descriptor.mirror(&state)` at line521. `ArtifactSchemaDescriptor::mirror` at lines483–487 sees the previously published Old facets differing from New and returns `SchemaDescriptorRegistryError { registry: "schema-export", id: "a" }`. It never reaches the replacement at line522. Single replacement therefore fails even when no independently conflicting export was registered.

The required invariant is coherent single replacement of its descriptor-owned mirror together under the canonical lock, while refusing genuinely conflicting independently established facet/export state before any write. Strict batch admission must retain all-or-nothing publication. Ownership/provenance needs an explicit coherent design, rather than treating a descriptor's own old mirror as an external conflict or silently overriding a conflicting external publication.

`🧪️tests/🧱️neutrality/🟦️.ts:64–89` models descriptors and exports as separately initialized arrays; the Node oracle at lines101–104 separately initializes Maps. Both accept this fixture because initial descriptors do not populate the modeled mirrors. They therefore do not establish parity with the real global initialization path. The real native registry test `📇️registry/🧪️tests/🧱️neutrality/🦀️.rs:64` exercises only the two explicit mirror-conflict vectors. It never executes `artifact-single-replace` through initial and replacement public calls.

Required authored native witness: seed the initial Old descriptor using the actual global artifact single-publication operation, observe all four mirror leaves, publish New using the same public operation, then compare both descriptor and mirrors to the fixture's exact independent expected leaves. Retain the hostile external-mirror refusal/no-prefix witness. Add corresponding strict-batch conflict preservation and same-descriptor duplicate witnesses as appropriate. SourceREADY is invalid until the canonical source/corpus/public-operation lifecycle mismatch is corrected and native verification is refreshed.

## Other Bounded Source Findings

`📇️registry/🦀️.rs:506–535` validates every batch and proposed duplicate against the existing descriptor map and mirrors while holding one canonical lock, before publication. The private Descriptor trait has only concrete internal implementations; no user callback is invoked there. Ordinary error returns do not publish prefixes. This is a source observation, not allocation-failure or native concurrency proof.

`📇️registry/🦀️.rs:568–605` clones owner snapshots in terminated let statements, then invokes callbacks after the mutex temporary drops. Sorted callbacks delegate to those snapshot functions. No callback-under-owner-lock issue was found in the reviewed operations.

The authored native concurrency witness uses actual public inference batch operations, Barrier-started actors, independently expected winner snapshots, and rejects a losing batch prefix. The callback witness actually reenters publication/read functions with a bounded receive timeout. These are genuine authored public-operation tests, not source text assertions; they were not run here. Artifact descriptor-plus-mirror concurrency is not exercised by that inference-only witness. Successful initial-then-replacement mirror lifecycle is the concrete missing hostile witness identified above.

`📶️state/🦀️.rs` owns the sole reviewed Rust `pub enum StateClass`; the other matching component occurrence is GraphQL SDL string content. State's runtime manifest depends only on neutral Value. Its authored native laws use an independent Serde enum, exact wire/kebab vectors, controlled cancellation and owned-byte admission. `🧩️composition/🦀️.rs` is std-only; Option and Vec implementations own their projection steps. Its native authored fixture law calls those real implementations and checks exact independent expected step/identity outputs, with a separately authored local child carrier. Actual higher artifact-child projection remains an independent runtime obligation.

`⚛️component/🦀️.rs:8–13` privately imports direct lower State/Registry providers. Package glue reexports its component declarations and Validator, with no lower State/Composition/Registry facade found in the reviewed glue. Public field-state output references canonical lower StateClass; public version functions accept exact lower descriptor types; structural_validator_in accepts the lower registry directly.

Protocol's actual package is `📡️replication/📦️packages/🦀️rust/Cargo.toml`, lib name `protocol`. Its direct manifest uses lower Schema State/Registry and has no mainSchema dependency. `🎮️mutation/🦀️.rs:218` returns the exact lower StateClass. No mainSchema cycle or hidden lower facade was found in this bounded review. This is not an exhaustive workspace closure or products-absent deletion proof.

## Reviewed Canonical Hashes

| Source | SHA256 |
| --- | --- |
| Schema component | 2b7b7e4d71afdbe80d9e93bbe9617bc29a377cc2e7d7e3e36c1c9dad8d702313 |
| Schema State | 37b8ed5aa32be25c906439d2b9e1de9e9433fea30840f1c6ed1a83e0260e0004 |
| Schema Composition | c9392872eae0266885c61d986129b394feb44d245c149cb7d5538827544c2d55 |
| Schema Registry | 0ebd1d9fa0e03b82b781164c90986faefd63d32d227b7ff8c798fe534f61150e |
| Protocol manifest | 43b8b6873c059429a2f4c48cbf375ecff243eb008ddf6aa1dfe5c24f2356af10 |

The first four match the current production-source handoff hashes. The mismatch is thus against that declared source snapshot, not an assumed later change. The full handoff was accessed; tool output truncated its long consumer roster. The audit directly reviewed the canonical owner bodies, package manifests, selected native laws, portable model, fixture replacement vector and Protocol direct mutation type. No exhaustive consumer compiler audit was attempted.
