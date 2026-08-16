# Wave 1 barrier — combined-tree verification

Run serially by the coordinator after W1-A/B/C reported.

## Gate results

| Gate | Result |
|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | **909 passed / 0 failed** — fully clean. The three failures carried over from the W0 barrier (io registry conflict, dsl fixture sweep, store alternative validation) were all fixed by the concurrent sessions that caused them. |
| `cargo test -p semio-framework --lib` | **137 passed / 0 failed** — clean. |
| `cargo check -p semio-framework-plugin-host` | clean (warnings only). |
| `cargo test -p semio-framework-plugin --lib` | 152 passed / 58 failed — **every failure external** (below). All of this ticket's own tests pass: 8/8 transaction testkit, 5/5 contribution + builder-dependency, 3/3 mutation-plan, 3/3 extension-bundle. |

## The one real defect this barrier found (ours, fixed)

`transaction_testkit_tests::generation_mismatch_is_rejected_with_the_frozen_code` failed — and it was right to.

The test moved the store generation with a local `dispatch_typed` and then expected the commit to reject with `transaction.generation-mismatch`. But contract §5.10 (added during the W0 design review) makes a pending transaction freeze the instance's mutating surface, so that local edit is rejected first with `transaction.instance-busy`. The test was asserting a state the contract makes unreachable.

The fix is not to relax §5.10 — it is to exercise §5.8 through the path that can actually reach it. **§5.8 and §5.10 are complementary**: §5.10 blocks *local* drift, so the only way a prepared member's base generation can go stale is an edit that never passes through a command at all — a remote envelope ingested from the backbone mid-transaction. The test now drives exactly that (`MemoryBackbone::pair` → peer edit → `ingest_operations` into the pending instance → commit rejects), and the reasoning is recorded in a doc comment above it so the next reader does not "simplify" it back into a local dispatch.

## External failures (attributed, not ours)

- **53** `"app id … must be a canonical surface id: missing '#'"` — ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` made app ids canonical surface ids (`role#dialect`) and has not yet migrated the pre-existing test fixtures.
- **5** others: `ArtifactDefinitionError { code: "artifact-definition.category-identity" }` on `s.stdio.ifc.localized` / `s.stdio.ifc.first-0`, a child-factory `Conflict { kind: "s.test.child" }`, and a remote-mutation-id payload conflict — ticket `26/08/16/FULL-STDIO-…`'s identity-grammar work.

## Verdict

Wave 1 closed. The guest side is complete: a plugin or extension can declare dependencies, contribute mutations and inferences onto a dependency's artifact, plan a composite, and act as a correct transaction member. Wave 2 (both hosts) opened.
