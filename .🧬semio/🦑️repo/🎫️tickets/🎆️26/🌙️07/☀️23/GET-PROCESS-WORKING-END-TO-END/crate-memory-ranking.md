# Crate memory ranking and carving (peak RSS per rustc unit)

Scope: PART 2 of the memory-admission ticket. Part 1 (admission code) and `memory-admission.md` are written elsewhere.

## Method

- `rustc-mem-wrap.sh` is a `RUSTC_WRAPPER` (`/usr/bin/time -l -o` on macOS, `-v` on Linux). It appends `crate<TAB>peakRssBytes<TAB>wallSeconds<TAB>kind` for every unit that has `--crate-name`.
- `mem-cargo.sh <workspace> <label> <cargo args>` runs one cargo through the stopgap gate (`T/bin/cargo`). It uses a private `CARGO_TARGET_DIR` under `🗑️generated/native/build-mem`, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_*_DEBUG=0` and the absolute nightly `rustc` (the repo's own `build.rustc`/`rustc-wrapper` in `.cargo/config.toml` are overridden). `mem-chain.sh` queues several measurements one after the other.
- Measured: `cargo test --no-run -p semio-s-artifact-process-process3d-editor` in the process3d workspace. That is 1085 distinct units, including the whole `semio-framework-job` / value-native / dsl-record / tool-run-tick-wire closure and many `stdio` artifact crates. A cold run of this one command compiles about 2700 s of rustc wall time.
- Caveat: the repo compiles with `-Z threads=0`, so RSS and wall depend on the cores free at that moment. Numbers from different runs are only comparable when taken back to back (see the A/B below). The machine had load 19 during the first run.
- Not measurable here: `semio-plugin-tool-run` and `semio-s-artifact-energy-model-mutations` are not in the process3d closure. Their cold builds were queued behind other agents' live breakage (see "Blocked") and are not ranked. `semio-framework-dsl-record-model` (313 MB) and `semio-framework-job` (274 MB) were measured.

## Result: the framework job crate and the value-native crates are not the problem

| crate (lib, non-test) | peak RSS MB | wall s |
|---|---|---|
| `semio_framework_job` | 274 | 1 |
| `semio_framework_dsl_record_model` | 313 | 1 |
| `semio_framework_value_native_owned` | 364 | 2 |
| `semio_framework_value_native_*` (all 25 others) | 160 to 236 | 0 to 2 |
| `semio_framework_value_derive` | 537 | 6 |

These are already carved small. The incident cost did not come from them.

## Ranking by peak RSS (top 15 of 1085 units)

22 units peak above 1 GB, all of them in the process3d editor family or in `stdio` artifact crates.

| # | unit | RSS MB | wall s | kind | source LOC | diagnosis | action |
|---|---|---|---|---|---|---|---|
| 1 | `process3d_workpiece_window` | 3358 | 222 | test | 329 (plus 418 harness) | the shared app harness and the history law are compiled into this one test target | split done (harness crate) |
| 2 | `semio_s_artifact_process_process3d_editor` (lib test) | 3336 | 288 | test | 10 430 (lib 499) | same harness compiled in-crate, plus the law; 21 tests | harness source shared, law kept once |
| 3 | `semio_s_artifact_stdio_binary_io_registry` | 2000 | 42 | lib | 4450 in 65 files | artifact root crate holds `editor` (474), `io` (327), `diff` (250), declarations and tests in one unit | proposed: split `editor`/`io`/`diff` into subset crates, as `stdio.semio` already does |
| 4 | `semio_s_artifact_stdio_txt_tests` | 1730 | 32 | lib | 375 | test-support crate; instantiates the same typed-store generics | proposed: depend on the subset `member` crate instead of re-instantiating |
| 5 | `semio_plugin_actor_input` | 1725 | 26 | lib | 788 | small source, heavy: macro/generic expansion | investigate with `-Z self-profile` |
| 6 | `semio_s_artifact_stdio_semio_presentation_member` | 1544 | 36 | lib | 14 | `space_members!` instantiates the typed store for one (snapshot, mutation) pair | already one crate per subset, nothing to split; see "Monomorphization" |
| 7 | `semio_s_artifact_stdio_gltf_model_snapshot` | 1536 | 25 | lib | 1845 | derive-heavy snapshot | proposed: separate mirror/mutation derive output |
| 8 | `semio_s_artifact_stdio_semio_model_member` | 1523 | 17 | lib | 14 | `space_members!` | as 6 |
| 9 | `semio_s_artifact_process_process3d_host` | 1349 | 58 | lib | 1541 in 8 files | monomorphization-heavy (58 s for 1.5 kLOC) | proposed |
| 10 | `semio_s_artifact_stdio_semio_brep_member` | 1315 | 34 | lib | 14 | `space_members!` (incident crate) | as 6 |
| 11 | `semio_s_artifact_stdio_semio_brep_snapshot` | 1307 | 15 | lib | derive output | `DslRecord` + `NativeReceive` derive on a large record | as 7 |
| 12 | `semio_framework_kernel` | 1264 | 15 | lib | 9333 in 43 files | oversized | proposed: split along its 43 files |
| 13 | `semio_s_artifact_stdio_semio_document_member` | 1249 | 13 | lib | 14 | `space_members!` | as 6 |
| 14 | `semio_framework_artifact_flow_flow_model` | 1247 | 23 | lib | n/a | derive-heavy | as 7 |
| 15 | `semio_s_artifact_stdio_semio_table_member` | 1211 | 11 | lib | 14 | `space_members!` | as 6 |

## Ranking by wall time (top 15)

| # | unit | wall s | RSS MB |
|---|---|---|---|
| 1 | `semio_s_artifact_process_process3d_editor` (lib test) | 288 | 3336 |
| 2 | `process3d_workpiece_window` | 222 | 3358 |
| 3 | `semio_s_artifact_process_process3d_host` | 58 | 1349 |
| 4 | `semio_s_artifact_stdio_binary_io_registry` | 42 | 2000 |
| 5 | `semio_s_artifact_stdio_semio_presentation_member` | 36 | 1544 |
| 6 | `semio_s_artifact_stdio_semio_brep_member` | 34 | 1315 |
| 7 | `semio_s_artifact_stdio_txt_tests` | 32 | 1730 |
| 8 | `semio_framework_manifest_arg_schema` | 30 | 1037 |
| 9 | `semio_s_artifact_stdio_semio_kit_member` | 27 | 1180 |
| 10 | `semio_plugin_actor_input` | 26 | 1725 |
| 11 | `semio_s_artifact_stdio_gltf_model_snapshot` | 25 | 1536 |
| 12 | `semio_framework_artifact_flow_flow_model` | 23 | 1247 |
| 13 | `semio_s_artifact_stdio_semio_graph_editor` | 23 | 1153 |
| 14 | `semio_s_artifact_stdio_semio_cad_member` | 22 | 1101 |
| 15 | `semio_s_artifact_stdio_gltf_mutations_mutations` | 18 | 727 |

The two process3d editor test units are 510 s of the roughly 2700 s of the whole command, and each of the 9 test targets of that crate repeated the same cost.

## Diagnosis of the top offenders

### 1 and 2: the process3d editor test targets (duplicated instantiation, not an oversized crate)

- The test code is small (329 lines for `process3d_workpiece_window`, 21 tests in the lib unit). Yet each unit costs 3.3 GB and 220 s or more.
- Cause: `🧪️tests/🧰️support/🦀️.rs` (418 lines) was mounted with `#[path] mod context;` into **ten** compilation units (nine `[[test]]` targets plus the lib's own `cfg(test)`). The harness builds `VcsArtifactApp<EditorApp<Process3dPlayApp>, SemioMembers>` and ends with `semio_plugin::history_edit_acceptance_law!(...)`, which expands the whole generic `assert_history_edits_end_to_end::<EditorApp<..>, ..>` law suite. So the same generic instantiation was compiled ten times.
- The one-line source is not oversized: the editor lib itself is 1.0 to 1.4 GB (33 s at most).

