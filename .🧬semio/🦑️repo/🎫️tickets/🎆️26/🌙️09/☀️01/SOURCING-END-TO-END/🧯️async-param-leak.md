# 🧯️ Async export parameter leak: sync stage exports

Ticket `26/09/01/SOURCING-END-TO-END`, 2026-11-23. Follow-up of `🧠️guest-memory.md` section 3.

## Fix

wit-bindgen never frees the indirect parameter area of an async-lifted export (wit-bindgen#1757); sync exports are freed. Three exports exceeded 16 flat values, so they became SYNC stage exports:

- `reactor.stage-actor-context-budget(budget)` (19 flats), consumed by `admit-actor-context`, `admit-actor-input`, `close-actor-context`, `close-actor-output`, `poll`, `checkpoint`, `restore`, `describe` and all 12 `codec` calls.
- `jobs.stage-job-budget(budget)`, consumed by `start-job` ... `close-jobs`, `publish/read/acknowledge-actor-failure`.
- `reactor.stage-cold-pair-page(page)` (26 flats): found by the new law, it was a second live leak (async, one call per cold-pair page).

All async exports now take at most 16 flat values. Law: the host stages immediately before each consuming call; the guest takes the staged value at the synchronous entry; nothing staged gives an empty cancelled budget, which consumers refuse with a typed fault.

## Changed files (under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` unless noted)

Schema: `🧬️schema/📜️reactor.wit`, `📜️jobs.wit`, `📜️checkpoint.wit`, `📜️describe.wit`, `📜️codec.wit`, `📜️.wit` (comment).
Guest: `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` (staging cells), `📣️macros/⚛️wp4-reactor/🦀️.rs` (sync stage exports, async exports read the staged budget).
Wasmtime host: `🖥️host/🔗️wit-bindings/🦀️.rs` and `🖨️describe/🧪️tests/🔬️actor-bindings/🦀️.rs` (bindgen `exports: {... async | store}` for the three stages, required for sync exports on a concurrency store), `🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs`, `🖥️host/🎭️guest-runtime/🐎️wasmtime-runtime/🦀️.rs`, `🖥️host/🚪️io/🪶️snapshot/🦀️.rs`, `🖨️describe/🛂️descriptor-emission/🦀️.rs` (stage, then call, inside one `run_concurrent`).
TS: `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts` (new `originalStagedGuest`, embedded into `originalOperationWorkerSource`), its two re-export indexes (`🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts`, `📦️packages/🟦️typescript/🎭️actor/🎟️retained-turn/🟦️.ts`), `🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`, `🌐️browser-bundle/🧩️actor-module/🟦️.ts`, `🌐️browser-bundle/🛂️descriptor/🟦️.ts` (`PLUGIN_DESCRIPTOR_PROBE_SOURCE` stages the budget, then `describe()`). The budget-taking TS domain surface and its fakes are unchanged; `originalStagedGuest` maps it to stage-then-call at the jco boundary.
Tests: `🖥️host/🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs`, new `🖥️host/🪞️schema-parity/🧪️tests/🧮️canonical-abi-flat-params/🦀️.rs` (mounted in `🖥️host/📦️packages/🦀️rust/🦀️.rs`; dev-dependency `wit-parser =0.252.0` in its `Cargo.toml`, already in `Cargo.lock` via wasmtime), `🌐️browser-bundle/🛂️descriptor/🧪️tests/🧫️fixtures/🔣️.json`, `🌐️browser-bundle/🏗️materialization/🧪️tests/🧬️wit-lowering/🟦️.ts`, `🧪️tests/🧬️component-codec-reply/🟦️.ts`.

## Law test

`canonical_abi_flat_params`: parses the schema with `wit-parser`, takes every export of `world actor`, counts core params with an own spec-based flattener, requires at most 16 for every async func, and cross-checks `Resolve::wasm_signature(GuestExportAsync)` (no indirect params, same count; own count over 16 equals wit-parser `indirect_params` for every export). A second law pins that exactly the three sync stages exceed 16. The canonical `MAX_FLAT_ASYNC_PARAMS` (4) only bounds async-lowered imports, whose block the guest caller owns and frees; the export bound is `MAX_FLAT_PARAMS` (16), named `MAX_FLAT_ASYNC_EXPORT_PARAMS` in the test.

## Verified (native cargo)

- `cargo test -p semio-plugin-host --lib -- canonical_abi_flat_params schema_parity`: 10 passed (3 new law tests, 7 parity).
- `cargo check` (lib) of reactor-context, host-wit, host-guest-runtime, describe, host: clean.
- `cargo check --target wasm32-wasip2 -p semio-hub-sourcing-component` (workspace dir `🌎️hub`): clean, so the export macros compile against the new WIT.
- NOT run: bun tests (`wit-lowering`, `component-codec-reply`, descriptor), the wasmtime host test suites that load a component, and `--tests` check of crates depending on the peer's in-flux `semio-framework-job-testing-caller` (it failed with E0308 / missing `fund_retirement` earlier, not from this change).
- Unverified until rebuilt: runtime behaviour of sync exports on jco and wasmtime `call_concurrent`, and the leak curve itself.

## For the coordinator

- The "Missing actor export reactor.stageActorContextBudget" probe failure is a stale component (16:20 build predates the guest macros). The guest side is complete: rebuild every plugin component (all hub components and the extension components link the same macros) and restage. No bindings are committed; jco and wasmtime bindings regenerate from the WIT at build.
- Probe and finalization: `PLUGIN_DESCRIPTOR_PROBE_SOURCE` already stages then describes, and `ACTOR_COMPONENT_EXPORTS` is derived from the WIT, so the new exports are required automatically. `finalizePluginDescriptor` needs no change.
- Descriptors: `describe()` returns a `PackageDescriptor` pack with no export-signature content, so the committed `🛂️.descriptor.semio` / `🔣️.json` pairs should be byte-identical. Regenerate through the stager (`native-stage-modules.ts`, which runs the probe then `finalizePluginDescriptor`) and diff; commit only changed bytes.
- Untouched and stale: the `ticket-owned-browser-host-staging` test-kit guest fake (`retainedGuestSource`) already used an obsolete `poll(events, grant)` signature.
- After restage: the guest-memory law (`✏️s/🔌️plugins/🌀️procedural/…/📈️guest-memory/🦀️.rs`) and the sourcing spin probe should show no per-turn growth.
