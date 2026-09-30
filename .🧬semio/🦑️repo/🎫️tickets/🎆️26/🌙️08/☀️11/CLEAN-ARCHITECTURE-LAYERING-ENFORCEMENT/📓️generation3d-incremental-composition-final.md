# Generation3d Incremental Composition Verification

The seven authored incremental evaluation laws remain in `✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🔁️generation3d-incremental-eval/🦀️.rs`. No law names, assertions, geometry fixtures, performance limits, validation gates, or production APIs were changed by this repair.

The concrete harness now supplies `geometry_session().port()` to both `FlowHost` and `FlowEvalSession`. Its convergence helper preserves the existing tick-until-converged walk and then releases the producer authority's temporary claims with `geometry_session().retain_geometry_handles(&[])`. The evaluation session has already claimed the current evaluated channels through its supplied port. The obsolete solid's packed mesh remains owned by the evaluation session for the unchanged preview fallback assertions, while stale kernel topology is no longer retained by the producer.

The reviewed incremental source SHA-256 is `4d59a66a5656c27ac4e83feed4892eb39ba24e802e9ec2729c6063a34a08c942`. Only its own entry in `✏️s/🧑‍💻dev/🧩️composition-laws/🧫️fixtures/🔣️.json` was hand-edited. Other agents own the geometry and app-law entries in the shared fixture.

## Recorded Native Runs

The baseline canonical target exited 1. Its first four laws passed, then `a_superseded_solid_handle_keeps_the_converged_mesh_until_the_new_pack_lands` failed at the edited solid tessellation with six `non-manifold-edge` findings: profile edges had four coedges. Adding the supplied host and session ports alone reproduced this failure. The producer was still retaining both old and new extrusions in its kernel family.

The unchanged edited-solid tessellation validation remains enabled. The claim-transfer repair was verified with the same canonical command:

```sh
bun run nx run @semio-tech/s-composition-laws-rs:canonical-architecture --skip-nx-cache --output-style=static -- --test generation3d-incremental-eval
```

Both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` were set to `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/native-canonical-final`. `SEMIO_TEST_ARTIFACT_DIR` was set to the same ticket's `🗑️generated/incremental-final`, with `NX_DAEMON=false` and `NX_CACHE_PROJECT_GRAPH=false`.

Temporary run logs and native per-law outputs are inside `🗑️generated/incremental-final`: `baseline.log`, `supplied-port.log`, and `producer-claim-transfer.log`. The coordinating agent owns final generated-output cleanup and ticket closure. No temporary `[DEBUG]` logs were introduced into source.

Final native result: **7 passed, 0 failed, 0 ignored**. The Nx canonical target exited 0, reported successful completion, and skipped its cache. Its total reported duration was 5.6 seconds.

The successful native artifact group was `🗑️generated/incremental-final/exact-cargo-laws-efREI8/00`. Each `law-0.json` through `law-6.json` records `status: 0`, `signal: null`, and exact execution with `--exact --test-threads=1 --show-output`; each corresponding stdout confirms one pass and zero failures or ignores. The runner's prior list step checked exactly seven native law names against the reviewed fixture.

| Law | Native Result |
| --- | --- |
| `an_edit_dirties_its_own_branch_and_leaves_every_other_node_to_free_ride` | Passed |
| `a_burst_of_values_costs_one_dirty_branch_per_value_and_never_a_growing_one` | Passed |
| `only_a_changed_handle_is_owed_a_tessellate_round_trip` | Passed |
| `an_unanswered_node_keeps_its_converged_answer_and_an_answered_one_never_does` | Passed |
| `a_superseded_solid_handle_keeps_the_converged_mesh_until_the_new_pack_lands` | Passed |
| `a_chain_hop_names_the_preview_and_names_the_graph_only_when_the_census_moved` | Passed |
| `the_chrome_digest_moves_on_a_fault_or_a_chrome_edge_and_never_on_a_reshuffle` | Passed |

Existing runtime `[STATS]` output confirmed height dirty nodes `height`, `extrusion-axis`, and `extrude`; radius dirty nodes `radius`, `profile`, and `extrude`; five height burst values with the same exact branch; and one retessellation against the unchanged budget of one. The previously failing preview fallback law now passes all unchanged assertions for old packed mesh retention, new solid handle, successful edited tessellation, landed pack, and republished evaluation.
