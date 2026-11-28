# Norm Base Hub: Unit Ceiling and Final Link (2026-11-27)

## Cause

1. `semio-hub-norm` (lib, wasm32-wasip2, prototype) peaked at 3,395 MB (3,222 MB in the attempt before). The crate is 6.5 KB of source: one `dyn_enum_close!` fleet of eight `VcsArtifactApp<EditorApp<X>>` / `VcsArtifactApp<ViewerApp<X>>` instantiations (DIN 4108, DIN 16798, ISO 16757, VDI 3805, each editor and viewer) and one `PluginBuilder` chain. The memory is monomorphization: the dispatch of the whole instance runtime (`PluginApp` methods, `with_registry`, factories, codecs) instantiated eight times in one compilation unit, about 400 MB per app. `semio-hub-process` (two apps) and `semio-hub-norm-eurocode-structural` (carved already) stay below the 1,536 MB library budget, which fits the per-app estimate.
2. The final `semio-hub-norm-component` was cancelled by the **host warden**, not by the admission broker and not by a grant: `alarms.jsonl` of the warden (`~/Library/Caches/semio/host-capacity/warden`) holds `action cancel, reason ceiling, kind local-unit, megabytes 4107, ceiling {unit semio_hub_norm_component|wasm32-wasip2, kind final, megabytes 4096, listed false}` at pressure `warn`. The `local-unit` kind signals SIGTERM (exit 143); the driver prints "ended by SIGTERM ... 4.01 GiB". The cap is the static final budget of the 16g class (4,096 MB) because the unit is not on the carving list.
3. Why the final needs 4+ GB: it was linked with the `prototype-component` row (`opt-level s`, 16 codegen units, **`lto=thin`**, serial back end), and the thin-LTO stage holds the bitcode of the whole 817-unit closure. The unit history already held a completed run at 4,176 MiB (198 s) and the learned peak file of the final says 4,775 MB (`b4f1b13ac4_semio_hub_norm_component.peak`), so every attempt was a coin toss against the ceiling and the host pressure. Same mechanism as the sourcing hub final (`🪚️hub-component-carving.md` of SOURCING-END-TO-END: 5,519 MB thin versus 2,523 MB lean).
4. The forecast never learned from the kill: `recordCompletedUnit` of the component pass dropped every unit with `exit != 0`, so a final cancelled at its ceiling left no observation and the next admission sized it from stale evidence.

## Fixes

### (1) Carve `semio-hub-norm` along its domain taxonomy (precedent: `norm/🧩️extensions/📓️eurocode-structural/🗃️apps/*`)

Eight new app crates under `🌎️hub/🧩️compositions/📕️norm/🗃️apps/<family>/<role>/📦️packages/🦀️rust`, one instantiation of the instance runtime each, mounted by the lib of `<role>/🦀️.rs`:

| crate | enum |
|---|---|
| `semio-hub-norm-din4108-editor` / `-viewer` (`🧱️din4108`) | `Din4108EditorApp` / `Din4108ViewerApp` |
| `semio-hub-norm-din16798-editor` / `-viewer` (`🌬️din16798`) | `Din16798EditorApp` / `Din16798ViewerApp` |
| `semio-hub-norm-iso16757-editor` / `-viewer` (`📇️iso16757`) | `Iso16757EditorApp` / `Iso16757ViewerApp` |
| `semio-hub-norm-vdi3805-editor` / `-viewer` (`🏭️vdi3805`) | `Vdi3805EditorApp` / `Vdi3805ViewerApp` |

`🦀️.rs` of the hub: `NormApps` now wraps the eight app enums; `apps_from_instance!` routes each `VcsArtifactApp<..>` instance into the crate that compiles it (`PluginBuilder::editor/viewer` still gets `NormApps: From<instance>`). Registration, artifact declarations, activations and requests are unchanged. No caller changed: `semio_hub_norm::plugin` and `NormApps` keep their names (the component entry, `🧩️component/🦀️.rs`, is untouched).

Updated: `📦️packages/🦀️rust/Cargo.toml` of the hub (eight dependencies), `🌎️hub/Cargo.toml` (eight workspace members, eight workspace dependencies), `🌎️hub/Cargo.lock` (eight packages, eight hub dependencies), `📋️build-owners.json` (eight owner rows).

### (2) The final

- **Policy (root fix).** `semio-hub-norm-component` joins `Protocol.leanCompositions.prototype` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json`, next to the bim, draw, procedural and sourcing finals. Its prototype pass then resolves to `component-lean` (`lto off`, `opt-level s`, 16 codegen units, no debug info, serial back end): no thin-LTO bitcode of the closure in memory, `wasm-ld` only gc-sections. The component rows already have `debug 0` and `strip symbols`, so debug info was not a factor. The lean row also gives the final its own profile key, so the stale learned 4,775 MB of the thin row no longer applies and the cold component forecast is used. Test: `🧪️tests/🟦️.ts` of the profile owner, "the norm base component final links under the lean prototype row" (single `-p` of the norm base final resolves to `component-lean`; the energy final and a two-package request keep `prototype-component`).
- **Forecast and cap learn from the kill.** New `unitCeilingCancelledV1` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`: a unit with exit 143 whose peak is above its runtime ceiling (budget of its kind, or its carving budget) is a ceiling cancel. `unitObservationV1` and `localUnitObservationV1` now record such a unit at the peak it reached (a lower bound), and `recordCompletedUnit` in `🧩️component-pass/🟦️.ts` no longer drops failed units before asking (every other failed unit and every cached unit still yields no observation). The next admission then predicts above the ceiling and `unitBeyondHostV1` queues the unit for a larger machine instead of killing it at 4,096 MB again. The ceiling stays a law (the carving list is the only place that raises a cap, and it may only shrink); a measured final above 4,096 MB after the lean row is declared there with its measured peak, not learned silently. Test: `🧪️tests/🪓️ceiling-cancel/` (corpus, schema, independent Python oracle `🐍️.py`, 10 cases: cancelled above the final budget, exactly at it, library above its budget, SIGKILL, SIGTERM below budget, carved final above and below its carving budget, success, cached, compile error), 3 tests pass; the existing `🧢️unit-ceiling` (11) and admission (18) suites pass.

