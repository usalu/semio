# Original Flow Receipt Export

The original FlowBridge now holds the actual retained receipt before propagating child failure, but the actual linear-memory export and browser receiver do not expose it. The new schema-first neutral contract and production receiving transport tests require all four original receipt fields on pending, failed allocation and terminal turns. Existing BigUint64Array independently validates the exported u64 projection. The export query will read the actual original Bridge scalar receipt, with no alternate ledger or synthesized child work.

Full normal transport allocations, feature argument/observer/Box closure, original domain Weak leases and actual browser Wasm publication remain unqualified. This work does not claim full original normal or closure qualification.

Registered source TDD replay 88507 reached the actual receiver and failed at missing `host.stepProgress()`. The source now adds mandatory `flow_bridge_step_progress(axis) -> u64` over the original Bridge receipt, resets that receipt at each actual FFI poll including cached transport turns, and reads all four axes immediately after every browser poll. Pending/failed/terminal receipts survive in the original Host state and outward failure object. No source pass is claimed until fresh replay.

Root combined replay 76073 terminated at Nx graph admission (`readCachedProjectGraph` unavailable) before compiler invocation; no compiler diagnostic was produced. This is distinct from prior 83810 Cargo build status 101.

The current canonical Value progress fields are `copied_items`, `copied_bytes`, `retained_capacity_bytes`, `released_bytes` (wire `copiedItems`, `copiedBytes`, `retainedCapacityBytes`, `releasedBytes`). New receipt schema, source projection and browser API use those defining names directly. No stale alias or compatibility field is retained.

Fresh source receiving replay 72730 terminated exit 0 with actual 3 receipt cases / 32 assertions, strict Ajv and independent BigUint64Array reference under the canonical current fields. Prior 67288 also passed the receiving cases before the field-name update; it does not qualify the current canonical source epoch. Neither result qualifies the current Wasm binary/publication or normal transport effects.

Original declarations replay 30531 terminated at the stale fixture count: current defining ABI has 109 operations, minus 3 declared excluded operations equals 106; fixture still specified 105. The normal runtime/projection names already matched. Updated the natural browser type schema/fixture to that actual census and the four canonical receipt fields, and restored real hostile fixture validation where previous source had empty probe bodies. Rust syntax replay 87862 passed 7 explicit original owners. Fresh declaration replay remains required.

Current declarations replay 12090 exited0; source declaration projection is green with TypeScript parser/prototype/package resolution parity and strict Ajv5 hostile probes. Actual Wasm publication remains pending.
