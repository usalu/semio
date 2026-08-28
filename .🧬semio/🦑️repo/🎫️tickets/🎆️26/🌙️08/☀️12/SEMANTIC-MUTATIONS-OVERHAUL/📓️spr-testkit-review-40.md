# SPR Testkit Mutation-Law Review 40

## Scope and evidence

Read-only review of the mounted test-only fixture tree and its direct consumers:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🧪️tests/🧬️mutation-laws/🧬️mutations/🦀️.rs`
- five direct leaf `🦀️.rs` files below that roster
- the `mutation_law_fixture` mount and its consumers in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`.

The reported source/neutral run is 84 passing cases. This review did not run Rust, so it does not claim compiler or runtime acceptance.

## Static result

No definite compile, type, or privacy blocker was found in the reviewed mount graph.

- The test-only root is mounted at crate scope under `#[cfg(test)]`; its `mutations` roster is public within that root, and each leaf's `super::super::super::tests::assert_leaf` resolves to the root-local test module.
- The leaf imports use the intended public aliases: `crate::os_spr` for command traits and `dsl_derive` for derives. `#[mutation_leaf(contract = ::protocol)]` is therefore not a spurious unresolved-contract concern.
- `AddObservedCounter` is constructible through `Default`; its skipped `Rc<Cell<i64>>` field has a default for deserialization and is deliberately private. No outside consumer constructs that private field.
- Every descriptor advertises only `rust`, `json-schema`, and `text`, and its matching direct leaf implements `OpText`; none falsely advertises binary support.

## Deliberate negative-law fixtures retained

The changed component consumers retain all three relevant `#[should_panic]` checks and route them to their corresponding direct leaves:

| Existing law purpose | Current fixture | Expected panic remains |
| --- | --- | --- |
| rejected forward must not be inverted | `AddRejectedCounter` | `must not have been rejected` |
| missing target must be an error | `AddUncheckedCounter` | `mutation.target-missing` |
| repeated outcome must be deterministic | `AddObservedCounter` | `must be deterministic` |

The ordinary positive missing-target check remains on `AddMissingCounter`, and the lawful inverse check remains on `AddCounter`. The old independent lossy text-codec `#[should_panic]` is still present as well.

## Inverse ordering

`CounterDiff` preserves delta arrival order on `absorb`. Its inverse reverses source deltas and converts a wide negation into representable `i64` steps. The leaf inverse then reverses those steps because store replay applies the returned inverse list in reverse order. For `i64::MIN`, the explicit test expects `[+1, +i64::MAX]`; reverse replay applies `+i64::MAX` then `+1`, restoring `i64::MAX` from `-1`. This is consistent with the documented store ordering and does not lose the minimum-value case.

## Remaining verification

Run the OS-kernel Rust test selection after the current source batch is compiled. In particular, that run must exercise the five leaf-local descriptor tests, the minimum inverse ordering test, and the retained three negative-law `#[should_panic]` tests. Static inspection and the neutral 84-case report are not substitutes for that run.
