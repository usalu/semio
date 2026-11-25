# Raster and Energy Port to the Current Framework API

Both native plugins compile against the current framework again, and both hub `descriptor_is_fresh` tests pass. The plugins' own test suites are not green. What still fails is listed under "Open failures" with its cause.

Everything ran with native cargo, a private `CARGO_TARGET_DIR` / `CARGO_BUILD_BUILD_DIR` under `🗑️generated/native/build-re` and the env from `replay-cargo.sh`. The build dir and all logs are deleted. The scripts are `re-cargo.sh`, `re-hub.sh` and `re-hub2.sh`.

## Ports

- **Raster, `retained_alias`:** `PersistentList` lost `retained_alias` and `retained_alias_work_demand`. The replacement is `PersistentList::project` / `project_into` with `projection_work_demand`. All three take a `RetainedCloneRef` that only an original `RetainedCloneSource` can lend.
  - `RasterSelectionSpans::clone` only has `&self`, so it cannot get such a reference. The old root alias in `clone` is gone.
  - `clone` now rebuilds fresh pages through `RasterSelectionSpansBuilder` under `BlockingTurns`. It is an O(n) copy and no longer shares the root.
  - `StoreMutationFor` requires `Clone` on every mutation, so `RasterMutation` and the four selection mutations still need it.
  - The alias test `packed_coverage_alias_shares_pages_for_exact_work` is now `packed_coverage_projection_shares_pages_for_exact_work`. It runs the work, grant, denial and `same_root` assertions through `project_into`.
- **Raster, native receiver:** `RasterCoverageReceiver` now implements `native_terminal_release`. The backing close refuses a receiver without it ("lacks an admitted real terminal destructor").
- **Energy:** the viewer's `render_with_camera` call passes `view_state.locale` as the third argument.

## Framework fixes (root causes)

- **`OriginalWorkFixture::try_receive_native`:** it drove source admission with `advance` only. Admission now has a mandatory projection stage (`projectionStart`) and a two-step output `take`, so every native receive failed with "original projection must be driven or returned through its resident frame". The new `OriginalWorkFixture::admit_source` drives `project_owned`, `advance` and `take_output` until the admission is terminal. `try_receive_native` and the plugin tests use it.
- **`dispose_plugin_bundle_cold`:** the 65,536 iteration cap was too small for energy's definition graph. Energy's `descriptor_is_fresh` hit the cap, then the `OriginalAppDefinition` drop assertion fired ("committed native definition requires original alias return and typed graph closure"). The bound is now the caller's own item stock (`cx.retained_grant().maximum_items`). Failures still surface as errors, with no weakened assertion.

## Verification

| Check | Result |
|---|---|
| `semio-hub-energy-component` `descriptor_is_fresh` | pass |
| `semio-hub-raster-component` `descriptor_is_fresh` | pass |
| `semio-s-artifact-energy-model-viewer` lib | 25 pass, 6 fail |
| `semio-s-artifact-raster-raster-model` lib | 15 pass, 10 fail |

## Open failures

None of these are weakened, and none of the framework causes are patched blind, because they sit in the receiving and retirement protocol that other agents have uncommitted edits in (`🛬️decode/🧳️owned/🧬️protocol`, `🏗️record-machine`, and others).

### Raster model, 10 failing tests

- **Native-receiver retirement in the fixture (7 tests):** `OriginalWorkFixture::retire_owned` closes a `NativeReceivingOriginal` with plain `ControlledRetirement::step`.
  - The receiver now needs its original close loan and a settled birth. The refusals are "native original receiver requires its original close loan" and "original receiver birth requires settlement".
  - I tried `plugin_controlled_effect_close`. It moved the failure to "native decode continuation lacks valid cumulative accounting", so I reverted it.
  - It needs the native-retirement receiving scope, as in `original_steps::retired_native` of `plugin/👷️job/🛬️receiving`.
  - Affected tests:
    - `original_flat_coverage_receives_packed_pages`
    - `original_coverage_construction_proves_span_order_length_and_coverage`
    - `packed_coverage_projection_shares_pages_for_exact_work`
    - `packed_coverage_read_codecs_match_the_independent_flat_vector`
    - `payload_native_receiving_and_canonical_bytes_match_image_and_serde`
    - `packed_coverage_pages_preserve_flat_rows_and_close_with_four_kib_releases`
    - `original_native_payload_cancels_at_every_actual_receiving_turn`
