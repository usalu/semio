# Mutation Source Roster Roles

## Scope

The public `MutationTaxonomySourceRecord` in [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts:20618) already declares four roles: `source`, `assignment-ledger`, `taxonomy-schema`, and `mutation-descriptor-schema`. The current source-index construction emits the latter two at [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts:20831). This packet changes only the closed v2 inventory schema role enum and adds a neutral regression fixture.

## Inputs

- [v2 inventory schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🛂️schema/🔣️inventory.json) pre-fix SHA-256: `f29ff0d9fcd179110d41249f634a9c4aee9240d0fd9d153b67a4702a9e9accee`.
- New neutral fixture: [vectors](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-roster-roles/🔣️.json) and its closed [schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-roster-roles/🧬️schema/🔣️.json).
- New Ajv/TypeScript regression: [🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-roster-roles/🟦️.ts).

The test compiles the actual draft-07 v2 output schema with Ajv, checks the actual public TypeScript literal union via the TypeScript AST, validates a complete minimal v2 envelope containing every emitted role, and rejects an unknown role and an extra source-roster field.

## Retained Pre-Fix Red

Command:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧪️source-roster-roles/🟦️.ts'
```

Exit `1`. The neutral vectors and current TypeScript union passed. The actual schema assertion failed because the roster entries at indexes 2 and 3 had roles `taxonomy-schema` and `mutation-descriptor-schema`, while its enum allowed only `source` and `assignment-ledger`.

## Repair and Green

The sole schema edit expands the existing closed role enum with the two already-emitted roles. It does not change the envelope version, fields, closure, or unknown-role behavior.

The same scoped Nx command then exited `0`: 3 tests passed, 0 failed, 6 assertions.

Final SHA-256 values:

- v2 inventory schema: `52af0f5ce3de04befb7e22a8d4da840c5f0719b4193c92759d9df3d274e12874`
- regression test: `03d93dab90e3a7ac77da06de550105658310072837425c893b79eaa0ef9c2094`
- neutral schema: `e59cec0d7f1fc079b1aa62145726451eee73be258137e005cb181fcfc990a9af`
- neutral vectors: `2619fa8ac3a2ce5e55d22c8aa35e4538b36a82227b6ded8037b777bd7115e195`
- public root script observed during both runs: `fdb34f8e4a9d1696915dc18d804876ed80a1f46c6d09d365f92b24914c5a991d`

This is schema/reference evidence only. No inventory collection, source API, native compiler, or runtime execution was run.
