# GIS Config Direct Neutral Gate

## Controller and command

The durable ticket-only controller is
[`📜️script.ts`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/📜️script.ts).

It was run with:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/📜️script.ts'
```

The frozen passing source snapshot is retained at
[`🧪️neutral-2026-08-27T18-18-10-817Z.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/🧪️runs/🧪️neutral-2026-08-27T18-18-10-817Z.json): seven descriptors and payload schemas, one aggregate, nine valid envelopes, seven invalid envelopes, 19 independently parsed JSON inputs, and stable before/after SHA-256 input hashes.

## Strengthened controller

The controller now records an assertion count and elapsed time. It snapshots every input before the gate and re-hashes every one afterward, failing on drift. That set includes the controller itself, authoritative descriptor contract, seven leaf Rust/descriptor/payload files, aggregate JSON/Rust, permanent fixture, sparse diff schema and Rust source, mounted config component and direct native test source, direct caller sources, and the adjacent JSON/TypeScript/GraphQL/protobuf/Rust config-schema surfaces.

It now executes every valid fixture payload both as a complete aggregate envelope and as the actual leaf payload with `operation` removed. For every leaf it checks each required field omitted individually and an unknown field rejected. It also compiles and tests the new sparse diff schema, covering identity, ordered steps, map null removals, scalar nullable identity, later-step precedence, and malformed top-level/delta/map values.

`jsonc-parser.modify` plus `applyEdits` is the independent state-update engine. The controller compares it against the neutral sparse model for every representative operation and inverse, all four fixture state cases, absent versus explicit-default map restoration, and a repeated same-field Store reverse-order replay. Comparisons are structural, so object-member order is not treated as semantic behavior.

The strengthened controller passed after the root-declared GIS source freeze. The retained stable result is [`🧪️neutral-2026-08-27T18-18-10-817Z.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/🧪️runs/🧪️neutral-2026-08-27T18-18-10-817Z.json): 548 assertions in 114.775 ms, 19 JSON inputs independently parsed, and identical before/after hashes for every tracked input.

It also proves the frozen seven-field snapshot metadata parity across JSON Schema, TypeScript, GraphQL, and protobuf, and explicitly rejects the retired `selectedIds`, `featureSelectionJson`, `hoverJson`, `selectionMethod`, and `selectionMode` fields in each sidecar.

## Earlier assertions

The controller uses Ajv Draft 2020-12 to compile every actual leaf payload schema and the actual aggregate envelope. It checks the descriptor JSON against the authoritative direct-mutation descriptor schema. `jsonc-parser` separately parses every JSON input and is required to agree byte-for-byte structurally with native `JSON.parse`.

It additionally verifies source provenance without compiling Rust:

- all seven physical leaves exist at their direct owner paths, expose the expected payload struct, canonical full `dsl` keyword, and direct `MutationKind<Gis2dConfig, Gis2dConfigMutation>` implementation;
- descriptor identity, display name, emoji, variant, payload locator, text opcode, binary tag, behavioral metadata, outcomes, and language-surface order match the frozen roster;
- the aggregate references the seven open payload `$defs` in roster order, uses the camelCase `operation` discriminator, and derives both `dsl::Mutations` and `dsl::DslOps`;
- the config component mounts that aggregate and owns the generic text/binary codecs; GIS view/example/locale callers construct each wrapped leaf variant.

Its independent neutral sparse-state model checks each operation's changed field and one-operation inverse, a populated no-op, absent map-entry inversion through `null`, explicit `true`/`1.0` map-entry preservation, sequential independent updates, and the Store convention of replaying stored inverse vectors in reverse order.

## Retained preliminary evidence

Two earlier retained runs are controller-construction failures, not GIS source defects:

- [`🧪️neutral-2026-08-27T18-08-29-434Z.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/🧪️runs/🧪️neutral-2026-08-27T18-08-29-434Z.json) used an incorrect generic title splitter for the intentional `Set LOD Mode` descriptor display name.
- [`🧪️neutral-2026-08-27T18-09-03-115Z.json`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gis-config-direct-41/🧪️runs/🧪️neutral-2026-08-27T18-09-03-115Z.json) exposed Ajv's Unicode-relative-reference normalization requirement in the controller. The controller now asserts the literal repository reference before compiling an in-memory URI-normalized copy; no source schema was changed.

This is a neutral/source gate only. No Cargo, Rust compiler, Rust test, launch, seed, or plugin lifecycle action was run. The root may independently replay the retained command; the recorded hashes make any source drift visible.