### 6, 8, 10, 13, 15 and the other `*_member` crates (monomorphization, already carved)

- Each is a 14-line crate whose only content is `space_members! { ... }` for one subset. It still costs 1.2 to 1.5 GB and 11 to 36 s. The crate's own doc comment says "one subset is one crate so the typed stores of the whole artifact are never instantiated in a single compilation unit".
- They are therefore already split at the smallest unit the domain taxonomy allows. A further split is impossible: one enum variant is one (snapshot, mutation) store. The remaining lever is outside carving: make the typed-store engine non-generic behind an erased core (trait object or `dyn` port), so `space_members!` stops instantiating the whole engine per subset. This is a framework change in `semio_os_kernel::space_members`, proposed, not done.

### 3: `semio_s_artifact_stdio_binary_io_registry` (oversized)

- 4450 lines in 65 files in one crate: root declaration, `subsets/any/{editor,io,diff,tests}`. The editor (474), io (327) and diff (250) are separable by taxonomy. Proposed split follows what `stdio.semio` already does, one crate per subset part.
- Not implemented. This crate's `Cargo.toml` has 28 dependencies, and several other agents edit the artifact at the same time (see "Blocked").

### 12: `semio_framework_kernel` (oversized)

- 9333 lines in 43 files. A split along its existing module folders (identity, ui, history, media-jobs, already partly extracted as `semio_framework_kernel_*`) is clean in principle. Not implemented in this pass.

