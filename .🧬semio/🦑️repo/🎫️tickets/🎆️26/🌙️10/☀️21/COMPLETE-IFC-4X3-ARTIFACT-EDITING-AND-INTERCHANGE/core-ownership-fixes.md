# IFC4x3 Owned Capture and Re-audit Fixes

The P1 caller-alias race and P2 base-free coordinate admission findings in `core-reaudit.md` are resolved. Regression scenarios and executable tests were added first: all three focused new tests initially failed, reproducing an invalid returned graph, invalid history publication, and admitted NaN absorption. A later omitted-complex fixture initially failed the snapshot JSON Schema before its domain semantics were aligned.

## Owned Authority Before Admission

The previous order admitted a graph that still referenced caller values, then yielded while copying those values. The central authority now captures base graph and sparse proposal into private owned data first, admits those owned inputs, computes next state and original-coordinate inverse exclusively from them, and validates the final owned graph and inverse before returning. No caller object is reread after its capture has become the admitted authority. Subsequent progress callbacks and scheduling cannot reach the prepared graph through the original base, diff or command.

Capture remains incremental, bounded and cancellable. It fixes each array cursor's length, checks typed array/value/depth budgets before expansion, copies without an eager child frontier, and rejects undeclared object fields before reading their values. The numeric-child getter tests still show **zero reads** for 100,000 children under `maxValues:3`. A new unknown-field getter test refuses without invoking the getter at all.

This moves the existing full graph copy before admission rather than adding a second full graph copy. The transaction does not copy the whole graph again after final admission. Sparse planning/inverse/absorption return their private owned results directly when no additional copy is needed. Public history events and snapshot/event reads still copy because they must isolate private stored state from caller mutation.

The documented capture contract guarantees a validated owned result. Incremental cooperative capture cannot promise an atomic start-of-call image of arbitrarily mutating caller data. Callers requiring that exact image must leave their inputs unchanged until the operation settles. Concurrent source changes may be observed during capture or cause a refusal; they cannot invalidate an already admitted private authority. The public graph/result types remain mutable and own their returned data.

## Command and Event Consistency

Commands are captured once before planning. The command used to produce the sparse diff is the same captured command recorded in the history event. Event preparation no longer separately rereads the original mutable command after applying its diff. Phase-by-phase tests mutate original base/proposal values during capture, base admission, proposal admission, application and result admission. Unsafe captured data is refused; accepted transactions reproduce their graph through the stored forward diff and restore the canonical base through the stored inverse.

History stores only private owned state. Its final publication remains a synchronous cancellation/revision check followed by snapshot/event/undo/redo updates. Prepared graphs/events are never exposed to progress callbacks. Cancellation during final result validation or public event-copy preparation publishes nothing, and existing competing revision tests still publish exactly one event.

## Base-free Sparse Admission

Public absorption now admits the domains it can check without a base:

- Every coordinate is a nonnegative safe integer; NaN, infinities, negatives and fractional positions are refused.
- Every entity identity is a positive u64 bigint.
- Entity names and complex constituent names are valid native EXPRESS identifiers.
- Removal/modification/insertion targets are unique within their coordinate lanes.
- Entity modification cannot overlap entity removal/insertion; argument removal cannot overlap argument modification. Original-coordinate modifications and result-coordinate insertions remain separate valid spaces.

Base-dependent existence and upper-bound checks remain in central application/validated composition. Captured absorption inputs and its owned result are admitted, preventing Map/Set coalescing from hiding malformed duplicate targets.

## Optional Complex Constituents

The shared IFC graph's optional `complex` field means an empty collection when omitted. Owned base and inserted-entity capture canonicalize omission to `[]`; inverses/history restore the canonical graph. The new IFC4x3 snapshot JSON Schema now permits omission, agreeing with its public shared graph alias and Rust's empty-field omission semantics. No shared type or compatibility adapter was introduced.

The retained language-neutral JSON fixture `omittedComplex` is checked against Ajv. Independent `fast-json-patch` adds the canonical empty collection and performs the scalar edit; its output agrees with central application. Mutation/inverse and history undo restore the same canonical graph.

## Verification

- Focused editing suite: **31 tests, 244 assertions, zero failures**.
- Strict TypeScript check of both core modules and editing suite passes: `--noEmit --strict --target ES2022 --module ESNext --moduleResolution Bundler --allowImportingTsExtensions --skipLibCheck --types bun`.
- `core-ownership-repro.ts`, based on the retained re-audit cases, confirms four safe refusals (synchronous capture corruption, asynchronous capture corruption, history capture corruption, NaN absorption), zero history revision/event advance after the failed capture, and recorded-command replay consistency after a valid capture interleaving.
- The earlier independent `core-repro.ts` still passes **1,864** coordinate composition/inverse law combinations, refuses numeric binary input, reports **0** wide-child reads, and reports `result mutation changes original false`.
- Existing independent `fast-json-patch` and Ajv checks remain; new optional-complex and captured-command replay assertions extend them.

Changed production/test files in this follow-up are the new IFC4x3 `🧬️schema/🔺️diff/🟦️.ts`, `🧬️schema/🧬️mutations/🟦️.ts`, `🧬️schema/🔣️.json`, and `🧪️tests/🧬️editing/{🟦️.ts,🔣️.json,🥒️.feature}`. No public facade export change is required; new capture helpers are internal sibling-module infrastructure. Ticket input scripts/reports are retained; this wave created no generated output files. No AGENTS, modifying Git command, worktree, build infrastructure or external runtime dependency changed. Focused direct source/Bun/TypeScript tools used the already recorded necessary fallback execution route. Whole-artifact/Rust build limitations remain unchanged and are not claimed as verified here.
