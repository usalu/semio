# Registry follow missed joined owner failures

## Cause of the two `BUILD_OWNER_FAILED` runs (10:04 and 13:24)
Another agent registered new native crates in the Cargo workspace before their directory or `build-owners.json` row was complete (`derived-replay` at 10:04, `window-resource` at 13:24). The owner inference of `framework-os-node-graph-rs` and `framework-os-shell-rs` then failed with `Canonical native dependency owner required: <root>`. Both crates are complete now and registered.

## Root defect fixed
`withOwnerRegistryCustodyV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧭️registry-follow/🟦️.ts`) only followed a raw fault whose own code and message opened a declared registry fault. The admission closes with a joined `BUILD_OWNER_FAILED` named failure, whose code and message never match, so the follow never ran and the load died although the registry could be repaired by reconciling the named roots.

Fix: `registryFaultTextV1` reads the root text from the owner reasons of a named failure when every owner failed on a registry fault (a failure with any real owner fault is never followed); the custody and `settleOwnerRegistryV1` use it. `registryFaultReasonV1` is exported from the owner-isolation module and reused there.

Test: `owner-isolation/🧪️tests/🟦️.ts`, "a joined failure whose owners all failed on a pending native root is followed ..." (passes with bun directly; the mixed case is not followed).

The running host (`host-0003b7003034`) loaded the old code; the fix applies to the next service host.
