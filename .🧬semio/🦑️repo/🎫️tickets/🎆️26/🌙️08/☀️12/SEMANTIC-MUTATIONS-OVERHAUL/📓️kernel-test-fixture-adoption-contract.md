# Kernel Test Fixture Adoption Contract

## Scope and status

This is a read-only adoption plan for the legacy inline fixtures in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`. It specifies the next source-owned fixture tree and the no-state publication correction discovered during the same inventory. No production source, Cargo manifest, mount, or compiler artifact was changed by this packet. No feature test has been run; the observations below are source inspection only.

The fixture move is required because the current `CompositeMutationKind<P, Op>` has `MutationLeaf` as a supertrait, while the inline `AddOp`, `DoubleAdd`, `QuadAdd`, `AddThenNotifyForeign`, and `DerivedDoubleAdd` types have no source-derived descriptor or provenance. The metadata contract therefore cannot be met by preserving them inline. The two identically shaped double-add fixtures collapse into one source-owned, derived composite so metadata cannot preserve a duplicate dummy identity.

## Current command fixture inventory

| Current type | Current role | Definition | Direct consumers in the command test module |
| --- | --- | --- | --- |
| `AddDiff` | `MutationDiff<i64>` and `DiffRegions` test diff | 879–889, 1579–1587 | mutation, inference, outcome, and composite laws |
| `AddOp` | atomic `Mutation<i64>` plus `OpText` | 892–913 | mutation laws, `Edit`, inference, planner, every composite |
| `DoubleAdd` | two-local-step composite | 944–957 | fold/apply/inverse and nested-composite laws |
| `QuadAdd` | composite of two `DoubleAdd` plans | 963–986 | nested flattening and inverse law |
| `AddThenNotifyForeign` | one local step plus foreign-plan fixture | 991–1009 | depth, foreign exclusion, and latest-plan tests |
| `DerivedDoubleAdd` | derive integration proof | 1760–1775 | `derive_composite_mutation_wires_delegating_mutation_kind` |

The affected named tests are `operation_diff_apply_matches_backwards_inverse`, `operation_diff_absorb_accumulates`, `operation_defaults_are_stable`, `op_text_round_trip`, `op_text_parse_error_carries_message`, `operation_meta_serde_round_trip`, `edit_serde_round_trip`, the four inference laws, `command_outcome_default_is_empty`, and the seven composite laws from `fold_plan_diff_equals_sequential_apply` through `derive_composite_mutation_wires_delegating_mutation_kind`.

## Source-owned fixture tree

Create one permanent, mounted test owner; do not retain inline payloads or create a compatibility alias.

```text
🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/
└── 🧪️tests/🧬️mutation-laws/
    ├── 🦀️.rs
    ├── 🧬️mutations/
    │   ├── 🦀️.rs
    │   ├── ➕️add-counter/
    │   │   ├── 🦀️.rs
    │   │   ├── 🔣️.json
    │   │   └── 🧬️schema/🔣️.json
    │   ├── ✌️add-counter-twice/
    │   │   ├── 🦀️.rs
    │   │   ├── 🔣️.json
    │   │   └── 🧬️schema/🔣️.json
    │   ├── 4️⃣add-counter-four-times/
    │   │   ├── 🦀️.rs
    │   │   ├── 🔣️.json
    │   │   └── 🧬️schema/🔣️.json
    │   ├── 🌐️add-counter-then-notify-foreign/
    │   │   ├── 🦀️.rs
    │   │   ├── 🔣️.json
    │   │   └── 🧬️schema/🔣️.json
    └── 🧫️fixtures/🔣️mutation-roster.json
```

The executor must validate each selected emoji against the taxonomy before creation. The semantic stems and Rust type names are frozen by this contract; a rejected emoji changes only that leading emoji, not owner, semantic kind, or aggregate variant.

`🦀️.rs` owns `Counter { value: i64 }`, `CounterDiff { delta: i64 }`, `MutationDiff<Counter>`, `DiffRegions`, and mounts the aggregate. This gives the test payload a visible domain owner instead of using the primitive `i64` as an undocumented pseudo-snapshot. `CounterDiff::apply` adds `delta`; `absorb` adds deltas; `touches` remains `value` for nonzero deltas and empty for zero.

`🧬️mutations/🦀️.rs` is a transparent `CounterMutation` enum derived with `dsl_derive::Mutations`, with exactly the four direct leaf variants in the tree order above. It has `#[mutations(snapshot = super::Counter, diff = super::CounterDiff, schema = "command.test.counter")]`; no payload, behavior, text branch, or descriptor is placed in that aggregate.

