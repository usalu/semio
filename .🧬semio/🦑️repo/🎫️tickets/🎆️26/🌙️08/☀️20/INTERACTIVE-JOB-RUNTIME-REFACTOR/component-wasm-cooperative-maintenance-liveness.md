# Component-Wasm Cooperative Maintenance Liveness

## Observed Runtime Boundary

The peer's fresh GIS log `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️gis-fresh-content-close-4.log` records real activation and five surfaces, then an accepted operation1 invocation plus Ephemeral output. At128-turn intervals through3968, command ingress is idle while reactor status remains more-work; command1 fails to settle within4096 turns. This is a distinct component-Wasm path, not covered by the native registered500us law or the bare Flow loader check.

## Exact Source Candidate

`plugin_step_live_cleanup` schedules a Maintenance closure when the cell is READY and, on wasm32, pumps the process pool once after successful submission. The QUEUED/RUNNING branch only returns true. The cooperative pool's one-round deficit scheduler uses Interactive cost8 and Maintenance weight1; the first Maintenance-only pump therefore returns no selected job while retaining the closure and deficit1. Without another pool pump, subsequent QUEUED turns cannot make progress. The typed worker is advanced only by maintenance stage0, while `plugin_continue_typed_operations` publishes only already-Publishing owners, so continued more-work responses do not themselves advance a stuck Worker owner.

This is a source-confirmed liveness gap consistent with the runtime symptom, not yet proof of the exact live GIS phase. Close/media branches may incidentally pump the same process pool and mask it. The correct next diagnostic is a bounded snapshot of maintenance status, pool pump/selection counts, retained lane counts and deficits, plus operation stage. No raised turn cap, busy loop, synthetic clock, or unrelated user command may be needed for progress.

## Clock Boundary

The component target's native/WASI clock is the shared optional microsecond source, projected to milliseconds only at `pool.pump`. The raw bare Flow generated initializer is a different startup path. Capture whether the component sees an actual clock sample together with the pool counters; never infer clock failure from idle ingress alone. The native host's owned-WASI clock helper remains source-ready but native-unexecuted, and the fresh Flow clock gate awaits peer package publication.

## Ownership

This lane owns trace/job, the narrow cooperative pool diagnostic and live-maintenance pump seam. DAG owns close-owner lifecycle; the demonstrator peer owns plugin export macros and shared generated outputs. No source or generated-output mutation was performed for the initial scout.

## Coherent Source And Small Native Gate

The shared async crate now mounts its actual cooperative pool under cfg(test), without replacing the native production pool. A neutral five-lane fixture and independent BigInt/Ajv oracle describe exact retained deficits and first selection. The first native invocation had two harness compile errors (missing test-only serde_json and an incorrect config constructor), so no semantic result was claimed for r1. The corrected source-binding law failed before production repair: `🧪️cooperative-maintenance-red-r2-native-2026-08-27.txt`,0PASS/1FAIL,one other test not run by nextest fail-fast. The actual pool law then passed separately in `🧪️cooperative-maintenance-deficit-r3-native-2026-08-27.txt`,1PASS/51skipped,.026s summary; its five DEBUG vectors include Maintenance8,Background4,UserVisible2,Timer3,Interactive1 host turns.

The live wasm-only helper now pumps once after successful Maintenance submission and once on every QUEUED/RUNNING revisit. Missing real clock marks the exact cell Fault. No synchronous drain, scheduling weight change, extra byte grant, synthetic clock, or turn-cap increase was introduced. The fixed six-lane snapshot exposes diagnostic copies only and returns None on mutex contention. Existing pool pump locking is unchanged and remains a separate review boundary.

Temporary `[DEBUG] cooperative-maintenance` output is limited to thirteen power-of-two observations through4096 helper calls per cell. Fields: instance,turn,generation,status before/after,actual maintenance-entry count,clock availability,pool pump/selection/no-selection counts,six queue lengths/deficits/selection counts. Phase bits are session1,outcome2,rejected4,closing8,faulted16,terminal32; None means the phase mutex was busy. This diagnostic is not an authority and does not claim a timing bound for host stderr delivery.

`🧪️cooperative-maintenance-green-r4-native-2026-08-27.txt`:3PASS/0FAIL,50skipped,.190s summary. Scope: actual cooperative five-vector execution/deficit/snapshot law; nonblocking snapshot contention preserving the queued closure; exact source binding plus five hostile mutations. The plugin wasm-only helper itself has not been compiled or executed. Fresh GIS phase attribution and successful operation publication therefore remain unproved until the peer consumes this source in its actual component build. No heavy native/plugin/Wasm build was launched here.

`🧪️cooperative-async-full-r5-native-2026-08-27.txt`:full async53PASS/0FAIL,0skipped,1.333s summary. This regression includes the three new cooperative laws but is still a native test build, not a consumed component-Wasm proof.

The canonical registry generator refreshed the launch catalog from the authoritative seed, adding `⚖️gate⏳️async🔁️cooperative-maintenance`. Its successful log is `🧪️cooperative-maintenance-launch-r1-2026-08-27.txt`. No installed plugin-module or Flow package output was written.

## Exact Changed Source Paths

- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`: cooperative test mount, fixed diagnostic counters/snapshot; scheduling weights and one-round selection unchanged.
- `🧰️framework/🔨️modules/⏳️async/⏱️cooperative/🧪️fixture.json`, `🧬️schema.json`, `🧪️component.rs`: neutral five-lane vectors, strict schema, three focused native laws.
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml`: existing serde_json library as test-only dependency.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`: wasm-only per-cell probe counters and exact one-pump queued continuation helper; no close-owner or export-macro rewrite.
- `📜️script.ts`: seven independent schema/arithmetic checks, added to existing source selftests; no broad proof bypass.
- `.vscode/🧩️launch.seed.jsonc`: focused existing async task route; generated launch/catalog refreshed through the canonical generator.

The three focused native laws do not instantiate the plugin runtime; the host binding is explicitly a source test. The fresh consumed component, operation publication, native host-WASI clock, and strict cold-close gates remain separate pending boundaries. The master target and all logs were retained.
