# glTF Node Name Local Inverse Boundary

## Read-Only Packet

Luna's bounded follow-up inspected the change-node-name operation and its direct/aggregate/schema/test fanout. The broader retained audit is `🧪️gltf-opaque-diff-audit/📓️audit.md`. No glTF source was changed or runtime law proved by this follow-up.

The direct owner is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name`. Its current public mutation wraps either the forward payload or unrestricted `Restore(GltfDiff)`. The latter must be replaced by a concrete operation-local inverse; aggregate `MutationOutcome<GltfDiff>` remains the internal state delta.

## Proposed Leaf Contract

Preserve explicit apply/restore phases, with a restore payload naming `node`, `before`, and `after`. `before` is the name to restore; `after` is the required current name. Both name values are required nullable fields, not omitted-field defaults. The narrow guard permits rejection of a stale inverse without transporting arbitrary document changes. Public node indices should use the existing protobuf uint32 domain with checked conversion at indexing; this remains a coordinated Rust/schema/language change, not an isolated cast.

The canonical touched path is derived as `document/nodes/{node}/name`, never supplied as an arbitrary list. A missing node rejects with the existing target-missing outcome; an equal requested/current name is the existing no-op outcome, with no inverse. Restore checks the current name before writing and rejects a stale value. The implementing lane must inspect the typed snapshot's null/absent normalization: two representations mapped to the same `Option::None` cannot be claimed to remain distinguishable by an `Option<String>` guard.

The aggregate currently owns text framing (`gltf-mutation payload=` plus lowercase JSON hex) and binary framing (format opcode, marker0x47, varint JSON length,64KiB limit). The one-leaf behavior correction must exercise these actual codecs. It does not accept the wider aggregate-codec ownership gap or justify fabricating leaf opcode/tag values; all120 glTF descriptor wire identities remain separately queued for coordinated codec ownership.

## Required Fanout and Tests

Update the direct Rust, TypeScript, GraphQL, protobuf, descriptor and payload schema together, plus the aggregate surfaces and mounted leaf tests. Current dormant contract/scenario Rust files and references to old triads must not count as executable coverage. The coordinator must review actual aggregate discriminator/phase wrappers against the union schema rather than relying on the current raw forward-only schema.

Language-neutral laws must cover name-to-name, absent-to-name, name-to-absent, both no-op cases, missing target, stale name, stale absent/current-name mismatch, required-nullable field omission and wrong types, codec round trips and concrete inverse application. Existing glTF oracle catalog rows are not themselves behavior evidence. A third-party glTF/schema oracle and real registered Rust execution remain required.

This is a bounded implementation design queued behind the mandatory metadata contract, not an accepted glTF conversion.

## GLTF-NODE-NAME-24 Release

The metadata schema, lower metadata boundary and public leaf derive are now frozen and independently verified. The coordinator releases this independent one-leaf semantic correction in parallel with the disjoint TXT root integration. Mandatory aggregate/registry enforcement remains open; this release does not claim that foundation or any glTF root is complete.

The executor first establishes the neutral law matrix, actual current red behavior and exact consumer edit map. Its production write ownership is the direct `change-node-name` leaf and only that operation's representation in glTF aggregate TypeScript, GraphQL, Protobuf and JSON Schema files, its exact mounted source reference, affected current scenario/oracle records and its source-law tests. Additional production consumers or shared mutation-support behavior must be reported before editing. No other glTF leaf, TXT source, shared traits, derive, registry, root script or taxonomy is assigned.

The leaf's public operation must no longer accept `GltfDiff` as a restore payload. The bounded restore payload carries only node identity, required-nullable `before` and `after` names; its touched path is derived, and stale/current-value rejection is explicit. Verify the existing required-nullable serde behavior rather than allowing omitted fields to become implicit null. Use the protobuf uint32 node domain and a checked platform-index conversion. Preserve no-op/missing-target outcome classifications and prove no inverse is emitted for a rejected/no-op operation. A restore's own inverse is concrete and equally narrow.

Complete this leaf's canonical physical primaries and current taxonomy payload-schema destination together, with actual metadata derive, full descriptor and provenance tests. No compatibility path, unrestricted diff, arbitrary path list or hidden snapshot carrier remains in this public leaf. Preserve its semantic kind and currently declared wire ownership; do not invent leaf opcode/tag identities while the glTF aggregate still owns generic framing.

All five required language/schema representations must describe the actual apply/restore envelope, not merely the forward payload. Use actual aggregate text/binary codecs in law tests and an existing third-party schema/glTF oracle. Retain all artifacts inside this ticket. Root alone runs Cargo after both STDIO source lanes report stable inputs; the executor uses scoped Bun/Nx plus genuine-artifact standalone compiler probes. A runtime/compiler blocker must be diagnosed, not replaced with a lookalike protocol or weakened assertion. No full glTF/root/monorepo acceptance is implied.
