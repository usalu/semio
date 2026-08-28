# Plugin Private Count Fixture Roots 50

## Decision

The five direct counter leaves remain children of the three canonical fixture roots. Each root now owns its private snapshot and diff, fixture app, command grammar, helper, and existing native tests. This is the narrow ownership repair: descendant leaves can read `base.count` without exposing fields, proxy accessors, aliases, or returning mutation logic to `🦀️component.rs`.

The moved roots are:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs`

The old inline `app::testkit::{testkit_tests,transaction_testkit_tests,surface_testkit_tests}` modules were removed from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`. The inline `app` module is explicitly rebased with `#[path = "."]`, and the single canonical fixture mount now lives at `crate::app::mutation_fixture`. This makes both the moved tests and direct leaves descendants of `app`, preserving access to existing app-private test APIs without exposing snapshot/diff fields. No `include!`, virtual path, crate-root legacy mount, or compatibility re-export remains.

## Schema-First Source Gate

The ticket controller is [📜️script.ts](../🧪️plugin-private-count-roots-50/📜️script.ts). It captures regular nofollow inputs, validates all five actual descriptor sidecars with the authoritative descriptor schema through Ajv, and requires the removed-inline/owned-root/direct-descendant topology.

The first feature RED was the old topology:

```text
mainHasLegacyModules=false
rootsOwnPrivateFixtures=false
directMounts=true
```

The initial attempted invocation also exposed a controller workspace-root calculation error before checks ran; that was corrected and is not counted as feature evidence.

The executed green command was:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-private-count-roots-50/📜️script.ts
```

The original `run-mtc06cwa` 8/8 result was scoped structure/schema/parser evidence only: its capture did not yet prove nofollow ancestry or final-read stability.

The hardened final run passed all 10 checks. It performs lexical rejection before filesystem access, rejects symlinked workspace ancestors and final inputs, uses `O_NOFOLLOW` on supported platforms, preserves first hashes without overwrite, then rereads all inputs and rejects drift. The retained result is [🔣️result.json](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0c1yc/🔣️result.json). Its 20 first and final hashes are identical.

## Inputs

The final controller captured 20 inputs, including itself, the app component, parent fixture root, all three fixture roots, all three mutation aggregators, five leaves, five descriptors, and the authoritative schema. Primary source hashes are:

- `component.rs`: `49e052ef1f38628104ace5c03b047b895556dee14108c8ca91b36c21f976b103`
- dummy root: `0ddd5d7b88e026cdcd55b4def9beb4fb30e6d63fd1ecf450b6130c720af53b35`
- transaction root: `c5fbfbd08edd772be5676cb3ead8502f6823b893567629a84b1a6ce301da98b8`
- surface root: `5f1187463355727519cf507e53d08fbf25204b9142bd1f100e56328319d8f643`

The final retained result lists every direct leaf, descriptor sidecar, authoritative schema hash, parser status, and first/final source hash.

## Native Status

Native Rust was not run; Cargo and rustc were out of scope. The move preserved 5 dummy, 10 transaction, and 8 surface async native test functions in their direct owners. The next Plugin native gate should compile and execute those 23 tests together with the five actual direct leaves.
