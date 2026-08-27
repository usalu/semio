# Mandatory Metadata Trait Compiler Preflight

## Executed Prototype

The coordinator ran five neutral compiler cases through registered Nx execution: `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun <ticket>/🧪️mandatory-metadata-trait-preflight/📜️script.ts`. All five matched expectations; transcript `🧪️mandatory-metadata-trait-preflight/🧪️root.log`, artifacts `🧪️mandatory-metadata-trait-preflight/🧫️run-uDfWdo`.

- A required static descriptor roster and per-instance descriptor method compile for generic aggregate variants, including a borrowed non-static payload. Both generic instantiations executed and returned the expected two kinds.
- Omitting either required item independently produces E0046.
- A supertrait's constant cannot be addressed as `<Payload as Kind>::DESCRIPTOR` when it is declared only on `Leaf`; Rust reports E0576. Derive emission must qualify the declaring metadata trait. Introducing a second independent descriptor constant solely for this spelling would create avoidable duplicate authority.
- A public manually implemented provenance trait can contain an arbitrary claimed owner and still compile. The runtime printed that false claim. A public marker/constant is not an unforgeable source proof.

## Scope

These are minimal trait-language prototypes, not the actual production trait or metadata implementation. They prove the compiler constraints used for the pending API freeze, not production ownership or successful integration. The full metadata type and existing mutation behavior methods were deliberately not copied into the prototype. Actual production compiler, source-policy, derive and runtime gates remain required.

The current proposed shape puts the full metadata type and source provenance in the lower replication mutation contract. Base `Mutation<P>` requires a descriptor roster and instance selection without defaults. Leaf kinds require the metadata trait, while the aggregate derives mechanically emit dispatch and full rosters. All manual source-proof implementations require independent policy rejection. The read-only generic/direct-implementation fanout audit remains the final freeze input.
