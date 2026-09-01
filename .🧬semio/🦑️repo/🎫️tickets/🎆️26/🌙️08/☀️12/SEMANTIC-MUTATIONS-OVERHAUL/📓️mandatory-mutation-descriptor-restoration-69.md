# Mandatory Mutation Descriptor Restoration 69

## Scope and Current Defect

This is a bounded read-only proposal for [the lower `Mutation` contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:105). Its current `DESCRIPTORS` associated constant defaults to `&[]`, and `descriptor(&self)` defaults to [`UNDECLARED_MUTATION_LEAF`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:229). The sentinel is a complete-looking `MutationLeafDescriptor` with `schema_version: 0`, `owner: "undeclared"`, empty outcomes/surfaces, and non-invertible/none metadata. It is rejected by the existing fourteen-field validator, but only after an implementation has been allowed to compile without declaring metadata.

The exact sentinel symbol has no external Rust consumer in the bounded source query; both the trait defaults and sentinel definition are in this one protocol component. The current `&[]` form is additionally written explicitly by three uninhabited Plugin test mutation enums and one children-fixture enum. Those are fan-out evidence, not authorization to keep an empty compatible implementation.

## Existing Mandatory Evidence

- [The language-neutral descriptor schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json) requires all fourteen fields and a `rust` surface.
- [The lower validator](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:290) rejects schema version zero, invalid owners, empty outcomes, and empty required surfaces. [Roster validation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:327) validates direct-child ownership and uniqueness, but currently permits an empty slice.
- `MutationLeaf` already has required by-value `DESCRIPTOR` and provenance constants at [the lower contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:650).
- [The `MutationLeaf` derive](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:577) parses the direct JSON descriptor and emits that real descriptor. [The `Mutations` derive](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:1790) emits `Mutation::DESCRIPTORS`, `descriptor`, direct-leaf source checks, and one roster validation. It needs no sentinel.
- Existing [source/provenance vectors](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧬️mutation-leaf-source-contract/🧫️fixtures/🔣️.json) test an individual real leaf but not an omitted base-trait declaration or an empty inhabited aggregate roster.

The earlier [mandatory contract record](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️mandatory-leaf-descriptor-contract.md) already freezes the intended direction: no default, no optional metadata, no sentinel descriptor, and `MutationKind: MutationLeaf` for concrete leaf behavior. This packet does not reopen schema vocabulary or infer metadata from type/variant names.

## Proposed Breaking Contract

1. Remove both default bodies from `Mutation<P>`: every implementation must write `DESCRIPTORS` and `descriptor(&self)`.
2. Delete `UNDECLARED_MUTATION_LEAF`; no invalid descriptor remains constructible as a fallback.
3. Replace the slice-valued `DESCRIPTORS` associated constant with one lower-contract nonempty roster value, e.g. `MutationDescriptorRoster { head: MutationLeafDescriptor, tail: &'static [MutationLeafDescriptor] }`, exposing `as_slice()` for registry/derive consumers. The shape makes an empty roster unrepresentable, rather than merely rejected by a late scan.
4. Preserve exact descriptor order through `head` then `tail`; run the existing source/owner/uniqueness validator over `as_slice()` in the aggregate derive. `descriptor(&self)` returns one member of this same roster, not a duplicate or inferred descriptor.
5. Keep the leaf contract by-value: every concrete `MutationKind` remains a `MutationLeaf` with exactly one `DESCRIPTOR`; aggregate derives use those constants. Manual implementations must supply equivalent real descriptors and provenance, then are subject to the same source policy.

This is intentionally a breaking migration. It does not add a default, an empty/fake roster, a compatibility trait, or a registry-side patch-up.

### Uninhabited Absence Types

`NoConfigMutation`, `NoPresenceMutation`, `NoTransientMutation`, and the children fixture use exhaustive `match *self {}` descriptors and explicit empty descriptor arrays. They are not inhabited mutation leaves, but they still satisfy generic `Mutation` bounds today. The nonempty roster contract cannot retain them unchanged. The follow-up owner must choose an explicit absence-type boundary in those generic APIs or remove their `Mutation` implementations; it must not keep `&[]` or fabricate a sentinel. This packet does not modify those Plugin/testkit owners.

## Language-Neutral Negative Fixture Proposal

Add a new closed fixture beside `mutation-leaf-source-contract`, separate from individual provenance vectors. It models trait declarations rather than descriptors:

```text
case fields
  implementsMutation: boolean
  descriptors: omitted | nonempty-real | explicit-empty
  descriptorMethod: omitted | real-member | invalid-sentinel
  inhabited: boolean
  expected: accept | reject-missing-associated-item | reject-empty-roster | reject-sentinel
```

Required vectors:

1. An inhabited aggregate with one real fourteen-field descriptor and matching selector: accept.
2. Omitted `DESCRIPTORS`: reject missing associated item.
3. Omitted `descriptor`: reject missing associated item.
4. Explicit empty roster for an inhabited enum: reject before registry construction.
5. A sentinel-shaped schema-version-zero descriptor: reject independent of roster position.
6. Descriptor selector not contained in the declared roster: reject.
7. Two real descriptors with a duplicate semantic kind: reject through existing roster rules.
8. An uninhabited absence mutation: **not accepted as a `Mutation` fixture** until its separate generic-bound decision is implemented; this keeps the decision visible rather than smuggling an empty exemption into the contract.

