# Hub component final: where the memory goes (measured 2026-11-24)

Unit: `semio-hub-cad-component` (`cdylib`, `wasm32-wasip2`, profile `release-wasm`: opt-level s, thin LTO, 16 codegen units), built on the coordinator (32 GiB) through `repo cargo --store=shared --configuration=release --parallelism=2` with `carve-C3-build.sh`. The script samples the compiler child of the build wrapper every 2 s (resident size from `ps`, charged size from `footprint -p`) and takes a one-second stack sample every 90 s.

## Result

| Quantity | Value |
|---|---|
| Peak charged memory (`phys_footprint`) of the final's `rustc` | **8,036 MB** |
| Charged memory during the thin-LTO phase (3 h 08 min under coordinator load) | 6,770 to 8,036 MB, never below 6,000 MB |
| Peak resident memory of the same process | 2,198 MB |
| Charged memory before thin LTO starts (the final's own codegen) | 1,650 to 1,900 MB |
| Size of the 953 dependency libraries the final links (`.rlib`, bitcode) | 6,259 MB |
| Output component | 269,180,384 bytes |

Earlier numbers for this unit were resident sizes: the learned peak file said 3,218 MB and later 2,880 MB, and reports of other tickets quote 2.78 to 3.33 GiB for finals. They are lower bounds. The OS compresses and swaps the compiler's pages under pressure, so resident stays small while the charge (which is what jetsam kills on) stays at 7 to 8 GB. The 6,550 MB of the play telemetry of 2026-11-22 was taken on a calm host, where resident and charged agree.

## Which stage owns it

Stack samples at the step from 1.65 GB (t=6387 s) to 6.64 GB (t=6522 s) are in `LLVMRustCreateThinLTOData` (`run_thin_lto`, `computeImportForModule`): the start of thin LTO. From then to the end the samples are in `thin_lto_work` (`LLVMRustPrepareThinLTOImport`, `LLVMRustParseBitcodeForLTO`, `LLVMRustOptimize`, `LLVMRustWriteOutputFile`) and the charge stays flat.

The step (about 5.0 GB in two minutes) and the plateau match the 6,259 MB of dependency bitcode: at thin-LTO start the compiler holds the bitcode of the whole dependency closure in anonymous memory for the rest of its life. The back end was already serial (`-Z no-parallel-backend`, added by ticket 26/09/09), so worker parallelism is not what holds the memory. The floor is the size of the closure.

## What this means for the 4,096 MB ceiling

No compiler setting the repository declares brings this unit under 4,096 MB:

- Thin LTO off is not available: component finals without LTO exceed the 1,000,000 function limit (ticket 26/08/29 `component-function-count.md`).
- Codegen units and back-end parallelism do not change the size of the closure's bitcode; the back end is already serial.
- Fat LTO holds the same bitcode merged into one module and is larger.

The reduction that works is a smaller closure. The largest libraries in it:

| MB | Library |
|---|---|
| 272 | `semio_hub_cad` |
| 229 | `semio_s_artifact_stdio_dwg_snapshot_objects` |
| 205 | `semio_plugin_actor_input` |
| 190 | `semio_s_artifact_stdio_binary_io_registry` |
| 165 | `semio_s_artifact_stdio_semio_presentation_member` |
| 160 | `semio_s_artifact_stdio_txt_tests` |
| 153 | `semio_s_artifact_stdio_semio_model_member` |
| 145 | `semio_s_artifact_stdio_semio_brep_member` |
| 140 | `semio_s_artifact_stdio_semio_graph_editor` |
| 104 | `semio_s_artifact_stdio_semio_document_member` |

`semio_s_artifact_stdio_txt_tests` is a tests crate inside a release component closure (160 MB of bitcode). The dwg snapshot objects crate is the same one that peaks at 4.9 GB when compiled on its own.

Not measured: the same run for `semio-hub-puzzle-component`. One run took 3 to 5 hours on the coordinator, and the mechanism is the closure size, which puzzle shares (1,192 crates). The ledger will carry its charged peak after its next build (see below).

## Changes made

1. `🎟️admission/🟦️.ts`: a completed unit is recorded with the larger of its peak resident bytes and its largest charged sample (`unitPeakBytesV1`; the build wrapper already sent a charged sample every second, the broker only kept the last one). The ledger, the learned peak and the forecast of every unit now follow the charge. For the cad final the next forecast is about 8 GB instead of 2.9 GB, so the broker holds it until that much is measured available instead of starting it into a worker that cannot carry it. Test `🎟️admission/🧪️tests/🗜️charged-peak/` (corpus, Python oracle, broker end to end): 2 pass.
2. `🎟️admission/🧬️schema/🔣️.json`: the carving entry of `semio_hub_cad_component` carries the measured 8,036 MB (was 6,550); `ObservationV1.peakRssBytes` is described as the larger of resident and charged.
3. `🎟️admission/🧪️tests/🧯️available/🎫️broker/🟦️.ts`: history rows carry `lines`, which another session's new forecast rule (`forecastHistoryV1`) requires; 7 pass again.

The cad and puzzle entries stay on the carving list. They leave it when the closure shrinks, not through a setting.

## Runs

| Label | Wall | Outcome |
|---|---|---|
| thin (3 attempts) | 177 s, 15,868 s, 4,061 s | two failed on other sessions' in-flight edits, third built; sampler followed the wrong process |
| passes | 333 s | refused: extra compiler flags belong to the repository's admission |
| local | 3,075 s | built; sampler followed the wrapper, not the compiler |
| local2 | 840 s | stopped by a syntax error in another session's file |
| local3 | 17,993 s | final measured as above; the command ended with exit 1 afterwards on a source boundary of `semio-s-artifact-stdio-xml-snapshot` (4,073 of 4,000 active lines, another session's crate) |
