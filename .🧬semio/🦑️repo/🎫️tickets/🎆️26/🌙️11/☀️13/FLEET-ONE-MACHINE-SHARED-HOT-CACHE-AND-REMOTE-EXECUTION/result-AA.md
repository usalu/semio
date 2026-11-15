# Package AA: plugin payload pipeline computed once, correct when shared (2026-11-14)

## Verdicts
- Component payload path/machine independence: PROVEN for the fixture wasm32-wasip2 component through the real rustc wrapper, real wasm profile, real jco and wasm-opt, at three roots with three Cargo homes (test `materialization/🧪️tests/🪞️two-checkout`, 5 pass, about 13 s). Bytes equal, no root/home/sysroot string in any output, store receipts equal, fleet content restore byte-identical.
- Real bug found and fixed: the unit-cache wrapper (`🧭️remap`, changed 06:40 today) added Apple `-Wl,-oso_prefix`/`-install_name` to every cdylib on a macOS host, including wasm32 targets; `wasm-component-ld` rejects `-W`, so every wasm component link through the wrapper failed. Now only for macOS targets (`appleLinkV1`), test added.
- Stage fleet tier built (`⚡️caching/☁️stage-fleet`, schema first, EN+DE texts): describe, transpile, optimize use the fleet cache (key = sha256(namespace, os-arch, stage key digest); claims/await/heartbeat single-flight; bounded waits; any fault computes locally; manifest+digest verified, forged entry = miss; warm local hits offered once). 13 tests incl. racing machines compute once, dead holder takeover, down/denied/read-only/stalling/corrupt server, two languages. Server is started by the test on a free port, temp store, test token.
- Slot supersession: new slot domain `stage-output`; remote via slot header/claimSlot; local via `.slots` index with grace 10 min using `classifySlotsV1` (test with fake clock).
- Leak scan of the wrapper now also covers CARGO_HOME, RUSTUP_HOME and resolved spellings (audit item l).
- Guard: component target stays unguarded (now proven, test asserts it). The `wasm` row stays: those targets are the browser wasm-pack/wasm-bindgen pipeline, a different pipeline with no proof (tools not installed here); reason text corrected (EN/DE).
- Negative control in the proof: a dependency from an absolute directory source gives different bytes per checkout, so the comparison can fail.

## Findings (code/measurement)
- cargo with trim-paths=object (the build's profile) gives identical wasm at different roots, also with debug=2; wasm profiles use debug 0. jco 1.39 output identical across roots.
- Component receipt and store receipt are closed digest/relative-path schemas (no path, no time). `provenanceSha256` includes artifacts of host units (proc-macros, build scripts) that are not byte reproducible, so two independent builds give different provenance for identical wasm; consistent only because units are shared and the Nx output is restored as one set.
- Cargo hashes the absolute path of directory/outside-workspace path sources into crate metadata (one byte differed): keep sources inside the workspace, registry or git.

## Changed
stage-cache run() takes optional `slot`; component-pass option `decorateStageCache`; driver-stages applies `stageFleetDecoratorV1`; commands export `transpileStageRequestV1`/`optimizeStageRequestV1` and pass slots; component-stage passes slot; slots, remote-guard, remap, script.ts as above. Lists in landings.md.

## NOT verified
- No real plugin build through `repo` (47 min cold, load 24); stage tier in the real driver is covered only by the fixture and unit tests. Second-root fleet hit of a real plugin not shown.
- Probe stage (private, needs repo runtime) not exercised by test; its slot is passed in code only. Real invocation control (the test bypasses it by calling the producers with the same request builders).
- Registry crates with different CARGO_HOME at component level (no network; unit level measured by B). Linux/Windows not run. Provenance paths outside the four roots stay absolute. Unit-cache `obtainFleetUnitV1` not yet migrated onto the generic flight. Suites JSON not updated for new test files. Test python and Binaryen steps skip with a message when missing.
