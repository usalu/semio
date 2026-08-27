# FND-METADATA-CONST-04

## Scope

This packet makes the existing exact fourteen-field static descriptor validation const-evaluable and adds a small explicit-owner roster validator. It does not alter mutation traits, derives, registries, artifact leaves, or metadata registration.

## Result

`MutationLeafDescriptor::validate` and `validate_mutation_leaf_descriptor` are now `const fn`, using allocation-free byte checks for the same scalar schema rules and typed comparisons for static enum slices. `validate_mutation_leaf_descriptor_roster` accepts a normalized repository-relative aggregate mutation-root path ending in `/🧬️mutations`; every descriptor retains its truthful direct-leaf owner and must be exactly one non-dot child of that root. It rejects unrelated, nested, duplicate, traversal, backslash, NUL, absolute, Windows-drive, and malformed-root owners before rejecting duplicate `semanticKind`, non-null `textOpcode`, and non-null `binaryTag` within that root. Repeated `None` identities remain valid; separate mutation roots may use the same wire identities.

The neutral fixture adds twenty positive/negative roster vectors. The retained actual-source `rustc` harness compiles the production descriptor region itself: one valid const assertion succeeds, while invalid schema-version and duplicate-tag assertions fail compilation as intended. This is a foundation type boundary only; mandatory `MutationKind`/derive/registration cutover remains open.

## Evidence

The executable compiler harness is [metadata-const script](🧪️metadata-const/📜️script.ts). It retains generated sources and diagnostics below its ticket-local run directory. The registered test route remains `bun nx run @semio-tech/framework-os-kernel:test -- mutation_leaf_descriptor` and was not invoked.
