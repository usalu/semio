# 1028 result: energy

STATUS: AMBER (update 19:40: hub lock refresh now passes and compilation runs; energy-owned compile errors fixed; mut-* split crates not yet reached because foreign store crates fail first). All 61 energy-model-mut-* crates pass the real source-boundary gate (census, no compile). Energy-owned wasm compile NOT yet verified: hub Cargo.lock refresh is blocked by a foreign manifest (see Remaining).

Guest crate: `semio-hub-energy-component` (`🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust`).

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-diagnostic` workspace=true (key absent in root workspace) -> path dep. SHARED CHANGE.
- Gate split of 11 energy-model-mut-* crates (file count over 32). Each domain crate was split into two cohesive crates; the new crate lives in a sub-directory of the domain dir (`<domain>/<emoji><sub>/📦️packages/🦀️rust` + `🦀️.rs`) and includes the moved op directories in place via `#[path]` (fixtures/oracles/paths unchanged):
- `semio-s-artifact-energy-model-mut-site` -> + `semio-s-artifact-energy-model-mut-site-ground` (dir `📍️site/🌋️ground`): moved change_ground_deep, change_ground_building, change_ground_shallow
- `semio-s-artifact-energy-model-mut-fenestration` -> + `semio-s-artifact-energy-model-mut-fenestration-geometry` (dir `🪟️fenestration/📐️geometry`): moved change_fenestration_height, change_fenestration_sill_height, change_fenestration_area, replace_fenestration_vertices
- `semio-s-artifact-energy-model-mut-lighting-gain` -> + `semio-s-artifact-energy-model-mut-lighting-gain-emission` (dir `💡️lighting-gain/🔆️emission`): moved change_lighting_gain_radiant_fraction, change_lighting_gain_return_air_fraction, change_lighting_gain_visible_fraction, change_lighting_gain_watts_per_area
- `semio-s-artifact-energy-model-mut-surface` -> + `semio-s-artifact-energy-model-mut-surface-properties` (dir `🧱️surface/🛠️properties`): moved change_surface_multiplier, replace_surface_vertices, change_surface_class, change_surface_construction
- `semio-s-artifact-energy-model-mut-window-layer-optics` -> + `semio-s-artifact-energy-model-mut-window-layer-optics-visible` (dir `🌈️window-layer-optics/🔭️visible`): moved change_glazing_material_visible_transmittance, change_glazing_material_visible_reflectance_front, change_glazing_material_visible_reflectance_back
- `semio-s-artifact-energy-model-mut-air-loop` -> + `semio-s-artifact-energy-model-mut-air-loop-nodes` (dir `🌬️air-loop/🔀️nodes`): moved change_air_loop_supply_node, change_air_loop_return_node, change_air_loop_design_supply_air_flow
- `semio-s-artifact-energy-model-mut-people-gain` -> + `semio-s-artifact-energy-model-mut-people-gain-fractions` (dir `👥️people-gain/🍰️fractions`): moved change_people_gain_sensible_fraction, change_people_gain_latent_fraction, change_people_gain_radiant_fraction
- `semio-s-artifact-energy-model-mut-ideal-loads` -> + `semio-s-artifact-energy-model-mut-ideal-loads-limits` (dir `💯️ideal-loads/🚧️limits`): moved change_ideal_loads_system_max_heating_capacity, change_ideal_loads_system_max_cooling_capacity, change_ideal_loads_system_max_heating_supply_air_temp, change_ideal_loads_system_min_cooling_supply_air_temp
- `semio-s-artifact-energy-model-mut-plant-loop` -> + `semio-s-artifact-energy-model-mut-plant-loop-design` (dir `🏭️plant-loop/🧭️design`): moved change_plant_loop_supply_temperature, change_plant_loop_return_temperature, change_plant_loop_design_flow
- `semio-s-artifact-energy-model-mut-year-schedule` -> + `semio-s-artifact-energy-model-mut-year-schedule-holidays` (dir `📆️year-schedule/🏖️holidays`): moved change_annual_schedule_holiday_daily_schedule, add_annual_schedule_holiday, remove_annual_schedule_holiday
- `semio-s-artifact-energy-model-mut-model` -> + `semio-s-artifact-energy-model-mut-model-simulation` (dir `🔋️model/🧮️simulation`): moved add_output_variable, remove_output_variable, replace_airflow_network
- Consumers updated (no shims): `🔋️model/Cargo.toml` (workspace deps), `📦️packages/🦀️rust/📜️script.ts` (package list), aggregate `🧬️mutations/🦀️.rs` + Cargo.toml, `✏️editor` lib + reduce-and-helpers + inspector-properties + Cargo.toml, `🧪️tests/🔬️*` (fixtures, probe-corpus, per-domain test libs), `surface` (re-exports + delete ops now use `fenestration_geometry_mutations`), `fenestration` and `year-schedule` (delete ops use the geometry/holidays crate for the inverse payloads).
- Design note: model split keeps the four weather/referenced-model ops together (shared `laws` test enum) and identity ops with `retained`; moved ops: add/remove output variable, replace airflow network.
- Tools/scripts: scratch only, `/private/tmp/claude-501/scr/` (not in repo).

