# 🪚️ Sourcing closure: peak memory per compile unit and carving (2026-11-24/25)

All builds ran through the admitted path: `sh …/SOURCING-END-TO-END/native-wasm-build.sh` (`repo cargo --store=shared --configuration=prototype`, wasm32-wasip2, grant decided by the gate, 4 jobs on this 16 GB Mac). Closure of `semio-hub-sourcing-component`: 918 crates (744 workspace). Peaks are the OS-reported maximum resident size of the compiler process tree of each unit.

## 1. Measurement that now exists

Nothing recorded per-unit peaks for a native `repo cargo` run (the broker telemetry `units.jsonl` only exists for the component pass; it had 0 sourcing rows). Added to the unit-cache wrapper (`📜️script.ts`):

- the compiler child runs under `Bun.spawn` (same jobserver descriptors, same signal forwarding, same exit mapping) and the process's `resourceUsage().maxRSS` is read at exit;
- every `miss` event of the lane log (`.cache/b/r/m/u/x/<lane>.jsonl`) carries it as the new schema-declared optional `Event.peakRssBytes` (`🧬️schema/🔣️.json`, `💾️store/🟦️.ts` type);
- `reportUnitCeilingV1` compares it with the budget of the admission policy (`unitCeilingAlarmV1`, the same verdict the component pass uses) and writes the bilingual `[unit-cache] … / …` line to the build log when an unlisted unit is above its budget. The policy JSON and the admission module are imported with literal specifiers (a computed specifier is not traced into the sealed copy and failed silently in the first run).

The other lane (fleet ticket) has since changed the policy to per-kind budgets (library 1536 MB, final `cdylib` 4096 MB serial) and already adapted `reportUnitCeilingV1` to it.

Note: a source edit while a build runs restarts the admitted command and recompiles the units whose key moved (cost me two 30 minute final recompiles); peers' framework edits did the same several times.

## 2. Baseline (thin LTO, the `prototype-component` row as shipped) — top 15 units by peak (MB)

| # | MB | wall s | unit | kind |
|---|---|---|---|---|
| 1 | 5519 (rerun 4450) | 1694 | `semio_hub_sourcing_component` | final, 269 MB wasm |
| 2 | 4273 (4080, 4062) | 32-85 | `semio_hub_sourcing` | lib |
| 3 | 3653 | 140 | `semio_s_artifact_sourcing_curation` | lib |
| 4 | 3565 | 365 | `semio_s_plugin_sourcing_slabs_component` | final |
| 5 | 2925 | 54 | `semio_s_artifact_stdio_binary_io_registry` | lib (listed) |
| 6 | 2592 | 38 | `semio_s_artifact_stdio_txt_tests` | lib (listed) |
| 7 | 2376 | 70 | `semio_s_artifact_stdio_semio_model_member` | lib (listed) |
| 8 | 2366 | 19 | `semio_plugin_actor_input` | lib (listed) |
| 9 | 2318 | 37 | `semio_s_artifact_stdio_semio_presentation_member` | lib (listed) |
| 10 | 2285 | 80 | `semio_s_artifact_stdio_semio_brep_member` | lib |
| 11 | 2148 | 374 | `semio_s_plugin_sourcing_windows_component` | final |
| 12 | 2116 | 71 | `semio_s_artifact_stdio_semio_graph_editor` | lib (listed) |
| 13 | 1845 | 20 | `semio_s_artifact_stdio_semio_animation_member` | lib (listed) |
| 14 | 1839 | 26 | `semio_s_artifact_stdio_semio_table_member` | lib |
| 15 | 1770 | 59 | `semio_s_artifact_stdio_semio_document_member` | lib |

Beams final 1591. Finals above 4096: only the hub final. Libraries above 1536 and not on the list before: hub_sourcing, curation, brep/table/document/value/kit member, curation_inferences (1546). Lane variance is large for the libs (e.g. binary_io_registry 1650 to 2925 across runs): their peak follows the number of front-end threads and back-end workers the grant gives.

## 3. Why the units are big (measured, not guessed)

Hub final (`cdylib`): the back end is already serial. With thin LTO the compiler holds the bitcode of the closure for the whole link-time stage (the fleet ticket measured 8.0 GB charged for the 953-library cad closure; here 4.5 to 5.5 GB resident for 918 crates).

