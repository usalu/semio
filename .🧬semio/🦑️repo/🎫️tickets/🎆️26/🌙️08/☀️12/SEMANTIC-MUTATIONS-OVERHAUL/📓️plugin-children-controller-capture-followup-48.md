# Plugin Children Controller Capture Follow-up 48

## Scope

Only `🧪️plugin-children-fixture-44` ticket inputs/controller were changed. The Plugin Children mutation source, native test source, schema, and domain case fixture remain frozen. No Cargo, rustc, or native test execution occurred.

## Test-first capture evidence

The new ticket-only capture probe schema initially required `schemaVersion`, while the initial probe fixture deliberately omitted it. The controller retained a RED at [run-iod2O0](./🧪️plugin-children-fixture-44/🧫️run-iod2O0/📓️result.md): `77/78`, only `capture probes: schema valid` failed.

After declaring `schemaVersion: 1`, the scoped Bun/Nx controller passed `78/78` at [run-RIWf6r](./🧪️plugin-children-fixture-44/🧫️run-RIWf6r/📓️result.md).

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-children-fixture-44/📜️script.ts
```

## Added controller guarantees

- It derives the repository root from `import.meta.url`, rejects noncanonical/symlinked workspace roots, and lstat-checks every ancestor and final input before reading.
- It rejects lexical absolute, dot, parent, and ASCII-case `compose` segments through ticket-only virtual probe cases. Those probes never materialize or traverse a real compose path.
- It captures the controller before work; first-hashes the actual main consumer, canonical mutation, native tests, empty schema, domain cases, baseline/vectors, and probe files; then canonical-nofollow rereads every captured input before the receipt.
- It asserts the exact inline private-type mount, the private unit snapshot/identity diff, `ChildrenTestConstruction` Snapshot/Mutation/Diff joins, and its existing empty text/binary snapshot construction behavior.
- It reads the native test body rather than only four names, asserting loops over every JSON/text/binary fixture category.

The GREEN receipt includes stable first/final hashes for all ten consumed inputs. Native execution remains pending: `78/78` is source/schema/reference/capture evidence only.
