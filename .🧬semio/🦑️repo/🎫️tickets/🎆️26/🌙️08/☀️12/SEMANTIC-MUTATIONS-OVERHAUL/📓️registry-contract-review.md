# Mutation Descriptor Registry Contract Review

## Scope and Method

This is a read-only, bounded review of the current public protocol command registry and the
`#[derive(Mutations)]` emission path. It does not edit production sources, run Cargo, inspect
`compose`, or claim an exhaustive repository inventory. “Observed” below means exact call sites
found in the scoped roots `🧰️framework/🔨️modules/📡️replication` and
`🧰️framework/🛍️products/💻️os/🔨️modules`, excluding `compose`.

The original neutral schema, matrix, script, and their logs were created and run, then the entire
`🧪️registry-contract-review/` directory disappeared externally. It has not been recreated by this
review. The observed runs were: a neutral reference green after its atomicity self-correction, and
a current-source audit red with seven detected target gaps. Those artifacts are now unavailable,
so these are observations rather than retained reproducible evidence.

The proposed (not frozen) target contract is: immutable stable leaf and semantic identity; equal
duplicate idempotence; conflicting same-id rejection without replacement; and
preflight-before-commit batch atomicity. `workspace_token` is provenance evidence but machine-local
and must not participate in a cross-machine identity fingerprint. The root owner will freeze the
exact stable field roster and assign a new evidence directory before implementation.

## Observed Consumer Inventory

| Location | Observed responsibility | Result handling |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` | Defines `MutationDescriptor`, a process-global `OnceLock<RwLock<HashMap>>`, registration, lookup, and the only observed direct unit-test startup call. | `register_mutation_descriptor` returns `()` and overwrites `HashMap` entries. Lookup returns an optional clone. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs` | Public façade reexports the construction, registration, and lookup APIs. | No validation/result surface is added. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` | Emits one `MutationDescriptor::new(...).with_semantics(...)` registration for each aggregate leaf inside `register_*_mutation_descriptors`. | The emitted calls are statement-only because the present registry returns `()`. The observed non-generated invocation is `register_mini_mutation_descriptors()` in the command unit test. |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs` | Owns the immutable 14-field leaf descriptor and 6-field source provenance validators used by `Mutation::DESCRIPTORS`. | The leaf roster is compile-time validated but is not yet transferred into the runtime `MutationDescriptor` registry. |

The observed search did not establish global startup registration coverage. In particular, an
aggregate may have a generated registration function with no observed production startup caller;
this audit therefore does not claim a complete startup graph.

## Current Gaps Against the Frozen Target

`MutationDescriptor` currently contains id, schema version, state class, the old four semantic
fields, and optional contributor/artifact-kind fields. It does not own the direct-leaf descriptor
or source provenance. `descriptor_fingerprint` hashes only id, schema version, and state class;
`with_semantics`, contributor, and artifact kind deliberately do not change it. The registration
function directly calls `HashMap::insert`, so a conflicting same-id value silently replaces the
established value. There is no public preflighted batch API and no failure channel for generated
startup to propagate.

The proposed replacement contract must make the stable registry id, schema version, state class,
semantic verb/entity/kind/record, and direct-leaf descriptor values immutable. Its fingerprint
must cover the root-frozen stable identity, excluding the machine-local provenance
`workspace_token`. Other provenance paths remain validation/audit data unless the root explicitly
freezes them as stable identity. Equal
re-registration must succeed without replacement; a same-id unequal identity must reject and
leave the original entry unchanged; every batch must fully preflight before any insertion.

## Preserved External Boundary

`FlowConfigMutation::SetContributions` is a reserved configuration mutation arm. It is outside
this registry change and must be preserved; this review proposes no Flow edit or mutation-arm
removal.

## Extended Scoped Invocation Inventory

This extension searched current Rust source under `🧰️framework` and `✏️s`, excluding actual
`compose`, package glue, explicit test/fixture directories, build targets, node modules, caches,
and ticket evidence. It is exhaustive only for that exact textual pattern and filter set; inline
test modules remain part of their source file and are classified below rather than silently
excluded.

Observed registry API owner and façade:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` defines the
  `()`-returning overwrite API and contains the `MiniMutation` inline-test caller.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs` reexports both registration
  and lookup without a result boundary.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` is the live derive
  source: each generated `register_*_mutation_descriptors` function emits statement-only calls.
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` is a
  matching generated/package mirror; it was observed but deliberately excluded from the live
  source inventory, so the implementation transaction must update it by its owning generation
  rule rather than hand-editing it here.

Observed OS config result graph:

- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs` has
  `register_os_config_mutation_descriptors()`, returning `()`, and sequentially invokes opening,
  merge-policy, and identity aggregate registrars. No non-test caller of this wrapper was found by
  the scoped pattern search, so startup propagation is unproven and a future `Result` must be
  threaded through any owner startup that invokes it.

Observed direct aggregate call sites in `✏️s` are inline tests (not production startup evidence):

- procedural2d; vcs demo; sequence; FEM 2d and 3d; wires; playbook; imperative; remodel;
  trinity rewrite and jack; dag; draw; note; puzzle 2d/5d/3d; block 2d/5d/3d; space home and space.
- Each source invokes its generated `register_*_mutation_descriptors()` and then inspects semantic
  kinds. Because these calls currently return `()`, none can propagate duplicate/conflict failure.

No source claim is made about unsearched language representations, dynamically assembled symbol
names, or startup reached only through generated artifacts. The implementation owner must repeat
the inventory after freezing the API, including all result-bearing startup paths.

## Reproduction

No current command is supplied because the assigned evidence directory and its script are absent.
Root will assign a new uniquely named evidence directory after the stable identity contract is
frozen; that replacement must retain a green neutral reference and a red-to-green implementation
gate without overwriting the missing path.