## Split implemented: one harness crate for the process3d editor tests

New crate `semio-s-artifact-process-process3d-harness` (compiled once, mounted by every test target as `context`).

- Moved `✏️editor/🧪️tests/🧰️support/🦀️.rs` to `✏️editor/🧪️harness/🦀️.rs` (crate root) with `📦️packages/🦀️rust/Cargo.toml`.
- Removed the `history_edit_acceptance_law!` line from the harness. The law now runs exactly once, in the lib's own unit tests (`🧪️tests/🔬️unit/🦀️.rs`), instead of in ten units.
- The eight external test targets (4 panels, workpiece window, interaction behavior, pane smoke target and root) replaced `#[path = "../🧰️support/🦀️.rs"] mod context;` by `use semio_s_artifact_process_process3d_harness as context;`.
- The lib's `cfg(test)` unit tests use crate-private items (`Process3dResumableCommandWork`, `PROCESS3D_*_TOOL_IDS`, `Process3dBoundedCommandJobFactory`), so they stay in the lib. They mount the same harness source with `#[path = "../../🧪️harness/🦀️.rs"] mod context;` (one source, no copy). A first attempt that moved them to an integration target failed with 62 unresolved-name errors for exactly that reason, and was reverted.
- The editor crate gets `semio-s-artifact-process-process3d-harness` as a dev-dependency (a dev-dependency cycle, which cargo allows). The workspace gets the member and the `[workspace.dependencies]` entry.

### Before and after (same machine, same flags, back to back in one cargo run)

| unit | peak RSS MB | wall s |
|---|---|---|
| `process3d_workpiece_window`, old structure (harness source plus the law mounted into the test; temporary target `ab_before`) | 6609 | 144 |
| `process3d_workpiece_window`, new structure (harness crate) | 4203 | 120 |
| harness crate itself, compiled once | 3366 | 25 |

- The new test unit needs 36 % less memory and 17 % less wall time. The two units compiled in parallel, which inflates both; the absolute numbers differ from the first run (3.3 GB and 222 s) because of `-Z threads=0` and load.
- The harness crate pays 3.4 GB once. Before, that cost was paid once per target (ten times).
- Net effect on the whole editor test command: nine units that each carried the harness and the law shrink to thin test crates; the 2 x 3.3 GB units that dominated the wall clock are replaced by one harness crate plus smaller tests.
- Compile verification: the lib `cfg(test)` unit, `process3d_workpiece_window` and the old-structure A/B target compiled (exit 0). The other seven targets use the identical one-line mount but their final full build was stopped by the unrelated brep-snapshot error below; they are not yet compile-verified.
- Not reached: a minimal test (`context::app()` only) to isolate the floor of the test unit. It was blocked twice by other agents' live edits (see "Blocked").
- The residual 4.2 GB in the test unit comes from generic method instantiation through the harness's `Deref<Target = VcsArtifactApp<EditorApp<..>>>` (`app.snapshot()` and others are monomorphized in the downstream crate). The next step is a non-generic facade in the harness (`harness::snapshot(&mut app)` and similar), proposed, not done.

## Blocked and caveats

- First attempts failed with `No space left on device` (the volume held 287 MB free: ticket build dirs `build-re2` 25 GB, `build-fuel` 19 GB, `build` 7 GB, `build-law` 3.8 GB belong to other agents). My private dir (9.3 GB) was removed to free space; this is itself a memory-admission input (disk is a shared resource; the unit cache and `cargo-gate` do not account for it).
- Builds were interrupted by other agents' live, unfinished edits that do not compile: `dbg::` calls in `🧬️protocol/📦️packages/🦀️rust/🦀️.rs` (value-native protocol), `semio_framework_trace::` in `🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs` (dev-only dependency used from a non-test unit), and a higher-ranked lifetime error in `semio-s-artifact-stdio-semio-brep-snapshot`. None is caused by this change.
- Measurements of `semio-plugin-tool-run` and `semio-s-artifact-energy-model-mutations` were not completed, so they are not ranked. By size: `tool-run` has 4378 lines in 14 domain folders and 40 dependencies; `energy-model-mutations` has 42 563 lines over about 75 domain folders (a split along the folder names is clean in principle, because each domain folder is already a module).
- The wrapper changes the rustc command line only by wrapping it. No product file is read by it.

