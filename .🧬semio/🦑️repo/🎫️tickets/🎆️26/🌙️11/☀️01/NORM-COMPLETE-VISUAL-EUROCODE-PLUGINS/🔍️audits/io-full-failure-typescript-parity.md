# Full IO Failure TypeScript Parity

2026-11-06. Goal active; ticket open. This closes the scoped TypeScript failure transport inconsistency. Current native and full norm acceptance remain incomplete.

## Problem and Result

The canonical IoError schema and Rust owner preserve a complete NativeSnapshotFailure. TypeScript previously stored and encoded only a ValueError, losing the independent receiving, allocation, retirement, encode-effect and outer settlement channels.

IoError now owns NativeSnapshotFailure. Its ValueError/text constructors preserve the exact original error inside the value cause. The public primaryValue, kind, message and retainedProgress projections borrow from the complete owner; they do not discard other causes. The public IO package explicitly exports the owner and its first-party nested types.

Controlled snapshot codecs encode/decode all closed records, exact tags and unsigned values. They preserve original ValueError objects on encoding and reconstruct typed errors on decoding, including every nested retirement cause and progress override. Independent allocation quote/post/cancel causes and performed-null/zero/positive extents remain separate. The three optional request-progress axes must be all absent or all present.

The existing canonical retained-progress codecs are now explicitly exported through the Value refusal codec facade and reused, rather than implementing another receipt parser. No new external runtime dependency was added.

## Evidence

- Genuine initial runtime red: 2 failed / 1 passed; the TypeScript encoder emitted the old three-field cause and its decoder rejected the canonical full failure wire.
- Full successor 76982: terminal 0, 4 tests and 295 assertions passed. Coverage includes all 17 shared success wires, hostile closed-field/receipt/control cases, diagnostic parity, a two-level four-cause retirement tree, full encode-effect allocation failure, all 12 receiving allocation error channels, actual extent null/0/73, and all 8 request-progress presence combinations. AJV validates the same full wire shapes independently.
- Strict TypeScript closures 24465, 59337 and final85991 completed with exit 0. The final closure includes the actual public IO package facade and updated tests.
- Native IO staged source/cancellation tests are owned by io_receive_terra and have not executed. Native source extraction, retirement seed conversion, original Birth selection and encode continuation/work ownership remain active.

## Changed Source

- `🧰️framework/🔨️modules/🚪️io/⏱️failure-owner/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/⏱️failure-owner/🔁️codec/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/⚠️refusal/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/⚠️refusal/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/⚠️refusal/🔁️codec/🟦️.ts`
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/⚠️refusal/🔁️codec/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🟦️typescript/🟦️.ts`

The ticket test entry imports the actual production test module; it does not copy source. The scoped compiler config and test entry are retained as inputs in the ticket; generated command logs are under the ticket generated folder.

## Remaining Goal Work

No current full native norm renderer/request/mounted/browser result is implied. Concrete failure/storage retirement seeds and their 18 native laws still need a coherent lower producer/standard seed gate; the last compiled retirement snapshot exposed 8 marker failures out of 16. Original Member lifecycle native laws, two-parent encode mediator/continuations/work, staged IO native Pending cancellation, source-reference issuance, full 15-family current acceptance and the bilingual browser witness remain outstanding.

## Final Captured TypeScript Parity Checkpoint

The final actual I/O refusal domain run passed 4 tests / 301 assertions after source type colocation. Strict TypeScript checking passed in session 16143. This supersedes the earlier 295-assertion checkpoint. Thirteen implementation and test files were changed, including the real encode allocation/effect type owners, their explicit public facades, and package exports; the full I/O cause codec uses those actual first-party types. The independent existing Ajv validation covers the shared wire corpus, hostile records, nested retirement causes, and all eight progress-presence combinations. These results establish the TypeScript domain boundary only; the Rust I/O and complete native pipeline are not yet verified.

## Context and Marker Cause Successor

The closed encoder effect wire gained two independent owned causes, `context` and `markerFailure`. The genuine pre-fix run 16255 failed (3 pass / 1 fail). The successor 79809 passed all 4 tests / 315 assertions, including each new cause as the sole primary, complete round trips, and rejection when either required field is missing. Strict TypeScript successor 19463 also passed. The Rust/schema worker independently passed 4 AJV tests / 44 assertions on the updated shared wire. Earlier 301-assertion results remain historical.

## Independent Work Overrun Causes

Encoded workOverrun parity first failed in 54373, then 63846 passed 4 tests / 322 assertions and strict TypeScript 13189 passed. Receiving workOverrun is a separate canonical owner field. Its new fixture exposed an over-broad allocation assertion; the old allocation assertions were retained in their applicable cases, and the new fixture separately checks both original work causes and primary identity. The completed runtime successor passed 4 tests / 338 assertions. The independent IO schema worker passed 5 tests / 57 assertions across both schemas and both overrun domains. Rust IO acceptance remains pending; these are TypeScript and schema results.

Final strict TypeScript successor 97080 exited zero after the final 338-assertion parity test edit.
