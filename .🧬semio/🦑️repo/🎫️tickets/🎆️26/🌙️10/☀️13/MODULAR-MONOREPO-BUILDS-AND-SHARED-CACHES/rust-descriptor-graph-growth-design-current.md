# Rust Descriptor Graph Growth Design

Pending implementation. This Source trace identifies repeated work; no Native compilation, actual warm-build measurement or physical descriptor-cache pressure result is claimed. Keep all existing Original/private Store/code/descriptor-reader ownership and Native/global SDK/publication holds.

## Verified Current Work

The actual selected producer materializes a selected Cargo graph, then loops over every materialized unit. For each one it calls selectedRustDescriptorSliceV1(materialized.units, selectedUnit.id), publishes that unit's descriptor, and invokes rustUnitNxProjectsV1 on the full selected predecessor slice before retaining only the selected project's entry. The slice implementation constructs a new byId Map from every supplied unit on every call, then traverses the selected predecessor closure and JSON-encodes that closure. The Nx projection replans the supplied slice.

For N independent units, the repeated map construction alone visits N squared input rows, even though every descriptor closure contains only one unit. This follows directly from the nested producer loop and slice constructor, not a timing extrapolation. Shared predecessor payloads and planning are also repeated for every consumer; their actual extent depends on DAG edges/depth and existing byte/shape caps. Do not claim unrestricted quadratic disk growth or an observed overflow from this Source trace.

## Intended End State

Prepare one owned immutable neutral unit graph and its semantic/context plans per actual selected materialization, independent of Nx. Construct graph indexes once. Project an individual already-owned plan into its Nx target without replanning its ancestor slice. Domain DATA ownership must continue to grant no Native/provider/Store/loan authority. The actual private Store/code admission remains before producer effects.

Use normalized content-addressed per-unit descriptor nodes with direct predecessor references, instead of republishing complete predecessor Unit arrays for every selected node. Each node must preserve the full unit/context/semantic facts and exact direct predecessor identities; readers must authenticate canonical filenames/byte hashes/closed schemas and rederive the plan/context relationships. Retain genuine Original-issued descriptor/Source leases, no-follow same-FD reads, per-node and cumulative byte/count/depth bounds, cancellation, complete accepted drainage and first-failure semantics through actual compiler consumers. Do not promote JSON copies to private Native authority.

Per-unit Nx task identity must follow only that unit's Source/effective configuration/code and predecessor artifacts. A workspace-wide descriptor filename, graph hash or environment value in every task command can invalidate unchanged siblings; do not trade the current correct sibling reuse law for cheaper graph preparation. Direct receipt dependencies and immutable bounded outputs remain explicit. Actual publication/restore/charge/retirement guards remain intact.

Design graph/request caps for the documented growth target, with bounded preparation and streaming/scoped readers. Do not introduce a new whole-graph limit that rejects graphs accepted by per-unit closures without auditing actual production limits first. Avoid global stale digest maps or indefinitely retained Source objects. Catalogue/descriptor storage must remain inside genuine fixed partitions and have a complete retained-reader/retirement strategy.

This is a greenfield wire/API change: update all actual readers, producers, CLI consumers, schemas and handwritten fixtures together. Do not leave a legacy descriptor decoder, alias, migration script, fallback or adapter that hides a stale caller. Retain all current API/frontier and physical qualification holds until genuine compilation is available.

## Test-First Acceptance

- Author language-neutral independent, chain, diamond, fan-in and disconnected DAG vectors. Use independent SQLite recursive closure and independent JSON/hash oracles to validate exact selected ancestry, node identities, equivalent permutations and affected consumers.
- Establish the existing repeated-index causal behavior with a bounded Source experiment or actual Source instrumentation; test the final implementation's prepared index once per graph and per-unit projection without replanning. Avoid asserting only implementation text or making up a Native hasher.
- Check actual encoded descriptor storage sums and unique nodes with neutral graph vectors; retained node-reference output must not duplicate transitive Unit payloads. Include exact limits, malformed refs, missing/duplicate/cyclic/deep graphs, scalar Unicode, mutable/shallow-frozen/callback-retained DATA and altered hash/filename cases.
- Preserve and extend the existing effective compiler code/receipt/input/output/edge/environment and unchanged-sibling laws. Change one node, predecessor artifact, compiler-code pair, effective profile/cfg or irrelevant planning-only fact and derive the expected affected set independently.
- Exercise genuine ordinary descriptor files/FDs/SQLite/streams under the existing issued Source harness, retaining byte/namespace checks before EOF and delivery, failed Original children as terminal DATA, copied/retired token refusal and accepted-operation settlement. No successful Native/provider/Store substitutes.
- Run all applicable existing launch-registered Source and strict checks in prototype/release with unchanged timing plans and disabled runtime caches. Separately qualify real Nx/artifact/compiler/storage/cache behavior only when genuine private owners and independent-review gates permit it.

## Other Open Work

Raw framework inventory is in rust-source-growth-audit-current.md (file bytes are not active crate size). Core metadata retirement and sustainable pressure remain open, as do actual compiler/API-frontier correctness, CPU utilization and zero-touch Windows/macOS/Linux behavior. A fixed managed-cache envelope alone does not bound Source/Git/tickets/historical unmanaged bytes or complete the broad goal.
