# Run Algebra Implementation

## Bounded source change

The three independently reproduced Run algebra failures are repaired only in `🔁️workflow/🦀️component.rs`.

- `RunDiff` now has structural `Sequence { steps }` composition. `absorb` flattens nested sequences, removes `Empty` identities, and preserves every atomic diff in order. `apply` executes a sequence against a local snapshot, so a later typed rejection returns `Err` without mutating the supplied base.
- `FinishRunNode` now replaces a matching record in place. A first completion still appends; a replacement retains its existing list position, so the existing inverse payload restores the prior record at the original position.

No Run leaf payload, aggregate operation spelling/tag, schema, descriptor classification, checked admission seam, RunSink behavior, clock behavior, root fixture, script, or compiler controller changed. This does not invent an inverse for first `FinishRunNode` insertion; its typed inverse-availability boundary remains open as recorded in `📓️typed-inverse-contract-35-review.md`.

## Permanent regressions

Five source-owned Run tests were added to the Workflow component. They cover the three accepted neutral laws plus associativity/identity and both later-step rejection branches:

1. `run_diff_absorb_preserves_each_append_in_order`
2. `run_diff_absorb_preserves_start_before_later_log`
3. `run_diff_absorb_is_associative_with_empty_identity`
4. `run_diff_sequence_rejects_later_steps_without_mutating_the_base`
5. `finish_run_node_replacement_inverse_restores_the_original_node_order`

The current actual-source roster was 49 tests; this adds five, so the root controller should expect 54 Workflow tests for the frozen source check.

## Language-neutral evidence

The existing root-owned `🧪️run-algebra-35/🔣️vectors.json` was reused unchanged. Before and after the source patch, this scoped command passed all three Ajv-validated independent TypeScript reference cases:

```text
SEMIO_TEST_ARTIFACT_DIR=<ticket>/🧪️run-algebra-35/🧫️post-fix-neutral bun ./📜️script.ts nx exec --projects=workspace -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️run-algebra-35/📜️script.ts
[DEBUG] Run algebra reference append-then-append=passed
[DEBUG] Run algebra reference start-then-append=passed
[DEBUG] Run algebra reference replacement-inverse-order=passed
[DEBUG] Run algebra neutral schema and independent reference passed=3
```

The Rust source tests and public algebra client were not run in this lane: root owns rustc/Cargo and the actual-source controller. Source SHA-256 after this bounded implementation is `7b4c1109194eb5187d9b228bd791343d146fb25f3ffa711e91a5e623bd2a49fe`.

## Remaining boundary

Application-time `store::now_iso()` still makes replay-time timestamps nondeterministic. It was intentionally not changed in this algebra packet and remains a separate deterministic replay design issue.
