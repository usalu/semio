# Original Fixed Result Backing

The new domain-neutral owner is actor `🖼️wire-turn/📬️typed-operation/📦️backing/🦀️.rs`, mounted as a child of the existing typed-operation module. No root facade module, FullOp fields, publication pump, or diagnostics lifecycle changed. The actor's cfg-inclusive physical source closure is 21 files; the new native-test path is 100 characters.

Schema-first input and tests live at actor `🧪️tests/📃️result-backing/{🔣️.json,🧬️.json,🟦️.ts}`. Ajv validates the closed fixture; lodash independently chunks the fixed extent and supplies initialization-length oracles. TDD: the source law failed while the owner was absent (1 pass, 1 fail), then passed after implementation.

OriginalResultPageBacking begins empty. A quoted raw birth retains exactly 4096 capacity with length zero. Initialization pays metadata plus strictly positive bytes, never more than 2048 payload bytes per call. A zero/submetadata/metadata-only copy grant returns zero work and unchanged state. The initialized slice borrows the retained allocation. A separately quoted from_original handoff preserves its pointer and empties backing custody. Every partial body closes through ControlledRetirement. ManuallyDrop plus explicit terminal Drop assertion prevents ordinary loss of original backing.

Raw birth, initialization, handoff and close return RetainedWorkOutcome with actual progress/work. No raw method creates a control, ceiling or caller budget. The raw owner allows cancellation-safe closure and retirement-funded birth; genuine caller ports decide admission. Returned written-byte extents from the separate wire adapter are not treated as complete traversal receipts.

Two native tests are authored under `📦️backing/🧪️tests/🦀️.rs`. They use genuine OriginalJobFixtureCaller and decoder, actual original construction/retirement ports, and physical heap versus caller receipt comparisons. Coverage includes denied item/copy/capacity/release/depth currencies, initialization cursor boundaries, fixed pointer handoff, close at lengths 0/1/2048/4096, cancelled retirement birth, and borrowed unwind retaining custody. Test data never raises production quotas or renews caller authority.

Exact queued target: `semio-framework-actor --lib original_result_backing_` (two tests), owning workspace `/home/deck/semio/🧰️framework`. The editor driver queues it after direct-emission and wire tests, under corrected core gates. This agent launched no heavy command.

Latest combined neutral validation: 21 tests, 304 assertions, 346 ms. New backing/native-test and lower source-sink files parse under Rust AST. Native semantic/physical results remain pending; no end-to-end browser claim is made.

The audit identified static refusal strings allocated by the existing borrowed JSON source-sink through ValueError::new. Five static call sites now use ValueError::literal, retaining their text and behavior without owned diagnostic strings. Native zero-heap refusal tests remain strict; remaining shared control errors are being audited separately.
