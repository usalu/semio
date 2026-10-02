# Current Script API And Source Boundaries Audit

Read-only source inspection on 2026-10-01. No tests or builds were run by this auditor. Files may be changing concurrently; findings describe the observed sources.

## P1: Value Still Exposes Runtime Serde And A Compatibility Bridge

`🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml` declares unconditional runtime `serde` and `serde_json` dependencies. The owned value implementation `🌱️value/🦀️.rs:273–358` implements public conversion and equality traits involving `serde_json::Value`, plus Serde serialization/deserialization. The package barrel at `📦️packages/🦀️rust/🦀️.rs:5` publishes the value API through `pub use value::*` without explicitly reexporting these external types. This survives a compile gate and contradicts the no external public API/runtime dependency requirement.

`🌱️value/🦀️.rs:423–430` explicitly says its infallible `to_dsl_value` is kept as `Result` for source compatibility. `from_dsl_value` at line 436 likewise reduces the owned `ValueError` to `String`. Actual production callers remain at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:905,917`. Remove the compatibility wrappers after converting all actual callers to `ToValue::to_value` and `FromValue::from_value`; remove the external conversions or put required boundary conversions behind an owned interface outside neutral Value. Do not merely change documentation.

## P1: Graph Still Depends On The OS Kernel API

`🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/🦀️.rs:11` aliases `semio_framework_os_kernel` as `dsl_core`, and line 16 publicly exports its engine. `⚙️engine/🦀️.rs:248,251,270,273` publishes conversion helpers returning `dsl_core::DslValue` and `dsl_core::ValueError`. `🗣️dsl/🦀️.rs:198,257` uses `dsl_core::json::to_dsl_value`. The new Value crate alone does not sever this dependency. Change Graph imports and Cargo ownership to the relevant neutral Value/Pack contracts; validate all public signature paths and actual conversion callers together.

## P2: Schema Test Commands Need One Canonical Taxonomy And Launch Registration

`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:54–56` registers three top-level names: `test-entity-ownership`, `test-subset-contract`, and `test-mutation-leaf-registration`. Their Nx targets at `📋️project.json:38–71` call the same names. The neutral process router instead uses `test <suite>` (`🏃️process/📜️script.ts:11,26–42`). Canonicalize all three Schema helpers under `test entity-ownership`, `test subset-contract`, and `test mutation-leaf-registration`, with explicit dispatch before Cargo argument interpretation. Update the three actual project target command strings together; no aliases are needed. Repository-wide hidden-file searches found no other non-ticket TypeScript/JSON clients for these three names. `.vscode/launch.json` currently contains no entries for them, so add the three executable Nx targets in the existing grouping/order.

## Verified Boundaries

Capture's public options/result (`🏃️process/📥️capture/🟦️.ts:5–14`) use owned records, primitive types and an owned cancellation callback; Node's Buffer/ChildProcess types remain implementation details. Capture performs direct `spawn(command,args)` at line 29 and does not parse or drop command arguments. `ScriptRouter.run` (`🏃️process/🧭️routing/🟦️.ts:68–76`) forwards `segments.slice(1)` to the selected owner. No capture-induced target forwarding defect was found by source inspection.

Cargo, exact Cargo, Wasm, native-artifact and Vitest public declarations inspected use owned records, first-party process types, or standard AbortSignal; no public third-party library type leak was found in these declarations. Their retained `V1` names identify schemas/policies in current code; this audit did not assume a name alone proves a legacy alias.

Value package raw mounts point to its own semantic implementation/retirement facets, not a cross-owner production mount. The only observed cross-owner raw Value mounts were OS Store's test-only public decode witnesses. Pack and UI Render package scripts inspected no longer import Repo helpers. Their source boundary behavior was not executed here.
