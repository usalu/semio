# Retained Original Command Wire Receiving

## Implemented source

The generic retained-command work contract now offers optional original-wire demand/receiving hooks. The existing default keeps the cold command decoder for works that do not opt in. IFC details work opts in and uses `command_wire.rs`, which drives the existing `JsonGrammarCursor<DslValue>` one transition per original turn through `StepContext::with_native_receiving`. It does not create a replacement decoder ledger, cancellation root, raw source or permissive graph adapter.

The original job retains its immutable raw Vec. The cursor binds its real address/length and rejects rebinding. It requires the existing external `EditSnapshot` wrapper and its single `event` field, rejects duplicate JSON members in the native parser, and moves the exact operation/path/value/from/key/source owners into the shared `SnapshotEditEvent`. Every event has a closed field roster. No `FromValue` call or recursive graph clone occurs on this production path. Actual IFC4/IFC4x3 command wrappers consume the received event through the editing trait hook.

Parser demand includes the existing parser's actual copy/capacity/release/depth demand plus explicit enclosing field/error/phase metadata. The child parser receives a narrowed grant. Its actual receipt is consumed through `consume_receiving_retained`, preserving the initiating decoder's construction accounting rather than double charging accepted child allocation.

The job close order now honors the optional work's `retains_original_wire` declaration: its original receiving frontier closes before raw backing is released. Once that cursor closes, the job resumes its existing raw and work owner retirement. Parser, partially moved DSL objects, fields, result and JSON error remain in work-held owners across cancellation/refusal and drain through controlled retirement. Other work close order remains unchanged.

## Tests and evidence

Schema-first `command-wire.json` contains six valid actual native commands, five invalid closed shapes, duplicate-member wire and trailing-data wire. `command-oracle.ts` uses independent Ajv2020 to validate six valid and reject five invalid commands against the existing event schema inside an exact closed wrapper; ECMAScript JSON independently rejects trailing data. The duplicate-member native test is authored, not independently executed.

Native tests use `OriginalWorkFixture`, check zero-item denial without heap or cursor/source movement, compare actual heap events with retained receipts, cancel every original parsing/field frontier, verify a text pointer moves unchanged from parsed field custody into the event, reject a rebound original wire before parsing and clean up after cancellation. An IFC worker test builds wire with the actual editor command codec, receives the real command, transfers its original event into work and closes it after original cancellation.

Rustfmt parses the new cursor/tests and necessary shared interfaces. Native compilation remains blocked before type checking by authentic repository compiler ingress. Native execution, heap/cancellation/close assertions and full mounted dispatcher integration have **not executed**.

## Still open

The host action bridge can still build an already typed command by cloning incoming action arguments before this wire route runs. This implementation closes the source boundary for retained raw-command decoding; it does not claim to make every UI action conversion progressive. Path-only admission still uses synchronous temporary pointer vectors/reference normalization. Entity/constituent reduction still has synchronous reconstruction. Physical JSON parser and Store cursor depth bounds remain separate from event DSL depth. Follow `host-next-retained-frontiers.md`; none of these limits are represented as complete end-to-end closure.