Every leaf derives `MutationLeaf` with `#[mutation_leaf(contract = ::protocol)]`, derives serde with denied unknown fields, and implements its behavior locally. No manual `MutationLeaf`, `DESCRIPTOR`, `PROVENANCE`, fabricated workspace token, or copied descriptor is permitted. The derive reads the leaf's real canonical `🔣️.json`, giving the test fixture the same source-authority proof as a product leaf.

## Schema-first roster and payloads

The neutral roster fixture must contain the exact 14 descriptor fields for every row, its descriptor JSON must equal the Rust `MutationLeaf::DESCRIPTOR` serialization, and every payload schema is an additional-properties-false JSON object. The direct aggregate's `Mutation::DESCRIPTORS` order must equal this table order.

| Leaf type and direct owner stem | Payload schema | Semantic descriptor | Metadata classification |
| --- | --- | --- | --- |
| `AddCounter` / `➕️add-counter` | required `delta: integer`, minimum `-9223372036854775808`, maximum `9223372036854775807` | `add`, `counter`, `add-counter`, `AddedCounter` | aggregate `AddCounter`; opcode `add-counter`; no binary tag; explicit-mutation; apply-only; applied; atomic; rust/json-schema/text |
| `AddCounterTwice` / `✌️add-counter-twice` | required `delta: integer`, minimum `-9223372036854775808`, maximum `9223372036854775807` | `add`, `counter`, `add-counter-twice`, `AddedCounterTwice` | aggregate `AddCounterTwice`; no opcode/tag; plan; plan; applied; composite; rust/json-schema |
| `AddCounterFourTimes` / `4️⃣add-counter-four-times` | required `delta: integer`, minimum `-9223372036854775808`, maximum `9223372036854775807` | `add`, `counter`, `add-counter-four-times`, `AddedCounterFourTimes` | aggregate `AddCounterFourTimes`; no opcode/tag; plan; plan; applied; composite; rust/json-schema |
| `AddCounterThenNotifyForeign` / `🌐️add-counter-then-notify-foreign` | required `delta: integer`, minimum `-9223372036854775808`, maximum `9223372036854775807`; required `foreignCount: integer`, minimum `0`, maximum `255` | `add`, `counter`, `add-counter-then-notify-foreign`, `AddedCounterThenNotifiedForeign` | aggregate `AddCounterThenNotifyForeign`; no opcode/tag; plan; plan; applied; composite; rust/json-schema |

The direct-cutover text grammar is `add-counter <integer>` and its descriptor `textOpcode` is `add-counter`. This is an intentional fixture wire cutover with no backward-compatible `add` parser branch. No current fixture has a binary operation codec, so every `binaryTag` is `null` and `binary` is absent from required surfaces.

`AddCounterTwice` derives both `MutationLeaf` and `CompositeMutation`, implements `CompositeMutationKind<Counter, CounterMutation>`, and calls `AddCounter` twice. The existing manual-plan and derive-delegation laws both use this same real type. `AddCounterFourTimes` calls `AddCounterTwice` twice against the same planner. `AddCounterThenNotifyForeign` calls `AddCounter` once and retains the existing explicit `ForeignStep` fixture loop. The aggregate-derived atomic registration function registers all four variants in one batch.

## Exact command test rewiring

Add one sibling test-module mount next to the existing registry fixture:

```rust
#[cfg(test)]
#[path = "🧪️tests/🧬️mutation-laws/🦀️.rs"]
mod mutation_laws_fixture;
```

The existing inline `tests` module imports the named public fixture items. Remove its old fixture and composite regions completely. Rewrite uses as follows:

| Old use | New use |
| --- | --- |
| `i64` snapshot and arithmetic expectations | `Counter { value: ... }` and explicit `Counter` expectations |
| `AddDiff` | `CounterDiff` |
| `AddOp` | direct `AddCounter` for direct-text laws; `CounterMutation::from(AddCounter { ... })` where the operation envelope/plan requires the aggregate |
| `Planner<i64, AddOp>` | `Planner<Counter, CounterMutation>` |
| `DoubleAdd` and `DerivedDoubleAdd` | the single derived `AddCounterTwice` type |
| `QuadAdd` | `AddCounterFourTimes` |
| `AddThenNotifyForeign` | `AddCounterThenNotifyForeign` |
| `AddInference` | `CounterInference`, colocated with `CounterDiff` because it reads the `value` region |

`Edit` must become `Edit<CounterMutation>` and contain the aggregate variants, so the serde test covers the actual owner aggregate. The `OpText` tests use `AddCounter` and assert only the new `add-counter <integer>` grammar; the historical `add` spelling must be rejected. The derive-composite law asserts `MutationKind<Counter, CounterMutation>` on `AddCounterTwice`, while the manual plan laws use that identical type. The assertions stay semantically equivalent: +5 then -5 restores the base; a two-step plan is +6; nested two-by-two is +8; foreign steps do not affect local diff; maximum depth and repeated foreign target still fail as typed errors.

## Required new neutral and runtime coverage

Before moving source, add a neutral roster fixture with positive rows for all four leaves and negative rows for duplicate semantic kind, aggregate variant mismatch, duplicate opcode, wrong owner, missing descriptor field, a composite falsely classified as atomic, i64 below/above-range `delta`, and `foreignCount` below zero or above 255. A schema validates every descriptor's exact nullable fields and payload schema references. A Node/Ajv test validates that fixture separately from the Rust derive. The schema carries the exact decimal i64 endpoints; the Rust parser uses exact decimal boundary vectors, while the Ajv vector uses an unambiguously out-of-range numeric literal such as `100000000000000000000` to avoid JavaScript number rounding masking the bound.

The Rust suite must add these fixture-owner tests in addition to preserving all rewired existing laws:

1. `CounterMutation::DESCRIPTORS` has four rows in roster order, each equals the corresponding leaf constant and has a source-valid direct owner/provenance.
2. Each aggregate variant's `descriptor()` equals its leaf descriptor and `From<Leaf>` selects that variant.
3. The generated registration succeeds once, equal replay is idempotent, and a conflicting descriptor batch returns an error without publishing a partial roster.
4. `AddCounter` text prints and parses `add-counter <integer>` exactly, rejects the legacy `add <integer>` spelling, and rejects i64 overflow; `AddCounterThenNotifyForeign` rejects foreign-count values outside `0..=255` before planning.
5. Every composite's plan diff and inverse execute through `CounterMutation`, including the derived-composite delegation and foreign-step exclusion.

The source unit invocation should target the named rewired tests and the new fixture-owner tests under the normal command project Nx target. The exact command is intentionally deferred to the owner because the current command project target and test filtering are being changed by the root; no unexecuted command is represented as a pass here.

## Generic proof coverage

A generic fixture is legitimate when it is physically authored under a canonical test mutation root and derives its own real metadata. It does not invent provenance merely because its Rust payload is generic. This packet does not claim that proof executed.

The next bounded generic fixture should be a sibling source-owned tree named `🧬️generic-mutation-laws`, with `GenericCounter<'a, T>`, `GenericCounterDiff`, `GenericAddCounter<'a, T>`, a transparent generic `GenericCounterMutation<'a, T>`, and a real `➕️add-generic-counter` leaf descriptor/schema. The payload carries `PhantomData<&'a T>` while its arithmetic remains over `value: i64`; a test instantiates `T = &'local u32` through a helper that infers the local lifetime. This is a real derive/generic-bound proof, not a hand-built `MutationLeaf` implementation. It must be compiled and run against a fresh paired kernel artifact before being claimed.

## No-state aggregate and publication correction

