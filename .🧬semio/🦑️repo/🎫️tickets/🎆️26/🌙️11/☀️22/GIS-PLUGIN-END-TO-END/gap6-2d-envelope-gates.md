# Gap 6: 2D envelope gates (static diagnosis, nothing built or run)

## Per test
| Test | Location | Exists / API compiles (static) | Likely cause | Fix |
|---|---|---|---|---|
| `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (crate `semio-s-artifact-gis-gismap-gis2d`) | Yes. Every API it calls still exists (`advance_artifact_envelope_load`, `seal_artifact_envelope_ingress`, `acknowledge_artifact_store_replacement`, `pump_worker_job_retirements`, `MountedOwnerPolicyV1`). | `Ready` is only reported at replacement `Complete`, which follows `RetiringCommittedStore` (displaced boot store retired one item per turn, ~121k turns measured). Budget is already 1_000_000. Ack order is correct: ack needs `terminal_is_empty`, reached before `Ready`. Cost, not a hang. | None needed. Optional: larger retirement grant in the harness (not done: unverifiable without a compile). |
| `drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed` (+ `_cancel_`, `drawing_live_initializer_candidate_*`, `_rejects_single_and_final_edit_id_*`) | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/✉️envelope-laws/🦀️.rs` (crate `semio-s-artifact-draw-drawing-editor`) | Yes, same APIs. | Drive loop budget was 100_000 turns, same displaced-store retirement as the GIS twin that needed 121k, so a correct load could panic "did not reach terminal". | Fixed: `DRAWING_LIVE_LOAD_TURN_BUDGET = 1_000_000` used by `drive_drawing_load`. |
| `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault`, `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/📨️envelope-decode-ladder/🦀️.rs`, mounted in crate `semio-plugin-tests-08` | Yes. The old gate used the non-existent package `semio-framework-plugin` (crate is `semio-plugin`; the tests are in `semio-plugin-tests-08`). | Slow: up to 100_000 reactor turns with a 256-step slow decoder, then a fixed 10_000-iteration cancel maintenance loop with no early exit. Prior aborts at ~25 min match this. Not changed: the terminal predicate after cancel could not be confirmed statically. | Script fixed (right package). Measure first; if still slow, give the cancel loop an early exit. |
| `component_cold_map_patch` (native) | `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`, `[[test]]` in crate `semio-hub-gis` | Yes | Needs env `SEMIO_GIS_COMPONENT_WASM`/`_SHA256`, set only by the project target; a bare cargo run panics on `expect`. | Run via `repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check`. |
| Full gismap 240/240 | crate `semio-s-artifact-gis-gismap-gis2d` `--lib` | n/a | Blocked only by the envelope test cost. The old gate ran `-p semio-s-artifact-gis-gismap --features component-app-assembly`: that crate is a facade, and the feature does not exist in any Cargo.toml. | Script fixed. |

## Gate script (`GIS-2D-END-TO-END-BUILD/📜️verify-gis2d-envelope-gates.sh`) was stale
- Raw `cargo`/`bun nx` replaced by `repo cargo` / `repo run`; nested workspaces use explicit `--manifest-path`.
- Wrong package `semio-framework-plugin` -> `semio-plugin-tests-08` (root workspace, no manifest-path needed).
- Non-existent feature `component-app-assembly` removed.
- `activate-gis2d-wgpu-dev` no longer exists anywhere outside ticket notes; dropped (launch step, not a gate).
- Order: fast first, `one_reactor_turn_*` last. CLT SDK fallback preamble kept.

## Commands for the coordinator (in order; `-- --test-threads=1` throughout; C = `repo cargo --store=shared --configuration=prototype --`)
1. `C test -p semio-plugin-tests-08 advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault -- --test-threads=1`
2. `C test --manifest-path ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml -p semio-s-artifact-draw-drawing-editor drawing_live_envelope -- --test-threads=1`
3. `C test --manifest-path ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml -p semio-s-artifact-gis-gismap-gis2d gis_map_live_envelope -- --test-threads=1`
4. `repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check`
5. `C test --manifest-path <gismap Cargo.toml> -p semio-s-artifact-gis-gismap-gis2d --lib -- --test-threads=1` (240-test gate)
6. `C test -p semio-plugin-tests-08 one_reactor_turn_pumps_the_envelope_decode_worker -- --test-threads=1` (slowest, last)
Or run the whole script. Log under the ticket `🗑️generated/`.