The fixture schema must require all case fields, be closed, and use the existing descriptor schema for each `nonempty-real` member. Ajv validates fixture closure/descriptor validity; that is the language-neutral oracle, not a substitute for Rust trait semantics.

## Independent Rust Oracles

Two independent checks are needed after a source change:

1. A test-only `syn` AST oracle parses the actual lower trait and asserts: the `DESCRIPTORS` associated item has no default expression, `descriptor` has no default block, and no `UNDECLARED_MUTATION_LEAF` item exists. It also verifies the actual nonempty roster type is used by the trait.
2. A minimal real `rustc` fixture compile proves behavior: an otherwise valid inhabited `impl Mutation` omitting either required item fails with the compiler's missing-associated-item diagnostic; an explicit empty roster cannot type-check against the nonempty roster; a real derived aggregate compiles. This is a compiler oracle, not a copied parser. It must be scheduled by the root compiler lane, not run in this audit.

The derive's existing expansion tests remain necessary to show its emitted aggregate uses the exact roster API and `MutationLeaf::DESCRIPTOR`; they are not independent evidence for handwritten implementations.

## Exact Write Boundary for a Future Packet

Authorized only after coordinator assignment:

- lower protocol mutation contract and its direct unit tests;
- OS public command facade reexports if the public roster type changes;
- both `Mutations` derive mirrors and their expansion fixtures;
- the new language-neutral test fixture/controller and isolated compiler fixture;
- immediate explicit empty-implementation consumers only after the separate uninhabited-bound decision.

Excluded: descriptor JSON vocabulary, `MutationLeaf` provenance model, artifact mutation owners, Plugin runtime/lifecycle, registry semantics other than adapting the roster API, TypeScript declared-surface work, and any broad inventory/census.

## Captured Inputs

- lower contract: `e5f2f9ce74cc305bcbc23c0d99ab70cc2af54cf299a561f7910d56a7dbbd8385`
- derive source authority: `17448e95b31aab2692a8d3917bec20245647cdd23128b62b16b2bc8a140a8be3`
- descriptor schema: `db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee`
- existing provenance fixture schema / vectors: `8cb0544bc6c83757dd0f9706e8b0138f42a4f901dbde88cd22f31ecad863810f` / `3bb385214d54bcfed2084f0b9c5b2c100d69406e1d57eab179e7e332c8a2eee2`

No production/test source, schema, controller, compiler command, or Compose path was changed or run in this audit.

## Correction: Accepted Minimal Restoration Boundary

The preceding `MutationDescriptorRoster` head/tail proposal is rejected and superseded. It would change public API shape, does not itself provide contiguous slice storage, and is outside this packet. The current slice API remains exactly as it is.

The accepted trait change is only:

```rust
const DESCRIPTORS: &'static [MutationLeafDescriptor];
fn descriptor(&self) -> &'static MutationLeafDescriptor;
```

Both associated items have no default. Remove `UNDECLARED_MUTATION_LEAF` and all compatibility/migration docstrings referring to the fallback. No new roster type, derive change, facade change, registry change, or source-provenance change belongs here.

An explicit empty roster is valid for a genuinely uninhabited enum whose `descriptor` is `match *self {}`: `NoConfigMutation`, `NoPresenceMutation`, `NoTransientMutation`, and the children fixture stay out of this packet. An empty **inhabited** aggregate is a separate structural-policy breach; it is not solved by changing the base trait's slice type.

### Exact Desired-Law Test Proposal

Create one ticket-only closed JSON schema/vector pair and one test controller later, without defining a copied/fake `Mutation` trait:

| Case | Actual subject | Desired result |
| --- | --- | --- |
| missing-descriptors | `impl Mutation` fixture against the actual compiled lower contract | compiler reports missing `DESCRIPTORS` |
| missing-descriptor | same | compiler reports missing `descriptor` |
| missing-both | same | compiler reports both required associated items |
| explicit-real | same, using a real fourteen-field descriptor and selector | compiles |
| uninhabited-empty | real zero-variant enum with explicit `&[]` and `match *self {}` | compiles; no concrete leaf claimed |
| inhabited-empty | real one-variant aggregate with explicit `&[]` | structural policy reports an empty-roster breach |
| sentinel-absent | actual lower trait component parsed directly | no sentinel item or fallback body exists |

The neutral fixture records those subject modes and expected stable categories, is closed, and references the existing descriptor schema for `explicit-real`. Ajv validates fixture closure; it does not emulate Rust trait checking.

The controller must extract the actual lower component source and parse it with the already present test-only `syn` AST dependency. It asserts one `Mutation` trait with required `DESCRIPTORS` and `descriptor` items that have no default expression/body, and no `UNDECLARED_MUTATION_LEAF` item. It must not embed a second trait declaration. The deferred compiler oracle writes only implementations against the compiled actual contract and proves the three missing-item failures plus explicit-real/uninhabited success. No compiler command was run in this packet.

The current source is observed to violate all three future extraction assertions: `Mutation::DESCRIPTORS` has `= &[]`, `Mutation::descriptor` has a body, and the sentinel item exists. No desired-law controller was created or run here, so this is not an executed RED receipt. The historical source/provenance vectors remain separate; they do not establish this trait requirement.
