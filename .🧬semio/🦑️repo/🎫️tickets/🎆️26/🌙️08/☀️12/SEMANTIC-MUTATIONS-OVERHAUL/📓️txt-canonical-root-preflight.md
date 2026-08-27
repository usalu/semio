# TXT Canonical Root Preflight

## Scope and Hold

This preflight is the schema-first preparation for `TXT-CANONICAL-ROOT-23`. It makes no production-source edit. The protocol writer retains the root binary protocol and its framing tests until the coordinator releases the explicit source hold.

## Neutral Matrix

The machine-readable five-operation contract is [neutral-matrix.json](../🧪️txt-canonical-root/🔣️neutral-matrix.json). It fixes the semantic and wire identities while selecting the distinct direct owners required by the controlling contract.

| Semantic kind | Direct owner | Variant | Text opcode | Binary tag |
| --- | --- | --- | --- | --- |
| `set-trailing-newline` | `↩️set-trailing-newline` | `SetTrailingNewline` | `set-trailing-newline` | 1 |
| `set-line-ending` | `🔚️set-line-ending` | `SetLineEnding` | `set-line-ending` | 2 |
| `insert-line` | `📥️insert-line` | `InsertLine` | `insert-line` | 3 |
| `remove-line` | `🗑️remove-line` | `RemoveLine` | `remove-line` | 4 |
| `set-line` | `✏️set-line` | `SetLine` | `set-line` | 5 |

The matrix gate derives ordinary primary filenames from the current taxonomy (`🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`, `📖️.grammar.semio`, and `📡️.protocol.semio`), including the payload location `🧬️schema/🔣️.json` from `mutationPayloadSchemaLocation`.

## Red Evidence

`📜️script.ts matrix` passed and established five unique semantic kinds, direct owners, emoji identities, variants, text opcodes, and ordered binary tags. `📜️script.ts red` completed successfully as a deliberate fail-closed red check: the current layout has **42** canonical-layout failures.

The failures prove the current component-stemmed aggregate primaries, component-stemmed leaf TypeScript/GraphQL/Protobuf/text/binary primaries, old standalone payload schemas, duplicate-pencil owner names, and absent aggregate-local public leaf mounts. The red check records no generated output file.

Command used:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️txt-canonical-root/📜️script.ts' matrix
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️txt-canonical-root/📜️script.ts' red
```

## Exact Post-Release Patch Map

1. In the TXT mutation tree only, rename aggregate primaries from `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, and `🛰️component.proto` to their taxonomy-derived kind-only names. Rename the generic text grammar and binary protocol primaries to `📝️text/📖️.grammar.semio` and `💾️binary/📡️.protocol.semio` without changing their payload or framing semantics.
2. Rename `✏️set-trailing-newline` to `↩️set-trailing-newline` and `✏️set-line-ending` to `🔚️set-line-ending`. Rename every direct leaf TypeScript, GraphQL, and Protobuf primary from `component` stems to kind-only names. Keep each direct leaf's existing `🦀️.rs` as its Rust primary, and rename its text/binary Rust child primary to `🦀️.rs`.
3. Move each `🔣️payload.schema.json` to that leaf's `🧬️schema/🔣️.json`. Update the direct descriptor's `owner`, `emoji`, and `payloadSchema` fields, and update aggregate JSON Schema `$ref` entries. No old payload filename, owner directory, or compatibility alias remains.
4. Update aggregate TypeScript imports, Protobuf imports, Rust aggregate local direct-leaf mounts/reexports, and all mutation-tree include or path references to the new physical paths. The aggregate stays transparent: it only mounts/reexports the five leaf implementations, preserves the current enum variants, derive, generic `apply_txt_mutation`, and no per-kind application or inverse switch.
5. Change only the TXT mutation surface `include_str!` paths in `🧬️schema/🦀️component.rs` to the aggregate kind-only language primaries. In the TXT mutation block of STDIO glue, mount aggregate `🦀️.rs`, remove the five sibling direct-leaf mounts, retain generic binary/text mounts at their boundary with their kind-only primary paths, and rely on the aggregate's public local mounts.
6. Update the TXT subset oracle catalog's `sourceMutationDirectoryName` and `mutationDirectoryName` identities for the two renamed owners, plus any directly affected descriptor/provenance assertions and current TXT schema tests in the owned mutation tree. Strengthen the aggregate roster test from a count-only assertion to the exact five semantic-kind/variant/descriptor correspondence.
7. Refresh the ticket-local actual-source runtime harness for the aggregate-local mount topology, run its path guards and five-operation source compile/runtime proof against the fresh `🧪️derive-contract-target/debug/deps` artifacts, then run AST-backed TypeScript, GraphQL, Protobuf, and JSON Schema parity plus real text/binary round trips with retained third-party-oracle evidence.

The only write targets in this map are the contract's TXT mutation tree, its permitted TXT schema-owner include references, its permitted STDIO glue module block, its TXT subset oracle/catalog identity records, and new ticket-local evidence. Any additional consumer will be reported to the coordinator before editing.