`semio_hub_sourcing` (53 lines of wiring plus `dyn_enum_close!` of `VcsArtifactApp<EditorApp<…>, SemioMembers>` and the viewer twin): variants through the admitted capacity gate, replaying the captured rustc command line of the unit into a scratch directory (`🪚️capture-unit-argv.sh`, `🪚️unit-variant.sh`, `repo capacity run --memory=6144`):

| variant (same crate, lto off, opt-level s, 16 CGUs) | peak RSS MB | footprint MB | wall s |
|---|---|---|---|
| as admitted (`-Z threads=4`, parallel back end) | 4496 | 4602 | 57 |
| `-Z threads=1` | 4011 | 4161 | 56 |
| `-Z no-parallel-backend` | 3269 | 2874 | 141 |
| both | 2824 | 2430 | 150 |
| both + `-C codegen-units=256` | 2827 | 2551 | 148 |
| `-C opt-level=1` | 5552 | - | 64 |

`-Z time-passes` of the serial run: `LLVM_passes` 125 s of 145 s, type check +396 MB, monomorphization +1143 MB, metadata +1311 MB, codegen holds about 1.9 GB flat. The floor of the crate is its monomorphized generic dispatch (about 2.4 to 2.8 GB serial); no setting reaches the 1536 MB library budget.

## 4. Experiment: LTO off for the component row

`prototype-component` `lto: "thin"` to `"off"` (profile source of truth, fixtures updated, profile tests green), rebuilt the hub through the admitted path:

| | thin LTO | LTO off |
|---|---|---|
| hub final peak (RSS) | 5519 / 4450 MB | 2523 MB |
| hub final wall | 1694 / 1706 s | 465 s |
| wasm | 269.1 MB | 255.0 MB |
| core module functions | not counted | 722,320 (limit 1,000,000) |
| `semio_hub_sourcing` lib | 4080 / 4062 | 4273 (no local thin pass, front end unchanged) |

It brings the final far below the 4096 MB final budget. It was **reverted** (profile schema and both consumption fixture rows are back to `thin`): the row was set to thin by the GIS ticket (`🌙️08/☀️29/GIS-MAP-END-TO-END/component-function-count.md`) because the GIS closure of 1,390 units hit `functions count exceeds limit of 1000000` without LTO. Sourcing has 28 % head room (722k), cad (1041 crates) and puzzle (1192) are unmeasured, and a profile row applies to a whole cargo invocation (cargo has no per-package `lto`). Switching it globally could break those builds and is outside this ticket's lane. If the lead wants the 2.5 GB final now, the setting is exactly that one word in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json` (`prototype-component.lto`) plus two fixture rows in `🧪️tests/📥️consumption/🔣️.json`; a safe form needs a per-workspace row selected by the owner the plan already carries, after a function-count check of each hub component.

The built artifacts in the lane are the LTO-off hub final (255.0 MB) and the thin-LTO plugin finals (beams, slabs, windows); the hub wasm in `.cache/b/r/m/u/l/p2bd1dc/t-ce555540/wasm32-wasip2/prototype/semio_hub_sourcing_component.wasm` is an LTO-off build and was encoded without error, but was not run in a host here.

## 5. Carving state after this ticket

Added to `Policy.unitCeiling.carving` (measured thin-LTO baseline numbers, closed list, may only shrink): `semio_hub_sourcing_component` 5519 (`thin-lto-closure`), `semio_hub_sourcing` 4273 (`split-app-fleet`), `semio_s_artifact_sourcing_curation` 3653, `semio_s_artifact_sourcing_curation_inferences` 1546 (`split-families`), `semio_s_artifact_stdio_semio_{brep,table,document,value,kit}_member` 2285/1839/1770/1747/1616 (`split-store`). Unit-ceiling law test: 10 pass; profile consumption test: 26 pass.

Not carved (reasons above): no crate split was done. The next real carve is `semio_hub_sourcing`: split the editor and viewer app instantiations into two crates (each half of the 2.4 to 2.8 GB serial floor, so about 1.3 GB, under 1536) and make the enum a delegation crate; its serial back end alone is 2.8 GB. The final needs either the LTO-off row (verified here at 2.5 GB) or a smaller closure.

## 6. Minav curation tests

`minav-cargo.sh curation-lib ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation test --lib -p semio-s-artifact-sourcing-curation` through `repo cargo`: **108 passed, 0 failed** (baseline 107). Three earlier attempts died from concurrent peer activity, not from this change: warden cancellations under host pressure, `Prepared member roster changed`, and a peer's half-edited Rust source (`🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs`).
