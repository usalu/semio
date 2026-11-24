# Flow hub cdylib: jetsam kill at 3.33 GiB

Unit `semio-hub-flow-component@semio_hub_flow_component (cdylib)` of `repo run semio-hub-flow-component:component -c prototype --exclude-dependencies` (log `🗑️generated/native-component-flow-hub-14.txt`, line 15763) was SIGKILLed by jetsam after 1877 s with 892 other units done. The broker had forecast 4381 MB for `semio_hub_flow` (line 15585) and held it back for 152 s waiting for free memory.

## What the unit is

- Profile is the `prototype-component` row of `📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json`: `opt-level=s`, `lto=thin`, `codegen-units=16`, `debug=0`, `incremental=false`, `strip=symbols`, `debug-assertions` and `overflow-checks` on, `panic` unwind (inert on wasip2). `[target.wasm32-wasip2]` in `.cargo/config.toml` adds `-Z threads=0`, `--max-memory=536870912` (a wasm limit, not an RSS cap) and an 8 MiB shadow stack. wasm-ld and the component encoder run after rustc and do not own the 3.33 GiB.
- The row is thin LTO on purpose: ticket `26/08/29/GIS-MAP-END-TO-END`, `component-function-count.md`, recorded the GIS final failing `functions count exceeds limit of 1000000` without LTO. Dropping LTO for components is therefore not a safe lever, and Cargo has no per-package `lto`.
- The leaf is `semio_plugin::plugin_exports!(…, semio_hub_flow::plugin::plugin, semio_hub_flow::plugin::FlowApps)`. The macro instantiates `PluginRuntime<FlowApps>` and everything generic over `PluginApp` (reactor turn, dispatch, dirty-surface render, instance registry, lifecycle) inside the cdylib, so the leaf has real codegen work on top of the ThinLTO import of the whole closure. Moving that out needs a non-generic runtime boundary in `semio-plugin`; it is a separate refactor, it would re-key every plugin and is not needed for the 3 GiB target.
- `-C share-generics` is already the default at `opt-level=s` (rustc enables it for levels 0, 1, s, z), so it is not a missing flag here.

## Root cause

A cdylib final under ThinLTO holds the bitcode of the whole 892-unit closure in the ThinLTO coordinator for its whole life. Every additional codegen or ThinLTO worker adds a parsed module with its imported function bodies on top of that floor. The build scheduler (`🎟️admission/🦀️.rs`) already bounds the front end of a cdylib final to one thread (`frontend_tokens`) but, since the relaxation documented in `bounded_threads`/`serial_backend`, appended `-Z no-parallel-backend` only when the run was admitted exactly ONE job. The run that died had been granted 3 to 4 jobs (`[capacity] run: grown from 3 to 4 of 8 job(s)`), so rustc handed the final's backend the idle jobserver tokens and ran several LTO workers at once. Together with other agents compiling on a 16 GiB host, jetsam picked the largest process at 3.33 GiB, which is below the 4381 MB forecast: the kill is host pressure acting on a footprint that grows with the grant.

Evidence from earlier tickets that the serial back end is the right floor: `26/11/17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/💥️demonstrator-oom.md` (front end 1 thread 2.9 GB versus 4 threads 5.4 GB; 5.87 GB final killed at a 3-token concurrent LTO phase) and `🔬️final-crate-cost.md` (the serial Energy final over a 1438-unit closure peaked at 2.78 GiB).

## Change

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs`: `serial_backend` now appends `-Z no-parallel-backend` to every `cdylib` final that carries any grant (`tokens >= 1`) instead of only to a one-job grant. Libraries and runs without a grant are untouched. The flag is added at the process that starts rustc, so Cargo fingerprints, unit cache keys and the artifact are unchanged: no dependency is recompiled and no profile row changes. Output is identical, only the schedule differs. Cost: the final no longer uses idle cores; the Energy final shows the serial wall is the same order as before (3155 s) because LTO there is already one-core-bound.

Files:

- `🎟️admission/🦀️.rs`: `serial_backend` condition, its docstring, unit test renamed `serializes_the_back_end_of_a_granted_cdylib_final` (13 tests pass with the pinned nightly).
- `🎟️admission/🧪️tests/🧵️frontend-threads/🐍️.py`: independent oracle serial for every granted cdylib final.
- `🎟️admission/🧪️tests/🧵️frontend-threads/🔣️.json`: `cdylib-final-separate`, `cdylib-final-joined` and `cdylib-two-jobs` now expect the appended flag, notes updated; corpus agrees with the oracle on all cases.
- `🎟️admission/🧪️tests/🧵️frontend-threads/🟦️.ts`: the CPU-ratio test now asserts a 4-token grant stays on one thread (it needs a rustup `+toolchain` proxy; it exits 127 inside this sandbox before and after the change, the corpus and scheduler-binary tests pass: 43 expectations).

## Measurement

Before: SIGKILL at 3.33 GiB resident (`ended by SIGKILL (exit 137) … had reached 3.33 GiB`), forecast 4381 MB, grant 3 to 4 jobs.

After: NOT measured. Standalone `repo cargo --store=shared --configuration=prototype -- build … --lib -j 1` opened a fresh lane (`pe9ac47`), waited 25 min inside the driver with no cargo child and was killed by me; the lane `p7d18b1` of the failed run no longer exists, so the 892 dependency units are not reusable. Two `repo run semio-hub-flow-component:component -c prototype --exclude-dependencies` attempts recompiled the closure at 4 to 6 units per minute because three sessions compete for 16 GiB (memory admission, `repo capacity status` showed 2 to 3 running grants of 2 to 4 GB each); both were stopped by me after about 25 and 18 minutes at 360 and 68 units. Nothing of theirs was touched.

To measure: run `repo run semio-hub-flow-component:component -c prototype --exclude-dependencies` once with the new scheduler (the driver rebuilds `rustc-wrapper-*` from the changed `🎟️admission/🦀️.rs` at start) and read the peak resident of the cdylib unit from `.cache/b/r/m/components/runs/<run>/r/report.md`, as the Energy final was read (`2.78 GiB peak resident`). Expected: about 2.5 to 3 GiB (closure floor of the serial back end, same order as Energy). If it still exceeds 3 GiB, the next levers in order are: (1) a non-generic entry point so `PluginRuntime<FlowApps>` and the reactor are instantiated in `semio-hub-flow` instead of the leaf (shrinks the leaf's own codegen, re-keys the hub closure only); (2) `prototype-component` `debug-assertions` off (smaller IR in all 892 units, re-keys everything); (3) a size-gated `lto` (not available per package in Cargo).