## Changed files (all relative to `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d`; `E` is `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`)

- Created: `E/🧪️harness/🦀️.rs`, `E/🧪️harness/📦️packages/🦀️rust/Cargo.toml`.
- Removed: `E/🧪️tests/🧰️support/🦀️.rs` (moved to the harness crate).
- Edited: `Cargo.toml` (workspace member and dependency), `E/📦️packages/🦀️rust/Cargo.toml` (dev-dependency), `E/🧪️tests/🔬️unit/🦀️.rs` (harness path mount and the single law line), and the harness mounts in `E/🧪️tests/🎮️interaction-behavior/🦀️.rs`, `E/🧪️tests/🪧️pane-smoke-target/🦀️.rs`, `E/🧪️tests/🪧️pane-smoke-root/🦀️.rs`, `E/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🧪️tests/🔬️unit/🦀️.rs`, `E/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`, `E/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`, `E/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`, `E/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs`.
- `E/🦀️.rs` (the lib) is unchanged in the end.
- Measurement tooling in the ticket folder: `rustc-mem-wrap.sh`, `mem-cargo.sh`, `mem-chain.sh`. Generated outputs and the private build dir were deleted.

## Further splits, in priority order

1. Make `space_members!` build one erased core and instantiate only thin per-subset glue. Gains: roughly 20 units of 1.1 to 1.5 GB and 11 to 36 s each.
2. Split `stdio_binary_io_registry` (editor, io, diff) and the same pattern for other artifact roots over 2 kLOC.
3. Replace `Deref` generic calls in the process3d harness by non-generic facade functions.
4. Split `semio_framework_kernel` along its folders.
5. Admission side: use the per-unit peaks this wrapper records as the unit-cache estimate. The units above 1 GB are the ones that must be admitted one or two at a time; everything under 400 MB (the vast majority) can run at full job count.

## Round 2: the 4.2 GB test unit and the `stdio` splits

### What dominates the harness test unit now

- The harness crate holds the one generic app `VcsArtifactApp<EditorApp<Process3dPlayApp>, SemioMembers>` (3.4 GB, once). A test unit that only mounts it still costs 4.2 GB. The test sources are small (329 lines in the workpiece window), so the cost is generic code instantiated in the downstream crate: `Process3dApp` derefs to the generic app, and `app.snapshot()` (48 call sites over the targets), `has_pending_typed_operations`, `maintenance_terminal_is_empty`, `close_terminal_is_empty` and `artifact_generation_now` are generic methods that each test crate instantiates for itself. Debug builds share generics only when an upstream crate already instantiated the same item, so a non-generic facade in the harness turns ten instantiations into one.
- Done: inherent facade methods on `Process3dApp` in `E/🧪️harness/🦀️.rs` (`snapshot`, `has_pending_typed_operations`, `maintenance_terminal_is_empty`, `close_terminal_is_empty`, `artifact_generation_now`). Inherent methods win over the `Deref` target, so no test changed. File parses (rustfmt); not compiled and not measured (see below).
- Remaining generic call sites (two or fewer per target): `window_measures`, `rendered_config`, `context_menu` and the `take_typed_operation_*` family. They return framework types and can be added the same way once the first facade is measured.
- Not verified: the compile and the A/B number. Every `check` of this workspace now stops before the compiler at `semio-plugin-ownership exceeds its source boundary: 262796 of 262144 bytes`, another agent's file growing past the repo's source-size law. Expected effect, stated as a hypothesis: the test unit falls towards the floor of its own 329 lines plus the harness link, because the repeated instantiation is gone.

### `stdio` splits: not clean, not done

- `semio_s_artifact_stdio_binary_io_registry` (4450 lines) and `semio_s_artifact_stdio_txt_tests` (375 lines, 28 dependencies) are not hand-written crates. Their roots are generated mirror nests (`🪞️mirror/🦀️.rs`) that mount canonical subset files with `#[path = "../../../🪆️subsets/✳️any/..."]` and expand `dyn_enum_close!` at the root. The editor, io and diff modules reach each other through that nest (`crate::standards::v_raw::...`), so cutting them into crates means rewriting the generator output and every `crate::` path in 65 files, and every other artifact root shares the template.
- The right owner is the artifact generator: emit one crate per subset exactly like the `*_member` crates, then the registry shrinks by construction. A hand edit would be overwritten by the next generation and could not be compile-verified now (graph blocked, see above). Left as the first item of the generator backlog.
- `semio_framework_kernel` (9333 lines, 43 files) stays proposed: it is hand-written and splittable, but it sits under every other crate, so a half-finished split would block the whole fleet while the graph is already red.
