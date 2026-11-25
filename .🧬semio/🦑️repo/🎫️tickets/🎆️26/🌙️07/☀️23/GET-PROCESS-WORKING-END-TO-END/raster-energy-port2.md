# Raster and Energy Port, Second Pass

Status: partly done. Neither plugin suite is green. The build dir and all logs are deleted. Two other agents' transient breakages and a full disk interrupted the runs more than once.

Builds ran through the cargo gate, one at a time, with a private build dir. The helper scripts are `re2-cargo.sh` and `re2-queue*.sh` in this folder. All temporary `[DEBUG]` code is removed.

## Results per decision

### 1. `OriginalWorkFixture::retire_owned` and native receivers: partly fixed, 7 raster tests still red

- **New `retire_receiver`:** it retires a `NativeReceivingOriginal` the way production does. It takes `begin_close_demand` or `close_demands` and runs `retired_native` with `receiver_close` on a fixture turn. `try_receive_native` and the two raster native tests now use it.
- **Admitted decoder:** `OriginalWorkFixture::with_source` now admits the decoder prerequisites via `admitted_decoder()`, as the pack tests do. This fixed both arc tests, so the raster lib went from 15 to 17 passing.
- **Still failing:** the six coverage and payload tests that retire a receiver (`original_flat_coverage_receives_packed_pages`, `original_coverage_construction_proves_span_order_length_and_coverage`, `packed_coverage_projection_shares_pages_for_exact_work`, `payload_native_receiving_and_canonical_bytes_match_image_and_serde`, `packed_coverage_read_codecs_match_the_independent_flat_vector`, `original_native_payload_cancels_at_every_actual_receiving_turn`). The retirement turn fails with "original work requires its original issuer and resolved quote".
- **Why it fails:** the fixture's receiving turns use the unquoted `with_native_receiving` and each turn is its own Source dispatch. The Work binding of the receiving scope does not match the dispatch of the retirement scope.
- **Dispatch retention does not help:** I tried `retain_dispatch()`, as the pack tests do. Receiving then stalls (`owner.output_ready()` fails). The likely cause is that the dispatch cumulates its retained progress, but I did not confirm it.
- **Plain close effect:** closing through a plain close effect instead of the retirement scope fails with "original receiver birth requires settlement" or "requires its original close loan".
- **Still needed:** an owner for the receiving protocol must say how the fixture should keep one Work scope across receiving and retirement.

### 2. Retirement frame of 4104 bytes against a 4096-byte grant: not fixed

- **What I measured:** `RasterSelectionSpan` is 12 B, `RasterSpanPage` 3080 B, `SealedShared<RasterSpanPage>` 72 B, its cursor 3176 B, the page's cursor 3096 B. The root owner's `ControlledRetirement` is 880 B and the page's is 848 B. None of these is 4104.
- **Where the demand appears:** the stuck demand is `copy_bytes 7184, capacity_bytes 4104, depth 2`, at turn 64 of retiring the builder output.
- **What it is not:** 7184 minus 4104 is 3080, which is exactly `size_of::<RasterSpanPage>()`.
- **Possible cause:** `RASTER_SELECTION_PAGE_ROWS` reserves 1024 bytes of metadata but does not count the 8-byte `len` field. Changing the formula to subtract `size_of::<usize>()` gives 255 rows and would also fit. That changes `pageRows` in several fixtures across plugins, so I did not do it without owner agreement.
- **Not found:** I did not find which recent change grew the frame. The derive and record machinery under `🌱️value` is being rewritten by the other agent, and the new `OriginalRecordRetirement` files are untracked, so there is no history to compare against.
- **Assertion:** the 4 KiB assertion is kept as written.

### 3. Denied `receive_arc`: no product change needed, the tests are unchanged

- **Result:** with the admitted decoder, a denied `receive_arc` returns an `Err` whose cause is `quote_refusal: "native construction exceeds original remaining Job grant"`. That is what both raster tests expect.
- **Earlier misdiagnosis:** the earlier `Ok(denied)` was the fixture turn returning early with `Ok(default)` because the decoder was not yet admitted.
- **Convention:** `receive_shared` also returns an `Err` for a refused quote (fixture case `quote-refused`) and a denied `Ok` for an insufficient turn grant (`denied-*`). The product is consistent, so no assertions were rewritten. A short-lived `Ok(denied)` rewrite of the first arc test was reverted.
- **Status:** both arc tests pass.

### 4. Raster host and pixels-tests targets: host ported, pixels-tests not checked

- **Fixture path:** the content-hash fixture exists. It was never missing. `pixel-work.rs` used five `..`, one too many; it is now four.
- **Receiving calls:** the two `with_native_receiving::<_,ValueError>` calls are ported to the current API: `with_native_receiving(&mut scope, |control,_|...)` with the step result captured and settled through `consume_receiving_retained(issued, ...)`. The foreign-caller case now computes `refused` from the step result and the scope's source or call refusal.
- **Other fixes:** the `inverse` call now goes through `::semio_os_kernel::ColdMutation::inverse`, the `apply_filter` mutation has an explicit `RasterMutation` type, and `with_effect` is `pub(crate)`.
- **Build:** the host lib test compiled without errors in the last gated run. I did not run it. A transient `higher-ranked lifetime error` in `semio-s-artifact-stdio-semio-brep-snapshot` (not mine) blocked a verified build of `raster-mutations-pixels-tests`. Its 7 type-annotation errors are unported and unchecked.