### Initially observed no-state aggregates

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` currently declares three one-variant `Noop` mutation enums:

| Aggregate | Current no-op variant | Current direct external emission | Required replacement |
| --- | --- | --- | --- |
| `NoConfigMutation` (also `NoDraftMutation` alias) | `Noop` | none; its only concrete use is its own inverse | uninhabited no-state aggregate |
| `NoPresenceMutation` | `Noop` | four publication test-factory calls at 17984, 18048, 18345, 18540; one `TestCommand::Increment` emit at 33961 | uninhabited no-state aggregate plus a real test presence leaf |
| `NoTransientMutation` | `Noop` | one `TestCommand::Increment` emit at 33961 | uninhabited no-state aggregate plus a real test transient leaf |

The four presence calls were all in the core retained publication test factory; the latter fifth call was the only production-test `TestCommand` emitter. The counts above exclude each enum's self-inverse expression because that expression disappears with the uninhabited type. No `NoConfigMutation::Noop` callsite outside its own implementation was observed.

These types represent absence, not concrete mutations. Per the mutation-tree contract, they receive no leaf directory, descriptor, provenance, semantic kind, opcode, binary tag, or registry entry. The correct implementation shape is `pub enum NoPresenceMutation {}` (and equivalents), `Mutation::DESCRIPTORS = &[]`, with uninhabited `diff`, `inverse`, `descriptor`, `print_op`, and `encode_op` matches. `parse_op` and `decode_op` must reject every input, including empty input. Do not derive a fake `DslOps` no-op surface and do not provide a default variant.

The language/schema representation for every no-state aggregate is the empty union/false schema rather than a one-member `noop` union. Codec tests must prove no text or binary input is accepted. A compile probe must prove the empty descriptor roster remains usable as an `ArtifactApp` lane type, while a separate negative compile probe attempting `NoPresenceMutation::Noop` fails. This is absence semantics, not a metadata exception.

### Real publication fixtures

Replace the five `Noop` test emissions with two physically authored, source-owned test fixture families: `PublicationPresence`/`PublicationPresenceMutation` and `PublicationTransient`/`PublicationTransientMutation`. Each has a concrete `revision: u64`, a single direct derived `ChangePublicationPresence` or `ChangePublicationTransient` leaf, a complete 14-field descriptor, and a payload schema. `change` is in `protocol::APPROVED_VERBS`; `advance` is not. Their `MutationDiff` is a structural `Option<u64>` replacement: default is identity, an explicit revision replaces, and absorb keeps the later explicit revision. Their inverses restore the arbitrary prior revision. Each leaf owns a text opcode and stable binary tag; the aggregate delegates its codecs and declares `text`/`binary` surfaces. The test app selects those real types only in the publication test factory; normal applications continue to select the uninhabited no-state aggregate and emit no lane mutations.

The publication runtime test must assert a real presence mutation reaches the begin/preflight/publish/ack/retirement path and advances presence generation, while a real transient mutation reaches the transient lane and advances transient generation. It must retain the cancellation and latest-wins boundary assertions formerly using `NoPresenceMutation::Noop`. This proves the pipeline with actual descriptors without turning the framework's no-state defaults into emitted production dummies.

### Separate concrete owners, not part of this packet

`InteractionConfigMutation::SetState` is a real concrete mutation: it is dispatched at plugin-core line 21850 and has active text/binary codecs. It must be assigned to a dedicated owner for a direct leaf conversion; this plan does not authorize that source edit. Its current whole-state payload, codec semantics, and inverse need a descriptor-first leaf design rather than the no-state treatment.

The inspected GIS configuration owners are likewise concrete publication metadata gaps, not no-state exceptions: `Gis3dConfigMutation` is central at the gisterrain editor config source (variants beginning at line 92) and `Gis2dConfigMutation` is central at the gismap editor config source (variants beginning at line 134). Both need independent per-operation leaf rosters and consumer rewiring. That broader GIS conversion is explicitly outside this packet; no claim of a completed GIS design is made.

## Evidence boundary

Inspected source paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧬️registry/**`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- non-compose GIS editor configuration sources under `✏️s/🔌️plugins/🌍️gis/**`

No `compose/**` path was accessed. No source was modified and no Rust compiler, Cargo, or runtime test was run in this packet.
