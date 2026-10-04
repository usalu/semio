# Incremental Prepared Scene Retirement

The preceding source-ownership milestone is verified: 516 native tests passed, and two focused runtime tests confirmed genuine store snapshot leases return on success, cancellation and failure. See [source evidence](🎬️vector-snapshot-source.md). The editor preview remains on the existing port 6064 process; this change does not replace that running bundle.

## Contract and Implementation

Prepared scene retirement now has an authored JSON schema, nine neutral fixtures, Rust and TypeScript implementations, and tests using real DocumentSceneJob output. One grant consumes one shallow owner or one container entry. Logical owners and genuine work are separate counters; fixtures specify both terminal totals. Collections are consumed from the end in both languages. Group scopes, operand strings, semantic text, image aliases, gradient stop buffers, and stroke dash buffers are retired explicitly. Invalid grants leave the plan untouched, and terminal progress is idempotent.

The tests independently census the expected prepared JSON, validate every TypeScript progress result with AJV, and parse the shared cases with serde_json in native tests. Retained image aliases remain valid; native terminal tests require Arc strong counts to return to exactly one. Source documents use the established binary64 lift in TypeScript, matching preparation tests.

These counters represent logical ownership, not allocator bytes. Flat native POD buffers can be dropped as one structural unit; source aliases can remain owned by other callers. An incomplete close job must remain owned by its eventual mounted registry and be stepped to completion. Its ordinary early Rust destruction is not an incremental close operation. This feature alone does not satisfy plugin byte-credit admission or active Boolean/trace kernel retirement.

## Verification

- Red gate `scene-retirement-ts-red.log`: missing retirement implementation, as expected before implementation.
- First implementation gate `scene-retirement-ts-green.log`: 243 passed, seven failures caused by test fixtures bypassing binary64 source conversion. Corrected without changing production source parsing.
- Corrected fixture gate `scene-retirement-ts-green-fixed.log`: all 250 tests passed; strict checking then correctly refused readonly group scopes in the consuming owner. Prepared DocumentSceneNode now explicitly owns a mutable group array, preserving readonly borrowing in raster inputs. No cast or whole-array clone was added.
- Full native gate `scene-retirement-native.log`, handle 64847: exit 0, **518 passed / zero failed or skipped**, 3m17 total including shared Cargo preparation. No lock/process was removed or interrupted.
- Full TypeScript gate `scene-retirement-ts-full.log`, handle 56763: exit 0, **561 passed / zero failed / 1,718,227 assertions / 40 files**, plus strict production and independent PDF/SVG, field-patch, scheduler and publication checks.
- Final focused TypeScript gate `scene-retirement-ts-final.log`, handle 64945: exit 0, **250 passed / zero failed / 1,499,354 assertions**, plus strict checks. This adds schema rejection of inconsistent phase/done pairs.
- Native runtime gate `scene-retirement-native-runtime.log`, handle 61329: exit 0, **two passed / 516 intentionally filtered**; actual `[DEBUG]` diagnostics confirm all nine authored ownership cases reach empty shells.
- Scoped `git diff --check`: exit 0. All current validation handles are terminal.

All logs are under this ticket's generated directory. Existing scene-raster and full Draw TypeScript Nx commands include the new tests and strict production checks. Existing native schema registration includes the retirement module. No new executable command or script was introduced.

## Files

The new directory is `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧹️retire`, containing `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, `🦀️.rs`, `🟦️.ts`, `🧪️tests/🔬️unit/🦀️.rs` and `🧪️tests/🔬️unit/🟦️.ts`. Registration changes are in the subset schema `🦀️.rs` and Draw TypeScript package `📜️script.ts`. The preparation `🎬️scene/📋️prepare/🟦️.ts` now declares its owned mutable group scopes explicitly.

## Remaining Goal Work

Actual scheduled editor production and canvas consumption still need mounted ownership and truthful cancellation/close across preparation, decoding, trace and Boolean stages. Selection, framing, picking and conversions must use the same complete current-revision geometry. Typography outlines, full import/export, browser journeys and multiuser acceptance remain open. The overall goal and ticket remain active. Repo MCP lifecycle tools remain unavailable in this session; no close operation is claimed.

## Next Integration Boundary

Active kernel ownership was read directly after verification. PathBooleanJob owns an IntoIter of PathBooleanOperand, an IntoIter of FlatContour, nested world contours, prepared BooleanOperand vectors and optional flatten/region child jobs. BooleanJob owns edges with BTreeSet parameters, nested source operands, grid/outgoing maps and ring vectors. BitmapTraceJob owns nested raw/base/candidate contours, outgoing maps, positions/kept/changes trees and an optional borrowed/moved mask. Their existing clear/cancel paths deeply release these structures synchronously; the document wrappers also eagerly clear input plans and string indexes.

The next implementation must add genuine grant-driven retirement to those active jobs and wrappers, preserving sticky cancellation/failure and private results. A flat complete-plan close cursor alone cannot justify mounted PluginCloseStep completion. Tests must interrupt real phases, verify one admitted close unit per grant, retain required source aliases until safe return, and establish an empty terminal destructor. Byte credits need actual capacity/owner admission accounting rather than the logical counters authored here. Then the actual Draw mounted hooks and canvas consumer can be activated using the captured store identity.