### 5. Energy viewer: 3 of the 6 failures fixed, 3 still red

- **Arena lifetime (fixed):** the three window tests (simulation, structure, zones) now open `semio_ui_contract_values::testkit::UiArenaFixture::new()`, as the fem tests do. The `semio-ui-contract-values` dev-dependency with `testkit` is added to the viewer.
- **Sealed-source handback (fixed):** the structure and zones tests now call `EnergyModelSnapshot::retire()` on every document they build. `render_lists_one_row_per_zone` builds its document with `from_model`.
- **Initial-source birth (unfixed):** the measured sizes in bytes are `EnergyNativeInitialSource` 13680, `OriginalStoreBirth` 10736, `OriginalConstructionAdmission` 7856, `OriginalReceivingPrimary` 2544, the Energy genesis cursor 2744.
  - The birth quotes `size_of::<Self>()` capacity and twice that as copy bytes, so it cannot fit the 4096-byte `TURN` grant.
  - `OriginalReceivingPrimary` is deliberately inline so no error frame is allocated after a refusal.
  - I raised `TURN` copy and capacity to 64 KiB, the fixture law and its schema to match, and the genesis policy copy to 256 KiB. The birth still did not complete, so there is another cause I did not find. I reverted those limit changes. The three tests still fail.
- **Viewer lib result:** 28 pass and 3 fail: `original_energy_initial_source_transfers_the_real_persistent_model`, `original_energy_initial_source_denials_foreign_caller_and_all_cancel_prefixes`, `original_energy_document_genesis_installs_the_same_native_publication_and_replay_issuer`.

### 6. Energy mutation-fixtures: ported, library compiles, test targets not verified

- **Constructors:** `snapshot()` now uses `crate::snapshot::reference::energy_snapshot_with_state`. The probe corpus imports the renamed or moved constructors (`change_glazing_material_{solar,visible}_reflectance_{front,back}`, `change_glazing_material_infrared_transmittance`, `change_pv_system_{temperature_coefficient,faiman_heat_loss_constant,faiman_heat_loss_wind,mounting}`).
- **Laws:** the framework's `assert_mutation_inverse_law`, `assert_mutation_inverse_sum_law` and `assert_mutation_diff_absorb_law` need one snapshot type that is both `Mutation<P>` and `ColdMutation<P>`. Energy has a native snapshot (`Mutation`) and a reference snapshot (`ColdMutation`), so no single type satisfies them. The fixtures now run identical assertions on the cold arithmetic in the new `inverse_law`, `inverse_sum_law` and `diff_absorb_law`, and `assert_inverse_sum(case)` is the entry point. `assert_semantics` and `declared_invariants` use the native snapshot type.
- **Leaf tests:** the 298 `inverse_diffs_sum_to_the_negative_diff` leaf tests now call `fixtures::assert_inverse_sum(&case())`, and the middle-row laws call the new law functions.
- **Link tests:** four leaf tests (`bind-weather-file` and `connect-referenced-model`, applies and refuses) used `store::LinkPin` and `store::ArtifactLink`, which do not resolve in the test crates. They now use `protocol::...`.
- **Build:** the fixtures library compiles. I did not get a clean build of the `model_vectors` and `construction_vectors` targets after the link fix, so I do not know whether the vectors pass.

## Changed files

Paths are relative to the repo root.

Framework:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/👷️work/🦀️.rs` (new `retire_receiver`, admitted decoder in `with_source`, `try_receive_native` retires through it)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/📦️packages/🦀️rust/Cargo.toml` (`semio-plugin-job-receiving` dependency)

Raster, under `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/`:
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖼️image-payload/🧪️tests/native.rs` (two `retire_receiver` calls)
- `🔨️modules/🏠️host/🧰️owned/🧪️tests/pixel-work.rs` and `🔨️modules/🏠️host/🧰️owned/🧪️tests/🦀️.rs`

Energy, under `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/`:
- `🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml`
- viewer window tests under `.../👁️viewer/🎭️modes/👁️view/🪟️windows/`: `⚡️simulation`, `🌳️structure`, `📊️zones` (`🧪️tests/🔬️unit/🦀️.rs` each)
- mutation fixtures under `.../🧬️schema/🧬️mutations/🧪️tests/`: `🔬️fixtures/🛠️constructors/🦀️.rs`, `🔬️fixtures/🧵️probe-corpus/🦀️.rs`, `🔬️fixtures/⚖️laws/🦀️.rs`, `🔬️fixtures/🦀️.rs`, `🔬️middle-row-laws/🦀️.rs`
- 298 leaf `🧪️tests/✅️.../🦀️.rs` files across the mutation domains, plus `🌦️bind-weather-file` and `🪢️connect-referenced-model` `⛔️refuses` and applies leaves

Ticket helpers in this folder: `re2-cargo.sh`, `re2-queue.sh` through `re2-queue13.sh`, and this report.
