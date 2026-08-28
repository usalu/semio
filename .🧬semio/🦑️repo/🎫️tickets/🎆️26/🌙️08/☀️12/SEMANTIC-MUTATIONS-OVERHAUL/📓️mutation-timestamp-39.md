# Authored Mutation Timestamp Forwarding — Checkpoint 39

## Contract And Scope

The Store timestamp fixture exposed a real metadata loss: the base Mutation supports an optional authored HybridLogicalTimestamp, but direct MutationKind leaves and CompositeMutationKind payloads had no forwarding hook. Those two traits now expose the same optional authored value. An untimestamped leaf returns None; no current clock is read and no zero timestamp is invented.

Both derive source surfaces now forward leaf timestamp through the transparent aggregate. CompositeMutation forwards its payload hook to MutationKind. The composite expansion was factored into a private pure helper so the actual generated syntax can be tested without invoking the proc-macro runtime.

No base Mutation, Store production, Plugin lifecycle, Kernel receipt, or UiTurnPatches region was changed by this packet. Store's concrete timestamped fixture adoption is separately in progress.

## Evidence

The schema-first neutral contract lives in the derive owner's 🧪️tests/⏱️mutation-timestamp directory. The source/neutral gate initially passed only4of12 checks in 🧪️mutation-timestamp-39/🧫️run-l799R8. After the final source change it passed12of12 in 🧪️mutation-timestamp-39/🧫️run-Lcpv8D, with stable inputs. This gate uses Ajv and source checks, not Rust execution.

The single authorized native gate ran through Bun/Nx and then Cargo:

`cargo test -p semio-framework-os-kernel-dsl-derive --lib --jobs 2 --target-dir <retained demonstrator target> -- --nocapture`

It compiled in1.57s and executed12tests in0.07s:12passed,0failed. Actual syn-AST checks include exhaustive aggregate timestamp delegation and direct composite timestamp delegation. Existing source-authority and sha2 provenance-oracle tests also passed. One pre-existing unused-qualification warning remains.

Exact native evidence is retained in 🧪️mutation-timestamp-39/🧪️native/🧫️run-u5CKXx/🧪️output.log and its 🔣️.json receipt. Inputs were stable throughout. The borrowed target was explicitly released immediately afterward; no broader build ran.

## Stable Native Input Hashes

- Derive component: dd42e0c13ed15e209879461347fb9003589035fded11d73ed89422ad2a5c48ad
- Compiled derive glue: 17448e95b31aab2692a8d3917bec20245647cdd23128b62b16b2bc8a140a8be3
- Derive Cargo manifest: 75ed143dd5d75f405d3ab5e4e4800085d94f6814ec212bece289181db21e70da
- Neutral contract: 1bd3933e08cddf627955673b07e5baa70eacd796db6818b3c81f7d68c3f673ba
- Neutral schema: cdabbc156aea638f883bb59e6ccb0a7df78af239322ef158941787c645068630

This is native derive readiness, not compiled concrete Store/Plugin/Infinite readiness or a monorepo acceptance result. All logs, manufactured test fixtures, caches and prior failed evidence remain retained.