## Verified
- `bun C/1028-tools/gatecensus/census.ts` (root semio-hub-energy-component): no energy-model-mut-* violator; only `energy-model-native-derive` reports the known proc-macro reader artefact. snapshot/native-mutations also green now (procedural's work).
- Static check: every `crate::X` used by moved/kept ops resolves in its crate (script); no compile done.

## Remaining
- Both wasm checks (`energy-chk4/5/6`) stop at hub Cargo.lock refresh: `failed to load manifest for workspace member 🌎️hub/🏗️bootstrap/⚙️runtime/...` (hub-runtime). Root cause found: untracked, peer-created `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/📦️packages/🦀️rust/Cargo.toml` has framework path deps with one `../` too few (7 instead of 8; resolves to `✏️s/🧰️framework/...`). Owner: stdio/process fleet. Re-run `energy` check after it is fixed.
- Model-workspace (🔋️model) `cargo check` also fails to load `🌱️initialization` member manifest (pre-existing, not investigated).
- Compile errors in the split crates (if any) are unknown until the hub check passes.

## SHARED CHANGE
- composition-publication Cargo.toml diagnostic dep (above).
- Energy-model workspace dependency list gained 11 crates.

## Update 19:40 (chk7-chk9, wasm32-wasip2, --keep-going)
Fixed (energy-owned, `semio-s-plugin-energy-engine-model`):
- `🔨️modules/⚡️simulation/⚙️engine/🔋️model/🏷️ids-and-fixed-table/🦀️.rs`: duplicate `EntityId` impls (ImmutableRootProjection, RetainedClone, NativeScalarValue, NativeReceive) from the trailing `id_projection!`/`id_receiving!` macros duplicating the explicit impls at the top -> removed the macros. This also cleared the E0599 `birth/begin` ambiguity and the E0367 NativeReceive Drop error in `🧱️spatial-material-and-gain-entities`.
SHARED CHANGE: `🧰️framework/🔨️modules/🧮️math/📶️rank/🦀️.rs` line 95: `PagedListProgress.released_bytes` was renamed `released_allocation_bytes` -> consumer updated (2 sites).
`🌱️initialization` member: manifest loads fine now (paths/workspace keys verified); the earlier failure was transient peer churn.

Foreign errors remaining (store tree mid-refactor by peers; mut-* split crates and the hub component are queued behind them, so they are still uncompiled):
- `semio-framework-os-store-replay`: `⏩️replay-owned-state-operations/🦀️.rs:503` `settlement.step(..)?` now returns `(bool, RetainedCloneProgress)` and `:600` calls private `OperationMessageSettlement::new` (operation crate `🎮️operation/🦀️.rs` edited 19:21 by a peer, consumer not migrated).
- `semio-framework-os-store-one-item` (earlier run): `semantic/🦀️.rs:3` `semio_framework_value::snapshot_projection` missing; imports `OriginalPostRead`, `PagedOneItemPost*` unresolved.
- chk9: `os-store-codec-registry` (E0404 `RetireOwned` derive used as trait, `📸️native-snapshot/🫴️receiving:98`), `os-store-presence`, `os-store-one-item-batch` (28 errors: `MutationMessageConcat` unresolved in `📬️paged-one-item/🧬️mutation-apply`, duplicate `original_registry`/`registry_retirement` fields in `📥️lifted/📬️publication:336`, duplicate `context/parent_copy/progress` definitions, SnapshotProjection Output mismatches).
Next: rerun `energy` check once the store owner lands; then fix any errors in the 22 split mut-* crates.

## Update 22:00 (chk10-chk20)
Energy-owned fixed: `semio-s-artifact-energy-model-diff-patch` `🧬️schema/🔺️diff/🩹️patch/🦀️.rs` (RowPatch rows: `row.key()` -> `P::row_key(row)`, 2 sites). No error has been reported in the 22 split mut-* crates or any other energy crate so far, but the mut-* crates and the hub component are still not compiled (they queue behind the plugin SDK).
Foreign crates fixed by me in free-fix mode (all files untouched >10 min when edited; SHARED CHANGE):
- `store/🕰️history/🎮️command` (`semio-framework-os-store-engine-command`): missing `protocol`/`ArtifactDerivedOperation(Mode)` imports, `native_vcs_refusal` helper, `native_vcs_header`->`header`, `native_vcs_progress`->`paid`, `pack` dependency (Cargo.toml), impl bounds -> `Clone+FromValue` / `Clone+ColdMutation` (now covered by StoreState/StoreMutationFor; may be replaced by the marker traits).
- `plugin/🪟️window/🫧️transient/🦀️.rs`: duplicate `RetireOwned` (derive + `artifact_retire_struct!`) -> macro line removed.
- `plugin/📜️declaration/🪆️child/🎯️editor/🧭️source/🦀️.rs`: `ChildEditorSourceCursor` derives `Clone, Copy` (needed by `artifact_retire_leaf!`).
- `plugin/📜️declaration/👁️view/🦀️.rs`: `pub use crate::child_view::ChildContentView`.
- `plugin/📜️declaration/🧩️app/🎛️artifact-app-trait`, `🪟️surface/✏️artifact-editor-trait`, `🪟️surface/👁️artifact-viewer-trait`: `type Snapshot: StoreState + ..`, `type Mutation: StoreMutationFor<Self::Snapshot> + ..` (puzzleplugin's marker traits) so ArtifactStore<Snapshot,Mutation> bounds hold; decl-app now compiles.
- `math/📶️rank` released_allocation_bytes (earlier). Reverted-by-me experiment: ArtifactStore struct bounds left as the peer's strong bounds (Drop impl needs them).
Frontier at chk20 (all in puzzleplugin's active StoreState sweep): `plugin-composition-publication` (store-replacement-* fns `P: Clone/FromValue/ArtifactPack` unsatisfied, `group:55` MemberValueError mismatch), `plugin-decl-surface` (10, editor-app PresenceMutation: ColdMutation), `plugin-command` (41, time-travel:551), `plugin-tool-run` (37), `plugin-document-media` (781). Re-run `energy` check after that sweep lands.

## CHECKPOINT 22:55 (chk21-chk27)
STATUS: AMBER. Energy-owned crates show no errors; hub component still blocked by plugin-layer crates. All my edits are compile-atomic.
Further fixes since 22:00 (free-fix, files untouched >10 min):
- `plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/staged.rs`: `StagedWindowContextCapture` derives `RetireOwned` (was leaf on non-Copy), `artifact_retire_leaf!(OriginalContext)`.
- `plugin/⚛️reactor/🚪️lifetime/🦀️.rs`: `artifact_retire_leaf!(NativeCloseKey)`.
- `plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`: `handle_intent_frame` call adapted to the 6-arg original-turn signature with local `args`/`original_fault`/`meta` (non-resumable across polls; caveat for the intent path).
- `plugin/🧵️instance/🏪️stores/📥️staging/📨️dispatch/🦀️.rs`: ArtifactDiff payloads built via `PagedBytes::try_from_slice` (CausalPayload is PagedBytes).
- `plugin/⚛️reactor/🧬️wit-bridge/🪢️bridge/🦀️.rs`: 80 flat `wasip2::X` paths remapped to `wit_events|wit_effects|wit_capabilities|wit_lifetime|wit_types|wit_ui::X`; `poll` now hands the kernel a single `Option<Event>` and refuses >1 event per poll (kernel turn API is single-event).
- `plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`: flat `wasip2::X` -> `Wire::X`/`JobsWire::X`, imports of OriginalActorGuestEntry/FixedReceipt/OwnedJobReceiptLoan and RetainedCloneGrant; `plugin/🧬️guest-contract/🎟️original-actor-context/🦀️.rs` now also exports `OriginalOwnedJobReceiptLoan`.
Last check chk27 (verifies the exports/guest-contract edit) was still running at checkpoint: output `C/🗑️generated/1028/energy-chk27.txt`, marker `energy-chk27.rc`.
Known open (not mine, not bounds): `plugin-instance-jobs` (7 errors, files edited 22:45 by a peer: `tasks_extension_continuation` path, `ToolActionBus::dispatch`, `ToolOperationSpec::new`, cancellation Option handle, `private_child_group_operation_close_demands` needs `M: ReferenceSpaceMember`); plugin-layer StoreState bounds (puzzleplugin); mut-* split crates and hub component not yet compiled.
NEXT STEP: read chk27 result, fix reactor-context/instance-jobs leftovers, then
`cd /Users/ueli/Documents/semio && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-energy-component` (detached with .rc marker per brief addendum 17:15).
