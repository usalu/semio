# Mandatory Mutation Descriptor Desired Law 70

## Boundary

The ticket-only packet is limited to the base trait at
`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`.
It requires a declaration-only `DESCRIPTORS` associated constant, a
declaration-only `descriptor` associated method, and no
`UNDECLARED_MUTATION_LEAF` sentinel. It neither changes the trait nor treats
an explicitly uninhabited mutation enum as an error. An inhabited empty
aggregate remains a separate structural-policy case.

## Artifacts

- `🧪️mandatory-mutation-descriptor-70/🧬️schema/🔣️.json` is a closed
  seven-case neutral-vector schema, SHA-256
  `2a19435b5772b67024a9a568de08a02584eb5983c2004c4026be7c7c82cdbd40`.
- `🧪️mandatory-mutation-descriptor-70/🔣️.json` carries the required
  missing-item, explicit implementation, uninhabited-empty, inhabited-empty,
  and sentinel cases, SHA-256
  `8f47018b76893614cef186d00d4748e6fe98d1a71de0086dfadd885b17a57eb0`.
- `🧪️mandatory-mutation-descriptor-70/📜️script.ts` uses the repository's
  existing Rust tokenizer and pairer as an owned source inspector only,
  SHA-256 `59caf20d6591032df213c8c8cfffea452fc9c16055efc141b45329097d8aec17`.

The controller uses TypeScript only to load the existing TypeScript source
that implements that repository-owned Rust tokenizer. It does not parse Rust
with TypeScript and makes no compiler/native claim. A bounded package-file
check found no installed independent JavaScript Rust parser
(`tree-sitter-rust`, `rust-parser`, or `rust-analyzer`). The independent oracle
remains `syn` in the existing derive test crate, to be mounted as a real
ticket test body and run only after an explicit cfg(test)/Cargo-slot decision.

## Desired-Law RED

Command:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mandatory-mutation-descriptor-70/📜️script.ts'
```

The retained run is
`🧪️mandatory-mutation-descriptor-70/🧫️runs/🧫️run-ca3767e3-3ce4-4274-a94d-a68b881a8bca`.
It exited 1 as intended. Ajv accepted all seven neutral cases, the five input
hashes were unchanged before/after, and the actual trait source reported:

```text
descriptorsHasInitializer: true
descriptorHasDefaultBody: true
sentinelPresent: true
```

`🔣️result.json` and `🔣️failure.json` retain the complete evidence. The
controller executed six source/schema assertions and records five deferred
Rust-compiler fixture cases. No Rust compiler, `syn`, or native test was run.

## Deferred Native Oracle

After the base-trait source change is approved, a ticket-local `syn` test
should parse the actual trait source and compile the real missing-associated-
item fixtures in the existing derive test crate. It must prove missing either
required item is rejected, an explicit implementation is accepted, the three
known explicit uninhabited enums remain valid, and no sentinel is present. That future
mount is deliberately not a copied or stub trait and is outside this packet.

## Superseding Declaration-Facts Contract

The original seven rows remain retained as deferred compiler/structural labels;
they are not behavioral vector execution. The superseding packet is
`🧪️mandatory-mutation-descriptor-70/🧪️declaration-facts`:

- `🧬️schema/🔣️.json` is closed and binds each exact
  `id`/`subject`/`category`/facts/acceptance/violation combination.
- `🔣️.json` exercises the accepted declaration-only trait facts and each
  independently rejected fallback fact.
- `📜️script.ts` evaluates every vector through one `violationsFor` function,
  proves duplicate, missing-subject, and mismatched-category inputs fail Ajv,
  then evaluates the actual source through the repository-owned Rust tokenizer.

Its retained desired-law RED is
`🧪️mandatory-mutation-descriptor-70/🧪️declaration-facts/🧫️runs/🧫️run-c8f1c0e9-3dd1-4047-8707-ee3ba6c38088/🔣️failure.json`.
It executed 14 checks before the actual desired-contract failure, retained the
three exact facts, and captured stable source hashes. It makes no native claim.

The earlier `🧪️mandatory-mutation-descriptor-70/🦀️syn-test.rs` is retained
as the first unmounted sketch. It is superseded by
`🧪️mandatory-mutation-descriptor-70/🧪️declaration-facts/🦀️syn-test.rs`,
SHA-256 `2bc2741f078ebeacd894dbf6e69fc3c292ba38d46881fbbeda1ec8858d7c30ae`.
That new unmounted `syn` body consumes adjacent `🔣️.json`, serializes actual
and expected facts with exact camel-case keys before asserting, and requires
exactly one public top-level `Mutation` trait plus one each named associated
item. It uses the derive crate's existing `syn`, `serde`, and `serde_json`.

The proposed canonical child leaf is
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mandatory-mutation-descriptor/🦀️.rs`,
with adjacent `🔣️.json` and `🧬️schema/🔣️.json`. The exact future include in
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
is:

```rust
#[cfg(test)]
#[path = "../../🧪️tests/🧬️mandatory-mutation-descriptor/🦀️.rs"]
mod mandatory_mutation_descriptor_tests;
```

From that leaf, its `include_str!` target resolves exactly to
`../../../../../../../🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`.
The unmounted body parses the real lower trait with `syn`, emits the same three
fact names, and asserts the declaration-only desired state. Missing-
implementation compiler fixtures remain explicitly unrun pending that mount and
an assigned Cargo slot.
