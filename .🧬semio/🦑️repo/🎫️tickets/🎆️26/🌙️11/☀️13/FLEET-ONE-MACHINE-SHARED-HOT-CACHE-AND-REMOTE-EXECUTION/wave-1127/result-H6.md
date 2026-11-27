# Wave 1127 H6: units over the 1536 MB library ceiling (16 GB workers)

## Sources
`S/origins/*/memory.json` (`lastAlarms`, last 8 per host, 14 origins read), coordinator ledger `.cache/b/r/m/components/build-telemetry/units*.jsonl` (old, 7 units above ceiling, all already listed). Full warden logs of the workers are not in the store; the list below is therefore the alarms still retained (a lower bound).

## Units over ceiling (peak MB, host)
Unlisted before this lane: process3d_harness host 2033 (m01, cancelled), process3d_host host 1750 (m01, per coordinator), stdio_pdf_model_graphics host 1577 (m02) / wasm 2059 (m12), puzzle_5d_testkit host 1599 (m04), sourcing_curation_history_inputs host 1549 (m06), _conflicts host 1713 (m06), gis_gismap_snapshot host 1651 (m07), gis_gismap final 4584 (m07, cancelled), puzzle_2d_retained final 4366 / puzzle_2d_v1 final 4227 (m04, cancelled), plugin_actor_input_decoder wasm 2280 (m11 m12) / host 1558 (m13), plugin_actor_input_wire wasm 2269 (m11 m12 m13), hub_procedural wasm 1757 (m11), hub_stdio_pdf_1_7_viewers host 1550 / editors host 1568 (m12), hub_procedural_generation2d_viewer_app wasm 2152 (m12) / editor_app wasm 1911 (m14), procedural_2d_mode_edit wasm 1606 (m14), bim_model_kit host 1538 (m15, cancelled).
Listed but above their entry: os_mcp_plugin_runtime host 3127 (entry 2711/2980), os_flow_host_core wasm 1716 (entry 1598/1700).

Cancel pattern: `ceiling` cancels (m01, m07, m15, m04) happen only when the host is also in danger; on calm hosts the ceiling only reports. Other cancels (m03 19, m10 38, m08, m14) are `danger` cancels of cargo/stage/local-unit/pool-child with 2.7-3.4 GB available, swap 5-9 GB on m03 m04 m07 m08 m10: host-wide pressure from several concurrent subjects, not one over-ceiling unit.

## Done
`🎟️admission/🧬️schema/🔣️.json` `Policy.unitCeiling.carving`: 21 new entries (measured = observed peak, budget = ceil(1.1 x measured), carving action named per unit) and two raised entries (os_mcp_plugin_runtime 3127/3441, os_flow_host_core 1716/1888). Effect: the warden no longer cancels these as unlisted units while the host is healthy, and cancels them only above their own budget. The list is a declaration of debt, not a carve.
Test: `repo test-file …/🧢️unit-ceiling/🟦️.ts` 11 pass (all policy laws: budget >= measured, <= +10 %, unique, family overlap).

## Not done (unverified)
- No crate was split and no peak RSS re-measured: the coordinator was at load 60-67, swap 6.7/8 GB (gate: load < 40), so no cargo run was started. The `repo cargo check` of carved crates is therefore not proven; the test only proves the policy is consistent.
- Source of process3d_host (1.5k lines, 5 modules) and harness (446 lines) is small: the peak comes from generic/derive instantiation over Process3dSnapshot/Mutation, so a carve means one crate per module (snapshot-decode, mutation-decode, envelope, genesis) and measuring each; candidates only.
- Cancelled work is requeued only for danger cancels (`warden-cancelled` refusal); a `ceiling` cancel is deliberately not retried (retry reproduces it). No admission code was changed.
- Next: with load < 40, measure each listed unit with `repo cargo check -p <crate>` plus telemetry, carve the largest (hub_procedural_generation2d apps, actor_input decoder/wire, process3d host/harness), and delete entries that fit.

## Addendum: carve attempt (after coordinator message)
- Load stayed 50-67 for 30 min; per the addendum the gate was memory only (free 69 %, swap free 1.5 GB), run niced.
- First call `repo cargo … check -p semio-s-artifact-process-process3d-host` failed after 15 min: the crate lives in the standalone workspace `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d`, so the package id must be given as `--manifest-path <…/🏠️host/🧰️owned/📦️packages/🦀️rust/Cargo.toml>` (works; -p does not).
- Baseline run (`nice -n 10 repo cargo --store=shared --configuration=prototype -- check --manifest-path … --lib`, pid 11223, log in the lane scratchpad `h6-host-before.log`) is cold: 805 dependency units, 163 checked in 43 min; the repo capacity gate holds most of them (`cargo-growth: swap is crowded`, 1-6 GB spare above the 4 GB floor). process3d_host itself had not started, so NO before/after peak was measured and NO crate was carved.
- Peak by ps is not usable (the scheduler wrapper hides the rustc child); the measurement source is `.cache/b/r/m/components/build-telemetry/units-darwin-arm64.jsonl` (`peakRssBytes` per unit key) once the unit has run.
- Source findings for the carve (not applied, unmeasured): host = 5 modules with disjoint dependency needs (snapshot-decode: model+job; mutation-decode: mutations+job; envelope: model+io+value; genesis: store-lifecycle+operation-wire; decode-owner-catalog: job) and a `#[cfg(test)]` retained-laws module (461 lines) that pulls dev-dependencies `semio-plugin[artifact-app-testing]`, kernel[protocol-laws,mutation-testing]: if the ceiling unit is the test build, moving that law into its own test crate is the cheapest cut. harness (446 lines) has 44 dependencies including the editor crate: it is a fixture over the whole editor, its peak is the editor's metadata, so splitting its file does nothing; the cut is a thinner fixture per editor test target.
- Carving entries were NOT removed (nothing fixed).
- Left running: the baseline cargo (pid 11223) keeps warming the shared cache; re-running the same command later finishes the dependency part, then read the process3d_host peak from the telemetry ledger, apply the test-crate split, re-run, compare.