- **Retirement frame size (2 tests):** `ControlledRetirement<SealedShared<RasterSpanPage>>` is now 4104 bytes. The builder close demand needs capacity 4104 and the test grant caps capacity and release at 4096.
  - The `d.release_bytes<=4096` assertion in `packed_coverage_pages_preserve_flat_rows_and_close_with_four_kib_releases` is kept as written.
  - `denied_and_cancelled_packed_frontiers_preserve_the_input_row` hits the same bound.
  - Fixing it means shrinking the frame (framework) or the page budget (4 KiB policy).
- **Arc denial semantics (2 tests):** `original_arc_quote_denial_keeps_input_before_allocating` and `born_original_arc_survives_enclosing_receipt_refusal_and_funded_close` expect `Err` from a denied `receive_arc`. It now returns `Ok(default)`, a denied turn.
  - Which behaviour is intended needs a framework owner. I did not rewrite the assertions.
  - They also need `maximum_items` above 1, which I raised to 64.

### Raster, other crates

These test targets no longer compile:
- `semio-s-artifact-raster-raster-host` lib test: `pixel-work.rs` uses removed receiving and `inverse` APIs, `with_effect` is private in `snapshot-clone` tests, and the `content-hash/🧪️tests/🔣️.json` fixture is missing.
- `semio-s-artifact-raster-raster-mutations-pixels-tests`: 7 type-annotation errors in the companion tests.

### Energy

- Viewer lib, 6 failures:
  - UI arena has no accepting lifetime registration: `the_viewer_renders_the_model_run_period_in_both_languages` and `render_lists_name_version_and_every_collection_count`.
  - `render_lists_one_row_per_zone`: building a persistent snapshot needs an original granted handback ("sealed source requires original granted handback"). I switched it to `EnergyModelSnapshot::from_model`.
  - Energy initial-source birth does not complete under the shared 1-item `TURN` grant. This hits the two `original_initial_source` tests and the genesis store test.
- `semio-s-artifact-energy-model-mutation-fixtures` has 19 compile errors, from `crate::snapshot::energy_snapshot_with_state`, the `change_glazing_material_*` and `change_pv_system_*` constructors, and the `Mutation` and `ColdDiffAlgebra` impls. It is a separate model refactor and I did not touch it.

### Not mine, but in the build graph

The hub raster build depends on the PDF crates (`stdio-pdf-lifter`, `pdf-lift`, `pdf-mut-family-*`). Other agents had them uncompilable for about two hours and they recovered unaided. I did not touch them.

## Disk

- The disk reached 99% full while the hub raster test binary was linked, and the binary came out truncated.
- I freed space by deleting every file over 20 MB under my `build-re`. That removed rlibs as well as test binaries, so the later hub builds redid about 1,000 crates. The build dir is now deleted entirely.

## Files changed

Framework:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/👷️work/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🪪️descriptor/🦀️.rs`

Raster, under `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
- `🧬️schema/🎯️selection/📦️coverage/🦀️.rs`
- `🧬️schema/🎯️selection/📦️coverage/🛬️receiving/🦀️.rs`
- `🧬️schema/🎯️selection/📦️coverage/🛬️receiving/🧪️tests/🦀️.rs`
- `🧬️schema/🎯️selection/📦️coverage/🛬️receiving/📥️owned/🧪️tests/🦀️.rs`
- `🧬️schema/🖼️image-payload/🧪️tests/🦀️.rs`
- `🧬️schema/🖼️asset-content-and-patch/🪪️identity/🧪️tests/🦀️.rs`
- `🚪️io/🖼️media-bridge/📦️packages/🦀️rust/Cargo.toml`
- `🚪️io/🧩️composite-and-media/🧵️prepare/🧪️tests/🦀️.rs`

Energy, under `✏️s/🔌️plugins/🔋️energy/`:
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️zones/🧪️tests/🔬️unit/🦀️.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🧪️tests/🔬️unit/🦀️.rs`
- `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🌱️initialization/🧪️tests/🦀️.rs`
- `🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️testing/🦀️.rs`

Ticket helpers:
- `re-cargo.sh`
- `re-hub.sh`
- `re-hub2.sh`
- `raster-energy-port.md`