## Not done / follow-up for the coordinator

- The changed profile policy changes the lean-list part of the profile plan digest; the first norm base publication after this builds the final with the lean row (no stale thin-LTO artifact is reused).
- If the lean final still measures above 4,096 MB, add a `Policy.unitCeiling.carving` entry for `semio_hub_norm_component|wasm32-wasip2` (action `serial-backend`, budget at most 110 percent of the measured peak) in the admission schema; none was added because nothing measured justifies it (the sourcing lean final measured 2,523 MB for a larger closure).

## Measurement

See the table below (one worker, through `repo capacity run`, `repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 build --offline --locked -p … --target wasm32-wasip2`, script `hub-norm-measure.sh`, logs in `🗑️generated/hub-norm-unit-ceiling`). Peaks are `peakRssBytes` of the `miss` records in `.cache/b/r/m/u/x/*.jsonl` (`en1995-peak.py <crate>`).

### Carved app crates (measured, one worker, `repo capacity run --memory=2663 --cpu=1`, exit 0)

One `repo cargo ... build -p <the eight app crates> --target wasm32-wasip2` (`hub-norm-measure.sh apps2`, 814 units, 776 compiled here, cold lane): the lane had to be rebuilt from the framework up because `repo cargo` (row `prototype-wasm`, opt-level 1) shares nothing with the component driver lane. Library budget 1,536 MB.

| unit | peak (MiB) | wall |
|---|---|---|
| `semio_hub_norm_iso16757_editor` | 1,381 | 53 s |
| `semio_hub_norm_vdi3805_viewer` | 1,362 | 54 s |
| `semio_hub_norm_din4108_viewer` | 1,359 | 54 s |
| `semio_hub_norm_din16798_editor` | 1,344 | 53 s |
| `semio_hub_norm_vdi3805_editor` | 1,290 | 46 s |
| `semio_hub_norm_din16798_viewer` | 1,271 | 54 s |
| `semio_hub_norm_iso16757_viewer` | 1,271 | 55 s |
| `semio_hub_norm_din4108_editor` | 1,239 | 49 s |

All eight are below 1,536 MB. The uncarved crate measured 3,395 MB (component driver, `opt-level s`); the apps cost about 1.3 GB each because the instance runtime instantiation is mostly per app, so nothing is saved by merging pairs, and one fleet crate per family would be about 2.6 GB (over budget): the per-role split is the smallest carve that clears the ceiling.

### Remaining hub lib (`semio-hub-norm`, now builder and routing only) and the final: NOT measured

The follow-up `hub-norm-measure.sh hub 2663 semio-hub-norm` (log `🗑️generated/hub-norm-unit-ceiling/hub.log`, status `hub.status`) was queued behind the capacity gate for more than 50 minutes, because a live peer job (pid 75202, a bun script) holds a growth grant `tjg63pHpDXhxolbJ5uoVWQ` written in a newer on-disk format that this reader cannot parse, so the gate "stays queued and admits nothing until it retires" (also `repo capacity status` throws `Unrecognized host capacity grant`). It is still queued and will compile on its own once that grant retires; read the peak with `python3 en1995-peak.py <hub.since> semio_hub_norm`. The residual hub lib holds eight `PluginBuilder::editor/viewer/..._mutation_roster` instantiations (generic over the artifact crates, not over the instance runtime) and no `VcsArtifactApp` dispatch; expect it well under the 3,395 MB of the uncarved crate, but this is an estimate until the log lands. If it is still above 1,536 MB, the next carve is to hoist `.editor::<E>`/`.viewer::<V>` registration into a non-generic function of each app crate (needs a `PluginBuilder` type free of the fleet enum).
The final was not built (the coordinator's chain does that); its expected peak under the lean row is about 2.5 GB (sourcing hub final, larger closure, 2,523 MB) against the 4,096 MB ceiling.

## Other findings fixed on the way

- The eight new app crates, and the six already existing `semio-hub-norm-eurocode-structural-en199{2,3,4}-{editor,viewer}` crates (copied from the same template, never built before), declared only `semio-plugin`, `semio-framework-dispatch-macros` and the artifact crate; `dyn_enum_close!` expands to paths of `semio_framework`, `semio_framework_job`, `semio_framework_value` and `semio_os_kernel` (109 E0433 errors on the first measurement build). All fourteen manifests and their `Cargo.lock` entries now declare those four dependencies. Without this the structural step of `publish-chain.sh` would have failed too.

## Files

- New: `🌎️hub/🧩️compositions/📕️norm/🗃️apps/{🧱️din4108,🌬️din16798,📇️iso16757,🏭️vdi3805}/{✏️editor,👁️viewer}/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}`; `🧪️tests/🪓️ceiling-cancel/{🟦️.ts,🐍️.py,🔣️.json,🧬️schema/🔣️.json}` of the admission owner; `hub-norm-measure.sh` (ticket).
- Updated: `🌎️hub/🧩️compositions/📕️norm/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, `🌎️hub/Cargo.toml`, `🌎️hub/Cargo.lock`, `📋️build-owners.json`, the six structural app `Cargo.toml`, the profile schema `🔣️.json` and its test, admission `🟦️.ts`, component-pass `🟦️.ts`.
- Generated (delete when the queued hub build has finished): `🗑️generated/hub-norm-unit-ceiling`.
